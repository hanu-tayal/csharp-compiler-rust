use csharp_compiler::parser::Parser;

fn main() {
    let source = "class Program { }";
    let mut parser = Parser::new(source);
    let tree = parser.parse();
    
    if parser.diagnostics().has_errors() {
        println!("Parser errors:");
        for diag in parser.diagnostics().iter() {
            println!("  {}: {}", diag.code(), diag.message());
        }
    } else {
        println!("Parsing successful!");
    }
}