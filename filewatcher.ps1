$EnvFile = Join-Path $PSScriptRoot ".env"

if (-not (Test-Path -Path $EnvFile)) {
    Write-Error "Failed not find .env file: $EnvFile"
    exit 1
}

$EnvSettings = @{}
Get-Content $EnvFile | ForEach-Object {
    $line = $_.Trim()
    if ([string]::IsNullOrEmpty($line) -or $line.StartsWith("#")) {
        return
    }

    $index = $line.IndexOf('=')
    if ($index -gt 0) {
        $key = $line.Substring(0, $index).Trim()
        $value = $line.Substring($index + 1).Trim().Trim("'").Trim('"')
        $EnvSettings[$key] = $value
    }
}

$DestinationDir = $EnvSettings["MOD_DIR"]

if ([string]::IsNullOrEmpty($DestinationDir)) {
    Write-Error "Failed not find MOD_DIR var in .env file"
    exit 1
}

$SourceDir = Join-Path $PSScriptRoot "mod"

if (Test-Path -Path $SourceDir) {
    if (-not (Test-Path -Path $DestinationDir)) {
        New-Item -ItemType Directory -Path $DestinationDir -Force | Out-Null
    }
    Copy-Item -Path "$SourceDir\*" -Destination $DestinationDir -Recurse -Force
    Write-Output "Success copy mod folder to: $DestinationDir"
}
else {
    Write-Warning "MOd $SourceDir not exist"
}
