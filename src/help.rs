const VERSION: &str = "v2.5.0";

use crate::Command;

impl Command {
    pub fn help(&self) {
        match self.args.get(2).map(String::as_str) {
            Some("bare") => self.bare_help(),
            Some("binary") => self.binary_help(),
            Some("text") => self.text_help(),
            Some(topic) => {
                eprintln!("unknown help topic: {topic}");
                self.common_help();
            }
            None => self.common_help(),
        }
    }

    fn common_help(&self) {
        println!(r#"pretty_files {VERSION} - Simple File Viewer

USAGE:
    pretty_files <COMMAND> [OPTIONS] <PATHS...>

COMMANDS:
    help [TOPIC]       Show help
    bare               Print file paths
    binary             View binary files
    version            Show version

HELP TOPICS:
    text
    bare
    binary

EXAMPLES:
    pretty_files help
    pretty_files help text
    pretty_files help binary
"#);
    }

    fn text_help(&self) {
        println!(r#"pretty_files {VERSION} - Text Mode

USAGE:
    pretty_files [OPTIONS] <FILES...>

OPTIONS:
    -n, --numbers       Display line numbers
    -r, --recursive     Search recursively
    -d, --debug         Display filenames
    -D                  Disable automatic debug mode
    -S                  Disable syntax highlighting
    -i, --ignore        Ignore files
"#);
    }

    fn bare_help(&self) {
        println!(r#"pretty_files {VERSION} - Bare Mode

USAGE:
    pretty_files bare [OPTIONS] <DIRECTORIES...>

OPTIONS:
    -r, --recursive     Search recursively
    -i, --ignore        Ignore files
"#);
    }

    fn binary_help(&self) {
        println!(r#"pretty_files {VERSION} - Binary Mode

USAGE:
    pretty_files binary [OPTIONS] <FILES...>

OPTIONS:
    -r, --recursive     Search recursively
    -d, --debug         Display filenames
    -D                  Disable automatic debug mode
    -i, --ignore        Ignore files
"#);
    }

    pub fn version(&self) {
        println!("pretty_files {VERSION}");
    }
}
