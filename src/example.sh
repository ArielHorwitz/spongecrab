#! /bin/bash

set -e

# Command line interface (based on `spongecrab --generate`)
APP_NAME=$(basename "$0")
ABOUT="$APP_NAME is a hello world program.

Examples:
$APP_NAME neo
$APP_NAME neo -n \"follow the white rabbit.\"
$APP_NAME -pn \"follow the white rabbit.\" neo"
# Argument syntax: "<arg_name>;<help_text>;<default_value>;<short_name>"
CLI=(
    -p "name;Name to greet;world"
    -o "greetings;Greeting to use;Hello"
    -O "notice;Add a notice;;n"
    -O "postscript;Add extra text;;s"
    -f "polite;Be extra polite;;p"
)
CLI=$(spongecrab --name "$APP_NAME" --about "$ABOUT" "${CLI[@]}" -- "$@") || exit 1
eval "$CLI" || exit 1

# Configure behavior from parsed arguments
if [[ -z $polite ]]; then
    end_stop="!"
    notice_start="By the way, "
else
    end_stop="."
    [[ $greetings != Hello ]] || greetings="Greetings,"
    notice_start="I've been told to let you know, "
fi

# Print greetings
echo $greetings $name$end_stop
[[ -z $notice ]] || echo $notice_start$notice
[[ -z $postscript ]] || echo $postscript
[[ -z $polite ]] || echo 'Thank you for your time.'
