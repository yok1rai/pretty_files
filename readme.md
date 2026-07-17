# Pretty Files version 1.0

A simple command-line file viewer for the terminal, written in Rust. It prints file contents with syntax highlighting, optional line numbers, and recursive directory search — a lightweight alternative to `cat` when you want your code to actually look readable.

## Features

- Syntax highlighting for source files (powered by [`syntect`](https://github.com/trishume/syntect))
- Recursive directory search
- Zero-config: point it at a file and go

## Installation

### Arch Linux (AUR)

A `PKGBUILD` is included in the repo. Build and install it with `makepkg`:

```bash
git clone https://github.com/yok1rai/pretty_files.git
cd pretty_files
makepkg -si
```

### From source (Cargo)

```bash
git clone https://github.com/yok1rai/pretty_files.git
cd pretty_files
cargo build --release
```

The compiled binary will be available at `target/release/pretty_files`. Optionally, install it to your Cargo bin path:

```bash
cargo install --path .
```

## Usage

```
pretty_files [SPECIAL COMMANDS] [OPTIONS] <FILES...>
```

### Special commands

| Command  | Description       |
|----------|-------------------|
| `help`   | Show the help menu |
| `binary` | Read a binary file (not implemented yet) |

### Options

| Flag                | Description                                  |
|----------------------|-----------------------------------------------|
| `-n`, `--numbers`    | Show line numbers                             |
| `-r`, `--recursive`  | Recursively search directories for files      |
| `-d`, `--debug`      | Print file names as headers before their contents |
| `-S`                 | Disable syntax highlighting                   |
| `-D`                 | Disable automatic debug mode                  |

### Examples

```bash
# Print a single file with syntax highlighting
pretty_files file.txt

# Print multiple files
pretty_files file1.txt file2.txt

# Show line numbers
pretty_files -n file.txt

# Recursively print every file in a directory
pretty_files -r ./src

# Recursive with line numbers
pretty_files -r -n ./src

# Show the help menu
pretty_files help
```

## Notes

- When reading fewer than 3 files, **debug mode is enabled automatically** — filenames are printed as headers so you can tell where each file starts. Use `-D` to disable this behavior.
- Directories passed with `-r` are searched recursively; all files found (at any depth) are read and printed.
- Syntax highlighting is auto-detected based on file extension. Files without a recognized extension fall back to plain text.

## Dependencies

- [`syntect`](https://crates.io/crates/syntect) — syntax highlighting engine
- [`walkdir`](https://crates.io/crates/walkdir) — recursive directory traversal

## Contributing

Issues and pull requests are welcome. If you're adding a new flag or command, please update the `HELP` text in `src/lib.rs` to match.

## License

Licensed under the [GNU General Public License v3.0](LICENSE).
