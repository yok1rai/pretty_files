# Pretty Files v1.1

**Pretty Files** is a lightweight command-line file viewer written in Rust. It displays files with syntax highlighting, optional line numbers, and recursive directory support, making it a convenient alternative to `cat` for viewing source code.

## Features

* Syntax highlighting with automatic language detection
* Recursive directory traversal
* Optional line numbers
* Automatic debug headers when viewing multiple files
* Zero configuration — just point it at a file and go

## Installation

### Arch Linux (AUR)

A `PKGBUILD` is included in the repository.

```bash
git clone https://github.com/yok1rai/pretty_files.git
cd pretty_files
makepkg -si
```

### From source

```bash
git clone https://github.com/yok1rai/pretty_files.git
cd pretty_files
cargo build --release
```

The compiled binary will be located at:

```text
target/release/pretty_files
```

Or install it directly with Cargo:

```bash
cargo install --path .
```

## Usage

```text
pretty_files [SPECIAL COMMAND] [OPTIONS] <FILES...>
```

## Special Commands

| Command  | Description                                                                      |
| -------- | -------------------------------------------------------------------------------- |
| `help`   | Display the built-in help menu.                                                  |
| `bare`   | List files contained in one or more directories without printing their contents. (new feature) |
| `binary` | Read binary files *(currently not implemented).*                                 |

## Options

| Flag                | Description                                         |
| ------------------- | --------------------------------------------------- |
| `-n`, `--numbers`   | Display line numbers.                               |
| `-r`, `--recursive` | Search directories recursively.                      |
| `-d`, `--debug`     | Display filenames as headers before their contents. |
| `-D`                | Disable automatic debug mode.                       |
| `-S`                | Disable syntax highlighting.                        |

## Examples

```bash
# Print a single file
pretty_files file.txt

# Print multiple files
pretty_files file1.txt file2.txt

# Print a file with line numbers
pretty_files -n file.txt

# Print every file inside a directory recursively
pretty_files -r ./src

# Recursive output with line numbers
pretty_files -r -n ./src

# Display the help menu
pretty_files help

# List every file inside a directory
pretty_files bare src

# List every file recursively
pretty_files bare -r src
```

## Notes

* When viewing fewer than **three files**, debug mode is enabled automatically so each file is preceded by its filename. Use `-D` to disable this behavior.
* Syntax highlighting is selected automatically based on each file's extension.
* Files with unrecognized extensions are displayed as plain text.
* Directories supplied with `-r` are searched recursively.

## Dependencies

* **syntect** — Syntax highlighting.
* **walkdir** — Recursive directory traversal.

## Contributing

Contributions are welcome! If you add or modify a command or option, please keep the built-in help text in `src/lib.rs` synchronized with the documentation.

## License

This project is licensed under the GNU General Public License v3.0. See the `LICENSE` file for details.
