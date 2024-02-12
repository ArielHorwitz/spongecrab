```
Spongecrab - bringing powerful argument parsing to bash scripts

See '--example' and '--generate' for reference.

Usage: spongecrab [OPTIONS] [-- <INPUT>...]

Arguments:
  [INPUT]...  Raw text to parse

Options:
      --name <NAME>              Application name [default: myscript]
      --about <ABOUT>            Application about text
  -p, --positional <POSITIONAL>  Add a (required) positional argument
  -o, --optional <OPTIONAL>      Add an optional positional argument
  -O, --option <OPTION>          Add an optional argument
  -f, --flag <FLAG>              Add a flag argument
  -c, --collect <COLLECT>        Collect remaining positional arguments
  -C, --collect+ <COLLECT>       Collect (required) remaining positional arguments
  -e, --extra <EXTRA>            Collect extra optional arguments after '--'
  -P, --prefix <PREFIX>          Prefix for parsed variable names
  -G, --generate                 Generate script boilerplate (see also '--example')
      --example                  Generate example script
  -h, --help                     Print help
  -V, --version                  Print version
```
Generated code (using `--generate`):
```bash

# Command line interface (based on `spongecrab --generate`)
APP_NAME=$(basename "$0")
ABOUT="program description"
# Argument syntax: "<arg_name>;<help_text>;<default_value>;<short_name>"
# -o, -c, -C are mutually exclusive
CLI=(
    -p "arg1;Positional argument"
    -o "arg2;Optional positional argument;<default value>"
    -O "option;Optional argument;;o"
    -f "flag;Optional flag argument;;f"
    -c "collect_any;Optional remaining positional arguments"
    -C "collect_some;Required remaining positional arguments"
    -e "extra;Optional extra arguments after '--'"
)
CLI=$(spongecrab --name "$APP_NAME" --about "$ABOUT" "${CLI[@]}" -- "$@") || exit 1
eval "$CLI" || exit 1

```
See the [example script](src/example.sh).
