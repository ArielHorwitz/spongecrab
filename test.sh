#! /bin/bash

clear
set -e

FOO_VALUE="eggs"
BAR_VALUE="spam"
BAZ_VALUE=""
BAZ_FLAG=$([[ -n $BAZ_VALUE ]] && echo '--baz' || echo '')

cd $(dirname $0)
cargo build --release
cd target/release

echo --- generate example ---
./spongecrab --generate | sed 's;spongecrab ;./spongecrab ;' > ./example.sh
echo "
echo foo: \$foo [expected: $FOO_VALUE] && [[ \$foo = \"$FOO_VALUE\" ]] || exit 1
echo bar: \$bar [expected: $BAR_VALUE] && [[ \$bar = \"$BAR_VALUE\" ]] || exit 1
echo baz: \$baz [expected: $BAZ_VALUE] && [[ \$baz = \"$BAZ_VALUE\" ]] || exit 1
echo all arguments verified.
" >> example.sh
chmod +x ./example.sh

bat --style full ./example.sh
./example.sh --help || true

echo --- run example ---
./example.sh $FOO_VALUE --bar $BAR_VALUE $BAZ_FLAG

# rm ./example.sh
echo --- completed succesfully ---

