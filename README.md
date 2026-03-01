# 🚀 Rust-FZF: Blazingly Fast Fuzzy Finder

A high-performance, interactive terminal fuzzy finder built with **Rust**. It features multi-threaded file discovery, a reactive UI governor for zero-flicker rendering, and powerful shell integration.

## ✨ Features

* **⚡ Parallel Enumeration**: Uses `ignore` (the engine behind `ripgrep`) to scan millions of files in parallel, respecting `.gitignore` and hidden file rules.
* **🖇️ Interactive Exec**: Pipe results directly into terminal commands using the `--exec` flag. Supports complex shell pipelines with `|`, `>`, and redirects.
* **✅ Multi-Selection**: Mark multiple files using `Tab` to perform batch operations.
* **🎯 Manual & Smart Case**: Toggle between case-sensitive and case-insensitive matching on the fly.
* **🎨 Live Theming**: Switch between **Nord**, **Dracula**, and **Catppuccin** palettes instantly.
* **📖 Stateful Preview**: Syntax-highlighted code previews powered by `syntect`, with frame-aware caching for smooth scrolling.
* **📟 Reactive Governor**: Smart rendering logic providing 60 FPS for input while throttling background discovery to maintain performance.

---

## ⌨️ Keybindings

| Key | Action |
| --- | --- |
| `Tab` | **Toggle selection** (Multi-select mode) |
| `Ctrl + t` | Open **Theme Selection** menu |
| `Ctrl + p` | Toggle **File Preview** |
| `Ctrl + r` | Toggle **Relative / Absolute** path display |
| `Ctrl + s` | Toggle **Case Sensitivity** |
| `Ctrl + u` | Clear search input |
| `Up / Down` | Navigate list (or scroll Preview if open) |
| `Enter` | Execute command on selection (or print to stdout) |
| `Esc` | Quit |

---

## 🛠️ Installation

### Prerequisites

Ensure you have the Rust toolchain installed.

```bash
# Clone the repository
git clone https://github.com/skugge74/rust-fzf
cd rust-fzf

# Build for release with optimizations
cargo build --release

# Install to your path
sudo cp target/release/rust-fzf /usr/local/bin/rfzf

```

---

## 🚀 Usage

### Basic Search

Scan the current directory:

```bash
rfzf

```

### Advanced Discovery

Scan your home folder with hidden files and preview enabled:

```bash
rfzf ~ --hidden --preview

```

### 🛠️ Shell Integration (The Power User Move)

The `--exec` flag allows you to pass selections into any terminal command. Use `{}` as a placeholder for the file path.

**Open multiple selected files in Neovim:**

```bash
rfzf --exec "nvim {}"

```

**Search content within found files and limit output:**

```bash
rfzf --exec "grep 'TODO' {} | head -n 20"

```

**Batch copy selected files to a backup directory:**

```bash
rfzf --exec "cp {} ~/backups/"

```

---

## 🎨 Themes Included

* **Nord**: A clean, arctic-blue aesthetic.
* **Dracula**: A high-contrast, vibrant dark theme.
* **Catppuccin**: A soothing, pastel-themed palette.

---

## ⚙️ Technical Details

* **TUI Framework**: `ratatui`
* **Fuzzy Engine**: `nucleo` (High-performance matching indices)
* **Concurrency**: `ignore` for lock-free parallel file injection.
* **Syntax Highlighting**: `syntect` (Sublime-compatible definitions).
* **Execution**: Commands are spawned via `sh -c` to support full shell syntax and piping.

---

### Roadmap

* [x] Interactive shell integration
* [x] Multi-selection mode
* [ ] File metadata sidebar (Size, Permissions, Modified)

---

