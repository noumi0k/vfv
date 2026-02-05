# Usage Guide

## Quick Start

```bash
# Build the project
cargo build --release

# Run from project directory
cargo run -- /path/to/browse

# Or install and run anywhere
cargo install --path .
vive-file-viewer ~/dev
```

## Navigation

### Moving Around

- Use `j`/`k` or arrow keys to move up/down in the file list
- Press `Enter` or `l` to enter a directory or preview a file
- Press `Backspace` or `h` to go back to parent directory
- Press `g` to jump to the first item
- Press `G` to jump to the last item

### File Preview

The right pane shows a preview of the selected file with:
- Line numbers
- Syntax highlighting (auto-detected by file extension)
- Use `PageUp`/`PageDown` to scroll the preview

### Search/Filter

1. Press `/` to enter search mode
2. Type your search query (filters file list in real-time)
3. Press `Enter` to confirm or `Esc` to cancel

### External Editor

Press `e` to open the selected file in your configured editor.

## Configuration

### Config File Location

- macOS: `~/Library/Application Support/vive-file-viewer/config.toml`
- Linux: `~/.config/vive-file-viewer/config.toml`

### Editor Configuration

```toml
# VSCode
editor = "code"

# Cursor
editor = "cursor"

# Vim
editor = "vim"
editor_args = []

# Neovim
editor = "nvim"
```

### Display Options

```toml
# Show hidden files by default
show_hidden = true

# Maximum preview lines (for large files)
preview_max_lines = 2000
```

### Themes

Available syntax highlighting themes:
- `base16-ocean.dark` (default)
- `base16-eighties.dark`
- `base16-mocha.dark`
- `InspiredGitHub`
- `Solarized (dark)`
- `Solarized (light)`
