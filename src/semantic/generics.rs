//! Generic type and method support for C#

use super::types::{TypeSymbol, TypeKind};
use super::symbols::{Symbol, SymbolKind};
use crate::syntax::{SyntaxNode, SyntaxKind};
use std::collections::HashMap;
use std::rc::Rc;

/// Generic type parameter
#[derive(Debug, Clone, PartialEq)]
pub struct TypeParameter {
    /// Name of the type parameter (e.g., "T")
    pub name: String,
    /// Position in the generic parameter list
    pub index: usize,
    /// Constraints on the type parameter
    pub constraints: Vec<TypeConstraint>,
    /// Variance modifier (for interfaces and delegates)
    pub variance: Variance,
}

/// Type parameter variance
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Variance {
    /// Invariant (default)
    Invariant,
    /// Covariant (out T)
    Covariant,
    /// Contravariant (in T)
    Contravariant,
}

/// Constraint on a type parameter
#[derive(Debug, Clone, PartialEq)]
pub enum TypeConstraint {
    /// Must be a reference type (class constraint)
    ReferenceType,
    /// Must be a value type (struct constraint)
    ValueType,
    /// Must have a parameterless constructor (new() constraint)
    Constructor,
    /// Must derive from or implement a type
    BaseType(Rc<TypeSymbol>),
    /// Must implement an interface
    Interface(Rc<TypeSymbol>),
}

/// Generic type instantiation
#[derive(Debug, Clone)]
pub struct GenericInstantiation {
    /// The generic type definition
    pub definition: Rc<TypeSymbol>,
    /// Type arguments for the instantiation
    pub type_arguments: Vec<Rc<TypeSymbol>>,
}

/// Generic method signature
#[derive(Debug, Clone)]
pub struct GenericMethodSignature {
    /// Method name
    pub name: String,
    /// Type parameters
    pub type_parameters: Vec<TypeParameter>,
    /// Return type (may reference type parameters)
    pub return_type: TypeReference,
    /// Parameter types (may reference type parameters)
    pub parameters: Vec<(String, TypeReference)>,
}

/// Reference to a type that may be generic
#[derive(Debug, Clone)]
pub enum TypeReference {
    /// Concrete type
    Concrete(Rc<TypeSymbol>),
    /// Type parameter reference
    TypeParameter(String),
    /// Generic type with arguments
    Generic(Box<TypeReference>, Vec<TypeReference>),
    /// Array type
    Array(Box<TypeReference>),
    /// Nullable type
    Nullable(Box<TypeReference>),
}

/// Generic type resolver
pub struct GenericResolver {
    /// Type parameter bindings
    type_bindings: HashMap<String, Rc<TypeSymbol>>,
    /// Constraint checker
    constraint_checker: ConstraintChecker,
}

impl GenericResolver {
    /// Create a new generic resolver
    pub fn new() -> Self {
        Self {
            type_bindings: HashMap::new(),
            constraint_checker: ConstraintChecker::new(),
        }
    }
    
    /// Instantiate a generic type with type arguments
    pub fn instantiate_type(
        &mut self,
        generic_type: &TypeSymbol,
        type_arguments: Vec<Rc<TypeSymbol>>,
    ) -> Result<Rc<TypeSymbol>, String> {
        // Check argument count
        if generic_type.generic_parameters.len() != type_arguments.len() {
            return Err(format!(
                "Wrong number of type arguments for '{}': expected {}, got {}",
                generic_type.name,
                generic_type.generic_parameters.len(),
                type_arguments.len()
            ));
        }
        
        // Create type parameter bindings
        self.type_bindings.clear();
        for (param, arg) in generic_type.generic_parameters.iter().zip(&type_arguments) {
            self.type_bindings.insert(param.clone(), arg.clone());
        }
        
        // Check constraints
        for (i, param) in generic_type.generic_parameters.iter().enumerate() {
            if let Some(type_param) = self.get_type_parameter(generic_type, param) {
                let arg = &type_arguments[i];
                self.constraint_checker.check_constraints(&type_param, arg)?;
            }
        }
        
        // Create instantiated type
        let instantiated = self.create_instantiated_type(generic_type, type_arguments);
        Ok(instantiated)
    }
    
    /// Resolve a type reference with current bindings
    pub fn resolve_type_reference(&self, type_ref: &TypeReference) -> Result<Rc<TypeSymbol>, String> {
        match type_ref {
            TypeReference::Concrete(typ) => Ok(typ.clone()),
            
            TypeReference::TypeParameter(name) => {
                self.type_bindings.get(name)
                    .cloned()
                    .ok_or_else(|| format!("Unbound type parameter '{}'", name))
            }
            
            TypeReference::Generic(base_ref, arg_refs) => {
                let base_type = self.resolve_type_reference(base_ref)?;
                let arg_types = arg_refs.iter()
                    .map(|arg| self.resolve_type_reference(arg))
                    .collect::<Result<Vec<_>, _>>()?;
                
                // Create generic instantiation
                self.create_generic_instantiation(&base_type, arg_types)
            }
            
            TypeReference::Array(element_ref) => {
                let element_type = self.resolve_type_reference(element_ref)?;
                Ok(self.create_array_type(element_type))
            }
            
            TypeReference::Nullable(value_ref) => {
                let value_type = self.resolve_type_reference(value_ref)?;
                Ok(self.create_nullable_type(value_type))
            }
        }
    }
    
    /// Get type parameter information
    fn get_type_parameter(&self, generic_type: &TypeSymbol, param_name: &str) -> Option<TypeParameter> {
        // This would look up the actual type parameter definition
        // For now, return a simple parameter
        Some(TypeParameter {
            name: param_name.to_string(),
            index: 0,
            constraints: Vec::new(),
            variance: Variance::Invariant,
        })
    }
    
    /// Create an instantiated generic type
    fn create_instantiated_type(
        &self,
        definition: &TypeSymbol,
        type_arguments: Vec<Rc<TypeSymbol>>,
    ) -> Rc<TypeSymbol> {
        // Create a new type symbol representing the instantiation
        // In a real implementation, this would create a specialized version
        let name = format!(
            "{}<{}>",
            definition.name,
            type_arguments.iter()
                .map(|t| t.name.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
        
        let mut type_symbol = TypeSymbol::new(name, definition.kind);
        type_symbol.is_nullable = definition.is_nullable;
        type_symbol.base_type = definition.base_type.clone();
        type_symbol.interfaces = definition.interfaces.clone();
        type_symbol.members = definition.members.clone(); // TODO: Substitute types in members
        Rc::new(type_symbol)
    }
    
    /// Create a generic instantiation
    fn create_generic_instantiation(
        &self,
        base_type: &TypeSymbol,
        type_arguments: Vec<Rc<TypeSymbol>>,
    ) -> Result<Rc<TypeSymbol>, String> {
        // This is a simplified version
        Ok(self.create_instantiated_type(base_type, type_arguments))
    }
    
    /// Create an array type
    fn create_array_type(&self, element_type: Rc<TypeSymbol>) -> Rc<TypeSymbol> {
        let mut array_type = TypeSymbol::new(format!("{}[]", element_type.name), TypeKind::Array);
        array_type.base_type = Some(Rc::new(TypeSymbol::new("System.Array".to_string(), TypeKind::Class)));
        Rc::new(array_type)
    }
    
    /// Create a nullable type
    fn create_nullable_type(&self, value_type: Rc<TypeSymbol>) -> Rc<TypeSymbol> {
        let mut nullable_type = TypeSymbol::new(format!("{}?", value_type.name), TypeKind::Struct);
        nullable_type.is_nullable = true;
        let mut nullable_base = TypeSymbol::new("System.Nullable`1".to_string(), TypeKind::Struct);
        nullable_base.generic_parameters = vec!["T".to_string()];
        nullable_type.base_type = Some(Rc::new(nullable_base));
        Rc::new(nullable_type)
    }
}

/// Constraint checker for generic type parameters
struct ConstraintChecker {
    // Type compatibility checker would be injected here
}

impl ConstraintChecker {
    fn new() -> Self {
        Self {}
    }
    
    /// Check if a type satisfies constraints
    fn check_constraints(&self, param: &TypeParameter, arg_type: &TypeSymbol) -> Result<(), String> {
        for constraint in &param.constraints {
            match constraint {
                TypeConstraint::ReferenceType => {
                    if !self.is_reference_type(arg_type) {
                        return Err(format!(
                            "Type '{}' must be a reference type",
                            arg_type.name
                        ));
                    }
                }
                
                TypeConstraint::ValueType => {
                    if !self.is_value_type(arg_type) {
                        return Err(format!(
                            "Type '{}' must be a value type",
                            arg_type.name
                        ));
                    }
                }
                
                TypeConstraint::Constructor => {
                    if !self.has_default_constructor(arg_type) {
                        return Err(format!(
                            "Type '{}' must have a public parameterless constructor",
                            arg_type.name
                        ));
                    }
                }
                
                TypeConstraint::BaseType(base) => {
                    if !self.is_derived_from(arg_type, base) {
                        return Err(format!(
                            "Type '{}' must derive from '{}'",
                            arg_type.name,
                            base.name
                        ));
                    }
                }
                
                TypeConstraint::Interface(iface) => {
                    if !self.implements_interface(arg_type, iface) {
                        return Err(format!(
                            "Type '{}' must implement '{}'",
                            arg_type.name,
                            iface.name
                        ));
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn is_reference_type(&self, typ: &TypeSymbol) -> bool {
        matches!(
            typ.kind,
            TypeKind::Class | TypeKind::Interface | TypeKind::Delegate | TypeKind::String
        )
    }
    
    fn is_value_type(&self, typ: &TypeSymbol) -> bool {
        matches!(
            typ.kind,
            TypeKind::Struct | TypeKind::Enum |
            TypeKind::Boolean | TypeKind::Byte | TypeKind::SByte |
            TypeKind::Short | TypeKind::UShort | TypeKind::Int | TypeKind::UInt |
            TypeKind::Long | TypeKind::ULong | TypeKind::Float | TypeKind::Double |
            TypeKind::Decimal | TypeKind::Char
        )
    }
    
    fn has_default_constructor(&self, typ: &TypeSymbol) -> bool {
        // Simplified: value types always have default constructor
        if self.is_value_type(typ) {
            return true;
        }
        
        // Check for parameterless constructor in members
        // This is simplified - would need to check actual constructors
        true
    }
    
    fn is_derived_from(&self, derived: &TypeSymbol, base: &TypeSymbol) -> bool {
        let mut current = derived.base_type.as_ref();
        
        while let Some(base_type) = current {
            if base_type.name == base.name {
                return true;
            }
            current = base_type.base_type.as_ref();
        }
        
        false
    }
    
    fn implements_interface(&self, typ: &TypeSymbol, iface: &TypeSymbol) -> bool {
        typ.interfaces.iter().any(|i| i.name == iface.name)
    }
}

/// Generic method resolver
pub struct GenericMethodResolver {
    /// Type resolver
    type_resolver: GenericResolver,
}

impl GenericMethodResolver {
    pub fn new() -> Self {
        Self {
            type_resolver: GenericResolver::new(),
        }
    }
    
    /// Infer type arguments for a generic method call
    pub fn infer_type_arguments(
        &mut self,
        method: &GenericMethodSignature,
        argument_types: &[Rc<TypeSymbol>],
    ) -> Result<Vec<Rc<TypeSymbol>>, String> {
        // Simple type inference algorithm
        let mut inferred: HashMap<String, Rc<TypeSymbol>> = HashMap::new();
        
        // Try to infer from parameter types
        for (i, (_, param_ref)) in method.parameters.iter().enumerate() {
            if i >= argument_types.len() {
                break;
            }
            
            self.infer_from_type_reference(param_ref, &argument_types[i], &mut inferred)?;
        }
        
        // Check that all type parameters were inferred
        let mut type_arguments = Vec::new();
        for param in &method.type_parameters {
            if let Some(arg) = inferred.get(&param.name) {
                type_arguments.push(arg.clone());
            } else {
                return Err(format!("Could not infer type argument for '{}'", param.name));
            }
        }
        
        Ok(type_arguments)
    }
    
    /// Infer type bindings from a type reference and actual type
    fn infer_from_type_reference(
        &self,
        type_ref: &TypeReference,
        actual_type: &TypeSymbol,
        inferred: &mut HashMap<String, Rc<TypeSymbol>>,
    ) -> Result<(), String> {
        match type_ref {
            TypeReference::TypeParameter(name) => {
                // Direct type parameter binding
                inferred.insert(name.clone(), Rc::new(actual_type.clone()));
                Ok(())
            }
            
            TypeReference::Generic(base_ref, arg_refs) => {
                // For generic types, we need to match the structure
                // This is simplified - real implementation would be more complex
                Ok(())
            }
            
            TypeReference::Array(element_ref) => {
                // If actual type is array, infer from element type
                if actual_type.kind == TypeKind::Array {
                    // Would need to extract element type from actual_type
                    Ok(())
                } else {
                    Err("Type mismatch: expected array".to_string())
                }
            }
            
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generic_instantiation() {
        let mut resolver = GenericResolver::new();
        
        // Create a generic List<T> type
        let list_type = TypeSymbol {
            name: "List".to_string(),
            kind: TypeKind::Class,
            is_nullable: false,
            generic_parameters: vec!["T".to_string()],
            base_type: None,
            interfaces: Vec::new(),
            members: HashMap::new(),
        };
        
        // Create int type
        let int_type = Rc::new(TypeSymbol {
            name: "int".to_string(),
            kind: TypeKind::Int,
            is_nullable: false,
            generic_parameters: Vec::new(),
            base_type: None,
            interfaces: Vec::new(),
            members: HashMap::new(),
        });
        
        // Instantiate List<int>
        let result = resolver.instantiate_type(&list_type, vec![int_type]);
        assert!(result.is_ok());
        
        let list_int = result.unwrap();
        assert_eq!(list_int.name, "List<int>");
    }
    
    #[test]
    fn test_type_constraints() {
        let checker = ConstraintChecker::new();
        
        let class_type = TypeSymbol {
            name: "MyClass".to_string(),
            kind: TypeKind::Class,
            is_nullable: false,
            generic_parameters: Vec::new(),
            base_type: None,
            interfaces: Vec::new(),
            members: HashMap::new(),
        };
        
        let struct_type = TypeSymbol {
            name: "MyStruct".to_string(),
            kind: TypeKind::Struct,
            is_nullable: false,
            generic_parameters: Vec::new(),
            base_type: None,
            interfaces: Vec::new(),
            members: HashMap::new(),
        };
        
        assert!(checker.is_reference_type(&class_type));
        assert!(!checker.is_reference_type(&struct_type));
        assert!(!checker.is_value_type(&class_type));
        assert!(checker.is_value_type(&struct_type));
    }
}