use std::{fs, process};
use walkdir::WalkDir;
use std::path::{Path, PathBuf};
use syntect::{
    easy::HighlightLines, highlighting::ThemeSet, parsing::SyntaxSet, util::{LinesWithEndings, as_24_bit_terminal_escaped}
};

enum CommandHelp {
    Read,
    Bare,
    Common,
}

pub struct Command {
    args: Vec<String>,
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
}

impl Command {
    pub fn new(args: Vec<String>) -> Self {
        let ps = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();

        Self { args, syntax_set: ps, theme_set: ts }
    }
    fn return_mode(&self) -> Result<CommandHelp, ()> {
        if self.args.len() < 2 {
            return Ok(CommandHelp::Common);
        }
        let command = &self.args[1];
        match command.as_str() {
            "bare" => Ok(CommandHelp::Bare),
            "help" => Ok(CommandHelp::Common),
            _ => Ok(CommandHelp::Read),
        }
    }

    fn recursive_search(root: impl AsRef<Path>) -> Vec<PathBuf> {
        WalkDir::new(root)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|result| result.file_type().is_file())
            .map(|entry| entry.into_path())
            .collect()
    }
    pub fn first_arg(&self) -> Option<&str> {
        self.args.get(1).map(String::as_str)
    }
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
            highlighted.push(as_24_bit_terminal_escaped(&ranges, false));
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

        if self.args[1].as_str() == "--help" {
            self.help();
            return Ok(());
        }

        for arg in &self.args[1..] {
            match arg.as_str() {
                "-n" | "--numbers" => count_lines = true,
                "-d" | "--debug" => debug = true,
                "-r" | "--recursive" => recursive = true,
                "-S" => syntax_highlight = false,
                "-D" => debug_allowed = false,
                _ => files.push(PathBuf::from(arg)),
            }
        }

        if recursive {
            let directories = files.clone();
            files.clear();
            for directory in directories {
                for file in Command::recursive_search(directory) {
                    files.push(file);
                }
            }
        }
        if files.len() > 1 && debug_allowed {
            debug = true;
        }

        for file in &files {
            let content = fs::read_to_string(file)?;

            if debug {
                println!("\n=== {} ===\n", file.display());
            }

            if syntax_highlight {
                let highlighted = self.highlight_path(file.as_path(), &content);

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
    pub fn bare_read(&self) -> std::io::Result<()> {
        let mut recursive = false;
        let mut directories = Vec::new();
        if self.args[2].as_str() == "--help" {
            self.help();
            return Ok(());
        }
        for arg in &self.args[2..] {
            match arg.as_str() {
                "-r" | "--recursive" => recursive = true,
                _ => directories.push(PathBuf::from(arg)),
            }
        }

        for path in &directories {
            if !path.is_dir() {
                eprintln!("{} is not a directory", path.display());
                return Ok(());
            }
        }

        if recursive {
            let tmp_directories = std::mem::take(&mut directories);

            for directory in tmp_directories {
                directories.extend(Command::recursive_search(directory));
            }

            for file in &directories {
                println!("{}", file.display());
            }
            return Ok(());
        }

        for directory in &directories {
            for entry in fs::read_dir(directory)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_file() {
                    println!("{}", entry.path().to_string_lossy());
                }
            }
        }

        Ok(())
    }
    pub fn help(&self) {
        let command = match self.return_mode() {
            Ok(cmd) => cmd,
            Err(()) => process::exit(1),
        };
        match command {
            CommandHelp::Bare => {
                const HELP: &str = r#"pretty_files 1.2.0 - Bare Mode

USAGE:
    pretty_files bare [OPTIONS] <DIRECTORIES...>

DESCRIPTION:
    Prints file paths without displaying file contents.
    Useful for piping into other commands or shell scripts.

OPTIONS:
    -r, --recursive     Search directories recursively

EXAMPLES:
    pretty_files bare src/
    pretty_files bare src/ target/
    pretty_files bare -r src/
    pretty_files bare -r .

Shell example:

    for file in $(pretty_files bare -r .); do
        echo "=== $file ==="
        cat "$file"
    done

NOTES:
    • Only directories are accepted as input.
    • With -r, files inside all subdirectories are included."#;
                println!("{HELP}");
            },
            CommandHelp::Common => {
                const HELP: &str = r#"pretty_files 1.2.0 - Simple File Viewer

USAGE:
    pretty_files [COMMAND] [OPTIONS] <PATHS...>

COMMANDS:
    help                Show this help menu
    bare                Print file paths instead of file contents
    binary              View binary files (coming soon)

EXAMPLES:
    pretty_files --help
    pretty_files bare --help
    pretty_files binary --help

NOTES:
    • for more information add `--help` after special commands
"#;
                println!("{HELP}");


            },
            CommandHelp::Read => {
                const HELP: &str = r#"pretty_files 1.2.0 - Read Mode

USAGE:
    pretty_files [OPTIONS] <FILES...>

OPTIONS:
    -n, --numbers       Display line numbers
    -r, --recursive     Search directories recursively
    -d, --debug         Print file names before their contents
    -D                  Disable automatic debug mode
    -S                  Disable syntax highlighting

EXAMPLES:
    pretty_files main.rs
    pretty_files src/lib.rs Cargo.toml
    pretty_files -n src/main.rs
    pretty_files -r src/
    pretty_files -r src/ docs/

NOTES:
    • Automatic debug mode is enabled when reading multiple files.
    • The -r option expects directories as input.
    • Files discovered recursively are treated like normal input files."#;
                println!("{HELP}");
            }
        }

    }
    pub fn binary(&self) {
        println!("not implemented yet");
    }
}
