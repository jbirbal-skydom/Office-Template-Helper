#!/bin/bash
# Enhanced uninstall script example
echo "Running enhanced uninstall script..."

# Define the target directories
SETTINGS_TARGET="$HOME/Library/Application Support/office-template-helper/settings"
REFERENCE_TARGET="$HOME/Library/Application Support/office-template-helper/reference"
MAIN_TAR="$HOME/Library/Application Support/office-template-helper"

# Function to remove directory if it exists
remove_directory() {
    if [ -d "$1" ]; then
        rm -rf "$1"
        echo "Removed $1"
    else
        echo "Directory not found: $1"
    fi
}

# Remove the directories and their contents
remove_directory "$SETTINGS_TARGET"
remove_directory "$REFERENCE_TARGET"
remove_directory "$MAIN_TAR"

echo "Office Template Helper settings and reference data have been cleaned up from your system."


