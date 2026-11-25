<# : batch
@echo off
title WuWa Private Server Launcher
set "LAUNCH_BAT_DIR=%~dp0"
set "LAUNCH_BAT_FILE=%~f0"
powershell -NoProfile -ExecutionPolicy Bypass -Command "$raw = [System.IO.File]::ReadAllText($env:LAUNCH_BAT_FILE, [System.Text.Encoding]::UTF8); $sep = '#' + '__PS__'; $parts = $raw -split [regex]::Escape($sep), 2; if ($parts.Length -lt 2) { exit 1 }; Invoke-Expression $parts[1]"
exit /b %errorlevel%
#>
#__PS__
$ErrorActionPreference = "Stop"
$ProgressPreference = "SilentlyContinue"

$DefaultRemoteIp = "43.167.4.171"
$localAppBase = if (-not [string]::IsNullOrWhiteSpace($env:LOCALAPPDATA)) { $env:LOCALAPPDATA } elseif (-not [string]::IsNullOrWhiteSpace($env:USERPROFILE)) { Join-Path $env:USERPROFILE "AppData\Local" } else { [System.IO.Path]::GetTempPath() }
$StateDir = Join-Path $localAppBase "WuwaPrivateServer"
$ConfigFile = Join-Path $StateDir "config.json"

function Load-Config {
    if (Test-Path -LiteralPath $ConfigFile) {
        try { return Get-Content -Raw -LiteralPath $ConfigFile | ConvertFrom-Json } catch {}
    }
    return [PSCustomObject]@{ GameExePath = ""; PublicIp = $DefaultRemoteIp; ServerRoot = "" }
}

function Save-Config {
    param([string]$ExePath, [string]$Ip, [string]$ServerRoot)
    if (-not (Test-Path -LiteralPath $StateDir)) {
        New-Item -ItemType Directory -Path $StateDir -Force | Out-Null
    }
    $json = ([PSCustomObject]@{ GameExePath = $ExePath; PublicIp = $Ip; ServerRoot = $ServerRoot }) | ConvertTo-Json
    [System.IO.File]::WriteAllText($ConfigFile, $json, [System.Text.UTF8Encoding]::new($false))
}

function Test-ServerAlive {
    param([string]$Ip)
    Write-Host "Checking $Ip..."
    foreach ($port in @(8080, 5500)) {
        $client = [System.Net.Sockets.TcpClient]::new()
        try {
            $iar = $client.BeginConnect($Ip, $port, $null, $null)
            if (-not $iar.AsyncWaitHandle.WaitOne(3000, $false)) {
                Write-Host "Timeout: ${Ip}:${port}"
                return $false
            }
            $client.EndConnect($iar)
        } catch {
            Write-Host "Unreachable: ${Ip}:${port}"
            return $false
        } finally {
            $client.Dispose()
        }
    }

    try {
        $health = Invoke-WebRequest -Uri "http://${Ip}:5500/health" -TimeoutSec 3 -UseBasicParsing
        if ($health.StatusCode -ne 200) {
            Write-Host "Health check failed ($($health.StatusCode)): ${Ip}:5500"
            return $false
        }
    } catch {
        Write-Host "Health check unreachable: ${Ip}:5500"
        return $false
    }

    try {
        $logicResp = Invoke-WebRequest -Uri "http://${Ip}:5500/aki-logic/status" -TimeoutSec 3 -UseBasicParsing
        $logicJson = $logicResp.Content | ConvertFrom-Json
        if ($logicJson.online -eq $false) {
            Write-Host "Game server offline on $Ip"
            return $false
        }
    } catch {
        Write-Host "Game server unreachable on $Ip"
        return $false
    }

    Write-Host "Server online: $Ip"
    return $true
}

function Resolve-ExePath {
    param([string]$BaseDir)
    if ([string]::IsNullOrWhiteSpace($BaseDir)) { return $null }
    $clean = $BaseDir.Trim().Trim('"')
    if ($clean -match 'Client-Win64-Shipping\.exe$' -and (Test-Path -LiteralPath $clean)) { return $clean }
    foreach ($sub in @(
        "Client\Binaries\Win64\Client-Win64-Shipping.exe",
        "Wuthering Waves Game\Client\Binaries\Win64\Client-Win64-Shipping.exe",
        "Wuthering Waves\Wuthering Waves Game\Client\Binaries\Win64\Client-Win64-Shipping.exe"
    )) {
        $p = Join-Path $clean $sub
        if (Test-Path -LiteralPath $p) { return $p }
    }
    return $null
}

function Find-GameExe {
    param([string]$SavedExe)
    if (-not [string]::IsNullOrWhiteSpace($SavedExe) -and (Test-Path -LiteralPath $SavedExe)) { return $SavedExe }

    if (-not [string]::IsNullOrWhiteSpace($env:APPDATA)) {
        $krDir = Join-Path $env:APPDATA "KRLauncher"
        if (Test-Path -LiteralPath $krDir) {
            foreach ($f in (Get-ChildItem -Path $krDir -Filter "*.json" -Recurse -ErrorAction SilentlyContinue)) {
                try {
                    $txt = Get-Content -Raw -LiteralPath $f.FullName
                    foreach ($m in [regex]::Matches($txt, '[A-Za-z]:[/\\]{1,2}[^"\r\n<>|?*]+')) {
                        $hit = Resolve-ExePath ($m.Value -replace '\\\\', '\' -replace '/', '\')
                        if ($hit) { return $hit }
                    }
                } catch {}
            }
        }
    }

    if (Test-Path -LiteralPath "HKCU:\") {
        foreach ($rr in @(
            "HKCU:\Software\Kuro*",
            "HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\*",
            "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\*",
            "HKLM:\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\*"
        )) {
            foreach ($item in (Get-ItemProperty -Path $rr -ErrorAction SilentlyContinue)) {
                if ($item.DisplayName -match "Wuthering|MingChao" -or $item.PSChildName -match "KRLauncher|Wuthering") {
                    foreach ($prop in @($item.InstallLocation, $item.InstallPath, $item.UninstallString)) {
                        $hit = Resolve-ExePath ($prop -replace ',0$', '')
                        if ($hit) { return $hit }
                    }
                }
            }
        }

        $steam = (Get-ItemProperty -Path "HKCU:\Software\Valve\Steam" -ErrorAction SilentlyContinue).SteamPath
        if (-not $steam) { $steam = (Get-ItemProperty -Path "HKLM:\SOFTWARE\WOW6432Node\Valve\Steam" -ErrorAction SilentlyContinue).InstallPath }
        if ($steam) {
            $libs = @($steam)
            $vdf = Join-Path $steam "steamapps\libraryfolders.vdf"
            if (Test-Path -LiteralPath $vdf) {
                $vdfText = Get-Content -Raw -LiteralPath $vdf -ErrorAction SilentlyContinue
                foreach ($m in [regex]::Matches($vdfText, '"path"\s+"([^"]+)"')) {
                    $libs += ($m.Groups[1].Value -replace '\\\\', '\')
                }
            }
            foreach ($lib in $libs) {
                $hit = Resolve-ExePath (Join-Path $lib "steamapps\common\Wuthering Waves")
                if ($hit) { return $hit }
            }
        }
    }

    if (-not [string]::IsNullOrWhiteSpace($env:ProgramData)) {
        $epicDir = Join-Path $env:ProgramData "Epic\EpicGamesLauncher\Data\Manifests"
        if (Test-Path -LiteralPath $epicDir) {
            foreach ($f in (Get-ChildItem -Path $epicDir -Filter "*.item" -ErrorAction SilentlyContinue)) {
                try {
                    $item = Get-Content -Raw -LiteralPath $f.FullName | ConvertFrom-Json
                    if ($item.DisplayName -match "Wuthering" -or $item.InstallLocation -match "Wuthering") {
                        $hit = Resolve-ExePath $item.InstallLocation
                        if ($hit) { return $hit }
                    }
                } catch {}
            }
        }
    }

    $drives = Get-PSDrive -PSProvider FileSystem | Where-Object { $_.Root -match '^[A-Za-z]:\\$' }
    foreach ($d in $drives) {
        $root = $d.Root
        foreach ($sub in @(
            "Wuthering Waves", "Wuthering Waves Game", "Games\Wuthering Waves", "Games\Wuthering Waves Game",
            "Program Files\Wuthering Waves", "Program Files (x86)\Wuthering Waves",
            "Program Files\Epic Games\WutheringWaves", "Program Files (x86)\Steam\steamapps\common\Wuthering Waves"
        )) {
            $hit = Resolve-ExePath (Join-Path $root $sub)
            if ($hit) { return $hit }
        }
        foreach ($dir in (Get-ChildItem -Path $root -Directory -ErrorAction SilentlyContinue)) {
            $hit = Resolve-ExePath $dir.FullName
            if ($hit) { return $hit }
        }
    }
    return $null
}

function Set-IniKey {
    param([System.Collections.Generic.List[string]]$Lines, [string]$Section, [string]$Key, [string]$Value)
    $header = "[$Section]"
    $secIdx = -1
    for ($i = 0; $i -lt $Lines.Count; $i++) {
        if ($Lines[$i].Trim() -ieq $header) { $secIdx = $i; break }
    }
    if ($secIdx -lt 0) {
        if ($Lines.Count -gt 0 -and $Lines[$Lines.Count - 1].Trim() -ne "") { $Lines.Add("") }
        $Lines.Add($header)
        $Lines.Add("$Key=$Value")
        Write-Host "[$Section] $Key=$Value (added)"
        return $true
    }
    $endIdx = $Lines.Count
    for ($i = $secIdx + 1; $i -lt $Lines.Count; $i++) {
        $t = $Lines[$i].Trim()
        if ($t.StartsWith("[") -and $t.EndsWith("]")) { $endIdx = $i; break }
        if ($t -match ("^" + [regex]::Escape($Key) + "\s*=\s*(.*)$")) {
            $cur = $Matches[1].Trim()
            if ($cur -ieq $Value) {
                Write-Host "[$Section] $Key=$cur (ok)"
                return $false
            }
            $Lines[$i] = "$Key=$Value"
            Write-Host "[$Section] $Key=$cur -> $Value"
            return $true
        }
    }
    $insertIdx = $endIdx
    while ($insertIdx -gt ($secIdx + 1) -and [string]::IsNullOrWhiteSpace($Lines[$insertIdx - 1])) {
        $insertIdx--
    }
    $Lines.Insert($insertIdx, "$Key=$Value")
    Write-Host "[$Section] $Key=$Value (added)"
    return $true
}

function Patch-EngineIni {
    param([string]$IniPath)
    $dir = Split-Path -Path $IniPath -Parent
    if (-not (Test-Path -LiteralPath $dir)) { New-Item -ItemType Directory -Path $dir -Force | Out-Null }

    $lines = [System.Collections.Generic.List[string]]::new()
    if (Test-Path -LiteralPath $IniPath) {
        Write-Host "Engine.ini: $IniPath"
        foreach ($ln in (Get-Content -LiteralPath $IniPath -Encoding UTF8)) { $lines.Add($ln) }
    } else {
        Write-Host "Engine.ini (new): $IniPath"
    }

    $c1 = Set-IniKey -Lines $lines -Section "/Script/Engine.NetworkSettings" -Key "bVerifyPeer" -Value "False"
    $c2 = Set-IniKey -Lines $lines -Section "/Script/Engine.NetworkSettings" -Key "n.VerifyPeer" -Value "False"
    $c3 = Set-IniKey -Lines $lines -Section "HTTP" -Key "bVerifyPeer" -Value "False"

    if ($c1 -or $c2 -or $c3 -or -not (Test-Path -LiteralPath $IniPath)) {
        if ((Test-Path -LiteralPath $IniPath) -and -not (Test-Path -LiteralPath "$IniPath.bak")) {
            Copy-Item -LiteralPath $IniPath -Destination "$IniPath.bak" -Force
        }
        [System.IO.File]::WriteAllLines($IniPath, $lines, [System.Text.UTF8Encoding]::new($false))
    }
}

function Refresh-ProcessPath {
    $machinePath = [Environment]::GetEnvironmentVariable("Path", "Machine")
    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    $cargoBin = Join-Path $env:USERPROFILE ".cargo\bin"
    $env:Path = "$cargoBin;$userPath;$machinePath;$env:Path"
}

function Find-ServerRoot {
    param([string]$SavedRoot)
    $candidates = [System.Collections.Generic.List[string]]::new()
    if (-not [string]::IsNullOrWhiteSpace($env:LAUNCH_BAT_DIR)) {
        $batDir = $env:LAUNCH_BAT_DIR.TrimEnd('\', '/')
        $candidates.Add($batDir)
        $parent = Split-Path -Path $batDir -Parent
        if (-not [string]::IsNullOrWhiteSpace($parent)) { $candidates.Add($parent) }
    }
    $candidates.Add($SavedRoot)
    $candidates.Add((Get-Location).Path)
    foreach ($c in $candidates) {
        if (-not [string]::IsNullOrWhiteSpace($c) -and (Test-Path -LiteralPath (Join-Path $c "Cargo.toml"))) {
            return $c
        }
    }
    return $null
}

function Find-ServerManagerScript {
    param([string]$ServerRoot)
    foreach ($p in @(
        (Join-Path $ServerRoot "proxy\server_manager.py"),
        (Join-Path $ServerRoot "server_manager.py"),
        (Join-Path (Split-Path -Path $ServerRoot -Parent) "proxy\server_manager.py")
    )) {
        if (Test-Path -LiteralPath $p) { return $p }
    }
    return $null
}

function Find-MsvcEnvScript {
    $pf86 = ${env:ProgramFiles(x86)}
    if (-not [string]::IsNullOrWhiteSpace($pf86)) {
        $vswhere = Join-Path $pf86 "Microsoft Visual Studio\Installer\vswhere.exe"
        if (Test-Path -LiteralPath $vswhere) {
            $installPath = & $vswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath 2>$null
            if (-not [string]::IsNullOrWhiteSpace($installPath)) {
                $devCmd = Join-Path $installPath.Trim() "Common7\Tools\VsDevCmd.bat"
                if (Test-Path -LiteralPath $devCmd) { return $devCmd }
                $vcVars = Join-Path $installPath.Trim() "VC\Auxiliary\Build\vcvars64.bat"
                if (Test-Path -LiteralPath $vcVars) { return $vcVars }
            }
        }
    }
    foreach ($p in @(
        "${env:ProgramFiles(x86)}\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat",
        "${env:ProgramFiles}\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat",
        "${env:ProgramFiles(x86)}\Microsoft Visual Studio\2019\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
    )) {
        if (-not [string]::IsNullOrWhiteSpace($p) -and (Test-Path -LiteralPath $p)) { return $p }
    }
    return $null
}

function Ensure-Compiler {
    if ((Get-Command cl.exe -ErrorAction SilentlyContinue) -or (Find-MsvcEnvScript) -or (Get-Command gcc.exe -ErrorAction SilentlyContinue)) {
        return
    }
    Write-Host "Downloading MSVC Build Tools..."
    $installer = Join-Path $env:TEMP "vs_BuildTools.exe"
    Invoke-WebRequest -Uri "https://aka.ms/vs/17/release/vs_BuildTools.exe" -OutFile $installer -UseBasicParsing
    Write-Host "Installing MSVC Build Tools..."
    $proc = Start-Process -FilePath $installer -ArgumentList "--quiet", "--wait", "--norestart", "--nocache", "--add", "Microsoft.VisualStudio.Workload.VCTools", "--includeRecommended" -Wait -PassThru
    if ($proc.ExitCode -notin @(0, 3010)) {
        throw "MSVC install failed ($($proc.ExitCode))"
    }
    Refresh-ProcessPath
}

function Ensure-Rust {
    Refresh-ProcessPath
    if ((Get-Command cargo.exe -ErrorAction SilentlyContinue) -and (Get-Command rustc.exe -ErrorAction SilentlyContinue)) {
        return
    }
    Write-Host "Downloading Rust..."
    $rustup = Join-Path $env:TEMP "rustup-init.exe"
    Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile $rustup -UseBasicParsing
    Write-Host "Installing Rust..."
    $proc = Start-Process -FilePath $rustup -ArgumentList "-y", "--default-toolchain", "stable", "--default-host", "x86_64-pc-windows-msvc" -Wait -PassThru
    if ($proc.ExitCode -ne 0) {
        throw "rustup install failed ($($proc.ExitCode))"
    }
    Refresh-ProcessPath
    if (-not (Get-Command cargo.exe -ErrorAction SilentlyContinue)) {
        throw "cargo.exe not found"
    }
}

function Ensure-Python {
    Refresh-ProcessPath
    $py = Get-Command python.exe -ErrorAction SilentlyContinue
    if ($py -and $py.Source -notmatch 'WindowsApps\\python\.exe$') { return $py.Source }
    $pyLauncher = Get-Command py.exe -ErrorAction SilentlyContinue
    if ($pyLauncher) { return $pyLauncher.Source }

    Write-Host "Downloading Python..."
    $pyInst = Join-Path $env:TEMP "python-3.12.8-amd64.exe"
    Invoke-WebRequest -Uri "https://www.python.org/ftp/python/3.12.8/python-3.12.8-amd64.exe" -OutFile $pyInst -UseBasicParsing
    $proc = Start-Process -FilePath $pyInst -ArgumentList "/quiet", "InstallAllUsers=0", "PrependPath=1", "Include_test=0" -Wait -PassThru
    if ($proc.ExitCode -ne 0) {
        throw "Python install failed ($($proc.ExitCode))"
    }
    Refresh-ProcessPath
    $py = Get-Command python.exe -ErrorAction SilentlyContinue
    if (-not $py) { throw "python.exe not found" }
    return $py.Source
}

function Ensure-ServerBuilt {
    param([string]$ServerRoot)
    $bins = @("sonoro-aki-logic.exe", "sonoro-cfgsv.exe", "sonoro-loginsv.exe", "sonoro-gamesv.exe")
    $releaseDir = Join-Path $ServerRoot "target\release"
    $missing = $false
    foreach ($b in $bins) {
        if (-not (Test-Path -LiteralPath (Join-Path $releaseDir $b))) {
            $missing = $true
            break
        }
    }
    if (-not $missing) {
        Write-Host "Server build: ready"
        return
    }

    Ensure-Compiler
    Ensure-Rust

    $msvcScript = Find-MsvcEnvScript
    Write-Host "Building server..."
    Push-Location $ServerRoot
    try {
        if ($msvcScript) {
            cmd.exe /c "`"$msvcScript`" && cargo build --release --workspace"
            if ($LASTEXITCODE -ne 0) { throw "cargo build failed ($LASTEXITCODE)" }
        } else {
            & cargo build --release --workspace
            if ($LASTEXITCODE -ne 0) { throw "cargo build failed ($LASTEXITCODE)" }
        }
    } finally {
        Pop-Location
    }
}

function Wait-HttpHealth {
    param([string]$Url, [int]$TimeoutSec = 30)
    $deadline = (Get-Date).AddSeconds($TimeoutSec)
    while ((Get-Date) -lt $deadline) {
        try {
            $r = Invoke-WebRequest -Uri $Url -TimeoutSec 2 -UseBasicParsing
            if ($r.StatusCode -eq 200) { return $true }
        } catch {}
        Start-Sleep -Milliseconds 500
    }
    return $false
}

function Get-DetectedIps {
    $lanIp = ""
    try {
        $addrs = [System.Net.Dns]::GetHostAddresses([System.Net.Dns]::GetHostName()) |
            Where-Object { $_.AddressFamily -eq [System.Net.Sockets.AddressFamily]::InterNetwork -and -not [System.Net.IPAddress]::IsLoopback($_) }
        foreach ($a in $addrs) {
            $s = $a.IPAddressToString
            if ($s -notmatch '^169\.254\.') { $lanIp = $s; break }
        }
    } catch {}

    $pubIp = ""
    try {
        $pubIp = (Invoke-WebRequest -Uri "https://api.ipify.org" -TimeoutSec 3 -UseBasicParsing).Content.Trim()
    } catch {}

    return @{ LanIp = $lanIp; PublicIp = $pubIp }
}

function Get-AssetsManifest {
    param([string]$ServerRoot)
    foreach ($url in @(
        "https://raw.githubusercontent.com/kaaelix/sonoroserver/main/assets.json",
        "https://raw.githubusercontent.com/kaaelix/wuwaserver/main/assets.json"
    )) {
        try {
            $resp = Invoke-WebRequest -Uri $url -TimeoutSec 4 -UseBasicParsing
            if ($resp.StatusCode -eq 200 -and -not [string]::IsNullOrWhiteSpace($resp.Content)) {
                return ($resp.Content | ConvertFrom-Json)
            }
        } catch {}
    }
    $localJson = Join-Path $ServerRoot "assets.json"
    if (Test-Path -LiteralPath $localJson) {
        try { return (Get-Content -Raw -LiteralPath $localJson | ConvertFrom-Json) } catch {}
    }
    return $null
}

function Download-AssetsDb {
    param($Manifest, [string]$DbPath, [string]$VerFile)
    $urls = [System.Collections.Generic.List[string]]::new()
    if ($Manifest.url) { $urls.Add([string]$Manifest.url) }
    if ($Manifest.mirrors) {
        foreach ($m in $Manifest.mirrors) {
            if (-not [string]::IsNullOrWhiteSpace([string]$m)) { $urls.Add([string]$m) }
        }
    }
    if ($urls.Count -eq 0) { throw "No assets.db URL in assets.json" }

    $dir = Split-Path -Path $DbPath -Parent
    if (-not (Test-Path -LiteralPath $dir)) { New-Item -ItemType Directory -Path $dir -Force | Out-Null }
    $tmpPath = "$DbPath.tmp"

    foreach ($u in $urls) {
        Write-Host "Downloading assets.db from $u..."
        try {
            Remove-Item -LiteralPath $tmpPath -Force -ErrorAction SilentlyContinue
            $curlExe = Get-Command curl.exe -ErrorAction SilentlyContinue
            if ($curlExe) {
                & $curlExe.Source -fL --progress-bar -o $tmpPath $u
                if ($LASTEXITCODE -ne 0) { throw "curl exit $LASTEXITCODE" }
            } else {
                Invoke-WebRequest -Uri $u -OutFile $tmpPath -UseBasicParsing
            }
            if ($Manifest.sha256) {
                $actual = (Get-FileHash -LiteralPath $tmpPath -Algorithm SHA256).Hash.ToLowerInvariant()
                $expected = ([string]$Manifest.sha256).ToLowerInvariant()
                if ($actual -ne $expected) { throw "SHA256 mismatch ($actual != $expected)" }
            }
            Move-Item -LiteralPath $tmpPath -Destination $DbPath -Force
            $fi = Get-Item -LiteralPath $DbPath
            $meta = [PSCustomObject]@{
                version = [string]$Manifest.version
                sha256 = ([string]$Manifest.sha256).ToLowerInvariant()
                size = [int64]$fi.Length
                mtime = [int64]$fi.LastWriteTimeUtc.ToFileTimeUtc()
            } | ConvertTo-Json
            [System.IO.File]::WriteAllText($VerFile, $meta, [System.Text.UTF8Encoding]::new($false))
            Write-Host "assets.db ready (v$($Manifest.version))"
            return $true
        } catch {
            Write-Host "Download failed: $($_.Exception.Message)"
            Remove-Item -LiteralPath $tmpPath -Force -ErrorAction SilentlyContinue
        }
    }
    throw "Failed to download assets.db"
}

function Ensure-AssetsDb {
    param([string]$ServerRoot)
    $manifest = Get-AssetsManifest -ServerRoot $ServerRoot
    if ($null -eq $manifest) {
        Write-Host "assets.json unavailable; skipping assets check."
        return
    }

    $assetsDir = Join-Path $ServerRoot "assets"
    $dbPath = Join-Path $assetsDir "assets.db"
    $verFile = Join-Path $assetsDir "assets.version.json"

    if (-not (Test-Path -LiteralPath $dbPath) -or (Get-Item -LiteralPath $dbPath).Length -le 0) {
        $sizeMb = [math]::Round(([double]$manifest.size / 1MB), 1)
        Write-Host "assets/assets.db not found (v$($manifest.version), ${sizeMb} MB)."
        $ans = Read-Host "Download assets.db now? [Y/n]"
        if ([string]::IsNullOrWhiteSpace($ans) -or $ans.Trim() -match '^(y|yes)$') {
            Download-AssetsDb -Manifest $manifest -DbPath $dbPath -VerFile $verFile | Out-Null
            return
        }
        throw "assets/assets.db required to start local server."
    }

    $fi = Get-Item -LiteralPath $dbPath
    $localVer = ""
    $localHash = ""
    if (Test-Path -LiteralPath $verFile) {
        try {
            $vf = Get-Content -Raw -LiteralPath $verFile | ConvertFrom-Json
            if ([int64]$vf.size -eq [int64]$fi.Length -and [int64]$vf.mtime -eq [int64]$fi.LastWriteTimeUtc.ToFileTimeUtc()) {
                $localVer = [string]$vf.version
                $localHash = ([string]$vf.sha256).ToLowerInvariant()
            }
        } catch {}
    }

    if ([string]::IsNullOrWhiteSpace($localHash)) {
        $localHash = (Get-FileHash -LiteralPath $dbPath -Algorithm SHA256).Hash.ToLowerInvariant()
        if ($manifest.sha256 -and $localHash -eq ([string]$manifest.sha256).ToLowerInvariant()) {
            $localVer = [string]$manifest.version
            $meta = [PSCustomObject]@{
                version = $localVer
                sha256 = $localHash
                size = [int64]$fi.Length
                mtime = [int64]$fi.LastWriteTimeUtc.ToFileTimeUtc()
            } | ConvertTo-Json
            [System.IO.File]::WriteAllText($verFile, $meta, [System.Text.UTF8Encoding]::new($false))
        }
    }

    $targetHash = if ($manifest.sha256) { ([string]$manifest.sha256).ToLowerInvariant() } else { "" }
    $targetVer = [string]$manifest.version
    if (($targetHash -and $localHash -eq $targetHash) -or ($localVer -and $localVer -eq $targetVer)) {
        Write-Host "assets.db: up to date (v$targetVer)"
        return
    }

    $curLabel = if ($localVer) { "v$localVer" } else { "local" }
    $sizeMb = [math]::Round(([double]$manifest.size / 1MB), 1)
    Write-Host "assets.db update available: $curLabel -> v$targetVer (${sizeMb} MB)"
    $ans = Read-Host "Update assets.db now? [y/N]"
    if (-not [string]::IsNullOrWhiteSpace($ans) -and $ans.Trim() -match '^(y|yes)$') {
        Download-AssetsDb -Manifest $manifest -DbPath $dbPath -VerFile $verFile | Out-Null
    } else {
        Write-Host "Keeping current assets.db."
    }
}

function Start-LocalServer {
    param([string]$ServerRoot, [string]$BindHost, [string]$AdvertiseIp)
    Ensure-AssetsDb -ServerRoot $ServerRoot
    Ensure-ServerBuilt -ServerRoot $ServerRoot
    $pyBin = Ensure-Python
    foreach ($sslDir in @("C:\Program Files\Git\usr\bin", "C:\Program Files\Git\mingw64\bin", "C:\Program Files\OpenSSL-Win64\bin")) {
        if ((Test-Path -LiteralPath $sslDir) -and ($env:Path -notlike "*$sslDir*")) { $env:Path = "$sslDir;$env:Path" }
    }
    $mgrScript = Find-ServerManagerScript -ServerRoot $ServerRoot
    if (-not $mgrScript) {
        throw "server_manager.py not found in $ServerRoot\proxy or $ServerRoot."
    }

    $logDir = Join-Path $ServerRoot "logs"
    if (-not (Test-Path -LiteralPath $logDir)) { New-Item -ItemType Directory -Path $logDir -Force | Out-Null }
    $releaseDir = Join-Path $ServerRoot "target\release"
    $stateArg = Join-Path $ServerRoot "state"
    $env:GAMESV_STATE_DIR = $stateArg

    $procs = [System.Collections.Generic.List[System.Diagnostics.Process]]::new()
    $specs = @(
        @{ Name = "sonoro-aki-logic"; Exe = (Join-Path $releaseDir "sonoro-aki-logic.exe"); Args = @(); Log = "aki-logic" },
        @{ Name = "sonoro-cfgsv"; Exe = (Join-Path $releaseDir "sonoro-cfgsv.exe"); Args = @("-s", $stateArg, "-l", "${BindHost}:15500"); Log = "cfgsv" },
        @{ Name = "sonoro-loginsv"; Exe = (Join-Path $releaseDir "sonoro-loginsv.exe"); Args = @("-s", $stateArg, "-l", "${BindHost}:10001", "-g", "${AdvertiseIp}:13100"); Log = "loginsv" },
        @{ Name = "sonoro-gamesv"; Exe = (Join-Path $releaseDir "sonoro-gamesv.exe"); Args = @("-s", $stateArg, "-b", "${BindHost}:13100", "-g", "127.0.0.1:13101"); Log = "gamesv" }
    )

    foreach ($s in $specs) {
        $outLog = Join-Path $logDir "$($s.Log).out.log"
        $errLog = Join-Path $logDir "$($s.Log).err.log"
        Write-Host "Starting $($s.Name)..."
        $p = Start-Process -FilePath $s.Exe -ArgumentList $s.Args -WorkingDirectory $ServerRoot -RedirectStandardOutput $outLog -RedirectStandardError $errLog -WindowStyle Hidden -PassThru
        $procs.Add($p)
    }

    $mgrArgs = @($mgrScript, "--host", $AdvertiseIp, "--login-port", "5500", "--proxy-port", "8080", "--socks-port", "1080", "--gateway-port", "13100", "--loginsv-url", "http://127.0.0.1:10001", "--gm-url", "http://127.0.0.1:13101")
    Write-Host "Starting server_manager.py..."
    $mgrOut = Join-Path $logDir "server_manager.out.log"
    $mgrErr = Join-Path $logDir "server_manager.err.log"
    $pMgr = Start-Process -FilePath $pyBin -ArgumentList $mgrArgs -WorkingDirectory (Split-Path -Path $mgrScript -Parent) -RedirectStandardOutput $mgrOut -RedirectStandardError $mgrErr -WindowStyle Hidden -PassThru
    $procs.Add($pMgr)

    if (-not (Wait-HttpHealth -Url "http://127.0.0.1:13101/health" -TimeoutSec 30)) {
        Stop-LocalServer -Procs $procs
        throw "gamesv failed to start ($logDir)"
    }
    if (-not (Wait-HttpHealth -Url "http://127.0.0.1:5500/health" -TimeoutSec 20)) {
        Stop-LocalServer -Procs $procs
        throw "server_manager.py failed to start ($logDir)"
    }
    return $procs
}

function Stop-LocalServer {
    param([System.Collections.Generic.List[System.Diagnostics.Process]]$Procs)
    if ($null -eq $Procs) { return }
    foreach ($p in $Procs) {
        try {
            if (-not $p.HasExited) { Stop-Process -Id $p.Id -Force -ErrorAction SilentlyContinue }
        } catch {}
    }
}

function Launch-GameClient {
    param([string]$ProxyIp, [string]$SavedExe, [string]$SaveIp, [string]$ServerRoot)
    $exePath = Find-GameExe -SavedExe $SavedExe
    while (-not $exePath) {
        $manual = Read-Host "Game not found. Enter game folder or Client-Win64-Shipping.exe path"
        $exePath = Resolve-ExePath $manual
    }

    $win64Dir = Split-Path -Path $exePath -Parent
    $clientDir = Split-Path -Path (Split-Path -Path $win64Dir -Parent) -Parent
    $engineIni = Join-Path $clientDir "Saved\Config\WindowsNoEditor\Engine.ini"

    Write-Host "EXE: $exePath"
    Patch-EngineIni -IniPath $engineIni
    if (-not [string]::IsNullOrWhiteSpace($env:LOCALAPPDATA)) {
        $appDataIni = Join-Path $env:LOCALAPPDATA "Wuthering Waves\Saved\Config\WindowsNoEditor\Engine.ini"
        if (Test-Path -LiteralPath $appDataIni) { Patch-EngineIni -IniPath $appDataIni }
    }

    Save-Config -ExePath $exePath -Ip $SaveIp -ServerRoot $ServerRoot

    $proxyUrl = "http://${ProxyIp}:8080"
    $inetKey = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Internet Settings"
    $prev = Get-ItemProperty -Path $inetKey -ErrorAction SilentlyContinue
    $prevEnable = if ($null -ne $prev.ProxyEnable) { [int]$prev.ProxyEnable } else { 0 }
    $prevServer = if ($null -ne $prev.ProxyServer) { [string]$prev.ProxyServer } else { "" }
    $prevOverride = if ($null -ne $prev.ProxyOverride) { [string]$prev.ProxyOverride } else { "" }

    try {
        Set-ItemProperty -Path $inetKey -Name ProxyEnable -Value 1
        Set-ItemProperty -Path $inetKey -Name ProxyServer -Value "${ProxyIp}:8080"
        Set-ItemProperty -Path $inetKey -Name ProxyOverride -Value "<local>"

        $env:HTTP_PROXY = $proxyUrl
        $env:HTTPS_PROXY = $proxyUrl

        Write-Host "Launching Client-Win64-Shipping.exe -> ${ProxyIp}:8080"
        $proc = Start-Process -FilePath $exePath -ArgumentList "-httpproxy=$proxyUrl", "-NoLauncher" -WorkingDirectory $win64Dir -PassThru
        $proc.WaitForExit()
    } finally {
        Set-ItemProperty -Path $inetKey -Name ProxyEnable -Value $prevEnable
        if ($prevServer -ne "") { Set-ItemProperty -Path $inetKey -Name ProxyServer -Value $prevServer }
        else { Remove-ItemProperty -Path $inetKey -Name ProxyServer -ErrorAction SilentlyContinue }
        if ($prevOverride -ne "") { Set-ItemProperty -Path $inetKey -Name ProxyOverride -Value $prevOverride }
        else { Remove-ItemProperty -Path $inetKey -Name ProxyOverride -ErrorAction SilentlyContinue }
    }
}

$cfg = Load-Config
Write-Host "1. Connect Server"
Write-Host "2. Local Server"
$modeRaw = Read-Host "Choice [1]"
$mode = if ([string]::IsNullOrWhiteSpace($modeRaw)) { "1" } else { $modeRaw.Trim() }

if ($mode -eq "1") {
    $savedIp = if ([string]::IsNullOrWhiteSpace($cfg.PublicIp)) { $DefaultRemoteIp } else { $cfg.PublicIp }
    $inputIp = Read-Host "Server IP [$savedIp]"
    $targetIp = if ([string]::IsNullOrWhiteSpace($inputIp)) { $savedIp } else { $inputIp.Trim() }

    if (-not (Test-ServerAlive -Ip $targetIp)) {
        Read-Host "Press Enter to exit"
        exit 1
    }

    Launch-GameClient -ProxyIp $targetIp -SavedExe $cfg.GameExePath -SaveIp $targetIp -ServerRoot $cfg.ServerRoot
    exit 0
}

if ($mode -ne "2") {
    Write-Host "Invalid choice."
    exit 1
}

$serverRoot = Find-ServerRoot -SavedRoot $cfg.ServerRoot
while (-not $serverRoot) {
    $manualRoot = Read-Host "Enter wuwa-server folder path"
    if (-not [string]::IsNullOrWhiteSpace($manualRoot)) {
        $cand = $manualRoot.Trim().Trim('"')
        if (Test-Path -LiteralPath (Join-Path $cand "Cargo.toml")) { $serverRoot = $cand }
    }
}

Write-Host "1. Solo (127.0.0.1)"
Write-Host "2. Co-op"
$hostModeRaw = Read-Host "Choice [1]"
$hostMode = if ([string]::IsNullOrWhiteSpace($hostModeRaw)) { "1" } else { $hostModeRaw.Trim() }

$bindHost = "127.0.0.1"
$advertiseIp = "127.0.0.1"
if ($hostMode -eq "2") {
    $bindHost = "0.0.0.0"
    $ips = Get-DetectedIps
    if ($ips.LanIp) { Write-Host "LAN IP: $($ips.LanIp)" }
    if ($ips.PublicIp) { Write-Host "Public IP: $($ips.PublicIp)" }
    $defaultAdv = if ($ips.PublicIp) { $ips.PublicIp } elseif ($ips.LanIp) { $ips.LanIp } else { "127.0.0.1" }
    $advInput = Read-Host "Host IP [$defaultAdv]"
    $advertiseIp = if ([string]::IsNullOrWhiteSpace($advInput)) { $defaultAdv } else { $advInput.Trim() }
}

$serverProcs = $null
try {
    $serverProcs = Start-LocalServer -ServerRoot $serverRoot -BindHost $bindHost -AdvertiseIp $advertiseIp
    Write-Host "Server ready."

    if ($hostMode -eq "2") {
        Write-Host "Friend IP: $advertiseIp"
        Write-Host "Ports: 5500/TCP, 8080/TCP, 13100/UDP (optional: 1080/TCP+UDP)"
    }

    $launchChoice = Read-Host "Launch game? [Y/n]"
    if ([string]::IsNullOrWhiteSpace($launchChoice) -or $launchChoice.Trim() -match '^(y|yes)$') {
        Launch-GameClient -ProxyIp "127.0.0.1" -SavedExe $cfg.GameExePath -SaveIp $cfg.PublicIp -ServerRoot $serverRoot
        if ($hostMode -eq "2") {
            Read-Host "Press Enter to stop server"
        }
    } else {
        Save-Config -ExePath $cfg.GameExePath -Ip $cfg.PublicIp -ServerRoot $serverRoot
        Read-Host "Press Enter to stop server"
    }
} finally {
    Stop-LocalServer -Procs $serverProcs
}
