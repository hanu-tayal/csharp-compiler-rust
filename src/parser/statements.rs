//! Statement parsing for C#

use crate::lexer::token::TokenKind;
use crate::syntax::{SyntaxNode, SyntaxKind, SyntaxElement};
use super::Parser;

pub struct StatementParser<'a, 'p> {
    parser: &'p mut Parser<'a>,
}

impl<'a, 'p> StatementParser<'a, 'p> {
    pub fn new(parser: &'p mut Parser<'a>) -> Self {
        Self { parser }
    }
    
    /// Parse a block statement
    pub fn parse_block(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::OpenBrace, "Expected '{'")));
        
        while !self.parser.is_at_end() && self.parser.current_kind() != Some(TokenKind::CloseBrace) {
            children.push(SyntaxElement::Node(self.parse_statement()));
        }
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::CloseBrace, "Expected '}'")));
        
        SyntaxNode {
            kind: SyntaxKind::Block,
            children,
        }
    }
    
    /// Parse any statement
    pub fn parse_statement(&mut self) -> SyntaxNode {
        // Check for attributes
        let mut attributes = Vec::new();
        while self.parser.current_kind() == Some(TokenKind::OpenBracket) {
            attributes.push(self.parse_attribute_list());
        }
        
        let statement = match self.parser.current_kind() {
            // Block
            Some(TokenKind::OpenBrace) => self.parse_block(),
            
            // Empty statement
            Some(TokenKind::Semicolon) => {
                let semi = self.parser.advance().unwrap();
                SyntaxNode {
                    kind: SyntaxKind::EmptyStatement,
                    children: vec![SyntaxElement::Token(semi)],
                }
            }
            
            // Control flow statements
            Some(TokenKind::If) => self.parse_if_statement(),
            Some(TokenKind::Switch) => self.parse_switch_statement(),
            Some(TokenKind::While) => self.parse_while_statement(),
            Some(TokenKind::Do) => self.parse_do_statement(),
            Some(TokenKind::For) => self.parse_for_statement(),
            Some(TokenKind::Foreach) => self.parse_foreach_statement(),
            Some(TokenKind::Break) => self.parse_break_statement(),
            Some(TokenKind::Continue) => self.parse_continue_statement(),
            Some(TokenKind::Goto) => self.parse_goto_statement(),
            Some(TokenKind::Return) => self.parse_return_statement(),
            Some(TokenKind::Throw) => self.parse_throw_statement(),
            Some(TokenKind::Try) => self.parse_try_statement(),
            Some(TokenKind::Lock) => self.parse_lock_statement(),
            Some(TokenKind::Using) => self.parse_using_statement(),
            Some(TokenKind::Yield) => self.parse_yield_statement(),
            
            // Local declarations
            Some(TokenKind::Const) => self.parse_local_declaration_statement(),
            
            // Check for local variable declaration or expression statement
            _ => {
                // This is tricky - need to disambiguate between:
                // - Type Name = expr;  (local declaration)
                // - expr;              (expression statement)
                // - Type Name;         (local declaration without initializer)
                
                if self.is_local_declaration() {
                    self.parse_local_declaration_statement()
                } else {
                    self.parse_expression_statement()
                }
            }
        };
        
        // Add attributes if any
        if !attributes.is_empty() {
            let kind = statement.kind;
            let mut children = vec![];
            for attr in attributes {
                children.push(SyntaxElement::Node(attr));
            }
            children.push(SyntaxElement::Node(statement));
            
            SyntaxNode {
                kind, // Keep the same kind
                children,
            }
        } else {
            statement
        }
    }
    
    /// Parse if statement
    fn parse_if_statement(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::If, "Expected 'if'")));
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::OpenParen, "Expected '('")));
        children.push(SyntaxElement::Node(self.parse_expression()));
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::CloseParen, "Expected ')'")));
        children.push(SyntaxElement::Node(self.parse_statement()));
        
        // Optional else clause
        if self.parser.current_kind() == Some(TokenKind::Else) {
            let else_token = self.parser.advance().unwrap();
            let else_statement = self.parse_statement();
            
            children.push(SyntaxElement::Node(SyntaxNode {
                kind: SyntaxKind::ElseClause,
                children: vec![
                    SyntaxElement::Token(else_token),
                    SyntaxElement::Node(else_statement),
                ],
            }));
        }
        
        SyntaxNode {
            kind: SyntaxKind::IfStatement,
            children,
        }
    }
    
    /// Parse switch statement
    fn parse_switch_statement(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Switch, "Expected 'switch'")));
        
        // Switch expression
        if self.parser.current_kind() == Some(TokenKind::OpenParen) {
            // Traditional switch statement
            children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
            children.push(SyntaxElement::Node(self.parse_expression()));
            children.push(SyntaxElement::Token(self.parser.expect(TokenKind::CloseParen, "Expected ')'")));
        } else {
            // Switch expression (C# 8+)
            children.push(SyntaxElement::Node(self.parse_expression()));
        }
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::OpenBrace, "Expected '{'")));
        
        // Switch sections
        while !self.parser.is_at_end() && self.parser.current_kind() != Some(TokenKind::CloseBrace) {
            children.push(SyntaxElement::Node(self.parse_switch_section()));
        }
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::CloseBrace, "Expected '}'")));
        
        SyntaxNode {
            kind: SyntaxKind::SwitchStatement,
            children,
        }
    }
    
    /// Parse switch section
    fn parse_switch_section(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        // Labels (case or default)
        while matches!(self.parser.current_kind(), Some(TokenKind::Case) | Some(TokenKind::Default)) {
            if self.parser.current_kind() == Some(TokenKind::Case) {
                let case_token = self.parser.advance().unwrap();
                let pattern = self.parse_pattern();
                
                // Optional when clause
                let mut case_children = vec![
                    SyntaxElement::Token(case_token),
                    SyntaxElement::Node(pattern),
                ];
                
                if self.parser.current_kind() == Some(TokenKind::When) {
                    case_children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
                    case_children.push(SyntaxElement::Node(self.parse_expression()));
                }
                
                case_children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Colon, "Expected ':'")));
                
                children.push(SyntaxElement::Node(SyntaxNode {
                    kind: SyntaxKind::CaseLabel,
                    children: case_children,
                }));
            } else {
                let default_token = self.parser.advance().unwrap();
                let colon = self.parser.expect(TokenKind::Colon, "Expected ':'");
                
                children.push(SyntaxElement::Node(SyntaxNode {
                    kind: SyntaxKind::DefaultLabel,
                    children: vec![
                        SyntaxElement::Token(default_token),
                        SyntaxElement::Token(colon),
                    ],
                }));
            }
        }
        
        // Statements
        while !self.parser.is_at_end() && 
              !matches!(self.parser.current_kind(), Some(TokenKind::Case) | Some(TokenKind::Default) | Some(TokenKind::CloseBrace)) {
            children.push(SyntaxElement::Node(self.parse_statement()));
        }
        
        SyntaxNode {
            kind: SyntaxKind::SwitchSection,
            children,
        }
    }
    
    /// Parse while statement
    fn parse_while_statement(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::While, "Expected 'while'")));
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::OpenParen, "Expected '('")));
        children.push(SyntaxElement::Node(self.parse_expression()));
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::CloseParen, "Expected ')'")));
        children.push(SyntaxElement::Node(self.parse_statement()));
        
        SyntaxNode {
            kind: SyntaxKind::WhileStatement,
            children,
        }
    }
    
    /// Parse do-while statement
    fn parse_do_statement(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Do, "Expected 'do'")));
        children.push(SyntaxElement::Node(self.parse_statement()));
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::While, "Expected 'while'")));
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::OpenParen, "Expected '('")));
        children.push(SyntaxElement::Node(self.parse_expression()));
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::CloseParen, "Expected ')'")));
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Semicolon, "Expected ';'")));
        
        SyntaxNode {
            kind: SyntaxKind::DoStatement,
            children,
        }
    }
    
    /// Parse for statement
    fn parse_for_statement(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::For, "Expected 'for'")));
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::OpenParen, "Expected '('")));
        
        // Initializer
        if self.parser.current_kind() != Some(TokenKind::Semicolon) {
            if self.is_local_declaration() {
                children.push(SyntaxElement::Node(self.parse_variable_declaration()));
            } else {
                // Expression list
                loop {
                    children.push(SyntaxElement::Node(self.parse_expression()));
                    if self.parser.current_kind() == Some(TokenKind::Comma) {
                        children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
                    } else {
                        break;
                    }
                }
            }
        }
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Semicolon, "Expected ';'")));
        
        // Condition
        if self.parser.current_kind() != Some(TokenKind::Semicolon) {
            children.push(SyntaxElement::Node(self.parse_expression()));
        }
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Semicolon, "Expected ';'")));
        
        // Incrementors
        if self.parser.current_kind() != Some(TokenKind::CloseParen) {
            loop {
                children.push(SyntaxElement::Node(self.parse_expression()));
                if self.parser.current_kind() == Some(TokenKind::Comma) {
                    children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
                } else {
                    break;
                }
            }
        }
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::CloseParen, "Expected ')'")));
        children.push(SyntaxElement::Node(self.parse_statement()));
        
        SyntaxNode {
            kind: SyntaxKind::ForStatement,
            children,
        }
    }
    
    /// Parse foreach statement
    fn parse_foreach_statement(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Foreach, "Expected 'foreach'")));
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::OpenParen, "Expected '('")));
        
        // Type or var
        children.push(SyntaxElement::Node(self.parse_type()));
        
        // Identifier
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Identifier, "Expected identifier")));
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::In, "Expected 'in'")));
        children.push(SyntaxElement::Node(self.parse_expression()));
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::CloseParen, "Expected ')'")));
        children.push(SyntaxElement::Node(self.parse_statement()));
        
        SyntaxNode {
            kind: SyntaxKind::ForEachStatement,
            children,
        }
    }
    
    /// Parse break statement
    fn parse_break_statement(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Break, "Expected 'break'")));
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Semicolon, "Expected ';'")));
        
        SyntaxNode {
            kind: SyntaxKind::BreakStatement,
            children,
        }
    }
    
    /// Parse continue statement
    fn parse_continue_statement(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Continue, "Expected 'continue'")));
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Semicolon, "Expected ';'")));
        
        SyntaxNode {
            kind: SyntaxKind::ContinueStatement,
            children,
        }
    }
    
    /// Parse goto statement
    fn parse_goto_statement(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Goto, "Expected 'goto'")));
        
        // goto case/default or label
        match self.parser.current_kind() {
            Some(TokenKind::Case) => {
                children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
                children.push(SyntaxElement::Node(self.parse_expression()));
            }
            Some(TokenKind::Default) => {
                children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
            }
            _ => {
                children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Identifier, "Expected label")));
            }
        }
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Semicolon, "Expected ';'")));
        
        SyntaxNode {
            kind: SyntaxKind::GotoStatement,
            children,
        }
    }
    
    /// Parse return statement
    fn parse_return_statement(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Return, "Expected 'return'")));
        
        if self.parser.current_kind() != Some(TokenKind::Semicolon) {
            children.push(SyntaxElement::Node(self.parse_expression()));
        }
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Semicolon, "Expected ';'")));
        
        SyntaxNode {
            kind: SyntaxKind::ReturnStatement,
            children,
        }
    }
    
    /// Parse throw statement
    fn parse_throw_statement(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Throw, "Expected 'throw'")));
        
        if self.parser.current_kind() != Some(TokenKind::Semicolon) {
            children.push(SyntaxElement::Node(self.parse_expression()));
        }
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Semicolon, "Expected ';'")));
        
        SyntaxNode {
            kind: SyntaxKind::ThrowStatement,
            children,
        }
    }
    
    /// Parse try statement
    fn parse_try_statement(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Try, "Expected 'try'")));
        children.push(SyntaxElement::Node(self.parse_block()));
        
        // Catch clauses
        while self.parser.current_kind() == Some(TokenKind::Catch) {
            let catch_token = self.parser.advance().unwrap();
            let mut catch_children = vec![SyntaxElement::Token(catch_token)];
            
            // Optional catch declaration
            if self.parser.current_kind() == Some(TokenKind::OpenParen) {
                catch_children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
                catch_children.push(SyntaxElement::Node(self.parse_type()));
                
                // Optional identifier
                if self.parser.current_kind() == Some(TokenKind::Identifier) {
                    catch_children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
                }
                
                catch_children.push(SyntaxElement::Token(self.parser.expect(TokenKind::CloseParen, "Expected ')'")));
            }
            
            // Optional when clause
            if self.parser.current_kind() == Some(TokenKind::When) {
                catch_children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
                catch_children.push(SyntaxElement::Token(self.parser.expect(TokenKind::OpenParen, "Expected '('")));
                catch_children.push(SyntaxElement::Node(self.parse_expression()));
                catch_children.push(SyntaxElement::Token(self.parser.expect(TokenKind::CloseParen, "Expected ')'")));
            }
            
            catch_children.push(SyntaxElement::Node(self.parse_block()));
            
            children.push(SyntaxElement::Node(SyntaxNode {
                kind: SyntaxKind::CatchClause,
                children: catch_children,
            }));
        }
        
        // Finally clause
        if self.parser.current_kind() == Some(TokenKind::Finally) {
            let finally_token = self.parser.advance().unwrap();
            let block = self.parse_block();
            
            children.push(SyntaxElement::Node(SyntaxNode {
                kind: SyntaxKind::FinallyClause,
                children: vec![
                    SyntaxElement::Token(finally_token),
                    SyntaxElement::Node(block),
                ],
            }));
        }
        
        SyntaxNode {
            kind: SyntaxKind::TryStatement,
            children,
        }
    }
    
    /// Parse lock statement
    fn parse_lock_statement(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Lock, "Expected 'lock'")));
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::OpenParen, "Expected '('")));
        children.push(SyntaxElement::Node(self.parse_expression()));
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::CloseParen, "Expected ')'")));
        children.push(SyntaxElement::Node(self.parse_statement()));
        
        SyntaxNode {
            kind: SyntaxKind::LockStatement,
            children,
        }
    }
    
    /// Parse using statement
    fn parse_using_statement(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Using, "Expected 'using'")));
        
        // Check for using declaration (C# 8+) vs using statement
        if self.parser.current_kind() == Some(TokenKind::OpenParen) {
            // Traditional using statement
            children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
            
            // Variable declaration or expression
            if self.is_local_declaration() {
                children.push(SyntaxElement::Node(self.parse_variable_declaration()));
            } else {
                children.push(SyntaxElement::Node(self.parse_expression()));
            }
            
            children.push(SyntaxElement::Token(self.parser.expect(TokenKind::CloseParen, "Expected ')'")));
            children.push(SyntaxElement::Node(self.parse_statement()));
        } else {
            // Using declaration
            children.push(SyntaxElement::Node(self.parse_variable_declaration()));
            children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Semicolon, "Expected ';'")));
        }
        
        SyntaxNode {
            kind: SyntaxKind::UsingStatement,
            children,
        }
    }
    
    /// Parse yield statement
    fn parse_yield_statement(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Yield, "Expected 'yield'")));
        
        match self.parser.current_kind() {
            Some(TokenKind::Return) => {
                children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
                children.push(SyntaxElement::Node(self.parse_expression()));
                children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Semicolon, "Expected ';'")));
                
                SyntaxNode {
                    kind: SyntaxKind::YieldReturnStatement,
                    children,
                }
            }
            Some(TokenKind::Break) => {
                children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
                children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Semicolon, "Expected ';'")));
                
                SyntaxNode {
                    kind: SyntaxKind::YieldBreakStatement,
                    children,
                }
            }
            _ => {
                self.parser.error("Expected 'return' or 'break' after 'yield'");
                SyntaxNode {
                    kind: SyntaxKind::Error,
                    children,
                }
            }
        }
    }
    
    /// Parse local declaration statement
    fn parse_local_declaration_statement(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        // Optional const
        if self.parser.current_kind() == Some(TokenKind::Const) {
            children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
        }
        
        children.push(SyntaxElement::Node(self.parse_variable_declaration()));
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Semicolon, "Expected ';'")));
        
        SyntaxNode {
            kind: SyntaxKind::LocalDeclarationStatement,
            children,
        }
    }
    
    /// Parse variable declaration
    fn parse_variable_declaration(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        // Type
        children.push(SyntaxElement::Node(self.parse_type()));
        
        // Variable declarators
        loop {
            // Variable name
            children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Identifier, "Expected variable name")));
            
            // Optional initializer
            if self.parser.current_kind() == Some(TokenKind::Equals) {
                children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
                children.push(SyntaxElement::Node(self.parse_expression()));
            }
            
            if self.parser.current_kind() == Some(TokenKind::Comma) {
                children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
            } else {
                break;
            }
        }
        
        SyntaxNode {
            kind: SyntaxKind::VariableDeclaration,
            children,
        }
    }
    
    /// Parse expression statement
    fn parse_expression_statement(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        children.push(SyntaxElement::Node(self.parse_expression()));
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::Semicolon, "Expected ';'")));
        
        SyntaxNode {
            kind: SyntaxKind::ExpressionStatement,
            children,
        }
    }
    
    // Helper methods
    
    fn is_local_declaration(&self) -> bool {
        // This is a simplified check
        // In reality, we'd need more sophisticated lookahead
        matches!(self.parser.current_kind(),
            Some(TokenKind::Var) | Some(TokenKind::Int) | Some(TokenKind::String) |
            Some(TokenKind::Bool) | Some(TokenKind::Double) | Some(TokenKind::Float) |
            Some(TokenKind::Long) | Some(TokenKind::Short) | Some(TokenKind::Byte) |
            Some(TokenKind::Char) | Some(TokenKind::Decimal) | Some(TokenKind::Object)
        ) || (self.parser.current_kind() == Some(TokenKind::Identifier) &&
              self.parser.peek_kind(1) == Some(TokenKind::Identifier))
    }
    
    fn parse_pattern(&mut self) -> SyntaxNode {
        // Pattern matching - simplified for now
        self.parse_expression()
    }
    
    fn parse_expression(&mut self) -> SyntaxNode {
        super::expressions::ExpressionParser::new(self.parser).parse_expression()
    }
    
    fn parse_type(&mut self) -> SyntaxNode {
        super::types::TypeParser::new(self.parser).parse_type()
    }
    
    fn parse_attribute_list(&mut self) -> SyntaxNode {
        super::declarations::DeclarationParser::new(self.parser).parse_attribute_list()
    }
}