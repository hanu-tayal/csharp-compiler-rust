//! Type system implementation for C#

use std::collections::HashMap;
use std::rc::Rc;

/// Represents a type in the C# type system
#[derive(Debug, Clone, PartialEq)]
pub struct TypeSymbol {
    /// Name of the type
    pub name: String,
    /// Kind of type
    pub kind: TypeKind,
    /// Base type (for classes and structs)
    pub base_type: Option<Rc<TypeSymbol>>,
    /// Implemented interfaces
    pub interfaces: Vec<Rc<TypeSymbol>>,
    /// Generic parameters
    pub type_parameters: Vec<TypeParameter>,
    /// Members (methods, properties, fields, etc.)
    pub members: HashMap<String, TypeMember>,
    /// Containing namespace
    pub namespace: Option<String>,
    /// Accessibility
    pub accessibility: TypeAccessibility,
    /// Type modifiers
    pub modifiers: TypeModifiers,
    /// Whether this type is nullable (for reference types or Nullable<T>)
    pub is_nullable: bool,
    /// Generic parameters (kept for compatibility)
    pub generic_parameters: Vec<String>,
}

/// Kind of type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeKind {
    // Primitive types
    Void,
    Boolean,
    Byte,
    SByte,
    Short,
    UShort,
    Int,
    UInt,
    Long,
    ULong,
    Float,
    Double,
    Decimal,
    Char,
    String,
    Object,
    
    // User-defined types
    Class,
    Struct,
    Interface,
    Enum,
    Delegate,
    
    // Special types
    Array,
    Pointer,
    Nullable,
    Tuple,
    Dynamic,
    Anonymous,
    TypeParameter,
    Error,
}

/// Type parameter (generic)
#[derive(Debug, Clone, PartialEq)]
pub struct TypeParameter {
    pub name: String,
    pub constraints: Vec<TypeConstraint>,
    pub variance: Variance,
}

/// Type constraint
#[derive(Debug, Clone, PartialEq)]
pub enum TypeConstraint {
    /// Must be a reference type
    ReferenceType,
    /// Must be a value type
    ValueType,
    /// Must have a parameterless constructor
    Constructor,
    /// Must inherit from or implement a type
    BaseType(Rc<TypeSymbol>),
}

/// Variance for generic type parameters
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variance {
    Invariant,
    Covariant,
    Contravariant,
}

/// Type member
#[derive(Debug, Clone, PartialEq)]
pub struct TypeMember {
    pub name: String,
    pub kind: MemberKind,
    pub type_info: MemberTypeInfo,
    pub accessibility: MemberAccessibility,
    pub modifiers: MemberModifiers,
}

/// Kind of member
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemberKind {
    Method,
    Property,
    Field,
    Event,
    Constructor,
    Destructor,
    Operator,
    Indexer,
}

/// Type information for a member
#[derive(Debug, Clone, PartialEq)]
pub enum MemberTypeInfo {
    /// For methods and constructors
    Method {
        return_type: Rc<TypeSymbol>,
        parameters: Vec<ParameterInfo>,
        type_parameters: Vec<TypeParameter>,
    },
    /// For properties
    Property {
        property_type: Rc<TypeSymbol>,
        has_getter: bool,
        has_setter: bool,
    },
    /// For fields
    Field {
        field_type: Rc<TypeSymbol>,
    },
    /// For events
    Event {
        event_type: Rc<TypeSymbol>,
    },
}

/// Parameter information
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterInfo {
    pub name: String,
    pub parameter_type: Rc<TypeSymbol>,
    pub modifier: ParameterModifier,
    pub default_value: Option<String>,
}

/// Parameter modifier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterModifier {
    None,
    Ref,
    Out,
    In,
    Params,
}

/// Type accessibility
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeAccessibility {
    Public,
    Private,
    Protected,
    Internal,
    ProtectedInternal,
    PrivateProtected,
}

/// Type modifiers
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TypeModifiers {
    pub is_abstract: bool,
    pub is_sealed: bool,
    pub is_static: bool,
    pub is_partial: bool,
    pub is_nested: bool,
    pub is_generic: bool,
}

/// Member accessibility
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemberAccessibility {
    Public,
    Private,
    Protected,
    Internal,
    ProtectedInternal,
    PrivateProtected,
}

/// Member modifiers
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MemberModifiers {
    pub is_static: bool,
    pub is_abstract: bool,
    pub is_virtual: bool,
    pub is_override: bool,
    pub is_sealed: bool,
    pub is_new: bool,
    pub is_readonly: bool,
    pub is_const: bool,
    pub is_extern: bool,
    pub is_async: bool,
    pub is_partial: bool,
}

impl TypeSymbol {
    /// Create a new type symbol
    pub fn new(name: String, kind: TypeKind) -> Self {
        Self {
            name,
            kind,
            base_type: None,
            interfaces: Vec::new(),
            type_parameters: Vec::new(),
            members: HashMap::new(),
            namespace: None,
            accessibility: TypeAccessibility::Private,
            modifiers: TypeModifiers::default(),
            is_nullable: false,
            generic_parameters: Vec::new(),
        }
    }
    
    /// Check if this type is a reference type
    pub fn is_reference_type(&self) -> bool {
        match self.kind {
            TypeKind::Class | TypeKind::Interface | TypeKind::Delegate |
            TypeKind::String | TypeKind::Object | TypeKind::Array |
            TypeKind::Dynamic => true,
            TypeKind::Struct | TypeKind::Enum => false,
            TypeKind::Nullable => {
                // Nullable<T> where T is value type is still a value type
                false
            }
            _ => false,
        }
    }
    
    /// Check if this type is a value type
    pub fn is_value_type(&self) -> bool {
        match self.kind {
            TypeKind::Struct | TypeKind::Enum | TypeKind::Boolean |
            TypeKind::Byte | TypeKind::SByte | TypeKind::Short |
            TypeKind::UShort | TypeKind::Int | TypeKind::UInt |
            TypeKind::Long | TypeKind::ULong | TypeKind::Float |
            TypeKind::Double | TypeKind::Decimal | TypeKind::Char => true,
            _ => false,
        }
    }
    
    /// Check if this type is a numeric type
    pub fn is_numeric_type(&self) -> bool {
        matches!(self.kind,
            TypeKind::Byte | TypeKind::SByte | TypeKind::Short |
            TypeKind::UShort | TypeKind::Int | TypeKind::UInt |
            TypeKind::Long | TypeKind::ULong | TypeKind::Float |
            TypeKind::Double | TypeKind::Decimal
        )
    }
    
    /// Check if this type is assignable from another type
    pub fn is_assignable_from(&self, other: &TypeSymbol) -> bool {
        // Same type
        if self.name == other.name && self.namespace == other.namespace {
            return true;
        }
        
        // Check inheritance hierarchy
        if let Some(base_type) = &other.base_type {
            if self.is_assignable_from(base_type) {
                return true;
            }
        }
        
        // Check interfaces
        for interface in &other.interfaces {
            if self.is_assignable_from(interface) {
                return true;
            }
        }
        
        // Special cases
        match (self.kind, other.kind) {
            // object is assignable from any reference type
            (TypeKind::Object, _) if other.is_reference_type() => true,
            // Numeric conversions
            (TypeKind::Double, kind) if matches!(kind, 
                TypeKind::Float | TypeKind::Long | TypeKind::ULong |
                TypeKind::Int | TypeKind::UInt | TypeKind::Short |
                TypeKind::UShort | TypeKind::Byte | TypeKind::SByte) => true,
            (TypeKind::Float, kind) if matches!(kind,
                TypeKind::Long | TypeKind::ULong | TypeKind::Int |
                TypeKind::UInt | TypeKind::Short | TypeKind::UShort |
                TypeKind::Byte | TypeKind::SByte) => true,
            // Add more conversion rules...
            _ => false,
        }
    }
    
    /// Get the default value for this type
    pub fn default_value(&self) -> &'static str {
        match self.kind {
            TypeKind::Boolean => "false",
            TypeKind::Byte | TypeKind::SByte | TypeKind::Short |
            TypeKind::UShort | TypeKind::Int | TypeKind::UInt |
            TypeKind::Long | TypeKind::ULong => "0",
            TypeKind::Float => "0.0f",
            TypeKind::Double => "0.0",
            TypeKind::Decimal => "0m",
            TypeKind::Char => "'\\0'",
            TypeKind::String | TypeKind::Object | TypeKind::Class |
            TypeKind::Interface | TypeKind::Delegate | TypeKind::Array => "null",
            _ => "default",
        }
    }
}

/// Type builder for constructing complex types
#[derive(Debug)]
pub struct TypeBuilder {
    symbol: TypeSymbol,
}

impl TypeBuilder {
    /// Create a new type builder
    pub fn new(name: String, kind: TypeKind) -> Self {
        Self {
            symbol: TypeSymbol::new(name, kind),
        }
    }
    
    /// Set the namespace
    pub fn with_namespace(mut self, namespace: String) -> Self {
        self.symbol.namespace = Some(namespace);
        self
    }
    
    /// Set the base type
    pub fn with_base_type(mut self, base_type: Rc<TypeSymbol>) -> Self {
        self.symbol.base_type = Some(base_type);
        self
    }
    
    /// Add an interface
    pub fn with_interface(mut self, interface: Rc<TypeSymbol>) -> Self {
        self.symbol.interfaces.push(interface);
        self
    }
    
    /// Add a type parameter
    pub fn with_type_parameter(mut self, param: TypeParameter) -> Self {
        self.symbol.type_parameters.push(param);
        self.symbol.modifiers.is_generic = true;
        self
    }
    
    /// Set accessibility
    pub fn with_accessibility(mut self, accessibility: TypeAccessibility) -> Self {
        self.symbol.accessibility = accessibility;
        self
    }
    
    /// Set abstract
    pub fn as_abstract(mut self) -> Self {
        self.symbol.modifiers.is_abstract = true;
        self
    }
    
    /// Set sealed
    pub fn as_sealed(mut self) -> Self {
        self.symbol.modifiers.is_sealed = true;
        self
    }
    
    /// Set static
    pub fn as_static(mut self) -> Self {
        self.symbol.modifiers.is_static = true;
        self
    }
    
    /// Build the type symbol
    pub fn build(self) -> TypeSymbol {
        self.symbol
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_type_properties() {
        let int_type = TypeSymbol::new("int".to_string(), TypeKind::Int);
        assert!(int_type.is_value_type());
        assert!(int_type.is_numeric_type());
        assert!(!int_type.is_reference_type());
        
        let string_type = TypeSymbol::new("string".to_string(), TypeKind::String);
        assert!(string_type.is_reference_type());
        assert!(!string_type.is_value_type());
        assert!(!string_type.is_numeric_type());
    }
    
    #[test]
    fn test_type_builder() {
        let object_type = Rc::new(TypeSymbol::new("object".to_string(), TypeKind::Object));
        let disposable = Rc::new(TypeSymbol::new("IDisposable".to_string(), TypeKind::Interface));
        
        let my_class = TypeBuilder::new("MyClass".to_string(), TypeKind::Class)
            .with_namespace("MyNamespace".to_string())
            .with_base_type(object_type)
            .with_interface(disposable)
            .with_accessibility(TypeAccessibility::Public)
            .as_sealed()
            .build();
        
        assert_eq!(my_class.name, "MyClass");
        assert_eq!(my_class.namespace, Some("MyNamespace".to_string()));
        assert!(my_class.modifiers.is_sealed);
        assert_eq!(my_class.interfaces.len(), 1);
    }
}
