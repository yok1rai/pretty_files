use std::{
    fs,
    path::PathBuf,
};

use crate::Command;

impl Command {
    pub(crate) fn bare_read(&self, directory: &PathBuf) -> std::io::Result<()> {
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                println!("{}", path.display());
            }
        }
        Ok(())
    }
}
