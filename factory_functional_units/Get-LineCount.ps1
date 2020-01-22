Split-Path -Parent $PSCommandPath | Push-Location
try {
    $total = 0
    $stats = Get-ChildItem -Recurse -Filter *.rs |
        Where-Object { git check-ignore $_ > $null; $LASTEXITCODE -ne 0 } |
        ForEach-Object { 
            $stats = Get-Content $_ | Measure-Object
            $total += $stats.Count
            @{ name = $_.Name; line_count = $stats.Count}
        }
    
    $stats | 
        Sort-Object -Descending -Property "line_count" |
        ForEach-Object { Write-Host ('{1,4} {0}' -f $_.name, $_.line_count) }
    Write-Host ('{1,4} {0}' -f "total", $total)
}
finally {
    Pop-Location
}