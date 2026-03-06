# Rust C# Compiler

A C# compiler implementation written in Rust, inspired by the Roslyn compiler architecture. This project demonstrates compiler construction techniques while leveraging Rust's memory safety and performance characteristics.

## Overview

This compiler implements a complete pipeline from C# source code to .NET IL bytecode, including:
- Full lexical analysis with all C# token types
- Recursive descent parser producing lossless syntax trees
- Semantic analysis with type checking and flow analysis
- IL code generation and PE file creation
- Built-in .NET standard library support

## Quick Start

```bash
# Build the compiler
cargo build --release

# Compile a C# file
./target/release/csharp_compiler compile examples/HelloWorld.cs -o HelloWorld.exe

# Run tests
cargo test

# View syntax tree (for debugging)
./target/release/csharp_compiler parse examples/HelloWorld.cs --format tree
```

## Features

### Language Support
- ✅ Classes, structs, interfaces, enums
- ✅ Methods, properties, fields, events
- ✅ Generics with constraints
- ✅ LINQ query expressions
- ✅ Pattern matching
- ✅ Async/await
- ✅ String interpolation
- ✅ All C# operators and expressions
- ✅ Exception handling (try/catch/finally)
- ✅ Iterators (foreach)

### Compiler Features
- Complete lexical analysis using `logos`
- Lossless syntax trees with `rowan`
- Multi-phase semantic analysis
- Direct IL bytecode generation
- PE/COFF file generation
- Built-in .NET type definitions
- Beautiful error messages with `miette`

### Standard Library Support
The compiler includes synthetic definitions for core .NET types:
- System namespace (Object, String, Int32, etc.)
- Collections (List<T>, Dictionary<K,V>, IEnumerable<T>)
- I/O operations (File, Directory, Console)
- LINQ operations
- DateTime and Math utilities

## Architecture

```
src/
├── lexer/          # Tokenization with logos
├── parser/         # Recursive descent parser
├── syntax/         # AST definitions (rowan-based)
├── semantic/       # Type checking and analysis
├── codegen/        # IL generation and PE creation
├── assembly_loader/# .NET standard library definitions
└── diagnostics/    # Error reporting
```

## Usage Examples

### Basic Compilation
```bash
# Compile to executable
csharp_compiler compile Program.cs -o Program.exe

# Compile to library
csharp_compiler compile Library.cs -o Library.dll --target dll

# With optimizations
csharp_compiler compile Program.cs -o Program.exe -O release
```

### Analysis Tools
```bash
# Tokenize source code
csharp_compiler lex Program.cs

# Parse and show syntax tree
csharp_compiler parse Program.cs

# Semantic analysis only
csharp_compiler analyze Program.cs
```

## Example Code

```csharp
using System;
using System.Collections.Generic;
using System.Linq;

public class Example
{
    public static void Main(string[] args)
    {
        var numbers = new List<int> { 1, 2, 3, 4, 5 };
        
        var evenNumbers = numbers
            .Where(n => n % 2 == 0)
            .Select(n => n * n);
        
        foreach (var num in evenNumbers)
        {
            Console.WriteLine($"Square of even: {num}");
        }
    }
}
```

## Building from Source

### Prerequisites
- Rust 1.70 or newer
- Cargo

### Build Steps
```bash
# Clone the repository
git clone https://github.com/hanu-tayal/rust-csharp-compiler
cd rust-csharp-compiler

# Build in release mode
cargo build --release

# Run tests
cargo test

# Run a specific test suite
cargo test semantic_tests
```

## Development

### Project Structure
- `lexer/` - Tokenization using logos crate
- `parser/` - Hand-written recursive descent parser
- `semantic/` - Type system and semantic analysis
- `codegen/` - IL bytecode and PE file generation
- `tests/` - Comprehensive test suite

### Running Examples
```bash
# Windows
showcase_components.bat

# Unix-like systems
./examples/demo_compilation.sh
```

## Design Philosophy

1. **Correctness First**: Accurate implementation of C# semantics
2. **Developer Experience**: Clear error messages and diagnostics
3. **Performance**: Leverage Rust's zero-cost abstractions
4. **Modularity**: Clear separation between compiler phases
5. **Testability**: Comprehensive test coverage

## Current Limitations

- Subset of .NET standard library implemented
- No unsafe code support yet
- Limited attribute support
- No source generators
- Single-threaded compilation

## Future Roadmap

- [ ] Language Server Protocol (LSP) support
- [ ] Incremental compilation
- [ ] More .NET APIs
- [ ] Source generators
- [ ] Parallel compilation
- [ ] REPL mode

## Contributing

Contributions are welcome! This is an educational project aimed at understanding compiler construction. Please feel free to:
- Report bugs
- Suggest features
- Submit pull requests
- Improve documentation

## References

- [C# Language Specification](https://docs.microsoft.com/en-us/dotnet/csharp/language-reference/language-specification/)
- [ECMA-335 CLI Specification](https://www.ecma-international.org/publications-and-standards/standards/ecma-335/)
- [Roslyn Compiler](https://github.com/dotnet/roslyn)
- [.NET IL Reference](https://docs.microsoft.com/en-us/dotnet/api/system.reflection.emit.opcodes)

## License

MIT License - see LICENSE file for details

## Acknowledgments

- Microsoft Roslyn team for the compiler architecture inspiration
- Rust compiler team for excellent tooling
- logos, rowan, and miette crate authors