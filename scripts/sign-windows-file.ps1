param(
    [Parameter(Mandatory = $true)][string]$FilePath,
    [Parameter(Mandatory = $true)][ValidatePattern('^[0-9a-fA-F]{40}$')][string]$CertificateThumbprint,
    [Parameter(Mandatory = $true)][string]$TimestampUrl,
    [string]$SignTool = 'signtool.exe',
    [switch]$VerifyOnly
)
$ErrorActionPreference = 'Stop'
try {
    if (!$VerifyOnly) {
        & $SignTool sign /sha1 $CertificateThumbprint /s My /fd SHA256 /tr $TimestampUrl /td SHA256 /d Quiet.md $FilePath
        if ($LASTEXITCODE -ne 0) { throw "Signing failed: $FilePath" }
    }
    & $SignTool verify /pa /all /tw $FilePath
    if ($LASTEXITCODE -ne 0) { throw "Signature verification failed: $FilePath" }
    $signature = Get-AuthenticodeSignature -FilePath $FilePath
    if ($signature.Status -ne 'Valid' -or $signature.SignerCertificate.Thumbprint -ne $CertificateThumbprint) {
        throw "Invalid signature or unexpected publisher: $FilePath"
    }
    if (!$signature.TimeStamperCertificate) { throw "Missing trusted timestamp: $FilePath" }
} catch {
    Write-Error $_ -ErrorAction Continue
    exit 1
}
