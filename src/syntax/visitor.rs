//! Visitor pattern for syntax trees

use crate::syntax::{SyntaxNode, SyntaxElement};

/// Visitor trait for walking syntax trees
pub trait SyntaxVisitor {
    fn visit_node(&mut self, node: &SyntaxNode) {
        for child in &node.children {
            match child {
                SyntaxElement::Node(n) => self.visit_node(n),
                SyntaxElement::Token(t) => self.visit_token(t),
            }
        }
    }
    
    fn visit_token(&mut self, token: &crate::syntax::SyntaxToken) {
        // Default: do nothing
    }
}