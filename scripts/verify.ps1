$ErrorActionPreference = "Stop"

function Require-Command([string]$Name) {
    if (-not (Get-Command $Name -ErrorAction SilentlyContinue)) {
        throw "Required command '$Name' was not found in PATH."
    }
}

function Assert-LastExitCode([string]$Step) {
    if ($LASTEXITCODE -ne 0) {
        throw "$Step failed with exit code $LASTEXITCODE"
    }
}

Require-Command cargo
Require-Command cmake
Require-Command ctest

$Root = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
Push-Location $Root
try {
    cargo fmt --all --check
    Assert-LastExitCode "cargo fmt"

    cargo clippy --locked --workspace --all-targets -- -D warnings
    Assert-LastExitCode "cargo clippy"

    cargo test --locked --workspace --all-targets
    Assert-LastExitCode "cargo test"

    cargo build --locked --workspace --release
    Assert-LastExitCode "cargo build"

    cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
    Assert-LastExitCode "cmake configure"

    cmake --build build --config Release
    Assert-LastExitCode "cmake build"

    ctest --test-dir build --output-on-failure -C Release
    Assert-LastExitCode "ctest"

    Write-Host "Verification complete: Rust workspace and C++ interoperability tests passed."
}
finally {
    Pop-Location
}
