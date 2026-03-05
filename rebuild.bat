@echo off
echo Rebuilding C# Compiler...
echo ========================
echo.

echo Cleaning previous build...
cargo clean

echo.
echo Building in release mode...
cargo build --release

if %errorlevel% neq 0 (
    echo.
    echo Build failed! Trying debug mode...
    cargo build
    if %errorlevel% neq 0 (
        echo.
        echo Debug build also failed!
        echo Please check the error messages above.
        pause
        exit /b 1
    )
    echo.
    echo Debug build succeeded!
    set COMPILER=target\debug\csharp_compiler.exe
) else (
    echo.
    echo Release build succeeded!
    set COMPILER=target\release\csharp_compiler.exe
)

echo.
echo Testing the rebuilt compiler...
echo -------------------------------

echo.
echo 1. Testing lexer with hello.cs:
%COMPILER% lex examples\hello.cs | head -10

echo.
echo 2. Testing parser with hello.cs:
%COMPILER% parse examples\hello.cs --max-depth 2

echo.
echo 3. Testing compilation with simple_test.cs:
%COMPILER% compile examples\simple_test.cs -o simple_test.exe

echo.
echo 4. Checking if output was generated:
if exist simple_test.exe (
    echo SUCCESS: simple_test.exe was generated!
    dir simple_test.exe
) else (
    echo WARNING: No output file was generated.
)

echo.
echo Rebuild complete!
pause