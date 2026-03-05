//! Command-line interface for the C# compiler

use clap::{Parser, Subcommand};
use csharp_compiler::{
    CSharpCompiler, CompilerOptions, OutputType, OptimizationLevel,
    lexer::Lexer,
    parser::Parser as CSharpParser,
};
use std::path::PathBuf;
use std::fs;

#[derive(Parser)]
#[command(name = "csc-rust")]
#[command(about = "A C# compiler written in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile C# source files
    Compile {
        /// Source files to compile
        #[arg(required = true)]
        files: Vec<PathBuf>,
        
        /// Output file name
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Target type (exe, winexe, dll, module)
        #[arg(short, long, default_value = "exe")]
        target: String,
        
        /// Optimization level (none, debug, release)
        #[arg(short = 'O', long, default_value = "debug")]
        optimize: String,
        
        /// Assembly references
        #[arg(short, long)]
        reference: Vec<String>,
        
        /// Define preprocessor symbols
        #[arg(short, long)]
        define: Vec<String>,
        
        /// Enable debug information
        #[arg(long)]
        debug: bool,
        
        /// Treat warnings as errors
        #[arg(long)]
        warnaserror: bool,
        
        /// Suppress specific warnings
        #[arg(long)]
        nowarn: Vec<u32>,
    },
    
    /// Lex a C# source file (for debugging)
    Lex {
        /// Source file to lex
        file: PathBuf,
        
        /// Show trivia (whitespace and comments)
        #[arg(long)]
        show_trivia: bool,
    },
    
    /// Parse a C# source file (for debugging)
    Parse {
        /// Source file to parse
        file: PathBuf,
        
        /// Maximum depth to display
        #[arg(long, default_value = "3")]
        max_depth: usize,
    },
    
    /// Show compiler version
    Version,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Compile { 
            files, 
            output, 
            target, 
            optimize,
            reference,
            define,
            debug,
            warnaserror,
            nowarn,
        } => {
            // Parse output type
            let output_type = match target.as_str() {
                "exe" => OutputType::Exe,
                "winexe" => OutputType::WinExe,
                "dll" | "library" => OutputType::Dll,
                "module" => OutputType::Module,
                _ => {
                    eprintln!("Error: Invalid target type '{}'. Valid options: exe, winexe, dll, module", target);
                    return Ok(());
                }
            };
            
            // Parse optimization level
            let optimization_level = match optimize.as_str() {
                "none" => OptimizationLevel::None,
                "debug" => OptimizationLevel::Debug,
                "release" => OptimizationLevel::Release,
                _ => {
                    eprintln!("Error: Invalid optimization level '{}'. Valid options: none, debug, release", optimize);
                    return Ok(());
                }
            };
            
            // Create compiler options
            let mut options = CompilerOptions {
                output_path: output,
                output_type,
                optimization_level,
                references: if reference.is_empty() {
                    vec!["mscorlib".to_string(), "System".to_string()]
                } else {
                    reference
                },
                defines: define,
                debug,
                warnings_as_errors: warnaserror,
                suppress_warnings: nowarn,
            };
            
            // Create compiler
            let mut compiler = CSharpCompiler::new(options);
            
            // Compile files
            println!("Compiling {} file(s)...", files.len());
            let result = if files.len() == 1 {
                compiler.compile_file(&files[0])
            } else {
                compiler.compile_files(&files)
            };
            
            // Display diagnostics
            if result.diagnostics.len() > 0 {
                println!("\nDiagnostics:");
                for diag in result.diagnostics.iter() {
                    let prefix = match diag.severity {
                        csharp_compiler::diagnostics::Severity::Error => "error",
                        csharp_compiler::diagnostics::Severity::Warning => "warning",
                        csharp_compiler::diagnostics::Severity::Info => "info",
                    };
                    println!("{} {}: {}", prefix, diag.code(), diag.message());
                }
            }
            
            if result.success {
                if let Some(output_file) = result.output_file {
                    println!("\nCompilation succeeded!");
                    println!("Output: {}", output_file.display());
                }
            } else {
                eprintln!("\nCompilation failed!");
                std::process::exit(1);
            }
        }
        
        Commands::Lex { file, show_trivia } => {
            let source = fs::read_to_string(&file)?;
            let mut lexer = Lexer::new(&source);
            
            println!("Tokens from {}:", file.display());
            println!("{:-<80}", "");
            println!("{:<25} {:<15} {:<40}", "KIND", "POSITION", "TEXT");
            println!("{:-<80}", "");
            
            for token in lexer.tokenize() {
                // Don't skip tokens based on trivia
                
                let position = format!("{:?}..{:?}", token.range.start(), token.range.end());
                let text = if token.text.len() > 40 {
                    format!("{}...", &token.text[..37])
                } else {
                    token.text.to_string()
                };
                
                println!("{:<25} {:<15} {:?}", 
                    format!("{:?}", token.kind), 
                    position,
                    text
                );
            }
        }
        
        Commands::Parse { file, max_depth } => {
            let source = fs::read_to_string(&file)?;
            let mut parser = CSharpParser::new(&source);
            let syntax_tree = parser.parse();
            
            println!("Parsing {}...", file.display());
            
            if parser.diagnostics().has_errors() {
                println!("\nParser errors:");
                for diag in parser.diagnostics().iter() {
                    println!("  error {}: {}", diag.code(), diag.message());
                }
            } else {
                println!("\nParsing successful!");
                println!("\nSyntax tree:");
                println!("{:-<80}", "");
                print_syntax_node(&syntax_tree.root, 0, max_depth);
            }
            
            if parser.diagnostics().has_warnings() {
                println!("\nParser warnings:");
                for diag in parser.diagnostics().iter() {
                    if diag.severity == csharp_compiler::diagnostics::Severity::Warning {
                        println!("  warning {}: {}", diag.code(), diag.message());
                    }
                }
            }
        }
        
        Commands::Version => {
            println!("C# Compiler in Rust");
            println!("Version: 0.1.0");
            println!("Copyright (c) 2024");
            println!();
            println!("Supported C# version: 11.0");
            println!("Target framework: .NET Framework 4.8 / .NET 6.0+");
        }
    }
    
    Ok(())
}

fn print_syntax_node(node: &csharp_compiler::syntax::SyntaxNode, depth: usize, max_depth: usize) {
    if depth > max_depth {
        return;
    }
    
    let indent = "  ".repeat(depth);
    
    // Count children
    let child_count = node.children.len();
    let node_info = if child_count > 0 {
        format!("{:?} ({} children)", node.kind, child_count)
    } else {
        format!("{:?}", node.kind)
    };
    
    println!("{}{}", indent, node_info);
    
    if depth < max_depth {
        for child in &node.children {
            match child {
                csharp_compiler::syntax::SyntaxElement::Node(n) => {
                    print_syntax_node(n, depth + 1, max_depth);
                }
                csharp_compiler::syntax::SyntaxElement::Token(t) => {
                    let token_indent = "  ".repeat(depth + 1);
                    let text = if t.text.len() > 40 {
                        format!("{}...", &t.text[..37])
                    } else {
                        t.text.to_string()
                    };
                    println!("{}{:?}: {:?}", token_indent, t.kind, text);
                }
            }
        }
    } else if child_count > 0 {
        let indent = "  ".repeat(depth + 1);
        println!("{}... ({} more)", indent, child_count);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cli_parsing() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}