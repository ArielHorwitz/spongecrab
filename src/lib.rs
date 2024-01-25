use clap::{Arg, ArgAction, ArgMatches, Command, Parser};

const ABOUT: &str = "
Spongecrab - bringing powerful argument parsing to bash scripts

See \u{1b}[1m'--example'\u{1b}[0m and \u{1b}[1m'--generate'\u{1b}[0m for reference.";
const GENERATED_BOILERPLATE: &str = r#"
# Command line interface (based on `spongecrab --generate`)
APP_NAME=$(basename "$0")
ABOUT="program description"
# Argument syntax: "<arg_name>;<help_text>;<default_value>;<short_name>"
CLI=(
    -p "arg1;positional argument"
    -o "arg2;optional positional argument;default"
    -O "option;optional argument;;o"
    -f "flag;optional flag argument;;f"
)
CLI=$(spongecrab --name "$APP_NAME" --about "$ABOUT" "${CLI[@]}" -- "$@") || exit 1
eval "$CLI" || exit 1
"#;
const EXAMPLE_SCRIPT: &str = include_str!("example.sh");
const ARG_DELIMITER: char = ';';

pub fn run() -> anyhow::Result<()> {
    let raw_args = std::env::args().collect::<Vec<String>>();
    let output = CliBuilder::new(&raw_args).parse()?;
    println!("{output}");
    Ok(())
}

/// Arguments for building the CLI.
#[derive(Debug, Parser)]
#[command(name = "spongecrab")]
#[command(version)]
#[command(about = ABOUT)]
#[command(author = "https://ariel.ninja")]
pub struct CliBuilder {
    /// Add a (required) positional argument
    #[arg(short = 'p', long)]
    pub positional: Vec<String>,
    /// Add an optional positional argument
    #[arg(short = 'o', long)]
    pub optional: Vec<String>,
    /// Add an optional argument
    #[arg(short = 'O', long)]
    pub option: Vec<String>,
    /// Add a flag argument
    #[arg(short = 'f', long)]
    pub flag: Vec<String>,
    /// Application name
    #[arg(short = 'N', long, default_value_t = String::from("myscript"))]
    pub name: String,
    /// Application about text
    #[arg(short = 'A', long)]
    pub about: Option<String>,
    /// Prefix for parsed variable names
    #[arg(short = 'P', long)]
    pub prefix: Option<String>,
    /// Generate example script
    #[arg(short = 'E', long)]
    pub example: bool,
    /// Generate script boilerplate (see also '--example')
    #[arg(short = 'G', long)]
    pub generate: bool,
    /// Raw text to parse
    #[arg(raw = true)]
    pub input: Vec<String>,
}

impl CliBuilder {
    /// Create a new CLI builder from command line arguments.
    ///
    /// # Errors
    /// Will error if fails to parse input strings.
    pub fn new<T>(input: &[T]) -> Self
    where
        T: std::convert::AsRef<std::ffi::OsStr>,
    {
        Self::parse_from(input)
    }

    /// Produce a string for shell evaluation containing parsed arguments and values.
    ///
    /// # Errors
    /// Will error if argument parsing fails.
    pub fn parse(&self) -> anyhow::Result<String> {
        if self.example {
            return Ok(EXAMPLE_SCRIPT.to_owned());
        }
        if self.generate {
            return Ok(GENERATED_BOILERPLATE.to_owned());
        }
        let args = self.build().try_get_matches_from(&self.input)?;
        let output = self.output_values(&args);
        Ok(output)
    }

    #[must_use]
    fn build(&self) -> Command {
        let mut cli = Command::new(self.name.clone()).no_binary_name(true);
        if let Some(about) = self.about.clone() {
            cli = cli.about(about);
        };
        cli = with_arguments(cli, &self.positional, ArgumentType::Positional);
        cli = with_arguments(cli, &self.optional, ArgumentType::Optional);
        cli = with_arguments(cli, &self.option, ArgumentType::Option);
        cli = with_arguments(cli, &self.flag, ArgumentType::Flag);
        cli
    }

    fn output_values(&self, matches: &ArgMatches) -> String {
        let prefix = self.prefix.to_owned().unwrap_or_else(|| "".to_owned());
        let mut output = Vec::new();
        let arguments = vec![&self.positional, &self.optional, &self.option]
            .into_iter()
            .flatten()
            .map(|data| get_arg_data(data).0);
        for name in arguments {
            let value_match = matches.get_one::<String>(&name);
            let value = if let Some(v) = value_match { v } else { "" };
            let name = format!("{prefix}{name}").replace('-', "_");
            output.push(format!("{name}='{value}'"));
        }
        let flags = self.flag.iter().map(|data| get_arg_data(data).0);
        for name in flags {
            let value = if matches.get_flag(&name) { "1" } else { "" };
            let name = format!("{prefix}{name}").replace('-', "_");
            output.push(format!("{name}='{value}'"));
        }
        output.join("\n")
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum ArgumentType {
    Positional,
    Optional,
    Option,
    Flag,
}

fn with_arguments(cli: Command, args: &[String], arg_type: ArgumentType) -> Command {
    cli.args(args.iter().map(|data| get_arg(data, arg_type)))
}

fn get_arg(data: &str, arg_type: ArgumentType) -> Arg {
    let (name, help, default, short) = get_arg_data(data);
    let mut arg = Arg::new(name.to_owned()).help(help.to_owned());
    if [ArgumentType::Optional, ArgumentType::Option].contains(&arg_type) {
        if let Some(default) = default {
            arg = arg.default_value(default.to_owned());
        };
    };
    if [ArgumentType::Option, ArgumentType::Flag].contains(&arg_type) {
        if let Some(short) = short {
            arg = arg.short(short);
        };
    };
    arg = match arg_type {
        ArgumentType::Positional => arg.required(true),
        ArgumentType::Optional => arg,
        ArgumentType::Option => arg.long(name.to_owned()),
        ArgumentType::Flag => arg.long(name.to_owned()).action(ArgAction::SetTrue),
    };
    arg
}

fn get_arg_data(data: &str) -> (String, String, Option<String>, Option<char>) {
    let (name, data) = data.split_once(crate::ARG_DELIMITER).unwrap_or((data, ""));
    let (help, data) = data.split_once(crate::ARG_DELIMITER).unwrap_or((data, ""));
    let (default, short) = data.split_once(crate::ARG_DELIMITER).unwrap_or((data, ""));
    let default = if default.is_empty() {
        None
    } else {
        Some(default.to_owned())
    };
    let short = short.chars().next();
    (name.to_owned(), help.to_owned(), default, short)
}
