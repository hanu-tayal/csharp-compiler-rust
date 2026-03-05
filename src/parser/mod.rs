//! Parser for C# source code
//! 
//! Converts tokens into an abstract syntax tree (AST).

use crate::lexer::{Lexer, token::TokenKind};
use crate::syntax::{SyntaxNode, SyntaxTree, SyntaxKind, SyntaxToken, SyntaxElement};
use crate::diagnostics::{DiagnosticBag, DiagnosticCode};
use text_size::TextRange;
use smol_str::SmolStr;

pub mod expressions;
pub mod statements;
pub mod declarations;
pub mod types;

use self::expressions::ExpressionParser;
use self::statements::StatementParser;
use self::declarations::DeclarationParser;
use self::types::TypeParser;

/// The C# parser
pub struct Parser<'a> {
    /// Token stream from lexer
    tokens: Vec<SyntaxToken>,
    /// Current position in token stream
    position: usize,
    /// Diagnostics
    diagnostics: DiagnosticBag,
    /// Parser configuration
    config: ParserConfig,
    /// Phantom data for lifetime
    _phantom: std::marker::PhantomData<&'a ()>,
}

/// Parser configuration
#[derive(Debug, Clone)]
pub struct ParserConfig {
    /// Allow incomplete constructs (for IDE scenarios)
    pub allow_incomplete: bool,
    /// Language version
    pub language_version: crate::LanguageVersion,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            allow_incomplete: false,
            language_version: crate::LanguageVersion::Latest,
        }
    }
}

impl<'a> Parser<'a> {
    /// Create a new parser for the given source
    pub fn new(source: &'a str) -> Self {
        Self::with_config(source, ParserConfig::default())
    }
    
    /// Create a new parser with custom configuration
    pub fn with_config(source: &'a str, config: ParserConfig) -> Self {
        // First, lex all tokens
        let mut lexer = Lexer::new(source);
        let mut tokens = Vec::new();
        
        while let Some(token) = lexer.next_token() {
            tokens.push(token);
        }
        
        Self {
            tokens,
            position: 0,
            diagnostics: DiagnosticBag::new(),
            config,
            _phantom: std::marker::PhantomData,
        }
    }
    
    /// Parse the source into a syntax tree
    pub fn parse(&mut self) -> SyntaxTree {
        let root = self.parse_compilation_unit();
        SyntaxTree { root }
    }
    
    /// Get the diagnostics
    pub fn diagnostics(&self) -> &DiagnosticBag {
        &self.diagnostics
    }
    
    /// Parse a compilation unit (top-level file structure)
    fn parse_compilation_unit(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        // Parse extern aliases
        while self.current_kind() == Some(TokenKind::Extern) && 
              self.peek_kind(1) == Some(TokenKind::Alias) {
            children.push(SyntaxElement::Node(self.parse_extern_alias()));
        }
        
        // Parse using directives
        while self.current_kind() == Some(TokenKind::Using) {
            children.push(SyntaxElement::Node(self.parse_using_directive()));
        }
        
        // Parse global attributes
        while self.current_kind() == Some(TokenKind::OpenBracket) {
            if self.is_global_attribute() {
                children.push(SyntaxElement::Node(self.parse_global_attribute()));
            } else {
                break;
            }
        }
        
        // Parse namespace members (namespaces, types, etc.)
        while !self.is_at_end() {
            match self.current_kind() {
                Some(TokenKind::Namespace) => {
                    children.push(SyntaxElement::Node(self.parse_namespace()));
                }
                Some(TokenKind::Public) | Some(TokenKind::Private) | Some(TokenKind::Protected) |
                Some(TokenKind::Internal) | Some(TokenKind::Static) | Some(TokenKind::Abstract) |
                Some(TokenKind::Sealed) | Some(TokenKind::Partial) | Some(TokenKind::Class) |
                Some(TokenKind::Struct) | Some(TokenKind::Interface) | Some(TokenKind::Enum) |
                Some(TokenKind::Delegate) | Some(TokenKind::Record) => {
                    children.push(SyntaxElement::Node(self.parse_type_declaration()));
                }
                Some(TokenKind::OpenBracket) => {
                    // Attribute on type declaration
                    children.push(SyntaxElement::Node(self.parse_type_declaration()));
                }
                _ => {
                    // Unexpected token
                    self.error_and_recover("Expected namespace or type declaration");
                }
            }
        }
        
        SyntaxNode {
            kind: SyntaxKind::CompilationUnit,
            children,
        }
    }
    
    // Helper methods
    
    /// Get the current token
    fn current(&self) -> Option<&SyntaxToken> {
        self.tokens.get(self.position)
    }
    
    /// Get the current token kind
    fn current_kind(&self) -> Option<TokenKind> {
        self.current().map(|t| self.syntax_kind_to_token_kind(t.kind))
    }
    
    /// Convert SyntaxKind to TokenKind
    fn syntax_kind_to_token_kind(&self, kind: SyntaxKind) -> TokenKind {
        match kind {
            // Keywords
            SyntaxKind::ClassKeyword => TokenKind::Class,
            SyntaxKind::StructKeyword => TokenKind::Struct,
            SyntaxKind::InterfaceKeyword => TokenKind::Interface,
            SyntaxKind::EnumKeyword => TokenKind::Enum,
            SyntaxKind::DelegateKeyword => TokenKind::Delegate,
            SyntaxKind::NamespaceKeyword => TokenKind::Namespace,
            SyntaxKind::UsingKeyword => TokenKind::Using,
            SyntaxKind::PublicKeyword => TokenKind::Public,
            SyntaxKind::PrivateKeyword => TokenKind::Private,
            SyntaxKind::ProtectedKeyword => TokenKind::Protected,
            SyntaxKind::InternalKeyword => TokenKind::Internal,
            SyntaxKind::StaticKeyword => TokenKind::Static,
            SyntaxKind::AbstractKeyword => TokenKind::Abstract,
            SyntaxKind::SealedKeyword => TokenKind::Sealed,
            SyntaxKind::VirtualKeyword => TokenKind::Virtual,
            SyntaxKind::OverrideKeyword => TokenKind::Override,
            SyntaxKind::PartialKeyword => TokenKind::Partial,
            SyntaxKind::ConstKeyword => TokenKind::Const,
            SyntaxKind::ReadonlyKeyword => TokenKind::Readonly,
            SyntaxKind::VoidKeyword => TokenKind::Void,
            SyntaxKind::IntKeyword => TokenKind::Int,
            SyntaxKind::StringKeyword => TokenKind::String,
            SyntaxKind::BoolKeyword => TokenKind::Bool,
            SyntaxKind::ObjectKeyword => TokenKind::Object,
            
            // Tokens
            SyntaxKind::OpenBraceToken => TokenKind::OpenBrace,
            SyntaxKind::CloseBraceToken => TokenKind::CloseBrace,
            SyntaxKind::OpenBracketToken => TokenKind::OpenBracket,
            SyntaxKind::CloseBracketToken => TokenKind::CloseBracket,
            SyntaxKind::OpenParenToken => TokenKind::OpenParen,
            SyntaxKind::CloseParenToken => TokenKind::CloseParen,
            SyntaxKind::SemicolonToken => TokenKind::Semicolon,
            SyntaxKind::ColonToken => TokenKind::Colon,
            SyntaxKind::CommaToken => TokenKind::Comma,
            SyntaxKind::DotToken => TokenKind::Dot,
            SyntaxKind::EqualsToken => TokenKind::Equals,
            SyntaxKind::LessThanToken => TokenKind::LessThan,
            SyntaxKind::GreaterThanToken => TokenKind::GreaterThan,
            SyntaxKind::PlusToken => TokenKind::Plus,
            SyntaxKind::MinusToken => TokenKind::Minus,
            SyntaxKind::IdentifierToken => TokenKind::Identifier,
            SyntaxKind::EndOfFile => TokenKind::EndOfFile,
            
            _ => TokenKind::Error,
        }
    }
    
    /// Peek at a token ahead
    fn peek_kind(&self, offset: usize) -> Option<TokenKind> {
        self.tokens.get(self.position + offset)
            .map(|t| self.syntax_kind_to_token_kind(t.kind))
    }
    
    /// Check if at end of tokens
    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
    }
    
    /// Advance to the next token
    fn advance(&mut self) -> Option<SyntaxToken> {
        if !self.is_at_end() {
            let token = self.tokens[self.position].clone();
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }
    
    /// Consume a token of the expected kind
    fn consume(&mut self, kind: TokenKind, message: &str) -> Option<SyntaxToken> {
        if self.current_kind() == Some(kind) {
            self.advance()
        } else {
            self.error(message);
            None
        }
    }
    
    /// Report an error
    fn error(&mut self, message: &str) {
        let range = self.current()
            .map(|t| t.range)
            .unwrap_or_else(|| TextRange::default());
        self.diagnostics.add_error(message, range);
    }
    
    /// Consume a token or create an error token
    fn expect(&mut self, kind: TokenKind, message: &str) -> SyntaxToken {
        self.consume(kind, message).unwrap_or_else(|| {
            // Create a synthetic error token
            SyntaxToken {
                kind: SyntaxKind::from(TokenKind::Error),
                text: SmolStr::new(""),
                range: self.current()
                    .map(|t| t.range)
                    .unwrap_or_else(|| TextRange::default()),
                leading_trivia: Vec::new(),
                trailing_trivia: Vec::new(),
            }
        })
    }
    
    /// Report an error and attempt recovery
    fn error_and_recover(&mut self, message: &str) {
        self.error(message);
        self.synchronize();
    }
    
    /// Synchronize parser after error
    fn synchronize(&mut self) {
        self.advance();
        
        while !self.is_at_end() {
            // Stop at statement/declaration boundaries
            match self.current_kind() {
                Some(TokenKind::Class) | Some(TokenKind::Struct) | 
                Some(TokenKind::Interface) | Some(TokenKind::Enum) |
                Some(TokenKind::Delegate) | Some(TokenKind::Namespace) |
                Some(TokenKind::Public) | Some(TokenKind::Private) |
                Some(TokenKind::Protected) | Some(TokenKind::Internal) => break,
                _ => {}
            }
            
            self.advance();
        }
    }
    
    // Parsing methods (stubs for now)
    
    fn parse_extern_alias(&mut self) -> SyntaxNode {
        todo!("parse_extern_alias")
    }
    
    fn parse_using_directive(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        // 'using' keyword
        children.push(SyntaxElement::Token(self.expect(TokenKind::Using, "Expected 'using'")));
        
        // Check for 'static' or alias
        if self.current_kind() == Some(TokenKind::Static) {
            children.push(SyntaxElement::Token(self.advance().unwrap()));
        } else if self.current_kind() == Some(TokenKind::Identifier) && 
                  self.peek_kind(1) == Some(TokenKind::Equals) {
            // Using alias: using Alias = Namespace.Type;
            children.push(SyntaxElement::Token(self.advance().unwrap())); // alias name
            children.push(SyntaxElement::Token(self.advance().unwrap())); // =
        }
        
        // Parse the namespace or type name
        children.push(SyntaxElement::Node(self.parse_qualified_name()));
        
        // Semicolon
        children.push(SyntaxElement::Token(self.expect(TokenKind::Semicolon, "Expected ';'")));
        
        SyntaxNode {
            kind: SyntaxKind::UsingDirective,
            children,
        }
    }
    
    fn parse_global_attribute(&mut self) -> SyntaxNode {
        todo!("parse_global_attribute")
    }
    
    fn is_global_attribute(&self) -> bool {
        // Check if this is [assembly: ...] or [module: ...]
        false // Simplified
    }
    
    fn parse_namespace(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        // 'namespace' keyword
        children.push(SyntaxElement::Token(self.expect(TokenKind::Namespace, "Expected 'namespace'")));
        
        // Namespace name
        children.push(SyntaxElement::Node(self.parse_qualified_name()));
        
        // Body
        children.push(SyntaxElement::Token(self.expect(TokenKind::OpenBrace, "Expected '{'")));
        
        // Parse namespace members
        while !self.is_at_end() && self.current_kind() != Some(TokenKind::CloseBrace) {
            match self.current_kind() {
                Some(TokenKind::Using) => {
                    children.push(SyntaxElement::Node(self.parse_using_directive()));
                }
                Some(TokenKind::Namespace) => {
                    children.push(SyntaxElement::Node(self.parse_namespace()));
                }
                _ => {
                    children.push(SyntaxElement::Node(self.parse_type_declaration()));
                }
            }
        }
        
        children.push(SyntaxElement::Token(self.expect(TokenKind::CloseBrace, "Expected '}'")));
        
        SyntaxNode {
            kind: SyntaxKind::NamespaceDeclaration,
            children,
        }
    }
    
    fn parse_type_declaration(&mut self) -> SyntaxNode {
        // This will be implemented by DeclarationParser
        DeclarationParser::new(self).parse_type_declaration()
    }
    
    fn parse_qualified_name(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        // First part
        children.push(SyntaxElement::Token(self.expect(TokenKind::Identifier, "Expected identifier")));
        
        // Additional parts separated by dots
        while self.current_kind() == Some(TokenKind::Dot) {
            children.push(SyntaxElement::Token(self.advance().unwrap())); // .
            children.push(SyntaxElement::Token(self.expect(TokenKind::Identifier, "Expected identifier")));
        }
        
        SyntaxNode {
            kind: if children.len() > 1 { SyntaxKind::QualifiedName } else { SyntaxKind::IdentifierName },
            children,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_using_directive() {
        let mut parser = Parser::new("using System;");
        let tree = parser.parse();
        // Basic structure test
        assert_eq!(tree.root.kind, SyntaxKind::CompilationUnit);
    }
    
    #[test]
    fn test_parse_namespace() {
        let source = r#"
            namespace MyApp {
                using System;
                
                public class Program {
                }
            }
        "#;
        let mut parser = Parser::new(source);
        let tree = parser.parse();
        assert_eq!(tree.root.kind, SyntaxKind::CompilationUnit);
    }
}