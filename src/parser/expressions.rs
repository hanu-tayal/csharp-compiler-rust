//! Expression parsing using precedence climbing

use crate::lexer::token::TokenKind;
use crate::syntax::{SyntaxNode, SyntaxKind, SyntaxElement};
use super::Parser;

pub struct ExpressionParser<'a, 'p> {
    parser: &'p mut Parser<'a>,
}

impl<'a, 'p> ExpressionParser<'a, 'p> {
    pub fn new(parser: &'p mut Parser<'a>) -> Self {
        Self { parser }
    }
    
    /// Parse any expression
    pub fn parse_expression(&mut self) -> SyntaxNode {
        self.parse_assignment_expression()
    }
    
    /// Parse assignment expression (lowest precedence)
    fn parse_assignment_expression(&mut self) -> SyntaxNode {
        let mut left = self.parse_conditional_expression();
        
        // Check for assignment operators
        match self.parser.current_kind() {
            Some(TokenKind::Equals) | Some(TokenKind::PlusEquals) | Some(TokenKind::MinusEquals) |
            Some(TokenKind::StarEquals) | Some(TokenKind::SlashEquals) | Some(TokenKind::PercentEquals) |
            Some(TokenKind::AmpersandEquals) | Some(TokenKind::PipeEquals) | Some(TokenKind::CaretEquals) |
            Some(TokenKind::LeftShiftEquals) | Some(TokenKind::RightShiftEquals) | 
            Some(TokenKind::UnsignedRightShiftEquals) | Some(TokenKind::QuestionQuestionEquals) => {
                let op = self.parser.advance().unwrap();
                let right = self.parse_assignment_expression();
                
                let kind = match op.kind {
                    SyntaxKind::EqualsToken => SyntaxKind::SimpleAssignmentExpression,
                    SyntaxKind::PlusEqualsToken => SyntaxKind::AddAssignmentExpression,
                    SyntaxKind::MinusEqualsToken => SyntaxKind::SubtractAssignmentExpression,
                    SyntaxKind::StarEqualsToken => SyntaxKind::MultiplyAssignmentExpression,
                    SyntaxKind::SlashEqualsToken => SyntaxKind::DivideAssignmentExpression,
                    SyntaxKind::PercentEqualsToken => SyntaxKind::ModuloAssignmentExpression,
                    SyntaxKind::AmpersandEqualsToken => SyntaxKind::AndAssignmentExpression,
                    SyntaxKind::PipeEqualsToken => SyntaxKind::OrAssignmentExpression,
                    SyntaxKind::CaretEqualsToken => SyntaxKind::ExclusiveOrAssignmentExpression,
                    SyntaxKind::LeftShiftEqualsToken => SyntaxKind::LeftShiftAssignmentExpression,
                    SyntaxKind::RightShiftEqualsToken => SyntaxKind::RightShiftAssignmentExpression,
                    _ => SyntaxKind::SimpleAssignmentExpression,
                };
                
                left = SyntaxNode {
                    kind,
                    children: vec![
                        SyntaxElement::Node(left),
                        SyntaxElement::Token(op),
                        SyntaxElement::Node(right),
                    ],
                };
            }
            _ => {}
        }
        
        left
    }
    
    /// Parse conditional (ternary) expression
    fn parse_conditional_expression(&mut self) -> SyntaxNode {
        let mut condition = self.parse_null_coalescing_expression();
        
        if self.parser.current_kind() == Some(TokenKind::Question) {
            let question = self.parser.advance().unwrap();
            let true_expr = self.parse_expression();
            let colon = self.parser.expect(TokenKind::Colon, "Expected ':' in conditional expression");
            let false_expr = self.parse_conditional_expression();
            
            condition = SyntaxNode {
                kind: SyntaxKind::ConditionalExpression,
                children: vec![
                    SyntaxElement::Node(condition),
                    SyntaxElement::Token(question),
                    SyntaxElement::Node(true_expr),
                    SyntaxElement::Token(colon),
                    SyntaxElement::Node(false_expr),
                ],
            };
        }
        
        condition
    }
    
    /// Parse null coalescing expression (??)
    fn parse_null_coalescing_expression(&mut self) -> SyntaxNode {
        let mut left = self.parse_logical_or_expression();
        
        while self.parser.current_kind() == Some(TokenKind::QuestionQuestion) {
            let op = self.parser.advance().unwrap();
            let right = self.parse_logical_or_expression();
            
            left = SyntaxNode {
                kind: SyntaxKind::CoalesceExpression,
                children: vec![
                    SyntaxElement::Node(left),
                    SyntaxElement::Token(op),
                    SyntaxElement::Node(right),
                ],
            };
        }
        
        left
    }
    
    /// Parse logical OR expression (||)
    fn parse_logical_or_expression(&mut self) -> SyntaxNode {
        let mut left = self.parse_logical_and_expression();
        
        while self.parser.current_kind() == Some(TokenKind::PipePipe) {
            let op = self.parser.advance().unwrap();
            let right = self.parse_logical_and_expression();
            
            left = SyntaxNode {
                kind: SyntaxKind::LogicalOrExpression,
                children: vec![
                    SyntaxElement::Node(left),
                    SyntaxElement::Token(op),
                    SyntaxElement::Node(right),
                ],
            };
        }
        
        left
    }
    
    /// Parse logical AND expression (&&)
    fn parse_logical_and_expression(&mut self) -> SyntaxNode {
        let mut left = self.parse_bitwise_or_expression();
        
        while self.parser.current_kind() == Some(TokenKind::AmpersandAmpersand) {
            let op = self.parser.advance().unwrap();
            let right = self.parse_bitwise_or_expression();
            
            left = SyntaxNode {
                kind: SyntaxKind::LogicalAndExpression,
                children: vec![
                    SyntaxElement::Node(left),
                    SyntaxElement::Token(op),
                    SyntaxElement::Node(right),
                ],
            };
        }
        
        left
    }
    
    /// Parse bitwise OR expression (|)
    fn parse_bitwise_or_expression(&mut self) -> SyntaxNode {
        let mut left = self.parse_bitwise_xor_expression();
        
        while self.parser.current_kind() == Some(TokenKind::Pipe) {
            let op = self.parser.advance().unwrap();
            let right = self.parse_bitwise_xor_expression();
            
            left = SyntaxNode {
                kind: SyntaxKind::BitwiseOrExpression,
                children: vec![
                    SyntaxElement::Node(left),
                    SyntaxElement::Token(op),
                    SyntaxElement::Node(right),
                ],
            };
        }
        
        left
    }
    
    /// Parse bitwise XOR expression (^)
    fn parse_bitwise_xor_expression(&mut self) -> SyntaxNode {
        let mut left = self.parse_bitwise_and_expression();
        
        while self.parser.current_kind() == Some(TokenKind::Caret) {
            let op = self.parser.advance().unwrap();
            let right = self.parse_bitwise_and_expression();
            
            left = SyntaxNode {
                kind: SyntaxKind::ExclusiveOrExpression,
                children: vec![
                    SyntaxElement::Node(left),
                    SyntaxElement::Token(op),
                    SyntaxElement::Node(right),
                ],
            };
        }
        
        left
    }
    
    /// Parse bitwise AND expression (&)
    fn parse_bitwise_and_expression(&mut self) -> SyntaxNode {
        let mut left = self.parse_equality_expression();
        
        while self.parser.current_kind() == Some(TokenKind::Ampersand) {
            let op = self.parser.advance().unwrap();
            let right = self.parse_equality_expression();
            
            left = SyntaxNode {
                kind: SyntaxKind::BitwiseAndExpression,
                children: vec![
                    SyntaxElement::Node(left),
                    SyntaxElement::Token(op),
                    SyntaxElement::Node(right),
                ],
            };
        }
        
        left
    }
    
    /// Parse equality expression (==, !=)
    fn parse_equality_expression(&mut self) -> SyntaxNode {
        let mut left = self.parse_relational_expression();
        
        while matches!(self.parser.current_kind(), Some(TokenKind::EqualsEquals) | Some(TokenKind::ExclamationEquals)) {
            let op = self.parser.advance().unwrap();
            let right = self.parse_relational_expression();
            
            let kind = match op.kind {
                SyntaxKind::EqualsEqualsToken => SyntaxKind::EqualsExpression,
                SyntaxKind::ExclamationEqualsToken => SyntaxKind::NotEqualsExpression,
                _ => unreachable!(),
            };
            
            left = SyntaxNode {
                kind,
                children: vec![
                    SyntaxElement::Node(left),
                    SyntaxElement::Token(op),
                    SyntaxElement::Node(right),
                ],
            };
        }
        
        left
    }
    
    /// Parse relational expression (<, >, <=, >=, is, as)
    fn parse_relational_expression(&mut self) -> SyntaxNode {
        let mut left = self.parse_shift_expression();
        
        while matches!(self.parser.current_kind(), 
            Some(TokenKind::LessThan) | Some(TokenKind::GreaterThan) |
            Some(TokenKind::LessThanEquals) | Some(TokenKind::GreaterThanEquals) |
            Some(TokenKind::Is) | Some(TokenKind::As)) {
            
            let op = self.parser.advance().unwrap();
            
            let (kind, right) = match op.kind {
                SyntaxKind::IsKeyword => {
                    // 'is' can be followed by pattern or type
                    let pattern = self.parse_pattern();
                    (SyntaxKind::IsExpression, pattern)
                }
                SyntaxKind::AsKeyword => {
                    // 'as' is followed by type
                    let type_node = self.parse_type();
                    (SyntaxKind::AsExpression, type_node)
                }
                _ => {
                    let right = self.parse_shift_expression();
                    let kind = match op.kind {
                        SyntaxKind::LessThanToken => SyntaxKind::LessThanExpression,
                        SyntaxKind::GreaterThanToken => SyntaxKind::GreaterThanExpression,
                        SyntaxKind::LessThanEqualsToken => SyntaxKind::LessThanOrEqualExpression,
                        SyntaxKind::GreaterThanEqualsToken => SyntaxKind::GreaterThanOrEqualExpression,
                        _ => unreachable!(),
                    };
                    (kind, right)
                }
            };
            
            left = SyntaxNode {
                kind,
                children: vec![
                    SyntaxElement::Node(left),
                    SyntaxElement::Token(op),
                    SyntaxElement::Node(right),
                ],
            };
        }
        
        left
    }
    
    /// Parse shift expression (<<, >>, >>>)
    fn parse_shift_expression(&mut self) -> SyntaxNode {
        let mut left = self.parse_additive_expression();
        
        while matches!(self.parser.current_kind(), 
            Some(TokenKind::LeftShift) | Some(TokenKind::RightShift) | Some(TokenKind::UnsignedRightShift)) {
            
            let op = self.parser.advance().unwrap();
            let right = self.parse_additive_expression();
            
            let kind = match op.kind {
                SyntaxKind::LeftShiftToken => SyntaxKind::LeftShiftExpression,
                SyntaxKind::RightShiftToken => SyntaxKind::RightShiftExpression,
                SyntaxKind::UnsignedRightShiftToken => SyntaxKind::UnsignedRightShiftExpression,
                _ => unreachable!(),
            };
            
            left = SyntaxNode {
                kind,
                children: vec![
                    SyntaxElement::Node(left),
                    SyntaxElement::Token(op),
                    SyntaxElement::Node(right),
                ],
            };
        }
        
        left
    }
    
    /// Parse additive expression (+, -)
    fn parse_additive_expression(&mut self) -> SyntaxNode {
        let mut left = self.parse_multiplicative_expression();
        
        while matches!(self.parser.current_kind(), Some(TokenKind::Plus) | Some(TokenKind::Minus)) {
            let op = self.parser.advance().unwrap();
            let right = self.parse_multiplicative_expression();
            
            let kind = match op.kind {
                SyntaxKind::PlusToken => SyntaxKind::AddExpression,
                SyntaxKind::MinusToken => SyntaxKind::SubtractExpression,
                _ => unreachable!(),
            };
            
            left = SyntaxNode {
                kind,
                children: vec![
                    SyntaxElement::Node(left),
                    SyntaxElement::Token(op),
                    SyntaxElement::Node(right),
                ],
            };
        }
        
        left
    }
    
    /// Parse multiplicative expression (*, /, %)
    fn parse_multiplicative_expression(&mut self) -> SyntaxNode {
        let mut left = self.parse_unary_expression();
        
        while matches!(self.parser.current_kind(), 
            Some(TokenKind::Star) | Some(TokenKind::Slash) | Some(TokenKind::Percent)) {
            
            let op = self.parser.advance().unwrap();
            let right = self.parse_unary_expression();
            
            let kind = match op.kind {
                SyntaxKind::StarToken => SyntaxKind::MultiplyExpression,
                SyntaxKind::SlashToken => SyntaxKind::DivideExpression,
                SyntaxKind::PercentToken => SyntaxKind::ModuloExpression,
                _ => unreachable!(),
            };
            
            left = SyntaxNode {
                kind,
                children: vec![
                    SyntaxElement::Node(left),
                    SyntaxElement::Token(op),
                    SyntaxElement::Node(right),
                ],
            };
        }
        
        left
    }
    
    /// Parse unary expression
    fn parse_unary_expression(&mut self) -> SyntaxNode {
        match self.parser.current_kind() {
            Some(TokenKind::Plus) => {
                let op = self.parser.advance().unwrap();
                let expr = self.parse_unary_expression();
                SyntaxNode {
                    kind: SyntaxKind::UnaryPlusExpression,
                    children: vec![SyntaxElement::Token(op), SyntaxElement::Node(expr)],
                }
            }
            Some(TokenKind::Minus) => {
                let op = self.parser.advance().unwrap();
                let expr = self.parse_unary_expression();
                SyntaxNode {
                    kind: SyntaxKind::UnaryMinusExpression,
                    children: vec![SyntaxElement::Token(op), SyntaxElement::Node(expr)],
                }
            }
            Some(TokenKind::Exclamation) => {
                let op = self.parser.advance().unwrap();
                let expr = self.parse_unary_expression();
                SyntaxNode {
                    kind: SyntaxKind::LogicalNotExpression,
                    children: vec![SyntaxElement::Token(op), SyntaxElement::Node(expr)],
                }
            }
            Some(TokenKind::Tilde) => {
                let op = self.parser.advance().unwrap();
                let expr = self.parse_unary_expression();
                SyntaxNode {
                    kind: SyntaxKind::BitwiseNotExpression,
                    children: vec![SyntaxElement::Token(op), SyntaxElement::Node(expr)],
                }
            }
            Some(TokenKind::PlusPlus) => {
                let op = self.parser.advance().unwrap();
                let expr = self.parse_unary_expression();
                SyntaxNode {
                    kind: SyntaxKind::PreIncrementExpression,
                    children: vec![SyntaxElement::Token(op), SyntaxElement::Node(expr)],
                }
            }
            Some(TokenKind::MinusMinus) => {
                let op = self.parser.advance().unwrap();
                let expr = self.parse_unary_expression();
                SyntaxNode {
                    kind: SyntaxKind::PreDecrementExpression,
                    children: vec![SyntaxElement::Token(op), SyntaxElement::Node(expr)],
                }
            }
            Some(TokenKind::OpenParen) => {
                // Could be cast or parenthesized expression
                if self.is_cast_expression() {
                    self.parse_cast_expression()
                } else {
                    self.parse_primary_expression()
                }
            }
            Some(TokenKind::Await) => {
                let await_token = self.parser.advance().unwrap();
                let expr = self.parse_unary_expression();
                SyntaxNode {
                    kind: SyntaxKind::AwaitExpression,
                    children: vec![SyntaxElement::Token(await_token), SyntaxElement::Node(expr)],
                }
            }
            _ => self.parse_postfix_expression(),
        }
    }
    
    /// Parse postfix expression
    fn parse_postfix_expression(&mut self) -> SyntaxNode {
        let mut expr = self.parse_primary_expression();
        
        loop {
            match self.parser.current_kind() {
                Some(TokenKind::Dot) => {
                    let dot = self.parser.advance().unwrap();
                    let member = self.parser.expect(TokenKind::Identifier, "Expected member name");
                    
                    // Check for generic arguments
                    let mut children = vec![
                        SyntaxElement::Node(expr),
                        SyntaxElement::Token(dot),
                        SyntaxElement::Token(member),
                    ];
                    
                    if self.parser.current_kind() == Some(TokenKind::LessThan) && self.is_generic_arguments() {
                        children.push(SyntaxElement::Node(self.parse_type_argument_list()));
                    }
                    
                    expr = SyntaxNode {
                        kind: SyntaxKind::SimpleMemberAccessExpression,
                        children,
                    };
                }
                Some(TokenKind::Arrow) => {
                    let arrow = self.parser.advance().unwrap();
                    let member = self.parser.expect(TokenKind::Identifier, "Expected member name");
                    
                    expr = SyntaxNode {
                        kind: SyntaxKind::PointerMemberAccessExpression,
                        children: vec![
                            SyntaxElement::Node(expr),
                            SyntaxElement::Token(arrow),
                            SyntaxElement::Token(member),
                        ],
                    };
                }
                Some(TokenKind::OpenParen) => {
                    // Method invocation
                    let args = self.parse_argument_list();
                    
                    expr = SyntaxNode {
                        kind: SyntaxKind::InvocationExpression,
                        children: vec![
                            SyntaxElement::Node(expr),
                            SyntaxElement::Node(args),
                        ],
                    };
                }
                Some(TokenKind::OpenBracket) => {
                    // Element access
                    let open_bracket = self.parser.advance().unwrap();
                    let mut indices = vec![self.parse_expression()];
                    
                    while self.parser.current_kind() == Some(TokenKind::Comma) {
                        self.parser.advance(); // consume comma
                        indices.push(self.parse_expression());
                    }
                    
                    let close_bracket = self.parser.expect(TokenKind::CloseBracket, "Expected ']'");
                    
                    let mut children = vec![
                        SyntaxElement::Node(expr),
                        SyntaxElement::Token(open_bracket),
                    ];
                    
                    for index in indices {
                        children.push(SyntaxElement::Node(index));
                    }
                    
                    children.push(SyntaxElement::Token(close_bracket));
                    
                    expr = SyntaxNode {
                        kind: SyntaxKind::ElementAccessExpression,
                        children,
                    };
                }
                Some(TokenKind::PlusPlus) => {
                    let op = self.parser.advance().unwrap();
                    expr = SyntaxNode {
                        kind: SyntaxKind::PostIncrementExpression,
                        children: vec![SyntaxElement::Node(expr), SyntaxElement::Token(op)],
                    };
                }
                Some(TokenKind::MinusMinus) => {
                    let op = self.parser.advance().unwrap();
                    expr = SyntaxNode {
                        kind: SyntaxKind::PostDecrementExpression,
                        children: vec![SyntaxElement::Node(expr), SyntaxElement::Token(op)],
                    };
                }
                Some(TokenKind::Question) if self.parser.peek_kind(1) == Some(TokenKind::Dot) => {
                    // Conditional access ?.
                    let question = self.parser.advance().unwrap();
                    let dot = self.parser.advance().unwrap();
                    let member = self.parser.expect(TokenKind::Identifier, "Expected member name");
                    
                    expr = SyntaxNode {
                        kind: SyntaxKind::ConditionalAccessExpression,
                        children: vec![
                            SyntaxElement::Node(expr),
                            SyntaxElement::Token(question),
                            SyntaxElement::Token(dot),
                            SyntaxElement::Token(member),
                        ],
                    };
                }
                _ => break,
            }
        }
        
        expr
    }
    
    /// Parse primary expression
    fn parse_primary_expression(&mut self) -> SyntaxNode {
        match self.parser.current_kind() {
            // Literals
            Some(TokenKind::True) => {
                let token = self.parser.advance().unwrap();
                SyntaxNode {
                    kind: SyntaxKind::TrueLiteralExpression,
                    children: vec![SyntaxElement::Token(token)],
                }
            }
            Some(TokenKind::False) => {
                let token = self.parser.advance().unwrap();
                SyntaxNode {
                    kind: SyntaxKind::FalseLiteralExpression,
                    children: vec![SyntaxElement::Token(token)],
                }
            }
            Some(TokenKind::Null) => {
                let token = self.parser.advance().unwrap();
                SyntaxNode {
                    kind: SyntaxKind::NullLiteralExpression,
                    children: vec![SyntaxElement::Token(token)],
                }
            }
            Some(TokenKind::NumericLiteral) => {
                let token = self.parser.advance().unwrap();
                SyntaxNode {
                    kind: SyntaxKind::NumericLiteralExpression,
                    children: vec![SyntaxElement::Token(token)],
                }
            }
            Some(TokenKind::StringLiteral) => {
                let token = self.parser.advance().unwrap();
                SyntaxNode {
                    kind: SyntaxKind::StringLiteralExpression,
                    children: vec![SyntaxElement::Token(token)],
                }
            }
            Some(TokenKind::CharacterLiteral) => {
                let token = self.parser.advance().unwrap();
                SyntaxNode {
                    kind: SyntaxKind::CharacterLiteralExpression,
                    children: vec![SyntaxElement::Token(token)],
                }
            }
            // Identifier or predefined type
            Some(TokenKind::Identifier) => {
                let name = self.parser.advance().unwrap();
                
                // Check for generic arguments
                if self.parser.current_kind() == Some(TokenKind::LessThan) && self.is_generic_arguments() {
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
                }
            }
            // Keywords that are expressions
            Some(TokenKind::This) => {
                let token = self.parser.advance().unwrap();
                SyntaxNode {
                    kind: SyntaxKind::ThisExpression,
                    children: vec![SyntaxElement::Token(token)],
                }
            }
            Some(TokenKind::Base) => {
                let token = self.parser.advance().unwrap();
                SyntaxNode {
                    kind: SyntaxKind::BaseExpression,
                    children: vec![SyntaxElement::Token(token)],
                }
            }
            Some(TokenKind::Typeof) => {
                let typeof_token = self.parser.advance().unwrap();
                let open_paren = self.parser.expect(TokenKind::OpenParen, "Expected '('");
                let type_node = self.parse_type();
                let close_paren = self.parser.expect(TokenKind::CloseParen, "Expected ')'");
                
                SyntaxNode {
                    kind: SyntaxKind::TypeofExpression,
                    children: vec![
                        SyntaxElement::Token(typeof_token),
                        SyntaxElement::Token(open_paren),
                        SyntaxElement::Node(type_node),
                        SyntaxElement::Token(close_paren),
                    ],
                }
            }
            Some(TokenKind::Sizeof) => {
                let sizeof_token = self.parser.advance().unwrap();
                let open_paren = self.parser.expect(TokenKind::OpenParen, "Expected '('");
                let type_node = self.parse_type();
                let close_paren = self.parser.expect(TokenKind::CloseParen, "Expected ')'");
                
                SyntaxNode {
                    kind: SyntaxKind::SizeofExpression,
                    children: vec![
                        SyntaxElement::Token(sizeof_token),
                        SyntaxElement::Token(open_paren),
                        SyntaxElement::Node(type_node),
                        SyntaxElement::Token(close_paren),
                    ],
                }
            }
            Some(TokenKind::Default) => {
                let default_token = self.parser.advance().unwrap();
                
                if self.parser.current_kind() == Some(TokenKind::OpenParen) {
                    // default(Type)
                    let open_paren = self.parser.advance().unwrap();
                    let type_node = self.parse_type();
                    let close_paren = self.parser.expect(TokenKind::CloseParen, "Expected ')'");
                    
                    SyntaxNode {
                        kind: SyntaxKind::DefaultExpression,
                        children: vec![
                            SyntaxElement::Token(default_token),
                            SyntaxElement::Token(open_paren),
                            SyntaxElement::Node(type_node),
                            SyntaxElement::Token(close_paren),
                        ],
                    }
                } else {
                    // default literal (C# 7.1+)
                    SyntaxNode {
                        kind: SyntaxKind::DefaultLiteralExpression,
                        children: vec![SyntaxElement::Token(default_token)],
                    }
                }
            }
            Some(TokenKind::New) => self.parse_object_creation_expression(),
            Some(TokenKind::Stackalloc) => self.parse_stackalloc_expression(),
            Some(TokenKind::OpenParen) => {
                // Parenthesized expression or tuple
                let open_paren = self.parser.advance().unwrap();
                let first_expr = self.parse_expression();
                
                if self.parser.current_kind() == Some(TokenKind::Comma) {
                    // Tuple
                    let mut elements = vec![first_expr];
                    
                    while self.parser.current_kind() == Some(TokenKind::Comma) {
                        self.parser.advance(); // consume comma
                        elements.push(self.parse_expression());
                    }
                    
                    let close_paren = self.parser.expect(TokenKind::CloseParen, "Expected ')'");
                    
                    let mut children = vec![SyntaxElement::Token(open_paren)];
                    for elem in elements {
                        children.push(SyntaxElement::Node(elem));
                    }
                    children.push(SyntaxElement::Token(close_paren));
                    
                    SyntaxNode {
                        kind: SyntaxKind::TupleExpression,
                        children,
                    }
                } else {
                    // Parenthesized expression
                    let close_paren = self.parser.expect(TokenKind::CloseParen, "Expected ')'");
                    
                    SyntaxNode {
                        kind: SyntaxKind::ParenthesizedExpression,
                        children: vec![
                            SyntaxElement::Token(open_paren),
                            SyntaxElement::Node(first_expr),
                            SyntaxElement::Token(close_paren),
                        ],
                    }
                }
            }
            _ => {
                self.parser.error("Expected expression");
                SyntaxNode {
                    kind: SyntaxKind::Error,
                    children: vec![],
                }
            }
        }
    }
    
    /// Parse cast expression
    fn parse_cast_expression(&mut self) -> SyntaxNode {
        let open_paren = self.parser.expect(TokenKind::OpenParen, "Expected '('");
        let type_node = self.parse_type();
        let close_paren = self.parser.expect(TokenKind::CloseParen, "Expected ')'");
        let expr = self.parse_unary_expression();
        
        SyntaxNode {
            kind: SyntaxKind::CastExpression,
            children: vec![
                SyntaxElement::Token(open_paren),
                SyntaxElement::Node(type_node),
                SyntaxElement::Token(close_paren),
                SyntaxElement::Node(expr),
            ],
        }
    }
    
    /// Parse object creation expression
    fn parse_object_creation_expression(&mut self) -> SyntaxNode {
        let new_token = self.parser.expect(TokenKind::New, "Expected 'new'");
        let type_node = self.parse_type();
        
        let mut children = vec![
            SyntaxElement::Token(new_token),
            SyntaxElement::Node(type_node),
        ];
        
        // Arguments
        if self.parser.current_kind() == Some(TokenKind::OpenParen) {
            children.push(SyntaxElement::Node(self.parse_argument_list()));
        }
        
        // Object initializer
        if self.parser.current_kind() == Some(TokenKind::OpenBrace) {
            children.push(SyntaxElement::Node(self.parse_object_initializer()));
        }
        
        SyntaxNode {
            kind: SyntaxKind::ObjectCreationExpression,
            children,
        }
    }
    
    /// Parse stackalloc expression
    fn parse_stackalloc_expression(&mut self) -> SyntaxNode {
        let stackalloc = self.parser.expect(TokenKind::Stackalloc, "Expected 'stackalloc'");
        let type_node = self.parse_type();
        let open_bracket = self.parser.expect(TokenKind::OpenBracket, "Expected '['");
        let size = self.parse_expression();
        let close_bracket = self.parser.expect(TokenKind::CloseBracket, "Expected ']'");
        
        SyntaxNode {
            kind: SyntaxKind::StackAllocArrayCreationExpression,
            children: vec![
                SyntaxElement::Token(stackalloc),
                SyntaxElement::Node(type_node),
                SyntaxElement::Token(open_bracket),
                SyntaxElement::Node(size),
                SyntaxElement::Token(close_bracket),
            ],
        }
    }
    
    /// Parse object initializer
    fn parse_object_initializer(&mut self) -> SyntaxNode {
        let mut children = Vec::new();
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::OpenBrace, "Expected '{'")));
        
        while self.parser.current_kind() != Some(TokenKind::CloseBrace) && !self.parser.is_at_end() {
            // Member initializer
            if self.parser.current_kind() == Some(TokenKind::Identifier) &&
               self.parser.peek_kind(1) == Some(TokenKind::Equals) {
                let name = self.parser.advance().unwrap();
                let equals = self.parser.advance().unwrap();
                let value = self.parse_expression();
                
                children.push(SyntaxElement::Token(name));
                children.push(SyntaxElement::Token(equals));
                children.push(SyntaxElement::Node(value));
            } else {
                // Expression (for collection initializers)
                children.push(SyntaxElement::Node(self.parse_expression()));
            }
            
            if self.parser.current_kind() == Some(TokenKind::Comma) {
                children.push(SyntaxElement::Token(self.parser.advance().unwrap()));
            } else {
                break;
            }
        }
        
        children.push(SyntaxElement::Token(self.parser.expect(TokenKind::CloseBrace, "Expected '}'")));
        
        SyntaxNode {
            kind: SyntaxKind::InitializerExpression,
            children,
        }
    }
    
    // Helper methods
    
    fn is_cast_expression(&self) -> bool {
        // Simplified check - in reality this is more complex
        // Need to look ahead to see if (Type) is followed by valid cast target
        false
    }
    
    fn is_generic_arguments(&self) -> bool {
        // Check if < starts generic arguments (not less-than operator)
        // This requires lookahead
        true // Simplified
    }
    
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
    
    fn parse_argument_list(&mut self) -> SyntaxNode {
        super::declarations::DeclarationParser::new(self.parser).parse_argument_list()
    }
    
    fn parse_pattern(&mut self) -> SyntaxNode {
        // Pattern matching - simplified for now
        self.parse_type()
    }
    
    fn parse_type(&mut self) -> SyntaxNode {
        super::types::TypeParser::new(self.parser).parse_type()
    }
}