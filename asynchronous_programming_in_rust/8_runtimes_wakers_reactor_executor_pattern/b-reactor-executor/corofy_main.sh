#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )


corofy_proj_dir="$SCRIPT_DIR/../../code_from_book/Asynchronous-Programming-in-Rust-main/ch07/corofy/"

pushd $corofy_proj_dir
cargo install --path $corofy_proj_dir

corofy_bin="$corofy_proj_dir/target/release/corofy"

pushd $SCRIPT_DIR
$corofy_bin src/main.rs.input src/main.rs

# Add note about generated code
printf '%s\n%s\n' "// GENERATED CODE! See corofy_main.sh" "$(cat src/main.rs)" > src/main.rs.new
mv -f src/main.rs.new src/main.rs

