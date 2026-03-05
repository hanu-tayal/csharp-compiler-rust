//! A C# compiler written in Rust, inspired by Roslyn
//! 
//! This compiler implements the C# language specification and provides
//! similar APIs to the Roslyn compiler platform.

#![warn(rust_2018_idioms)]
#![warn(missing_docs)]

pub mod lexer;
pub mod parser;
pub mod syntax;
pub mod semantic;
pub mod emit;
pub mod diagnostics;
pub mod compilation;
pub mod workspace;
pub mod assembly_loader;
pub mod codegen;
pub mod compiler;

// Re-export main types
pub use compilation::Compilation;
pub use compiler::{CSharpCompiler, CompilerOptions, OutputType, OptimizationLevel, CompilationResult};

/// C# language versions
#[derive(Debug, Clone, Copy)]
pub enum LanguageVersion {
    /// C# 1.0
    CSharp1,
    /// C# 2.0
    CSharp2,
    /// C# 3.0
    CSharp3,
    /// C# 4.0
    CSharp4,
    /// C# 5.0
    CSharp5,
    /// C# 6.0
    CSharp6,
    /// C# 7.0
    CSharp7,
    /// C# 7.1
    CSharp7_1,
    /// C# 7.2
    CSharp7_2,
    /// C# 7.3
    CSharp7_3,
    /// C# 8.0
    CSharp8,
    /// C# 9.0
    CSharp9,
    /// C# 10.0
    CSharp10,
    /// C# 11.0
    CSharp11,
    /// C# 12.0
    CSharp12,
    /// Latest version
    Latest,
}
pub use diagnostics::{Diagnostic, DiagnosticBag};
pub use syntax::{SyntaxKind, SyntaxNode, SyntaxToken, SyntaxTree};

