# Rust-FZF

Rust-FZF is a terminal-based fuzzy finder written in Rust. It provides a text user interface (TUI) for discovering and selecting files, built on top of the `ignore` crate for concurrent traversal and `ratatui` for the interface. It supports multi-selection, live file previews, and direct shell command execution.

## Features

* **Parallel Traversal**: Uses the `ignore` crate to scan directories across multiple threads while respecting `.gitignore`, `.ignore`, and hidden file rules.
* **Shell Execution (`--exec`)**: Execute shell commands against selected files directly from the TUI. Supports standard shell syntax, including pipes and redirects.
* **Multi-Selection**: Mark multiple files to batch process them through a command or print them as a list to standard output.
* **Match Highlighting**: Real-time visual highlighting of fuzzy-matched characters within file paths.
* **File Previews & Metadata**: Integrated preview pane featuring syntax highlighting, file size, Unix permissions, and modification timestamps.
* **Case Matching**: Toggle between case-sensitive and smart-case matching during runtime.
* **Theming**: Includes built-in color schemes (Nord, Dracula, Catppuccin) accessible via an in-app menu.
* **Render Throttling**: Prioritizes UI responsiveness for user input while batching background discovery updates to minimize terminal flicker.

## Keybindings

| Key | Action |
| --- | --- |
| `Tab` | Toggle selection (Multi-select) |
| `Enter` | Execute command on selection (or print to stdout) |
| `Ctrl + p` | Toggle Preview pane and metadata sidebar |
| `Ctrl + t` | Open Theme Selection menu |
| `Ctrl + s` | Toggle Case Sensitivity |
| `Ctrl + r` | Toggle Relative/Absolute path display |
| `Ctrl + u` | Clear search input |
| `Up / Down` | Navigate list (scrolls preview text when active) |
| `Esc` | Quit |

## Usage

### Basic Search

Search the current directory:

```bash
rfzf

```

Search a specific directory with hidden files and the preview pane enabled:

```bash
rfzf ~ --hidden --preview

```

### Command Execution (`--exec`)

Pass selected files to standard terminal commands. The `{}` placeholder is replaced by the selected file path(s) and is safely escaped.

Open a file in an editor:

```bash
rfzf --exec "nvim {}"

```

Process file content through a pipeline:

```bash
rfzf --exec "grep 'TODO' {} | head -n 20"

```

Perform bulk file operations:

```bash
rfzf --exec "cp {} ~/backups/archive_$(date +%F)/"

```

## Installation

### Prerequisites

* [Rust toolchain](https://rustup.rs/)

### Build from Source

```bash
git clone https://github.com/skugge74/rust-fzf
cd rust-fzf
cargo build --release

# Move the binary to a location in your PATH
sudo cp target/release/rust-fzf /usr/local/bin/rfzf

```

## Technical Dependencies

* **TUI Framework**: `ratatui`
* **Fuzzy Engine**: `nucleo-matcher`
* **Syntax Highlighting**: `syntect`
* **Execution**: Commands are spawned via `sh -c`, enabling support for shell operators and logic.
