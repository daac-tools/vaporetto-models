#!/bin/bash
set -e

DIRNAME="$(dirname $0)"

mkdir -p "${DIRNAME}/../work"

find "${DIRNAME}/../../resources/bccwj_core" -name "*.xml" ! -name "PN*" \
    | sort \
    | xargs cargo run --manifest-path "${DIRNAME}/../../convert_bccwj_xml/Cargo.toml" --release -- \
        --luw \
  > "${DIRNAME}/../work/bccwj_core-luw.txt"

find "${DIRNAME}/../../resources/csj/LDB" -name "*.ldb" \
    | sort \
    | xargs cargo run --manifest-path "${DIRNAME}/../../convert_csj/Cargo.toml" --release -- \
        --file-list "${DIRNAME}/../../resources/csj/fileList.csv" \
  > "${DIRNAME}/../work/csj-luw.txt"

cargo run --manifest-path "${DIRNAME}/../../vaporetto/train/Cargo.toml" --release -- \
    --tok "${DIRNAME}/../work/bccwj_core-luw.txt" \
    --tok "${DIRNAME}/../work/csj-luw.txt" \
    --solver 5 \
    --charw 5 \
    --charn 5 \
    --typew 5 \
    --typen 5 \
    --model "${DIRNAME}/bccwj+csj-luw.model.zst"

tar cJf "${DIRNAME}/../bccwj+csj-luw.tar.xz" -C "${DIRNAME}/.." "bccwj+csj-luw"
