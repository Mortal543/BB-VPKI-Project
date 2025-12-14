Param()

# PowerShell helper to convert docs/run_instructions.md to PDF using pandoc
$md = "docs/run_instructions.md"
$out = "docs/run_instructions.pdf"

function Check-Command($cmd) {
    $p = Get-Command $cmd -ErrorAction SilentlyContinue
    return $null -ne $p
}

if (-not (Check-Command pandoc)) {
    Write-Host "pandoc not found. Install using Chocolatey: 'choco install pandoc -y' or winget: 'winget install --id=Pandoc.Pandoc'" -ForegroundColor Yellow
    exit 2
}

Write-Host "Generating PDF from $md → $out"

$proc = Start-Process -FilePath pandoc -ArgumentList "$md","-o","$out" -NoNewWindow -Wait -PassThru
if ($proc.ExitCode -ne 0) {
    Write-Host "pandoc failed with exit code $($proc.ExitCode)" -ForegroundColor Red
    exit $proc.ExitCode
}

Write-Host "Done: $out" -ForegroundColor Green