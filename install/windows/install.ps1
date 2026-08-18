$ErrorActionPreference = 'Stop'

$Root = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
$Prefix = if ($env:FLOCK_SIGNAL_PREFIX) { $env:FLOCK_SIGNAL_PREFIX } else { Join-Path $env:LOCALAPPDATA 'FlockSignal' }
$Bin = Join-Path $Prefix 'bin'
$Include = Join-Path $Prefix 'include'

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) { throw 'cargo is required' }
if (-not (Get-Command cmake -ErrorAction SilentlyContinue)) { throw 'cmake is required' }

Push-Location $Root
try {
    cargo build --release --workspace
    cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
    cmake --build build --config Release --parallel 2
    ctest --test-dir build -C Release --output-on-failure

    New-Item -ItemType Directory -Force -Path $Bin | Out-Null
    New-Item -ItemType Directory -Force -Path $Include | Out-Null
    Copy-Item (Join-Path $Root 'target\release\flock-signal-gateway.exe') (Join-Path $Bin 'flock-signal-gateway.exe') -Force
    if (Test-Path (Join-Path $Root 'target\release\flock_signal_ffi.dll')) {
        Copy-Item (Join-Path $Root 'target\release\flock_signal_ffi.dll') $Bin -Force
    }
    Copy-Item (Join-Path $Root 'cpp\include\flock_signal') $Include -Recurse -Force
    Copy-Item (Join-Path $Root 'install\windows\start.ps1') (Join-Path $Prefix 'start.ps1') -Force

    Write-Host "Installed Flock Signal Safety to $Prefix"
    Write-Host 'Set FLOCK_SIGNAL_API_TOKEN to a strong secret, then run:'
    Write-Host "  & '$Prefix\start.ps1'"
}
finally {
    Pop-Location
}
