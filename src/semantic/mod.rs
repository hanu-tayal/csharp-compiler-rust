//! Semantic analysis for C#
//! 
//! Type checking, symbol resolution, and semantic validation.

use crate::syntax::{SyntaxTree, SyntaxNode, SyntaxKind};
use crate::diagnostics::{DiagnosticBag, Diagnostic, DiagnosticCode};
use crate::assembly_loader::{AssemblyLoader, Assembly};
use std::collections::HashMap;
use std::rc::Rc;
use indexmap::IndexMap;

pub mod symbols;
pub mod types;
pub mod binding;
pub mod flow_analysis;
pub mod type_checker;
pub mod overload_resolution;
pub mod generics;

use self::symbols::{Symbol, SymbolTable, SymbolKind, MethodSymbol};
use self::types::{TypeSymbol, TypeKind};
use self::binding::Binder;
use self::type_checker::TypeChecker;

/// The semantic model provides semantic information about the syntax tree
pub struct SemanticModel {
    /// The syntax tree being analyzed
    syntax_tree: SyntaxTree,
    /// Global symbol table
    symbol_table: SymbolTable,
    /// Type cache
    type_cache: HashMap<String, TypeSymbol>,
    /// Diagnostics
    diagnostics: DiagnosticBag,
    /// Compilation references
    references: Vec<AssemblyReference>,
    /// Assembly loader
    assembly_loader: AssemblyLoader,
}

/// A reference to an external assembly
pub struct AssemblyReference {
    pub name: String,
    pub symbols: SymbolTable,
}

impl SemanticModel {
    /// Create a new semantic model for a syntax tree
    pub fn new(syntax_tree: SyntaxTree) -> Self {
        let mut model = Self {
            syntax_tree,
            symbol_table: SymbolTable::new(),
            type_cache: HashMap::new(),
            diagnostics: DiagnosticBag::new(),
            references: Vec::new(),
            assembly_loader: AssemblyLoader::new(),
        };
        
        // Initialize built-in types
        model.initialize_builtin_types();
        
        // Import types from loaded assemblies
        model.import_assembly_types();
        
        model
    }
    
    /// Perform semantic analysis
    pub fn analyze(&mut self) {
        // Phase 1: Build symbol table (declarations)
        self.build_symbol_table();
        
        // Phase 2: Resolve types
        self.resolve_types();
        
        // Phase 3: Bind expressions and statements
        self.bind_syntax_tree();
        
        // Phase 4: Flow analysis
        self.perform_flow_analysis();
        
        // Phase 5: Additional semantic checks
        self.perform_semantic_checks();
    }
    
    /// Get the diagnostics
    pub fn diagnostics(&self) -> &DiagnosticBag {
        &self.diagnostics
    }
    
    /// Get symbol information for a node
    pub fn get_symbol(&self, node: &SyntaxNode) -> Option<&Symbol> {
        // Look up symbol by node
        // This would be implemented with a mapping from nodes to symbols
        None
    }
    
    /// Get type information for an expression
    pub fn get_type_info(&self, node: &SyntaxNode) -> Option<&TypeSymbol> {
        // Look up type by expression node
        None
    }
    
    // Phase 1: Build symbol table
    
    fn build_symbol_table(&mut self) {
        let mut builder = SymbolTableBuilder::new(&mut self.symbol_table, &mut self.diagnostics);
        builder.visit(&self.syntax_tree.root);
    }
    
    // Phase 2: Resolve types
    
    fn resolve_types(&mut self) {
        // Resolve base types, interfaces, type parameters, etc.
        let type_cache = &self.type_cache;
        let mut resolver = TypeResolver::new(&mut self.symbol_table, type_cache, &mut self.diagnostics);
        resolver.resolve_all();
    }
    
    // Phase 3: Bind expressions
    
    fn bind_syntax_tree(&mut self) {
        let mut binder = Binder::new(&self.symbol_table, &mut self.diagnostics);
        binder.bind_compilation_unit(&self.syntax_tree.root);
    }
    
    // Phase 4: Flow analysis
    
    fn perform_flow_analysis(&mut self) {
        let mut analyzer = flow_analysis::FlowAnalyzer::new(&self.symbol_table, &mut self.diagnostics);
        analyzer.analyze(&self.syntax_tree.root);
    }
    
    // Phase 5: Semantic checks
    
    fn perform_semantic_checks(&mut self) {
        // Additional checks like:
        // - Accessibility checks
        // - Obsolete member usage
        // - Async/await validation
        // - Pattern exhaustiveness
        // etc.
    }
    
    // Built-in types initialization
    
    fn initialize_builtin_types(&mut self) {
        // Add primitive types
        self.add_builtin_type("void", TypeKind::Void);
        self.add_builtin_type("bool", TypeKind::Boolean);
        self.add_builtin_type("byte", TypeKind::Byte);
        self.add_builtin_type("sbyte", TypeKind::SByte);
        self.add_builtin_type("short", TypeKind::Short);
        self.add_builtin_type("ushort", TypeKind::UShort);
        self.add_builtin_type("int", TypeKind::Int);
        self.add_builtin_type("uint", TypeKind::UInt);
        self.add_builtin_type("long", TypeKind::Long);
        self.add_builtin_type("ulong", TypeKind::ULong);
        self.add_builtin_type("float", TypeKind::Float);
        self.add_builtin_type("double", TypeKind::Double);
        self.add_builtin_type("decimal", TypeKind::Decimal);
        self.add_builtin_type("char", TypeKind::Char);
        self.add_builtin_type("string", TypeKind::String);
        self.add_builtin_type("object", TypeKind::Object);
        
        // Add type aliases
        self.add_type_alias("System.Void", "void");
        self.add_type_alias("System.Boolean", "bool");
        self.add_type_alias("System.Byte", "byte");
        self.add_type_alias("System.SByte", "sbyte");
        self.add_type_alias("System.Int16", "short");
        self.add_type_alias("System.UInt16", "ushort");
        self.add_type_alias("System.Int32", "int");
        self.add_type_alias("System.UInt32", "uint");
        self.add_type_alias("System.Int64", "long");
        self.add_type_alias("System.UInt64", "ulong");
        self.add_type_alias("System.Single", "float");
        self.add_type_alias("System.Double", "double");
        self.add_type_alias("System.Decimal", "decimal");
        self.add_type_alias("System.Char", "char");
        self.add_type_alias("System.String", "string");
        self.add_type_alias("System.Object", "object");
    }
    
    fn add_builtin_type(&mut self, name: &str, kind: TypeKind) {
        let type_symbol = TypeSymbol::new(name.to_string(), kind);
        self.type_cache.insert(name.to_string(), type_symbol);
    }
    
    fn add_type_alias(&mut self, full_name: &str, alias: &str) {
        if let Some(type_symbol) = self.type_cache.get(alias).cloned() {
            self.type_cache.insert(full_name.to_string(), type_symbol);
        }
    }
    
    /// Import types from loaded assemblies
    fn import_assembly_types(&mut self) {
        // Import types from mscorlib and System assemblies
        for assembly in self.assembly_loader.get_assemblies() {
            for type_info in &assembly.types {
                // Convert assembly type info to TypeSymbol
                let type_symbol = Rc::new(TypeSymbol::new(type_info.full_name.clone(), type_info.kind));
                // TODO: Resolve base types and interfaces
                
                self.type_cache.insert(type_info.full_name.clone(), (*type_symbol).clone());
                
                // Also add short name if in System namespace
                if type_info.namespace == "System" {
                    self.type_cache.insert(type_info.name.clone(), (*type_symbol).clone());
                }
            }
            
            // Import symbols from assembly symbol table
            // TODO: Merge assembly symbols into global symbol table
        }
    }
    
    /// Add assembly reference
    pub fn add_reference(&mut self, assembly_name: &str) -> Result<(), String> {
        // Load assembly using assembly loader
        self.assembly_loader.load_assembly(assembly_name)?;
        
        // Import newly loaded types
        self.import_assembly_types();
        
        Ok(())
    }
}

/// Builds the symbol table from declarations
struct SymbolTableBuilder<'a> {
    symbol_table: &'a mut SymbolTable,
    diagnostics: &'a mut DiagnosticBag,
    current_scope: Vec<String>,
}

impl<'a> SymbolTableBuilder<'a> {
    fn new(symbol_table: &'a mut SymbolTable, diagnostics: &'a mut DiagnosticBag) -> Self {
        Self {
            symbol_table,
            diagnostics,
            current_scope: Vec::new(),
        }
    }
    
    fn visit(&mut self, node: &SyntaxNode) {
        match node.kind {
            SyntaxKind::NamespaceDeclaration => self.visit_namespace(node),
            SyntaxKind::ClassDeclaration => self.visit_class(node),
            SyntaxKind::StructDeclaration => self.visit_struct(node),
            SyntaxKind::InterfaceDeclaration => self.visit_interface(node),
            SyntaxKind::EnumDeclaration => self.visit_enum(node),
            SyntaxKind::DelegateDeclaration => self.visit_delegate(node),
            SyntaxKind::MethodDeclaration => self.visit_method(node),
            SyntaxKind::PropertyDeclaration => self.visit_property(node),
            SyntaxKind::FieldDeclaration => self.visit_field(node),
            SyntaxKind::EventDeclaration => self.visit_event(node),
            _ => {
                // Visit children
                for child in &node.children {
                    if let crate::syntax::SyntaxElement::Node(child_node) = child {
                        self.visit(child_node);
                    }
                }
            }
        }
    }
    
    fn visit_namespace(&mut self, node: &SyntaxNode) {
        // Extract namespace name and enter scope
        let name = self.extract_name(node);
        self.current_scope.push(name.clone());
        
        // Add namespace symbol
        let symbol = Symbol::new(name, SymbolKind::Namespace);
        self.symbol_table.add_symbol(symbol);
        
        // Visit children
        for child in &node.children {
            if let crate::syntax::SyntaxElement::Node(child_node) = child {
                self.visit(child_node);
            }
        }
        
        // Exit scope
        self.current_scope.pop();
    }
    
    fn visit_class(&mut self, node: &SyntaxNode) {
        let name = self.extract_name(node);
        let full_name = self.get_full_name(&name);
        
        // Add class symbol
        let type_symbol = Rc::new(TypeSymbol::new(name.clone(), TypeKind::Class));
        let symbol = Symbol::new(full_name, SymbolKind::Type(type_symbol));
        self.symbol_table.add_symbol(symbol);
        
        // Enter type scope and visit members
        self.current_scope.push(name);
        for child in &node.children {
            if let crate::syntax::SyntaxElement::Node(child_node) = child {
                self.visit(child_node);
            }
        }
        self.current_scope.pop();
    }
    
    fn visit_struct(&mut self, node: &SyntaxNode) {
        // Similar to class
        self.visit_class(node);
    }
    
    fn visit_interface(&mut self, node: &SyntaxNode) {
        // Similar to class
        self.visit_class(node);
    }
    
    fn visit_enum(&mut self, node: &SyntaxNode) {
        let name = self.extract_name(node);
        let full_name = self.get_full_name(&name);
        
        // Add enum symbol
        let type_symbol = Rc::new(TypeSymbol::new(name.clone(), TypeKind::Enum));
        let symbol = Symbol::new(full_name.clone(), SymbolKind::Type(type_symbol));
        self.symbol_table.add_symbol(symbol);
        
        // Visit enum members
        self.current_scope.push(name);
        for child in &node.children {
            if let crate::syntax::SyntaxElement::Node(child_node) = child {
                if child_node.kind == SyntaxKind::EnumMemberDeclaration {
                    self.visit_enum_member(child_node);
                }
            }
        }
        self.current_scope.pop();
    }
    
    fn visit_delegate(&mut self, node: &SyntaxNode) {
        let name = self.extract_name(node);
        let full_name = self.get_full_name(&name);
        
        // Add delegate symbol
        let type_symbol = Rc::new(TypeSymbol::new(name.clone(), TypeKind::Delegate));
        let symbol = Symbol::new(full_name, SymbolKind::Type(type_symbol));
        self.symbol_table.add_symbol(symbol);
    }
    
    fn visit_method(&mut self, node: &SyntaxNode) {
        let name = self.extract_name(node);
        let full_name = self.get_full_name(&name);
        
        // Add method symbol
        let method_symbol = Rc::new(MethodSymbol {
            name: name.clone(),
            return_type: Rc::new(TypeSymbol::new("void".to_string(), TypeKind::Void)),
            parameters: Vec::new(),
            type_parameters: Vec::new(),
            is_static: false,
            is_virtual: false,
            is_abstract: false,
            is_override: false,
            is_async: false,
        });
        let symbol = Symbol::new(full_name, SymbolKind::Method(method_symbol));
        self.symbol_table.add_symbol(symbol);
    }
    
    fn visit_property(&mut self, node: &SyntaxNode) {
        let name = self.extract_name(node);
        let full_name = self.get_full_name(&name);
        
        // Add property symbol
        let symbol = Symbol::new(full_name, SymbolKind::Property);
        self.symbol_table.add_symbol(symbol);
    }
    
    fn visit_field(&mut self, node: &SyntaxNode) {
        let name = self.extract_name(node);
        let full_name = self.get_full_name(&name);
        
        // Add field symbol
        let field_type = Rc::new(TypeSymbol::new("object".to_string(), TypeKind::Object));
        let symbol = Symbol::new(full_name, SymbolKind::Field(field_type));
        self.symbol_table.add_symbol(symbol);
    }
    
    fn visit_event(&mut self, node: &SyntaxNode) {
        let name = self.extract_name(node);
        let full_name = self.get_full_name(&name);
        
        // Add event symbol
        let symbol = Symbol::new(full_name, SymbolKind::Event);
        self.symbol_table.add_symbol(symbol);
    }
    
    fn visit_enum_member(&mut self, node: &SyntaxNode) {
        let name = self.extract_name(node);
        let full_name = self.get_full_name(&name);
        
        // Add enum member symbol
        let field_type = Rc::new(TypeSymbol::new("int".to_string(), TypeKind::Int));
        let symbol = Symbol::new(full_name, SymbolKind::Field(field_type));
        self.symbol_table.add_symbol(symbol);
    }
    
    fn extract_name(&self, node: &SyntaxNode) -> String {
        // Extract name from declaration node
        // This is simplified - would need to properly traverse the AST
        "UnknownName".to_string()
    }
    
    fn get_full_name(&self, name: &str) -> String {
        if self.current_scope.is_empty() {
            name.to_string()
        } else {
            format!("{}.{}", self.current_scope.join("."), name)
        }
    }
}

/// Resolves types in the symbol table
struct TypeResolver<'a> {
    symbol_table: &'a mut SymbolTable,
    type_cache: &'a HashMap<String, TypeSymbol>,
    diagnostics: &'a mut DiagnosticBag,
}

impl<'a> TypeResolver<'a> {
    fn new(
        symbol_table: &'a mut SymbolTable,
        type_cache: &'a HashMap<String, TypeSymbol>,
        diagnostics: &'a mut DiagnosticBag,
    ) -> Self {
        Self {
            symbol_table,
            type_cache,
            diagnostics,
        }
    }
    
    fn resolve_all(&self) {
        // Resolve types for all symbols
        // This includes:
        // - Base types and interfaces
        // - Generic type parameters
        // - Member types (return types, parameter types, etc.)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;
    
    #[test]
    fn test_semantic_analysis() {
        let source = r#"
            namespace Test {
                public class Program {
                    public static void Main() {
                        int x = 42;
                        Console.WriteLine(x);
                    }
                }
            }
        "#;
        
        let mut parser = Parser::new(source);
        let syntax_tree = parser.parse();
        
        let mut model = SemanticModel::new(syntax_tree);
        model.analyze();
        
        assert!(!model.diagnostics().has_errors());
    }
}