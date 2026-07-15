use std::env;
use pretty_files::Commands;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {

        let command = Commands::new(args);

        match command.args[1].trim() {
            "read" => command.read_file().unwrap_or_else(|e| eprintln!("{}", e)),
            _ => {
                eprintln!("invalid operation");
            }
        }
    } else {
        eprintln!("not enough arguments");
    }
}
