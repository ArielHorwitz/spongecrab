#! /bin/bash

set -e
cd $(dirname $0)
cargo build --release

TEMPFILE="target/README.md"

exec &> $TEMPFILE
echo '```'
target/release/spongecrab -h
echo '```'
echo 'Generated code (using `--generate`):'
echo '```bash # my_script.sh'
target/release/spongecrab --generate
echo '```'
exec &> /dev/tty

mv $TEMPFILE .

