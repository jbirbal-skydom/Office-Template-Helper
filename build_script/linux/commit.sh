#!/bin/bash
# Get all commits since the last tag and format them for the Debian changelog.

# Define your package name and email
PKG_NAME="office-template-helper"
EMAIL="JBirbal@skydom.ai"

# Fetch the latest tag name
LAST_TAG=$(git describe --tags --abbrev=0)

# Generate the changelog entries
echo "$PKG_NAME ($(date +%Y-%m-%d)-1) unstable; urgency=low"
echo
git log $LAST_TAG..HEAD --oneline | while read line; do
    echo "  * ${line}"
done
echo
echo " -- Jason Birbal <$EMAIL>  $(date -R)"
echo