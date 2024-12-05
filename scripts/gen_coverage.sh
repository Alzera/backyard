#!/bin/bash

set -e
rm -rf coverage

RUSTFLAGS="-C instrument-coverage" LLVM_PROFILE_FILE="default_%p_%m.profraw" cargo test --workspace

grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o coverage/
rm -rf default*.profraw */**/default*.profraw

for arg in "$@"; do
  if [[ "$arg" == "--open" ]]; then
    open coverage/index.html
    break
  fi
done