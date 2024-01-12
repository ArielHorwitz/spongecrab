mod builder;
mod result;
mod parser;

pub use builder::CliBuilder;
pub use result::{Error, Result};
pub use parser::{parse_args, output_values, output_flags};

const ABOUT: &str = "
Spongecrab - define CLIs, parse arguments, and evaluate variables in bash.

Spongecrab can create a CLI and parse arguments. The resulting arguments are
printed along with their values to stdout, which can be evaluated in the shell.
";

const EXAMPLE: &str = "
\u{1b}[1;4mExample:\u{1b}[0m
``` \u{1b}[4mmy_script.sh\u{1b}[0m
\u{1b}[2m# Create cli and parse arguments\u{1b}[0m
args=$(spongecrab foo --flag bar --name my_script -- $@) || {
    echo \"$args\" \u{1b}[2m# Print help and errors\u{1b}[0m
    exit 1
}
\u{1b}[2m# Evaluate results\u{1b}[0m
eval $args
\u{1b}[2m# Process arguments\u{1b}[0m
echo Argument: $foo
[[ -n $bar ]] && echo \"bar\" flag set || echo \"bar\" flag not set
```
";

pub fn run() -> Result<()> {
    // Outer parse
    let raw_args = std::env::args().collect::<Vec<String>>();
    let builder = CliBuilder::new(&raw_args)?;
    let debug = builder.debug;
    // Build CLI
    if debug { println!(">> spongecrab building cli: {builder:#?}"); }
    let mut cli = builder.build()?;
    // Inner parse
    if debug { println!(">> spongecrab parsing inner cli..."); }
    let args = parse_args(&mut cli, &builder.input)?;
    // Output values to stdout for shell evaluation
    if debug { println!(">> spongecrab printing argument values..."); }
    output_values(&builder.positional, &args)?;
    output_values(&builder.option, &args)?;
    output_flags(&builder.flag, &args)?;
    if debug { println!(">> spongecrab parsing complete."); }
    Ok(())
}

