use std::{fs, path::PathBuf, process};

use content_inspector::{ContentType, inspect};
use syntect::{highlighting::ThemeSet, parsing::SyntaxSet};

#[derive(PartialEq)]
enum Mode {
    Normal,
    Bare,
}

pub mod bare;
pub mod binary;
pub mod help;
pub mod text;
pub mod utils;

pub struct Command {
    args: Vec<String>,
    pub(crate) syntax_set: SyntaxSet,
    pub(crate) theme_set: ThemeSet,

    debug: bool,
    recursive: bool,
    syntax_highlight: bool,
    count_lines: bool,
    mode: Mode,
}

impl Command {
    pub fn new(args: Vec<String>) -> Self {
        Self {
            args,
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
            debug: false,
            recursive: false,
            syntax_highlight: true,
            count_lines: false,
            mode: Mode::Normal,
        }
    }

    pub fn read(&mut self) {
        self.handle_special_commands();

        let (mut files, ignored) = self.parse_arguments();

        if self.recursive {
            files = self.expand_recursive(files, &ignored);
        }

        if files.len() > 1 {
            self.debug = true;
        }

        files.retain(|file| {
            let path = file.to_string_lossy();
            !ignored.iter().any(|i| path.ends_with(i))
        });

        match self.mode {
            Mode::Bare => {
                if self.recursive {
                    for file in files {
                        println!("{}", file.display());
                    }
                } else {
                    for directory in files {
                        if let Err(e) = self.bare_read(&directory) {
                            self.print_error(&directory, e);
                        }
                    }
                }
            }

            Mode::Normal => {
                for file in files {
                    match file.metadata() {
                        Ok(metadata) => {
                            if !metadata.is_file() {
                                eprintln!("`{}` is not a file", file.display());
                                continue;
                            }

                            self.read_path(&file);
                        }
                        Err(e) => {
                            self.print_error(&file, e);
                            continue;
                        }
                    }
                }
            }
        }
    }

    fn handle_special_commands(&mut self) {
        if let Some(command) = self.args.get(1) {
            match command.as_str() {
                "help" => {
                    self.help();
                    process::exit(0);
                }
                "version" => self.version(),
                "bare" => self.mode = Mode::Bare,
                _ => {}
            }
        }
    }

    fn parse_arguments(&mut self) -> (Vec<PathBuf>, Vec<String>) {
        let mut files = Vec::new();
        let mut ignored = Vec::new();

        let start = match self.args.get(1).map(String::as_str) {
            Some("bare" | "self" | "help" | "version") => 2,
            _ => 1,
        };

        let mut args = self.args[start..].iter();

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-d" | "--debug" => self.debug = true,
                "-r" | "--recursive" => self.recursive = true,
                "-S" => self.syntax_highlight = false,
                "-n" | "--numbers" => self.count_lines = true,

                "-i" | "--ignore" => {
                    if let Some(path) = args.next() {
                        ignored.push(path.clone());
                    } else {
                        eprintln!("missing argument for --ignore");
                        process::exit(1);
                    }
                }

                _ => files.push(PathBuf::from(arg)),
            }
        }

        (files, ignored)
    }

    fn expand_recursive(&self, files: Vec<PathBuf>, ignored: &[String]) -> Vec<PathBuf> {
        let mut result = Vec::new();

        for directory in files {
            result.extend(utils::recursive_search(directory, ignored));
        }

        result
    }

    fn read_path(&self, file: &PathBuf) {
        let bytes = match fs::read(file) {
            Ok(bytes) => bytes,
            Err(e) => {
                self.print_error(file, e);
                return;
            }
        };

        let result = if inspect(&bytes) == ContentType::BINARY {
            self.read_binary(file)
        } else {
            self.read_file(file)
        };

        if let Err(e) = result {
            self.print_error(file, e);
        }
    }

    fn print_error(&self, file: &PathBuf, e: std::io::Error) {
        use std::io::ErrorKind::*;

        match e.kind() {
            IsADirectory => {
                eprintln!("`{}` is a directory", file.display());
            }

            NotFound => {
                eprintln!("`{}` does not exist", file.display());
            }

            PermissionDenied => {
                eprintln!("You don't have permission to access `{}`", file.display());
            }

            InvalidData => {
                eprintln!("`{}` is a binary file (depreacted)", file.display());
            }

            NotADirectory => {
                eprintln!("`{}` is not a directory", file.display());
            }

            _ => {
                eprintln!("`{}`: {}", file.display(), e);
            }
        }
    }
}
