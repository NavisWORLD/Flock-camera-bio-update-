param(
    [string]$InstallDir = "$env:ProgramFiles\FlockSignal",
    [switch]$RegisterService
)

$ErrorActionPreference = "Stop"

function Require-Command([string]$Name) {
    if (-not (Get-Command $Name -ErrorAction SilentlyContinue)) {
        throw "Required command '$Name' was not found in PATH."
    }
}

Require-Command cargo
Require-Command cmake

$RepoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..\..")).Path
Push-Location $RepoRoot
try {
    cargo build -p flock-signal-gateway --release
    cargo build -p signal-ffi --release
    cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
    cmake --build build --config Release

    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
    New-Item -ItemType Directory -Force -Path (Join-Path $InstallDir "bin") | Out-Null
    New-Item -ItemType Directory -Force -Path (Join-Path $InstallDir "include\flock_signal") | Out-Null

    Copy-Item "target\release\flock-signal-gateway.exe" (Join-Path $InstallDir "bin") -Force
    if (Test-Path "target\release\signal_ffi.dll") {
        Copy-Item "target\release\signal_ffi.dll" (Join-Path $InstallDir "bin") -Force
    }
    Copy-Item "cpp\include\flock_signal\flock_signal.h" (Join-Path $InstallDir "include\flock_signal") -Force
    Copy-Item "cpp\include\flock_signal\flock_signal.hpp" (Join-Path $InstallDir "include\flock_signal") -Force

    $EnvFile = Join-Path $InstallDir "gateway.env.example"
    @"
FLOCK_SIGNAL_BIND=0.0.0.0:8080
FLOCK_SIGNAL_API_KEY=REPLACE_WITH_SECRET_MANAGER_VALUE
FLOCK_SIGNAL_DATABASE_URL=postgresql://USER:PASSWORD@HOST:5432/flock_signal
"@ | Set-Content -Path $EnvFile -Encoding UTF8

    if ($RegisterService) {
        $principal = New-Object Security.Principal.WindowsPrincipal([Security.Principal.WindowsIdentity]::GetCurrent())
        if (-not $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
            throw "-RegisterService requires an elevated PowerShell session."
        }
        $Exe = Join-Path $InstallDir "bin\flock-signal-gateway.exe"
        if (Get-Service -Name "FlockSignalGateway" -ErrorAction SilentlyContinue) {
            sc.exe stop FlockSignalGateway | Out-Null
            sc.exe delete FlockSignalGateway | Out-Null
        }
        sc.exe create FlockSignalGateway binPath= "`"$Exe`"" start= auto DisplayName= "Flock Signal Safety Gateway" | Out-Null
        sc.exe description FlockSignalGateway "Anonymous signal correlation and forensic evidence gateway" | Out-Null
    }

    Write-Host "Installed Flock Signal platform to $InstallDir"
    Write-Host "Set FLOCK_SIGNAL_API_KEY and database secrets outside source control before starting the service."
}
finally {
    Pop-Location
}
