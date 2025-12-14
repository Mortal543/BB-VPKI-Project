Param(
    [int]$SleepSeconds = 60
)

# Continuously run the BB-VPKI benchmark on Windows (PowerShell)
# Usage: .\scripts\continuous_run.ps1 -SleepSeconds 60

$outdir = "logs"
if (-not (Test-Path $outdir)) { New-Item -ItemType Directory -Path $outdir | Out-Null }

Write-Host "Starting continuous-run loop. Logs will be written to $outdir\"

try {
    while ($true) {
        $ts = (Get-Date).ToUniversalTime().ToString("yyyyMMddTHHmmssZ")
        $log = "$outdir\benchmark-$ts.log"
        Write-Host "=== Starting run at $ts (logs -> $log) ==="
        # Run cargo run --release and tee output to file
        & cargo run --release 2>&1 | Tee-Object -FilePath $log
        Write-Host "=== Run finished at $((Get-Date).ToUniversalTime().ToString('yyyyMMddTHHmmssZ')) ==="
        Write-Host "Sleeping for $SleepSeconds seconds before next run..."
        Start-Sleep -Seconds $SleepSeconds
    }
} catch {
    Write-Host "Continuous-run interrupted: $_" -ForegroundColor Yellow
}
