use clap::{Arg, ArgAction, Command, Parser};

const ABOUT: &str = "
Spongecrab - define CLIs, parse arguments, and evaluate variables in bash.

Spongecrab can create a CLI and parse arguments. The resulting arguments are
printed along with their values to stdout, which can be evaluated in the shell.
";

const EXAMPLE: &str = "
\u{1b}[1;4mExample:\u{1b}[0m
``` \u{1b}[4mmy_script.sh\u{1b}[0m
\u{1b}[2m# Create cli and parse arguments\u{1b}[0m
args=$(spongecrab foo --flag bar -- $@) || {
    echo \"$args\" \u{1b}[2m# Print help and errors\u{1b}[0m
    exit 1
}
\u{1b}[2m# Evaluate results\u{1b}[0m
eval $args
\u{1b}[2m# Process arguments\u{1b}[0m
echo Argument: $foo
[[ -z $bar ]] && echo \"bar\" flag not set
```
";

#[derive(Debug, Parser)]
#[command(name = "spongecrab")]
#[command(about = "spongecrab argument parser")]
#[command(author = "https://ariel.ninja")]
#[command(long_about = ABOUT)]
#[command(after_help = EXAMPLE)]
struct Cli {
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
    /// Show debug info
    #[arg(long)]
    pub debug: bool,
    /// Raw text to parse
    #[arg(raw = true)]
    pub input: Vec<String>,
}

fn main() {
    // Outer parse
    let outer_args = Cli::parse();
    let debug = outer_args.debug;
    if debug { println!("spongecrab args: {outer_args:#?}"); }
    // Build inner CLI
    let name = outer_args.name.unwrap_or_else(|| String::from("spongecrab"));
    let mut cli = Command::new(name).no_binary_name(true);
    if let Some(about) = outer_args.about {
        cli = cli.about(about);
    };
    for positional in outer_args.positional.iter() {
        let arg = Arg::new(positional);
        cli = cli.arg(arg);
    }
    for option in outer_args.option.iter() {
        let arg = Arg::new(option).long(option).required(false);
        cli = cli.arg(arg);
    }
    for flag in outer_args.flag.iter() {
        let arg = Arg::new(flag).long(flag).action(ArgAction::SetTrue);
        cli = cli.arg(arg);
    }
    if debug { println!(">> spongecrab parsing inner cli..."); }

    // Inner parse
    let inner_args = cli.clone().try_get_matches_from(&outer_args.input).unwrap_or_else(|error| {
        if error.kind() == clap::error::ErrorKind::DisplayHelp {
            cli.print_long_help().expect("print help");
        } else {
            eprintln!("{error}");
        }
        std::process::exit(1);
    });

    // if debug { println!("inner args: {inner_args:#?}"); }
    // Output inner arguments for evaluation
    for positional_name in outer_args.positional.iter() {
        if let Some(positional) = inner_args.get_one::<String>(positional_name) {
            println!("{positional_name}={positional}");
        }
    }
    for option_name in outer_args.option.iter() {
        if let Some(option) = inner_args.get_one::<String>(option_name) {
            println!("{option_name}={option}");
        }
    }
    for flag_name in outer_args.flag.iter() {
        if inner_args.get_flag(flag_name) {
            println!("{flag_name}=1");
        }
    }
    if debug { println!(">> spongecrab parsing complete."); }
}

