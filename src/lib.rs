use syntect::{
    highlighting::ThemeSet,
    parsing::SyntaxSet,
};

pub mod utils;
pub mod help;
pub mod text;
pub mod bare;
pub mod binary;

pub struct Command {
    args: Vec<String>,
    pub(crate) syntax_set: SyntaxSet,
    pub(crate) theme_set: ThemeSet,
}

impl Command {
    pub fn new(args: Vec<String>) -> Self {
        Self {
            args,
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
        }
    }

    pub fn args(&self) -> &[String] {
        &self.args
    }

    pub fn first_arg(&self) -> Option<&str> {
        self.args.get(1).map(String::as_str)
    }
}
