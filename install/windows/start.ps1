$ErrorActionPreference = 'Stop'

if (-not $env:FLOCK_SIGNAL_API_TOKEN) {
    throw 'Set FLOCK_SIGNAL_API_TOKEN before starting the gateway.'
}
if (-not $env:FLOCK_SIGNAL_BIND) {
    $env:FLOCK_SIGNAL_BIND = '127.0.0.1:8080'
}

$Prefix = Split-Path -Parent $MyInvocation.MyCommand.Path
$Gateway = Join-Path $Prefix 'bin\flock-signal-gateway.exe'
if (-not (Test-Path $Gateway)) { throw "Gateway binary not found at $Gateway" }

& $Gateway
