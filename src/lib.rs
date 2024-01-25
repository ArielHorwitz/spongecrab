use clap::{Arg, ArgAction, ArgMatches, Command, Parser};

const ABOUT: &str = "
Spongecrab - A powerful argument parser for bash.

Use \u{1b}[1m--generate\u{1b}[0m to generate boilerplate code for a script.
";

const GENERATED_BOILERPLATE: &str = r#"
APP_NAME=$(basename "$0")
ABOUT_APP="$APP_NAME is a hello world program."
CLI=(
    -p "name;Name to say hello to"
    -o "greetings;Greetings to use;hello"
    -O "notice;Add a notice after greeting;;n"
    -f "polite;Be polite;;p"
)
CLI=$(spongecrab --name "$APP_NAME" --about "$ABOUT_APP" "${CLI[@]}" -- "$@") || exit 1
eval "$CLI" || exit 1
"#;

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
#[command(about = ABOUT)]
#[command(author = "https://ariel.ninja")]
#[command(version)]
#[command(long_about = crate::ABOUT)]
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
    /// Prefix final variable names
    #[arg(short = 'X', long, default_value_t = String::from(""))]
    pub prefix: String,
    /// Generate script boilerplate
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

    /// Get a string for shell evaluation with all arguments parsed with their values.
    ///
    /// # Errors
    /// Will error if argument parsing fails.
    pub fn parse(&self) -> anyhow::Result<String> {
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
        let mut output = Vec::new();
        let arguments = vec![&self.positional, &self.optional, &self.option]
            .into_iter()
            .flatten()
            .map(|data| get_arg_data(data).0);
        for name in arguments {
            let value_match = matches.get_one::<String>(&name);
            let value = if let Some(v) = value_match { v } else { "" };
            let name = &name.replace('-', "_");
            output.push(format!("{}{name}='{value}'", self.prefix));
        }
        let flags = self.flag.iter().map(|data| get_arg_data(data).0);
        for name in flags {
            let value = if matches.get_flag(&name) { "1" } else { "" };
            let name = &name.replace('-', "_");
            output.push(format!("{}{name}='{value}'", self.prefix));
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
