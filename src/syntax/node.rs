//! Syntax node implementation

use crate::syntax::SyntaxKind;

/// A node in the syntax tree
#[derive(Debug, Clone)]
pub struct SyntaxNode {
    pub kind: SyntaxKind,
    pub children: Vec<SyntaxElement>,
}

/// Either a node or a token
#[derive(Debug, Clone)]
pub enum SyntaxElement {
    Node(SyntaxNode),
    Token(crate::syntax::SyntaxToken),
}