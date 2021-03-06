# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2021-06-30

### Added

- Allow specifying --skip and --length in terms of bytes/kilobytes/kibibytes/etc

### Changed

- Read in files incrementally, to avoid high memory usage when reading large files
- Fixed bug where lines would often print extra trailing spaces

## [0.1.0] - 2021-06-29

### Added

- Initial CLI
- One-byte octal display
- One-byte character display
- Two-byte octal display
- Two-byte hexadecimal display
- Canonical hex+ASCII display
- Two-byte decimal display
- Skip bytes in display
- Limit bytes in display

[unreleased]: https://github.com/aszecsei/hex/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/aszecsei/hex/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/aszecsei/hex/releases/tag/v0.1.0
