Spongecrab - define CLIs, parse arguments, and evaluate variables in bash.

Spongecrab can create a CLI and parse arguments. The resulting arguments are
printed along with their values to stdout, which can be evaluated in the shell.


```
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

      --debug
          Show debug info

  -h, --help
          Print help (see a summary with '-h')
```

Example:
```bash
# my_script.sh
# Create cli and parse arguments
args=$(spongecrab foo --flag bar -- $@) || {
    echo "$args" # Print help and errors
    exit 1
}
# Evaluate results
eval $args
# Process arguments
echo Argument: $foo
[[ -n $bar ]] && echo \"bar\" flag set
```
