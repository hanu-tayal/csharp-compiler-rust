//! Type checking and resolution for C# code

use super::types::{TypeSymbol, TypeKind};
use super::symbols::{Symbol, SymbolKind, SymbolTable};
use super::binding::{BoundNode, BoundNodeKind, BinaryOperator, UnaryOperator, LiteralValue};
use super::generics::{GenericResolver, TypeReference};
use crate::syntax::{SyntaxNode, SyntaxKind};
use crate::diagnostics::DiagnosticBag;
use std::rc::Rc;
use std::collections::HashMap;

/// Type checker for semantic analysis
pub struct TypeChecker {
    /// Symbol table
    symbol_table: SymbolTable,
    /// Current scope
    current_scope: usize,
    /// Type cache
    type_cache: HashMap<String, Rc<TypeSymbol>>,
    /// Diagnostics
    diagnostics: DiagnosticBag,
    /// Generic resolver
    generic_resolver: GenericResolver,
}

impl TypeChecker {
    /// Create a new type checker
    pub fn new() -> Self {
        let mut checker = Self {
            symbol_table: SymbolTable::new(),
            current_scope: 0,
            type_cache: HashMap::new(),
            diagnostics: DiagnosticBag::new(),
            generic_resolver: GenericResolver::new(),
        };
        
        // Initialize built-in types
        checker.initialize_builtin_types();
        
        checker
    }
    
    /// Initialize built-in types
    fn initialize_builtin_types(&mut self) {
        // Primitive types
        self.register_builtin_type("void", TypeKind::Void);
        self.register_builtin_type("bool", TypeKind::Boolean);
        self.register_builtin_type("byte", TypeKind::Byte);
        self.register_builtin_type("sbyte", TypeKind::SByte);
        self.register_builtin_type("short", TypeKind::Short);
        self.register_builtin_type("ushort", TypeKind::UShort);
        self.register_builtin_type("int", TypeKind::Int);
        self.register_builtin_type("uint", TypeKind::UInt);
        self.register_builtin_type("long", TypeKind::Long);
        self.register_builtin_type("ulong", TypeKind::ULong);
        self.register_builtin_type("float", TypeKind::Float);
        self.register_builtin_type("double", TypeKind::Double);
        self.register_builtin_type("decimal", TypeKind::Decimal);
        self.register_builtin_type("char", TypeKind::Char);
        self.register_builtin_type("string", TypeKind::String);
        self.register_builtin_type("object", TypeKind::Object);
        
        // Type aliases
        self.register_type_alias("System.Void", "void");
        self.register_type_alias("System.Boolean", "bool");
        self.register_type_alias("System.Byte", "byte");
        self.register_type_alias("System.SByte", "sbyte");
        self.register_type_alias("System.Int16", "short");
        self.register_type_alias("System.UInt16", "ushort");
        self.register_type_alias("System.Int32", "int");
        self.register_type_alias("System.UInt32", "uint");
        self.register_type_alias("System.Int64", "long");
        self.register_type_alias("System.UInt64", "ulong");
        self.register_type_alias("System.Single", "float");
        self.register_type_alias("System.Double", "double");
        self.register_type_alias("System.Decimal", "decimal");
        self.register_type_alias("System.Char", "char");
        self.register_type_alias("System.String", "string");
        self.register_type_alias("System.Object", "object");
    }
    
    /// Register a built-in type
    fn register_builtin_type(&mut self, name: &str, kind: TypeKind) {
        let type_symbol = Rc::new(TypeSymbol::new(name.to_string(), kind));
        
        self.type_cache.insert(name.to_string(), type_symbol.clone());
        
        let symbol = Symbol {
            name: name.to_string(),
            kind: SymbolKind::Type(type_symbol),
            is_public: true,
            is_static: false,
            is_readonly: false,
            scope_id: 0,
        };
        
        self.symbol_table.add_symbol(symbol);
    }
    
    /// Register a type alias
    fn register_type_alias(&mut self, full_name: &str, alias: &str) {
        if let Some(type_symbol) = self.type_cache.get(alias).cloned() {
            self.type_cache.insert(full_name.to_string(), type_symbol);
        }
    }
    
    /// Check types in a compilation unit
    pub fn check_compilation_unit(&mut self, node: &SyntaxNode) -> Result<(), String> {
        // Process using directives
        self.process_using_directives(node)?;
        
        // Process type declarations
        self.process_type_declarations(node)?;
        
        // Check all method bodies
        self.check_method_bodies(node)?;
        
        Ok(())
    }
    
    /// Process using directives
    fn process_using_directives(&mut self, node: &SyntaxNode) -> Result<(), String> {
        for child in &node.children {
            if let crate::syntax::SyntaxElement::Node(n) = child {
                if n.kind == SyntaxKind::UsingDirective {
                    // TODO: Add namespace to imports
                }
            }
        }
        Ok(())
    }
    
    /// Process type declarations
    fn process_type_declarations(&mut self, node: &SyntaxNode) -> Result<(), String> {
        for child in &node.children {
            if let crate::syntax::SyntaxElement::Node(n) = child {
                match n.kind {
                    SyntaxKind::ClassDeclaration |
                    SyntaxKind::StructDeclaration |
                    SyntaxKind::InterfaceDeclaration |
                    SyntaxKind::EnumDeclaration => {
                        self.process_type_declaration(n)?;
                    }
                    SyntaxKind::NamespaceDeclaration => {
                        self.process_type_declarations(n)?;
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
    
    /// Process a single type declaration
    fn process_type_declaration(&mut self, node: &SyntaxNode) -> Result<(), String> {
        let type_name = self.get_type_name(node)?;
        let type_kind = match node.kind {
            SyntaxKind::ClassDeclaration => TypeKind::Class,
            SyntaxKind::StructDeclaration => TypeKind::Struct,
            SyntaxKind::InterfaceDeclaration => TypeKind::Interface,
            SyntaxKind::EnumDeclaration => TypeKind::Enum,
            _ => return Err("Invalid type declaration".to_string()),
        };
        
        let mut type_symbol = TypeSymbol::new(type_name.clone(), type_kind);
        type_symbol.base_type = self.get_base_type(node);
        type_symbol.interfaces = self.get_interfaces(node);
        let type_symbol = Rc::new(type_symbol);
        
        self.type_cache.insert(type_name.clone(), type_symbol.clone());
        
        let symbol = Symbol {
            name: type_name,
            kind: SymbolKind::Type(type_symbol),
            is_public: self.has_modifier(node, "public"),
            is_static: self.has_modifier(node, "static"),
            is_readonly: false,
            scope_id: self.current_scope,
        };
        
        self.symbol_table.add_symbol(symbol);
        
        // Process members
        self.process_type_members(node)?;
        
        Ok(())
    }
    
    /// Get type name from declaration
    fn get_type_name(&self, node: &SyntaxNode) -> Result<String, String> {
        for child in &node.children {
            if let crate::syntax::SyntaxElement::Token(t) = child {
                if t.kind == SyntaxKind::IdentifierToken {
                    return Ok(t.text.to_string());
                }
            }
        }
        Err("Type declaration missing name".to_string())
    }
    
    /// Get base type from declaration
    fn get_base_type(&self, node: &SyntaxNode) -> Option<Rc<TypeSymbol>> {
        // TODO: Parse base list and resolve base type
        None
    }
    
    /// Get interfaces from declaration
    fn get_interfaces(&self, node: &SyntaxNode) -> Vec<Rc<TypeSymbol>> {
        // TODO: Parse base list and resolve interfaces
        Vec::new()
    }
    
    /// Check if declaration has a modifier
    fn has_modifier(&self, node: &SyntaxNode, modifier: &str) -> bool {
        for child in &node.children {
            if let crate::syntax::SyntaxElement::Token(t) = child {
                if t.text == modifier {
                    return true;
                }
            }
        }
        false
    }
    
    /// Process type members
    fn process_type_members(&mut self, type_node: &SyntaxNode) -> Result<(), String> {
        for child in &type_node.children {
            if let crate::syntax::SyntaxElement::Node(n) = child {
                match n.kind {
                    SyntaxKind::FieldDeclaration => self.process_field(n)?,
                    SyntaxKind::PropertyDeclaration => self.process_property(n)?,
                    SyntaxKind::MethodDeclaration => self.process_method(n)?,
                    SyntaxKind::ConstructorDeclaration => self.process_constructor(n)?,
                    _ => {}
                }
            }
        }
        Ok(())
    }
    
    /// Process field declaration
    fn process_field(&mut self, node: &SyntaxNode) -> Result<(), String> {
        // TODO: Extract field info and add to symbol table
        Ok(())
    }
    
    /// Process property declaration
    fn process_property(&mut self, node: &SyntaxNode) -> Result<(), String> {
        // TODO: Extract property info and add to symbol table
        Ok(())
    }
    
    /// Process method declaration
    fn process_method(&mut self, node: &SyntaxNode) -> Result<(), String> {
        // TODO: Extract method info and add to symbol table
        Ok(())
    }
    
    /// Process constructor declaration
    fn process_constructor(&mut self, node: &SyntaxNode) -> Result<(), String> {
        // TODO: Extract constructor info and add to symbol table
        Ok(())
    }
    
    /// Check all method bodies
    fn check_method_bodies(&mut self, node: &SyntaxNode) -> Result<(), String> {
        // TODO: Traverse tree and check each method body
        Ok(())
    }
    
    /// Check types in an expression
    pub fn check_expression(&mut self, expr: &BoundNode) -> Result<Rc<TypeSymbol>, String> {
        match &expr.kind {
            BoundNodeKind::Literal(lit) => self.check_literal(lit),
            BoundNodeKind::Identifier(name) => self.check_identifier(name),
            BoundNodeKind::Binary { operator, left, right } => {
                self.check_binary_expression(*operator, left, right)
            }
            BoundNodeKind::Unary { operator, operand } => {
                self.check_unary_expression(*operator, operand)
            }
            BoundNodeKind::Assignment { target, value } => {
                self.check_assignment(target, value)
            }
            BoundNodeKind::Call { method, arguments } => {
                // TODO: Properly implement method call type checking
                // For now, return a placeholder type
                Ok(Rc::new(TypeSymbol::new("object".to_string(), TypeKind::Object)))
            }
            BoundNodeKind::MemberAccess { instance, member } => {
                self.check_member_access(instance, &member)
            }
            _ => Err("Unsupported expression type".to_string()),
        }
    }
    
    /// Check literal type
    fn check_literal(&self, literal: &LiteralValue) -> Result<Rc<TypeSymbol>, String> {
        let type_name = match literal {
            LiteralValue::Boolean(_) => "bool",
            LiteralValue::Integer(_) => "int",
            LiteralValue::Float(_) => "double",
            LiteralValue::String(_) => "string",
            LiteralValue::Character(_) => "char",
            LiteralValue::Null => "object",
        };
        
        self.type_cache.get(type_name)
            .cloned()
            .ok_or_else(|| format!("Type '{}' not found", type_name))
    }
    
    /// Check identifier type
    fn check_identifier(&self, name: &str) -> Result<Rc<TypeSymbol>, String> {
        self.symbol_table.lookup(name)
            .and_then(|symbol| match &symbol.kind {
                SymbolKind::Variable(var_type) => Some(var_type.clone()),
                SymbolKind::Parameter(param_type) => Some(param_type.clone()),
                SymbolKind::Field(field_type) => Some(field_type.clone()),
                _ => None,
            })
            .ok_or_else(|| format!("Identifier '{}' not found", name))
    }
    
    /// Check binary expression type
    fn check_binary_expression(
        &mut self,
        operator: BinaryOperator,
        left: &BoundNode,
        right: &BoundNode,
    ) -> Result<Rc<TypeSymbol>, String> {
        let left_type = self.check_expression(left)?;
        let right_type = self.check_expression(right)?;
        
        // Type compatibility check
        match operator {
            BinaryOperator::Add | BinaryOperator::Subtract |
            BinaryOperator::Multiply | BinaryOperator::Divide => {
                if self.is_numeric_type(&left_type) && self.is_numeric_type(&right_type) {
                    // Numeric promotion rules
                    self.promote_numeric_types(&left_type, &right_type)
                } else if left_type.kind == TypeKind::String && operator == BinaryOperator::Add {
                    // String concatenation
                    Ok(left_type)
                } else {
                    Err(format!("Invalid operands for {:?}", operator))
                }
            }
            BinaryOperator::Equals | BinaryOperator::NotEquals => {
                if self.are_compatible_types(&left_type, &right_type) {
                    self.get_bool_type()
                } else {
                    Err("Incompatible types for equality comparison".to_string())
                }
            }
            BinaryOperator::LessThan | BinaryOperator::LessThanOrEqual |
            BinaryOperator::GreaterThan | BinaryOperator::GreaterThanOrEqual => {
                if self.is_numeric_type(&left_type) && self.is_numeric_type(&right_type) {
                    self.get_bool_type()
                } else {
                    Err("Comparison operators require numeric types".to_string())
                }
            }
            BinaryOperator::And | BinaryOperator::Or => {
                if left_type.kind == TypeKind::Boolean && right_type.kind == TypeKind::Boolean {
                    Ok(left_type)
                } else {
                    Err("Logical operators require boolean operands".to_string())
                }
            }
            _ => Err("Unsupported binary operator".to_string()),
        }
    }
    
    /// Check unary expression type
    fn check_unary_expression(
        &mut self,
        operator: UnaryOperator,
        operand: &BoundNode,
    ) -> Result<Rc<TypeSymbol>, String> {
        let operand_type = self.check_expression(operand)?;
        
        match operator {
            UnaryOperator::Plus | UnaryOperator::Minus => {
                if self.is_numeric_type(&operand_type) {
                    Ok(operand_type)
                } else {
                    Err("Unary +/- requires numeric operand".to_string())
                }
            }
            UnaryOperator::LogicalNot => {
                if operand_type.kind == TypeKind::Boolean {
                    Ok(operand_type)
                } else {
                    Err("Logical NOT requires boolean operand".to_string())
                }
            }
            UnaryOperator::BitwiseNot => {
                if self.is_integral_type(&operand_type) {
                    Ok(operand_type)
                } else {
                    Err("Bitwise NOT requires integral operand".to_string())
                }
            }
            _ => Err("Unsupported unary operator".to_string()),
        }
    }
    
    /// Check assignment type compatibility
    fn check_assignment(
        &mut self,
        target: &BoundNode,
        value: &BoundNode,
    ) -> Result<Rc<TypeSymbol>, String> {
        let target_type = self.check_expression(target)?;
        let value_type = self.check_expression(value)?;
        
        if self.is_assignable(&target_type, &value_type) {
            Ok(value_type)
        } else {
            Err(format!(
                "Cannot assign '{}' to '{}'",
                value_type.name, target_type.name
            ))
        }
    }
    
    /// Check method call
    fn check_call(
        &mut self,
        target: &BoundNode,
        arguments: &[BoundNode],
    ) -> Result<Rc<TypeSymbol>, String> {
        // TODO: Implement method resolution and overload resolution
        Err("Method calls not yet implemented".to_string())
    }
    
    /// Check member access
    fn check_member_access(
        &mut self,
        object: &BoundNode,
        member: &str,
    ) -> Result<Rc<TypeSymbol>, String> {
        let object_type = self.check_expression(object)?;
        
        // Look up member in object type
        if let Some(member_type) = object_type.members.get(member) {
            // Extract the appropriate type from the member
            let result_type = match &member_type.type_info {
                super::types::MemberTypeInfo::Method { return_type, .. } => return_type.clone(),
                super::types::MemberTypeInfo::Property { property_type, .. } => property_type.clone(),
                super::types::MemberTypeInfo::Field { field_type } => field_type.clone(),
                super::types::MemberTypeInfo::Event { event_type } => event_type.clone(),
            };
            Ok(result_type)
        } else {
            Err(format!(
                "Type '{}' does not contain member '{}'",
                object_type.name, member
            ))
        }
    }
    
    /// Check if type is numeric
    fn is_numeric_type(&self, type_symbol: &TypeSymbol) -> bool {
        matches!(
            type_symbol.kind,
            TypeKind::Byte | TypeKind::SByte |
            TypeKind::Short | TypeKind::UShort |
            TypeKind::Int | TypeKind::UInt |
            TypeKind::Long | TypeKind::ULong |
            TypeKind::Float | TypeKind::Double |
            TypeKind::Decimal
        )
    }
    
    /// Check if type is integral
    fn is_integral_type(&self, type_symbol: &TypeSymbol) -> bool {
        matches!(
            type_symbol.kind,
            TypeKind::Byte | TypeKind::SByte |
            TypeKind::Short | TypeKind::UShort |
            TypeKind::Int | TypeKind::UInt |
            TypeKind::Long | TypeKind::ULong
        )
    }
    
    /// Promote numeric types
    fn promote_numeric_types(
        &self,
        left: &TypeSymbol,
        right: &TypeSymbol,
    ) -> Result<Rc<TypeSymbol>, String> {
        // Simplified numeric promotion
        if left.kind == TypeKind::Double || right.kind == TypeKind::Double {
            self.get_double_type()
        } else if left.kind == TypeKind::Float || right.kind == TypeKind::Float {
            self.get_float_type()
        } else if left.kind == TypeKind::Long || right.kind == TypeKind::Long {
            self.get_long_type()
        } else {
            self.get_int_type()
        }
    }
    
    /// Check if types are compatible
    fn are_compatible_types(&self, left: &TypeSymbol, right: &TypeSymbol) -> bool {
        // Simplified compatibility check
        left.name == right.name || 
        (self.is_numeric_type(left) && self.is_numeric_type(right))
    }
    
    /// Check if value type is assignable to target type
    fn is_assignable(&self, target: &TypeSymbol, value: &TypeSymbol) -> bool {
        // Simplified assignability check
        target.name == value.name ||
        (self.is_numeric_type(target) && self.is_numeric_type(value)) ||
        (target.kind == TypeKind::Object) // Everything is assignable to object
    }
    
    /// Get bool type
    fn get_bool_type(&self) -> Result<Rc<TypeSymbol>, String> {
        self.type_cache.get("bool")
            .cloned()
            .ok_or_else(|| "Bool type not found".to_string())
    }
    
    /// Get int type
    fn get_int_type(&self) -> Result<Rc<TypeSymbol>, String> {
        self.type_cache.get("int")
            .cloned()
            .ok_or_else(|| "Int type not found".to_string())
    }
    
    /// Get long type
    fn get_long_type(&self) -> Result<Rc<TypeSymbol>, String> {
        self.type_cache.get("long")
            .cloned()
            .ok_or_else(|| "Long type not found".to_string())
    }
    
    /// Get float type
    fn get_float_type(&self) -> Result<Rc<TypeSymbol>, String> {
        self.type_cache.get("float")
            .cloned()
            .ok_or_else(|| "Float type not found".to_string())
    }
    
    /// Get double type
    fn get_double_type(&self) -> Result<Rc<TypeSymbol>, String> {
        self.type_cache.get("double")
            .cloned()
            .ok_or_else(|| "Double type not found".to_string())
    }
    
    /// Get diagnostics
    pub fn diagnostics(&self) -> &DiagnosticBag {
        &self.diagnostics
    }
}