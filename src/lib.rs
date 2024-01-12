mod builder;
mod result;
mod parser;

pub use builder::CliBuilder;
pub use result::{Error, Result};
pub use parser::{parse_args, output_values, output_flags};

const ABOUT: &str = "
Spongecrab - define CLIs, parse arguments, and evaluate variables in bash.

Use \u{1b}[1m--generate\u{1b}[0m to generate boilerplate code for a script.
";

const BOILERPLATE: &str = r#"
# Create cli and parse arguments
spongecrab_args=$(
    CLI="foo -o bar -f baz"
    NAME="my_script"
    ABOUT="$NAME reticulates splines."
    spongecrab $CLI --name $NAME --about "$ABOUT" -- $@
) || { echo $spongecrab_args; exit 1 # Print help or errors and quit
}; eval $spongecrab_args # Evaluate results
"#;

pub fn run() -> Result<()> {
    // Outer parse
    let raw_args = std::env::args().collect::<Vec<String>>();
    let builder = CliBuilder::new(&raw_args)?;
    if builder.generate {
        println!("{BOILERPLATE}");
        return Ok(())
    }
    // Build CLI
    let mut cli = builder.build()?;
    // Inner parse
    let args = parse_args(&mut cli, &builder.input)?;
    // Output values to stdout for shell evaluation
    output_values(&builder.positional, &args)?;
    output_values(&builder.option, &args)?;
    output_flags(&builder.flag, &args)?;
    Ok(())
}

