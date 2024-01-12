```
Spongecrab - define CLIs, parse arguments, and evaluate variables in bash.

Use --generate to generate boilerplate code for a script.


Usage: spongecrab [OPTIONS] [POSITIONAL]... [-- <INPUT>...]

Arguments:
  [POSITIONAL]...
          Positional argument

  [INPUT]...
          Raw text to parse

Options:
  -o, --option <OPTION>
          Optional argument

  -f, --flag <FLAG>
          Flag argument

      --name <NAME>
          Name

      --about <ABOUT>
          About text

      --generate
          Generate script boilerplate

  -h, --help
          Print help (see a summary with '-h')
```

Generated code:
```bash
# my_script.sh
# Create cli and parse arguments
spongecrab_args=$(
    CLI="foo -o bar -f baz"
    NAME="my_script"
    ABOUT="$NAME reticulates splines."
    spongecrab $CLI --name $NAME --about "$ABOUT" -- $@
) || { echo $spongecrab_args; exit 1 # Print help or errors and quit
}; eval $spongecrab_args # Evaluate results
```
