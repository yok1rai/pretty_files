use std::{
    fs,
    path::{Path, PathBuf},
};

use syntect::{
    easy::HighlightLines,
    util::{LinesWithEndings, as_24_bit_terminal_escaped},
};

use crate::{help, utils, Command};


impl Command {
    fn highlight_path(&self, path: &Path, content: &str) -> Vec<String> {
        let syntax = self
            .syntax_set
            .find_syntax_for_file(path)
            .unwrap()
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        let theme = self
            .theme_set
            .themes
            .get("base16-ocean.dark")
            .unwrap();

        let mut highlighter = HighlightLines::new(syntax, theme);

        let mut highlighted = Vec::new();

        for line in LinesWithEndings::from(content) {
            let ranges = highlighter
                .highlight_line(line, &self.syntax_set)
                .unwrap();
            highlighted.push(
                as_24_bit_terminal_escaped(&ranges, false)
            );
        }
        highlighted
    }


    pub fn read_file(&self) -> std::io::Result<()> {
        let mut count_lines = false;
        let mut debug = false;
        let mut debug_allowed = true;
        let mut recursive = false;
        let mut syntax_highlight = true;

        let mut files = Vec::new();
        let mut ignored = Vec::new();


        if self.args[1].as_str() == "--help" {
            help::help(&self.args);
            return Ok(());
        }

        let mut args = self.args[1..].iter();

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-n" | "--numbers" => count_lines = true,

                "-d" | "--debug" => debug = true,

                "-r" | "--recursive" => recursive = true,

                "-S" => syntax_highlight = false,

                "-D" => debug_allowed = false,

                "-i" | "--ignore" => {
                    if let Some(name) = args.next() {
                        ignored.push(name.clone());
                    } else {
                        eprintln!("missing argument for --ignore");
                        return Ok(());
                    }

                }
                _ => files.push(PathBuf::from(arg)),
            }
        }

        if recursive {
            let directories = files.clone();
            files.clear();

            for directory in directories {
                for file in utils::recursive_search(directory, &ignored) {
                    files.push(file);
                }
            }
        }

        if files.len() > 1 && debug_allowed {
            debug = true;
        }

        for file in &files {
            let content = match fs::read_to_string(file) {
                Ok(content) => content,
                Err(e) => {
                    match e.kind() {
                        std::io::ErrorKind::IsADirectory => {
                            eprintln!(
                                "`{}` is a directory",
                                file.display()
                            );
                        }
                        std::io::ErrorKind::NotFound => {
                            eprintln!(
                                "`{}` does not exist",
                                file.display()
                            );
                        }
                        std::io::ErrorKind::PermissionDenied => {
                            eprintln!(
                                "You don't have permission to access `{}`",
                                file.display()
                            );
                        }
                        std::io::ErrorKind::InvalidData => {
                            eprintln!(
                                "`{}` is a binary file (try running with `binary` special command)",
                                file.display()
                            );
                        }
                        _ => {
                            eprintln!(
                                "`{}`: {}",
                                file.display(),
                                e
                            );
                        }
                    }
                    continue;
                }
            };
            let path = file.to_string_lossy();
            if ignored.iter().any(|i| path.ends_with(i)) {
                continue;
            }
            if debug {
                println!("\n{}:\n", file.display());
            }
            if syntax_highlight {
                let highlighted =
                    self.highlight_path(file.as_path(), &content);
                for (i, line) in highlighted.iter().enumerate() {
                    if count_lines {
                        print!("{:>4}. {}", i + 1, line);
                    } else {
                        print!("{line}");
                    }
                }
            } else {
                for (i, line) in content.lines().enumerate() {
                    if count_lines {
                        println!("{:>4}. {}", i + 1, line);
                    } else {
                        println!("{line}");
                    }
                }
            }
        }
        Ok(())
    }
}
