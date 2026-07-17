# Changelog

All notable changes to **pretty_files** are documented in this file.

> **Note:** During development, each milestone corresponded to a single commit. The project was officially released as **v1.4.0**, making it the first public release.

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

## [1.1.0] — Bare Command Feature

### Added

- Added the `bare` special command to list file paths inside one or more directories.
- Added support for recursive directory traversal using the `-r` / `--recursive` option.

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
