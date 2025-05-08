#!/usr/bin/env sh

cargo fmt --all
cargo clippy --all --release -- -D warnings