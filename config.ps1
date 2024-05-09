# Prompt user for GitHub username
$githubUsername = Read-Host "Enter your GitHub username"

# Prompt user for GitHub email address
$githubEmail = Read-Host "Enter your GitHub email address"

# Prompt user for GitHub personal access token
$githubToken = Read-Host "Enter your GitHub personal access token" -AsSecureString

# Convert secure string to plain text
$githubTokenPlainText = [Runtime.InteropServices.Marshal]::PtrToStringAuto([Runtime.InteropServices.Marshal]::SecureStringToBSTR($githubToken))

# Set GitHub username
git config --global github.user "$githubUsername"

# Set GitHub email address
git config --global github.email "$githubEmail"

# Set GitHub personal access token
git config --global github.token "$githubTokenPlainText"

Write-Host "GitHub credentials configured successfully."
