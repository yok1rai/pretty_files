# Pretty Files v1.3

**Pretty Files** is a lightweight command-line file viewer written in Rust. It displays files with syntax highlighting, optional line numbers, and recursive directory support, providing a simple and convenient alternative to tools like `cat` for viewing source code.

## Features

* Syntax highlighting with automatic language detection
* Recursive directory traversal
* Optional line numbers
* Automatic debug headers when viewing multiple files
* File path listing through the `bare` command
* Zero configuration — point it at a file and start reading

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

The compiled binary will be available at:

```text
target/release/pretty_files
```

You can also install it directly with Cargo:

```bash
cargo install --path .
```

## Usage

```text
pretty_files [COMMAND] [OPTIONS] <PATHS...>
```

## Commands

| Command  | Description                                                                     |
| -------- | ------------------------------------------------------------------------------- |
| `help`   | Display the general help menu.                                                  |
| `bare`   | Print file paths from one or more directories without displaying file contents. |
| `binary` | Read binary files *(currently not implemented).*                                |

## Options

| Flag                | Description                             |
| ------------------- | --------------------------------------- |
| `-n`, `--numbers`   | Display line numbers.                   |
| `-r`, `--recursive` | Search directories recursively.         |
| `-d`, `--debug`     | Display filenames before file contents. |
| `-D`                | Disable automatic debug mode.           |
| `-S`                | Disable syntax highlighting.            |

## Examples

```bash
# Display a single file
pretty_files file.txt

# Display multiple files
pretty_files file1.txt file2.txt

# Display a file with line numbers
pretty_files -n file.txt

# Recursively display files inside a directory
pretty_files -r ./src

# Recursive reading with line numbers
pretty_files -r -n ./src

# Display the help menu
pretty_files help

# List files inside a directory
pretty_files bare src

# Recursively list files
pretty_files bare -r src
```

## Notes

* When multiple files are displayed, debug mode is enabled automatically to separate each file's output with a filename header.
* Use `-D` to disable automatic debug headers.
* Syntax highlighting is detected automatically using each file's extension.
* Files with unsupported or unknown extensions are displayed as plain text.
* The `-r` option expects directories and searches all files inside them recursively.

## Dependencies

* **syntect** — Provides syntax highlighting and language detection.
* **walkdir** — Provides recursive directory traversal.

## Contributing

Contributions are welcome! If you add or modify commands or options, please keep the built-in help messages in `src/lib.rs` synchronized with this documentation.

## License

This project is licensed under the GNU General Public License v3.0. See the `LICENSE` file for details.
