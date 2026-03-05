//! Method overload resolution for C#

use super::types::{TypeSymbol, TypeKind};
use super::symbols::{MethodSymbol, ParameterSymbol};
use std::rc::Rc;

/// Result of overload resolution
#[derive(Debug)]
pub struct OverloadResolutionResult {
    /// The selected method
    pub method: Rc<MethodSymbol>,
    /// Whether type conversions are needed
    pub requires_conversions: Vec<TypeConversion>,
    /// Score for ranking (lower is better)
    pub score: i32,
}

/// Type conversion information
#[derive(Debug, Clone)]
pub struct TypeConversion {
    /// Parameter index
    pub parameter_index: usize,
    /// Source type
    pub from_type: Rc<TypeSymbol>,
    /// Target type
    pub to_type: Rc<TypeSymbol>,
    /// Conversion kind
    pub kind: ConversionKind,
}

/// Kind of type conversion
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConversionKind {
    /// No conversion needed
    Identity,
    /// Implicit numeric conversion
    ImplicitNumeric,
    /// Implicit reference conversion
    ImplicitReference,
    /// Boxing conversion
    Boxing,
    /// Unboxing conversion
    Unboxing,
    /// User-defined implicit conversion
    UserDefinedImplicit,
    /// Explicit conversion (not allowed in overload resolution)
    Explicit,
}

/// Overload resolver
pub struct OverloadResolver {
    /// Type compatibility checker
    type_checker: TypeCompatibility,
}

impl OverloadResolver {
    /// Create a new overload resolver
    pub fn new() -> Self {
        Self {
            type_checker: TypeCompatibility::new(),
        }
    }
    
    /// Resolve method overload
    pub fn resolve_method(
        &self,
        candidates: &[Rc<MethodSymbol>],
        argument_types: &[Rc<TypeSymbol>],
    ) -> Result<OverloadResolutionResult, String> {
        let mut viable_candidates = Vec::new();
        
        // Phase 1: Find applicable methods
        for method in candidates {
            if let Some(result) = self.is_applicable_method(method, argument_types) {
                viable_candidates.push(result);
            }
        }
        
        if viable_candidates.is_empty() {
            return Err("No applicable method found".to_string());
        }
        
        // Phase 2: Find best method
        if viable_candidates.len() == 1 {
            return Ok(viable_candidates.into_iter().next().unwrap());
        }
        
        // Sort by score (better conversions have lower scores)
        viable_candidates.sort_by_key(|r| r.score);
        
        // Check for ambiguity
        if viable_candidates.len() >= 2 {
            let best = &viable_candidates[0];
            let second = &viable_candidates[1];
            
            if best.score == second.score {
                return Err("Ambiguous method call".to_string());
            }
        }
        
        Ok(viable_candidates.into_iter().next().unwrap())
    }
    
    /// Check if method is applicable with given arguments
    fn is_applicable_method(
        &self,
        method: &Rc<MethodSymbol>,
        argument_types: &[Rc<TypeSymbol>],
    ) -> Option<OverloadResolutionResult> {
        // Check parameter count
        if method.parameters.len() != argument_types.len() {
            // TODO: Handle optional parameters and params arrays
            return None;
        }
        
        let mut conversions = Vec::new();
        let mut score = 0;
        
        // Check each parameter
        for (i, (param, arg_type)) in method.parameters.iter()
            .zip(argument_types.iter())
            .enumerate() 
        {
            match self.type_checker.get_conversion(arg_type, &param.param_type) {
                Some(conversion_kind) => {
                    if conversion_kind != ConversionKind::Identity {
                        conversions.push(TypeConversion {
                            parameter_index: i,
                            from_type: arg_type.clone(),
                            to_type: param.param_type.clone(),
                            kind: conversion_kind,
                        });
                    }
                    score += self.score_conversion(conversion_kind);
                }
                None => return None, // No valid conversion
            }
        }
        
        Some(OverloadResolutionResult {
            method: method.clone(),
            requires_conversions: conversions,
            score,
        })
    }
    
    /// Score a conversion (lower is better)
    fn score_conversion(&self, kind: ConversionKind) -> i32 {
        match kind {
            ConversionKind::Identity => 0,
            ConversionKind::ImplicitNumeric => 1,
            ConversionKind::ImplicitReference => 2,
            ConversionKind::Boxing => 3,
            ConversionKind::Unboxing => 4,
            ConversionKind::UserDefinedImplicit => 5,
            ConversionKind::Explicit => 100, // Should not occur
        }
    }
}

/// Type compatibility checker
struct TypeCompatibility {
    /// Numeric type hierarchy
    numeric_hierarchy: Vec<TypeKind>,
}

impl TypeCompatibility {
    fn new() -> Self {
        Self {
            numeric_hierarchy: vec![
                TypeKind::Byte,
                TypeKind::SByte,
                TypeKind::Short,
                TypeKind::UShort,
                TypeKind::Int,
                TypeKind::UInt,
                TypeKind::Long,
                TypeKind::ULong,
                TypeKind::Float,
                TypeKind::Double,
                TypeKind::Decimal,
            ],
        }
    }
    
    /// Get conversion from source to target type
    fn get_conversion(&self, from: &TypeSymbol, to: &TypeSymbol) -> Option<ConversionKind> {
        // Identity conversion
        if from.name == to.name {
            return Some(ConversionKind::Identity);
        }
        
        // Numeric conversions
        if let Some(conversion) = self.get_numeric_conversion(from, to) {
            return Some(conversion);
        }
        
        // Reference conversions
        if let Some(conversion) = self.get_reference_conversion(from, to) {
            return Some(conversion);
        }
        
        // Boxing/unboxing
        if let Some(conversion) = self.get_boxing_conversion(from, to) {
            return Some(conversion);
        }
        
        // No implicit conversion available
        None
    }
    
    /// Get numeric conversion
    fn get_numeric_conversion(&self, from: &TypeSymbol, to: &TypeSymbol) -> Option<ConversionKind> {
        let from_pos = self.numeric_hierarchy.iter().position(|k| k == &from.kind)?;
        let to_pos = self.numeric_hierarchy.iter().position(|k| k == &to.kind)?;
        
        // Only allow widening conversions
        if from_pos <= to_pos {
            Some(ConversionKind::ImplicitNumeric)
        } else {
            None
        }
    }
    
    /// Get reference conversion
    fn get_reference_conversion(&self, from: &TypeSymbol, to: &TypeSymbol) -> Option<ConversionKind> {
        // Check base type chain
        if self.is_base_type(from, to) {
            return Some(ConversionKind::ImplicitReference);
        }
        
        // Check interface implementation
        if to.kind == TypeKind::Interface && self.implements_interface(from, to) {
            return Some(ConversionKind::ImplicitReference);
        }
        
        // Array covariance
        if from.kind == TypeKind::Array && to.kind == TypeKind::Array {
            // TODO: Check element type compatibility
        }
        
        // Everything converts to object
        if to.kind == TypeKind::Object {
            return Some(ConversionKind::ImplicitReference);
        }
        
        None
    }
    
    /// Get boxing conversion
    fn get_boxing_conversion(&self, from: &TypeSymbol, to: &TypeSymbol) -> Option<ConversionKind> {
        // Value type to object or interface
        if self.is_value_type(from) && (to.kind == TypeKind::Object || to.kind == TypeKind::Interface) {
            return Some(ConversionKind::Boxing);
        }
        
        // Object or interface to value type
        if (from.kind == TypeKind::Object || from.kind == TypeKind::Interface) && self.is_value_type(to) {
            return Some(ConversionKind::Unboxing);
        }
        
        None
    }
    
    /// Check if type is a value type
    fn is_value_type(&self, type_symbol: &TypeSymbol) -> bool {
        matches!(
            type_symbol.kind,
            TypeKind::Struct |
            TypeKind::Enum |
            TypeKind::Boolean |
            TypeKind::Byte |
            TypeKind::SByte |
            TypeKind::Short |
            TypeKind::UShort |
            TypeKind::Int |
            TypeKind::UInt |
            TypeKind::Long |
            TypeKind::ULong |
            TypeKind::Float |
            TypeKind::Double |
            TypeKind::Decimal |
            TypeKind::Char
        )
    }
    
    /// Check if 'derived' is derived from 'base'
    fn is_base_type(&self, derived: &TypeSymbol, base: &TypeSymbol) -> bool {
        let mut current = derived.base_type.as_ref();
        
        while let Some(base_type) = current {
            if base_type.name == base.name {
                return true;
            }
            current = base_type.base_type.as_ref();
        }
        
        false
    }
    
    /// Check if type implements interface
    fn implements_interface(&self, type_symbol: &TypeSymbol, interface: &TypeSymbol) -> bool {
        type_symbol.interfaces.iter().any(|i| i.name == interface.name)
    }
}

/// Extension methods for better overloading
impl MethodSymbol {
    /// Get method signature for display
    pub fn signature(&self) -> String {
        let params = self.parameters.iter()
            .map(|p| format!("{} {}", p.param_type.name, p.name))
            .collect::<Vec<_>>()
            .join(", ");
        
        format!("{} {}({})", self.return_type.name, self.name, params)
    }
    
    /// Check if method is more specific than another
    pub fn is_more_specific_than(&self, other: &MethodSymbol) -> bool {
        if self.parameters.len() != other.parameters.len() {
            return false;
        }
        
        // A method is more specific if all its parameter types are
        // more derived than the corresponding parameter types of the other method
        for (self_param, other_param) in self.parameters.iter().zip(&other.parameters) {
            // TODO: Implement proper specificity check
            if self_param.param_type.name != other_param.param_type.name {
                return false;
            }
        }
        
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_numeric_conversions() {
        let type_compat = TypeCompatibility::new();
        
        let int_type = TypeSymbol {
            name: "int".to_string(),
            kind: TypeKind::Int,
            is_nullable: false,
            generic_parameters: Vec::new(),
            base_type: None,
            interfaces: Vec::new(),
            members: std::collections::HashMap::new(),
        };
        
        let long_type = TypeSymbol {
            name: "long".to_string(),
            kind: TypeKind::Long,
            is_nullable: false,
            generic_parameters: Vec::new(),
            base_type: None,
            interfaces: Vec::new(),
            members: std::collections::HashMap::new(),
        };
        
        // int to long should work
        let conversion = type_compat.get_conversion(&int_type, &long_type);
        assert!(matches!(conversion, Some(ConversionKind::ImplicitNumeric)));
        
        // long to int should not work implicitly
        let conversion = type_compat.get_conversion(&long_type, &int_type);
        assert!(conversion.is_none());
    }
}