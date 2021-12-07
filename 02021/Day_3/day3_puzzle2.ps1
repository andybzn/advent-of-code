<#
    ** ADVENT OF CODE 02021 **
    **   DAY 3 - PUZZLE 2   **
    **     DARK-COFFEE      **
#>

$challengeInput = Get-Content 'input.txt'
$metricCount = ($challengeInput | Measure-Object | Select-Object -ExpandProperty Count)

[int]$D1  = 0
[int]$h = 0
[int]$position = 1

[array]$numsToFilter = @()
[array]$tempNumsToFilter = @()

foreach($metric in $challengeInput){if(($metric.Substring(0,1))  -eq 0){ $D1  = $D1  + 1 }}
if($D1  -le ($metricCount/2)){  $g1 = 0;  $e1 = 1 }else{  $g1 = 1;  $e1 = 0 }
if($g1 -eq 1){ $h = $g1 }else{ $h = $e1}

foreach ($metric in $challengeInput){
    if(($metric.substring(0,1)) -eq $h){
        $numsToFilter = $numsToFilter + $metric
    }
}

while(($numsToFilter -eq 0) -or ($numsToFilter -ne 1)){

    $tempNumsToFilter = $numsToFilter
    $numsToFilter = $null
    [array]$numsToFilter = @()
    $f1 = 0
    $g1 = 0
    $e1 = 0
    $filter = 0
    foreach ($number in $tempNumsToFilter) { if(($number.Substring(($position),1))  -eq 0){ $f1  = $f1  + 1 }}
    if($f1  -le (($tempNumsToFilter | Measure-Object | Select-Object -ExpandProperty Count)/2)){  $g1 = 0;  $e1 = 1 }else{  $g1 = 1;  $e1 = 0 }
    if($g1 -eq 1){ $filter = $g1 }else{ $filter = $e1}

    foreach ($number in $tempNumsToFilter){
        if(($number.substring(($position),1)) -eq $filter){
            $numsToFilter = $numsToFilter + $number
        }
    }
    Write-Host "Numbers in Filter: $($numsToFilter| Measure-Object | Select-Object -ExpandProperty Count)"
    $position = $position + 1
    return $numsToFilter
}
Write-Host $numsToFilter



#[string]$gammaRate = "$($g1)$($g2)$($g3)$($g4)$($g5)$($g6)$($g7)$($g8)$($g9)$($g10)$($g11)$($g12)"
#[string]$epsilonRate = "$($e1)$($e2)$($e3)$($e4)$($e5)$($e6)$($e7)$($e8)$($e9)$($e10)$($e11)$($e12)"
#[int]$powerConsumption = (([Convert]::ToInt32($gammaRate, 2)) * ([Convert]::ToInt32($epsilonRate, 2)))

# Print Output
Clear-Host
Write-Output "***********************************"
Write-Output "  Advent of Code: Day 3, Puzzle 2  "
Write-Output "***********************************"
Write-Output ""
Write-Output "SUBMARINE DIAGNOSTIC DATA"
Write-Output "-----------------------------------"
Write-Output "O2 GENERATOR RATING: $($oGRating)"
Write-Output "CO2 SCRUBBER RATING: $($cSRating)"
Write-Output "LIFE SUPPORT RATING: $($lSRating)"
Write-Output "-----------------------------------"
Write-Output "TRANSMISSION END."