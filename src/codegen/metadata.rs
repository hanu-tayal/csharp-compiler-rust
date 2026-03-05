//! Metadata generation for .NET assemblies

use super::il::{ILAssembly, ILType, ILMethod, ILField};
use std::collections::HashMap;

/// Metadata tables as defined in ECMA-335
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetadataTable {
    Module = 0x00,
    TypeRef = 0x01,
    TypeDef = 0x02,
    Field = 0x04,
    MethodDef = 0x06,
    Param = 0x08,
    InterfaceImpl = 0x09,
    MemberRef = 0x0A,
    Constant = 0x0B,
    CustomAttribute = 0x0C,
    FieldMarshal = 0x0D,
    DeclSecurity = 0x0E,
    ClassLayout = 0x0F,
    FieldLayout = 0x10,
    StandAloneSig = 0x11,
    EventMap = 0x12,
    Event = 0x14,
    PropertyMap = 0x15,
    Property = 0x17,
    MethodSemantics = 0x18,
    MethodImpl = 0x19,
    ModuleRef = 0x1A,
    TypeSpec = 0x1B,
    ImplMap = 0x1C,
    FieldRVA = 0x1D,
    Assembly = 0x20,
    AssemblyProcessor = 0x21,
    AssemblyOS = 0x22,
    AssemblyRef = 0x23,
    AssemblyRefProcessor = 0x24,
    AssemblyRefOS = 0x25,
    File = 0x26,
    ExportedType = 0x27,
    ManifestResource = 0x28,
    NestedClass = 0x29,
    GenericParam = 0x2A,
    MethodSpec = 0x2B,
    GenericParamConstraint = 0x2C,
}

/// Metadata builder for creating .NET metadata
pub struct MetadataBuilder {
    /// String heap
    strings: StringHeap,
    /// User string heap
    user_strings: UserStringHeap,
    /// GUID heap
    guids: GuidHeap,
    /// Blob heap
    blobs: BlobHeap,
    /// Metadata tables
    tables: HashMap<MetadataTable, Vec<TableRow>>,
}

/// String heap for metadata strings
#[derive(Debug)]
struct StringHeap {
    data: Vec<u8>,
    strings: HashMap<String, u32>,
}

/// User string heap for string literals
#[derive(Debug)]
struct UserStringHeap {
    data: Vec<u8>,
    strings: HashMap<String, u32>,
}

/// GUID heap
#[derive(Debug)]
struct GuidHeap {
    guids: Vec<[u8; 16]>,
}

/// Blob heap for signatures and other binary data
#[derive(Debug)]
struct BlobHeap {
    data: Vec<u8>,
    blobs: HashMap<Vec<u8>, u32>,
}

/// Generic table row
#[derive(Debug, Clone)]
struct TableRow {
    values: Vec<u32>,
}

/// Type definition flags
#[derive(Debug, Clone, Copy)]
pub struct TypeDefFlags(u32);

impl TypeDefFlags {
    pub const VISIBILITY_MASK: u32 = 0x00000007;
    pub const NOT_PUBLIC: u32 = 0x00000000;
    pub const PUBLIC: u32 = 0x00000001;
    pub const NESTED_PUBLIC: u32 = 0x00000002;
    pub const NESTED_PRIVATE: u32 = 0x00000003;
    pub const NESTED_FAMILY: u32 = 0x00000004;
    pub const NESTED_ASSEMBLY: u32 = 0x00000005;
    pub const NESTED_FAM_AND_ASSEM: u32 = 0x00000006;
    pub const NESTED_FAM_OR_ASSEM: u32 = 0x00000007;
    
    pub const LAYOUT_MASK: u32 = 0x00000018;
    pub const AUTO_LAYOUT: u32 = 0x00000000;
    pub const SEQUENTIAL_LAYOUT: u32 = 0x00000008;
    pub const EXPLICIT_LAYOUT: u32 = 0x00000010;
    
    pub const CLASS_SEMANTICS_MASK: u32 = 0x00000020;
    pub const CLASS: u32 = 0x00000000;
    pub const INTERFACE: u32 = 0x00000020;
    
    pub const ABSTRACT: u32 = 0x00000080;
    pub const SEALED: u32 = 0x00000100;
    pub const SPECIAL_NAME: u32 = 0x00000400;
    pub const RT_SPECIAL_NAME: u32 = 0x00000800;
    pub const IMPORT: u32 = 0x00001000;
    pub const SERIALIZABLE: u32 = 0x00002000;
    
    pub const STRING_FORMAT_MASK: u32 = 0x00030000;
    pub const ANSI_CLASS: u32 = 0x00000000;
    pub const UNICODE_CLASS: u32 = 0x00010000;
    pub const AUTO_CLASS: u32 = 0x00020000;
    
    pub const BEFORE_FIELD_INIT: u32 = 0x00100000;
}

/// Method definition flags
#[derive(Debug, Clone, Copy)]
pub struct MethodDefFlags(u16);

impl MethodDefFlags {
    pub const MEMBER_ACCESS_MASK: u16 = 0x0007;
    pub const COMPILER_CONTROLLED: u16 = 0x0000;
    pub const PRIVATE: u16 = 0x0001;
    pub const FAM_AND_ASSEM: u16 = 0x0002;
    pub const ASSEMBLY: u16 = 0x0003;
    pub const FAMILY: u16 = 0x0004;
    pub const FAM_OR_ASSEM: u16 = 0x0005;
    pub const PUBLIC: u16 = 0x0006;
    
    pub const STATIC: u16 = 0x0010;
    pub const FINAL: u16 = 0x0020;
    pub const VIRTUAL: u16 = 0x0040;
    pub const HIDE_BY_SIG: u16 = 0x0080;
    
    pub const VTABLE_LAYOUT_MASK: u16 = 0x0100;
    pub const REUSE_SLOT: u16 = 0x0000;
    pub const NEW_SLOT: u16 = 0x0100;
    
    pub const ABSTRACT: u16 = 0x0400;
    pub const SPECIAL_NAME: u16 = 0x0800;
    pub const RT_SPECIAL_NAME: u16 = 0x1000;
}

impl MetadataBuilder {
    /// Create a new metadata builder
    pub fn new() -> Self {
        Self {
            strings: StringHeap::new(),
            user_strings: UserStringHeap::new(),
            guids: GuidHeap::new(),
            blobs: BlobHeap::new(),
            tables: HashMap::new(),
        }
    }
    
    /// Add a string to the string heap
    pub fn add_string(&mut self, s: &str) -> u32 {
        self.strings.add(s)
    }
    
    /// Add a user string
    pub fn add_user_string(&mut self, s: &str) -> u32 {
        self.user_strings.add(s)
    }
    
    /// Add a GUID
    pub fn add_guid(&mut self, guid: [u8; 16]) -> u32 {
        self.guids.add(guid)
    }
    
    /// Add a blob
    pub fn add_blob(&mut self, data: &[u8]) -> u32 {
        self.blobs.add(data)
    }
    
    /// Add a module definition
    pub fn add_module(&mut self, name: &str, mvid: [u8; 16]) -> u32 {
        let name_idx = self.add_string(name);
        let mvid_idx = self.add_guid(mvid);
        
        self.add_table_row(MetadataTable::Module, vec![
            0,          // Generation
            name_idx,   // Name
            mvid_idx,   // Mvid
            0,          // EncId
            0,          // EncBaseId
        ])
    }
    
    /// Add a type definition
    pub fn add_type_def(
        &mut self,
        flags: u32,
        name: &str,
        namespace: &str,
        extends: u32,
        field_list: u32,
        method_list: u32,
    ) -> u32 {
        let name_idx = self.add_string(name);
        let namespace_idx = self.add_string(namespace);
        
        self.add_table_row(MetadataTable::TypeDef, vec![
            flags,
            name_idx,
            namespace_idx,
            extends,
            field_list,
            method_list,
        ])
    }
    
    /// Add a method definition
    pub fn add_method_def(
        &mut self,
        rva: u32,
        impl_flags: u16,
        flags: u16,
        name: &str,
        signature: &[u8],
        param_list: u32,
    ) -> u32 {
        let name_idx = self.add_string(name);
        let sig_idx = self.add_blob(signature);
        
        self.add_table_row(MetadataTable::MethodDef, vec![
            rva,
            impl_flags as u32,
            flags as u32,
            name_idx,
            sig_idx,
            param_list,
        ])
    }
    
    /// Add a field definition
    pub fn add_field_def(
        &mut self,
        flags: u16,
        name: &str,
        signature: &[u8],
    ) -> u32 {
        let name_idx = self.add_string(name);
        let sig_idx = self.add_blob(signature);
        
        self.add_table_row(MetadataTable::Field, vec![
            flags as u32,
            name_idx,
            sig_idx,
        ])
    }
    
    /// Add an assembly reference
    pub fn add_assembly_ref(
        &mut self,
        major: u16,
        minor: u16,
        build: u16,
        revision: u16,
        flags: u32,
        public_key: &[u8],
        name: &str,
        culture: &str,
    ) -> u32 {
        let public_key_idx = if public_key.is_empty() {
            0
        } else {
            self.add_blob(public_key)
        };
        let name_idx = self.add_string(name);
        let culture_idx = self.add_string(culture);
        
        self.add_table_row(MetadataTable::AssemblyRef, vec![
            major as u32,
            minor as u32,
            build as u32,
            revision as u32,
            flags,
            public_key_idx,
            name_idx,
            culture_idx,
            0, // HashValue
        ])
    }
    
    /// Add a type reference
    pub fn add_type_ref(
        &mut self,
        resolution_scope: u32,
        name: &str,
        namespace: &str,
    ) -> u32 {
        let name_idx = self.add_string(name);
        let namespace_idx = self.add_string(namespace);
        
        self.add_table_row(MetadataTable::TypeRef, vec![
            resolution_scope,
            name_idx,
            namespace_idx,
        ])
    }
    
    /// Add a member reference
    pub fn add_member_ref(
        &mut self,
        class: u32,
        name: &str,
        signature: &[u8],
    ) -> u32 {
        let name_idx = self.add_string(name);
        let sig_idx = self.add_blob(signature);
        
        self.add_table_row(MetadataTable::MemberRef, vec![
            class,
            name_idx,
            sig_idx,
        ])
    }
    
    /// Add a row to a metadata table
    fn add_table_row(&mut self, table: MetadataTable, values: Vec<u32>) -> u32 {
        let rows = self.tables.entry(table).or_insert_with(Vec::new);
        let index = rows.len() as u32;
        rows.push(TableRow { values });
        index + 1 // Metadata indices are 1-based
    }
    
    /// Build the final metadata
    pub fn build(&self, assembly: &ILAssembly) -> Result<Vec<u8>, String> {
        // This would generate the actual metadata binary format
        // For now, return a minimal metadata structure
        let mut metadata = Vec::new();
        
        // Metadata header
        metadata.extend_from_slice(b"BSJB"); // Signature
        metadata.extend_from_slice(&[1, 0]); // Major version
        metadata.extend_from_slice(&[1, 0]); // Minor version
        metadata.extend_from_slice(&[0, 0, 0, 0]); // Reserved
        
        // Version string length and string
        let version = b"v4.0.30319\0";
        metadata.extend_from_slice(&[version.len() as u8, 0, 0, 0]);
        metadata.extend_from_slice(version);
        
        // Flags
        metadata.extend_from_slice(&[0, 0]);
        
        // Number of streams
        metadata.extend_from_slice(&[5, 0]); // 5 streams
        
        // TODO: Add stream headers and data
        // - #~ (metadata tables)
        // - #Strings
        // - #US (user strings)
        // - #GUID
        // - #Blob
        
        Ok(metadata)
    }
    
    /// Build metadata from IL assembly
    pub fn build_from_assembly(&mut self, assembly: &ILAssembly) -> Result<(), String> {
        // Add module
        let mvid = [0u8; 16]; // Generate proper MVID
        self.add_module(&assembly.name, mvid);
        
        // Add assembly info
        self.add_assembly_info(&assembly.name, &assembly.version);
        
        // Add assembly references
        for reference in &assembly.references {
            self.add_assembly_ref(4, 0, 0, 0, 0, &[], reference, "");
        }
        
        // Add types
        for (idx, il_type) in assembly.types.iter().enumerate() {
            self.add_il_type(il_type, idx as u32 + 1)?;
        }
        
        Ok(())
    }
    
    /// Add assembly info
    fn add_assembly_info(&mut self, name: &str, version: &str) -> u32 {
        let name_idx = self.add_string(name);
        let version_parts: Vec<u16> = version.split('.')
            .map(|s| s.parse().unwrap_or(0))
            .collect();
        
        let (major, minor, build, revision) = match version_parts.as_slice() {
            [a, b, c, d, ..] => (*a, *b, *c, *d),
            [a, b, c] => (*a, *b, *c, 0),
            [a, b] => (*a, *b, 0, 0),
            [a] => (*a, 0, 0, 0),
            [] => (1, 0, 0, 0),
        };
        
        self.add_table_row(MetadataTable::Assembly, vec![
            0x8004, // HashAlgId (SHA1)
            major as u32,
            minor as u32,
            build as u32,
            revision as u32,
            0, // Flags
            0, // PublicKey
            name_idx,
            0, // Culture
        ])
    }
    
    /// Add IL type to metadata
    fn add_il_type(&mut self, il_type: &ILType, type_idx: u32) -> Result<u32, String> {
        let mut flags = TypeDefFlags::CLASS;
        if il_type.is_sealed {
            flags |= TypeDefFlags::SEALED;
        }
        if il_type.is_abstract {
            flags |= TypeDefFlags::ABSTRACT;
        }
        
        // Determine base type token
        let base_token = if let Some(base_type) = &il_type.base_type {
            // For now, use a simple type reference
            self.add_type_ref(1, base_type, "System")
        } else {
            0
        };
        
        let field_list = 1; // TODO: Calculate actual field list
        let method_list = 1; // TODO: Calculate actual method list
        
        let type_def_idx = self.add_type_def(
            flags,
            &il_type.name,
            &il_type.namespace,
            base_token,
            field_list,
            method_list,
        );
        
        // Add methods
        for method in &il_type.methods {
            self.add_il_method(method, type_def_idx)?;
        }
        
        // Add fields
        for field in &il_type.fields {
            self.add_il_field(field)?;
        }
        
        Ok(type_def_idx)
    }
    
    /// Add IL method to metadata
    fn add_il_method(&mut self, method: &ILMethod, type_idx: u32) -> Result<u32, String> {
        let mut flags = MethodDefFlags::HIDE_BY_SIG;
        if method.is_static {
            flags |= MethodDefFlags::STATIC;
        }
        flags |= MethodDefFlags::PUBLIC; // TODO: Get actual visibility
        
        // Build method signature
        let mut signature = Vec::new();
        // Calling convention
        signature.push(if method.is_static { 0x00 } else { 0x20 });
        // Parameter count
        signature.push(method.parameters.len() as u8);
        // Return type (simplified)
        signature.push(0x01); // void
        // Parameters (simplified)
        for _ in &method.parameters {
            signature.push(0x0E); // string
        }
        
        let method_idx = self.add_method_def(
            0x2050, // RVA (placeholder)
            0,      // ImplFlags
            flags as u16,
            &method.name,
            &signature,
            1,      // ParamList
        );
        
        Ok(method_idx)
    }
    
    /// Add IL field to metadata
    fn add_il_field(&mut self, field: &ILField) -> Result<u32, String> {
        let mut flags = 0u16;
        if field.is_static {
            flags |= 0x0010; // Static
        }
        if field.is_public {
            flags |= 0x0006; // Public
        } else if field.is_private {
            flags |= 0x0001; // Private
        }
        
        // Build field signature
        let mut signature = Vec::new();
        signature.push(0x06); // FIELD
        signature.push(0x0E); // string (simplified)
        
        let field_idx = self.add_field_def(
            flags,
            &field.name,
            &signature,
        );
        
        Ok(field_idx)
    }
}

impl StringHeap {
    fn new() -> Self {
        let mut heap = Self {
            data: Vec::new(),
            strings: HashMap::new(),
        };
        // Empty string at offset 0
        heap.data.push(0);
        heap
    }
    
    fn add(&mut self, s: &str) -> u32 {
        if let Some(&offset) = self.strings.get(s) {
            return offset;
        }
        
        let offset = self.data.len() as u32;
        self.data.extend_from_slice(s.as_bytes());
        self.data.push(0); // Null terminator
        self.strings.insert(s.to_string(), offset);
        offset
    }
}

impl UserStringHeap {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            strings: HashMap::new(),
        }
    }
    
    fn add(&mut self, s: &str) -> u32 {
        if let Some(&offset) = self.strings.get(s) {
            return offset;
        }
        
        let offset = self.data.len() as u32;
        
        // Encode string length and UTF-16 data
        let utf16: Vec<u16> = s.encode_utf16().collect();
        let len = utf16.len() * 2;
        
        // Compressed length encoding
        if len < 0x80 {
            self.data.push(len as u8);
        } else if len < 0x4000 {
            self.data.push((0x80 | (len >> 8)) as u8);
            self.data.push((len & 0xFF) as u8);
        } else {
            self.data.push((0xC0 | (len >> 24)) as u8);
            self.data.push(((len >> 16) & 0xFF) as u8);
            self.data.push(((len >> 8) & 0xFF) as u8);
            self.data.push((len & 0xFF) as u8);
        }
        
        // UTF-16 data
        for ch in utf16 {
            self.data.extend_from_slice(&ch.to_le_bytes());
        }
        
        // Trailing byte (indicates if string has special chars)
        self.data.push(0);
        
        self.strings.insert(s.to_string(), offset);
        offset
    }
}

impl GuidHeap {
    fn new() -> Self {
        Self {
            guids: Vec::new(),
        }
    }
    
    fn add(&mut self, guid: [u8; 16]) -> u32 {
        let index = self.guids.len() as u32;
        self.guids.push(guid);
        index + 1 // GUID indices are 1-based
    }
}

impl BlobHeap {
    fn new() -> Self {
        let mut heap = Self {
            data: Vec::new(),
            blobs: HashMap::new(),
        };
        // Empty blob at offset 0
        heap.data.push(0);
        heap
    }
    
    fn add(&mut self, blob: &[u8]) -> u32 {
        if let Some(&offset) = self.blobs.get(blob) {
            return offset;
        }
        
        let offset = self.data.len() as u32;
        
        // Compressed length encoding
        let len = blob.len();
        if len < 0x80 {
            self.data.push(len as u8);
        } else if len < 0x4000 {
            self.data.push((0x80 | (len >> 8)) as u8);
            self.data.push((len & 0xFF) as u8);
        } else {
            self.data.push((0xC0 | (len >> 24)) as u8);
            self.data.push(((len >> 16) & 0xFF) as u8);
            self.data.push(((len >> 8) & 0xFF) as u8);
            self.data.push((len & 0xFF) as u8);
        }
        
        self.data.extend_from_slice(blob);
        self.blobs.insert(blob.to_vec(), offset);
        offset
    }
}

impl Default for MetadataBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_string_heap() {
        let mut heap = StringHeap::new();
        
        let idx1 = heap.add("Hello");
        let idx2 = heap.add("World");
        let idx3 = heap.add("Hello"); // Duplicate
        
        assert_eq!(idx1, 1); // After empty string
        assert_ne!(idx1, idx2);
        assert_eq!(idx1, idx3); // Same string, same index
    }
    
    #[test]
    fn test_metadata_builder() {
        let mut builder = MetadataBuilder::new();
        
        // Add a simple type
        let type_idx = builder.add_type_def(
            TypeDefFlags::PUBLIC | TypeDefFlags::CLASS,
            "Program",
            "TestNamespace",
            0, // No base type for now
            1, // Field list
            1, // Method list
        );
        
        assert_eq!(type_idx, 1); // First type
    }
}