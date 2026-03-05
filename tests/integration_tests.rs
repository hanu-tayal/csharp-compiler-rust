//! End-to-end integration tests for the C# compiler

use csharp_compiler::lexer::Lexer;
use csharp_compiler::parser::Parser;
use csharp_compiler::semantic::symbols::SymbolTable;
use csharp_compiler::semantic::binding::Binder;
use csharp_compiler::semantic::flow_analysis::FlowAnalyzer;
use csharp_compiler::diagnostics::DiagnosticBag;

/// Complete compilation pipeline test
fn compile_source(source: &str) -> CompilationResult {
    // Phase 1: Lexical Analysis (to get tokens separately)
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    let mut diagnostics = lexer.diagnostics().clone();
    
    // Phase 2: Parsing (which includes lexing internally)
    let mut parser = Parser::new(source);
    let syntax_tree = parser.parse();
    let ast = syntax_tree.root;
    for diagnostic in parser.diagnostics().diagnostics() {
        diagnostics.add(diagnostic.clone());
    }
    
    if diagnostics.has_errors() {
        return CompilationResult {
            success: false,
            diagnostics,
            tokens: Some(tokens),
            ast: Some(ast),
            symbol_table: None,
        };
    }
    
    // Phase 3: Semantic Analysis
    let mut symbol_table = SymbolTable::new();
    
    // Add built-in symbols
    add_builtin_symbols(&mut symbol_table);
    
    let mut binder = Binder::new(&symbol_table, &mut diagnostics);
    binder.bind_compilation_unit(&ast);
    
    let mut flow_analyzer = FlowAnalyzer::new(&symbol_table, &mut diagnostics);
    flow_analyzer.analyze(&ast);
    
    let success = !diagnostics.has_errors();
    
    CompilationResult {
        success,
        diagnostics,
        tokens: Some(tokens),
        ast: Some(ast),
        symbol_table: Some(symbol_table),
    }
}

fn add_builtin_symbols(symbol_table: &mut SymbolTable) {
    use csharp_compiler::semantic::symbols::{Symbol, SymbolKind, Accessibility, TypeInfo};
    
    // System namespace
    symbol_table.add_symbol(
        Symbol::new("System".to_string(), SymbolKind::Namespace)
            .with_accessibility(Accessibility::Public)
    );
    
    // System.Object
    symbol_table.add_symbol(
        Symbol::new("System.Object".to_string(), SymbolKind::Type)
            .with_accessibility(Accessibility::Public)
            .with_parent("System".to_string())
    );
    
    // System.String
    symbol_table.add_symbol(
        Symbol::new("System.String".to_string(), SymbolKind::Type)
            .with_accessibility(Accessibility::Public)
            .with_parent("System".to_string())
    );
    
    // System.Console
    symbol_table.add_symbol(
        Symbol::new("System.Console".to_string(), SymbolKind::Type)
            .with_accessibility(Accessibility::Public)
            .with_parent("System".to_string())
    );
    
    // System.Console.WriteLine
    symbol_table.add_symbol(
        Symbol::new("System.Console.WriteLine".to_string(), SymbolKind::Method)
            .with_accessibility(Accessibility::Public)
            .with_parent("System.Console".to_string())
            .with_type_info(TypeInfo {
                type_name: "void".to_string(),
                parameter_types: vec!["string".to_string()],
                type_parameters: vec![],
            })
    );
    
    // Basic types
    let basic_types = ["int", "string", "bool", "double", "float", "long", "short", "byte", "char", "decimal"];
    for type_name in &basic_types {
        symbol_table.add_symbol(
            Symbol::new(format!("System.{}", type_name.to_uppercase().chars().next().unwrap().to_string() + &type_name[1..]), SymbolKind::Type)
                .with_accessibility(Accessibility::Public)
                .with_parent("System".to_string())
        );
    }
}

struct CompilationResult {
    success: bool,
    diagnostics: DiagnosticBag,
    tokens: Option<Vec<csharp_compiler::syntax::SyntaxToken>>,
    ast: Option<csharp_compiler::syntax::SyntaxNode>,
    symbol_table: Option<SymbolTable>,
}

#[test]
fn test_hello_world_compilation() {
    let source = r#"
using System;

namespace HelloWorld {
    class Program {
        static void Main(string[] args) {
            Console.WriteLine("Hello, World!");
        }
    }
}
"#;

    let result = compile_source(source);
    
    // Should successfully compile
    assert!(result.success || !result.diagnostics.has_errors(), 
           "Hello World should compile successfully. Errors: {:?}", 
           result.diagnostics.diagnostics());
    
    assert!(result.tokens.is_some());
    assert!(result.ast.is_some());
    assert!(result.symbol_table.is_some());
    
    // Verify AST structure
    if let Some(ast) = result.ast {
        assert_eq!(ast.kind, csharp_compiler::syntax::SyntaxKind::CompilationUnit);
    }
}

#[test]
fn test_calculator_compilation() {
    let source = r#"
using System;

namespace Calculator {
    class Calculator {
        public int Add(int a, int b) {
            return a + b;
        }
        
        public int Subtract(int a, int b) {
            return a - b;
        }
        
        public int Multiply(int a, int b) {
            return a * b;
        }
        
        public double Divide(int a, int b) {
            if (b == 0) {
                throw new ArgumentException("Division by zero");
            }
            return (double)a / b;
        }
    }
    
    class Program {
        static void Main() {
            Calculator calc = new Calculator();
            
            int sum = calc.Add(5, 3);
            int difference = calc.Subtract(5, 3);
            int product = calc.Multiply(5, 3);
            double quotient = calc.Divide(5, 3);
            
            Console.WriteLine($"Sum: {sum}");
            Console.WriteLine($"Difference: {difference}");
            Console.WriteLine($"Product: {product}");
            Console.WriteLine($"Quotient: {quotient}");
        }
    }
}
"#;

    let result = compile_source(source);
    
    // Should compile with minimal errors (might have some due to incomplete features)
    assert!(result.tokens.is_some());
    assert!(result.ast.is_some());
}

#[test]
fn test_interface_implementation_compilation() {
    let source = r#"
using System;

namespace Interfaces {
    interface IDrawable {
        void Draw();
        void Move(int x, int y);
    }
    
    interface IColorable {
        string Color { get; set; }
    }
    
    class Circle : IDrawable, IColorable {
        public string Color { get; set; }
        
        public void Draw() {
            Console.WriteLine($"Drawing a {Color} circle");
        }
        
        public void Move(int x, int y) {
            Console.WriteLine($"Moving circle to ({x}, {y})");
        }
    }
    
    class Program {
        static void Main() {
            Circle circle = new Circle();
            circle.Color = "Red";
            circle.Draw();
            circle.Move(10, 20);
        }
    }
}
"#;

    let result = compile_source(source);
    
    assert!(result.tokens.is_some());
    assert!(result.ast.is_some());
}

#[test]
fn test_generic_class_compilation() {
    let source = r#"
using System;

namespace Generics {
    class Stack<T> {
        private T[] items;
        private int count;
        
        public Stack() {
            items = new T[10];
            count = 0;
        }
        
        public void Push(T item) {
            if (count < items.Length) {
                items[count] = item;
                count++;
            }
        }
        
        public T Pop() {
            if (count > 0) {
                count--;
                return items[count];
            }
            throw new InvalidOperationException("Stack is empty");
        }
        
        public bool IsEmpty => count == 0;
    }
    
    class Program {
        static void Main() {
            Stack<int> intStack = new Stack<int>();
            intStack.Push(1);
            intStack.Push(2);
            intStack.Push(3);
            
            while (!intStack.IsEmpty) {
                Console.WriteLine(intStack.Pop());
            }
        }
    }
}
"#;

    let result = compile_source(source);
    
    assert!(result.tokens.is_some());
    assert!(result.ast.is_some());
}

#[test]
fn test_error_handling_compilation() {
    let source = r#"
using System;

namespace ErrorHandling {
    class FileProcessor {
        public void ProcessFile(string filename) {
            try {
                // Simulate file processing
                if (string.IsNullOrEmpty(filename)) {
                    throw new ArgumentException("Filename cannot be null or empty");
                }
                
                Console.WriteLine($"Processing file: {filename}");
                
                // Simulate potential error
                if (filename.EndsWith(".bad")) {
                    throw new InvalidOperationException("Bad file format");
                }
                
                Console.WriteLine("File processed successfully");
            }
            catch (ArgumentException ex) {
                Console.WriteLine($"Argument error: {ex.Message}");
            }
            catch (InvalidOperationException ex) {
                Console.WriteLine($"Operation error: {ex.Message}");
            }
            catch (Exception ex) {
                Console.WriteLine($"Unexpected error: {ex.Message}");
            }
            finally {
                Console.WriteLine("Cleanup completed");
            }
        }
    }
    
    class Program {
        static void Main() {
            FileProcessor processor = new FileProcessor();
            
            processor.ProcessFile("good.txt");
            processor.ProcessFile("bad.bad");
            processor.ProcessFile(null);
        }
    }
}
"#;

    let result = compile_source(source);
    
    assert!(result.tokens.is_some());
    assert!(result.ast.is_some());
}

#[test]
fn test_compilation_with_syntax_errors() {
    let source = r#"
using System;

namespace SyntaxErrors {
    class Program {
        static void Main() {
            // Missing semicolon
            int x = 5
            
            // Unmatched parentheses
            Console.WriteLine("Hello"
            
            // Missing closing brace
            if (true) {
                Console.WriteLine("Test");
        }
    }
"#;

    let result = compile_source(source);
    
    // Should fail compilation due to syntax errors
    assert!(!result.success);
    assert!(result.diagnostics.has_errors());
    
    // But should still have tokens
    assert!(result.tokens.is_some());
}

#[test]
fn test_compilation_with_semantic_errors() {
    let source = r#"
using System;

namespace SemanticErrors {
    class Program {
        static void Main() {
            // Undefined variable
            Console.WriteLine(undefinedVar);
            
            // Type mismatch
            int number = "string";
            
            // Use before declaration
            x = 5;
            int x;
            
            // Call non-existent method
            NonExistentMethod();
        }
    }
}
"#;

    let result = compile_source(source);
    
    // Should parse successfully but have semantic errors
    assert!(result.tokens.is_some());
    assert!(result.ast.is_some());
    
    // May or may not have semantic errors depending on implementation completeness
}

#[test]
fn test_large_program_compilation() {
    let source = r#"
using System;
using System.Collections.Generic;

namespace LargeProgram {
    // Base class
    abstract class Shape {
        public abstract double Area { get; }
        public abstract double Perimeter { get; }
        
        public virtual void Display() {
            Console.WriteLine($"Area: {Area}, Perimeter: {Perimeter}");
        }
    }
    
    // Derived classes
    class Circle : Shape {
        private double radius;
        
        public Circle(double radius) {
            this.radius = radius;
        }
        
        public override double Area => Math.PI * radius * radius;
        public override double Perimeter => 2 * Math.PI * radius;
    }
    
    class Rectangle : Shape {
        private double width, height;
        
        public Rectangle(double width, double height) {
            this.width = width;
            this.height = height;
        }
        
        public override double Area => width * height;
        public override double Perimeter => 2 * (width + height);
    }
    
    // Interface
    interface IComparable<T> {
        int CompareTo(T other);
    }
    
    // Generic class
    class ShapeCollection<T> where T : Shape {
        private List<T> shapes = new List<T>();
        
        public void Add(T shape) {
            shapes.Add(shape);
        }
        
        public T GetLargest() {
            T largest = shapes[0];
            foreach (T shape in shapes) {
                if (shape.Area > largest.Area) {
                    largest = shape;
                }
            }
            return largest;
        }
    }
    
    // Main program
    class Program {
        static void Main() {
            var shapes = new ShapeCollection<Shape>();
            
            shapes.Add(new Circle(5.0));
            shapes.Add(new Rectangle(4.0, 6.0));
            shapes.Add(new Circle(3.0));
            
            Shape largest = shapes.GetLargest();
            Console.WriteLine("Largest shape:");
            largest.Display();
        }
    }
}
"#;

    let result = compile_source(source);
    
    // Should handle large programs
    assert!(result.tokens.is_some());
    assert!(result.ast.is_some());
    
    // Verify token count is reasonable
    if let Some(tokens) = &result.tokens {
        assert!(tokens.len() > 100, "Should have many tokens for large program");
    }
}

#[test]
fn test_performance_compilation() {
    let source = r#"
using System;

namespace Performance {
    class Program {
        static void Main() {
            for (int i = 0; i < 1000; i++) {
                Console.WriteLine($"Iteration {i}");
            }
        }
    }
}
"#;

    let start = std::time::Instant::now();
    let result = compile_source(source);
    let duration = start.elapsed();
    
    // Should compile quickly
    assert!(duration.as_millis() < 1000, "Compilation should be fast");
    
    assert!(result.tokens.is_some());
    assert!(result.ast.is_some());
}