  #!/bin/bash
set -e
cd "`dirname $0`"
source flags.sh
cargo build --target wasm32-unknown-unknown --release
cp ./../../target/wasm32-unknown-unknown/release/request-interface.wasm  ../../res
cp ../../res/request-interface.wasm ../../oracle/tests/it/wasm
