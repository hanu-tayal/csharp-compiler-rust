//! Declaration parsing for types, methods, fields, etc.

use crate::lexer::token::TokenKind;
use crate::syntax::{SyntaxNode, SyntaxKind, SyntaxElement};
use super::Parser;

pub struct DeclarationParser<'a, 'p> {
    parser: &'p mut Parser<'a>,
}

impl<'a, 'p> DeclarationParser<'a, 'p> {
    pub fn new(parser: &'p mut Parser<'a>) -> Self {
        Self { parser }
    }
    
    /// Parse a type declaration (class, struct, interface, enum, delegate)
    pub fn parse_type_declaration(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        // Parse attributes
        while self.parser.current_kind() == Some(TokenKind::OpenBracket) {
            children.push(SyntaxElement::Node(self.parse_attribute_list()));
        }
        
        // Parse modifiers
        while self.is_modifier() {
            children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
        }
        
        // Parse the type keyword and declaration
        match self.parser.current_kind() {
            Some(TokenKind::Class) => self.parse_class_declaration(&mut children),
            Some(TokenKind::Struct) => self.parse_struct_declaration(&mut children),
            Some(TokenKind::Interface) => self.parse_interface_declaration(&mut children),
            Some(TokenKind::Enum) => self.parse_enum_declaration(&mut children),
            Some(TokenKind::Delegate) => self.parse_delegate_declaration(&mut children),
            Some(TokenKind::Record) => self.parse_record_declaration(&mut children),
            _ => {
                self.parser.error_and_recover("Expected type declaration");
                return SyntaxNode {
                    kind: SyntaxKind::Error,
                    children,
                };
            }
        }
    }
    
    fn parse_class_declaration(&mut self, children: &mut Vec<SyntaxElement>) -> SyntaxNode {
        // 'class' keyword
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Class, "Expected 'class'")));
        
        // Class name
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Identifier, "Expected class name")));
        
        // Type parameters
        if self.parser.current_kind() == Some(TokenKind::LessThan) {
            children.push(SyntaxElement::Node(self.parse_type_parameter_list()));
        }
        
        // Base list
        if self.parser.current_kind() == Some(TokenKind::Colon) {
            children.push(SyntaxElement::Node(self.parse_base_list()));
        }
        
        // Type parameter constraints
        while self.parser.current_kind() == Some(TokenKind::Where) {
            children.push(SyntaxElement::Node(self.parse_type_parameter_constraint()));
        }
        
        // Class body
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::OpenBrace, "Expected '{'")));
        
        while !self.parser.is_at_end() && self.parser.current_kind() != Some(TokenKind::CloseBrace) {
            children.push(SyntaxElement::Node(self.parse_class_member()));
        }
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::CloseBrace, "Expected '}'")));
        
        SyntaxNode {
            kind: SyntaxKind::ClassDeclaration,
            children: children.clone(),
        }
    }
    
    fn parse_struct_declaration(&mut self, children: &mut Vec<SyntaxElement>) -> SyntaxNode {
        // Similar to class but with 'struct' keyword
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Struct, "Expected 'struct'")));
        
        // Rest is similar to class
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Identifier, "Expected struct name")));
        
        // Type parameters, base list, constraints, body...
        // (Similar implementation to class)
        
        SyntaxNode {
            kind: SyntaxKind::StructDeclaration,
            children: children.clone(),
        }
    }
    
    fn parse_interface_declaration(&mut self, children: &mut Vec<SyntaxElement>) -> SyntaxNode {
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Interface, "Expected 'interface'")));
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Identifier, "Expected interface name")));
        
        // Type parameters, base list, constraints, body...
        
        SyntaxNode {
            kind: SyntaxKind::InterfaceDeclaration,
            children: children.clone(),
        }
    }
    
    fn parse_enum_declaration(&mut self, children: &mut Vec<SyntaxElement>) -> SyntaxNode {
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Enum, "Expected 'enum'")));
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Identifier, "Expected enum name")));
        
        // Optional base type
        if self.parser.current_kind() == Some(TokenKind::Colon) {
            children.push(SyntaxElement::Token(self.parser.advance().unwrap())); // :
            // Parse integral type
            children.push(SyntaxElement::Node(self.parse_type()));
        }
        
        // Enum body
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::OpenBrace, "Expected '{'")));
        
        // Parse enum members
        while !self.parser.is_at_end() && self.parser.current_kind() != Some(TokenKind::CloseBrace) {
            children.push(SyntaxElement::Node(self.parse_enum_member()));
            
            if self.parser.current_kind() == Some(TokenKind::Comma) {
                children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
            } else {
                break;
            }
        }
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::CloseBrace, "Expected '}'")));
        
        SyntaxNode {
            kind: SyntaxKind::EnumDeclaration,
            children: children.clone(),
        }
    }
    
    fn parse_delegate_declaration(&mut self, children: &mut Vec<SyntaxElement>) -> SyntaxNode {
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Delegate, "Expected 'delegate'")));
        
        // Return type
        children.push(SyntaxElement::Node(self.parse_type()));
        
        // Delegate name
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Identifier, "Expected delegate name")));
        
        // Type parameters
        if self.parser.current_kind() == Some(TokenKind::LessThan) {
            children.push(SyntaxElement::Node(self.parse_type_parameter_list()));
        }
        
        // Parameter list
        children.push(SyntaxElement::Node(self.parse_parameter_list()));
        
        // Type parameter constraints
        while self.parser.current_kind() == Some(TokenKind::Where) {
            children.push(SyntaxElement::Node(self.parse_type_parameter_constraint()));
        }
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Semicolon, "Expected ';'")));
        
        SyntaxNode {
            kind: SyntaxKind::DelegateDeclaration,
            children: children.clone(),
        }
    }
    
    fn parse_record_declaration(&mut self, children: &mut Vec<SyntaxElement>) -> SyntaxNode {
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Record, "Expected 'record'")));
        
        // Optional 'class' or 'struct'
        if matches!(self.parser.current_kind(), Some(TokenKind::Class) | Some(TokenKind::Struct)) {
            children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
        }
        
        // Rest is similar to class
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Identifier, "Expected record name")));
        
        SyntaxNode {
            kind: SyntaxKind::ClassDeclaration, // Records are treated as classes
            children: children.clone(),
        }
    }
    
    fn parse_class_member(&mut self) -> SyntaxNode {
        // Parse attributes
        let mut children = Vec::new();
        while self.parser.current_kind() == Some(TokenKind::OpenBracket) {
            children.push(SyntaxElement::Node(self.parse_attribute_list()));
        }
        
        // Parse modifiers
        while self.is_modifier() {
            children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
        }
        
        // Determine member type
        match self.parser.current_kind() {
            Some(TokenKind::Const) => self.parse_const_declaration(children),
            Some(TokenKind::Event) => self.parse_event_declaration(children),
            Some(TokenKind::Class) | Some(TokenKind::Struct) | Some(TokenKind::Interface) |
            Some(TokenKind::Enum) | Some(TokenKind::Delegate) => {
                // Nested type
                self.parse_type_declaration()
            }
            _ => {
                // Could be field, property, method, constructor, destructor, operator, indexer
                self.parse_member_declaration(children)
            }
        }
    }
    
    fn parse_member_declaration(&mut self, mut children: Vec<SyntaxElement>) -> SyntaxNode {
        // This is complex because we need to disambiguate between:
        // - Field: Type Name;
        // - Property: Type Name { get; set; }
        // - Method: Type Name(...) { }
        // - Constructor: Name(...) { }
        // - Destructor: ~Name() { }
        // - Operator: Type operator +(...) { }
        // - Indexer: Type this[...] { get; set; }
        
        // For now, simplified implementation
        if self.parser.current_kind() == Some(TokenKind::Tilde) {
            // Destructor
            children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
            children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Identifier, "Expected destructor name")));
            children.push(SyntaxElement::Node(self.parse_parameter_list()));
            children.push(SyntaxElement::Node(self.parse_block()));
            
            SyntaxNode {
                kind: SyntaxKind::DestructorDeclaration,
                children,
            }
        } else {
            // Try to parse as method/property/field
            // This is a simplified version
            children.push(SyntaxElement::Node(self.parse_type()));
            children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Identifier, "Expected member name")));
            
            match self.parser.current_kind() {
                Some(TokenKind::OpenParen) => {
                    // Method
                    children.push(SyntaxElement::Node(self.parse_parameter_list()));
                    children.push(SyntaxElement::Node(self.parse_block()));
                    SyntaxNode {
                        kind: SyntaxKind::MethodDeclaration,
                        children,
                    }
                }
                Some(TokenKind::OpenBrace) => {
                    // Property
                    children.push(SyntaxElement::Node(self.parse_property_accessors()));
                    SyntaxNode {
                        kind: SyntaxKind::PropertyDeclaration,
                        children,
                    }
                }
                Some(TokenKind::Semicolon) => {
                    // Field
                    children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
                    SyntaxNode {
                        kind: SyntaxKind::FieldDeclaration,
                        children,
                    }
                }
                _ => {
                    self.parser.error("Expected '(', '{', or ';'");
                    SyntaxNode {
                        kind: SyntaxKind::Error,
                        children,
                    }
                }
            }
        }
    }
    
    fn parse_const_declaration(&mut self, mut children: Vec<SyntaxElement>) -> SyntaxNode {
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Const, "Expected 'const'")));
        children.push(SyntaxElement::Node(self.parse_type()));
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Identifier, "Expected const name")));
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Equals, "Expected '='")));
        children.push(SyntaxElement::Node(self.parse_expression()));
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Semicolon, "Expected ';'")));
        
        SyntaxNode {
            kind: SyntaxKind::FieldDeclaration,
            children,
        }
    }
    
    fn parse_event_declaration(&mut self, mut children: Vec<SyntaxElement>) -> SyntaxNode {
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Event, "Expected 'event'")));
        children.push(SyntaxElement::Node(self.parse_type()));
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Identifier, "Expected event name")));
        
        if self.parser.current_kind() == Some(TokenKind::OpenBrace) {
            // Event with accessors
            children.push(SyntaxElement::Node(self.parse_event_accessors()));
        } else {
            // Field-like event
            children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Semicolon, "Expected ';'")));
        }
        
        SyntaxNode {
            kind: SyntaxKind::EventDeclaration,
            children,
        }
    }
    
    fn parse_enum_member(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        // Attributes
        while self.parser.current_kind() == Some(TokenKind::OpenBracket) {
            children.push(SyntaxElement::Node(self.parse_attribute_list()));
        }
        
        // Member name
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Identifier, "Expected enum member name")));
        
        // Optional value
        if self.parser.current_kind() == Some(TokenKind::Equals) {
            children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
            children.push(SyntaxElement::Node(self.parse_expression()));
        }
        
        SyntaxNode {
            kind: SyntaxKind::EnumMemberDeclaration,
            children,
        }
    }
    
    // Helper methods
    
    fn is_modifier(&self) -> bool {
        matches!(self.parser.current_kind(),
            Some(TokenKind::Public) | Some(TokenKind::Private) | Some(TokenKind::Protected) |
            Some(TokenKind::Internal) | Some(TokenKind::Static) | Some(TokenKind::Virtual) |
            Some(TokenKind::Abstract) | Some(TokenKind::Sealed) | Some(TokenKind::Override) |
            Some(TokenKind::Readonly) | Some(TokenKind::Unsafe) | Some(TokenKind::Volatile) |
            Some(TokenKind::Async) | Some(TokenKind::Partial) | Some(TokenKind::New) |
            Some(TokenKind::Extern)
        )
    }
    
    pub fn parse_attribute_list(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::OpenBracket, "Expected '['")));
        
        // Parse attributes
        loop {
            children.push(SyntaxElement::Node(self.parse_attribute()));
            
            if self.parser.current_kind() == Some(TokenKind::Comma) {
                children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
            } else {
                break;
            }
        }
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::CloseBracket, "Expected ']'")));
        
        SyntaxNode {
            kind: SyntaxKind::AttributeList,
            children,
        }
    }
    
    fn parse_attribute(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        // Attribute name
        children.push(SyntaxElement::Node(self.parser.parse_qualified_name()));
        
        // Optional arguments
        if self.parser.current_kind() == Some(TokenKind::OpenParen) {
            children.push(SyntaxElement::Node(self.parse_attribute_arguments()));
        }
        
        SyntaxNode {
            kind: SyntaxKind::Attribute,
            children,
        }
    }
    
    fn parse_attribute_arguments(&mut self) -> SyntaxNode {
        // Similar to method arguments
        self.parse_argument_list()
    }
    
    fn parse_type_parameter_list(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::LessThan, "Expected '<'")));
        
        loop {
            children.push(SyntaxElement::Node(self.parse_type_parameter()));
            
            if self.parser.current_kind() == Some(TokenKind::Comma) {
                children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
            } else {
                break;
            }
        }
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::GreaterThan, "Expected '>'")));
        
        SyntaxNode {
            kind: SyntaxKind::TypeParameterList,
            children,
        }
    }
    
    fn parse_type_parameter(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        // Attributes
        while self.parser.current_kind() == Some(TokenKind::OpenBracket) {
            children.push(SyntaxElement::Node(self.parse_attribute_list()));
        }
        
        // Variance (in/out)
        if matches!(self.parser.current_kind(), Some(TokenKind::In) | Some(TokenKind::Out)) {
            children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
        }
        
        // Type parameter name
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Identifier, "Expected type parameter name")));
        
        SyntaxNode {
            kind: SyntaxKind::TypeParameter,
            children,
        }
    }
    
    fn parse_base_list(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Colon, "Expected ':'")));
        
        loop {
            children.push(SyntaxElement::Node(self.parse_type()));
            
            if self.parser.current_kind() == Some(TokenKind::Comma) {
                children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
            } else {
                break;
            }
        }
        
        SyntaxNode {
            kind: SyntaxKind::BaseList,
            children,
        }
    }
    
    fn parse_type_parameter_constraint(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Where, "Expected 'where'")));
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Identifier, "Expected type parameter name")));
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Colon, "Expected ':'")));
        
        // Parse constraints
        loop {
            if self.parser.current_kind() == Some(TokenKind::Class) {
                children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
            } else if self.parser.current_kind() == Some(TokenKind::Struct) {
                children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
            } else if self.parser.current_kind() == Some(TokenKind::New) &&
                      self.parser.peek_kind(1) == Some(TokenKind::OpenParen) {
                children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
                children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
                children.push(SyntaxElement::Token(self.parser.expect(TokenKind::CloseParen, "Expected ')'")));
            } else {
                children.push(SyntaxElement::Node(self.parse_type()));
            }
            
            if self.parser.current_kind() == Some(TokenKind::Comma) {
                children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
            } else {
                break;
            }
        }
        
        SyntaxNode {
            kind: SyntaxKind::TypeConstraintClause,
            children,
        }
    }
    
    fn parse_parameter_list(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::OpenParen, "Expected '('")));
        
        if self.parser.current_kind() != Some(TokenKind::CloseParen) {
            loop {
                children.push(SyntaxElement::Node(self.parse_parameter()));
                
                if self.parser.current_kind() == Some(TokenKind::Comma) {
                    children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
                } else {
                    break;
                }
            }
        }
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::CloseParen, "Expected ')'")));
        
        SyntaxNode {
            kind: SyntaxKind::ParameterList,
            children,
        }
    }
    
    fn parse_parameter(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        // Attributes
        while self.parser.current_kind() == Some(TokenKind::OpenBracket) {
            children.push(SyntaxElement::Node(self.parse_attribute_list()));
        }
        
        // Parameter modifiers (ref, out, in, this, params)
        if matches!(self.parser.current_kind(), 
            Some(TokenKind::Ref) | Some(TokenKind::Out) | Some(TokenKind::In) | 
            Some(TokenKind::This) | Some(TokenKind::Params)) {
            children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
        }
        
        // Type
        children.push(SyntaxElement::Node(self.parse_type()));
        
        // Name
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Identifier, "Expected parameter name")));
        
        // Default value
        if self.parser.current_kind() == Some(TokenKind::Equals) {
            children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
            children.push(SyntaxElement::Node(self.parse_expression()));
        }
        
        SyntaxNode {
            kind: SyntaxKind::Parameter,
            children,
        }
    }
    
    fn parse_property_accessors(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::OpenBrace, "Expected '{'")));
        
        while self.parser.current_kind() != Some(TokenKind::CloseBrace) {
            // Attributes
            while self.parser.current_kind() == Some(TokenKind::OpenBracket) {
                children.push(SyntaxElement::Node(self.parse_attribute_list()));
            }
            
            // Modifiers
            while self.is_modifier() {
                children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
            }
            
            // get/set/init
            if matches!(self.parser.current_kind(), Some(TokenKind::Get) | Some(TokenKind::Set) | Some(TokenKind::Init)) {
                children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
                
                // Body or semicolon
                if self.parser.current_kind() == Some(TokenKind::OpenBrace) {
                    children.push(SyntaxElement::Node(self.parse_block()));
                } else if self.parser.current_kind() == Some(TokenKind::FatArrow) {
                    // Expression-bodied accessor
                    children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
                    children.push(SyntaxElement::Node(self.parse_expression()));
                    children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Semicolon, "Expected ';'")));
                } else {
                    children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Semicolon, "Expected ';'")));
                }
            } else {
                self.parser.error("Expected 'get', 'set', or 'init'");
                break;
            }
        }
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::CloseBrace, "Expected '}'")));
        
        SyntaxNode {
            kind: SyntaxKind::AccessorList,
            children,
        }
    }
    
    fn parse_event_accessors(&mut self) -> SyntaxNode {
        // Similar to property accessors but with add/remove
        let mut children = Vec::new();
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::OpenBrace, "Expected '{'")));
        
        while self.parser.current_kind() != Some(TokenKind::CloseBrace) {
            if matches!(self.parser.current_kind(), Some(TokenKind::Add) | Some(TokenKind::Remove)) {
                children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
                children.push(SyntaxElement::Node(self.parse_block()));
            } else {
                self.parser.error("Expected 'add' or 'remove'");
                break;
            }
        }
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::CloseBrace, "Expected '}'")));
        
        SyntaxNode {
            kind: SyntaxKind::AccessorList,
            children,
        }
    }
    
    pub fn parse_argument_list(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::OpenParen, "Expected '('")));
        
        if self.parser.current_kind() != Some(TokenKind::CloseParen) {
            loop {
                children.push(SyntaxElement::Node(self.parse_argument()));
                
                if self.parser.current_kind() == Some(TokenKind::Comma) {
                    children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
                } else {
                    break;
                }
            }
        }
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::CloseParen, "Expected ')'")));
        
        SyntaxNode {
            kind: SyntaxKind::ArgumentList,
            children,
        }
    }
    
    fn parse_argument(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        // Named argument or ref/out/in
        if self.parser.current_kind() == Some(TokenKind::Identifier) &&
           self.parser.peek_kind(1) == Some(TokenKind::Colon) {
            // Named argument
            children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
            children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
        } else if matches!(self.parser.current_kind(), Some(TokenKind::Ref) | Some(TokenKind::Out) | Some(TokenKind::In)) {
            children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
        }
        
        // Expression
        children.push(SyntaxElement::Node(self.parse_expression()));
        
        SyntaxNode {
            kind: SyntaxKind::Argument,
            children,
        }
    }
    
    // Stub methods - these would be implemented in other modules
    
    fn parse_type(&mut self) -> SyntaxNode {
        // This would be implemented by TypeParser
        super::types::TypeParser::new(self.parser).parse_type()
    }
    
    fn parse_expression(&mut self) -> SyntaxNode {
        // This would be implemented by ExpressionParser
        super::expressions::ExpressionParser::new(self.parser).parse_expression()
    }
    
    fn parse_block(&mut self) -> SyntaxNode {
        // This would be implemented by StatementParser
        super::statements::StatementParser::new(self.parser).parse_block()
    }
}