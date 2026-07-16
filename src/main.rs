use std::env;
use pretty_files::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = Command::new(args);
    match command.first_arg() {
        Some(arg) => {
            match arg.trim() {
                "help" => Command::help(),
                "b" | "binary" => command.binary(),
                _ => command.read_file().unwrap_or_else(|e| {
                    eprintln!("{e}");
                })
            }
        },
        None => {
            eprintln!("no argument given! Run `help`")
        }
    }
}

