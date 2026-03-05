//! Compilation unit management
//! 
//! Orchestrates the entire compilation process.

use crate::{CompilerOptions, parser::Parser, semantic::SemanticModel, emit::Emitter};
use crate::diagnostics::DiagnosticBag;

/// Represents a single compilation session
pub struct Compilation {
    sources: Vec<SourceFile>,
    options: CompilerOptions,
    diagnostics: DiagnosticBag,
}

/// A source file in the compilation
pub struct SourceFile {
    pub path: String,
    pub content: String,
}

impl Compilation {
    /// Create a new compilation
    pub fn new(options: CompilerOptions) -> Self {
        Self {
            sources: Vec::new(),
            options,
            diagnostics: DiagnosticBag::new(),
        }
    }
    
    /// Add a source file to the compilation
    pub fn add_source(&mut self, path: String, content: String) {
        self.sources.push(SourceFile { path, content });
    }
    
    /// Compile all sources
    pub fn compile(&mut self) -> Result<Vec<u8>, DiagnosticBag> {
        // Parse all sources
        for source in &self.sources {
            let mut parser = Parser::new(&source.content);
            let syntax_tree = parser.parse();
            
            // Perform semantic analysis
            let mut semantic_model = SemanticModel::new(syntax_tree);
            semantic_model.analyze();
            
            // Generate code
            let mut emitter = Emitter::new(semantic_model);
            match emitter.emit() {
                Ok(bytes) => return Ok(bytes),
                Err(diagnostics) => self.diagnostics = diagnostics,
            }
        }
        
        Err(std::mem::take(&mut self.diagnostics))
    }
}