# Changelog

All notable changes to **pretty_files** are documented in this file.

> version **`3.0.0`** is the final release

## [3.0.0] — Final Release

### Added

- Released on [crates.io](https://crates.io/crates/pretty_files)
- Flake support

### Changed

- `--ignored` renamed to `--ignored`

## [2.5.1] — Tool renamed

### Changed

* renamed utility name from `pretty_files` to `pf` for calling

```bash
# old way
pretty_files src/
# new way
pf src/
```

## [2.5.0] — Automatic File Detection and Unified Reader

### Added

* Added automatic file type detection using `content_inspector`.
* Added unified file reading through the default reader instead of requiring separate text and binary modes.
* Added support for displaying text and binary files together in a single command.
* Added help topics for specific sections:
  * `pretty_files help text`
  * `pretty_files help bare`
* Added centralized command argument parsing and state handling.
* Added automatic binary file inspection with hexadecimal output and ASCII previews.

### Changed

* Removed the need to manually select binary mode when reading files.
* Reworked the file reading pipeline:
  * Files are now inspected automatically.
  * Text files are displayed with syntax highlighting when supported.
  * Binary files are displayed as hexadecimal dumps.
* Replaced the old separate binary command workflow with automatic detection.
* Refactored command handling into a single `read()` execution flow.
* Improved error handling by centralizing I/O error reporting.
* Simplified text and binary readers by making them responsible only for displaying already-selected file types.
* Updated the help system to use topic-based documentation.
* Updated the man page and documentation to reflect the new automatic detection system.

### Removed

* Removed the requirement to use the `binary` command for binary files.
* Removed the old binary command workflow.
* Removed duplicated argument parsing logic from individual modes.

### Dependencies

* Added `content_inspector` for detecting binary files.

---

## [2.0.0] — Binary Mode Added

### Added

* binary mode

### Changed

* file headers is now "file_name:" not "=== file_name ==="

---

## [1.6.0] — Improved Modularity

### Added

* Split command functionality into separate modules for better organization and maintainability.
* Added dedicated modules for:
  * Help system
  * Bare command handling
  * Utility functions
* Moved recursive directory searching into the utility module.
* Prepared the project structure for future features, including binary and hex file viewing.

### Changed

* Refactored the `Command` implementation by removing unrelated functionality from `lib.rs`.
* Improved code organization by separating CLI features into independent modules.
* Updated internal function calls to use shared modules instead of methods inside `Command`.
* Simplified the main command handler by keeping only command dispatch logic.

### Removed

* Removed the old integrated help system from `lib.rs`.
* Removed duplicated command-related code from the main command implementation.

---

## [1.5.0] — Added Ignore Option

### Added

* Added the `-i` / `--ignore` option to exclude specified files or paths from processing. The option may be used multiple times.
* Added the `version` command to display the current version of **pretty_files**.

---
## [1.4.0] — Better Error Handling

### Added

* Added descriptive error messages for common I/O errors:
  IsADirectory: displayed when attempting to read a directory.
  NotFound: displayed when a file or directory does not exist.
  PermissionDenie: displayed when access to a file or directory is denied.
  InvalidData — displayed when attempting to read binary or otherwise non-UTF-8 data as text.

### Fixed

* Fixed a panic caused by invoking `pretty_files bare` without any additional arguments.

---

## [1.3.0] — Man page support

### Added

* Added man page support

## [1.3.0] — Improved help menu

### Added

* Added command-specific help menus for better readability.
* Added dedicated help sections for:
  * General usage
  * File reading mode
  * `bare` command usage

### Changed

* Reworked the help system from a single large help page into a structured command-based help interface.
* Improved help output by showing only relevant options and examples for the selected command.
* Updated command documentation to match the current CLI structure.

---

## [1.1.0] — Bare Command Feature

### Added

- Added the `bare` special command to list file paths inside one or more directories.
- Added support for recursive directory traversal using the `-r` / `--recursive` option.

---

## [1.0.0] — First public release

### Added

* Comprehensive project documentation, including installation instructions, feature descriptions, command reference, and usage examples.
* GPL-3.0 license.

---

## [0.9.0] — Arch Linux Packaging improvements

### Changed

* Switched from the external **Oniguruma** regex library to a pure Rust implementation, simplifying the build process.
* Updated the Arch Linux `PKGBUILD` to build directly from the latest GitHub commit instead of a fixed release.
* Updated package metadata to reflect the project's licensing.

### Removed

* Removed the requirement to install the `oniguruma` system library before building the project.

---

## [0.8.0] — Binary command foundation

### Added

* Added a `binary` special command as the foundation for future binary file viewing support.

### Notes

* The command is currently a placeholder and reports that the feature has not yet been implemented.

---

## [0.7.5] — Performance optimization

### Changed

* Optimized syntax highlighting by loading syntax definitions and themes only once per execution instead of once per file.
* Reduced overhead when reading multiple files.
* Improved overall responsiveness when processing large batches of files.

---

## [0.7.0] — Syntax highlighting and command overhaul

### Added

* Syntax highlighting based on file type.
* Built-in `help` special command with usage information.
* Dedicated section for special commands within the CLI.

### Changed

* Reading files became the default behavior. Files can now be opened directly:

```text
pretty_files file.txt
```

instead of:

```text
pretty_files read file.txt
```

* Renamed utility commands to **special commands** for a clearer and more consistent command structure.
* Improved the overall command-line interface by reducing unnecessary boilerplate.

### Removed

* Removed the `read` command. Reading files is now the default behavior.

---

## [0.6.0] — Arch Linux packaging

### Added

* Added a `PKGBUILD`, allowing the project to be built with `makepkg` and distributed through the AUR.
* Added support for installing the application as a standard Arch Linux package.

---

## [0.5.0] — Recursive directory reading

### Added

* Added the `-r` / `--recursive` option to recursively read every file inside a directory and its subdirectories.
* Added recursive directory traversal using `walkdir`.

---

## [0.4.0] — Debug mode

### Added

* Added the `-d` / `--debug` option to display filenames before file contents.
* Added the `-D` / `--no-debug` option to explicitly disable automatic debug headers.
* Added automatic debug headers when reading fewer than three files.

### Changed

* Refactored the internal output pipeline to eliminate duplicated printing logic.
* Simplified the implementation by sharing the same rendering logic between numbered and non-numbered output.

---

## [0.3.0] — Line numbering

### Added

* Added the `-n` / `--numbers` option to display line numbers alongside file contents.

---

## [0.2.0] — Multiple file support

### Added

* Added support for reading multiple files in a single command, similar to the Unix `cat` utility.
* Improved output formatting when multiple files are displayed.

---

## [0.1.0] — Basic file reading

### Added

* Implemented basic file reading.
* Added the `read` command for reading text files.
* Displayed the filename before printing each file's contents.
* Added basic error handling for invalid or inaccessible file paths.

---

## [0.0.1] — Initial project setup

### Added

* Created the initial Cargo project.
* Added the first application skeleton.
* Set up the project's command-line interface.
* Established the initial project structure for future development.
