```
spongecrab argument parser

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
```bash # my_script.sh

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

```
