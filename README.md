```
Spongecrab - bringing powerful argument parsing to bash scripts

See '--example' and '--generate' for reference.

Usage: spongecrab [OPTIONS] [-- <INPUT>...]

Arguments:
  [INPUT]...  Raw text to parse

Options:
  -p, --positional <POSITIONAL>  Add a (required) positional argument
  -o, --optional <OPTIONAL>      Add an optional positional argument
  -O, --option <OPTION>          Add an optional argument
  -f, --flag <FLAG>              Add a flag argument
  -N, --name <NAME>              Application name [default: myscript]
  -A, --about <ABOUT>            Application about text
  -P, --prefix <PREFIX>          Prefix for parsed variable names
  -E, --example                  Generate example script
  -G, --generate                 Generate script boilerplate (see also '--example')
  -h, --help                     Print help
  -V, --version                  Print version
```
Generated code (using `--generate`):
```bash

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

```
See the [example script](src/example.sh).
