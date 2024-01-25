#! /bin/bash

set -e
cd $(dirname $0)
cargo build --release

TEMPFILE="target/README.md"

exec &> $TEMPFILE
echo '```'
target/release/spongecrab -h
echo '```
Generated code (using `--generate`):
```bash'
target/release/spongecrab --generate
echo '```'
echo "See [the example](example.sh)."
exec &> /dev/tty

mv $TEMPFILE .
echo success

