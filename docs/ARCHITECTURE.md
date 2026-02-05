# Architecture

## Module Overview

```
src/
├── main.rs           # Entry point, event loop
├── app.rs            # Application state management
├── ui.rs             # UI rendering (Ratatui)
├── file_browser.rs   # File system operations
├── preview.rs        # File preview with syntax highlighting
├── config.rs         # Configuration management
└── editor.rs         # External editor integration
```

## Module Responsibilities

### main.rs
- Command line argument parsing
- Terminal setup/teardown (raw mode, alternate screen)
- Main event loop
- Keyboard input handling

### app.rs
- Central application state (`App` struct)
- Coordinates between modules
- Handles user actions (navigation, search, etc.)
- Input mode management (Normal/Search)

### ui.rs
- Ratatui frame rendering
- Layout management (header, main area, footer)
- File list rendering
- Preview pane rendering with syntax colors

### file_browser.rs
- Directory reading and listing
- File entry representation
- Sorting (directories first, then alphabetical)
- Search/filter functionality
- Hidden file handling

### preview.rs
- File content reading
- Binary file detection
- Syntax highlighting using syntect
- Line-by-line highlighting with styles

### config.rs
- TOML configuration parsing
- Default values
- Config file path resolution using `directories` crate

### editor.rs
- External process spawning
- Editor command construction

## Data Flow

```
User Input → main.rs (event loop)
                ↓
            app.rs (state update)
           /    |    \
    file_browser  preview  editor
          \      |      /
           ui.rs (render)
                ↓
            Terminal
```

## Dependencies

- **ratatui**: TUI framework
- **crossterm**: Terminal manipulation
- **syntect**: Syntax highlighting
- **serde/toml**: Configuration
- **directories**: Cross-platform config paths
