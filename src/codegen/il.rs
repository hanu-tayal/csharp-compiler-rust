//! IL (Intermediate Language) bytecode definitions and structures

use std::collections::HashMap;

/// IL Instruction opcodes
#[derive(Debug, Clone, PartialEq)]
pub enum ILInstruction {
    // Stack manipulation
    Nop,
    Pop,
    Dup,
    
    // Load constants
    Ldnull,
    Ldc_i4_m1,
    Ldc_i4_0,
    Ldc_i4_1,
    Ldc_i4_2,
    Ldc_i4_3,
    Ldc_i4_4,
    Ldc_i4_5,
    Ldc_i4_6,
    Ldc_i4_7,
    Ldc_i4_8,
    Ldc_i4_s(i8),
    Ldc_i4(i32),
    Ldc_i8(i64),
    Ldc_r4(f32),
    Ldc_r8(f64),
    Ldstr(String),
    
    // Load/store locals
    Ldloc_0,
    Ldloc_1,
    Ldloc_2,
    Ldloc_3,
    Ldloc(u16),
    Ldloca(u16),
    Stloc_0,
    Stloc_1,
    Stloc_2,
    Stloc_3,
    Stloc(u16),
    
    // Load/store arguments
    Ldarg_0,
    Ldarg_1,
    Ldarg_2,
    Ldarg_3,
    Ldarg(u16),
    Ldarga(u16),
    Starg(u16),
    
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Div_un,
    Rem,
    Rem_un,
    Neg,
    
    // Bitwise
    And,
    Or,
    Xor,
    Not,
    Shl,
    Shr,
    Shr_un,
    
    // Comparison
    Ceq,
    Cgt,
    Cgt_un,
    Clt,
    Clt_un,
    
    // Branching
    Br(u32),
    Brfalse(u32),
    Brtrue(u32),
    Beq(u32),
    Bge(u32),
    Bgt(u32),
    Ble(u32),
    Blt(u32),
    Bne_un(u32),
    Bge_un(u32),
    Bgt_un(u32),
    Ble_un(u32),
    Blt_un(u32),
    
    // Method calls
    Call(String),
    Callvirt(String),
    Ret,
    
    // Object model
    Newobj(String),
    Newarr(String),
    Ldlen,
    Ldelema(String),
    Ldelem_i1,
    Ldelem_i2,
    Ldelem_i4,
    Ldelem_i8,
    Ldelem_r4,
    Ldelem_r8,
    Ldelem_ref,
    Stelem_i1,
    Stelem_i2,
    Stelem_i4,
    Stelem_i8,
    Stelem_r4,
    Stelem_r8,
    Stelem_ref,
    
    // Fields
    Ldfld(String),
    Ldflda(String),
    Stfld(String),
    Ldsfld(String),
    Ldsflda(String),
    Stsfld(String),
    
    // Type conversion
    Conv_i1,
    Conv_i2,
    Conv_i4,
    Conv_i8,
    Conv_r4,
    Conv_r8,
    Conv_u1,
    Conv_u2,
    Conv_u4,
    Conv_u8,
    
    // Exception handling
    Throw,
    Rethrow,
    Leave(u32),
    Endfinally,
    Endfilter,
    
    // Other
    Ldtoken(String),
    Box(String),
    Unbox(String),
    Unbox_any(String),
    Castclass(String),
    Isinst(String),
    Sizeof(String),
}

/// IL Method definition
#[derive(Debug, Clone)]
pub struct ILMethod {
    pub name: String,
    pub parameters: Vec<String>,
    pub return_type: String,
    pub is_static: bool,
    pub is_instance: bool,
    pub instructions: Vec<ILInstruction>,
    pub local_count: u16,
    pub max_stack: u16,
    labels: HashMap<u32, usize>,
    next_label: u32,
}

impl ILMethod {
    pub fn new(name: &str, parameters: Vec<String>, return_type: String, is_static: bool, is_instance: bool) -> Self {
        Self {
            name: name.to_string(),
            parameters,
            return_type,
            is_static,
            is_instance,
            instructions: Vec::new(),
            local_count: 0,
            max_stack: 8, // Default, should be calculated
            labels: HashMap::new(),
            next_label: 0,
        }
    }
    
    pub fn add_instruction(&mut self, instruction: ILInstruction) {
        self.instructions.push(instruction);
    }
    
    pub fn create_label(&mut self) -> u32 {
        let label = self.next_label;
        self.next_label += 1;
        label
    }
    
    pub fn mark_label(&mut self, label: u32) {
        self.labels.insert(label, self.instructions.len());
    }
    
    pub fn resolve_labels(&mut self) {
        // Convert label references to actual offsets
        for (i, instruction) in self.instructions.iter_mut().enumerate() {
            match instruction {
                ILInstruction::Br(label) |
                ILInstruction::Brfalse(label) |
                ILInstruction::Brtrue(label) |
                ILInstruction::Beq(label) |
                ILInstruction::Bge(label) |
                ILInstruction::Bgt(label) |
                ILInstruction::Ble(label) |
                ILInstruction::Blt(label) |
                ILInstruction::Bne_un(label) |
                ILInstruction::Bge_un(label) |
                ILInstruction::Bgt_un(label) |
                ILInstruction::Ble_un(label) |
                ILInstruction::Blt_un(label) |
                ILInstruction::Leave(label) => {
                    if let Some(&target) = self.labels.get(label) {
                        *label = (target as i32 - i as i32) as u32;
                    }
                }
                _ => {}
            }
        }
    }
}

/// IL Type definition
#[derive(Debug, Clone)]
pub struct ILType {
    pub name: String,
    pub namespace: String,
    pub base_type: Option<String>,
    pub interfaces: Vec<String>,
    pub fields: Vec<ILField>,
    pub methods: Vec<ILMethod>,
    pub is_value_type: bool,
    pub is_sealed: bool,
    pub is_abstract: bool,
}

/// IL Field definition
#[derive(Debug, Clone)]
pub struct ILField {
    pub name: String,
    pub field_type: String,
    pub is_static: bool,
    pub is_readonly: bool,
    pub is_private: bool,
    pub is_public: bool,
}

/// IL Assembly definition
#[derive(Debug, Clone)]
pub struct ILAssembly {
    pub name: String,
    pub version: String,
    pub types: Vec<ILType>,
    pub entry_point: Option<String>,
    pub references: Vec<String>,
    type_index: HashMap<String, usize>,
}

impl ILAssembly {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            version: "1.0.0.0".to_string(),
            types: Vec::new(),
            entry_point: None,
            references: vec![
                "mscorlib".to_string(),
                "System".to_string(),
                "System.Runtime".to_string(),
            ],
            type_index: HashMap::new(),
        }
    }
    
    pub fn add_class(&mut self, name: &str, base_type: &str) -> usize {
        let il_type = ILType {
            name: name.to_string(),
            namespace: String::new(),
            base_type: Some(base_type.to_string()),
            interfaces: Vec::new(),
            fields: Vec::new(),
            methods: Vec::new(),
            is_value_type: false,
            is_sealed: false,
            is_abstract: false,
        };
        
        let index = self.types.len();
        self.types.push(il_type);
        self.type_index.insert(name.to_string(), index);
        index
    }
    
    pub fn add_method(&mut self, type_index: usize, method: ILMethod) {
        if let Some(il_type) = self.types.get_mut(type_index) {
            il_type.methods.push(method);
        }
    }
    
    pub fn set_entry_point(&mut self, method_ref: &str) {
        self.entry_point = Some(method_ref.to_string());
    }
    
    pub fn finalize(&mut self) {
        // Resolve all labels in methods
        for il_type in &mut self.types {
            for method in &mut il_type.methods {
                method.resolve_labels();
            }
        }
    }
}

/// Get the opcode byte(s) for an instruction
pub fn get_opcode(instruction: &ILInstruction) -> Vec<u8> {
    match instruction {
        ILInstruction::Nop => vec![0x00],
        ILInstruction::Pop => vec![0x26],
        ILInstruction::Dup => vec![0x25],
        
        ILInstruction::Ldnull => vec![0x14],
        ILInstruction::Ldc_i4_m1 => vec![0x15],
        ILInstruction::Ldc_i4_0 => vec![0x16],
        ILInstruction::Ldc_i4_1 => vec![0x17],
        ILInstruction::Ldc_i4_2 => vec![0x18],
        ILInstruction::Ldc_i4_3 => vec![0x19],
        ILInstruction::Ldc_i4_4 => vec![0x1A],
        ILInstruction::Ldc_i4_5 => vec![0x1B],
        ILInstruction::Ldc_i4_6 => vec![0x1C],
        ILInstruction::Ldc_i4_7 => vec![0x1D],
        ILInstruction::Ldc_i4_8 => vec![0x1E],
        ILInstruction::Ldc_i4_s(_) => vec![0x1F],
        ILInstruction::Ldc_i4(_) => vec![0x20],
        ILInstruction::Ldc_i8(_) => vec![0x21],
        ILInstruction::Ldc_r4(_) => vec![0x22],
        ILInstruction::Ldc_r8(_) => vec![0x23],
        ILInstruction::Ldstr(_) => vec![0x72],
        
        ILInstruction::Ldloc_0 => vec![0x06],
        ILInstruction::Ldloc_1 => vec![0x07],
        ILInstruction::Ldloc_2 => vec![0x08],
        ILInstruction::Ldloc_3 => vec![0x09],
        ILInstruction::Ldloc(_) => vec![0xFE, 0x0C],
        ILInstruction::Ldloca(_) => vec![0xFE, 0x0D],
        ILInstruction::Stloc_0 => vec![0x0A],
        ILInstruction::Stloc_1 => vec![0x0B],
        ILInstruction::Stloc_2 => vec![0x0C],
        ILInstruction::Stloc_3 => vec![0x0D],
        ILInstruction::Stloc(_) => vec![0xFE, 0x0E],
        
        ILInstruction::Add => vec![0x58],
        ILInstruction::Sub => vec![0x59],
        ILInstruction::Mul => vec![0x5A],
        ILInstruction::Div => vec![0x5B],
        ILInstruction::Div_un => vec![0x5C],
        ILInstruction::Rem => vec![0x5D],
        ILInstruction::Rem_un => vec![0x5E],
        ILInstruction::Neg => vec![0x65],
        
        ILInstruction::And => vec![0x5F],
        ILInstruction::Or => vec![0x60],
        ILInstruction::Xor => vec![0x61],
        ILInstruction::Not => vec![0x66],
        
        ILInstruction::Ceq => vec![0xFE, 0x01],
        ILInstruction::Cgt => vec![0xFE, 0x02],
        ILInstruction::Cgt_un => vec![0xFE, 0x03],
        ILInstruction::Clt => vec![0xFE, 0x04],
        ILInstruction::Clt_un => vec![0xFE, 0x05],
        
        ILInstruction::Br(_) => vec![0x38],
        ILInstruction::Brfalse(_) => vec![0x39],
        ILInstruction::Brtrue(_) => vec![0x3A],
        
        ILInstruction::Call(_) => vec![0x28],
        ILInstruction::Callvirt(_) => vec![0x6F],
        ILInstruction::Ret => vec![0x2A],
        
        ILInstruction::Newobj(_) => vec![0x73],
        ILInstruction::Throw => vec![0x7A],
        
        // Add more opcodes as needed
        _ => vec![0x00], // Default to NOP
    }
}