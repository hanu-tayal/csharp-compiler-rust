//! Syntax tree representation for C#
//! 
//! This module defines the syntax tree structure used throughout the compiler.

use smol_str::SmolStr;
use text_size::TextRange;
use crate::lexer::Trivia;

pub mod kind;
pub mod node;
pub mod tree;
pub mod visitor;

pub use self::kind::SyntaxKind;
pub use self::node::{SyntaxNode, SyntaxElement};
pub use self::tree::SyntaxTree;

/// A token in the syntax tree
#[derive(Debug, Clone)]
pub struct SyntaxToken {
    /// The kind of token
    pub kind: SyntaxKind,
    /// The text of the token
    pub text: SmolStr,
    /// The range in the source text
    pub range: TextRange,
    /// Leading trivia (whitespace, comments before the token)
    pub leading_trivia: Vec<Trivia>,
    /// Trailing trivia (whitespace, comments after the token)
    pub trailing_trivia: Vec<Trivia>,
}

impl SyntaxToken {
    /// Get the full span including trivia
    pub fn full_range(&self) -> TextRange {
        let start = self.leading_trivia
            .first()
            .map(|t| t.range.start())
            .unwrap_or(self.range.start());
        
        let end = self.trailing_trivia
            .last()
            .map(|t| t.range.end())
            .unwrap_or(self.range.end());
        
        TextRange::new(start, end)
    }
    
    /// Check if this token is a keyword
    pub fn is_keyword(&self) -> bool {
        self.kind.is_keyword()
    }
    
    /// Check if this token is an operator
    pub fn is_operator(&self) -> bool {
        self.kind.is_operator()
    }
}