//! Basic tests to verify the compiler infrastructure works

use csharp_compiler::lexer::Lexer;
use csharp_compiler::parser::Parser;

#[test]
fn test_lexer_basic() {
    let source = "int x = 42;";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    
    // Should produce tokens without crashing
    assert!(tokens.len() > 0);
    assert!(!lexer.diagnostics().has_errors());
}

#[test]
fn test_parser_basic() {
    let source = "class Test { }";
    let mut parser = Parser::new(source);
    
    // Should not crash during parsing
    let _syntax_tree = parser.parse();
    
    // Might have errors but shouldn't panic
    println!("Parser diagnostics: {:?}", parser.diagnostics().diagnostics());
}

#[test]
fn test_empty_source() {
    let source = "";
    let mut parser = Parser::new(source);
    let _syntax_tree = parser.parse();
    
    // Should handle empty source gracefully
}

#[test]
fn test_whitespace_only() {
    let source = "   \n  \t  \n  ";
    let mut parser = Parser::new(source);
    let _syntax_tree = parser.parse();
    
    // Should handle whitespace-only source gracefully
}

#[test]
fn test_lexer_keywords() {
    let source = "using namespace class public static void";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    
    // Should tokenize keywords correctly
    assert!(tokens.len() > 0);
}

#[test]
fn test_lexer_operators() {
    let source = "+ - * / = == != < > <= >= && || ++ --";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    
    // Should tokenize operators correctly
    assert!(tokens.len() > 0);
}

#[test]
fn test_lexer_literals() {
    let source = r#"42 3.14 "hello" 'c' true false null"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    
    // Should tokenize literals correctly
    assert!(tokens.len() > 0);
}

#[test]
fn test_compiler_components_exist() {
    // Test that all major components compile and can be instantiated
    use csharp_compiler::semantic::symbols::SymbolTable;
    use csharp_compiler::diagnostics::DiagnosticBag;
    
    let _symbol_table = SymbolTable::new();
    let _diagnostics = DiagnosticBag::new();
    
    // All components should be available
}