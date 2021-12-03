<#
    ** ADVENT OF CODE 02021 **
    **   DAY 3 - PUZZLE 1   **
    **     DARK-COFFEE      **
#>

$challengeInput = Get-Content 'input.txt'
$metricCount = ($challengeInput | Measure-Object | Select-Object -ExpandProperty Count)

[int]$D1  = 0
[int]$D2  = 0
[int]$D3  = 0
[int]$D4  = 0
[int]$D5  = 0
[int]$D6  = 0
[int]$D7  = 0
[int]$D8  = 0
[int]$D9  = 0
[int]$D10 = 0
[int]$D11 = 0
[int]$D12 = 0

foreach($metric in $challengeInput){

    if(($metric.Substring(0,1))  -eq 0){ $D1  = $D1  + 1 }
    if(($metric.Substring(1,1))  -eq 0){ $D2  = $D2  + 1 }
    if(($metric.Substring(2,1))  -eq 0){ $D3  = $D3  + 1 }
    if(($metric.Substring(3,1))  -eq 0){ $D4  = $D4  + 1 }
    if(($metric.Substring(4,1))  -eq 0){ $D5  = $D5  + 1 }
    if(($metric.Substring(5,1))  -eq 0){ $D6  = $D6  + 1 }
    if(($metric.Substring(6,1))  -eq 0){ $D7  = $D7  + 1 }
    if(($metric.Substring(7,1))  -eq 0){ $D8  = $D8  + 1 }
    if(($metric.Substring(8,1))  -eq 0){ $D9  = $D9  + 1 }
    if(($metric.Substring(9,1))  -eq 0){ $D10 = $D10 + 1 }
    if(($metric.Substring(10,1)) -eq 0){ $D11 = $D11 + 1 }
    if(($metric.Substring(11,1)) -eq 0){ $D12 = $D12 + 1 }

}

if($D1  -le ($metricCount/2)){  $g1 = 0;  $e1 = 1 }else{  $g1 = 1;  $e1 = 0 }
if($D2  -le ($metricCount/2)){  $g2 = 0;  $e2 = 1 }else{  $g2 = 1;  $e2 = 0 }
if($D3  -le ($metricCount/2)){  $g3 = 0;  $e3 = 1 }else{  $g3 = 1;  $e3 = 0 }
if($D4  -le ($metricCount/2)){  $g4 = 0;  $e4 = 1 }else{  $g4 = 1;  $e4 = 0 }
if($D5  -le ($metricCount/2)){  $g5 = 0;  $e5 = 1 }else{  $g5 = 1;  $e5 = 0 }
if($D6  -le ($metricCount/2)){  $g6 = 0;  $e6 = 1 }else{  $g6 = 1;  $e6 = 0 }
if($D7  -le ($metricCount/2)){  $g7 = 0;  $e7 = 1 }else{  $g7 = 1;  $e7 = 0 }
if($D8  -le ($metricCount/2)){  $g8 = 0;  $e8 = 1 }else{  $g8 = 1;  $e8 = 0 }
if($D9  -le ($metricCount/2)){  $g9 = 0;  $e9 = 1 }else{  $g9 = 1;  $e9 = 0 }
if($D10 -le ($metricCount/2)){ $g10 = 0; $e10 = 1 }else{ $g10 = 1; $e10 = 0 }
if($D11 -le ($metricCount/2)){ $g11 = 0; $e11 = 1 }else{ $g11 = 1; $e11 = 0 }
if($D12 -le ($metricCount/2)){ $g12 = 0; $e12 = 1 }else{ $g12 = 1; $e12 = 0 }

[string]$gammaRate = "$($g1)$($g2)$($g3)$($g4)$($g5)$($g6)$($g7)$($g8)$($g9)$($g10)$($g11)$($g12)"
[string]$epsilonRate = "$($e1)$($e2)$($e3)$($e4)$($e5)$($e6)$($e7)$($e8)$($e9)$($e10)$($e11)$($e12)"
[int]$powerConsumption = (([Convert]::ToInt32($gammaRate, 2)) * ([Convert]::ToInt32($epsilonRate, 2)))

# Print Output
Clear-Host
Write-Output "***********************************"
Write-Output "  Advent of Code: Day 3, Puzzle 1  "
Write-Output "***********************************"
Write-Output ""
Write-Output "SUBMARINE POWER CONSUMPTION DATA"
Write-Output "-----------------------------------"
Write-Output "  GAMMA RATE: $($gammaRate)"
Write-Output "EPSILON RATE: $($epsilonRate)"
Write-Output " CONSUMPTION: $($powerConsumption)"
Write-Output "-----------------------------------"
Write-Output "TRANSMISSION END."