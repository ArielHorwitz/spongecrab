use crate::result::Result;
use clap::{Arg, ArgAction, Command, Parser};

#[derive(Debug, Parser)]
#[command(name = "spongecrab")]
#[command(about = "spongecrab argument parser")]
#[command(author = "https://ariel.ninja")]
#[command(long_about = crate::ABOUT)]
pub struct CliBuilder {
    /// Positional argument
    #[arg()]
    pub positional: Vec<String>,
    /// Optional argument
    #[arg(short, long)]
    pub option: Vec<String>,
    /// Flag argument
    #[arg(short, long)]
    pub flag: Vec<String>,
    /// Name
    #[arg(long)]
    pub name: Option<String>,
    /// About text
    #[arg(long)]
    pub about: Option<String>,
    /// Generate script boilerplate
    #[arg(long)]
    pub generate: bool,
    /// Raw text to parse
    #[arg(raw = true)]
    pub input: Vec<String>,
}

impl CliBuilder {
    pub fn new(input: &[String]) -> Result<Self> {
        Ok(Self::parse_from(input))
    }

    pub fn build(&self) -> Result<Command> {
        let name = match &self.name {
            Some(name) => name.to_owned(),
            None => String::from("spongecrab"),
        };
        let mut cli = Command::new(name).no_binary_name(true);
        if let Some(about) = self.about.clone() {
            cli = cli.about(about);
        };
        for positional in self.positional.iter() {
            let arg = Arg::new(positional).required(true);
            cli = cli.arg(arg);
        }
        for option in self.option.iter() {
            let arg = Arg::new(option).long(option).required(false);
            cli = cli.arg(arg);
        }
        for flag in self.flag.iter() {
            let arg = Arg::new(flag).long(flag).action(ArgAction::SetTrue);
            cli = cli.arg(arg);
        }
        Ok(cli)
    }
}
