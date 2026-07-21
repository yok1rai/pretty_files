use std::{env, process};
use pretty_files::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("no arguments given");
        process::exit(1); 
    }
    let mut command = Command::new(args);
    command.read();
}
