//! Symbol representation and symbol table

use std::collections::HashMap;
use indexmap::IndexMap;

/// A symbol in the program (type, method, variable, etc.)
#[derive(Debug, Clone)]
pub struct Symbol {
    /// Fully qualified name
    pub name: String,
    /// Kind of symbol
    pub kind: SymbolKind,
    /// Is public
    pub is_public: bool,
    /// Is static
    pub is_static: bool,
    /// Is readonly
    pub is_readonly: bool,
    /// Scope ID
    pub scope_id: usize,
}

use std::rc::Rc;

/// Kind of symbol
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    /// Namespace
    Namespace,
    /// Type (class, struct, interface, enum, delegate)
    Type(Rc<super::types::TypeSymbol>),
    /// Method
    Method(Rc<MethodSymbol>),
    /// Property
    Property,
    /// Field
    Field(Rc<super::types::TypeSymbol>),
    /// Event
    Event,
    /// Parameter
    Parameter(Rc<super::types::TypeSymbol>),
    /// Local variable
    Variable(Rc<super::types::TypeSymbol>),
    /// Type parameter
    TypeParameter,
}

/// Method symbol information
#[derive(Debug, Clone, PartialEq)]
pub struct MethodSymbol {
    pub name: String,
    pub return_type: Rc<super::types::TypeSymbol>,
    pub parameters: Vec<ParameterSymbol>,
    pub type_parameters: Vec<String>,
    pub is_static: bool,
    pub is_virtual: bool,
    pub is_abstract: bool,
    pub is_override: bool,
    pub is_async: bool,
}

/// Parameter symbol information
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterSymbol {
    pub name: String,
    pub param_type: Rc<super::types::TypeSymbol>,
    pub is_ref: bool,
    pub is_out: bool,
    pub is_params: bool,
    pub has_default: bool,
    pub default_value: Option<String>,
}

impl Symbol {
    /// Create a new symbol
    pub fn new(name: String, kind: SymbolKind) -> Self {
        Self {
            name,
            kind,
            is_public: false,
            is_static: false,
            is_readonly: false,
            scope_id: 0,
        }
    }
}

/// Symbol table for storing and looking up symbols
#[derive(Debug)]
pub struct SymbolTable {
    /// Symbols by ID
    symbols: HashMap<usize, Symbol>,
    /// Scopes (scope ID -> list of symbol IDs)
    scopes: HashMap<usize, Vec<usize>>,
    /// Current scope ID
    current_scope: usize,
    /// Next symbol ID
    next_id: usize,
}


impl SymbolTable {
    /// Create a new symbol table
    pub fn new() -> Self {
        let mut table = Self {
            symbols: HashMap::new(),
            scopes: HashMap::new(),
            current_scope: 0,
            next_id: 1,
        };
        // Create global scope
        table.scopes.insert(0, Vec::new());
        table
    }
    
    /// Add a symbol to the table
    pub fn add_symbol(&mut self, symbol: Symbol) {
        let id = self.next_id;
        self.next_id += 1;
        
        // Add to current scope
        if let Some(scope_symbols) = self.scopes.get_mut(&self.current_scope) {
            scope_symbols.push(id);
        }
        
        // Add to symbol map
        self.symbols.insert(id, symbol);
    }
    
    /// Look up a symbol by name
    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        // Search from current scope up to global scope
        let mut scope_id = self.current_scope;
        
        loop {
            // Check symbols in current scope
            if let Some(symbol_ids) = self.scopes.get(&scope_id) {
                for &symbol_id in symbol_ids {
                    if let Some(symbol) = self.symbols.get(&symbol_id) {
                        if symbol.name == name {
                            return Some(symbol);
                        }
                    }
                }
            }
            
            // Move to parent scope
            if scope_id == 0 {
                break; // Global scope reached
            }
            
            // For now, just go to global
            scope_id = 0;
        }
        
        None
    }
    
    /// Enter a new scope
    pub fn enter_scope(&mut self) -> usize {
        let new_scope = self.next_id;
        self.next_id += 1;
        self.scopes.insert(new_scope, Vec::new());
        let old_scope = self.current_scope;
        self.current_scope = new_scope;
        old_scope
    }
    
    /// Exit current scope
    pub fn exit_scope(&mut self, previous_scope: usize) {
        self.current_scope = previous_scope;
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_symbol_table() {
        let mut table = SymbolTable::new();
        
        // Add namespace
        let ns = Symbol::new("System".to_string(), SymbolKind::Namespace);
        table.add_symbol(ns);
        
        // Add type
        let class = Symbol::new("System.String".to_string(), SymbolKind::Type)
            .with_accessibility(Accessibility::Public);
        table.add_symbol(class);
        
        // Add method
        let method = Symbol::new("System.String.Length".to_string(), SymbolKind::Property)
            .with_parent("System.String".to_string())
            .with_accessibility(Accessibility::Public);
        table.add_symbol(method);
        
        // Lookup tests
        assert!(table.lookup("System").is_some());
        assert!(table.lookup("System.String").is_some());
        assert!(table.lookup("System.String.Length").is_some());
        
        let members = table.lookup_type_members("System.String");
        assert_eq!(members.len(), 1);
    }
}