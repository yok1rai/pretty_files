use std::fs;
use walkdir::WalkDir;
use std::path::{Path, PathBuf};

pub struct Command {
    args: Vec<String>,
    command: String,
}

const HELP: &str = r#"pretty_files - Simple file viewer

USAGE:
    pretty_files <COMMAND> [OPTIONS] <FILES...>

COMMANDS:
    read                Read and print one or more files.
    help                Show this help message.

OPTIONS:
    -n, --numbers       Show line numbers.
    -r, --recursive     Recursively search directories for files.
    -d, --debug         Print file names before their contents.
    !d, !debug          Disable automatic debug mode.

EXAMPLES:
    pretty_files read file.txt
    pretty_files read file1.txt file2.txt
    pretty_files read -n file.txt
    pretty_files read -r ./src
    pretty_files read -r -n ./src
    pretty_files help

NOTES:
    • When reading fewer than 3 files, debug mode is enabled automatically.
    • Directories passed with -r are searched recursively.
"#;

impl Command {
    pub fn new(args: Vec<String>) -> Result<Self, ()> {
        if args.len() < 2 {
            eprintln!("you must specify the command");
            Err(())
        } else {
            let command = args[1].clone();
            Ok(Self { args, command } )
        }
    }
    pub fn command_str(&self) -> &str {
        &self.command
    }
    fn recursive_search(root: impl AsRef<Path>) -> Vec<PathBuf> {
        WalkDir::new(root)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|result| result.file_type().is_file())
            .map(|entry| entry.into_path())
            .collect()

    }
    pub fn read_file(&self) -> std::io::Result<()> {
        if self.args.len() < 3 {
            eprintln!("you must specify the path");
            return Ok(());
        }
        let mut count_lines = false;
        let mut debug = false;
        let mut debug_allowed = true;
        let mut recursive = false;
        let mut files = Vec::new();

        for arg in &self.args[2..] {
            match arg.as_str() {
                "-n" | "--numbers" => count_lines = true,
                "-d" | "--debug" => debug = true,
                "-r" | "--recursive" => recursive = true,
                "!d" | "!debug" => debug_allowed = false,
                _ => files.push(PathBuf::from(arg)),
            }
        }
        if files.len() < 3 && debug_allowed {
            debug = true;
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
            for file in &files {
                let content: String = fs::read_to_string(file)?;
                if debug {
                    println!("\n=== {} ===\n", file.display());
                }

                if count_lines {
                    for (i, line) in content.lines().enumerate() {
                        println!("{}: {}", i + 1, line);
                    }
                } else {
                    println!("{content}");
                }
            }

        Ok(())
    }
    pub fn help() {
        println!("{HELP}");
    }

}
