const VERSION: &str = "v1.5.0";

enum CommandHelp {
    Read,
    Bare,
    Common,
}

fn return_mode(args: &[String]) -> CommandHelp {
    if args.len() < 2 {
        return CommandHelp::Common;
    }

    match args[1].as_str() {
        "bare" => CommandHelp::Bare,
        "help" => CommandHelp::Common,
        _ => CommandHelp::Read,
    }
}

pub fn help(args: &[String]) {
        let command = return_mode(args);
        match command {
            CommandHelp::Bare => {
                let help: String = format!(r#"pretty_files {} - Bare Mode

USAGE:
    pretty_files bare [OPTIONS] <DIRECTORIES...>

DESCRIPTION:
    Prints file paths without displaying file contents.
    Useful for piping into other commands or shell scripts.

OPTIONS:
    -r, --recursive     Search directories recursively
    -i  --ignore        Ignore the following file

EXAMPLES:
    pretty_files bare src/
    pretty_files bare src/ target/
    pretty_files bare -r src/
    pretty_files bare -r .
    pretty_files bare -r src/ -i src/utils.rs

Shell example:

    for file in $(pretty_files bare -r .); do
        echo "=== $file ==="
        cat "$file"
    done

NOTES:
    • Only directories are accepted as input.
    • With -r, files inside all subdirectories are included."#, VERSION);
                println!("{help}");
            }
            CommandHelp::Common => {
                let help: String = format!("pretty_files {} - Simple File Viewer

USAGE:
    pretty_files [COMMAND] [OPTIONS] <PATHS...>

COMMANDS:
    help                Show this help menu
    bare                Print file paths instead of file contents
    binary              View binary files (coming soon)
    version             Print the current version number

EXAMPLES:
    pretty_files --help
    pretty_files bare --help
    pretty_files binary --help

NOTES:
    • for more information add `--help` after special commands
", VERSION);
                println!("{help}");
            }
            CommandHelp::Read => {
                let help: String = format!("pretty_files {} - Read Mode

USAGE:
    pretty_files [OPTIONS] <FILES...>

OPTIONS:
    -n, --numbers       Display line numbers
    -r, --recursive     Search directories recursively
    -d, --debug         Print file names before their contents
    -D                  Disable automatic debug mode
    -S                  Disable syntax highlighting
    -i  --ignore        Ignore the following file

EXAMPLES:
    pretty_files main.rs
    pretty_files src/lib.rs Cargo.toml
    pretty_files -n src/main.rs
    pretty_files -r src/
    pretty_files -r src/ docs/
    pretty_files -r src/ -i src/utils.rs

NOTES:
    • Automatic debug mode is enabled when reading multiple files.
    • The -r option expects directories as input.
    • Files discovered recursively are treated like normal input files.", VERSION);
                println!("{help}");
            }
        }
    }

pub fn version() {
        println!("pretty_files {VERSION}");
}
