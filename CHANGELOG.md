# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

-   Introduce `KyResult<()>`, which is an alias for `Result<(), KyError>` [#15](https://github.com/numToStr/ky/pull/15)
-   Introduce `Encrypted` and `Decrypted` types [#16](https://github.com/numToStr/ky/pull/16)

### Changed

-   Store master password in common database [#10](https://github.com/numToStr/ky/pull/10)

This is **BREAKING** change which will break the existing stored data. To migrate to the new format you need to export your data.

```bash
# Export data from previous version likely v0.1.0
ky export

# Update to new version and import the exported data
# Make sure to delete the existing database
ky import
```

-   Internal refactoring [#19](https://github.com/numToStr/ky/pull/19)

### Fixed

-   Printing encrypted key on error [#11](https://github.com/numToStr/ky/pull/11)

## [0.1.0] - 2020-06-19

### Added

-   Restore vault from a git repository [#4](https://github.com/numToStr/ky/pull/4)
-   Readme and Changelog [#7](https://github.com/numToStr/ky/pull/7)

### Changed

-   Rename `git push` to `git backup` [#6](https://github.com/numToStr/ky/pull/6)
-   New encryption and hashing strategy [#8](https://github.com/numToStr/ky/pull/8)
-   Connect to a named database instead of default [#9](https://github.com/numToStr/ky/pull/9)

## [0.0.1] - 2020-06-15

Initial Release
