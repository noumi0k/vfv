# vive-file-viewer

A terminal file viewer with syntax highlighting, built with Rust and Ratatui.

## Features

- Directory navigation with keyboard shortcuts
- Syntax highlighting for code preview
- Open files in external editor (VSCode, Cursor, etc.)
- Configurable via TOML
- File search/filter

## Installation

```bash
cd ~/dev/vive-file-viewer
cargo install --path .
```

## Usage

```bash
# Browse current directory
vive-file-viewer

# Browse specific directory
vive-file-viewer ~/dev
```

## Key Bindings

| Key | Action |
|-----|--------|
| `j` / `↓` | Move down |
| `k` / `↑` | Move up |
| `Enter` / `l` / `→` | Enter directory / Preview file |
| `Backspace` / `h` / `←` | Go to parent directory |
| `e` | Open in editor |
| `/` | Search/filter |
| `.` | Toggle hidden files |
| `g` | Go to top |
| `G` | Go to bottom |
| `Ctrl+d` / `Ctrl+f` | Scroll preview down (half/full page) |
| `Ctrl+u` / `Ctrl+b` | Scroll preview up (half/full page) |
| `PageUp` / `PageDown` | Scroll preview (full page) |
| `q` / `Ctrl+C` | Quit |

## Configuration

Create `~/.config/vive-file-viewer/config.toml`:

```toml
editor = "cursor"
editor_args = []
show_hidden = false
preview_max_lines = 1000
theme = "base16-ocean.dark"
```

See `config.toml.example` for all options.
