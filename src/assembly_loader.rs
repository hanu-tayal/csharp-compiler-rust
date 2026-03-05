//! Assembly reference loading and metadata reading

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::collections::HashMap;
use crate::semantic::symbols::{Symbol, SymbolKind, SymbolTable, MethodSymbol, ParameterSymbol};
use crate::semantic::types::{TypeSymbol, TypeKind};
use std::rc::Rc;

/// Assembly loader for loading .NET assemblies
pub struct AssemblyLoader {
    /// Loaded assemblies
    assemblies: HashMap<String, Assembly>,
    /// Search paths
    search_paths: Vec<PathBuf>,
}

/// Loaded assembly information
#[derive(Debug)]
pub struct Assembly {
    /// Assembly name
    pub name: String,
    /// Version
    pub version: String,
    /// Exported types
    pub types: Vec<TypeInfo>,
    /// Symbol table
    pub symbols: SymbolTable,
}

/// Type information from assembly
#[derive(Debug, Clone)]
pub struct TypeInfo {
    /// Full type name
    pub full_name: String,
    /// Namespace
    pub namespace: String,
    /// Type name
    pub name: String,
    /// Type kind
    pub kind: TypeKind,
    /// Is public
    pub is_public: bool,
    /// Base type
    pub base_type: Option<String>,
    /// Implemented interfaces
    pub interfaces: Vec<String>,
    /// Members
    pub members: Vec<MemberInfo>,
}

/// Member information
#[derive(Debug, Clone)]
pub struct MemberInfo {
    /// Member name
    pub name: String,
    /// Member kind
    pub kind: MemberKind,
    /// Is public
    pub is_public: bool,
    /// Is static
    pub is_static: bool,
    /// Type signature
    pub signature: String,
}

/// Member kind
#[derive(Debug, Clone, Copy)]
pub enum MemberKind {
    Method,
    Property,
    Field,
    Event,
    Constructor,
}

impl AssemblyLoader {
    /// Create a new assembly loader
    pub fn new() -> Self {
        let mut loader = Self {
            assemblies: HashMap::new(),
            search_paths: Vec::new(),
        };
        
        // Add default search paths
        loader.add_default_search_paths();
        
        // Load core assemblies
        loader.load_core_assemblies();
        
        loader
    }
    
    /// Add default search paths
    fn add_default_search_paths(&mut self) {
        // .NET Framework paths
        if cfg!(windows) {
            // Try different .NET Framework versions
            let framework_paths = vec![
                r"C:\Windows\Microsoft.NET\Framework64\v4.0.30319",
                r"C:\Windows\Microsoft.NET\Framework\v4.0.30319",
                r"C:\Program Files\dotnet\shared\Microsoft.NETCore.App",
                r"C:\Program Files (x86)\Reference Assemblies\Microsoft\Framework\.NETFramework",
            ];
            
            for path in framework_paths {
                let path = PathBuf::from(path);
                if path.exists() {
                    self.search_paths.push(path);
                }
            }
        }
        
        // Current directory
        if let Ok(current_dir) = std::env::current_dir() {
            self.search_paths.push(current_dir);
        }
    }
    
    /// Load core assemblies
    fn load_core_assemblies(&mut self) {
        // For now, we'll create synthetic definitions for core types
        self.create_mscorlib_assembly();
        self.create_system_assembly();
    }
    
    /// Create synthetic mscorlib assembly
    fn create_mscorlib_assembly(&mut self) {
        let mut assembly = Assembly {
            name: "mscorlib".to_string(),
            version: "4.0.0.0".to_string(),
            types: Vec::new(),
            symbols: SymbolTable::new(),
        };
        
        // Add System.Object
        self.add_object_type(&mut assembly);
        
        // Add System.String
        self.add_string_type(&mut assembly);
        
        // Add System.Console
        self.add_console_type(&mut assembly);
        
        // Add primitive types
        self.add_primitive_types(&mut assembly);
        
        // Add System.Array
        self.add_array_type(&mut assembly);
        
        // Add System.Delegate
        self.add_delegate_type(&mut assembly);
        
        // Add System.Enum
        self.add_enum_type(&mut assembly);
        
        // Add System.ValueType
        self.add_value_type(&mut assembly);
        
        // Add System.DateTime and System.TimeSpan
        self.add_datetime_types(&mut assembly);
        
        // Add System.Guid
        self.add_guid_type(&mut assembly);
        
        // Add System.Math
        self.add_math_type(&mut assembly);
        
        // Add System.Convert
        self.add_convert_type(&mut assembly);
        
        // Add System.Environment
        self.add_environment_type(&mut assembly);
        
        self.assemblies.insert("mscorlib".to_string(), assembly);
    }
    
    /// Add System.Object type
    fn add_object_type(&self, assembly: &mut Assembly) {
        let object_type = TypeInfo {
            full_name: "System.Object".to_string(),
            namespace: "System".to_string(),
            name: "Object".to_string(),
            kind: TypeKind::Class,
            is_public: true,
            base_type: None,
            interfaces: Vec::new(),
            members: vec![
                MemberInfo {
                    name: "ToString".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: false,
                    signature: "() -> System.String".to_string(),
                },
                MemberInfo {
                    name: "GetHashCode".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: false,
                    signature: "() -> System.Int32".to_string(),
                },
                MemberInfo {
                    name: "Equals".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: false,
                    signature: "(System.Object) -> System.Boolean".to_string(),
                },
                MemberInfo {
                    name: "GetType".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: false,
                    signature: "() -> System.Type".to_string(),
                },
            ],
        };
        
        assembly.types.push(object_type);
        
        // Add to symbol table
        let type_symbol = Rc::new(TypeSymbol::new("System.Object".to_string(), TypeKind::Class));
        
        let symbol = Symbol {
            name: "System.Object".to_string(),
            kind: SymbolKind::Type(type_symbol),
            is_public: true,
            is_static: false,
            is_readonly: false,
            scope_id: 0,
        };
        
        assembly.symbols.add_symbol(symbol);
    }
    
    /// Add System.String type
    fn add_string_type(&self, assembly: &mut Assembly) {
        let string_type = TypeInfo {
            full_name: "System.String".to_string(),
            namespace: "System".to_string(),
            name: "String".to_string(),
            kind: TypeKind::Class,
            is_public: true,
            base_type: Some("System.Object".to_string()),
            interfaces: Vec::new(),
            members: vec![
                MemberInfo {
                    name: "Length".to_string(),
                    kind: MemberKind::Property,
                    is_public: true,
                    is_static: false,
                    signature: "System.Int32".to_string(),
                },
                MemberInfo {
                    name: "Substring".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: false,
                    signature: "(System.Int32) -> System.String".to_string(),
                },
                MemberInfo {
                    name: "Concat".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.String, System.String) -> System.String".to_string(),
                },
            ],
        };
        
        assembly.types.push(string_type);
    }
    
    /// Add System.Console type
    fn add_console_type(&self, assembly: &mut Assembly) {
        let console_type = TypeInfo {
            full_name: "System.Console".to_string(),
            namespace: "System".to_string(),
            name: "Console".to_string(),
            kind: TypeKind::Class,
            is_public: true,
            base_type: Some("System.Object".to_string()),
            interfaces: Vec::new(),
            members: vec![
                MemberInfo {
                    name: "WriteLine".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.String) -> System.Void".to_string(),
                },
                MemberInfo {
                    name: "WriteLine".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.Object) -> System.Void".to_string(),
                },
                MemberInfo {
                    name: "Write".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.String) -> System.Void".to_string(),
                },
                MemberInfo {
                    name: "ReadLine".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "() -> System.String".to_string(),
                },
            ],
        };
        
        assembly.types.push(console_type);
        
        // Add Console.WriteLine to symbol table
        let void_type = Rc::new(TypeSymbol::new("void".to_string(), TypeKind::Void));
        let string_type = Rc::new(TypeSymbol::new("string".to_string(), TypeKind::String));
        
        let method_symbol = Rc::new(MethodSymbol {
            name: "WriteLine".to_string(),
            return_type: void_type,
            parameters: vec![
                ParameterSymbol {
                    name: "value".to_string(),
                    param_type: string_type,
                    is_ref: false,
                    is_out: false,
                    is_params: false,
                    has_default: false,
                    default_value: None,
                }
            ],
            type_parameters: Vec::new(),
            is_static: true,
            is_virtual: false,
            is_abstract: false,
            is_override: false,
            is_async: false,
        });
        
        let symbol = Symbol {
            name: "System.Console.WriteLine".to_string(),
            kind: SymbolKind::Method(method_symbol),
            is_public: true,
            is_static: true,
            is_readonly: false,
            scope_id: 0,
        };
        
        assembly.symbols.add_symbol(symbol);
    }
    
    /// Add primitive types
    fn add_primitive_types(&self, assembly: &mut Assembly) {
        let primitives = vec![
            ("System.Boolean", "Boolean", TypeKind::Boolean),
            ("System.Byte", "Byte", TypeKind::Byte),
            ("System.SByte", "SByte", TypeKind::SByte),
            ("System.Int16", "Int16", TypeKind::Short),
            ("System.UInt16", "UInt16", TypeKind::UShort),
            ("System.Int32", "Int32", TypeKind::Int),
            ("System.UInt32", "UInt32", TypeKind::UInt),
            ("System.Int64", "Int64", TypeKind::Long),
            ("System.UInt64", "UInt64", TypeKind::ULong),
            ("System.Single", "Single", TypeKind::Float),
            ("System.Double", "Double", TypeKind::Double),
            ("System.Decimal", "Decimal", TypeKind::Decimal),
            ("System.Char", "Char", TypeKind::Char),
        ];
        
        for (full_name, name, kind) in primitives {
            let type_info = TypeInfo {
                full_name: full_name.to_string(),
                namespace: "System".to_string(),
                name: name.to_string(),
                kind,
                is_public: true,
                base_type: Some("System.ValueType".to_string()),
                interfaces: Vec::new(),
                members: Vec::new(),
            };
            
            assembly.types.push(type_info);
        }
    }
    
    /// Create synthetic System assembly
    fn create_system_assembly(&mut self) {
        let mut assembly = Assembly {
            name: "System".to_string(),
            version: "4.0.0.0".to_string(),
            types: Vec::new(),
            symbols: SymbolTable::new(),
        };
        
        // Add System.Type
        self.add_type_type(&mut assembly);
        
        // Add System.Exception and derived types
        self.add_exception_types(&mut assembly);
        
        // Add System.Collections types
        self.add_collection_types(&mut assembly);
        
        // Add System.IO types
        self.add_io_types(&mut assembly);
        
        // Add System.Text types
        self.add_text_types(&mut assembly);
        
        // Add System.Threading types
        self.add_threading_types(&mut assembly);
        
        // Add System.Linq types
        self.add_linq_types(&mut assembly);
        
        self.assemblies.insert("System".to_string(), assembly);
    }
    
    /// Load an assembly by name
    pub fn load_assembly(&mut self, name: &str) -> Result<&Assembly, String> {
        // Check if already loaded
        if self.assemblies.contains_key(name) {
            return Ok(&self.assemblies[name]);
        }
        
        // Try to find assembly file
        let assembly_file = self.find_assembly_file(name)?;
        
        // Load assembly from file
        self.load_assembly_from_file(&assembly_file)?;
        
        self.assemblies.get(name)
            .ok_or_else(|| format!("Failed to load assembly '{}'", name))
    }
    
    /// Find assembly file
    fn find_assembly_file(&self, name: &str) -> Result<PathBuf, String> {
        let dll_name = format!("{}.dll", name);
        
        for search_path in &self.search_paths {
            let path = search_path.join(&dll_name);
            if path.exists() {
                return Ok(path);
            }
        }
        
        Err(format!("Assembly '{}' not found", name))
    }
    
    /// Load assembly from file
    fn load_assembly_from_file(&mut self, path: &Path) -> Result<(), String> {
        // This is a simplified implementation
        // In reality, we would parse the PE file and read metadata tables
        
        let name = path.file_stem()
            .and_then(|s| s.to_str())
            .ok_or("Invalid assembly file name")?
            .to_string();
        
        let assembly = Assembly {
            name: name.clone(),
            version: "1.0.0.0".to_string(),
            types: Vec::new(),
            symbols: SymbolTable::new(),
        };
        
        self.assemblies.insert(name, assembly);
        Ok(())
    }
    
    /// Get loaded assembly
    pub fn get_assembly(&self, name: &str) -> Option<&Assembly> {
        self.assemblies.get(name)
    }
    
    /// Get all loaded assemblies
    pub fn get_assemblies(&self) -> Vec<&Assembly> {
        self.assemblies.values().collect()
    }
    
    /// Resolve type by full name
    pub fn resolve_type(&self, full_name: &str) -> Option<&TypeInfo> {
        for assembly in self.assemblies.values() {
            for type_info in &assembly.types {
                if type_info.full_name == full_name {
                    return Some(type_info);
                }
            }
        }
        None
    }
    
    /// Add System.Type type
    fn add_type_type(&self, assembly: &mut Assembly) {
        let type_type = TypeInfo {
            full_name: "System.Type".to_string(),
            namespace: "System".to_string(),
            name: "Type".to_string(),
            kind: TypeKind::Class,
            is_public: true,
            base_type: Some("System.Object".to_string()),
            interfaces: Vec::new(),
            members: vec![
                MemberInfo {
                    name: "GetType".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.String) -> System.Type".to_string(),
                },
                MemberInfo {
                    name: "Name".to_string(),
                    kind: MemberKind::Property,
                    is_public: true,
                    is_static: false,
                    signature: "System.String".to_string(),
                },
                MemberInfo {
                    name: "FullName".to_string(),
                    kind: MemberKind::Property,
                    is_public: true,
                    is_static: false,
                    signature: "System.String".to_string(),
                },
                MemberInfo {
                    name: "IsClass".to_string(),
                    kind: MemberKind::Property,
                    is_public: true,
                    is_static: false,
                    signature: "System.Boolean".to_string(),
                },
                MemberInfo {
                    name: "IsInterface".to_string(),
                    kind: MemberKind::Property,
                    is_public: true,
                    is_static: false,
                    signature: "System.Boolean".to_string(),
                },
            ],
        };
        
        assembly.types.push(type_type);
    }
    
    /// Add exception types
    fn add_exception_types(&self, assembly: &mut Assembly) {
        // System.Exception
        let exception_type = TypeInfo {
            full_name: "System.Exception".to_string(),
            namespace: "System".to_string(),
            name: "Exception".to_string(),
            kind: TypeKind::Class,
            is_public: true,
            base_type: Some("System.Object".to_string()),
            interfaces: Vec::new(),
            members: vec![
                MemberInfo {
                    name: "Message".to_string(),
                    kind: MemberKind::Property,
                    is_public: true,
                    is_static: false,
                    signature: "System.String".to_string(),
                },
                MemberInfo {
                    name: "StackTrace".to_string(),
                    kind: MemberKind::Property,
                    is_public: true,
                    is_static: false,
                    signature: "System.String".to_string(),
                },
                MemberInfo {
                    name: "InnerException".to_string(),
                    kind: MemberKind::Property,
                    is_public: true,
                    is_static: false,
                    signature: "System.Exception".to_string(),
                },
            ],
        };
        
        assembly.types.push(exception_type);
        
        // Common derived exception types
        let derived_exceptions = vec![
            ("System.ArgumentException", "ArgumentException"),
            ("System.ArgumentNullException", "ArgumentNullException"),
            ("System.InvalidOperationException", "InvalidOperationException"),
            ("System.NotImplementedException", "NotImplementedException"),
            ("System.NullReferenceException", "NullReferenceException"),
            ("System.IndexOutOfRangeException", "IndexOutOfRangeException"),
        ];
        
        for (full_name, name) in derived_exceptions {
            let exception = TypeInfo {
                full_name: full_name.to_string(),
                namespace: "System".to_string(),
                name: name.to_string(),
                kind: TypeKind::Class,
                is_public: true,
                base_type: Some("System.Exception".to_string()),
                interfaces: Vec::new(),
                members: Vec::new(),
            };
            assembly.types.push(exception);
        }
    }
    
    /// Add collection types
    fn add_collection_types(&self, assembly: &mut Assembly) {
        // System.Collections.Generic.List<T>
        let list_type = TypeInfo {
            full_name: "System.Collections.Generic.List`1".to_string(),
            namespace: "System.Collections.Generic".to_string(),
            name: "List`1".to_string(),
            kind: TypeKind::Class,
            is_public: true,
            base_type: Some("System.Object".to_string()),
            interfaces: vec!["System.Collections.Generic.IList`1".to_string()],
            members: vec![
                MemberInfo {
                    name: "Add".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: false,
                    signature: "(T) -> System.Void".to_string(),
                },
                MemberInfo {
                    name: "Remove".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: false,
                    signature: "(T) -> System.Boolean".to_string(),
                },
                MemberInfo {
                    name: "Count".to_string(),
                    kind: MemberKind::Property,
                    is_public: true,
                    is_static: false,
                    signature: "System.Int32".to_string(),
                },
                MemberInfo {
                    name: "Clear".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: false,
                    signature: "() -> System.Void".to_string(),
                },
            ],
        };
        
        assembly.types.push(list_type);
        
        // System.Collections.Generic.Dictionary<TKey, TValue>
        let dict_type = TypeInfo {
            full_name: "System.Collections.Generic.Dictionary`2".to_string(),
            namespace: "System.Collections.Generic".to_string(),
            name: "Dictionary`2".to_string(),
            kind: TypeKind::Class,
            is_public: true,
            base_type: Some("System.Object".to_string()),
            interfaces: vec!["System.Collections.Generic.IDictionary`2".to_string()],
            members: vec![
                MemberInfo {
                    name: "Add".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: false,
                    signature: "(TKey, TValue) -> System.Void".to_string(),
                },
                MemberInfo {
                    name: "ContainsKey".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: false,
                    signature: "(TKey) -> System.Boolean".to_string(),
                },
                MemberInfo {
                    name: "TryGetValue".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: false,
                    signature: "(TKey, out TValue) -> System.Boolean".to_string(),
                },
            ],
        };
        
        assembly.types.push(dict_type);
        
        // IEnumerable<T>
        let enumerable_type = TypeInfo {
            full_name: "System.Collections.Generic.IEnumerable`1".to_string(),
            namespace: "System.Collections.Generic".to_string(),
            name: "IEnumerable`1".to_string(),
            kind: TypeKind::Interface,
            is_public: true,
            base_type: None,
            interfaces: Vec::new(),
            members: vec![
                MemberInfo {
                    name: "GetEnumerator".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: false,
                    signature: "() -> System.Collections.Generic.IEnumerator`1".to_string(),
                },
            ],
        };
        
        assembly.types.push(enumerable_type);
    }
    
    /// Add IO types
    fn add_io_types(&self, assembly: &mut Assembly) {
        // System.IO.File
        let file_type = TypeInfo {
            full_name: "System.IO.File".to_string(),
            namespace: "System.IO".to_string(),
            name: "File".to_string(),
            kind: TypeKind::Class,
            is_public: true,
            base_type: Some("System.Object".to_string()),
            interfaces: Vec::new(),
            members: vec![
                MemberInfo {
                    name: "ReadAllText".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.String) -> System.String".to_string(),
                },
                MemberInfo {
                    name: "WriteAllText".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.String, System.String) -> System.Void".to_string(),
                },
                MemberInfo {
                    name: "Exists".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.String) -> System.Boolean".to_string(),
                },
                MemberInfo {
                    name: "Delete".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.String) -> System.Void".to_string(),
                },
            ],
        };
        
        assembly.types.push(file_type);
        
        // System.IO.Directory
        let directory_type = TypeInfo {
            full_name: "System.IO.Directory".to_string(),
            namespace: "System.IO".to_string(),
            name: "Directory".to_string(),
            kind: TypeKind::Class,
            is_public: true,
            base_type: Some("System.Object".to_string()),
            interfaces: Vec::new(),
            members: vec![
                MemberInfo {
                    name: "CreateDirectory".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.String) -> System.IO.DirectoryInfo".to_string(),
                },
                MemberInfo {
                    name: "Exists".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.String) -> System.Boolean".to_string(),
                },
                MemberInfo {
                    name: "GetFiles".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.String) -> System.String[]".to_string(),
                },
            ],
        };
        
        assembly.types.push(directory_type);
        
        // System.IO.Path
        let path_type = TypeInfo {
            full_name: "System.IO.Path".to_string(),
            namespace: "System.IO".to_string(),
            name: "Path".to_string(),
            kind: TypeKind::Class,
            is_public: true,
            base_type: Some("System.Object".to_string()),
            interfaces: Vec::new(),
            members: vec![
                MemberInfo {
                    name: "Combine".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.String, System.String) -> System.String".to_string(),
                },
                MemberInfo {
                    name: "GetFileName".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.String) -> System.String".to_string(),
                },
                MemberInfo {
                    name: "GetExtension".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.String) -> System.String".to_string(),
                },
            ],
        };
        
        assembly.types.push(path_type);
    }
    
    /// Add text types
    fn add_text_types(&self, assembly: &mut Assembly) {
        // System.Text.StringBuilder
        let stringbuilder_type = TypeInfo {
            full_name: "System.Text.StringBuilder".to_string(),
            namespace: "System.Text".to_string(),
            name: "StringBuilder".to_string(),
            kind: TypeKind::Class,
            is_public: true,
            base_type: Some("System.Object".to_string()),
            interfaces: Vec::new(),
            members: vec![
                MemberInfo {
                    name: "Append".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: false,
                    signature: "(System.String) -> System.Text.StringBuilder".to_string(),
                },
                MemberInfo {
                    name: "AppendLine".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: false,
                    signature: "(System.String) -> System.Text.StringBuilder".to_string(),
                },
                MemberInfo {
                    name: "ToString".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: false,
                    signature: "() -> System.String".to_string(),
                },
                MemberInfo {
                    name: "Length".to_string(),
                    kind: MemberKind::Property,
                    is_public: true,
                    is_static: false,
                    signature: "System.Int32".to_string(),
                },
            ],
        };
        
        assembly.types.push(stringbuilder_type);
        
        // System.Text.Encoding
        let encoding_type = TypeInfo {
            full_name: "System.Text.Encoding".to_string(),
            namespace: "System.Text".to_string(),
            name: "Encoding".to_string(),
            kind: TypeKind::Class,
            is_public: true,
            base_type: Some("System.Object".to_string()),
            interfaces: Vec::new(),
            members: vec![
                MemberInfo {
                    name: "UTF8".to_string(),
                    kind: MemberKind::Property,
                    is_public: true,
                    is_static: true,
                    signature: "System.Text.Encoding".to_string(),
                },
                MemberInfo {
                    name: "GetBytes".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: false,
                    signature: "(System.String) -> System.Byte[]".to_string(),
                },
                MemberInfo {
                    name: "GetString".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: false,
                    signature: "(System.Byte[]) -> System.String".to_string(),
                },
            ],
        };
        
        assembly.types.push(encoding_type);
    }
    
    /// Add threading types
    fn add_threading_types(&self, assembly: &mut Assembly) {
        // System.Threading.Thread
        let thread_type = TypeInfo {
            full_name: "System.Threading.Thread".to_string(),
            namespace: "System.Threading".to_string(),
            name: "Thread".to_string(),
            kind: TypeKind::Class,
            is_public: true,
            base_type: Some("System.Object".to_string()),
            interfaces: Vec::new(),
            members: vec![
                MemberInfo {
                    name: "Start".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: false,
                    signature: "() -> System.Void".to_string(),
                },
                MemberInfo {
                    name: "Sleep".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.Int32) -> System.Void".to_string(),
                },
                MemberInfo {
                    name: "CurrentThread".to_string(),
                    kind: MemberKind::Property,
                    is_public: true,
                    is_static: true,
                    signature: "System.Threading.Thread".to_string(),
                },
            ],
        };
        
        assembly.types.push(thread_type);
        
        // System.Threading.Tasks.Task
        let task_type = TypeInfo {
            full_name: "System.Threading.Tasks.Task".to_string(),
            namespace: "System.Threading.Tasks".to_string(),
            name: "Task".to_string(),
            kind: TypeKind::Class,
            is_public: true,
            base_type: Some("System.Object".to_string()),
            interfaces: Vec::new(),
            members: vec![
                MemberInfo {
                    name: "Run".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.Action) -> System.Threading.Tasks.Task".to_string(),
                },
                MemberInfo {
                    name: "Wait".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: false,
                    signature: "() -> System.Void".to_string(),
                },
                MemberInfo {
                    name: "Delay".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.Int32) -> System.Threading.Tasks.Task".to_string(),
                },
            ],
        };
        
        assembly.types.push(task_type);
    }
    
    /// Add LINQ types
    fn add_linq_types(&self, assembly: &mut Assembly) {
        // System.Linq.Enumerable
        let enumerable_type = TypeInfo {
            full_name: "System.Linq.Enumerable".to_string(),
            namespace: "System.Linq".to_string(),
            name: "Enumerable".to_string(),
            kind: TypeKind::Class,
            is_public: true,
            base_type: Some("System.Object".to_string()),
            interfaces: Vec::new(),
            members: vec![
                MemberInfo {
                    name: "Where".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(IEnumerable<T>, Func<T, bool>) -> IEnumerable<T>".to_string(),
                },
                MemberInfo {
                    name: "Select".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(IEnumerable<T>, Func<T, TResult>) -> IEnumerable<TResult>".to_string(),
                },
                MemberInfo {
                    name: "ToList".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(IEnumerable<T>) -> List<T>".to_string(),
                },
                MemberInfo {
                    name: "ToArray".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(IEnumerable<T>) -> T[]".to_string(),
                },
                MemberInfo {
                    name: "FirstOrDefault".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(IEnumerable<T>) -> T".to_string(),
                },
                MemberInfo {
                    name: "Count".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(IEnumerable<T>) -> System.Int32".to_string(),
                },
            ],
        };
        
        assembly.types.push(enumerable_type);
    }
    
    /// Add System.Array type
    fn add_array_type(&self, assembly: &mut Assembly) {
        let array_type = TypeInfo {
            full_name: "System.Array".to_string(),
            namespace: "System".to_string(),
            name: "Array".to_string(),
            kind: TypeKind::Class,
            is_public: true,
            base_type: Some("System.Object".to_string()),
            interfaces: vec!["System.Collections.IEnumerable".to_string()],
            members: vec![
                MemberInfo {
                    name: "Length".to_string(),
                    kind: MemberKind::Property,
                    is_public: true,
                    is_static: false,
                    signature: "System.Int32".to_string(),
                },
                MemberInfo {
                    name: "GetLength".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: false,
                    signature: "(System.Int32) -> System.Int32".to_string(),
                },
                MemberInfo {
                    name: "GetValue".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: false,
                    signature: "(System.Int32) -> System.Object".to_string(),
                },
                MemberInfo {
                    name: "SetValue".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: false,
                    signature: "(System.Object, System.Int32) -> System.Void".to_string(),
                },
                MemberInfo {
                    name: "Copy".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.Array, System.Array, System.Int32) -> System.Void".to_string(),
                },
                MemberInfo {
                    name: "Sort".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.Array) -> System.Void".to_string(),
                },
            ],
        };
        
        assembly.types.push(array_type);
    }
    
    /// Add System.Delegate type
    fn add_delegate_type(&self, assembly: &mut Assembly) {
        let delegate_type = TypeInfo {
            full_name: "System.Delegate".to_string(),
            namespace: "System".to_string(),
            name: "Delegate".to_string(),
            kind: TypeKind::Class,
            is_public: true,
            base_type: Some("System.Object".to_string()),
            interfaces: Vec::new(),
            members: vec![
                MemberInfo {
                    name: "Invoke".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: false,
                    signature: "() -> System.Object".to_string(),
                },
                MemberInfo {
                    name: "Combine".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.Delegate, System.Delegate) -> System.Delegate".to_string(),
                },
                MemberInfo {
                    name: "Remove".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.Delegate, System.Delegate) -> System.Delegate".to_string(),
                },
            ],
        };
        
        assembly.types.push(delegate_type);
        
        // Add common delegate types
        let action_type = TypeInfo {
            full_name: "System.Action".to_string(),
            namespace: "System".to_string(),
            name: "Action".to_string(),
            kind: TypeKind::Delegate,
            is_public: true,
            base_type: Some("System.Delegate".to_string()),
            interfaces: Vec::new(),
            members: Vec::new(),
        };
        assembly.types.push(action_type);
        
        let func_type = TypeInfo {
            full_name: "System.Func`1".to_string(),
            namespace: "System".to_string(),
            name: "Func`1".to_string(),
            kind: TypeKind::Delegate,
            is_public: true,
            base_type: Some("System.Delegate".to_string()),
            interfaces: Vec::new(),
            members: Vec::new(),
        };
        assembly.types.push(func_type);
    }
    
    /// Add System.Enum type
    fn add_enum_type(&self, assembly: &mut Assembly) {
        let enum_type = TypeInfo {
            full_name: "System.Enum".to_string(),
            namespace: "System".to_string(),
            name: "Enum".to_string(),
            kind: TypeKind::Class,
            is_public: true,
            base_type: Some("System.ValueType".to_string()),
            interfaces: Vec::new(),
            members: vec![
                MemberInfo {
                    name: "Parse".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.Type, System.String) -> System.Object".to_string(),
                },
                MemberInfo {
                    name: "GetValues".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.Type) -> System.Array".to_string(),
                },
                MemberInfo {
                    name: "GetNames".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.Type) -> System.String[]".to_string(),
                },
            ],
        };
        
        assembly.types.push(enum_type);
    }
    
    /// Add System.ValueType type
    fn add_value_type(&self, assembly: &mut Assembly) {
        let value_type = TypeInfo {
            full_name: "System.ValueType".to_string(),
            namespace: "System".to_string(),
            name: "ValueType".to_string(),
            kind: TypeKind::Class,
            is_public: true,
            base_type: Some("System.Object".to_string()),
            interfaces: Vec::new(),
            members: Vec::new(),
        };
        
        assembly.types.push(value_type);
    }
    
    /// Add DateTime and TimeSpan types
    fn add_datetime_types(&self, assembly: &mut Assembly) {
        // System.DateTime
        let datetime_type = TypeInfo {
            full_name: "System.DateTime".to_string(),
            namespace: "System".to_string(),
            name: "DateTime".to_string(),
            kind: TypeKind::Struct,
            is_public: true,
            base_type: Some("System.ValueType".to_string()),
            interfaces: Vec::new(),
            members: vec![
                MemberInfo {
                    name: "Now".to_string(),
                    kind: MemberKind::Property,
                    is_public: true,
                    is_static: true,
                    signature: "System.DateTime".to_string(),
                },
                MemberInfo {
                    name: "Today".to_string(),
                    kind: MemberKind::Property,
                    is_public: true,
                    is_static: true,
                    signature: "System.DateTime".to_string(),
                },
                MemberInfo {
                    name: "UtcNow".to_string(),
                    kind: MemberKind::Property,
                    is_public: true,
                    is_static: true,
                    signature: "System.DateTime".to_string(),
                },
                MemberInfo {
                    name: "Year".to_string(),
                    kind: MemberKind::Property,
                    is_public: true,
                    is_static: false,
                    signature: "System.Int32".to_string(),
                },
                MemberInfo {
                    name: "Month".to_string(),
                    kind: MemberKind::Property,
                    is_public: true,
                    is_static: false,
                    signature: "System.Int32".to_string(),
                },
                MemberInfo {
                    name: "Day".to_string(),
                    kind: MemberKind::Property,
                    is_public: true,
                    is_static: false,
                    signature: "System.Int32".to_string(),
                },
                MemberInfo {
                    name: "AddDays".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: false,
                    signature: "(System.Double) -> System.DateTime".to_string(),
                },
                MemberInfo {
                    name: "ToString".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: false,
                    signature: "(System.String) -> System.String".to_string(),
                },
            ],
        };
        
        assembly.types.push(datetime_type);
        
        // System.TimeSpan
        let timespan_type = TypeInfo {
            full_name: "System.TimeSpan".to_string(),
            namespace: "System".to_string(),
            name: "TimeSpan".to_string(),
            kind: TypeKind::Struct,
            is_public: true,
            base_type: Some("System.ValueType".to_string()),
            interfaces: Vec::new(),
            members: vec![
                MemberInfo {
                    name: "Days".to_string(),
                    kind: MemberKind::Property,
                    is_public: true,
                    is_static: false,
                    signature: "System.Int32".to_string(),
                },
                MemberInfo {
                    name: "Hours".to_string(),
                    kind: MemberKind::Property,
                    is_public: true,
                    is_static: false,
                    signature: "System.Int32".to_string(),
                },
                MemberInfo {
                    name: "Minutes".to_string(),
                    kind: MemberKind::Property,
                    is_public: true,
                    is_static: false,
                    signature: "System.Int32".to_string(),
                },
                MemberInfo {
                    name: "TotalDays".to_string(),
                    kind: MemberKind::Property,
                    is_public: true,
                    is_static: false,
                    signature: "System.Double".to_string(),
                },
                MemberInfo {
                    name: "FromDays".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.Double) -> System.TimeSpan".to_string(),
                },
            ],
        };
        
        assembly.types.push(timespan_type);
    }
    
    /// Add System.Guid type
    fn add_guid_type(&self, assembly: &mut Assembly) {
        let guid_type = TypeInfo {
            full_name: "System.Guid".to_string(),
            namespace: "System".to_string(),
            name: "Guid".to_string(),
            kind: TypeKind::Struct,
            is_public: true,
            base_type: Some("System.ValueType".to_string()),
            interfaces: Vec::new(),
            members: vec![
                MemberInfo {
                    name: "NewGuid".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "() -> System.Guid".to_string(),
                },
                MemberInfo {
                    name: "Empty".to_string(),
                    kind: MemberKind::Field,
                    is_public: true,
                    is_static: true,
                    signature: "System.Guid".to_string(),
                },
                MemberInfo {
                    name: "Parse".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.String) -> System.Guid".to_string(),
                },
                MemberInfo {
                    name: "ToString".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: false,
                    signature: "() -> System.String".to_string(),
                },
            ],
        };
        
        assembly.types.push(guid_type);
    }
    
    /// Add System.Math type
    fn add_math_type(&self, assembly: &mut Assembly) {
        let math_type = TypeInfo {
            full_name: "System.Math".to_string(),
            namespace: "System".to_string(),
            name: "Math".to_string(),
            kind: TypeKind::Class,
            is_public: true,
            base_type: Some("System.Object".to_string()),
            interfaces: Vec::new(),
            members: vec![
                MemberInfo {
                    name: "PI".to_string(),
                    kind: MemberKind::Field,
                    is_public: true,
                    is_static: true,
                    signature: "System.Double".to_string(),
                },
                MemberInfo {
                    name: "E".to_string(),
                    kind: MemberKind::Field,
                    is_public: true,
                    is_static: true,
                    signature: "System.Double".to_string(),
                },
                MemberInfo {
                    name: "Abs".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.Double) -> System.Double".to_string(),
                },
                MemberInfo {
                    name: "Max".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.Double, System.Double) -> System.Double".to_string(),
                },
                MemberInfo {
                    name: "Min".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.Double, System.Double) -> System.Double".to_string(),
                },
                MemberInfo {
                    name: "Sqrt".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.Double) -> System.Double".to_string(),
                },
                MemberInfo {
                    name: "Pow".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.Double, System.Double) -> System.Double".to_string(),
                },
                MemberInfo {
                    name: "Round".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.Double) -> System.Double".to_string(),
                },
                MemberInfo {
                    name: "Ceiling".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.Double) -> System.Double".to_string(),
                },
                MemberInfo {
                    name: "Floor".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.Double) -> System.Double".to_string(),
                },
                MemberInfo {
                    name: "Sin".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.Double) -> System.Double".to_string(),
                },
                MemberInfo {
                    name: "Cos".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.Double) -> System.Double".to_string(),
                },
            ],
        };
        
        assembly.types.push(math_type);
    }
    
    /// Add System.Convert type
    fn add_convert_type(&self, assembly: &mut Assembly) {
        let convert_type = TypeInfo {
            full_name: "System.Convert".to_string(),
            namespace: "System".to_string(),
            name: "Convert".to_string(),
            kind: TypeKind::Class,
            is_public: true,
            base_type: Some("System.Object".to_string()),
            interfaces: Vec::new(),
            members: vec![
                MemberInfo {
                    name: "ToInt32".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.String) -> System.Int32".to_string(),
                },
                MemberInfo {
                    name: "ToDouble".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.String) -> System.Double".to_string(),
                },
                MemberInfo {
                    name: "ToString".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.Int32) -> System.String".to_string(),
                },
                MemberInfo {
                    name: "ToBoolean".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.String) -> System.Boolean".to_string(),
                },
                MemberInfo {
                    name: "ToBase64String".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.Byte[]) -> System.String".to_string(),
                },
                MemberInfo {
                    name: "FromBase64String".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.String) -> System.Byte[]".to_string(),
                },
            ],
        };
        
        assembly.types.push(convert_type);
    }
    
    /// Add System.Environment type
    fn add_environment_type(&self, assembly: &mut Assembly) {
        let environment_type = TypeInfo {
            full_name: "System.Environment".to_string(),
            namespace: "System".to_string(),
            name: "Environment".to_string(),
            kind: TypeKind::Class,
            is_public: true,
            base_type: Some("System.Object".to_string()),
            interfaces: Vec::new(),
            members: vec![
                MemberInfo {
                    name: "NewLine".to_string(),
                    kind: MemberKind::Property,
                    is_public: true,
                    is_static: true,
                    signature: "System.String".to_string(),
                },
                MemberInfo {
                    name: "CurrentDirectory".to_string(),
                    kind: MemberKind::Property,
                    is_public: true,
                    is_static: true,
                    signature: "System.String".to_string(),
                },
                MemberInfo {
                    name: "MachineName".to_string(),
                    kind: MemberKind::Property,
                    is_public: true,
                    is_static: true,
                    signature: "System.String".to_string(),
                },
                MemberInfo {
                    name: "OSVersion".to_string(),
                    kind: MemberKind::Property,
                    is_public: true,
                    is_static: true,
                    signature: "System.OperatingSystem".to_string(),
                },
                MemberInfo {
                    name: "Exit".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.Int32) -> System.Void".to_string(),
                },
                MemberInfo {
                    name: "GetEnvironmentVariable".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "(System.String) -> System.String".to_string(),
                },
                MemberInfo {
                    name: "GetCommandLineArgs".to_string(),
                    kind: MemberKind::Method,
                    is_public: true,
                    is_static: true,
                    signature: "() -> System.String[]".to_string(),
                },
            ],
        };
        
        assembly.types.push(environment_type);
    }
}

impl Default for AssemblyLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_assembly_loader() {
        let loader = AssemblyLoader::new();
        
        // Check core assemblies are loaded
        assert!(loader.get_assembly("mscorlib").is_some());
        assert!(loader.get_assembly("System").is_some());
        
        // Check core types
        assert!(loader.resolve_type("System.Object").is_some());
        assert!(loader.resolve_type("System.String").is_some());
        assert!(loader.resolve_type("System.Console").is_some());
        assert!(loader.resolve_type("System.Int32").is_some());
    }
    
    #[test]
    fn test_mscorlib_types() {
        let loader = AssemblyLoader::new();
        let mscorlib = loader.get_assembly("mscorlib").unwrap();
        
        // Check that all expected types are present
        let type_names: Vec<&str> = mscorlib.types.iter()
            .map(|t| t.full_name.as_str())
            .collect();
        
        // Core types
        assert!(type_names.contains(&"System.Object"));
        assert!(type_names.contains(&"System.String"));
        assert!(type_names.contains(&"System.Console"));
        
        // Primitive types
        assert!(type_names.contains(&"System.Boolean"));
        assert!(type_names.contains(&"System.Int32"));
        assert!(type_names.contains(&"System.Double"));
        
        // Other important types
        assert!(type_names.contains(&"System.Array"));
        assert!(type_names.contains(&"System.Delegate"));
        assert!(type_names.contains(&"System.Enum"));
        assert!(type_names.contains(&"System.ValueType"));
        assert!(type_names.contains(&"System.DateTime"));
        assert!(type_names.contains(&"System.Guid"));
        assert!(type_names.contains(&"System.Math"));
        assert!(type_names.contains(&"System.Convert"));
        assert!(type_names.contains(&"System.Environment"));
    }
    
    #[test]
    fn test_system_assembly_types() {
        let loader = AssemblyLoader::new();
        let system = loader.get_assembly("System").unwrap();
        
        let type_names: Vec<&str> = system.types.iter()
            .map(|t| t.full_name.as_str())
            .collect();
        
        // Exception types
        assert!(type_names.contains(&"System.Exception"));
        assert!(type_names.contains(&"System.ArgumentException"));
        assert!(type_names.contains(&"System.InvalidOperationException"));
        
        // Collection types
        assert!(type_names.contains(&"System.Collections.Generic.List`1"));
        assert!(type_names.contains(&"System.Collections.Generic.Dictionary`2"));
        assert!(type_names.contains(&"System.Collections.Generic.IEnumerable`1"));
        
        // IO types
        assert!(type_names.contains(&"System.IO.File"));
        assert!(type_names.contains(&"System.IO.Directory"));
        assert!(type_names.contains(&"System.IO.Path"));
        
        // Text types
        assert!(type_names.contains(&"System.Text.StringBuilder"));
        assert!(type_names.contains(&"System.Text.Encoding"));
        
        // Threading types
        assert!(type_names.contains(&"System.Threading.Thread"));
        assert!(type_names.contains(&"System.Threading.Tasks.Task"));
        
        // LINQ types
        assert!(type_names.contains(&"System.Linq.Enumerable"));
    }
    
    #[test]
    fn test_type_members() {
        let loader = AssemblyLoader::new();
        
        // Test System.String members
        let string_type = loader.resolve_type("System.String").unwrap();
        assert_eq!(string_type.name, "String");
        assert_eq!(string_type.namespace, "System");
        
        let member_names: Vec<&str> = string_type.members.iter()
            .map(|m| m.name.as_str())
            .collect();
        
        assert!(member_names.contains(&"Length"));
        assert!(member_names.contains(&"Substring"));
        assert!(member_names.contains(&"Concat"));
        
        // Test System.Console members
        let console_type = loader.resolve_type("System.Console").unwrap();
        let console_members: Vec<&str> = console_type.members.iter()
            .map(|m| m.name.as_str())
            .collect();
        
        assert!(console_members.contains(&"WriteLine"));
        assert!(console_members.contains(&"Write"));
        assert!(console_members.contains(&"ReadLine"));
        
        // Test Math members
        let math_type = loader.resolve_type("System.Math").unwrap();
        let math_members: Vec<&str> = math_type.members.iter()
            .map(|m| m.name.as_str())
            .collect();
        
        assert!(math_members.contains(&"PI"));
        assert!(math_members.contains(&"Sqrt"));
        assert!(math_members.contains(&"Sin"));
        assert!(math_members.contains(&"Cos"));
    }
    
    #[test]
    fn test_search_paths() {
        let loader = AssemblyLoader::new();
        
        // Should have at least one search path
        assert!(!loader.search_paths.is_empty());
        
        // On Windows, should include framework paths
        #[cfg(windows)]
        {
            let paths_str: Vec<String> = loader.search_paths.iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect();
            
            // Should have at least one .NET Framework path
            let has_framework_path = paths_str.iter()
                .any(|p| p.contains("Microsoft.NET") || p.contains("dotnet"));
            
            assert!(has_framework_path, "No .NET Framework paths found");
        }
    }
}