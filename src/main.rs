use std::env;
use pretty_files::{help, Command};

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = Command::new(args);

    match command.first_arg() {
        Some(arg) => {
            match arg.trim() {
                "help" => help::help(command.args()),
                "binary" => command.read_binary().unwrap(),
                "bare" => command.bare_read().unwrap(),
                "version" => help::version(),
                _ => command.read_file().unwrap(),
            }
        }

        None => {
            eprintln!("no argument given! Run `help`");
        }
    }
}
