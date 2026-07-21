use std::{
    fs,
    path::{Path, PathBuf},
};

use syntect::{
    easy::HighlightLines,
    util::{LinesWithEndings, as_24_bit_terminal_escaped},
};

use crate::Command;


impl Command {
    fn highlight_path(&self, path: &Path, content: &str) -> Vec<String> {
        let syntax = self
            .syntax_set
            .find_syntax_for_file(path)
            .unwrap()
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        let theme = self
            .theme_set
            .themes
            .get("base16-ocean.dark")
            .unwrap();

        let mut highlighter = HighlightLines::new(syntax, theme);

        let mut highlighted = Vec::new();

        for line in LinesWithEndings::from(content) {
            let ranges = highlighter
                .highlight_line(line, &self.syntax_set)
                .unwrap();
            highlighted.push(
                as_24_bit_terminal_escaped(&ranges, false)
            );
        }
        highlighted
    }


    pub(crate) fn read_file(&self, file: &PathBuf) -> std::io::Result<()> {
        let content = fs::read_to_string(&file)?;

        if self.debug {
            println!("\n{}:\n", file.display());
        }

        if self.syntax_highlight {
            let highlighted = self.highlight_path(file.as_path(), &content);

            for (i, line) in highlighted.iter().enumerate() {
                if self.count_lines {
                    print!("{:>4}. {}", i + 1, line);
                } else {
                    print!("{line}");
                }
            }
        } else {
            for (i, line) in content.lines().enumerate() {
                if self.count_lines {
                    println!("{:>4}. {}", i + 1, line);
                } else {
                    println!("{line}");
                }
            }
        }

        Ok(())
    }
}
