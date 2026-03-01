# 🚀 Rust-FZF: Blazingly Fast Fuzzy Finder

**Rust-FZF** is a high-performance, interactive TUI fuzzy finder engineered for speed and extensibility. Designed to handle millions of entries without breaking a sweat, it combines the parallel discovery power of `ripgrep` with a sophisticated, reactive UI and full shell pipeline support.

---

## ✨ Features

* **⚡ Parallel Discovery Engine**: Built on the `ignore` crate (the core of `ripgrep`), scanning files across all CPU cores while respecting `.gitignore`, `.ignore`, and hidden file rules.
* **🖇️ Deep Shell Integration**: The `--exec` flag turns your finder into a command composer. Supports full shell syntax, including pipes (`|`), redirects (`>`), and complex one-liners.
* **✅ Multi-Selection Workflow**: Press `Tab` to mark multiple files for batch processing. Selected paths are handed off to your shell command or printed as a clean list.
* **🔍 Fuzzy Match Highlighting**: Real-time visual feedback! Characters matched by the fuzzy engine are highlighted and underlined within the filename.
* **📊 Contextual Metadata Sidebar**: Instant access to file size, Unix permissions, and "time-ago" modification stamps within the preview pane.
* **🎯 Dynamic Case Matching**: Toggle between case-sensitive and smart-case matching instantly without restarting your search.
* **🎨 Designer Themes**: Beautiful, pre-configured palettes including **Nord**, **Dracula**, and **Catppuccin** with a live-switch menu.
* **📟 Reactive UI Governor**: A smart rendering engine that prioritizes 60 FPS for user input while intelligently throttling background discovery updates to eliminate terminal flicker.

---

## ⌨️ Keybindings

| Key | Action |
| --- | --- |
| `Tab` | **Toggle selection** (Multi-select mode) |
| `Enter` | **Execute** command on selection (or print to stdout) |
| `Ctrl + p` | Toggle **Live Preview** & Metadata Sidebar |
| `Ctrl + t` | Open **Theme Selection** menu |
| `Ctrl + s` | Toggle **Case Sensitivity** |
| `Ctrl + r` | Toggle **Relative / Absolute** path display |
| `Ctrl + u` | **Clear** search input |
| `Up / Down` | Navigate list (scrolls Preview text when preview is open) |
| `Esc` | Quit |

---

## 🚀 Usage

### 🔍 Discovery

Search the current directory:

```bash
rfzf

```

Search your home directory with hidden files and preview enabled:

```bash
rfzf ~ --hidden --preview

```

### 🛠️ The Power of `--exec`

Pass selections into any terminal command. The `{}` placeholder is safely escaped for the shell.

**Interactive File Editing:**

```bash
rfzf --exec "nvim {}"

```

**Advanced Content Pipelines:**

```bash
rfzf --exec "grep 'TODO' {} | head -n 20"

```

**Bulk File Management:**

```bash
rfzf --exec "cp {} ~/backups/archive_$(date +%F)/"

```

---

## 🛠️ Installation

### Prerequisites

Ensure you have the [Rust toolchain](https://rustup.rs/) installed.

```bash
# Clone the repository
git clone https://github.com/skugge74/rust-fzf
cd rust-fzf

# Build the highly-optimized release binary
cargo build --release

# Move to your local bin path
sudo cp target/release/rust-fzf /usr/local/bin/rfzf

```

---

## ⚙️ Technical Details

* **TUI Engine**: Powered by `ratatui` for a stateful, modern terminal interface.
* **Fuzzy Logic**: `nucleo-matcher` provides high-performance, index-aware matching.
* **Syntax Highlighting**: `syntect` using Sublime-compatible definitions for professional-grade code previews.
* **Execution Layer**: Commands are spawned via `sh -c`, enabling full shell-native features (pipes, variables, and logic).

---

## 🎨 Themes Included

* **Nord**: Arctic-cold blues for focused work.
* **Dracula**: High-contrast vibrant colors for dark-room hackers.
* **Catppuccin**: A soothing, modern pastel aesthetic.

---

## ✅ Completed Roadmap

* [x] **Multi-threaded parallel file walker**
* [x] **Interactive shell integration (`--exec`)**
* [x] **Stateful multi-selection mode (`Tab`)**
* [x] **Live metadata sidebar (Size, Date, Perms)**
* [x] **Custom Theme Engine**

