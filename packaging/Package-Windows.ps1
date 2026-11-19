$ErrorActionPreference = 'Stop'

if (-not [Environment]::Is64BitOperatingSystem) { throw 'Windows x64 is required.' }
$target = 'x86_64-pc-windows-gnu'
$repo = Split-Path -Parent $PSScriptRoot
$build = Join-Path $repo 'target'
$cache = Join-Path $build 'engine-archives'
$dist = Join-Path $repo 'dist'
$tar = Join-Path $env:SystemRoot 'System32/tar.exe'
$manifestName = 'engines.windows-x64.json'
$manifestPath = Join-Path $PSScriptRoot $manifestName
$manifest = Get-Content -LiteralPath $manifestPath -Raw | ConvertFrom-Json

function Invoke-Checked($program, $arguments) {
    & $program @arguments
    if ($LASTEXITCODE -ne 0) { throw "$program failed (exit $LASTEXITCODE)" }
}

function Expand-Asset($asset, $destination) {
    New-Item -ItemType Directory -Path $destination | Out-Null
    Invoke-Checked $tar @('-xf', (Join-Path $cache $asset.file), '-C', $destination)
}

Push-Location -LiteralPath $repo
try {
    & (Join-Path $PSScriptRoot 'Get-EngineArchives.ps1') -Destination $cache
    $metadata = (Invoke-Checked cargo @('metadata', '--no-deps', '--locked', '--format-version', '1')) | ConvertFrom-Json
    $version = ($metadata.packages | Where-Object name -eq 'stylebrella').version
    $name = "stylebrella-$version-windows-x64"
    Invoke-Checked cargo @('build', '--release', '--locked', '-p', 'stylebrella', '--target', $target, '--target-dir', $build)

    $scratch = Join-Path $build ("package-" + [guid]::NewGuid())
    $stage = Join-Path $scratch $name
    $engines = Join-Path $stage 'engines'
    $llvm = Join-Path $engines 'llvm'
    $kotlin = Join-Path $engines 'kotlin'
    $licenses = Join-Path $stage 'licenses'
    $zip = Join-Path $scratch "$name.zip"

    try {
        foreach ($dir in @((Join-Path $llvm 'bin'), $kotlin, $licenses, $dist)) {
            New-Item -ItemType Directory -Force -Path $dir | Out-Null
        }
        Copy-Item -LiteralPath (Join-Path $build "$target/release/stylebrella.exe") -Destination $stage
        Copy-Item -LiteralPath $manifestPath -Destination $stage
        Copy-Item -LiteralPath (Join-Path $repo 'LICENSE') -Destination (Join-Path $licenses 'stylebrella.txt')
        Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'THIRD-PARTY.md') -Destination $stage
        foreach ($asset in @($manifest.'ruff-license', $manifest.'ktlint-license')) {
            Copy-Item -LiteralPath (Join-Path $cache $asset.file) -Destination $licenses
        }

        # Strip only the outer wheel structure. Keep the native binaries and Clang resource headers.
        foreach ($tool in @('clang-format', 'clang-tidy')) {
            $asset = $manifest.$tool
            $wheelName = $tool.Replace('-', '_')
            $wheel = Join-Path $scratch $wheelName
            Expand-Asset $asset $wheel
            Copy-Item -LiteralPath (Join-Path $wheel "$wheelName/data/bin/$tool.exe") -Destination (Join-Path $llvm 'bin')
            Copy-Item -LiteralPath (Join-Path $wheel "$wheelName-$($asset.version).dist-info/licenses") -Recurse -Destination (Join-Path $licenses $tool)
            if ($tool -eq 'clang-tidy') {
                Copy-Item -LiteralPath (Join-Path $wheel "$wheelName/data/lib") -Recurse -Destination $llvm
            }
        }

        $ruff = Join-Path $engines 'ruff'
        $ruffExtract = Join-Path $scratch 'ruff'
        Expand-Asset $manifest.ruff $ruffExtract
        $ruffExe = Get-ChildItem -LiteralPath $ruffExtract -Filter 'ruff.exe' -Recurse -File
        if (@($ruffExe).Count -ne 1) { throw 'Expected one ruff.exe in the Ruff archive.' }
        Copy-Item -LiteralPath $ruffExe.DirectoryName -Recurse -Destination $ruff

        $javaExtract = Join-Path $scratch 'java'
        Expand-Asset $manifest.java $javaExtract
        $javaHome = @(Get-ChildItem -LiteralPath $javaExtract -Directory)
        if ($javaHome.Count -ne 1 -or -not (Test-Path -LiteralPath (Join-Path $javaHome[0].FullName 'bin/java.exe'))) {
            throw 'Unexpected layout in the Java runtime archive.'
        }
        Move-Item -LiteralPath $javaHome[0].FullName -Destination (Join-Path $kotlin 'java')
        Copy-Item -LiteralPath (Join-Path $cache $manifest.ktlint.file) -Destination (Join-Path $kotlin 'ktlint.jar')

        Invoke-Checked $tar @('-a', '-cf', $zip, '-C', $scratch, $name)

        # Exercise the actual archive from a separate working directory and with no tools on PATH.
        $verification = Join-Path $scratch 'smoke test'
        New-Item -ItemType Directory -Path $verification | Out-Null
        Invoke-Checked $tar @('-xf', $zip, '-C', $verification)
        $installation = Join-Path $verification $name
        $oldPath = $env:PATH
        Push-Location -LiteralPath $verification
        try {
            $env:PATH = ''
            Invoke-Checked (Join-Path $installation 'stylebrella.exe') @('--version', '--engines')
        } finally {
            $env:PATH = $oldPath
            Pop-Location
        }
        $oldInstall = $env:STYLEBRELLA_TEST_INSTALL
        try {
            $env:STYLEBRELLA_TEST_INSTALL = $installation
            Invoke-Checked cargo @('test', '--locked', '-p', 'stylebrella-core', '--target', $target,
                '--target-dir', $build, '--test', 'bundled_engines', '--', '--ignored')
        } finally {
            $env:STYLEBRELLA_TEST_INSTALL = $oldInstall
        }

        $output = Join-Path $dist "$name.zip"
        Move-Item -Force -LiteralPath $zip -Destination $output
        Write-Host "ZIP: $output"
        Write-Host "SHA-256: $((Get-FileHash -LiteralPath $output -Algorithm SHA256).Hash.ToLowerInvariant())"
    } finally {
        if (Test-Path -LiteralPath $scratch) { Remove-Item -LiteralPath $scratch -Recurse -Force }
    }
} finally {
    Pop-Location
}
