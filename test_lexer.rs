// Simple test for the lexer
// Run with: rustc test_lexer.rs -o test_lexer.exe && ./test_lexer.exe

use std::fs;

fn main() {
    let source = r#"
using System;

namespace HelloWorld
{
    class Program
    {
        static void Main(string[] args)
        {
            // This is a comment
            Console.WriteLine("Hello, World!");
            
            int x = 42;
            var y = x + 10;
            
            if (y > 50)
            {
                Console.WriteLine($"y = {y}");
            }
        }
    }
}
"#;

    println!("Testing C# Lexer");
    println!("================");
    println!("Source code:");
    println!("{}", source);
    println!("\nTokens:");
    println!("-------");
    
    // Simple tokenization based on whitespace and special chars
    let mut tokens = Vec::new();
    let mut current = String::new();
    
    for ch in source.chars() {
        match ch {
            ' ' | '\t' | '\n' | '\r' => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
                if ch == '\n' {
                    tokens.push("\\n".to_string());
                }
            }
            '(' | ')' | '{' | '}' | '[' | ']' | ';' | ',' | '.' => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
                tokens.push(ch.to_string());
            }
            _ => {
                current.push(ch);
            }
        }
    }
    
    if !current.is_empty() {
        tokens.push(current);
    }
    
    // Print tokens
    for (i, token) in tokens.iter().enumerate() {
        if token == "\\n" {
            println!();
        } else {
            print!("{} ", token);
        }
        
        if i > 100 {
            println!("\n... (truncated)");
            break;
        }
    }
    
    println!("\n\nTotal tokens: {}", tokens.len());
}