#! /bin/bash

set -e

APP_NAME=$(basename "$0")
ABOUT_APP="$APP_NAME is a hello world program.

Examples:
$0 Newman
$0 neo -n \"follow the white rabbit.\"
$0 -pn \"follow the white rabbit.\" neo
"
CLI=(
    -p "name;Name to say hello to;world"
    -o "greeting;Greeting to use;Hello"
    -O "notice;Add a notice;;n"
    -O "postscript;Add extra text;;s"
    -f "polite;Be extra polite;;p"
)
CLI=$(spongecrab --name "$APP_NAME" --about "$ABOUT_APP" "${CLI[@]}" -- "$@") || exit 1
eval "$CLI" || exit 1

if [[ -n $polite ]]; then
    notice_start="I've been told to let you know, "
    end_stop="."
    [[ $greeting != Hello ]] || greeting="Greetings"
else
    notice_start="By the way, "
    end_stop="!"
fi

echo $greeting $name$end_stop
[[ -z $notice ]] || echo $notice_start$notice
[[ -z $postscript ]] || echo $postscript
[[ -z $polite ]] || echo 'Thank you for your time.'

