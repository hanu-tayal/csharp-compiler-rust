//! PE (Portable Executable) file generation

use super::il::{ILAssembly, ILMethod, ILInstruction, get_opcode};
use super::metadata::MetadataBuilder;
use std::collections::HashMap;
use std::io::Write;

/// PE file generator
pub struct PEGenerator {
    /// Assembly to generate
    assembly: ILAssembly,
    /// Metadata builder
    metadata_builder: MetadataBuilder,
    /// RVA (Relative Virtual Address) for sections
    current_rva: u32,
    /// File offset
    current_file_offset: u32,
}

/// DOS header structure
#[repr(C)]
struct DosHeader {
    signature: [u8; 2],           // MZ
    bytes_on_last_page: u16,
    pages_in_file: u16,
    relocations: u16,
    size_of_header: u16,
    min_extra_paragraphs: u16,
    max_extra_paragraphs: u16,
    initial_ss: u16,
    initial_sp: u16,
    checksum: u16,
    initial_ip: u16,
    initial_cs: u16,
    reloc_table_offset: u16,
    overlay_number: u16,
    reserved1: [u16; 4],
    oem_id: u16,
    oem_info: u16,
    reserved2: [u16; 10],
    pe_header_offset: u32,
}

/// PE header structure
#[repr(C)]
struct PEHeader {
    signature: [u8; 4],          // PE\0\0
    machine: u16,                // 0x14c for x86, 0x8664 for x64
    number_of_sections: u16,
    time_date_stamp: u32,
    pointer_to_symbol_table: u32,
    number_of_symbols: u32,
    size_of_optional_header: u16,
    characteristics: u16,
}

/// Optional header structure (64-bit)
#[repr(C)]
struct OptionalHeader64 {
    magic: u16,                  // 0x20b for PE32+
    major_linker_version: u8,
    minor_linker_version: u8,
    size_of_code: u32,
    size_of_initialized_data: u32,
    size_of_uninitialized_data: u32,
    address_of_entry_point: u32,
    base_of_code: u32,
    image_base: u64,
    section_alignment: u32,
    file_alignment: u32,
    major_os_version: u16,
    minor_os_version: u16,
    major_image_version: u16,
    minor_image_version: u16,
    major_subsystem_version: u16,
    minor_subsystem_version: u16,
    reserved: u32,
    size_of_image: u32,
    size_of_headers: u32,
    checksum: u32,
    subsystem: u16,
    dll_characteristics: u16,
    size_of_stack_reserve: u64,
    size_of_stack_commit: u64,
    size_of_heap_reserve: u64,
    size_of_heap_commit: u64,
    loader_flags: u32,
    number_of_rva_and_sizes: u32,
}

/// Data directory entry
#[repr(C)]
struct DataDirectory {
    virtual_address: u32,
    size: u32,
}

/// Section header
#[repr(C)]
struct SectionHeader {
    name: [u8; 8],
    virtual_size: u32,
    virtual_address: u32,
    size_of_raw_data: u32,
    pointer_to_raw_data: u32,
    pointer_to_relocations: u32,
    pointer_to_line_numbers: u32,
    number_of_relocations: u16,
    number_of_line_numbers: u16,
    characteristics: u32,
}

impl PEGenerator {
    /// Create a new PE generator
    pub fn new(assembly: ILAssembly) -> Self {
        Self {
            assembly,
            metadata_builder: MetadataBuilder::new(),
            current_rva: 0x2000,     // Start after headers
            current_file_offset: 0,
        }
    }
    
    /// Generate PE file
    pub fn generate(&mut self) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();
        
        // DOS header
        self.write_dos_header(&mut output)?;
        
        // DOS stub
        self.write_dos_stub(&mut output)?;
        
        // PE signature
        output.extend_from_slice(b"PE\0\0");
        
        // PE header
        self.write_pe_header(&mut output)?;
        
        // Optional header
        self.write_optional_header(&mut output)?;
        
        // Data directories
        self.write_data_directories(&mut output)?;
        
        // Section headers
        self.write_section_headers(&mut output)?;
        
        // Pad to section alignment
        while output.len() < 0x200 {
            output.push(0);
        }
        
        // Write sections
        self.write_text_section(&mut output)?;
        self.write_rsrc_section(&mut output)?;
        self.write_reloc_section(&mut output)?;
        
        Ok(output)
    }
    
    /// Write DOS header
    fn write_dos_header(&self, output: &mut Vec<u8>) -> Result<(), String> {
        // MZ signature
        output.extend_from_slice(&[0x4D, 0x5A]);
        
        // DOS header fields
        output.extend_from_slice(&[0x90, 0x00]); // Bytes on last page
        output.extend_from_slice(&[0x03, 0x00]); // Pages in file
        output.extend_from_slice(&[0x00, 0x00]); // Relocations
        output.extend_from_slice(&[0x04, 0x00]); // Size of header
        output.extend_from_slice(&[0x00, 0x00]); // Min extra paragraphs
        output.extend_from_slice(&[0xFF, 0xFF]); // Max extra paragraphs
        output.extend_from_slice(&[0x00, 0x00]); // Initial SS
        output.extend_from_slice(&[0xB8, 0x00]); // Initial SP
        output.extend_from_slice(&[0x00, 0x00]); // Checksum
        output.extend_from_slice(&[0x00, 0x00]); // Initial IP
        output.extend_from_slice(&[0x00, 0x00]); // Initial CS
        output.extend_from_slice(&[0x40, 0x00]); // Relocation table offset
        output.extend_from_slice(&[0x00, 0x00]); // Overlay number
        
        // Reserved
        for _ in 0..8 {
            output.extend_from_slice(&[0x00, 0x00]);
        }
        
        // PE header offset
        output.extend_from_slice(&[0x80, 0x00, 0x00, 0x00]); // 0x80
        
        Ok(())
    }
    
    /// Write DOS stub
    fn write_dos_stub(&self, output: &mut Vec<u8>) -> Result<(), String> {
        // Simple DOS stub program
        let stub = b"This program cannot be run in DOS mode.\r\r\n$\0";
        output.extend_from_slice(stub);
        
        // Pad to PE header offset (0x80)
        while output.len() < 0x80 {
            output.push(0);
        }
        
        Ok(())
    }
    
    /// Write PE header
    fn write_pe_header(&self, output: &mut Vec<u8>) -> Result<(), String> {
        // Machine type (x64)
        output.extend_from_slice(&[0x64, 0x86]); // IMAGE_FILE_MACHINE_AMD64
        
        // Number of sections
        output.extend_from_slice(&[0x03, 0x00]); // 3 sections: .text, .rsrc, .reloc
        
        // Time stamp
        output.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
        
        // Symbol table pointer
        output.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
        
        // Number of symbols
        output.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
        
        // Size of optional header
        output.extend_from_slice(&[0xF0, 0x00]); // 240 bytes
        
        // Characteristics
        output.extend_from_slice(&[0x22, 0x00]); // Executable, large address aware
        
        Ok(())
    }
    
    /// Write optional header
    fn write_optional_header(&self, output: &mut Vec<u8>) -> Result<(), String> {
        // Magic (PE32+)
        output.extend_from_slice(&[0x0B, 0x02]);
        
        // Linker version
        output.extend_from_slice(&[0x0E, 0x00]); // 14.0
        
        // Size of code
        output.extend_from_slice(&[0x00, 0x10, 0x00, 0x00]); // 4096
        
        // Size of initialized data
        output.extend_from_slice(&[0x00, 0x10, 0x00, 0x00]); // 4096
        
        // Size of uninitialized data
        output.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
        
        // Entry point RVA
        output.extend_from_slice(&[0x00, 0x20, 0x00, 0x00]); // 0x2000
        
        // Base of code
        output.extend_from_slice(&[0x00, 0x20, 0x00, 0x00]); // 0x2000
        
        // Image base
        output.extend_from_slice(&[0x00, 0x00, 0x40, 0x00, 0x01, 0x00, 0x00, 0x00]); // 0x140000000
        
        // Section alignment
        output.extend_from_slice(&[0x00, 0x20, 0x00, 0x00]); // 8192
        
        // File alignment
        output.extend_from_slice(&[0x00, 0x02, 0x00, 0x00]); // 512
        
        // OS version
        output.extend_from_slice(&[0x06, 0x00, 0x00, 0x00]); // 6.0
        
        // Image version
        output.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
        
        // Subsystem version
        output.extend_from_slice(&[0x06, 0x00, 0x00, 0x00]); // 6.0
        
        // Reserved
        output.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
        
        // Size of image
        output.extend_from_slice(&[0x00, 0x80, 0x00, 0x00]); // 32768
        
        // Size of headers
        output.extend_from_slice(&[0x00, 0x02, 0x00, 0x00]); // 512
        
        // Checksum
        output.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
        
        // Subsystem (CUI - console)
        output.extend_from_slice(&[0x03, 0x00]);
        
        // DLL characteristics
        output.extend_from_slice(&[0x60, 0x81]); // High entropy VA, NX compatible, terminal server aware
        
        // Stack reserve
        output.extend_from_slice(&[0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00]); // 1MB
        
        // Stack commit
        output.extend_from_slice(&[0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // 4KB
        
        // Heap reserve
        output.extend_from_slice(&[0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00]); // 1MB
        
        // Heap commit
        output.extend_from_slice(&[0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // 4KB
        
        // Loader flags
        output.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
        
        // Number of data directories
        output.extend_from_slice(&[0x10, 0x00, 0x00, 0x00]); // 16
        
        Ok(())
    }
    
    /// Write data directories
    fn write_data_directories(&self, output: &mut Vec<u8>) -> Result<(), String> {
        // Export table
        output.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        
        // Import table
        output.extend_from_slice(&[0x00, 0x40, 0x00, 0x00, 0x4F, 0x00, 0x00, 0x00]); // RVA 0x4000, size 0x4F
        
        // Resource table
        output.extend_from_slice(&[0x00, 0x60, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00]); // RVA 0x6000, size 0x400
        
        // Exception table
        output.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        
        // Certificate table
        output.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        
        // Base relocation table
        output.extend_from_slice(&[0x00, 0x80, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00]); // RVA 0x8000, size 0x0C
        
        // Debug
        output.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        
        // Architecture
        output.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        
        // Global pointer
        output.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        
        // TLS table
        output.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        
        // Load config table
        output.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        
        // Bound import
        output.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        
        // IAT
        output.extend_from_slice(&[0x00, 0x20, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00]); // RVA 0x2000, size 0x08
        
        // Delay import
        output.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        
        // CLR runtime header
        output.extend_from_slice(&[0x08, 0x20, 0x00, 0x00, 0x48, 0x00, 0x00, 0x00]); // RVA 0x2008, size 0x48
        
        // Reserved
        output.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        
        Ok(())
    }
    
    /// Write section headers
    fn write_section_headers(&self, output: &mut Vec<u8>) -> Result<(), String> {
        // .text section
        self.write_section_header(output, b".text\0\0\0", 0x1000, 0x2000, 0x1000, 0x200, 0x60000020)?;
        
        // .rsrc section
        self.write_section_header(output, b".rsrc\0\0\0", 0x1000, 0x4000, 0x1000, 0x1200, 0x40000040)?;
        
        // .reloc section
        self.write_section_header(output, b".reloc\0\0", 0x1000, 0x6000, 0x1000, 0x2200, 0x42000040)?;
        
        Ok(())
    }
    
    /// Write a section header
    fn write_section_header(
        &self,
        output: &mut Vec<u8>,
        name: &[u8; 8],
        virtual_size: u32,
        virtual_address: u32,
        size_of_raw_data: u32,
        pointer_to_raw_data: u32,
        characteristics: u32,
    ) -> Result<(), String> {
        output.extend_from_slice(name);
        output.extend_from_slice(&virtual_size.to_le_bytes());
        output.extend_from_slice(&virtual_address.to_le_bytes());
        output.extend_from_slice(&size_of_raw_data.to_le_bytes());
        output.extend_from_slice(&pointer_to_raw_data.to_le_bytes());
        output.extend_from_slice(&[0; 4]); // Pointer to relocations
        output.extend_from_slice(&[0; 4]); // Pointer to line numbers
        output.extend_from_slice(&[0; 2]); // Number of relocations
        output.extend_from_slice(&[0; 2]); // Number of line numbers
        output.extend_from_slice(&characteristics.to_le_bytes());
        Ok(())
    }
    
    /// Write .text section
    fn write_text_section(&self, output: &mut Vec<u8>) -> Result<(), String> {
        let section_start = output.len();
        
        // Import Address Table (IAT)
        output.extend_from_slice(&[0x00, 0x30, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00]); // Import
        
        // CLR header
        self.write_clr_header(output)?;
        
        // IL code
        self.write_il_code(output)?;
        
        // Pad to section size
        while output.len() < section_start + 0x1000 {
            output.push(0);
        }
        
        Ok(())
    }
    
    /// Write CLR header
    fn write_clr_header(&self, output: &mut Vec<u8>) -> Result<(), String> {
        // Size
        output.extend_from_slice(&[0x48, 0x00, 0x00, 0x00]);
        
        // Major runtime version
        output.extend_from_slice(&[0x02, 0x00]);
        
        // Minor runtime version
        output.extend_from_slice(&[0x05, 0x00]);
        
        // Metadata RVA and size
        output.extend_from_slice(&[0x00, 0x22, 0x00, 0x00]); // RVA 0x2200
        output.extend_from_slice(&[0x00, 0x04, 0x00, 0x00]); // Size 0x400
        
        // Flags
        output.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]); // ILONLY
        
        // Entry point token
        output.extend_from_slice(&[0x01, 0x00, 0x00, 0x06]); // Main method token
        
        // Resources
        output.extend_from_slice(&[0; 8]);
        
        // Strong name signature
        output.extend_from_slice(&[0; 8]);
        
        // Code manager table
        output.extend_from_slice(&[0; 8]);
        
        // VTable fixups
        output.extend_from_slice(&[0; 8]);
        
        // Export address table jumps
        output.extend_from_slice(&[0; 8]);
        
        // Managed native header
        output.extend_from_slice(&[0; 8]);
        
        Ok(())
    }
    
    /// Write IL code
    fn write_il_code(&self, output: &mut Vec<u8>) -> Result<(), String> {
        // Find entry point method
        let entry_point = self.assembly.entry_point.as_ref()
            .ok_or("No entry point defined")?;
        
        // Simple IL code for Hello World
        // Method header (tiny format)
        output.push(0x02 | (5 << 2)); // Tiny header, 5 bytes of code
        
        // IL instructions
        output.extend_from_slice(&[0x72]); // ldstr
        output.extend_from_slice(&[0x01, 0x00, 0x00, 0x70]); // String token
        
        output.extend_from_slice(&[0x28]); // call
        output.extend_from_slice(&[0x02, 0x00, 0x00, 0x0A]); // Method token
        
        output.push(0x2A); // ret
        
        Ok(())
    }
    
    /// Write .rsrc section
    fn write_rsrc_section(&mut self, output: &mut Vec<u8>) -> Result<(), String> {
        let section_start = output.len();
        
        // Metadata
        self.write_metadata(output)?;
        
        // Pad to section size
        while output.len() < section_start + 0x1000 {
            output.push(0);
        }
        
        Ok(())
    }
    
    /// Write metadata
    fn write_metadata(&mut self, output: &mut Vec<u8>) -> Result<(), String> {
        // Build metadata
        self.metadata_builder.build_from_assembly(&self.assembly)?;
        let metadata = self.metadata_builder.build(&self.assembly)?;
        
        output.extend(metadata);
        Ok(())
    }
    
    /// Write .reloc section
    fn write_reloc_section(&self, output: &mut Vec<u8>) -> Result<(), String> {
        let section_start = output.len();
        
        // Simple relocation table
        output.extend_from_slice(&[0x00, 0x20, 0x00, 0x00]); // Page RVA
        output.extend_from_slice(&[0x0C, 0x00, 0x00, 0x00]); // Block size
        output.extend_from_slice(&[0x00, 0x30]); // Type 3, offset 0
        output.extend_from_slice(&[0x00, 0x00]); // Padding
        
        // Pad to section size
        while output.len() < section_start + 0x1000 {
            output.push(0);
        }
        
        Ok(())
    }
}

/// Calculate PE checksum
pub fn calculate_checksum(data: &[u8]) -> u32 {
    let mut checksum: u64 = 0;
    let mut i = 0;
    
    // Process 16-bit words
    while i + 1 < data.len() {
        let word = (data[i] as u64) | ((data[i + 1] as u64) << 8);
        checksum = (checksum & 0xFFFFFFFF) + word + (checksum >> 32);
        i += 2;
    }
    
    // Handle odd byte
    if i < data.len() {
        checksum = (checksum & 0xFFFFFFFF) + (data[i] as u64) + (checksum >> 32);
    }
    
    // Fold 32-bit sum
    checksum = (checksum & 0xFFFF) + (checksum >> 16);
    checksum = checksum + (checksum >> 16);
    checksum = checksum & 0xFFFF;
    
    // Add file size
    checksum += data.len() as u64;
    
    checksum as u32
}