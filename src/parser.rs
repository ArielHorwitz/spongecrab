use crate::result::{Error, Result};
use clap::{ArgMatches, Command};

pub fn parse_args(cli: &mut Command, input: &[String]) -> Result<ArgMatches> {
    cli.clone()
        .try_get_matches_from(input)
        .map_err(|error| match error.kind() {
            clap::error::ErrorKind::DisplayHelp => Error::from(cli.render_help().to_string()),
            _other_kind => Error::from(error.to_string()),
        })
}

pub fn output_values(arguments: &[String], matches: &ArgMatches) -> Result<()> {
    for name in arguments.iter() {
        if let Some(value) = matches.get_one::<String>(name) {
            println!("{name}={value}");
        }
    }
    Ok(())
}

pub fn output_flags(flags: &[String], matches: &ArgMatches) -> Result<()> {
    for name in flags.iter() {
        if matches.get_flag(name) {
            println!("{name}=1");
        }
    }
    Ok(())
}
