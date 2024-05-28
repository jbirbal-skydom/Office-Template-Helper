# Changelog

## v1.0.0

Previous settings location will disrupt and break with this change 
### Added or Changed
- Move the settings to a more accessible location
  | **Platform** | **Configuration Directory**               | **Example Path**                                 |
  |--------------|------------------------------------------|--------------------------------------------------|
  | **Linux**    | `$XDG_CONFIG_HOME` or `$HOME/.config`    | `/home/alice/.config`                            |
  | **macOS**    | `$HOME/Library/Application Support`      | `/Users/Alice/Library/Application Support`       |
  | **Windows**  | `{FOLDERID_LocalAppData}`                | `C:\Users\Alice\AppData\Local`                   |

- Prevent reopening of reference file (valid_file.yaml)

## v0.1.0

### Added or Changed
- Added this changelog :)
- Fixed typos in both templates
- Back to top links
- Added more "Built With" frameworks/libraries
- Changed table of contents to start collapsed
- Added checkboxes for major features on roadmap

### Removed

- Some packages/libraries from acknowledgements I no longer use