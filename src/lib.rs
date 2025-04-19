use clap::{Arg, ArgAction, ArgMatches, Command, Parser};

const ABOUT: &str = "
Spongecrab - bringing powerful argument parsing to bash scripts

See \u{1b}[1m'--example'\u{1b}[0m and \u{1b}[1m'--generate'\u{1b}[0m for reference.";
const GENERATED_BOILERPLATE: &str = r#"
# Command line interface (based on `spongecrab --generate`)
APP_NAME=$(basename "${0%.*}")
ABOUT="program description"
# Argument syntax: "<arg_name>;<help_text>;<default_value>;<short_name>"
# -o, -c, -C are mutually exclusive
CLI=(
    -p "arg1;Positional argument"
    -o "arg2;Optional positional argument;<default value>"
    -O "option;Optional argument;;o"
    -f "flag;Optional flag argument;;f"
    # -c "collect_any;Optional remaining positional arguments"
    # -C "collect_some;Required remaining positional arguments"
    -e "extra;Optional extra arguments after '--'"
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
    /// Application name
    #[arg(long, default_value_t = String::from("myscript"))]
    pub name: String,
    /// Application about text
    #[arg(long)]
    pub about: Option<String>,
    /// Add a (required) positional argument
    #[arg(short = 'p', long)]
    pub positional: Vec<String>,
    /// Add an optional positional argument
    #[arg(short = 'o', long, conflicts_with_all = ["collect", "collect_required"])]
    pub optional: Vec<String>,
    /// Add an optional argument
    #[arg(short = 'O', long)]
    pub option: Vec<String>,
    /// Add a flag argument
    #[arg(short = 'f', long)]
    pub flag: Vec<String>,
    /// Collect remaining positional arguments
    #[arg(short = 'c', long, conflicts_with_all = ["optional", "collect_required"])]
    pub collect: Option<String>,
    /// Collect (required) remaining positional arguments
    #[arg(short = 'C', long = "collect+")]
    #[arg(value_name = "COLLECT", conflicts_with_all = ["optional", "collect"])]
    pub collect_required: Option<String>,
    /// Collect extra optional arguments after '--'
    #[arg(short = 'e', long)]
    pub extra: Option<String>,
    /// Prefix for parsed variable names
    #[arg(short = 'P', long)]
    pub prefix: Option<String>,
    /// Generate script boilerplate (see also '--example')
    #[arg(short = 'G', long)]
    pub generate: bool,
    /// Generate example script
    #[arg(long)]
    pub example: bool,
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
        if let Some(collect_var) = &self.collect {
            cli = cli.arg(get_arg(collect_var, ArgumentType::Collect));
        } else if let Some(collect_var) = &self.collect_required {
            cli = cli.arg(get_arg(collect_var, ArgumentType::CollectRequired));
        };
        if let Some(extra_args) = &self.extra {
            cli = cli.arg(get_arg(extra_args, ArgumentType::Last));
        };
        cli
    }

    fn output_values(&self, matches: &ArgMatches) -> String {
        let prefix = self.prefix.clone().unwrap_or_default();
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
        if let Some(collect) = self.collect.as_ref().or(self.collect_required.as_ref()) {
            let (name, _, _, _) = get_arg_data(collect);
            let collected = matches.get_many(&name).map_or_else(String::new, |values| {
                values
                    .map(|v: &String| format!("'{v}'"))
                    .collect::<Vec<String>>()
                    .join(" ")
            });
            let name = format!("{prefix}{name}").replace('-', "_");
            output.push(format!("{name}=({collected})"));
        }
        if let Some(extra) = self.extra.as_ref() {
            let (name, _, _, _) = get_arg_data(extra);
            let extras = matches.get_many(&name).map_or_else(String::new, |values| {
                values
                    .map(|v: &String| format!("'{v}'"))
                    .collect::<Vec<String>>()
                    .join(" ")
            });
            let name = format!("{prefix}{name}").replace('-', "_");
            output.push(format!("{name}=({extras})"));
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
    Collect,
    CollectRequired,
    Last,
}

fn with_arguments(cli: Command, args: &[String], arg_type: ArgumentType) -> Command {
    cli.args(args.iter().map(|data| get_arg(data, arg_type)))
}

fn get_arg(data: &str, arg_type: ArgumentType) -> Arg {
    let (name, help, default, short) = get_arg_data(data);
    let mut arg = Arg::new(name.clone()).help(help);
    if [ArgumentType::Optional, ArgumentType::Option].contains(&arg_type) {
        if let Some(default) = default {
            arg = arg.default_value(default);
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
        ArgumentType::Option => arg.long(name),
        ArgumentType::Flag => arg.long(name).action(ArgAction::SetTrue),
        ArgumentType::Collect => arg.num_args(0..),
        ArgumentType::CollectRequired => arg.num_args(1..).required(true),
        ArgumentType::Last => arg.num_args(0..).last(true),
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
