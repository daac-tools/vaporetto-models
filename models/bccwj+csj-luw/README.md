# bccwj+csj-luw

A Vaporetto model for long unit word (LUW) segmentation. Compared to `bccwj-luw`, which is trained only on written
language, this model is also trained on spoken language (CSJ).

```
% echo '駒と盤は持っていません。' | predict --model bccwj+csj-luw.model.zst
```

## Training data

- [Balanced Corpus of Contemporary Written Japanese (BCCWJ)](https://clrd.ninjal.ac.jp/bccwj/en/): long units of the
  core data (M-XML), excluding newspapers (PN).
- [Corpus of Spontaneous Japanese (CSJ)](https://clrd.ninjal.ac.jp/csj/en/): long units (`MORPH/LDB`) of the 396 talks
  whose morphological information was manually annotated. Each utterance unit is used as a sentence. Fillers are
  removed, and utterance units containing other tags (disfluencies, masked words, etc.) are not used.

The training data contain 728,553 words from BCCWJ and 370,185 words from CSJ.

## Evaluation

The following results are obtained by models trained on the same data except for 5% of the CSJ talks, which are used as
test data (19,297 words). The window size and the n-gram length are set to the same value.

| Window size | Precision | Recall | F1 |
| --- | --- | --- | --- |
| 3 | 97.05% | 96.25% | 96.65% |
| 4 | 97.54% | 96.86% | 97.20% |
| 5 (this model) | 97.53% | 96.86% | 97.20% |

## Building

`build.sh` expects the following files in `../../resources`:

- `bccwj_core/*.xml` (M-XML of the BCCWJ core data)
- `csj/LDB/` and `csj/fileList.csv` (`MORPH/LDB` and `fileList.csv` of CSJ)

The model is trained with `--solver 5`, and the window sizes and n-gram lengths are set to 5 as in `bccwj-luw`.

## License

This model is licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

This model was created using the following language resources:

- Kikuo Maekawa, Makoto Yamazaki, Toshinobu Ogiso, Takehiko Maruyama, Hideki Ogura, Wakako Kashino, Hanae Koiso,
  Masaya Yamaguchi, Makiro Tanaka, and Yasuharu Den. 2014. Balanced Corpus of Contemporary Written Japanese.
  Language Resources and Evaluation, 48(2):345–371.
- Kikuo Maekawa. 2003. Corpus of Spontaneous Japanese: Its design and evaluation. In Proceedings of the ISCA & IEEE
  Workshop on Spontaneous Speech Processing and Recognition (SSPR 2003).
