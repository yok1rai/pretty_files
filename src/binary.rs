use std::{fs, path::PathBuf};

use crate::Command;

impl Command {
    pub(crate) fn read_binary(&self, file: &PathBuf) -> std::io::Result<()> {
        let content = fs::read(&file)?;

        if self.debug {
            println!("\n{}:\n", file.display());
        }

        if content.is_empty() {
            println!("{:08X}", 0);
            return Ok(());
        }

        for (offset, chunk) in content.chunks(16).enumerate() {
            let offset = offset * 16;

            // Offset
            print!("{:08X}  ", offset);

            // Hex bytes
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

            // ASCII
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

        Ok(())
    }
}
