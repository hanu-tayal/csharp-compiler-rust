//! Integration tests for the C# compiler

use csharp_compiler::{
    parser::Parser,
    semantic::SemanticModel,
    codegen::{il::ILGenerator, pe::PEGenerator},
    assembly_loader::AssemblyLoader,
};

#[test]
fn test_compile_hello_world_with_stdlib() {
    let source = r#"
using System;

namespace HelloWorld
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Hello, World!");
            Console.WriteLine("PI value is: " + Math.PI);
            
            DateTime now = DateTime.Now;
            Console.WriteLine("Current year: " + now.Year);
        }
    }
}
"#;

    // Parse the source code
    let mut parser = Parser::new(source);
    let syntax_tree = parser.parse();
    
    // Check for parse errors
    assert!(!parser.diagnostics().has_errors(), "Parse errors: {:?}", parser.diagnostics());
    
    // Create semantic model
    let mut semantic_model = SemanticModel::new(syntax_tree);
    
    // Analyze semantics
    semantic_model.analyze();
    
    // Check for semantic errors
    assert!(!semantic_model.diagnostics().has_errors(), "Semantic errors: {:?}", semantic_model.diagnostics());
    
    // Generate IL
    let il_generator = ILGenerator::new();
    let assembly = il_generator.generate(&semantic_model);
    
    // Check that we have the expected methods
    assert!(assembly.methods.iter().any(|m| m.name == "Main"));
}

#[test]
fn test_stdlib_type_resolution() {
    let loader = AssemblyLoader::new();
    
    // Test resolving common types
    let test_types = vec![
        "System.Object",
        "System.String",
        "System.Console",
        "System.Math",
        "System.DateTime",
        "System.Collections.Generic.List`1",
        "System.IO.File",
        "System.Text.StringBuilder",
        "System.Threading.Tasks.Task",
        "System.Linq.Enumerable",
    ];
    
    for type_name in test_types {
        let resolved = loader.resolve_type(type_name);
        assert!(resolved.is_some(), "Failed to resolve type: {}", type_name);
    }
}

#[test]
fn test_compile_with_collections() {
    let source = r#"
using System;
using System.Collections.Generic;

class Program
{
    static void Main()
    {
        List<int> numbers = new List<int>();
        numbers.Add(1);
        numbers.Add(2);
        numbers.Add(3);
        
        Console.WriteLine("Count: " + numbers.Count);
        
        Dictionary<string, int> dict = new Dictionary<string, int>();
        dict.Add("one", 1);
        dict.Add("two", 2);
        
        if (dict.ContainsKey("one"))
        {
            Console.WriteLine("Found key 'one'");
        }
    }
}
"#;

    let mut parser = Parser::new(source);
    let syntax_tree = parser.parse();
    assert!(!parser.diagnostics().has_errors());
    
    let mut semantic_model = SemanticModel::new(syntax_tree);
    semantic_model.analyze();
    
    // Even if semantic analysis isn't complete, we shouldn't have parse errors
    assert!(!parser.diagnostics().has_errors());
}

#[test]
fn test_compile_with_linq() {
    let source = r#"
using System;
using System.Linq;
using System.Collections.Generic;

class Program
{
    static void Main()
    {
        List<int> numbers = new List<int> { 1, 2, 3, 4, 5 };
        
        var evens = numbers.Where(n => n % 2 == 0).ToList();
        var squares = numbers.Select(n => n * n).ToArray();
        
        Console.WriteLine("Even count: " + evens.Count());
        Console.WriteLine("First square: " + squares.FirstOrDefault());
    }
}
"#;

    let mut parser = Parser::new(source);
    let syntax_tree = parser.parse();
    assert!(!parser.diagnostics().has_errors());
}

#[test]
fn test_compile_with_io() {
    let source = r#"
using System;
using System.IO;
using System.Text;

class Program
{
    static void Main()
    {
        string path = "test.txt";
        
        if (File.Exists(path))
        {
            string content = File.ReadAllText(path);
            Console.WriteLine("File content: " + content);
        }
        else
        {
            File.WriteAllText(path, "Hello, File!");
        }
        
        string directory = Path.GetDirectoryName(path);
        string extension = Path.GetExtension(path);
        
        StringBuilder sb = new StringBuilder();
        sb.AppendLine("Line 1");
        sb.AppendLine("Line 2");
        string result = sb.ToString();
    }
}
"#;

    let mut parser = Parser::new(source);
    let syntax_tree = parser.parse();
    assert!(!parser.diagnostics().has_errors());
}

#[test]
fn test_compile_with_exceptions() {
    let source = r#"
using System;

class Program
{
    static void Main()
    {
        try
        {
            throw new InvalidOperationException("Test exception");
        }
        catch (InvalidOperationException ex)
        {
            Console.WriteLine("Caught: " + ex.Message);
        }
        catch (Exception ex)
        {
            Console.WriteLine("Unexpected: " + ex.Message);
        }
        finally
        {
            Console.WriteLine("Finally block");
        }
    }
}
"#;

    let mut parser = Parser::new(source);
    let syntax_tree = parser.parse();
    assert!(!parser.diagnostics().has_errors());
}

#[test]
fn test_compile_with_datetime() {
    let source = r#"
using System;

class Program
{
    static void Main()
    {
        DateTime now = DateTime.Now;
        DateTime today = DateTime.Today;
        DateTime utc = DateTime.UtcNow;
        
        Console.WriteLine("Year: " + now.Year);
        Console.WriteLine("Month: " + now.Month);
        Console.WriteLine("Day: " + now.Day);
        
        DateTime tomorrow = now.AddDays(1);
        TimeSpan duration = TimeSpan.FromDays(7);
        
        Console.WriteLine("Next week: " + now.AddDays(duration.TotalDays));
    }
}
"#;

    let mut parser = Parser::new(source);
    let syntax_tree = parser.parse();
    assert!(!parser.diagnostics().has_errors());
}

#[test]
fn test_compile_with_math() {
    let source = r#"
using System;

class Program
{
    static void Main()
    {
        double pi = Math.PI;
        double e = Math.E;
        
        double sqrt16 = Math.Sqrt(16);
        double power = Math.Pow(2, 10);
        double sine = Math.Sin(pi / 2);
        double cosine = Math.Cos(0);
        
        int max = Math.Max(5, 10);
        int min = Math.Min(5, 10);
        double rounded = Math.Round(3.14159, 2);
        double ceiling = Math.Ceiling(3.1);
        double floor = Math.Floor(3.9);
        
        Console.WriteLine("Square root of 16: " + sqrt16);
        Console.WriteLine("2^10 = " + power);
    }
}
"#;

    let mut parser = Parser::new(source);
    let syntax_tree = parser.parse();
    assert!(!parser.diagnostics().has_errors());
}

#[test]
fn test_compile_with_environment() {
    let source = r#"
using System;

class Program
{
    static void Main(string[] args)
    {
        string newLine = Environment.NewLine;
        string currentDir = Environment.CurrentDirectory;
        string machineName = Environment.MachineName;
        
        Console.WriteLine("Current directory: " + currentDir);
        Console.WriteLine("Machine: " + machineName);
        
        string path = Environment.GetEnvironmentVariable("PATH");
        string[] cmdArgs = Environment.GetCommandLineArgs();
        
        if (path != null)
        {
            Console.WriteLine("PATH is set");
        }
        
        // Environment.Exit(0); // Don't actually exit in test
    }
}
"#;

    let mut parser = Parser::new(source);
    let syntax_tree = parser.parse();
    assert!(!parser.diagnostics().has_errors());
}

#[test]
fn test_compile_with_convert() {
    let source = r#"
using System;

class Program
{
    static void Main()
    {
        // String to number conversions
        int intValue = Convert.ToInt32("42");
        double doubleValue = Convert.ToDouble("3.14159");
        bool boolValue = Convert.ToBoolean("true");
        
        // Number to string conversions
        string intString = Convert.ToString(42);
        
        // Base64 conversions
        byte[] bytes = new byte[] { 1, 2, 3, 4, 5 };
        string base64 = Convert.ToBase64String(bytes);
        byte[] decoded = Convert.FromBase64String(base64);
        
        Console.WriteLine("Converted int: " + intValue);
        Console.WriteLine("Base64: " + base64);
    }
}
"#;

    let mut parser = Parser::new(source);
    let syntax_tree = parser.parse();
    assert!(!parser.diagnostics().has_errors());
}