#!/bin/bash
set -e

DIRNAME="$(dirname $0)"

mkdir -p "${DIRNAME}/../work"

find "${DIRNAME}/../../resources" -name "*.xml" \
    | sort \
    | xargs cargo run --manifest-path "${DIRNAME}/../../convert_bccwj_xml/Cargo.toml" --release -- \
        --luw \
  > "${DIRNAME}/../work/bccwj-luw.txt"

cargo run --manifest-path "${DIRNAME}/../../vaporetto/train/Cargo.toml" --release -- \
    --tok "${DIRNAME}/../work/bccwj-luw.txt" \
    --solver 5 \
    --charw 5 \
    --charn 5 \
    --typew 5 \
    --typen 5 \
    --model "${DIRNAME}/bccwj-luw.model.zst"

tar cJf "${DIRNAME}/../bccwj-luw.tar.xz" -C "${DIRNAME}/.." "bccwj-luw"
