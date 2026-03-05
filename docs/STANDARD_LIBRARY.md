# Standard Library Bindings

This document describes the .NET standard library bindings implemented in the Rust C# compiler.

## Overview

The compiler provides synthetic definitions for core .NET types and methods, allowing C# code to use standard library functionality without requiring actual .NET assemblies. This is achieved through the `AssemblyLoader` module.

## Supported Assemblies

### mscorlib

The core runtime library containing fundamental types:

#### System Namespace

**Core Types:**
- `System.Object` - Base type for all classes
- `System.String` - String type with methods like Substring, Concat
- `System.Console` - Console I/O operations (WriteLine, ReadLine)
- `System.Type` - Runtime type information
- `System.Array` - Base class for arrays
- `System.Delegate` - Base class for delegates
- `System.Enum` - Base class for enumerations
- `System.ValueType` - Base class for value types

**Primitive Types:**
- `System.Boolean` (bool)
- `System.Byte` (byte)
- `System.SByte` (sbyte)
- `System.Int16` (short)
- `System.UInt16` (ushort)
- `System.Int32` (int)
- `System.UInt32` (uint)
- `System.Int64` (long)
- `System.UInt64` (ulong)
- `System.Single` (float)
- `System.Double` (double)
- `System.Decimal` (decimal)
- `System.Char` (char)

**Date/Time Types:**
- `System.DateTime` - Date and time representation
- `System.TimeSpan` - Time interval representation

**Utility Types:**
- `System.Guid` - Globally unique identifier
- `System.Math` - Mathematical functions (Sin, Cos, Sqrt, etc.)
- `System.Convert` - Type conversion utilities
- `System.Environment` - Environment information and control

**Common Delegates:**
- `System.Action` - Delegate with no return value
- `System.Func<T>` - Delegate with return value

### System

Extended functionality beyond core runtime:

#### Exception Types
- `System.Exception` - Base exception class
- `System.ArgumentException`
- `System.ArgumentNullException`
- `System.InvalidOperationException`
- `System.NotImplementedException`
- `System.NullReferenceException`
- `System.IndexOutOfRangeException`

#### Collections (System.Collections.Generic)
- `List<T>` - Dynamic array implementation
- `Dictionary<TKey, TValue>` - Hash table implementation
- `IEnumerable<T>` - Enumerable interface

#### I/O (System.IO)
- `File` - File operations (ReadAllText, WriteAllText, Exists, Delete)
- `Directory` - Directory operations (CreateDirectory, Exists, GetFiles)
- `Path` - Path manipulation utilities (Combine, GetFileName, GetExtension)

#### Text (System.Text)
- `StringBuilder` - Mutable string builder
- `Encoding` - Text encoding/decoding (UTF8 property, GetBytes, GetString)

#### Threading (System.Threading)
- `Thread` - Thread management (Start, Sleep, CurrentThread)
- `Task` (System.Threading.Tasks) - Task-based asynchronous programming

#### LINQ (System.Linq)
- `Enumerable` - LINQ query operators (Where, Select, ToList, ToArray, FirstOrDefault, Count)

## Usage Examples

### Basic Console Application
```csharp
using System;

class Program {
    static void Main() {
        Console.WriteLine("Hello, World!");
        string input = Console.ReadLine();
        Console.WriteLine("You entered: " + input);
    }
}
```

### Working with Collections
```csharp
using System;
using System.Collections.Generic;

class Program {
    static void Main() {
        List<string> names = new List<string>();
        names.Add("Alice");
        names.Add("Bob");
        
        Dictionary<string, int> ages = new Dictionary<string, int>();
        ages.Add("Alice", 30);
        ages.Add("Bob", 25);
        
        Console.WriteLine("Count: " + names.Count);
    }
}
```

### File I/O Operations
```csharp
using System;
using System.IO;

class Program {
    static void Main() {
        string path = "data.txt";
        
        // Write to file
        File.WriteAllText(path, "Hello, File!");
        
        // Read from file
        if (File.Exists(path)) {
            string content = File.ReadAllText(path);
            Console.WriteLine(content);
        }
        
        // Path operations
        string dir = Path.GetDirectoryName(path);
        string ext = Path.GetExtension(path);
    }
}
```

### Using LINQ
```csharp
using System;
using System.Linq;
using System.Collections.Generic;

class Program {
    static void Main() {
        List<int> numbers = new List<int> { 1, 2, 3, 4, 5 };
        
        var evens = numbers.Where(n => n % 2 == 0).ToList();
        var squares = numbers.Select(n => n * n).ToArray();
        
        Console.WriteLine("First even: " + evens.FirstOrDefault());
    }
}
```

### Date and Time
```csharp
using System;

class Program {
    static void Main() {
        DateTime now = DateTime.Now;
        Console.WriteLine("Current date: " + now.ToString("yyyy-MM-dd"));
        
        DateTime tomorrow = now.AddDays(1);
        TimeSpan duration = tomorrow - now;
        
        Console.WriteLine("Hours until tomorrow: " + duration.TotalHours);
    }
}
```

### Math Operations
```csharp
using System;

class Program {
    static void Main() {
        double radius = 5.0;
        double area = Math.PI * Math.Pow(radius, 2);
        
        Console.WriteLine("Circle area: " + area);
        Console.WriteLine("Square root of 16: " + Math.Sqrt(16));
        Console.WriteLine("Sin(PI/2): " + Math.Sin(Math.PI / 2));
    }
}
```

## Implementation Notes

1. **Synthetic Definitions**: All type definitions are created synthetically without parsing actual .NET assemblies. This provides compatibility while keeping the compiler self-contained.

2. **Type Resolution**: Types can be referenced by their full name (e.g., `System.String`) or short name (e.g., `string`) for types in the System namespace.

3. **Generic Types**: Generic types like `List<T>` and `Dictionary<TKey, TValue>` are represented with the CLR naming convention (e.g., `List`1`, `Dictionary`2`).

4. **Member Signatures**: Method and property signatures are stored as strings in a simplified format for type checking.

5. **Assembly Loading**: The `AssemblyLoader` class manages loading and caching of assembly definitions. Core assemblies (mscorlib and System) are loaded automatically.

## Extending the Standard Library

To add new types or members:

1. Edit `src/assembly_loader.rs`
2. Add new type definitions in the appropriate `add_*_types` method
3. Include all necessary members (methods, properties, fields)
4. Update tests to verify the new types are available

Example:
```rust
let new_type = TypeInfo {
    full_name: "System.NewType".to_string(),
    namespace: "System".to_string(),
    name: "NewType".to_string(),
    kind: TypeKind::Class,
    is_public: true,
    base_type: Some("System.Object".to_string()),
    interfaces: Vec::new(),
    members: vec![
        MemberInfo {
            name: "DoSomething".to_string(),
            kind: MemberKind::Method,
            is_public: true,
            is_static: false,
            signature: "() -> System.Void".to_string(),
        },
    ],
};
assembly.types.push(new_type);
```

## Limitations

1. **Partial Implementation**: Not all .NET standard library types and members are implemented. Only the most commonly used ones are included.

2. **Simplified Signatures**: Method signatures don't capture all details like ref/out parameters, params arrays, or generic constraints.

3. **No Runtime Behavior**: These are compile-time definitions only. The actual runtime behavior must be implemented in the code generator.

4. **Version Agnostic**: The definitions represent a general subset of .NET functionality without targeting a specific framework version.