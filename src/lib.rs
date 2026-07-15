use std::{fs};

pub struct Command {
    args: Vec<String>,
    command: String,
}

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
    pub fn read_file(&self) -> std::io::Result<()> {
        if self.args.len() < 3 {
            eprintln!("you must specify the path");
            return Ok(());
        }
        let mut count_lines = false;
        let mut debug = false;
        let mut debug_allowed = true;
        let mut files = Vec::new();

        for arg in &self.args[2..] {
            match arg.as_str() {
                "-n" | "--numbers" => count_lines = true,
                "-d" | "--debug" => debug = true,
                "-!d" | "!--debug" => debug_allowed = false,
                _ => files.push(arg),
            }
        }
        if files.len() < 3 && debug_allowed {
            debug = true;
        }
            for file in files {
                if debug {
                    println!("\n=== {file} ===\n");
                }
                let content = fs::read_to_string(file)?;

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
}
