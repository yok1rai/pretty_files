use std::{
    fs,
    path::PathBuf,
};


use crate::{Command, help, utils};


impl Command {
    pub fn read_binary(&self) -> std::io::Result<()> {
        let mut files = Vec::new();
        let mut ignored: Vec<String> = Vec::new();

        let mut recursive = false;
        let mut debug = false;
        let mut debug_allowed = true;

        if self.args[2].as_str() == "--help" {
            help::help(&self.args);
            return Ok(());
        }

        let mut args = self.args[2..].iter();

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-i" | "--ignore" => {
                    if let Some(name) = args.next() {
                        ignored.push(name.clone());
                    } else {
                        eprintln!("missing argument for --ignore");
                        return Ok(());
                    }
                },
                "-r" | "--recursive" => recursive = true,
                "-d" | "--debug" => debug = true,
                "-D" => debug_allowed = false,
                _ => files.push(PathBuf::from(arg)),
            }
        }

        if recursive {
            let directories = files.clone();
            files.clear();

            for directory in directories {
                for file in utils::recursive_search(directory, &ignored) {
                    files.push(file);
                }
            }
        }

        if files.len() > 1 && debug_allowed {
            debug = true;
        }

        for file in &files {
            let path = file.to_string_lossy();

            if ignored.iter().any(|i| path.ends_with(i)) {
                continue;
            }

            let content = match fs::read(&file) {
                Ok(content) => content,
                Err(e) => {
                    match e.kind() {
                        std::io::ErrorKind::IsADirectory => {
                            eprintln!(
                                "`{}` is a directory",
                                file.display()
                            );
                        }
                        std::io::ErrorKind::NotFound => {
                            eprintln!(
                                "`{}` does not exist",
                                file.display()
                            );
                        }
                        std::io::ErrorKind::PermissionDenied => {
                            eprintln!(
                                "You don't have permission to access `{}`",
                                file.display()
                            );
                        }
                        std::io::ErrorKind::InvalidData => {
                            eprintln!(
                                "`{}` is a binary file (try running with `binary` special command)",
                                file.display()
                            );
                        }
                        _ => {
                            eprintln!(
                                "`{}`: {}",
                                file.display(),
                                e
                            );
                        }
                    }
                    continue;
                }
            };
            if debug {
                println!("\n{}:\n", path);
            }

            if content.is_empty() {
                println!("{:08X}", 0);
                continue;
            }
            for (offset, chunk) in content.chunks(16).enumerate() {
            let offset = offset * 16;

            print!("{:08X}  ", offset);

            for i in 0..16 {
                if let Some(byte) = chunk.get(i) {
                    print!("{:02X} ", byte);
                } else {
                    print!("   ");
                }

                if i == 7 {
                    print!(" ");
                }
            }
            print!(" ");

            for &byte in chunk {
                let c = if byte.is_ascii_graphic() || byte == b' ' {
                    byte as char
                } else {
                    '.'
                };

                print!("{c}");
            }

            println!();
            }
        }
        Ok(())
    }
}
