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
        let mut files = Vec::new();

        for arg in &self.args[2..] {
            match arg.as_str() {
                "-n" => count_lines = true,
                _ => files.push(arg),
            }
        }

        if count_lines {
            for file in files {
                println!("\n=== {file} ===\n");
                let content = fs::read_to_string(file)?;
                for (i, line) in content.lines().enumerate() {
                    println!("{}: {}", i + 1, line);
                }
            }
        } else {
            for file in files {
                println!("\n=== {file} ===\n");
                let content = fs::read_to_string(file)?;
                println!("{content}");
            }
        }

        Ok(())
    }
}
