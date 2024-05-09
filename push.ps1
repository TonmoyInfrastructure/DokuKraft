# Author        : Eshanized <m.eshanized@gmail.com>
# Author URI    : https://tonmoyinfrastructure.github.io/eshanized/

################################ NOTE ################################
#!!!!!!!!!!!!! Execute all the scripts at your own risk !!!!!!!!!!!!!
# I have written the push script for Arch Linux and other Arch Based #
# Linux Distribution. So this script will only work in ArchLinux and #
# Arch based Linux Distribution. You may customize it according to   #
# your Distribution.                                                 #
######################################################################

# Function to check if Commitizen is installed
function Check-Commitizen {
    if (-not (Get-Command commitizen-go -ErrorAction SilentlyContinue)) {
        Write-Host "Commitizen is not installed. Please install it using 'yay -S commitizen-go'." -ForegroundColor Red
        exit 1
    }
}

# Function to stage, commit, and push changes
function Push-To-Github {
    git add .
    git cz
    git push origin master
}

# Main Function
function Main {
    Check-Commitizen
    Push-To-Github
}

# Call the main function
Main
