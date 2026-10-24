#!/bin/bash
set -e

DIRNAME="$(dirname $0)"

mkdir -p "${DIRNAME}/../work"

find "${DIRNAME}/../../resources/bccwj_core" -name "*.xml" ! -name "PN*" \
    | sort \
    | xargs cargo run --manifest-path "${DIRNAME}/../../convert_bccwj_xml/Cargo.toml" --release -- \
        --attr pron \
        --untagged-words "${DIRNAME}/untagged_words.txt" \
  > "${DIRNAME}/../work/bccwj_core-suw_pron.txt"

find "${DIRNAME}/../../resources/csj/SDB" -name "*.sdb" \
    | sort \
    | xargs cargo run --manifest-path "${DIRNAME}/../../convert_csj/Cargo.toml" --release -- \
        --file-list "${DIRNAME}/../../resources/csj/fileList.csv" \
        --pron \
  > "${DIRNAME}/../work/csj-suw_pron.txt"

cargo run --manifest-path "${DIRNAME}/../../convert_unidic_csv/Cargo.toml" --release -- \
    --tag "{13}|" \
    "${DIRNAME}/../../resources/lex_3_1.csv" \
  > "${DIRNAME}/../work/unidic_pron.txt"

cargo run --manifest-path "${DIRNAME}/../../vaporetto/train/Cargo.toml" --release -- \
    --tok "${DIRNAME}/../work/bccwj_core-suw_pron.txt" \
    --tok "${DIRNAME}/../work/csj-suw_pron.txt" \
    --dict "${DIRNAME}/../work/unidic_pron.txt" \
    --solver 5 \
    --model "${DIRNAME}/bccwj+csj-suw+unidic_pron.model.zst"

tar cJf "${DIRNAME}/../bccwj+csj-suw+unidic_pron.tar.xz" -C "${DIRNAME}/.." "bccwj+csj-suw+unidic_pron"
