# Pretty Files v2.0.0

**Pretty Files** is a lightweight command-line file viewer written in Rust. It provides syntax-highlighted text viewing, hexadecimal binary viewing, recursive directory traversal, optional line numbers, and automatic file headers, making it a convenient alternative to tools like `cat` and `hexdump` for everyday file inspection.

## Features

- Syntax highlighting with automatic language detection
- Hexadecimal binary viewer with ASCII preview
- Recursive directory traversal
- Optional line numbers
- Automatic file headers when viewing multiple files
- File path listing through the `bare` command
- File filtering with `--ignore`
- Zero configuration — point it at a file and start reading

## Installation

### Arch Linux (AUR)

A `PKGBUILD` is included in the repository.

```bash
git clone https://github.com/yok1rai/pretty_files.git
cd pretty_files
makepkg -si
```

### From Source

```bash
git clone https://github.com/yok1rai/pretty_files.git
cd pretty_files
cargo build --release
```

The compiled binary will be available at:

```text
target/release/pretty_files
```

Or install it directly with Cargo:

```bash
cargo install --path .
```

## Usage

```text
pretty_files [COMMAND] [OPTIONS] <PATHS...>
```

## Commands

| Command | Description |
|--------|-------------|
| `help` | Display the help menu. |
| `bare` | Print file paths without displaying file contents. |
| `binary` | Display binary files as hexadecimal dumps with an ASCII preview. |
| `version` | Display the installed version. |

## Read Mode Options

| Flag | Description |
|------|-------------|
| `-n`, `--numbers` | Display line numbers. |
| `-r`, `--recursive` | Search directories recursively. |
| `-d`, `--debug` | Print file names before displaying their contents. |
| `-D` | Disable automatic debug mode. |
| `-S` | Disable syntax highlighting. |
| `-i`, `--ignore` | Ignore a file or path. May be specified multiple times. |

## Binary Mode Options

| Flag | Description |
|------|-------------|
| `-r`, `--recursive` | Search directories recursively. |
| `-d`, `--debug` | Print file names before displaying their contents. |
| `-D` | Disable automatic debug mode. |
| `-i`, `--ignore` | Ignore a file or path. May be specified multiple times. |

## Bare Mode Options

| Flag | Description |
|------|-------------|
| `-r`, `--recursive` | Search directories recursively. |
| `-i`, `--ignore` | Ignore a file or path. May be specified multiple times. |

## Examples

### Read text files

```bash
# Display a file
pretty_files main.rs

# Display multiple files
pretty_files main.rs Cargo.toml

# Display line numbers
pretty_files -n src/main.rs

# Display every file recursively
pretty_files -r src/

# Disable syntax highlighting
pretty_files -S README.md
```

### View binary files

```bash
# Display a binary file
pretty_files binary image.png

# Display an executable
pretty_files binary /bin/ls

# Search recursively
pretty_files binary -r build/

# Ignore a file
pretty_files binary -r -i build/cache.bin build/
```

### List file paths

```bash
# List files
pretty_files bare src/

# List files recursively
pretty_files bare -r src/

# Ignore a file
pretty_files bare -r -i src/lib.rs src/
```

## Notes

- Automatic debug mode is enabled when displaying multiple files.
- Use `-D` to disable automatic debug mode.
- Syntax highlighting is selected automatically based on the detected file type.
- Files with unsupported or unknown extensions are displayed as plain text.
- Binary mode displays hexadecimal values alongside printable ASCII characters.
- The `-r` option expects directories as input.

## Dependencies

- **syntect** — Syntax highlighting and language detection.
- **walkdir** — Recursive directory traversal.

## Contributing

Contributions are welcome. If you add or modify commands or options, please keep the README, man page, and built-in help messages synchronized.

## License

This project is licensed under the GNU General Public License v3.0. See the `LICENSE` file for details.
