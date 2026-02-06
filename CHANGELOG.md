# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-02-06

## [0.1.1] - 2025-02-06

### Added
- Published to [crates.io](https://crates.io/crates/vfv) - now installable via `cargo install vfv`
- `-b/--base` option in TUI search to specify search base directory
- Automated release workflow (GitHub Releases + crates.io)

### Changed
- Package name changed from `vive-file-viewer` to `vfv` for easier installation
- TUI search now uses the currently opened directory as the base (not the startup directory)

## [0.1.0] - 2025-02-04

### Added
- Initial release
- TUI file browser with Vim keybindings
- Fuzzy search powered by nucleo (same as Helix editor)
- Syntax highlighting for file preview
- `.gitignore` aware file listing
- CLI search command (`vfv find`) for AI/script integration
- Jump navigation (`f` key)
- Help screen (`?` key)
- Path copy to clipboard (`y` key)
- Hidden file toggle (`.` key)
- External editor integration (`e` key)
- Exact match option (`-e/--exact`)
- Directory-only search (`-d/--dir`)
- Path filtering (query containing `/`)
- JSON output for CLI (`-j/--json`)
