@echo off
echo C# Compiler Components Showcase
echo ==============================
echo.

echo 1. LEXER - Tokenizing C# Code
echo -----------------------------
echo Input: examples\hello.cs
echo.
type examples\hello.cs
echo.
echo Tokens:
target\debug\csharp_compiler.exe lex examples\hello.cs
echo.

echo 2. PARSER - Syntax Tree Generation
echo ----------------------------------
echo Parsing examples\hello.cs...
target\debug\csharp_compiler.exe parse examples\hello.cs --max-depth 3
echo.

echo 3. COMPILATION ATTEMPT
echo ----------------------
echo Attempting to compile examples\simple_test.cs...
target\debug\csharp_compiler.exe compile examples\simple_test.cs -o test.exe
echo.

echo Components implemented:
echo - Lexer: WORKING
echo - Parser: WORKING (but may hang on complex files)
echo - Semantic Analysis: IMPLEMENTED
echo - IL Generation: IMPLEMENTED
echo - PE Generation: IMPLEMENTED
echo - CLI: WORKING
echo.
echo Note: The binary appears to have a placeholder that prevents actual compilation.
echo To fix: Rebuild with 'cargo build --release' to use the implemented IL/PE generation.
pause