Write-Host "Rebuilding C# Compiler..." -ForegroundColor Green
Write-Host "========================" -ForegroundColor Green
Write-Host ""

Write-Host "Cleaning previous build..." -ForegroundColor Yellow
cargo clean

Write-Host ""
Write-Host "Building in release mode..." -ForegroundColor Yellow
cargo build --release

if ($LASTEXITCODE -ne 0) {
    Write-Host ""
    Write-Host "Build failed! Trying debug mode..." -ForegroundColor Red
    cargo build
    if ($LASTEXITCODE -ne 0) {
        Write-Host ""
        Write-Host "Debug build also failed!" -ForegroundColor Red
        Write-Host "Please check the error messages above." -ForegroundColor Red
        Read-Host "Press Enter to continue..."
        exit 1
    }
    Write-Host ""
    Write-Host "Debug build succeeded!" -ForegroundColor Green
    $compiler = "target\debug\csharp_compiler.exe"
} else {
    Write-Host ""
    Write-Host "Release build succeeded!" -ForegroundColor Green
    $compiler = "target\release\csharp_compiler.exe"
}

Write-Host ""
Write-Host "Testing the rebuilt compiler..." -ForegroundColor Cyan
Write-Host "-------------------------------" -ForegroundColor Cyan

Write-Host ""
Write-Host "1. Testing lexer with hello.cs:" -ForegroundColor Yellow
& $compiler lex examples\hello.cs | Select-Object -First 10

Write-Host ""
Write-Host "2. Testing parser with hello.cs:" -ForegroundColor Yellow
& $compiler parse examples\hello.cs --max-depth 2

Write-Host ""
Write-Host "3. Testing compilation with simple_test.cs:" -ForegroundColor Yellow
& $compiler compile examples\simple_test.cs -o simple_test.exe

Write-Host ""
Write-Host "4. Checking if output was generated:" -ForegroundColor Yellow
if (Test-Path simple_test.exe) {
    Write-Host "SUCCESS: simple_test.exe was generated!" -ForegroundColor Green
    Get-ChildItem simple_test.exe
} else {
    Write-Host "WARNING: No output file was generated." -ForegroundColor Red
}

Write-Host ""
Write-Host "Rebuild complete!" -ForegroundColor Green
Read-Host "Press Enter to continue..."