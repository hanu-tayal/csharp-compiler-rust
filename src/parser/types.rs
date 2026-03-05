//! Type parsing for C#

use crate::lexer::token::TokenKind;
use crate::syntax::{SyntaxNode, SyntaxKind, SyntaxElement};
use super::Parser;

pub struct TypeParser<'a, 'p> {
    parser: &'p mut Parser<'a>,
}

impl<'a, 'p> TypeParser<'a, 'p> {
    pub fn new(parser: &'p mut Parser<'a>) -> Self {
        Self { parser }
    }
    
    /// Parse any type
    pub fn parse_type(&mut self) -> SyntaxNode {
        let base_type = self.parse_non_nullable_type();
        
        // Check for nullable type (?)
        if self.parser.current_kind() == Some(TokenKind::Question) {
            let question = self.parser.advance().unwrap();
            SyntaxNode {
                kind: SyntaxKind::NullableType,
                children: vec![
                    SyntaxElement::Node(base_type),
                    SyntaxElement::Token(question),
                ],
            }
        } else {
            base_type
        }
    }
    
    /// Parse non-nullable type
    fn parse_non_nullable_type(&mut self) -> SyntaxNode {
        // Check for array or pointer after parsing base type
        let mut base_type = self.parse_base_type();
        
        loop {
            match self.parser.current_kind() {
                Some(TokenKind::OpenBracket) => {
                    // Array type
                    let open_bracket = self.parser.advance().unwrap();
                    let mut rank_specifiers = vec![SyntaxElement::Token(open_bracket)];
                    
                    // Count commas for array rank
                    while self.parser.current_kind() == Some(TokenKind::Comma) {
                        rank_specifiers.push(SyntaxElement::Token(self.parser.advance().unwrap()));
                    }
                    
                    let close_bracket = self.parser.expect(TokenKind::CloseBracket, "Expected ']'");
                    rank_specifiers.push(SyntaxElement::Token(close_bracket));
                    
                    base_type = SyntaxNode {
                        kind: SyntaxKind::ArrayType,
                        children: vec![
                            SyntaxElement::Node(base_type),
                            SyntaxElement::Node(SyntaxNode {
                                kind: SyntaxKind::ArrayRankSpecifier,
                                children: rank_specifiers,
                            }),
                        ],
                    };
                }
                Some(TokenKind::Star) => {
                    // Pointer type
                    let star = self.parser.advance().unwrap();
                    base_type = SyntaxNode {
                        kind: SyntaxKind::PointerType,
                        children: vec![
                            SyntaxElement::Node(base_type),
                            SyntaxElement::Token(star),
                        ],
                    };
                }
                _ => break,
            }
        }
        
        base_type
    }
    
    /// Parse base type (predefined, named, or tuple)
    fn parse_base_type(&mut self) -> SyntaxNode {
        match self.parser.current_kind() {
            // Predefined types
            Some(TokenKind::Bool) | Some(TokenKind::Byte) | Some(TokenKind::Sbyte) |
            Some(TokenKind::Short) | Some(TokenKind::Ushort) | Some(TokenKind::Int) |
            Some(TokenKind::Uint) | Some(TokenKind::Long) | Some(TokenKind::Ulong) |
            Some(TokenKind::Char) | Some(TokenKind::Float) | Some(TokenKind::Double) |
            Some(TokenKind::Decimal) | Some(TokenKind::String) | Some(TokenKind::Object) => {
                let keyword = self.parser.advance().unwrap();
                SyntaxNode {
                    kind: SyntaxKind::PredefinedType,
                    children: vec![SyntaxElement::Token(keyword)],
                }
            }
            Some(TokenKind::Void) => {
                let keyword = self.parser.advance().unwrap();
                SyntaxNode {
                    kind: SyntaxKind::PredefinedType,
                    children: vec![SyntaxElement::Token(keyword)],
                }
            }
            Some(TokenKind::Var) => {
                // Implicitly typed (var)
                let var_token = self.parser.advance().unwrap();
                SyntaxNode {
                    kind: SyntaxKind::IdentifierName,
                    children: vec![SyntaxElement::Token(var_token)],
                }
            }
            Some(TokenKind::Dynamic) => {
                // Dynamic type
                let dynamic_token = self.parser.advance().unwrap();
                SyntaxNode {
                    kind: SyntaxKind::IdentifierName,
                    children: vec![SyntaxElement::Token(dynamic_token)],
                }
            }
            Some(TokenKind::OpenParen) => {
                // Tuple type
                self.parse_tuple_type()
            }
            Some(TokenKind::Identifier) | Some(TokenKind::Global) => {
                // Named type (possibly qualified and/or generic)
                self.parse_named_type()
            }
            _ => {
                self.parser.error("Expected type");
                SyntaxNode {
                    kind: SyntaxKind::Error,
                    children: vec![],
                }
            }
        }
    }
    
    /// Parse named type (with qualification and generics)
    fn parse_named_type(&mut self) -> SyntaxNode {
        let mut parts = Vec::new();
        
        // Handle 'global::' prefix
        if self.parser.current_kind() == Some(TokenKind::Global) &&
           self.parser.peek_kind(1) == Some(TokenKind::ColonColon) {
            parts.push(self.parser.advance().unwrap()); // global
            parts.push(self.parser.advance().unwrap()); // ::
        }
        
        // Parse first identifier
        let first_name = self.parser.expect(TokenKind::Identifier, "Expected type name");
        
        // Check for generic arguments
        let first_part = if self.parser.current_kind() == Some(TokenKind::LessThan) && self.is_generic_arguments() {
            let type_args = self.parse_type_argument_list();
            SyntaxNode {
                kind: SyntaxKind::GenericName,
                children: vec![
                    SyntaxElement::Token(first_name),
                    SyntaxElement::Node(type_args),
                ],
            }
        } else {
            SyntaxNode {
                kind: SyntaxKind::IdentifierName,
                children: vec![SyntaxElement::Token(first_name)],
            }
        };
        
        let mut current = first_part;
        
        // Parse additional qualified parts
        while self.parser.current_kind() == Some(TokenKind::Dot) {
            let dot = self.parser.advance().unwrap();
            let name = self.parser.expect(TokenKind::Identifier, "Expected identifier after '.'");
            
            let right = if self.parser.current_kind() == Some(TokenKind::LessThan) && self.is_generic_arguments() {
                let type_args = self.parse_type_argument_list();
                SyntaxNode {
                    kind: SyntaxKind::GenericName,
                    children: vec![
                        SyntaxElement::Token(name),
                        SyntaxElement::Node(type_args),
                    ],
                }
            } else {
                SyntaxNode {
                    kind: SyntaxKind::IdentifierName,
                    children: vec![SyntaxElement::Token(name)],
                }
            };
            
            current = SyntaxNode {
                kind: SyntaxKind::QualifiedName,
                children: vec![
                    SyntaxElement::Node(current),
                    SyntaxElement::Token(dot),
                    SyntaxElement::Node(right),
                ],
            };
        }
        
        // If we had global:: prefix, wrap in AliasQualifiedName
        if !parts.is_empty() {
            let mut children = vec![];
            for part in parts {
                children.push(SyntaxElement::Token(part));
            }
            children.push(SyntaxElement::Node(current));
            
            SyntaxNode {
                kind: SyntaxKind::AliasQualifiedName,
                children,
            }
        } else {
            current
        }
    }
    
    /// Parse tuple type
    fn parse_tuple_type(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::OpenParen, "Expected '('")));
        
        // Parse tuple elements
        loop {
            let element_type = self.parse_type();
            
            // Optional element name
            if self.parser.current_kind() == Some(TokenKind::Identifier) {
                let name = self.parser.advance().unwrap();
                children.push(SyntaxElement::Node(SyntaxNode {
                    kind: SyntaxKind::TupleElement,
                    children: vec![
                        SyntaxElement::Node(element_type),
                        SyntaxElement::Token(name),
                    ],
                }));
            } else {
                children.push(SyntaxElement::Node(SyntaxNode {
                    kind: SyntaxKind::TupleElement,
                    children: vec![SyntaxElement::Node(element_type)],
                }));
            }
            
            if self.parser.current_kind() == Some(TokenKind::Comma) {
                children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
            } else {
                break;
            }
        }
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::CloseParen, "Expected ')'")));
        
        SyntaxNode {
            kind: SyntaxKind::TupleType,
            children,
        }
    }
    
    /// Parse type argument list
    fn parse_type_argument_list(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::LessThan, "Expected '<'")));
        
        loop {
            children.push(SyntaxElement::Node(self.parse_type()));
            
            if self.parser.current_kind() == Some(TokenKind::Comma) {
                children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
            } else {
                break;
            }
        }
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::GreaterThan, "Expected '>'")));
        
        SyntaxNode {
            kind: SyntaxKind::TypeArgumentList,
            children,
        }
    }
    
    /// Check if '<' starts generic arguments
    fn is_generic_arguments(&self) -> bool {
        // This is a simplified check
        // In reality, we'd need more sophisticated lookahead to distinguish
        // between generic arguments and less-than operators
        true
    }
}