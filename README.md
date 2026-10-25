<div align="center">

# rfzf

**A fast, good-looking fuzzy file finder for the terminal, written in Rust.**

Type a few letters, see matching files instantly, preview them with syntax highlighting, and run any shell command on what you pick.

[![Rust](https://img.shields.io/badge/rust-2024_edition-orange?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS-lightgrey)

</div>

---

## Why rfzf?

`rfzf` is a single-binary file picker in the spirit of [fzf](https://github.com/junegunn/fzf), focused on files. It walks your directory tree in parallel, feeds results into the same matching engine that powers the [Helix](https://helix-editor.com/) editor, and keeps the interface responsive while thousands of paths are still streaming in.

- **Fast discovery.** Directories are scanned on up to 8 threads with the [`ignore`](https://crates.io/crates/ignore) crate (the walker behind ripgrep). `.gitignore`, `.ignore` and hidden files are respected by default.
- **Fuzzy matching with [`nucleo`](https://crates.io/crates/nucleo).** Results update as you type, and narrowing a query reuses the previous match set instead of starting over.
- **Live preview.** A preview pane shows the first 300 lines of the selected file with syntax highlighting, plus a sidebar with its size, last-modified time and Unix permissions.
- **Run commands on your picks.** `--exec "nvim {}"` opens the file in your editor; any shell syntax works, including pipes and redirects.
- **Multi-select.** Mark several files with `Tab` and handle them all at once.
- **Themes.** Nord, Dracula and Catppuccin are built in and can be switched from inside the app.
- **No flicker.** Keystrokes redraw immediately, while background updates from the file walker are batched to about 30 fps.

## Installation

You need a [Rust toolchain](https://rustup.rs/) (1.85 or newer, for the 2024 edition).

**With cargo:**

```bash
cargo install --git https://github.com/CtxtArc/rfzf
```

**From source:**

```bash
git clone https://github.com/CtxtArc/rfzf
cd rfzf
cargo build --release
sudo cp target/release/rfzf /usr/local/bin/
```

> [!TIP]
> File-type icons use [Nerd Font](https://www.nerdfonts.com/) glyphs. Without a Nerd Font in your terminal they show up as empty boxes; everything else works normally.

## Usage

```text
rfzf [OPTIONS] [PATH]
```

| Option | Description |
| --- | --- |
| `PATH` | Directory to search (default: current directory) |
| `-H`, `--hidden` | Include hidden files and directories |
| `-p`, `--preview` | Start with the preview pane open |
| `-r`, `--relative` | Show paths relative to `PATH` |
| `-s`, `--sensitive` | Start in case-sensitive mode |
| `-e`, `--exec <CMD>` | Command to run on each selected file; `{}` is replaced by the path |
| `-h`, `--help` | Print help |
| `-V`, `--version` | Print version |

### Examples

Search the current directory:

```bash
rfzf
```

Search your home directory, including dotfiles, with the preview open:

```bash
rfzf ~ --hidden --preview
```

Open the chosen file in Neovim:

```bash
rfzf --exec "nvim {}"
```

Use shell pipelines:

```bash
rfzf --exec "grep -n 'TODO' {} | head -n 20"
```

Back up several files at once (select them with `Tab`, then press `Enter`):

```bash
rfzf --exec "cp {} ~/backups/"
```

### How `--exec` works

- `{}` is replaced by the selected path, wrapped in single quotes and escaped. Paths with spaces or quotes are safe.
- The command runs through `sh -c`, so pipes, redirects, `&&` and `$(...)` all work.
- With several files selected, the command runs **once per file**, one after another.
- Without `--exec`, pressing `Enter` prints the selected path(s) once the interface closes.

## Keybindings

| Key | Action |
| --- | --- |
| *typing* | Update the search query |
| `Up` / `Down` | Move through the results (scroll the preview while it is open) |
| `Tab` / `Shift+Tab` | Select or deselect the highlighted file |
| `Enter` | Run `--exec` on the selection, or print it |
| `Ctrl+P` | Show or hide the preview pane |
| `Ctrl+T` | Open the theme menu (`Up`/`Down` to pick, `Enter` to apply) |
| `Ctrl+S` | Switch between case-insensitive and case-sensitive matching |
| `Ctrl+R` | Switch between relative and absolute paths |
| `Ctrl+U` | Clear the query |
| `Backspace` | Delete the last character |
| `Esc` | Quit (or close the theme menu) |

The footer shows the current selection count and the state of each toggle.

## How it works

```text
┌──────────────┐  paths   ┌────────────────┐ snapshot ┌──────────────────┐
│ ignore walker│ ───────▶ │ nucleo matcher │ ───────▶ │ ratatui renderer │
│ (≤ 8 threads)│ injector │ (background)   │          │ + syntect preview│
└──────────────┘          └────────────────┘          └──────────────────┘
                                   ▲                           │
                                   └──── query from keyboard ──┘
```

1. A background thread walks the tree with `ignore`'s parallel walker and pushes file paths into nucleo's injector.
2. nucleo matches in the background; the UI loop calls `tick()` and reads a snapshot of the current results.
3. The render loop draws right away on user input, about every 33 ms while results are still changing, and every 250 ms when idle.
4. Query changes are debounced by 20 ms. When the new query extends the old one, nucleo only refines the existing matches.
5. The preview is highlighted with `syntect` and cached until the selection changes.

## Built with

| Crate | Role |
| --- | --- |
| [`ratatui`](https://ratatui.rs/) + [`crossterm`](https://crates.io/crates/crossterm) | Terminal UI and input |
| [`nucleo`](https://crates.io/crates/nucleo) | Fuzzy matching engine |
| [`ignore`](https://crates.io/crates/ignore) | Parallel, gitignore-aware directory walking |
| [`syntect`](https://crates.io/crates/syntect) | Syntax highlighting in the preview |
| [`clap`](https://crates.io/crates/clap) | Command-line parsing |

## Roadmap

- [ ] Draw the TUI on `/dev/tty` so the output can be piped (`rfzf | xargs ...`, `vim $(rfzf)`)
- [ ] Highlight matched characters in results
- [ ] Smart-case matching mode
- [ ] Configurable themes and keybindings
- [ ] Directory search mode

## License

Released under the [MIT License](LICENSE).
