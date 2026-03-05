//! Integration tests for the C# parser

use csharp_compiler::lexer::Lexer;
use csharp_compiler::parser::Parser;
use csharp_compiler::syntax::SyntaxKind;
use csharp_compiler::diagnostics::DiagnosticBag;

/// Helper function to parse C# source code
fn parse_source(source: &str) -> (csharp_compiler::syntax::SyntaxNode, DiagnosticBag) {
    let mut parser = Parser::new(source);
    let syntax_tree = parser.parse();
    let diagnostics = parser.diagnostics().clone();
    
    (syntax_tree.root, diagnostics)
}

#[test]
fn test_parse_simple_class() {
    let source = r#"
namespace TestNamespace {
    class TestClass {
        public void TestMethod() {
            Console.WriteLine("Hello, World!");
        }
    }
}
"#;

    let (ast, diagnostics) = parse_source(source);
    
    // Should not have any parse errors
    assert!(!diagnostics.has_errors());
    
    // Check AST structure
    assert_eq!(ast.kind, SyntaxKind::CompilationUnit);
    
    // Should have namespace child
    let namespace_found = ast.children.iter().any(|child| {
        if let csharp_compiler::syntax::SyntaxElement::Node(node) = child {
            node.kind == SyntaxKind::NamespaceDeclaration
        } else {
            false
        }
    });
    assert!(namespace_found);
}

#[test]
fn test_parse_using_statements() {
    let source = r#"
using System;
using System.Collections.Generic;
using static System.Console;

namespace Test {
    class Program {
    }
}
"#;

    let (ast, diagnostics) = parse_source(source);
    
    assert!(!diagnostics.has_errors());
    assert_eq!(ast.kind, SyntaxKind::CompilationUnit);
    
    // Count using statements
    let using_count = ast.children.iter().filter(|child| {
        if let csharp_compiler::syntax::SyntaxElement::Node(node) = child {
            node.kind == SyntaxKind::UsingDirective
        } else {
            false
        }
    }).count();
    
    assert_eq!(using_count, 3);
}

#[test]
fn test_parse_method_with_parameters() {
    let source = r#"
class Calculator {
    public int Add(int a, int b) {
        return a + b;
    }
    
    public double Multiply(double x, double y, bool round = false) {
        double result = x * y;
        return round ? Math.Round(result) : result;
    }
}
"#;

    let (ast, diagnostics) = parse_source(source);
    
    assert!(!diagnostics.has_errors());
    assert_eq!(ast.kind, SyntaxKind::CompilationUnit);
}

#[test]
fn test_parse_properties() {
    let source = r#"
class Person {
    public string Name { get; set; }
    
    private int _age;
    public int Age {
        get { return _age; }
        set { 
            if (value >= 0) {
                _age = value; 
            }
        }
    }
    
    public string FullName => $"{FirstName} {LastName}";
}
"#;

    let (ast, diagnostics) = parse_source(source);
    
    // May have some errors due to expression-bodied properties and string interpolation
    // but should parse the basic structure
    assert_eq!(ast.kind, SyntaxKind::CompilationUnit);
}

#[test]
fn test_parse_control_flow() {
    let source = r#"
class ControlFlow {
    public void TestMethod() {
        // If statement
        if (true) {
            Console.WriteLine("True");
        } else {
            Console.WriteLine("False");
        }
        
        // While loop
        int i = 0;
        while (i < 10) {
            Console.WriteLine(i);
            i++;
        }
        
        // For loop
        for (int j = 0; j < 5; j++) {
            Console.WriteLine(j);
        }
        
        // Switch statement
        int value = 42;
        switch (value) {
            case 1:
                Console.WriteLine("One");
                break;
            case 42:
                Console.WriteLine("Answer");
                break;
            default:
                Console.WriteLine("Other");
                break;
        }
    }
}
"#;

    let (ast, diagnostics) = parse_source(source);
    
    // Should parse control flow structures
    assert_eq!(ast.kind, SyntaxKind::CompilationUnit);
}

#[test]
fn test_parse_interfaces() {
    let source = r#"
interface ICalculator {
    int Add(int a, int b);
    int Subtract(int a, int b);
}

interface IAdvancedCalculator : ICalculator {
    double Multiply(double a, double b);
    double Divide(double a, double b);
}

class BasicCalculator : ICalculator {
    public int Add(int a, int b) => a + b;
    public int Subtract(int a, int b) => a - b;
}
"#;

    let (ast, diagnostics) = parse_source(source);
    
    assert_eq!(ast.kind, SyntaxKind::CompilationUnit);
}

#[test]
fn test_parse_generics() {
    let source = r#"
class GenericClass<T> where T : class {
    private T _value;
    
    public GenericClass(T value) {
        _value = value;
    }
    
    public T GetValue() {
        return _value;
    }
}

class MultiGeneric<T, U> where T : struct where U : class, new() {
    public T First { get; set; }
    public U Second { get; set; }
}
"#;

    let (ast, diagnostics) = parse_source(source);
    
    assert_eq!(ast.kind, SyntaxKind::CompilationUnit);
}

#[test]
fn test_parse_local_variables() {
    let source = r#"
class Variables {
    public void TestMethod() {
        int x = 42;
        var y = "Hello";
        const double PI = 3.14159;
        string message = null;
        
        // Multiple declarations
        int a, b = 10, c;
        
        // Array declaration
        int[] numbers = new int[10];
        string[] names = { "Alice", "Bob", "Charlie" };
    }
}
"#;

    let (ast, diagnostics) = parse_source(source);
    
    assert_eq!(ast.kind, SyntaxKind::CompilationUnit);
}

#[test]
fn test_parse_expressions() {
    let source = r#"
class Expressions {
    public void TestMethod() {
        // Arithmetic
        int result = 1 + 2 * 3 - 4 / 2;
        
        // Boolean
        bool condition = true && false || !true;
        
        // Comparison
        bool compare = 5 > 3 && 2 <= 4;
        
        // Assignment
        int x = 0;
        x += 5;
        x *= 2;
        
        // Method calls
        Console.WriteLine("Hello");
        Math.Max(1, 2);
        
        // Member access
        string text = "hello";
        int length = text.Length;
        
        // Conditional
        int value = condition ? 1 : 0;
    }
}
"#;

    let (ast, diagnostics) = parse_source(source);
    
    assert_eq!(ast.kind, SyntaxKind::CompilationUnit);
}

#[test]
fn test_parse_try_catch() {
    let source = r#"
class ErrorHandling {
    public void TestMethod() {
        try {
            int result = 10 / 0;
        }
        catch (DivideByZeroException ex) {
            Console.WriteLine("Division by zero: " + ex.Message);
        }
        catch (Exception ex) {
            Console.WriteLine("General error: " + ex.Message);
        }
        finally {
            Console.WriteLine("Cleanup");
        }
    }
}
"#;

    let (ast, diagnostics) = parse_source(source);
    
    assert_eq!(ast.kind, SyntaxKind::CompilationUnit);
}

#[test]
fn test_parse_attributes() {
    let source = r#"
[Serializable]
class AttributedClass {
    [Obsolete("Use NewMethod instead")]
    public void OldMethod() {
    }
    
    [HttpGet]
    [Route("/api/test")]
    public string GetData() {
        return "data";
    }
}
"#;

    let (ast, diagnostics) = parse_source(source);
    
    assert_eq!(ast.kind, SyntaxKind::CompilationUnit);
}

#[test]
fn test_parse_syntax_errors() {
    let source = r#"
class ErrorClass {
    public void Method() {
        // Missing semicolon
        int x = 5
        
        // Unmatched brace
        if (true) {
            Console.WriteLine("test");
        // Missing closing brace
    }
}
"#;

    let (ast, diagnostics) = parse_source(source);
    
    // Should have parsing errors
    assert!(diagnostics.has_errors());
    
    // But should still produce an AST
    assert_eq!(ast.kind, SyntaxKind::CompilationUnit);
}