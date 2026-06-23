<#
.SYNOPSIS
  Packages ChromaVale into an MSIX and signs it with a local test certificate.

.DESCRIPTION
  Proves the Store-packaging path end to end. The resulting .msix can be
  test-installed locally (after trusting the test cert). For the Microsoft
  Store you do NOT sign it yourself: update AppxManifest.xml with your Partner
  Center identity, pack without signing, and submit -- Microsoft re-signs it.

  Run this from the desktop/ folder after `npm run tauri build`:
    powershell -ExecutionPolicy Bypass -File msix\pack.ps1
#>
param(
  [string]$Pfx = "$PSScriptRoot\devcert.pfx",
  [string]$PfxPassword = "chromavale",
  [switch]$SkipSign
)
$ErrorActionPreference = "Stop"

$desktop = Split-Path $PSScriptRoot -Parent
$exe = Join-Path $desktop "src-tauri\target\release\chromavale.exe"
if (-not (Test-Path $exe)) {
  throw "Build the app first: npm run tauri build"
}

# Stage the payload.
$stage = Join-Path $PSScriptRoot "stage"
if (Test-Path $stage) { Remove-Item $stage -Recurse -Force }
New-Item -ItemType Directory -Force $stage | Out-Null
Copy-Item $exe (Join-Path $stage "chromavale.exe")
Copy-Item (Join-Path $PSScriptRoot "AppxManifest.xml") (Join-Path $stage "AppxManifest.xml")

$assets = Join-Path $stage "Assets"
New-Item -ItemType Directory -Force $assets | Out-Null
$icons = Join-Path $desktop "src-tauri\icons"
foreach ($logo in @("Square150x150Logo.png", "Square44x44Logo.png", "StoreLogo.png")) {
  Copy-Item (Join-Path $icons $logo) $assets
}

# Locate the newest Windows SDK tools.
$sdkBin = Get-ChildItem "C:\Program Files (x86)\Windows Kits\10\bin" -Directory |
  Where-Object { Test-Path "$($_.FullName)\x64\makeappx.exe" } |
  Sort-Object Name -Descending | Select-Object -First 1
if (-not $sdkBin) { throw "Windows SDK (makeappx.exe) not found." }
$makeappx = Join-Path $sdkBin.FullName "x64\makeappx.exe"
$signtool = Join-Path $sdkBin.FullName "x64\signtool.exe"

# Pack.
$msix = Join-Path $PSScriptRoot "ChromaVale.msix"
& $makeappx pack /o /d $stage /p $msix
if ($LASTEXITCODE -ne 0) { throw "makeappx failed" }

# Sign with a local test certificate (skip for Store submission).
if (-not $SkipSign) {
  if (-not (Test-Path $Pfx)) {
    Write-Host "Creating a self-signed test certificate (CN=ChromaVale)..."
    $cert = New-SelfSignedCertificate -Type Custom -Subject "CN=ChromaVale" `
      -KeyUsage DigitalSignature -FriendlyName "ChromaVale Test" `
      -CertStoreLocation "Cert:\CurrentUser\My" `
      -TextExtension @("2.5.29.37={text}1.3.6.1.5.5.7.3.3", "2.5.29.19={text}")
    $pw = ConvertTo-SecureString -String $PfxPassword -Force -AsPlainText
    Export-PfxCertificate -Cert $cert -FilePath $Pfx -Password $pw | Out-Null
  }
  & $signtool sign /fd SHA256 /a /f $Pfx /p $PfxPassword $msix
  if ($LASTEXITCODE -ne 0) { throw "signtool failed" }
}

Write-Host ""
Write-Host "Built: $msix"
