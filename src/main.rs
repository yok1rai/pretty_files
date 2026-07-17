use std::env;
use pretty_files::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = Command::new(args);
    match command.first_arg() {
        Some(arg) => {
            match arg.trim() {
                "help" => command.help(),
                "binary" => command.binary(),
                "bare" => command.bare_read().unwrap(),
                "version" => Command::version(),
                _ => command.read_file().unwrap()
            }
        },
        None => {
            eprintln!("no argument given! Run `help`")
        }
    }
}

