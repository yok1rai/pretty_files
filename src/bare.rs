use std::{
    fs,
    path::PathBuf,
};

use crate::{help, utils, Command};

impl Command {
    pub fn bare_read(&self) -> std::io::Result<()> {
        let mut recursive = false;
        let mut directories = Vec::new();
        let mut ignored = Vec::new();

        match self.args.get(2) {
            Some(help_arg) => {
                if help_arg.as_str() == "--help" {
                    help::help(&self.args);
                    return Ok(());
                }

            }
            None => {}
        }
        let mut args = self.args[2..].iter();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-r" | "--recursive" => {
                    recursive = true;
                }
                "-i" | "--ignore" => {
                    if let Some(name) = args.next() {
                        ignored.push(name.clone());
                    } else {
                        eprintln!("missing argument for --ignore");
                        return Ok(());
                    }

                }
                _ => {
                    directories.push(PathBuf::from(arg));
                }
            }
        }
        for path in &directories {
            if !path.is_dir() {
                eprintln!(
                    "{} is not a directory",
                    path.display()
                );
                return Ok(());
            }
        }
        if recursive {
            let tmp_directories = std::mem::take(&mut directories);

            for directory in tmp_directories {
                directories.extend(
                    utils::recursive_search(directory, &ignored)
                );
            }

            for file in &directories {
                println!("{}", file.display());
            }
            return Ok(());
        }

        for directory in &directories {
            for entry in fs::read_dir(directory)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_file() {
                    let name = path.to_string_lossy();

                    if ignored.iter().any(|i| i == &name) {
                        continue;
                    }

                    println!("{}", path.display());
                }
            }
        }
        Ok(())
    }
}
