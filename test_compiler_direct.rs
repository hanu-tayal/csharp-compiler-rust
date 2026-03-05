use csharp_compiler::{CSharpCompiler, CompilerOptions};
use std::path::Path;

fn main() {
    let source = r#"
using System;

class Program
{
    static void Main()
    {
        Console.WriteLine("Hello, World!");
    }
}
"#;

    let options = CompilerOptions::default();
    let mut compiler = CSharpCompiler::new(options);
    
    println!("Testing direct compilation...");
    let result = compiler.compile_source(source, Path::new("test_direct.exe"));
    
    println!("Compilation success: {}", result.success);
    println!("Diagnostics count: {}", result.diagnostics().len());
    
    for diag in result.diagnostics() {
        println!("  {}: {}", diag.code(), diag.message());
    }
    
    if let Some(output) = result.output_file {
        println!("Output file: {}", output.display());
    }
}