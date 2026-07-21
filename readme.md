# Pretty Files v2.5.0

**Pretty Files** is a lightweight command-line file viewer written in Rust. It provides syntax-highlighted text viewing, automatic binary inspection, recursive file searching, file path listing, and optional debugging features.

Unlike traditional tools where you need to manually choose between text and binary modes, Pretty Files automatically detects file types and selects the appropriate viewer. Text files are displayed with syntax highlighting, while binary files are displayed as hexadecimal dumps with an ASCII preview.

It is designed as a simple alternative to tools like `cat`, `less`, and `hexdump` for quickly inspecting files directly from the terminal.

## Features

* Automatic text and binary file detection
* Syntax highlighting with automatic language detection
* Hexadecimal binary inspection with ASCII preview
* Recursive directory traversal
* Optional line numbers
* Automatic file headers when viewing multiple files
* Bare mode for printing file paths
* File filtering with `--ignore`
* Multiple file support
* Zero configuration — open files and start reading

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

You can also install it with Cargo:

```bash
cargo install --path .
```

## Usage

```text
pretty_files [COMMAND] [OPTIONS] <PATHS...>
```

## Commands

| Command   | Description                                |
| --------- | ------------------------------------------ |
| `help`    | Display help information.                  |
| `bare`    | Print file paths instead of file contents. |
| `version` | Display the current version.               |

## Automatic File Detection

Pretty Files automatically detects the type of each file.

Text files:

* Are displayed normally.
* Receive syntax highlighting when supported.
* Can optionally include line numbers.

Binary files:

* Are displayed as hexadecimal dumps.
* Include printable ASCII previews.
* Do not require a separate command.

Examples:

```bash
# Display a text file
pretty_files main.rs

# Display a binary file
pretty_files image.png

# Display mixed file types together
pretty_files main.rs image.png /bin/ls
```

## Text Display Options

| Flag                | Description                                        |
| ------------------- | -------------------------------------------------- |
| `-n`, `--numbers`   | Display line numbers.                              |
| `-r`, `--recursive` | Search directories recursively.                    |
| `-d`, `--debug`     | Display file names before contents.                |
| `-D`                | Disable automatic debug mode.                      |
| `-S`                | Disable syntax highlighting.                       |
| `-i`, `--ignore`    | Ignore a file or path. Can be used multiple times. |

## Bare Mode Options

| Flag                | Description                                        |
| ------------------- | -------------------------------------------------- |
| `-r`, `--recursive` | Search directories recursively.                    |
| `-i`, `--ignore`    | Ignore a file or path. Can be used multiple times. |

## Examples

### Read files

Display a file:

```bash
pretty_files main.rs
```

Display multiple files:

```bash
pretty_files main.rs Cargo.toml
```

Display line numbers:

```bash
pretty_files -n src/main.rs
```

Search recursively:

```bash
pretty_files -r src/
```

Disable syntax highlighting:

```bash
pretty_files -S README.md
```

Display mixed text and binary files:

```bash
pretty_files src/main.rs image.png /bin/ls
```

Ignore files:

```bash
pretty_files -r -i src/lib.rs src/
```

---

### List file paths

Display files inside a directory:

```bash
pretty_files bare src/
```

Search recursively:

```bash
pretty_files bare -r src/
```

Ignore files:

```bash
pretty_files bare -r -i src/lib.rs src/
```

## Notes

* Multiple files automatically enable debug headers.
* Use `-D` to disable automatic debug mode.
* File type detection is performed automatically.
* Syntax highlighting is selected based on the detected file type.
* Unknown extensions are displayed as plain text.
* Binary files are displayed as hexadecimal values with printable ASCII output.
* Bare mode only prints file paths and does not display file contents.
* Recursive mode expects directories as input.

## Dependencies

* **syntect** — Syntax highlighting and language detection.
* **walkdir** — Recursive directory traversal.
* **content_inspector** — Binary file detection.

## Contributing

Contributions are welcome.

If adding new commands, options, or behavior changes, keep the following synchronized:

* README
* Built-in help messages
* Man page

## License

This project is licensed under the GNU General Public License v3.0.

See the [LICENSE](./LICENSE) file for details.
