<#
    ** ADVENT OF CODE 02021 **
    **   DAY 2 - PUZZLE 1   **
    **     DARK-COFFEE      **
#>

$challengeInput = Get-Content input.txt

[int]$horizontalPosition = 0
[int]$verticalPosition = 0
[int]$combinedAxis = 0

foreach($navigation in $challengeInput){
    [string]$direction = $navigation -replace '\s.*'
    [int]$distance = $navigation -replace '.*\s'

    if($direction -eq 'forward'){
        $horizontalPosition = $horizontalPosition + $distance
    }elseif($direction -eq 'up'){
        $verticalPosition = $verticalPosition - $distance
    }elseif($direction -eq 'down'){
        $verticalPosition = $verticalPosition + $distance
    }else{
        #Do Nothing
    }
}

$combinedAxis = $horizontalPosition * $verticalPosition

# Print Output
Clear-Host
Write-Output "***********************************"
Write-Output "  Advent of Code: Day 2, Puzzle 1  "
Write-Output "***********************************"
Write-Output ""
Write-Output "SUBMARINE POSITIONING DATA"
Write-Output "-----------------------------------"
Write-Output "DISTANCE: $($horizontalPosition)"
Write-Output "   DEPTH: $($verticalPosition)"
Write-Output "COMBINED: $($combinedAxis)"
Write-Output "-----------------------------------"
Write-Output "TRANSMISSION END."