//! IL emitter for generating .NET assemblies

use super::il::{ILAssembly, ILMethod, ILInstruction, get_opcode};
use std::collections::HashMap;
use std::io::Write;

/// IL emitter for converting IL assembly to bytecode
pub struct ILEmitter {
    /// String heap
    string_heap: Vec<String>,
    /// Type references
    type_refs: HashMap<String, u32>,
    /// Method references
    method_refs: HashMap<String, u32>,
    /// Field references
    field_refs: HashMap<String, u32>,
    /// Next metadata token
    next_token: u32,
}

impl ILEmitter {
    /// Create a new IL emitter
    pub fn new() -> Self {
        Self {
            string_heap: Vec::new(),
            type_refs: HashMap::new(),
            method_refs: HashMap::new(),
            field_refs: HashMap::new(),
            next_token: 0x01000001, // Start of user tokens
        }
    }
    
    /// Emit IL assembly to bytecode
    pub fn emit(&mut self, assembly: &ILAssembly) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();
        
        // Build metadata tables
        self.build_metadata_tables(assembly)?;
        
        // Emit each type and its methods
        for il_type in &assembly.types {
            for method in &il_type.methods {
                let method_bytes = self.emit_method(method)?;
                output.extend(method_bytes);
            }
        }
        
        Ok(output)
    }
    
    /// Build metadata tables from assembly
    fn build_metadata_tables(&mut self, assembly: &ILAssembly) -> Result<(), String> {
        // Add assembly references
        for reference in &assembly.references {
            self.add_assembly_ref(reference);
        }
        
        // Add types
        for il_type in &assembly.types {
            let type_name = format!("{}.{}", il_type.namespace, il_type.name);
            self.add_type_ref(&type_name);
            
            // Add methods
            for method in &il_type.methods {
                let method_sig = format!("{}::{}", type_name, method.name);
                self.add_method_ref(&method_sig);
            }
        }
        
        Ok(())
    }
    
    /// Emit a single method
    fn emit_method(&mut self, method: &ILMethod) -> Result<Vec<u8>, String> {
        let mut bytes = Vec::new();
        
        // Method header
        let header = self.build_method_header(method);
        bytes.extend(header);
        
        // Method body
        for instruction in &method.instructions {
            let inst_bytes = self.emit_instruction(instruction)?;
            bytes.extend(inst_bytes);
        }
        
        Ok(bytes)
    }
    
    /// Build method header
    fn build_method_header(&self, method: &ILMethod) -> Vec<u8> {
        let mut header = Vec::new();
        
        // Tiny header format (for simple methods)
        if method.local_count == 0 && method.max_stack <= 8 {
            let header_byte = 0x02 | ((method.instructions.len() as u8) << 2);
            header.push(header_byte);
        } else {
            // Fat header format
            header.push(0x03); // Flags: Fat format
            header.push(0x30); // Header size
            header.extend_from_slice(&method.max_stack.to_le_bytes());
            header.extend_from_slice(&(method.instructions.len() as u32).to_le_bytes());
            header.extend_from_slice(&method.local_count.to_le_bytes());
        }
        
        header
    }
    
    /// Emit a single instruction
    fn emit_instruction(&mut self, instruction: &ILInstruction) -> Result<Vec<u8>, String> {
        let mut bytes = Vec::new();
        
        // Get opcode bytes
        let opcode_bytes = get_opcode(instruction);
        bytes.extend(opcode_bytes);
        
        // Add operands
        match instruction {
            ILInstruction::Ldc_i4_s(value) => bytes.push(*value as u8),
            ILInstruction::Ldc_i4(value) => bytes.extend_from_slice(&value.to_le_bytes()),
            ILInstruction::Ldc_i8(value) => bytes.extend_from_slice(&value.to_le_bytes()),
            ILInstruction::Ldc_r4(value) => bytes.extend_from_slice(&value.to_le_bytes()),
            ILInstruction::Ldc_r8(value) => bytes.extend_from_slice(&value.to_le_bytes()),
            
            ILInstruction::Ldstr(s) => {
                let token = self.add_string(s.clone());
                bytes.extend_from_slice(&token.to_le_bytes());
            }
            
            ILInstruction::Ldloc(index) | ILInstruction::Stloc(index) => {
                if *index <= 255 {
                    bytes.push(*index as u8);
                } else {
                    bytes.extend_from_slice(&index.to_le_bytes());
                }
            }
            
            ILInstruction::Call(method_ref) | ILInstruction::Callvirt(method_ref) => {
                let token = self.get_method_token(method_ref)?;
                bytes.extend_from_slice(&token.to_le_bytes());
            }
            
            ILInstruction::Br(offset) | ILInstruction::Brfalse(offset) | 
            ILInstruction::Brtrue(offset) | ILInstruction::Beq(offset) |
            ILInstruction::Bge(offset) | ILInstruction::Bgt(offset) |
            ILInstruction::Ble(offset) | ILInstruction::Blt(offset) => {
                bytes.extend_from_slice(&(*offset as i32).to_le_bytes());
            }
            
            ILInstruction::Newobj(ctor_ref) => {
                let token = self.get_method_token(ctor_ref)?;
                bytes.extend_from_slice(&token.to_le_bytes());
            }
            
            _ => {} // Instructions without operands
        }
        
        Ok(bytes)
    }
    
    /// Add string to heap and return token
    fn add_string(&mut self, value: String) -> u32 {
        // Check if string already exists
        if let Some(index) = self.string_heap.iter().position(|s| s == &value) {
            return 0x70000001 + index as u32; // String token
        }
        
        let index = self.string_heap.len();
        self.string_heap.push(value);
        0x70000001 + index as u32
    }
    
    /// Add assembly reference
    fn add_assembly_ref(&mut self, name: &str) {
        self.type_refs.insert(format!("[{}]", name), self.next_token);
        self.next_token += 1;
    }
    
    /// Add type reference
    fn add_type_ref(&mut self, name: &str) {
        if !self.type_refs.contains_key(name) {
            self.type_refs.insert(name.to_string(), self.next_token);
            self.next_token += 1;
        }
    }
    
    /// Add method reference
    fn add_method_ref(&mut self, signature: &str) {
        if !self.method_refs.contains_key(signature) {
            self.method_refs.insert(signature.to_string(), self.next_token);
            self.next_token += 1;
        }
    }
    
    /// Get method token
    fn get_method_token(&self, signature: &str) -> Result<u32, String> {
        self.method_refs.get(signature)
            .copied()
            .ok_or_else(|| format!("Method not found: {}", signature))
    }
}

impl Default for ILEmitter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::il::{ILAssembly, ILMethod, ILInstruction};
    
    #[test]
    fn test_il_emitter() {
        let mut emitter = ILEmitter::new();
        let mut assembly = ILAssembly::new("TestAssembly");
        
        // Create a simple method
        let mut method = ILMethod::new("Test", vec![], "System.Int32".to_string(), true, false);
        method.add_instruction(ILInstruction::Ldc_i4(42));
        method.add_instruction(ILInstruction::Ret);
        
        let type_idx = assembly.add_class("TestClass", "System.Object");
        assembly.add_method(type_idx, method);
        
        let result = emitter.emit(&assembly);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_string_constants() {
        let mut emitter = ILEmitter::new();
        
        let token1 = emitter.add_string("Hello".to_string());
        let token2 = emitter.add_string("World".to_string());
        let token3 = emitter.add_string("Hello".to_string()); // Duplicate
        
        assert_eq!(token1, 0x70000001);
        assert_eq!(token2, 0x70000002);
        assert_eq!(token3, token1); // Should reuse existing string
    }
}