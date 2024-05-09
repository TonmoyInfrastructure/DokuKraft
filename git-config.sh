#!/bin/bash

# Author    : Eshan Roy <eshanized@proton.me>

# Prompt user for GitHub username
read -p "Enter your GitHub username: " github_username

# Prompt user for GitHub email address
read -p "Enter your GitHub email address: " github_email

# Prompt user for GitHub personal access token
# Documentation : https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens
read -sp "Enter your GitHub personal access token: " github_token

# Set GitHub username
git config --global github.user "$github_username"

# Set GitHub email address
git config --global github.email "$github_email"

# Set GitHub personal access token
git config --global github.token "$github_token"

echo "GitHub credentials configured successfully."
