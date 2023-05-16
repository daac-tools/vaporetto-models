#!/bin/bash
set -e

DIRNAME="$(dirname $0)"

mkdir -p "${DIRNAME}/../work"

find "${DIRNAME}/../../resources" -name "*.xml" \
    | sort \
    | xargs cargo run --manifest-path "${DIRNAME}/../../convert_bccwj_xml/Cargo.toml" --release -- \
  > "${DIRNAME}/../work/bccwj-suw.txt"

cargo run --manifest-path "${DIRNAME}/../../vaporetto/train/Cargo.toml" --release -- \
    --tok "${DIRNAME}/../work/bccwj-suw.txt" \
    --solver 5 \
    --model "${DIRNAME}/bccwj-suw.model.zst"

tar cJf "${DIRNAME}/../bccwj-suw.tar.xz" -C "${DIRNAME}/.." "bccwj-suw"
