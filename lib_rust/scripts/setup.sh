#!/usr/bin/env sh

if ! command -v uv > /dev/null 2>&1
then
  curl -LsSf https://astral.sh/uv/install.sh | sh
fi

if ! command -v rustup > /dev/null 2>&1
then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
fi

if ! command -v cargo-nextest > /dev/null 2>&1
then
  cargo instal cargo-nextest
fi
