# 🚀 Rust-FZF: Blazingly Fast Fuzzy Finder

A high-performance, interactive terminal fuzzy finder built with **Rust**. It features multi-threaded file discovery, a reactive UI governor for zero-flicker rendering, and stateful syntax highlighting.

## ✨ Features

* **⚡ Parallel Enumeration**: Uses `ignore` (the engine behind `ripgrep`) to scan millions of files in parallel, respecting `.gitignore` and hidden file rules.
* **🎯 Smart-Case Matching**: Search is case-insensitive by default but can be toggled.
* **🎨 Live Theming**: Switch between **Nord**, **Dracula**, and **Catppuccin** on the fly with a dedicated UI menu.
* **📖 Stateful Preview**: Code previews with syntax highlighting powered by `syntect`, using a frame-aware cache for buttery-smooth scrolling.
* **📟 Reactive Governor**: Smart rendering logic that provides 60 FPS for user input while throttling background discovery to prevent terminal flicker.

---

## ⌨️ Keybindings

| Key | Action |
| :--- | :--- |
| `Ctrl + t` | Open **Theme Selection** menu |
| `Ctrl + p` | Toggle **File Preview** |
| `Ctrl + u` | Clear search input |
| `Ctrl + r` | Toggle relative filepath |
| `Ctrl + s` | Toggle case-sensitive|
| `Up / Down` | Navigate file list (or scroll Preview if open) |
| `Enter` | Select file and exit (prints path to stdout) |
| `Esc` | Quit |

---

## 🛠️ Installation

### Prerequisites
Ensure you have the Rust toolchain installed.

```bash
# Clone the repository
git clone [https://github.com/skugge74/rust-fzf](https://github.com/skugge74/rust-fzf)
cd rust-fzf

# Build for release
cargo build --release

# (Optional) Move to your path
sudo cp target/release/rust-fzf /usr/local/bin/rfzf
---

## 🚀 Usage

Scan the current directory:
```bash
rfzf

```

Scan a specific directory (e.g., your home folder) with hidden files enabled:

```bash
rfzf ~ --hidden --preview

```

### Integration with other tools

You can use `rfzf` to open files in your favorite editor:

```bash
nvim $(rfzf)

```

---

## 🎨 Themes Included

* **Nord**: A clean, arctic-blue aesthetic.
* **Dracula**: A high-contrast, vibrant dark theme.
* **Catppuccin**: A soothing, pastel-themed palette for modern terminals.

---

## ⚙️ Technical Details

* **TUI Framework**: Built on `ratatui` for the terminal interface.
* **Fuzzy Engine**: Powered by `nucleo`, designed for high-performance matching.
* **Concurrency**: Uses `ignore` for lock-free parallel file injection.
* **Syntax Highlighting**: Powered by `syntect` using Sublime-compatible definitions.


- [x] TODO: interactive shell integration: `rfzf --exec "nvim {}"`
- [x] TODO: multi selection mode
- [ ] TODO: file metadata sidebar
- [ ] TODO: fuzzy match highlighting
- [ ] TODO: --preview is useless now (only as an arg)
