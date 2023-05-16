#!/bin/bash
set -e

DIRNAME="$(dirname $0)"

mkdir -p "${DIRNAME}/../work"

if [ ! -f "${DIRNAME}/../work/bccwj-suw.txt" ]; then
  find "${DIRNAME}/../../resources" -name "*.xml" \
      | sort \
      | xargs cargo run --manifest-path "${DIRNAME}/../../convert_bccwj_xml/Cargo.toml" --release -- \
    > "${DIRNAME}/../work/bccwj-suw.txt"
fi

cargo run --manifest-path "${DIRNAME}/../../vaporetto/train/Cargo.toml" --release -- \
    --tok "${DIRNAME}/../work/bccwj-suw.txt" \
    --solver 5 \
    --cost 0.003 \
    --model "${DIRNAME}/bccwj-suw_c0.003.model.zst"

tar cJf "${DIRNAME}/../bccwj-suw_c0.003.tar.xz" -C "${DIRNAME}/.." "bccwj-suw_c0.003"
