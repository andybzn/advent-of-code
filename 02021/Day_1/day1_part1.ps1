<#
    ** ADVENT OF CODE 02021 **
    **   DAY 1 - PUZZLE 1   **
    **     DARK-COFFEE      **
#>

$challengeInput = Get-Content input.txt
[int]$counter = 1
[int]$arrayPosition = 0

foreach ($measurement in $challengeInput){

    if($arrayPosition -eq 0){
        #Do Nothing
    }elseif($measurement -gt ($challengeInput[($arrayPosition -1)])){
        $counter = $counter + 1
    }else{
        #Do Nothing
    }

    $arrayPosition = $arrayPosition + 1
}

# Print Output
Clear-Host
Write-Output "***********************************"
Write-Output "  Advent of Code: Day 1, Puzzle 1  "
Write-Output "***********************************"
Write-Output ""
Write-Output "SUBMARINE SONAR DATA"
Write-Output "-----------------------------------"
Write-Output "SEA FLOOR DEPTH INCREASES: $counter"
Write-Output "-----------------------------------"
Write-Output "TRANSMISSION END."