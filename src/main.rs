use pretty_files::Command;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("no arguments given");
        process::exit(1);
    }
    let mut command = Command::new(args);
    command.read();
}
