//! Syntax tree representation

use crate::syntax::SyntaxNode;

/// A complete syntax tree
#[derive(Debug)]
pub struct SyntaxTree {
    pub root: SyntaxNode,
}