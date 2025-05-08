#!/usr/bin/env sh

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

"${SCRIPT_DIR}/setup.sh"

OLD_CWD=$(pwd)

if [ ! -d "${SCRIPT_DIR}/../pos/.venv" ]; then
  cd "${SCRIPT_DIR}/../pos"
  uv venv
  uv sync
  cd "${OLD_CWD}"
fi

cargo nextest run --all