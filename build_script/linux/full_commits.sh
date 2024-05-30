#!/bin/bash
# Script to format all git commits into a Debian changelog entry

# Define your package name and email
PKG_NAME="office-template-helper"
EMAIL="JBirbal@skydom.ai"

# Generate the changelog entries
echo "$PKG_NAME ($(date +%Y-%m-%d)-1) unstable; urgency=low"
echo

# Include all commits from the beginning of the project
git log --oneline | while read line; do
    echo "  * ${line}"
done
echo

echo " -- Jason Birbal <$EMAIL>  $(date -R)"
echo