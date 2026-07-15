use std::env;
use pretty_files::Command;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = match Command::new(args) {
        Ok(ok) => ok,
        Err(()) => process::exit(1)
    };
    match command.command_str().trim() {
        "read" => command.read_file().unwrap_or_else(|e| eprintln!("{e}")),
        _ => {
            eprintln!("invalid entry"); 
        }
    }
}

