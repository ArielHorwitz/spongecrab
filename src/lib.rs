use clap::{Arg, ArgAction, ArgMatches, Command, Parser};

const ABOUT: &str = "
Spongecrab - A powerful argument parser for bash.

Use \u{1b}[1m--generate\u{1b}[0m to generate boilerplate code for a script.

Argument can be encoded using this format (where applicable):
NAME;HELP_TEXT;DEFAULT_VALUE;SHORT_NAME
";

const GENERATED_BOILERPLATE: &str = r#"
APP_NAME=$(basename "$0")
ABOUT_APP="$APP_NAME reticulates splines."
CLI=(
    -p "source;File path of existing file"
    -o "destination;Target path;."
    -O "backup-path;Backup path for existing file;;b"
    -f "verbose;Print more logs;;v"
)
CLI=$(spongecrab --name "$APP_NAME" --about "$ABOUT_APP" "${CLI[@]}" -- "$@") || exit 1
eval "$CLI" || exit 1
"#;

const ARG_DELIMITER: char = ';';

pub fn run() -> anyhow::Result<()> {
    let raw_args = std::env::args().collect::<Vec<String>>();
    CliBuilder::new(&raw_args).parse()
}

/// Arguments for building the CLI.
#[derive(Debug, Parser)]
#[command(name = "spongecrab")]
#[command(about = "spongecrab argument parser")]
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
    #[arg(long, default_value_t = String::from("myscript"))]
    pub name: String,
    /// Application about text
    #[arg(long)]
    pub about: Option<String>,
    /// Prefix final variable names
    #[arg(long, default_value_t = String::from(""))]
    pub prefix: String,
    /// Generate script boilerplate
    #[arg(long)]
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
    pub fn new(input: &[String]) -> Self {
        Self::parse_from(input)
    }

    pub fn parse(&self) -> anyhow::Result<()> {
        if self.generate {
            println!("{GENERATED_BOILERPLATE}");
            return Ok(());
        }
        let args = self.build().try_get_matches_from(&self.input)?;
        self.output_values(&args);
        self.output_flags(&args);
        Ok(())
    }

    #[must_use]
    pub fn build(&self) -> Command {
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

    fn output_values(&self, matches: &ArgMatches) {
        let arguments = vec![&self.positional, &self.optional, &self.option]
            .into_iter()
            .flatten()
            .map(|data| get_arg_data(data).0);
        for name in arguments {
            let value_match = matches.get_one::<String>(&name);
            let value = if let Some(v) = value_match { v } else { "" };
            let name = &name.replace('-', "_");
            println!("{name}={value}");
        }
    }

    fn output_flags(&self, matches: &ArgMatches) {
        let flags = self.flag.iter().map(|data| get_arg_data(data).0);
        for name in flags {
            let value = if matches.get_flag(&name) { "1" } else { "" };
            let name = &name.replace('-', "_");
            println!("{name}={value}");
        }
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
    let default = if default.is_empty() { None } else { Some(default.to_owned()) };
    let short = short.chars().next();
    (name.to_owned(), help.to_owned(), default, short)
}

