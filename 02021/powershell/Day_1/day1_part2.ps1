<#
    ** ADVENT OF CODE 02021 **
    **   DAY 1 - PUZZLE 2   **
    **     DARK-COFFEE      **
#>

$challengeInput = Get-Content input.txt
[int]$counter = 0
[int]$arrayPosition = 0
[int]$previousSum = 0

foreach ($measurement in $challengeInput){

    [int]$measurement1 = $measurement
    [int]$measurement2 = $challengeInput[($arrayPosition +1)]
    [int]$measurement3 = $challengeInput[($arrayPosition +2)]
    [int]$sum = $($measurement1) + $($measurement2) + $($measurement3)
    
    if($arrayPosition -eq 0){
        #do nothing 
    }elseif($sum -gt $previousSum){
        $counter = $counter + 1
    }

    $arrayPosition = $arrayPosition + 1
    $previousSum = $sum
}
    
# Print Output
Clear-Host
Write-Output "***********************************"
Write-Output "  Advent of Code: Day 1, Puzzle 2  "
Write-Output "***********************************"
Write-Output ""
Write-Output "SUBMARINE SONAR DATA"
Write-Output "-----------------------------------"
Write-Output "SEA FLOOR DEPTH INCREASES: $counter"
Write-Output "-----------------------------------"
Write-Output "TRANSMISSION END."