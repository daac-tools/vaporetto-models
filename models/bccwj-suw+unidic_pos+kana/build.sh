#!/bin/bash
set -e

DIRNAME="$(dirname $0)"

mkdir -p "${DIRNAME}/../work"

find "${DIRNAME}/../../resources" -name "*.xml" \
    | sort \
    | xargs cargo run --manifest-path "${DIRNAME}/../../convert_bccwj_xml/Cargo.toml" --release -- \
        --attr pos \
        --attr "kana|formBase" \
  > "${DIRNAME}/../work/bccwj-suw_pos+kana.txt"

cargo run --manifest-path "${DIRNAME}/../../convert_unidic_csv/Cargo.toml" --release -- \
    --tag "{4}-{5}-{6}-{7}|{4}-{5}-{6}|{4}-{5}|{4}" \
    --tag "{24}|" \
    "${DIRNAME}/../../resources/lex_3_1.csv" \
  > "${DIRNAME}/../work/unidic_pos+kana.txt"

cargo run --manifest-path "${DIRNAME}/../../vaporetto/train/Cargo.toml" --release -- \
    --tok "${DIRNAME}/../work/bccwj-suw_pos+kana.txt" \
    --dict "${DIRNAME}/../work/unidic_pos+kana.txt" \
    --solver 5 \
    --model "${DIRNAME}/bccwj-suw+unidic_pos+kana.model.zst"

tar cJf "${DIRNAME}/../bccwj-suw+unidic_pos+kana.tar.xz" -C "${DIRNAME}/.." "bccwj-suw+unidic_pos+kana"
