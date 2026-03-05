#!/bin/bash

echo "C# Compiler in Rust - Demo Script"
echo "================================="
echo

# Build the compiler first
echo "Building the compiler..."
cargo build --release
if [ $? -ne 0 ]; then
    echo "Build failed!"
    exit 1
fi
echo

COMPILER="target/release/csharp_compiler"

# Show version
echo "Compiler version:"
$COMPILER version
echo

# Example 1: Simple Hello World
echo "Example 1: Compiling Hello World"
echo "--------------------------------"
$COMPILER compile examples/hello_world.cs -o hello.exe
echo

# Example 2: Using standard library
echo "Example 2: Standard Library Test"
echo "--------------------------------"
$COMPILER compile examples/test_stdlib.cs -o stdlib_test.exe
echo

# Example 3: Parse-only mode
echo "Example 3: Parse Tree Display"
echo "-----------------------------"
$COMPILER parse examples/hello_world.cs --max-depth 5
echo

# Example 4: Lexer output
echo "Example 4: Token Stream"
echo "-----------------------"
$COMPILER lex examples/hello_world.cs | head -20
echo

# Example 5: Compile with optimizations
echo "Example 5: Release Build"
echo "------------------------"
$COMPILER compile examples/test_stdlib.cs -o stdlib_release.exe -O release
echo

# Example 6: DLL compilation
echo "Example 6: Library Compilation"
echo "------------------------------"
$COMPILER compile examples/generics_test.cs -o generics.dll --target dll
echo

echo "Demo completed!"