#!/bin/bash

sudo apt update
sudo apt install lld clang -y


cargo install cargo-watch

# Check if Homebrew is installed
if command -v brew >/dev/null 2>&1; then
    echo "Homebrew is installed. Installing gh..."
    brew install gh
else
    echo "Homebrew is not installed. Please install Homebrew first: https://brew.sh/"
    exit 1
fi

exit