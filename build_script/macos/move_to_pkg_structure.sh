#!/bin/bash


# use a script to move the files to the correct location from current location to /tmp/

# from: 
# .
# ├── CHANGELOG.md
# ├── Cargo.lock
# ├── Cargo.toml
# ├── LICENSE.txt
# ├── README.md
# ├── app_icon.rc
# ├── app_icon.res
# ├── assets
# │   ├── icons
# │   │   ├── _oth_logo.ico
# │   │   └── logo.ico
# │   ├── logo.png
# │   └── logo.svg
# ├── build.rs
# ├── build_script
# │   ├── linux
# │   │   ├── changelog
# │   │   ├── commit.sh
# │   │   ├── control
# │   │   ├── full_commits.sh
# │   │   ├── move.sh
# │   │   ├── postinst
# │   │   └── prerm
# │   └── windows
# │       └── buildscript.iss
# ├── built
# │   ├── linux
# │   ├── macos
# │   │   ├── reference
# │   │   │   └── visio.vsdx
# │   │   └── settings
# │   │       ├── GUI_theme.yaml
# │   │       ├── addon.yaml
# │   │       └── valid_file.yaml
# │   └── windows
# │       └── office-template-helper.exe
# ├── createpackage.md
# ├── icon.png
# ├── images
# │   ├── _logo.png
# │   ├── _screenshot.png
# │   ├── logo.png
# │   └── screenshot.png
# ├── reference
# │   ├── complex.abc
# │   ├── simple
# │   │   ├── inside
# │   │   │   └── inner.xml
# │   │   └── outer.xml
# │   ├── simple.xyz
# │   ├── simple.xz
# │   ├── simple.zip
# │   └── visio.vsdx
# ├── settings
# │   ├── GUI_theme.yaml
# │   ├── addon.yaml
# │   └── valid_file.yaml
# ├── src
# │   ├── addon.rs
# │   ├── arguments.rs
# │   ├── cli.rs
# │   ├── file_handler.rs
# │   ├── gui.rs
# │   ├── gui_helper.rs
# │   ├── lib.rs
# │   └── modify.rs
# ├── target
# │   └── x86_64-unknown-linux-gnu
# │       ├── CACHEDIR.TAG
# │       └── release
# │           ├── liboffice_template_helper.d
# │           ├── liboffice_template_helper.rlib
# │           ├── office-template-helper-CLI
# │           ├── office-template-helper-CLI.d
# │           ├── office-template-helper-GUI
# │           └── office-template-helper-GUI.d
# ├── test
# │   ├── WUaMGgQFU4.zip
# │   ├── blank-Geqo(modified)-E8Dx(modified)-BVNE(modified).vsdx
# │   ├── blank-Geqo(modified)-E8Dx(modified).vsdx
# │   ├── blank-Geqo(modified).vsdx
# │   ├── blank-suy0(modified).vsdx
# │   ├── blank-t2ng(modified).vsdx
# │   ├── blank-zwEd(modified).vsdx
# │   ├── blank.vsdx
# │   ├── complex.abc
# │   ├── complex.xml
# │   ├── complex.zip
# │   ├── hierachy.zip
# │   ├── i0ROwPJ1Rj.zip
# │   ├── inside
# │   │   └── inner.xml
# │   ├── iui9TXpjVP.zip
# │   ├── modified.vsdx
# │   └── simple.zip
# └── ui
#     └── appwindow.slint

#     === to built/linux  == 

# .
# ├───config
# │   ├───reference
# │   │       visio.vsdx
# │   │
# │   └───settings
# │           addon.yaml
# │           GUI_theme.yaml
# │           valid_file.yaml
# │
# ├───payload
# │   └───usr
# │       └───local
# │           └───bin
# │                   office-template-helper-CLI
# │                   office-template-helper-GUI
# │
# └───script
#         postinstall
#         preinstall



# Set the base directory where the current source files are located
BASE_DIR=$(dirname "$(dirname "$(pwd)")")

# Set the destination directory for the .deb packaging
DEST_DIR="$BASE_DIR/built/macos/office-template-helper-1.0.0"
APP_DIR="$DEST_DIR/OfficeTemplateHelper.app"


# Create directories in the destination
mkdir -p "$DEST_DIR"
mkdir -p "$APP_DIR"
mkdir -p "$DEST_DIR/config/reference"
mkdir -p "$DEST_DIR/config/settings"
mkdir -p "$APP_DIR/Contents/MacOS/"
mkdir -p "$APP_DIR/Contents/Resources/"
mkdir -p "$APP_DIR/Contents/Frameworks/"
mkdir -p "$DEST_DIR/script"

# Move config files
cp "$BASE_DIR/reference/visio.vsdx" "$DEST_DIR/config/reference/"
cp -r "$BASE_DIR/settings/." "$DEST_DIR/config/settings/"

# Move binaries
cp "$BASE_DIR/build_script/macos/office-template-helper-CLI" "$APP_DIR/Contents/MacOS/"
cp "$BASE_DIR/build_script/macos/OfficeTemplateHelper" "$APP_DIR/Contents/MacOS/"

# Move icon
cp "$BASE_DIR/assets/icons/MyIcon.icns" "$APP_DIR/Contents/Resources/"

# Move Info
cp "$BASE_DIR/build_script/macos/Info.plist" "$APP_DIR/Contents/"

# Move scripts (assuming you already have these scripts ready)
cp "$BASE_DIR/build_script/macos/postinstall" "$DEST_DIR/script"
cp "$BASE_DIR/build_script/macos/preinstall" "$DEST_DIR/script"

echo "Files have been moved to $DEST_DIR"