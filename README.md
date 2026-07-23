# Pretty Files v3.0.6

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

### From [crates.io](https://crates.io/crates/pretty_files)

Install **Pretty Files** using Cargo:

```bash
cargo install pretty_files
```

This installs the `pf` command:

```bash
pf file.txt
```

### Arch Linux

A `PKGBUILD` is included in the repository:

```bash
git clone https://github.com/yok1rai/pretty_files.git
cd pretty_files
makepkg -si
```

The package also installs the manual page:

```bash
man pf
```

### NixOS / Nix

Run directly without installing:

```bash
nix run github:yok1rai/pretty_files
```

Or run directly from a local clone:

```bash
nix run
```

#### Install to profile

```bash
nix profile install github:yok1rai/pretty_files
```

#### Add to a Flake configuration

```nix
{
  inputs = {
    pretty_files.url = "github:yok1rai/pretty_files";
  };

  outputs = { self, nixpkgs, pretty_files }: {
    # ...
    # Access the package via:
    # pretty_files.packages.${system}.default
  };
}
```

### From Source

#### Requirements

- Rust toolchain

#### Build from Source

```bash
git clone https://github.com/yok1rai/pretty_files.git
cd pretty_files
cargo build --release
```

The compiled binary will be available at:

```text
target/release/pf
```

You can also install it locally:

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
pf main.rs

# Display a binary file
pf image.png

# Display mixed file types together
pf main.rs image.png /bin/ls
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
pf main.rs
```

Display multiple files:

```bash
pf main.rs Cargo.toml
```

Display line numbers:

```bash
pf -n src/main.rs
```

Search recursively:

```bash
pf -r src/
```

Disable syntax highlighting:

```bash
pf -S README.md
```

Display mixed text and binary files:

```bash
pf src/main.rs image.png /bin/ls
```

Ignore files:

```bash
pf -r -i src/lib.rs src/
```

---

### List file paths

Display files inside a directory:

```bash
pf bare src/
```

Search recursively:

```bash
pf bare -r src/
```

Ignore files:

```bash
pf bare -r -i src/lib.rs src/
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
