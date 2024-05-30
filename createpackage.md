# create package

To package app windows need a res made into the binary

## Windows

- Install  [Inno Setup](https://jrsoftware.org/)
- Run `buildscript.iss`

## Linux

```sh
office-template-helper_1.0.0-1_amd64
├── debian
│   ├── changelog
│   ├── compat
│   ├── control
│   ├── copyright
│   ├── format
│   ├── postinst
│   ├── prerm
│   └── source
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

### Generate a .deb manual (not recommended)

All we need is the basic `control` and `changelog` files
we can generate a mimic changelog from the git commit since it has to be in a certain format
    ```./generate_changelog.sh >> debian/changelog```

- Build and Install:

    ```sh
    cp -r office-template-helper_1.0.0-1_amd64/ /tmp/
    dpkg --build office-template-helper_1.0.0-1_amd64
    sudo apt install /tmp/office-template-helper_1.0.0-1_amd64.deb
    ```

### PPA

The forma is different using a lowercase `debian` vs `DEBIAN` folder

1. Using debuild or dch:
   1. When updating your debian/changelog, you can use the dch command to automatically handle the formatting and updating for you:
   2. `dch -i`
   3. `tar czvf office-template-helper_1.0.0.orig.tar.gz office-template-helper_1.0.0-1_amd64/`
   4. `debuild -S -sa`  
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
      1. `gpg --full-gen-key`
      2. `gpg --list-keys` [cheatsheet](https://gock.net/blog/2020/gpg-cheat-sheet)
   2. Follow the prompts to complete the key generation.
   3. Publish your GPG key to Ubuntu keyserver:

        ```sh
        gpg --send-keys --keyserver keyserver.ubuntu.com YOUR_KEY_ID
        ```

8. Create Your PPA
9. Prepare and Upload Your Package
    1. build a `.tar.xz` of the source and tructure the folder: `tar czvf office-template-helper_1.0.0.orig.tar.gz office-template-helper-1.0.0`

        ```sh
            .
            ├── office-template-helper-1.0.0
            │   ├── bin
            │   │   ├── office-template-helper-CLI
            │   │   └── office-template-helper-GUI
            │   ├── debian
            │   │   ├── changelog
            │   │   ├── compat
            │   │   ├── control
            │   │   ├── copyright
            │   │   ├── files
            │   │   ├── postinst
            │   │   ├── prerm
            │   │   ├── rules
            │   │   └── source
            │   │       └── format
            │   └── share
            │       └── office-template-helper
            │           ├── reference
            │           │   └── visio.vsdx
            │           └── settings
            │               ├── GUI_theme.yaml
            │               ├── addon.yaml
            │               └── valid_file.yaml
            └── office-template-helper_1.0.0-1.debian.tar.xz

        ```

    2. With `debuild -S -sa` everything is done:
        1. This typically involves creating a .dsc file, a .changes file, and the actual .deb package.
        2. Use tools like debuild or dpkg-buildpackage to create these files from your source directory.
        3. Sign the Package: Sign the package with your GPG key:
        4. `debsign -kYOUR_KEY_ID your-package_version_source.changes`
10. Upload the Package: Use dput to upload your package to your PPA:

    ```sh
    dput ppa:yourusername/ppaname your-package_version_source.changes
    ```

    Replace yourusername and ppaname with your actual Launchpad username and PPA name.

11. Manage Your PPA

12. Users Adding Your PPA

    1. To use your PPA, users will need to add it to their system and then install the packages:

    ```sh
    sudo add-apt-repository ppa:yourusername/ppaname
    sudo apt-get update
    sudo apt-get install your-package
    ```

## MacOs

there are three packages: 

- dmg: [dmgcanvas](https://www.araelium.com/dmgcanvas)
   * put install script here with  [platypus](https://sveinbjorn.org/platypus)

  - pkg: most of this is created by the application [Packages](http://s.sudre.free.fr/Software/Packages/about.html)
  
    ```sh
    .
    └── expanded
        ├── Bom
        ├── PackageInfo
        ├── Payload.app
        └── Scripts
            ├── postinstall
            ├── preinstall
            ├── reference
            │   └── visio.vsdx
            └── settings
                ├── GUI_theme.yaml
                ├── addon.yaml
                └── valid_file.yaml
    ```

    - app (Manual)
  
        ```sh
        OfficeTemplateHelper.app/            # Root of the application bundle
        └── Contents/                        # Mandatory root folder inside the app bundle
            ├── Info.plist                   # Contains metadata about the application
            ├── MacOS/                       # Directory containing the executable files
            │   └── OfficeTemplateHelper     # Main executable for the app
            ├── Resources/                   # Resources like images, sounds, and other assets
            │   ├── icon.icns                # Application icon file
            │   └── other resources...       # Other resource files as needed
            └── Frameworks/                  # (Optional) Directory for bundled frameworks or shared libraries
                └── someframework.framework  # Example of a framework (if needed)

        ```



