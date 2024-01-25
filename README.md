```
Spongecrab - A powerful argument parser for bash.

Use --generate to generate boilerplate code for a script.


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
  -X, --prefix <PREFIX>          Prefix final variable names [default: ]
  -G, --generate                 Generate script boilerplate
  -h, --help                     Print help (see more with '--help')
  -V, --version                  Print version
```
Generated code (using `--generate`):
```bash

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

```
See [the example](example.sh).
