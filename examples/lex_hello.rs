//! Example that lexes the Hello.cs file

use csharp_compiler::lexer::Lexer;

fn main() {
    let source = r#"
using System;

namespace HelloWorld {
    class Program {
        static void Main(string[] args) {
            Console.WriteLine("Hello, World!");
            int x = 42;
            var y = x + 10;
        }
    }
}
"#;

    println!("Lexing C# source code:");
    println!("{}", "-".repeat(60));
    println!("{}", source);
    println!("{}", "-".repeat(60));
    println!();

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    
    println!("Tokens:");
    for (i, token) in tokens.iter().enumerate() {
        println!("{:3}: {:20} {:?}", i, format!("{:?}", token.kind), token.text);
        
        if i > 50 {
            println!("... (truncated)");
            break;
        }
    }
    
    println!();
    println!("Total tokens: {}", tokens.len());
    
    if lexer.diagnostics().has_errors() {
        println!("\nLexer errors:");
        for diag in lexer.diagnostics().iter() {
            println!("  {}: {}", diag.code(), diag.message());
        }
    } else {
        println!("\nNo lexer errors!");
    }
}