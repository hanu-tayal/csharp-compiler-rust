//! Expression and statement binding for semantic analysis

use crate::syntax::{SyntaxNode, SyntaxKind};
use crate::diagnostics::{DiagnosticBag, Diagnostic, DiagnosticCode};
use super::symbols::{SymbolTable, Symbol, SymbolKind};
use super::types::{TypeSymbol, TypeKind};
use std::rc::Rc;
use std::collections::HashMap;

/// Binder for expressions and statements
pub struct Binder<'a> {
    /// Symbol table
    symbol_table: &'a SymbolTable,
    /// Diagnostics
    diagnostics: &'a mut DiagnosticBag,
    /// Current scope
    scope: ScopeStack,
    /// Bound nodes (maps syntax nodes to bound nodes)
    bound_nodes: HashMap<*const SyntaxNode, BoundNode>,
}

/// Stack of scopes for name resolution
#[derive(Debug)]
struct ScopeStack {
    scopes: Vec<Scope>,
}

/// A scope containing local symbols
#[derive(Debug)]
struct Scope {
    /// Local symbols in this scope
    locals: HashMap<String, LocalSymbol>,
    /// Parent type (if in a type scope)
    containing_type: Option<String>,
    /// Parent method (if in a method scope)
    containing_method: Option<String>,
}

/// A local symbol (parameter or local variable)
#[derive(Debug, Clone)]
struct LocalSymbol {
    pub name: String,
    pub symbol_type: Rc<TypeSymbol>,
    pub is_readonly: bool,
    pub kind: LocalKind,
}

/// Kind of local symbol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LocalKind {
    Parameter,
    Variable,
    Constant,
}

/// Bound node (result of binding)
#[derive(Debug, Clone)]
pub struct BoundNode {
    pub kind: BoundNodeKind,
    pub result_type: Option<Rc<TypeSymbol>>,
}

/// Kind of bound node
#[derive(Debug, Clone)]
pub enum BoundNodeKind {
    // Expressions
    Literal(LiteralValue),
    Identifier(String),
    Binary {
        operator: BinaryOperator,
        left: Box<BoundNode>,
        right: Box<BoundNode>,
    },
    Unary {
        operator: UnaryOperator,
        operand: Box<BoundNode>,
    },
    Call {
        method: String,
        arguments: Vec<BoundNode>,
    },
    MemberAccess {
        instance: Box<BoundNode>,
        member: String,
    },
    Assignment {
        target: Box<BoundNode>,
        value: Box<BoundNode>,
    },
    Cast {
        expression: Box<BoundNode>,
        target_type: Rc<TypeSymbol>,
    },
    // Statements
    Block {
        statements: Vec<BoundNode>,
    },
    If {
        condition: Box<BoundNode>,
        then_statement: Box<BoundNode>,
        else_statement: Option<Box<BoundNode>>,
    },
    While {
        condition: Box<BoundNode>,
        body: Box<BoundNode>,
    },
    Return {
        value: Option<Box<BoundNode>>,
    },
    LocalDeclaration {
        name: String,
        initializer: Option<Box<BoundNode>>,
    },
    Expression {
        expression: Box<BoundNode>,
    },
}

/// Literal value
#[derive(Debug, Clone)]
pub enum LiteralValue {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Character(char),
    Null,
}

/// Binary operator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    // Comparison
    Equals,
    NotEquals,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    // Logical
    And,
    Or,
    // Bitwise
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
}

/// Unary operator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Plus,
    Minus,
    LogicalNot,
    BitwiseNot,
    PreIncrement,
    PreDecrement,
    PostIncrement,
    PostDecrement,
}

impl<'a> Binder<'a> {
    /// Create a new binder
    pub fn new(symbol_table: &'a SymbolTable, diagnostics: &'a mut DiagnosticBag) -> Self {
        Self {
            symbol_table,
            diagnostics,
            scope: ScopeStack::new(),
            bound_nodes: HashMap::new(),
        }
    }
    
    /// Bind a compilation unit
    pub fn bind_compilation_unit(&mut self, node: &SyntaxNode) {
        for child in &node.children {
            if let crate::syntax::SyntaxElement::Node(child_node) = child {
                match child_node.kind {
                    SyntaxKind::NamespaceDeclaration => self.bind_namespace(child_node),
                    SyntaxKind::ClassDeclaration |
                    SyntaxKind::StructDeclaration |
                    SyntaxKind::InterfaceDeclaration => self.bind_type_declaration(child_node),
                    _ => {}
                }
            }
        }
    }
    
    /// Bind a namespace
    fn bind_namespace(&mut self, node: &SyntaxNode) {
        // Enter namespace scope
        for child in &node.children {
            if let crate::syntax::SyntaxElement::Node(child_node) = child {
                match child_node.kind {
                    SyntaxKind::NamespaceDeclaration => self.bind_namespace(child_node),
                    SyntaxKind::ClassDeclaration |
                    SyntaxKind::StructDeclaration |
                    SyntaxKind::InterfaceDeclaration => self.bind_type_declaration(child_node),
                    _ => {}
                }
            }
        }
    }
    
    /// Bind a type declaration
    fn bind_type_declaration(&mut self, node: &SyntaxNode) {
        // Enter type scope
        self.scope.push_type_scope(self.get_type_name(node));
        
        // Bind members
        for child in &node.children {
            if let crate::syntax::SyntaxElement::Node(child_node) = child {
                match child_node.kind {
                    SyntaxKind::MethodDeclaration => self.bind_method(child_node),
                    SyntaxKind::PropertyDeclaration => self.bind_property(child_node),
                    SyntaxKind::FieldDeclaration => self.bind_field(child_node),
                    _ => {}
                }
            }
        }
        
        // Exit type scope
        self.scope.pop();
    }
    
    /// Bind a method
    fn bind_method(&mut self, node: &SyntaxNode) {
        // Enter method scope
        self.scope.push_method_scope(self.get_method_name(node));
        
        // Bind parameters
        self.bind_parameters(node);
        
        // Bind method body
        // TODO: Fix borrowing issue
        // let has_body = self.find_method_body(node).is_some();
        // if has_body {
        //     if let Some(body) = self.find_method_body(node) {
        //         self.bind_statement(body);
        //     }
        // }
        
        // Exit method scope
        self.scope.pop();
    }
    
    /// Bind parameters
    fn bind_parameters(&mut self, method_node: &SyntaxNode) {
        // Find parameter list and bind each parameter
        // This is simplified - would need to properly traverse the AST
    }
    
    /// Bind a property
    fn bind_property(&mut self, node: &SyntaxNode) {
        // Bind property accessors
    }
    
    /// Bind a field
    fn bind_field(&mut self, node: &SyntaxNode) {
        // Bind field initializer if present
    }
    
    /// Bind a statement
    fn bind_statement(&mut self, node: &SyntaxNode) -> BoundNode {
        match node.kind {
            SyntaxKind::Block => self.bind_block(node),
            SyntaxKind::IfStatement => self.bind_if_statement(node),
            SyntaxKind::WhileStatement => self.bind_while_statement(node),
            SyntaxKind::ForStatement => self.bind_for_statement(node),
            SyntaxKind::ReturnStatement => self.bind_return_statement(node),
            SyntaxKind::LocalDeclarationStatement => self.bind_local_declaration(node),
            SyntaxKind::ExpressionStatement => self.bind_expression_statement(node),
            _ => {
                self.diagnostics.add(Diagnostic::error(
                    DiagnosticCode::UnexpectedSyntax,
                    format!("Unexpected statement kind: {:?}", node.kind),
                ));
                BoundNode {
                    kind: BoundNodeKind::Expression {
                        expression: Box::new(self.error_node()),
                    },
                    result_type: None,
                }
            }
        }
    }
    
    /// Bind a block statement
    fn bind_block(&mut self, node: &SyntaxNode) -> BoundNode {
        self.scope.push_block_scope();
        
        let mut statements = Vec::new();
        for child in &node.children {
            if let crate::syntax::SyntaxElement::Node(child_node) = child {
                if child_node.kind != SyntaxKind::OpenBraceToken &&
                   child_node.kind != SyntaxKind::CloseBraceToken {
                    statements.push(self.bind_statement(child_node));
                }
            }
        }
        
        self.scope.pop();
        
        BoundNode {
            kind: BoundNodeKind::Block { statements },
            result_type: None,
        }
    }
    
    /// Bind an if statement
    fn bind_if_statement(&mut self, node: &SyntaxNode) -> BoundNode {
        let condition = self.bind_expression(&self.find_condition(node));
        let then_statement = self.bind_statement(&self.find_then_statement(node));
        // TODO: Fix borrowing issue
        let else_statement = None;
        // let else_statement = if let Some(else_stmt) = self.find_else_statement(node) {
        //     Some(Box::new(self.bind_statement(else_stmt)))
        // } else {
        //     None
        // };
        
        BoundNode {
            kind: BoundNodeKind::If {
                condition: Box::new(condition),
                then_statement: Box::new(then_statement),
                else_statement,
            },
            result_type: None,
        }
    }
    
    /// Bind a while statement
    fn bind_while_statement(&mut self, node: &SyntaxNode) -> BoundNode {
        let condition = self.bind_expression(&self.find_condition(node));
        let body = self.bind_statement(&self.find_body(node));
        
        BoundNode {
            kind: BoundNodeKind::While {
                condition: Box::new(condition),
                body: Box::new(body),
            },
            result_type: None,
        }
    }
    
    /// Bind a for statement
    fn bind_for_statement(&mut self, node: &SyntaxNode) -> BoundNode {
        // For statements are complex - simplified here
        self.bind_block(node)
    }
    
    /// Bind a return statement
    fn bind_return_statement(&mut self, node: &SyntaxNode) -> BoundNode {
        // TODO: Fix borrowing issue
        let value = None;
        // let value = if let Some(expr) = self.find_return_expression(node) {
        //     Some(Box::new(self.bind_expression(expr)))
        // } else {
        //     None
        // };
        
        BoundNode {
            kind: BoundNodeKind::Return { value },
            result_type: None,
        }
    }
    
    /// Bind a local declaration
    fn bind_local_declaration(&mut self, node: &SyntaxNode) -> BoundNode {
        let name = self.get_variable_name(node);
        // TODO: Fix borrowing issue
        let initializer = None;
        // let initializer = if let Some(init) = self.find_initializer(node) {
        //     Some(Box::new(self.bind_expression(init)))
        // } else {
        //     None
        // };
        
        // Add to scope
        if let Some(var_type) = self.get_variable_type(node) {
            self.scope.add_local(LocalSymbol {
                name: name.clone(),
                symbol_type: var_type,
                is_readonly: false,
                kind: LocalKind::Variable,
            });
        }
        
        BoundNode {
            kind: BoundNodeKind::LocalDeclaration { name, initializer },
            result_type: None,
        }
    }
    
    /// Bind an expression statement
    fn bind_expression_statement(&mut self, node: &SyntaxNode) -> BoundNode {
        // TODO: Fix borrowing issue
        let expr = self.error_node();
        // let expr = if let Some(expression) = self.find_expression(node) {
        //     self.bind_expression(expression)
        // } else {
        //     self.error_node()
        // };
        
        BoundNode {
            kind: BoundNodeKind::Expression {
                expression: Box::new(expr),
            },
            result_type: None,
        }
    }
    
    /// Bind an expression
    fn bind_expression(&mut self, node: &SyntaxNode) -> BoundNode {
        match node.kind {
            SyntaxKind::TrueLiteralExpression => self.bind_boolean_literal(true),
            SyntaxKind::FalseLiteralExpression => self.bind_boolean_literal(false),
            SyntaxKind::NumericLiteralExpression => self.bind_numeric_literal(node),
            SyntaxKind::StringLiteralExpression => self.bind_string_literal(node),
            SyntaxKind::CharacterLiteralExpression => self.bind_character_literal(node),
            SyntaxKind::NullLiteralExpression => self.bind_null_literal(),
            SyntaxKind::IdentifierName => self.bind_identifier(node),
            SyntaxKind::SimpleMemberAccessExpression => self.bind_member_access(node),
            SyntaxKind::InvocationExpression => self.bind_invocation(node),
            SyntaxKind::SimpleAssignmentExpression => self.bind_assignment(node),
            SyntaxKind::AddExpression |
            SyntaxKind::SubtractExpression |
            SyntaxKind::MultiplyExpression |
            SyntaxKind::DivideExpression => self.bind_binary_expression(node),
            SyntaxKind::UnaryPlusExpression |
            SyntaxKind::UnaryMinusExpression |
            SyntaxKind::LogicalNotExpression => self.bind_unary_expression(node),
            SyntaxKind::CastExpression => self.bind_cast_expression(node),
            SyntaxKind::ParenthesizedExpression => self.bind_parenthesized_expression(node),
            _ => {
                self.diagnostics.add(Diagnostic::error(
                    DiagnosticCode::UnexpectedSyntax,
                    format!("Unexpected expression kind: {:?}", node.kind),
                ));
                self.error_node()
            }
        }
    }
    
    /// Bind a boolean literal
    fn bind_boolean_literal(&mut self, value: bool) -> BoundNode {
        BoundNode {
            kind: BoundNodeKind::Literal(LiteralValue::Boolean(value)),
            result_type: Some(self.get_boolean_type()),
        }
    }
    
    /// Bind a numeric literal
    fn bind_numeric_literal(&mut self, node: &SyntaxNode) -> BoundNode {
        // Extract value from token - simplified
        let value = 0i64; // Would parse from token text
        
        BoundNode {
            kind: BoundNodeKind::Literal(LiteralValue::Integer(value)),
            result_type: Some(self.get_int_type()),
        }
    }
    
    /// Bind a string literal
    fn bind_string_literal(&mut self, node: &SyntaxNode) -> BoundNode {
        // Extract value from token - simplified
        let value = String::new(); // Would parse from token text
        
        BoundNode {
            kind: BoundNodeKind::Literal(LiteralValue::String(value)),
            result_type: Some(self.get_string_type()),
        }
    }
    
    /// Bind a character literal
    fn bind_character_literal(&mut self, node: &SyntaxNode) -> BoundNode {
        // Extract value from token - simplified
        let value = '\0'; // Would parse from token text
        
        BoundNode {
            kind: BoundNodeKind::Literal(LiteralValue::Character(value)),
            result_type: Some(self.get_char_type()),
        }
    }
    
    /// Bind a null literal
    fn bind_null_literal(&mut self) -> BoundNode {
        BoundNode {
            kind: BoundNodeKind::Literal(LiteralValue::Null),
            result_type: Some(self.get_object_type()),
        }
    }
    
    /// Bind an identifier
    fn bind_identifier(&mut self, node: &SyntaxNode) -> BoundNode {
        let name = self.get_identifier_text(node);
        
        // Look up in scope
        if let Some(local) = self.scope.lookup_local(&name) {
            BoundNode {
                kind: BoundNodeKind::Identifier(name),
                result_type: Some(local.symbol_type.clone()),
            }
        } else {
            // Look up in symbol table
            if let Some(symbol) = self.symbol_table.lookup(&name) {
                BoundNode {
                    kind: BoundNodeKind::Identifier(name),
                    result_type: self.get_symbol_type(symbol),
                }
            } else {
                self.diagnostics.add(Diagnostic::error(
                    DiagnosticCode::UndefinedName,
                    format!("Undefined name: {}", name),
                ));
                self.error_node()
            }
        }
    }
    
    /// Bind member access
    fn bind_member_access(&mut self, node: &SyntaxNode) -> BoundNode {
        // Simplified implementation
        self.error_node()
    }
    
    /// Bind invocation
    fn bind_invocation(&mut self, node: &SyntaxNode) -> BoundNode {
        // Simplified implementation
        self.error_node()
    }
    
    /// Bind assignment
    fn bind_assignment(&mut self, node: &SyntaxNode) -> BoundNode {
        // Simplified implementation
        self.error_node()
    }
    
    /// Bind binary expression
    fn bind_binary_expression(&mut self, node: &SyntaxNode) -> BoundNode {
        // Simplified implementation
        self.error_node()
    }
    
    /// Bind unary expression
    fn bind_unary_expression(&mut self, node: &SyntaxNode) -> BoundNode {
        // Simplified implementation
        self.error_node()
    }
    
    /// Bind cast expression
    fn bind_cast_expression(&mut self, node: &SyntaxNode) -> BoundNode {
        // Simplified implementation
        self.error_node()
    }
    
    /// Bind parenthesized expression
    fn bind_parenthesized_expression(&mut self, node: &SyntaxNode) -> BoundNode {
        // Find the inner expression and bind it
        self.error_node()
    }
    
    // Helper methods
    
    fn error_node(&self) -> BoundNode {
        BoundNode {
            kind: BoundNodeKind::Literal(LiteralValue::Null),
            result_type: Some(self.get_error_type()),
        }
    }
    
    fn get_boolean_type(&self) -> Rc<TypeSymbol> {
        Rc::new(TypeSymbol::new("bool".to_string(), TypeKind::Boolean))
    }
    
    fn get_int_type(&self) -> Rc<TypeSymbol> {
        Rc::new(TypeSymbol::new("int".to_string(), TypeKind::Int))
    }
    
    fn get_string_type(&self) -> Rc<TypeSymbol> {
        Rc::new(TypeSymbol::new("string".to_string(), TypeKind::String))
    }
    
    fn get_char_type(&self) -> Rc<TypeSymbol> {
        Rc::new(TypeSymbol::new("char".to_string(), TypeKind::Char))
    }
    
    fn get_object_type(&self) -> Rc<TypeSymbol> {
        Rc::new(TypeSymbol::new("object".to_string(), TypeKind::Object))
    }
    
    fn get_error_type(&self) -> Rc<TypeSymbol> {
        Rc::new(TypeSymbol::new("<error>".to_string(), TypeKind::Error))
    }
    
    fn get_symbol_type(&self, symbol: &Symbol) -> Option<Rc<TypeSymbol>> {
        // Convert symbol to type - simplified
        None
    }
    
    // AST navigation helpers - these would properly traverse the tree
    
    fn get_type_name(&self, node: &SyntaxNode) -> String {
        "UnknownType".to_string()
    }
    
    fn get_method_name(&self, node: &SyntaxNode) -> String {
        "UnknownMethod".to_string()
    }
    
    fn get_variable_name(&self, node: &SyntaxNode) -> String {
        "UnknownVariable".to_string()
    }
    
    fn get_variable_type(&self, node: &SyntaxNode) -> Option<Rc<TypeSymbol>> {
        Some(self.get_object_type())
    }
    
    fn get_identifier_text(&self, node: &SyntaxNode) -> String {
        "UnknownIdentifier".to_string()
    }
    
    fn find_method_body(&self, node: &SyntaxNode) -> Option<&SyntaxNode> {
        None
    }
    
    fn find_condition<'b>(&self, node: &'b SyntaxNode) -> &'b SyntaxNode {
        node // Simplified
    }
    
    fn find_then_statement<'b>(&self, node: &'b SyntaxNode) -> &'b SyntaxNode {
        node // Simplified
    }
    
    fn find_else_statement(&self, _node: &SyntaxNode) -> Option<&SyntaxNode> {
        None
    }
    
    fn find_body<'b>(&self, node: &'b SyntaxNode) -> &'b SyntaxNode {
        node // Simplified
    }
    
    fn find_return_expression(&self, _node: &SyntaxNode) -> Option<&SyntaxNode> {
        None
    }
    
    fn find_initializer(&self, _node: &SyntaxNode) -> Option<&SyntaxNode> {
        None
    }
    
    fn find_expression(&self, _node: &SyntaxNode) -> Option<&SyntaxNode> {
        None
    }
}

impl ScopeStack {
    fn new() -> Self {
        Self {
            scopes: vec![Scope {
                locals: HashMap::new(),
                containing_type: None,
                containing_method: None,
            }],
        }
    }
    
    fn push_block_scope(&mut self) {
        let parent = self.current();
        self.scopes.push(Scope {
            locals: HashMap::new(),
            containing_type: parent.containing_type.clone(),
            containing_method: parent.containing_method.clone(),
        });
    }
    
    fn push_type_scope(&mut self, type_name: String) {
        self.scopes.push(Scope {
            locals: HashMap::new(),
            containing_type: Some(type_name),
            containing_method: None,
        });
    }
    
    fn push_method_scope(&mut self, method_name: String) {
        let parent = self.current();
        self.scopes.push(Scope {
            locals: HashMap::new(),
            containing_type: parent.containing_type.clone(),
            containing_method: Some(method_name),
        });
    }
    
    fn pop(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }
    
    fn current(&self) -> &Scope {
        self.scopes.last().unwrap()
    }
    
    fn current_mut(&mut self) -> &mut Scope {
        self.scopes.last_mut().unwrap()
    }
    
    fn add_local(&mut self, local: LocalSymbol) {
        self.current_mut().locals.insert(local.name.clone(), local);
    }
    
    fn lookup_local(&self, name: &str) -> Option<&LocalSymbol> {
        // Search from innermost to outermost scope
        for scope in self.scopes.iter().rev() {
            if let Some(local) = scope.locals.get(name) {
                return Some(local);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_scope_stack() {
        let mut scope = ScopeStack::new();
        
        // Add a local to global scope
        scope.add_local(LocalSymbol {
            name: "x".to_string(),
            symbol_type: Rc::new(TypeSymbol::new("int".to_string(), TypeKind::Int)),
            is_readonly: false,
            kind: LocalKind::Variable,
        });
        
        assert!(scope.lookup_local("x").is_some());
        assert!(scope.lookup_local("y").is_none());
        
        // Enter a new scope
        scope.push_block_scope();
        
        // Shadow x in inner scope
        scope.add_local(LocalSymbol {
            name: "x".to_string(),
            symbol_type: Rc::new(TypeSymbol::new("string".to_string(), TypeKind::String)),
            is_readonly: false,
            kind: LocalKind::Variable,
        });
        
        // Should find the inner x
        let x = scope.lookup_local("x").unwrap();
        assert_eq!(x.symbol_type.kind, TypeKind::String);
        
        // Exit inner scope
        scope.pop();
        
        // Should find the outer x again
        let x = scope.lookup_local("x").unwrap();
        assert_eq!(x.symbol_type.kind, TypeKind::Int);
    }
}