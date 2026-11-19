param([Parameter(Mandatory = $true)][string]$Destination, [string[]]$Only)

$ErrorActionPreference = 'Stop'
$manifest = Get-Content -Raw -LiteralPath (Join-Path $PSScriptRoot 'engines.windows-x64.json') | ConvertFrom-Json
New-Item -ItemType Directory -Force -Path $Destination | Out-Null
foreach ($entry in $manifest.PSObject.Properties) {
    if ($Only -and $entry.Name -notin $Only) { continue }
    $asset = $entry.Value
    $path = Join-Path $Destination $asset.file
    if ((Test-Path -LiteralPath $path) -and (Get-FileHash -LiteralPath $path).Hash -eq $asset.sha256) { continue }
    Write-Host "Downloading $($entry.Name) $($asset.version)..."
    $partial = "$path.partial"
    try {
        & curl.exe --fail --location --silent --show-error --retry 2 --connect-timeout 30 --max-time 180 --output $partial $asset.url
        if ($LASTEXITCODE -ne 0) { throw "Download failed: $($asset.url)" }
        if ((Get-FileHash -LiteralPath $partial).Hash -ne $asset.sha256) { throw "SHA-256 mismatch: $($asset.file)" }
        Move-Item -Force -LiteralPath $partial -Destination $path
    } finally {
        if (Test-Path -LiteralPath $partial) { Remove-Item -LiteralPath $partial }
    }
}
