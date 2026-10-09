param([ValidateSet('build','check','test','clippy','all')][string]$Task='all')
$ErrorActionPreference='Stop'
Set-Location $PSScriptRoot
$finder=Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio\Installer\vswhere.exe'
if (!(Test-Path -LiteralPath $finder)) { throw 'Visual Studio Build Tools (C++デスクトップ開発) が必要です。' }
$install=& $finder -latest -products '*' -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
if (!$install) { throw 'MSVC C++ビルドツールが見つかりません。' }
$developer=Join-Path $install 'Common7\Tools\VsDevCmd.bat'
$envLines=& cmd.exe /d /c "call `"$developer`" -arch=x64 -host_arch=x64 >nul && set"
if ($LASTEXITCODE -ne 0) { throw 'MSVCの環境を初期化できませんでした。' }
foreach ($line in $envLines) { if ($line -match '^([^=]+)=(.*)$') { [Environment]::SetEnvironmentVariable($Matches[1],$Matches[2],'Process') } }
$env:CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER=Join-Path $env:VCToolsInstallDir 'bin\Hostx64\x64\link.exe'
$env:PATH=(Join-Path $env:VCToolsInstallDir 'bin\Hostx64\x64')+';'+$env:PATH
$portableSdk=Join-Path $PSScriptRoot '..\..\work\windows-sdk'
if (Test-Path -LiteralPath $portableSdk) {
    $sdkVersion=(Get-ChildItem (Join-Path $portableSdk 'c\Include') -Directory | Sort-Object Name -Descending | Select-Object -First 1).Name
    $env:LIB=(Join-Path $portableSdk 'c\um\x64')+';'+(Join-Path $portableSdk 'c\ucrt\x64')+';'+$env:LIB
    $env:INCLUDE=(Join-Path $portableSdk "c\Include\$sdkVersion\ucrt")+';'+(Join-Path $portableSdk "c\Include\$sdkVersion\shared")+';'+(Join-Path $portableSdk "c\Include\$sdkVersion\um")+';'+$env:INCLUDE
    $env:PATH=(Join-Path $portableSdk "c\bin\$sdkVersion\x64")+';'+$env:PATH
}
function Invoke-Cargo([string[]]$Arguments) { & cargo @Arguments; if ($LASTEXITCODE -ne 0) { throw "cargo $Arguments failed" } }
if ($Task -eq 'all') {
    Invoke-Cargo @('fmt','--','--check')
    Invoke-Cargo @('clippy','--offline','--locked','--all-targets','--','-D','warnings')
    Invoke-Cargo @('test','--offline','--locked')
    Invoke-Cargo @('build','--offline','--locked','--release')
} elseif ($Task -eq 'build') { Invoke-Cargo @('build','--offline','--locked','--release') }
elseif ($Task -eq 'clippy') { Invoke-Cargo @('clippy','--offline','--locked','--all-targets','--','-D','warnings') }
else { Invoke-Cargo @($Task,'--offline','--locked') }
