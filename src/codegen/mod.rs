//! Code generation module for the C# compiler
//! 
//! This module handles the generation of .NET IL bytecode from the semantic model.

use crate::semantic::SemanticModel;
use crate::semantic::binding::{BoundNode, BoundNodeKind, LiteralValue, BinaryOperator, UnaryOperator};
use crate::semantic::types::{TypeSymbol, TypeKind};
use crate::diagnostics::DiagnosticBag;

use std::collections::HashMap;
use std::rc::Rc;

pub mod emitter;
pub mod metadata;
pub mod il;
pub mod pe;

use self::emitter::ILEmitter;
use self::metadata::MetadataBuilder;
use self::il::{ILInstruction, ILMethod, ILAssembly};
use self::pe::PEGenerator;

/// Code generator that produces .NET IL bytecode
pub struct CodeGenerator {
    /// IL Assembly being built
    assembly: ILAssembly,
    /// Semantic model
    semantic_model: SemanticModel,
    /// Current method being generated
    current_method: Option<ILMethod>,
    /// Variable indices for current method
    variables: HashMap<String, u16>,
    /// Type cache
    type_cache: HashMap<String, String>,
    /// Diagnostics
    diagnostics: DiagnosticBag,
    /// IL Emitter
    emitter: ILEmitter,
    /// Metadata builder
    metadata_builder: MetadataBuilder,
}

impl CodeGenerator {
    /// Create a new code generator
    pub fn new(semantic_model: SemanticModel, assembly_name: &str) -> Self {
        let assembly = ILAssembly::new(assembly_name);
        let emitter = ILEmitter::new();
        let metadata_builder = MetadataBuilder::new();
        
        Self {
            assembly,
            semantic_model,
            current_method: None,
            variables: HashMap::new(),
            type_cache: HashMap::new(),
            diagnostics: DiagnosticBag::new(),
            emitter,
            metadata_builder,
        }
    }
    
    /// Generate code for the entire compilation
    pub fn generate(&mut self) -> Result<Vec<u8>, String> {
        // Initialize built-in types
        self.initialize_types();
        
        // Generate code for all bound nodes
        // In a real implementation, we would traverse the bound tree
        // For now, we'll create a simple example
        self.generate_example();
        
        // Build the assembly
        self.assembly.finalize();
        
        // Generate PE file
        self.generate_pe_file()
    }
    
    /// Initialize built-in types
    fn initialize_types(&mut self) {
        // Integer types
        self.type_cache.insert("bool".to_string(), "System.Boolean".to_string());
        self.type_cache.insert("byte".to_string(), "System.Byte".to_string());
        self.type_cache.insert("sbyte".to_string(), "System.SByte".to_string());
        self.type_cache.insert("short".to_string(), "System.Int16".to_string());
        self.type_cache.insert("ushort".to_string(), "System.UInt16".to_string());
        self.type_cache.insert("int".to_string(), "System.Int32".to_string());
        self.type_cache.insert("uint".to_string(), "System.UInt32".to_string());
        self.type_cache.insert("long".to_string(), "System.Int64".to_string());
        self.type_cache.insert("ulong".to_string(), "System.UInt64".to_string());
        
        // Floating point types
        self.type_cache.insert("float".to_string(), "System.Single".to_string());
        self.type_cache.insert("double".to_string(), "System.Double".to_string());
        
        // Other types
        self.type_cache.insert("char".to_string(), "System.Char".to_string());
        self.type_cache.insert("string".to_string(), "System.String".to_string());
        self.type_cache.insert("object".to_string(), "System.Object".to_string());
        self.type_cache.insert("void".to_string(), "System.Void".to_string());
    }
    
    /// Generate example code
    fn generate_example(&mut self) {
        // Create Program class
        let program_class = self.assembly.add_class("Program", "System.Object");
        
        // Create Main method
        let mut main_method = ILMethod::new(
            "Main",
            vec!["System.String[]".to_string()],
            "System.Void".to_string(),
            true, // static
            false, // instance
        );
        
        // Generate simple Hello World
        main_method.add_instruction(ILInstruction::Ldstr("Hello, World!".to_string()));
        main_method.add_instruction(ILInstruction::Call("System.Console::WriteLine(System.String)".to_string()));
        main_method.add_instruction(ILInstruction::Ret);
        
        self.assembly.add_method(program_class, main_method);
        self.assembly.set_entry_point("Program::Main");
    }
    
    /// Generate code for a bound node
    pub fn generate_node(&mut self, node: &BoundNode) {
        match &node.kind {
            BoundNodeKind::Block { statements } => {
                for stmt in statements {
                    self.generate_node(stmt);
                }
            }
            BoundNodeKind::LocalDeclaration { name, initializer } => {
                self.generate_local_declaration(name, initializer.as_deref());
            }
            BoundNodeKind::Expression { expression } => {
                self.generate_expression(expression);
                // Pop the result if it's not used
                if let Some(method) = &mut self.current_method {
                    method.add_instruction(ILInstruction::Pop);
                }
            }
            BoundNodeKind::If { condition, then_statement, else_statement } => {
                self.generate_if_statement(condition, then_statement, else_statement.as_deref());
            }
            BoundNodeKind::While { condition, body } => {
                self.generate_while_statement(condition, body);
            }
            BoundNodeKind::Return { value } => {
                self.generate_return_statement(value.as_deref());
            }
            _ => {
                // Other node kinds
            }
        }
    }
    
    /// Generate a local variable declaration
    fn generate_local_declaration(&mut self, name: &str, initializer: Option<&BoundNode>) {
        // Early return if no current method
        if self.current_method.is_none() {
            return;
        }
        
        // Get variable index and update local count
        let var_index = {
            let method = self.current_method.as_mut().unwrap();
            let var_index = method.local_count;
            method.local_count += 1;
            var_index
        };
        
        // Store variable index
        self.variables.insert(name.to_string(), var_index);
        
        // Initialize if needed
        if let Some(init) = initializer {
            self.generate_expression(init);
            // Store to local variable
            if let Some(method) = &mut self.current_method {
                method.add_instruction(ILInstruction::Stloc(var_index));
            }
        }
    }
    
    /// Generate an expression
    fn generate_expression(&mut self, node: &BoundNode) {
        match &node.kind {
            BoundNodeKind::Literal(lit) => self.generate_literal(lit),
            BoundNodeKind::Identifier(name) => self.generate_identifier(name),
            BoundNodeKind::Binary { operator, left, right } => {
                self.generate_binary_expression(*operator, left, right)
            }
            BoundNodeKind::Unary { operator, operand } => {
                self.generate_unary_expression(*operator, operand)
            }
            BoundNodeKind::Assignment { target, value } => {
                self.generate_assignment(target, value)
            }
            _ => {},
        }
    }
    
    /// Generate a literal value
    fn generate_literal(&mut self, literal: &LiteralValue) {
        if let Some(method) = &mut self.current_method {
            match literal {
                LiteralValue::Boolean(b) => {
                    method.add_instruction(if *b { ILInstruction::Ldc_i4_1 } else { ILInstruction::Ldc_i4_0 });
                }
                LiteralValue::Integer(i) => {
                    match *i {
                        -1 => method.add_instruction(ILInstruction::Ldc_i4_m1),
                        0 => method.add_instruction(ILInstruction::Ldc_i4_0),
                        1 => method.add_instruction(ILInstruction::Ldc_i4_1),
                        2 => method.add_instruction(ILInstruction::Ldc_i4_2),
                        3 => method.add_instruction(ILInstruction::Ldc_i4_3),
                        4 => method.add_instruction(ILInstruction::Ldc_i4_4),
                        5 => method.add_instruction(ILInstruction::Ldc_i4_5),
                        6 => method.add_instruction(ILInstruction::Ldc_i4_6),
                        7 => method.add_instruction(ILInstruction::Ldc_i4_7),
                        8 => method.add_instruction(ILInstruction::Ldc_i4_8),
                        -128..=127 => method.add_instruction(ILInstruction::Ldc_i4_s(*i as i8)),
                        _ => method.add_instruction(ILInstruction::Ldc_i4(*i as i32)),
                    }
                }
                LiteralValue::Float(f) => {
                    method.add_instruction(ILInstruction::Ldc_r8(*f));
                }
                LiteralValue::String(s) => {
                    method.add_instruction(ILInstruction::Ldstr(s.clone()));
                }
                LiteralValue::Character(c) => {
                    method.add_instruction(ILInstruction::Ldc_i4(*c as i32));
                }
                LiteralValue::Null => {
                    method.add_instruction(ILInstruction::Ldnull);
                }
            }
        }
    }
    
    /// Generate an identifier reference
    fn generate_identifier(&mut self, name: &str) {
        if let Some(method) = &mut self.current_method {
            if let Some(&var_index) = self.variables.get(name) {
                // Load local variable
                match var_index {
                    0 => method.add_instruction(ILInstruction::Ldloc_0),
                    1 => method.add_instruction(ILInstruction::Ldloc_1),
                    2 => method.add_instruction(ILInstruction::Ldloc_2),
                    3 => method.add_instruction(ILInstruction::Ldloc_3),
                    _ => method.add_instruction(ILInstruction::Ldloc(var_index)),
                }
            }
        }
    }
    
    /// Generate a binary expression
    fn generate_binary_expression(
        &mut self,
        operator: BinaryOperator,
        left: &BoundNode,
        right: &BoundNode
    ) {
        // Early return if no current method
        if self.current_method.is_none() {
            return;
        }
        
        // Generate left operand
        self.generate_expression(left);
        // Generate right operand
        self.generate_expression(right);
        
        // Generate operator instructions
        let method = self.current_method.as_mut().unwrap();
        match operator {
            BinaryOperator::Add => method.add_instruction(ILInstruction::Add),
            BinaryOperator::Subtract => method.add_instruction(ILInstruction::Sub),
            BinaryOperator::Multiply => method.add_instruction(ILInstruction::Mul),
            BinaryOperator::Divide => method.add_instruction(ILInstruction::Div),
            BinaryOperator::Modulo => method.add_instruction(ILInstruction::Rem),
            BinaryOperator::Equals => method.add_instruction(ILInstruction::Ceq),
            BinaryOperator::NotEquals => {
                method.add_instruction(ILInstruction::Ceq);
                method.add_instruction(ILInstruction::Ldc_i4_0);
                method.add_instruction(ILInstruction::Ceq);
            }
            BinaryOperator::LessThan => method.add_instruction(ILInstruction::Clt),
            BinaryOperator::LessThanOrEqual => {
                method.add_instruction(ILInstruction::Cgt);
                method.add_instruction(ILInstruction::Ldc_i4_0);
                method.add_instruction(ILInstruction::Ceq);
            }
            BinaryOperator::GreaterThan => method.add_instruction(ILInstruction::Cgt),
            BinaryOperator::GreaterThanOrEqual => {
                method.add_instruction(ILInstruction::Clt);
                method.add_instruction(ILInstruction::Ldc_i4_0);
                method.add_instruction(ILInstruction::Ceq);
            }
            BinaryOperator::And => method.add_instruction(ILInstruction::And),
            BinaryOperator::Or => method.add_instruction(ILInstruction::Or),
            _ => {}, // Other operators
        }
    }
    
    /// Generate a unary expression
    fn generate_unary_expression(
        &mut self,
        operator: UnaryOperator,
        operand: &BoundNode
    ) {
        // Early return if no current method
        if self.current_method.is_none() {
            return;
        }
        
        // Generate operand
        self.generate_expression(operand);
        
        // Generate operator instructions
        let method = self.current_method.as_mut().unwrap();
        match operator {
            UnaryOperator::Minus => method.add_instruction(ILInstruction::Neg),
            UnaryOperator::LogicalNot => {
                method.add_instruction(ILInstruction::Ldc_i4_0);
                method.add_instruction(ILInstruction::Ceq);
            }
            UnaryOperator::BitwiseNot => method.add_instruction(ILInstruction::Not),
            UnaryOperator::Plus => {}, // No operation needed
            _ => {}, // Other operators
        }
    }
    
    /// Generate an assignment
    fn generate_assignment(
        &mut self,
        target: &BoundNode,
        value: &BoundNode
    ) {
        // Early return if no current method
        if self.current_method.is_none() {
            return;
        }
        
        if let BoundNodeKind::Identifier(name) = &target.kind {
            if let Some(&var_index) = self.variables.get(name) {
                // Generate value
                self.generate_expression(value);
                
                // Add assignment instructions
                let method = self.current_method.as_mut().unwrap();
                // Duplicate for assignment expression result
                method.add_instruction(ILInstruction::Dup);
                // Store to variable
                match var_index {
                    0 => method.add_instruction(ILInstruction::Stloc_0),
                    1 => method.add_instruction(ILInstruction::Stloc_1),
                    2 => method.add_instruction(ILInstruction::Stloc_2),
                    3 => method.add_instruction(ILInstruction::Stloc_3),
                    _ => method.add_instruction(ILInstruction::Stloc(var_index)),
                }
            }
        }
    }
    
    /// Generate an if statement
    fn generate_if_statement(
        &mut self,
        condition: &BoundNode,
        then_statement: &BoundNode,
        else_statement: Option<&BoundNode>
    ) {
        // Early return if no current method
        if self.current_method.is_none() {
            return;
        }
        
        // Generate condition
        self.generate_expression(condition);
        
        // Create labels and branch if false
        let (else_label, end_label) = {
            let method = self.current_method.as_mut().unwrap();
            let else_label = method.create_label();
            let end_label = method.create_label();
            method.add_instruction(ILInstruction::Brfalse(else_label));
            (else_label, end_label)
        };
        
        // Generate then branch
        self.generate_node(then_statement);
        
        // Jump to end and mark else label
        {
            let method = self.current_method.as_mut().unwrap();
            method.add_instruction(ILInstruction::Br(end_label));
            method.mark_label(else_label);
        }
        
        // Generate else branch if present
        if let Some(else_stmt) = else_statement {
            self.generate_node(else_stmt);
        }
        
        // Mark end label
        {
            let method = self.current_method.as_mut().unwrap();
            method.mark_label(end_label);
        }
    }
    
    /// Generate a while statement
    fn generate_while_statement(&mut self, condition: &BoundNode, body: &BoundNode) {
        // Early return if no current method
        if self.current_method.is_none() {
            return;
        }
        
        // Create labels and mark loop start
        let (loop_start, loop_end) = {
            let method = self.current_method.as_mut().unwrap();
            let loop_start = method.create_label();
            let loop_end = method.create_label();
            method.mark_label(loop_start);
            (loop_start, loop_end)
        };
        
        // Generate condition
        self.generate_expression(condition);
        
        // Branch if false
        {
            let method = self.current_method.as_mut().unwrap();
            method.add_instruction(ILInstruction::Brfalse(loop_end));
        }
        
        // Generate body
        self.generate_node(body);
        
        // Jump back to start and mark loop end
        {
            let method = self.current_method.as_mut().unwrap();
            method.add_instruction(ILInstruction::Br(loop_start));
            method.mark_label(loop_end);
        }
    }
    
    /// Generate a return statement
    fn generate_return_statement(&mut self, value: Option<&BoundNode>) {
        // Early return if no current method
        if self.current_method.is_none() {
            return;
        }
        
        if let Some(val_node) = value {
            self.generate_expression(val_node);
        }
        
        let method = self.current_method.as_mut().unwrap();
        method.add_instruction(ILInstruction::Ret);
    }
    
    /// Generate PE file
    fn generate_pe_file(&mut self) -> Result<Vec<u8>, String> {
        // Create PE generator
        let mut pe_generator = PEGenerator::new(self.assembly.clone());
        
        // Generate the PE file
        pe_generator.generate()
    }
    
    /// Get IL type signature for a C# type
    fn get_il_type_signature(&self, cs_type: &TypeSymbol) -> String {
        match cs_type.kind {
            TypeKind::Void => "void".to_string(),
            TypeKind::Boolean => "bool".to_string(),
            TypeKind::Byte => "unsigned int8".to_string(),
            TypeKind::SByte => "int8".to_string(),
            TypeKind::Short => "int16".to_string(),
            TypeKind::UShort => "unsigned int16".to_string(),
            TypeKind::Int => "int32".to_string(),
            TypeKind::UInt => "unsigned int32".to_string(),
            TypeKind::Long => "int64".to_string(),
            TypeKind::ULong => "unsigned int64".to_string(),
            TypeKind::Float => "float32".to_string(),
            TypeKind::Double => "float64".to_string(),
            TypeKind::Char => "char".to_string(),
            TypeKind::String => "string".to_string(),
            TypeKind::Object => "object".to_string(),
            _ => "object".to_string(), // Default to object for unknown types
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_code_generator_creation() {
        let syntax_tree = crate::syntax::SyntaxTree {
            root: crate::syntax::SyntaxNode {
                kind: crate::syntax::SyntaxKind::CompilationUnit,
                children: Vec::new(),
            },
        };
        let semantic_model = SemanticModel::new(syntax_tree);
        let mut codegen = CodeGenerator::new(semantic_model, "TestAssembly");
        
        let result = codegen.generate();
        assert!(result.is_ok());
    }
}