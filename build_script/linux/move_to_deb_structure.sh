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
# └── office-template-helper_1.0.0-1_amd64
#     ├── debian
#     │   ├── changelog
#     │   ├── control
#     │   ├── postinst
#     │   └── prerm
#     └── usr
#         ├── bin
#         │   ├── office-template-helper-CLI
#         │   └── office-template-helper-GUI
#         └── share
#             └── office-template-helper
#                 ├── reference
#                 │   └── visio.vsdx
#                 └── settings
#                     ├── GUI_theme.yaml
#                     ├── addon.yaml
#                     └── valid_file.yaml



# Set the base directory where the current source files are located
BASE_DIR=$(dirname "$(dirname "$(pwd)")")

# Set the destination directory for the .deb packaging
DEST_DIR="$BASE_DIR/built/linux/office-template-helper-1.0.0"
mkdir -p "$DEST_DIR"

# Ensure the destination directories exist
mkdir -p "$DEST_DIR/debian"
mkdir -p "$DEST_DIR/debian/source"
mkdir -p "$DEST_DIR/bin"
mkdir -p "$DEST_DIR/share/office-template-helper/reference"
mkdir -p "$DEST_DIR/share/office-template-helper/settings"

# Copy debian control files
cp "$BASE_DIR/build_script/linux/changelog" "$DEST_DIR/debian"
cp "$BASE_DIR/build_script/linux/control" "$DEST_DIR/debian"
cp "$BASE_DIR/build_script/linux/copyright" "$DEST_DIR/debian"
cp "$BASE_DIR/build_script/linux/postinst" "$DEST_DIR/debian"
cp "$BASE_DIR/build_script/linux/prerm" "$DEST_DIR/debian"
cp "$BASE_DIR/build_script/linux/rules" "$DEST_DIR/debian"
cp "$BASE_DIR/build_script/linux/compat" "$DEST_DIR/debian"
cp "$BASE_DIR/build_script/linux/format" "$DEST_DIR/debian/source"

# Copy executable files to /usr/bin
cp "$BASE_DIR/target/x86_64-unknown-linux-gnu/release/office-template-helper-CLI" "$DEST_DIR/bin"
cp "$BASE_DIR/target/x86_64-unknown-linux-gnu/release/office-template-helper-GUI" "$DEST_DIR/bin"

# Copy settings and reference files
cp -r "$BASE_DIR/settings/." "$DEST_DIR/share/office-template-helper/settings"
cp "$BASE_DIR/reference/visio.vsdx" "$DEST_DIR/share/office-template-helper/reference"

# Ensure all copied files have correct permissions
chmod -R 755 "$DEST_DIR/bin"
chmod -R 644 "$DEST_DIR/share/office-template-helper/settings/"
chmod 644 "$DEST_DIR/share/office-template-helper/reference/visio.vsdx"

# Set correct permissions for debian control files
chmod 755 "$DEST_DIR/debian/postinst"
chmod 755 "$DEST_DIR/debian/prerm"
chmod 644 "$DEST_DIR/debian/control"
chmod 644 "$DEST_DIR/debian/copyright"
chmod 644 "$DEST_DIR/debian/changelog"
chmod 755 "$DEST_DIR/debian/rules"
chmod 755 "$DEST_DIR/debian/compat"

echo "Files have been successfully moved and permissions set."
