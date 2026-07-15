use std::fs;



pub struct Commands {

    pub args: Vec<String>

}





impl Commands {

    pub fn new(args: Vec<String>) -> Self {

        Self { args }

    }

    pub fn read_file(&self) -> std::io::Result<()> {

        if self.args.len() < 3 {

            eprintln!("you must specify the path");

            Ok(())

        } else {

            let dest = &self.args[2];

            let content = fs::read_to_string(dest)?;

            println!("==={dest}===\n");

            println!("{content}");

            Ok(())

        }

    }

}
