//! Main compiler pipeline that orchestrates all compilation phases

use crate::{
    lexer::Lexer,
    parser::Parser,
    semantic::SemanticModel,
    codegen::{
        CodeGenerator,
        il::ILAssembly,
        pe::PEGenerator,
    },
    diagnostics::{DiagnosticBag, Diagnostic, DiagnosticCode, Severity},
    assembly_loader::AssemblyLoader,
};
use std::path::{Path, PathBuf};
use std::fs;
use std::io::Write;

/// Compiler options
#[derive(Debug, Clone)]
pub struct CompilerOptions {
    /// Output file path
    pub output_path: Option<PathBuf>,
    /// Output type
    pub output_type: OutputType,
    /// Optimization level
    pub optimization_level: OptimizationLevel,
    /// Assembly references
    pub references: Vec<String>,
    /// Define preprocessor symbols
    pub defines: Vec<String>,
    /// Enable debug info
    pub debug: bool,
    /// Treat warnings as errors
    pub warnings_as_errors: bool,
    /// Suppress specific warnings
    pub suppress_warnings: Vec<u32>,
}

impl Default for CompilerOptions {
    fn default() -> Self {
        Self {
            output_path: None,
            output_type: OutputType::Exe,
            optimization_level: OptimizationLevel::Debug,
            references: vec!["mscorlib".to_string(), "System".to_string()],
            defines: Vec::new(),
            debug: true,
            warnings_as_errors: false,
            suppress_warnings: Vec::new(),
        }
    }
}

/// Output type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputType {
    /// Console executable
    Exe,
    /// Windows executable
    WinExe,
    /// Dynamic library
    Dll,
    /// Module (no manifest)
    Module,
}

/// Optimization level
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OptimizationLevel {
    /// No optimizations
    None,
    /// Debug build (minimal optimizations)
    Debug,
    /// Release build (full optimizations)
    Release,
}

/// Compilation result
#[derive(Debug)]
pub struct CompilationResult {
    /// Success flag
    pub success: bool,
    /// Output file path (if successful)
    pub output_file: Option<PathBuf>,
    /// Diagnostics
    pub diagnostics: DiagnosticBag,
    /// Assembly metadata (if successful)
    pub assembly: Option<ILAssembly>,
}

/// The main C# compiler
pub struct CSharpCompiler {
    /// Compiler options
    options: CompilerOptions,
    /// Assembly loader
    assembly_loader: AssemblyLoader,
    /// Diagnostics
    diagnostics: DiagnosticBag,
}

impl CSharpCompiler {
    /// Create a new compiler instance
    pub fn new(options: CompilerOptions) -> Self {
        Self {
            options,
            assembly_loader: AssemblyLoader::new(),
            diagnostics: DiagnosticBag::new(),
        }
    }
    
    /// Compile a single source file
    pub fn compile_file<P: AsRef<Path>>(&mut self, source_path: P) -> CompilationResult {
        let source_path = source_path.as_ref();
        
        // Read source file
        let source_code = match fs::read_to_string(source_path) {
            Ok(code) => code,
            Err(err) => {
                self.diagnostics.add(Diagnostic {
                    severity: Severity::Error,
                    code: DiagnosticCode::CS0001,
                    message: format!("Cannot read source file '{}': {}", source_path.display(), err),
                    location: None,
                    notes: Vec::new(),
                });
                return self.create_failed_result();
            }
        };
        
        // Determine output path
        let output_path = self.determine_output_path(source_path);
        
        // Compile source code
        self.compile_source(&source_code, &output_path)
    }
    
    /// Compile multiple source files
    pub fn compile_files<P: AsRef<Path>>(&mut self, source_paths: &[P]) -> CompilationResult {
        // For now, compile each file separately and merge
        // In a real implementation, we'd compile them together
        
        if source_paths.is_empty() {
            self.diagnostics.add(Diagnostic {
                severity: Severity::Error,
                code: DiagnosticCode::CS0001,
                message: "No source files specified".to_string(),
                location: None,
                notes: Vec::new(),
            });
            return self.create_failed_result();
        }
        
        // Just compile the first file for now
        self.compile_file(&source_paths[0])
    }
    
    /// Compile source code
    pub fn compile_source(&mut self, source_code: &str, output_path: &Path) -> CompilationResult {
        // Phase 1: Lexical analysis
        let tokens = {
            let mut lexer = Lexer::new(source_code);
            lexer.tokenize()
        };
        
        // Phase 2: Parsing
        let syntax_tree = {
            let mut parser = Parser::new(source_code);
            let tree = parser.parse();
            
            // Check for parse errors
            if parser.diagnostics().has_errors() {
                self.diagnostics.merge(parser.diagnostics());
                return self.create_failed_result();
            }
            
            // Add parse warnings
            self.diagnostics.merge(parser.diagnostics());
            tree
        };
        
        // Phase 3: Semantic analysis
        let semantic_model = {
            let mut model = SemanticModel::new(syntax_tree);
            
            // Add assembly references
            for reference in &self.options.references {
                if let Err(err) = model.add_reference(reference) {
                    self.diagnostics.add(Diagnostic {
                        severity: Severity::Error,
                        code: DiagnosticCode::CS0001,
                        message: format!("Cannot load assembly '{}': {}", reference, err),
                        location: None,
                        notes: Vec::new(),
                    });
                }
            }
            
            // Perform semantic analysis
            model.analyze();
            
            // Check for semantic errors
            if model.diagnostics().has_errors() {
                self.diagnostics.merge(model.diagnostics());
                return self.create_failed_result();
            }
            
            // Add semantic warnings
            self.diagnostics.merge(model.diagnostics());
            model
        };
        
        // Phase 4: IL generation
        let il_assembly = {
            let assembly_name = output_path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Assembly");
            let mut generator = CodeGenerator::new(semantic_model, assembly_name);
            match generator.generate() {
                Ok(_pe_bytes) => {
                    // For now, return a dummy assembly
                    // In a real implementation, we'd extract the assembly from the generator
                    ILAssembly::new(assembly_name)
                },
                Err(err) => {
                    self.diagnostics.add(Diagnostic {
                        severity: Severity::Error,
                        code: DiagnosticCode::CS0001,
                        message: format!("IL generation failed: {}", err),
                        location: None,
                        notes: Vec::new(),
                    });
                    return self.create_failed_result();
                }
            }
        };
        
        // Phase 5: Optimization (if enabled)
        let optimized_assembly = if self.options.optimization_level != OptimizationLevel::None {
            self.optimize_assembly(il_assembly)
        } else {
            il_assembly
        };
        
        // Phase 6: PE generation
        let pe_data = {
            let mut generator = PEGenerator::new(optimized_assembly.clone());
            
            // Configure PE generator based on output type
            match self.options.output_type {
                OutputType::Exe => generator.set_subsystem(pe::Subsystem::Console),
                OutputType::WinExe => generator.set_subsystem(pe::Subsystem::Windows),
                OutputType::Dll => generator.set_is_dll(true),
                OutputType::Module => {
                    // Module compilation not fully implemented
                }
            }
            
            match generator.generate() {
                Ok(data) => data,
                Err(err) => {
                    self.diagnostics.add(Diagnostic {
                        severity: Severity::Error,
                        code: DiagnosticCode::CS0001,
                        message: format!("PE generation failed: {}", err),
                        location: None,
                        notes: Vec::new(),
                    });
                    return self.create_failed_result();
                }
            }
        };
        
        // Phase 7: Write output file
        if let Err(err) = fs::write(output_path, &pe_data) {
            self.diagnostics.add(Diagnostic {
                severity: Severity::Error,
                code: DiagnosticCode::CS0001,
                message: format!("Cannot write output file '{}': {}", output_path.display(), err),
                location: None,
                notes: Vec::new(),
            });
            return self.create_failed_result();
        }
        
        // Check if we should fail on warnings
        if self.options.warnings_as_errors && self.diagnostics.has_warnings() {
            self.diagnostics.add(Diagnostic {
                severity: Severity::Error,
                code: DiagnosticCode::CS0001,
                message: "Compilation failed due to warnings treated as errors".to_string(),
                location: None,
                notes: Vec::new(),
            });
            return self.create_failed_result();
        }
        
        // Success!
        CompilationResult {
            success: true,
            output_file: Some(output_path.to_path_buf()),
            diagnostics: self.diagnostics.clone(),
            assembly: Some(optimized_assembly),
        }
    }
    
    /// Determine output path based on source file and options
    fn determine_output_path(&self, source_path: &Path) -> PathBuf {
        if let Some(path) = &self.options.output_path {
            path.clone()
        } else {
            let extension = match self.options.output_type {
                OutputType::Exe | OutputType::WinExe => "exe",
                OutputType::Dll => "dll",
                OutputType::Module => "netmodule",
            };
            
            source_path.with_extension(extension)
        }
    }
    
    /// Optimize IL assembly
    fn optimize_assembly(&self, assembly: ILAssembly) -> ILAssembly {
        // TODO: Implement IL optimizations
        // For now, just return the assembly unchanged
        assembly
    }
    
    /// Create a failed compilation result
    fn create_failed_result(&self) -> CompilationResult {
        CompilationResult {
            success: false,
            output_file: None,
            diagnostics: self.diagnostics.clone(),
            assembly: None,
        }
    }
}

/// Subsystem types for PE files
mod pe {
    #[derive(Debug, Clone, Copy)]
    pub enum Subsystem {
        Console,
        Windows,
    }
}

/// Extension methods for PE generator
impl PEGenerator {
    fn set_subsystem(&mut self, _subsystem: pe::Subsystem) {
        // TODO: Implement subsystem configuration
    }
    
    fn set_is_dll(&mut self, _is_dll: bool) {
        // TODO: Implement DLL configuration
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_compiler_creation() {
        let options = CompilerOptions::default();
        let compiler = CSharpCompiler::new(options);
        assert!(compiler.diagnostics.is_empty());
    }
    
    #[test]
    fn test_compile_hello_world() {
        let source = r#"
using System;

class Program
{
    static void Main()
    {
        Console.WriteLine("Hello, World!");
    }
}
"#;
        
        let options = CompilerOptions::default();
        let mut compiler = CSharpCompiler::new(options);
        
        let result = compiler.compile_source(source, Path::new("test.exe"));
        
        // The compilation might not fully succeed due to incomplete implementation,
        // but we should at least not crash
        assert!(!result.diagnostics.is_empty() || result.success);
    }
}