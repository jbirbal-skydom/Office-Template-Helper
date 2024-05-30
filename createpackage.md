# create package 
To package app windows need a res made into the binary 
## Windows
- Install  [Inno Setup](https://jrsoftware.org/) 
- Run `buildscript.iss`

## linux
```
office-template-helper_1.0.0-1_amd64
├── DEBIAN
│   ├── changelog
│   ├── control
│   ├── postinst
│   └── prerm
└── usr
    ├── bin
    │   ├── office-template-helper-CLI
    │   └── office-template-helper-GUI
    └── share
        └── office-template-helper
            ├── reference
            │   └── visio.vsdx
            └── settings
                ├── GUI_theme.yaml
                ├── addon.yaml
                └── valid_file.yaml
```
./generate_changelog.sh >> debian/changelog

cp -r office-template-helper_1.0.0-1_amd64/ /tmp/
dpkg --build office-template-helper_1.0.0-1_amd64
sudo apt install /tmp/office-template-helper_1.0.0-1_amd64.deb

### PPA 
`gpg --full-gen-key`
`gpg --list-keys` [cheatsheet](https://gock.net/blog/2020/gpg-cheat-sheet)

gpg --armor --export jbirbal@skydom.ai > publickey.gpg
apt-ftparchive release . > Release

1. Using debuild or dch:
   1. When updating your debian/changelog, you can use the dch command to automatically handle the formatting and updating for you:
   2. `dch -i`
   3. ` tar czvf office-template-helper_1.0.0.orig.tar.gz office-template-helper_1.0.0-1_amd64/` 
   4. `debuild -S -sa `  
       1. testing `dpkg-buildpackage -us -uc`
   5. `dput ppa:skydom/office-template-helper ./office-template-helper_1.0.0-1_source.changes`
      1. `changelog` needs to match key exactly

2. Create a debian directory inside your package's root with necessary files (control, rules, changelog, etc.).
3. Use debuild -S to generate a source package which will create the .dsc and .changes files.
4. Sign and upload the package using dput with the .changes file.

5. Create a Launchpad Account   
   1. Sign Up: Go to Launchpad.net and sign up for an account if you don't already have one.
   2. Login: Once your account is set up and verified, log in to Launchpad.
6. Set Up GPG and SSH Keys
7. GPG Key: You need a GPG key to sign your packages. This ensures authenticity and integrity.
    1. Generate a GPG key: `gpg --full-gen-key`
    2. Follow the prompts to complete the key generation.
    3. Publish your GPG key to Ubuntu keyserver:
        ```sh
        gpg --send-keys --keyserver keyserver.ubuntu.com YOUR_KEY_ID
        ```
8. Create Your PPA
9. Prepare and Upload Your Package
    Prepare Your Package: Make sure your package is properly packaged in the Debian format. This typically involves creating a .dsc file, a .changes file, and the actual .deb package.
       Use tools like debuild or dpkg-buildpackage to create these files from your source directory.
    Sign the Package: Sign the package with your GPG key:

        debsign -kYOUR_KEY_ID your-package_version_source.changes
    Upload the Package: Use dput to upload your package to your PPA:

        dput ppa:yourusername/ppaname your-package_version_source.changes
    Replace yourusername and ppaname with your actual Launchpad username and PPA name.

10. Manage Your PPA
   1. To use your PPA, users will need to add it to their system and then install the packages:
    ```sh
    sudo add-apt-repository ppa:yourusername/ppaname
    sudo apt-get update
    sudo apt-get install your-package
    ```
    
11. Users Adding Your PPA





## MacOs