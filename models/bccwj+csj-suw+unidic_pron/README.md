# bccwj+csj-suw+unidic_pron

A Vaporetto model for short unit word (SUW) segmentation and pronunciation estimation. Compared to
`bccwj-suw+unidic_pos+pron`, which is trained only on written language, this model is also trained on spoken language
(CSJ) and estimates pronunciations closer to how words are actually spoken. This model does not estimate POS tags.

```
% echo '私は本当にそう思います。' | predict --model bccwj+csj-suw+unidic_pron.model.zst --predict-tags
```

## Training data

- [Balanced Corpus of Contemporary Written Japanese (BCCWJ)](https://clrd.ninjal.ac.jp/bccwj/en/): short units of the
  core data (M-XML), excluding newspapers (PN).
- [Corpus of Spontaneous Japanese (CSJ)](https://clrd.ninjal.ac.jp/csj/en/): short units (`MORPH/SDB`) of the 396 talks
  whose morphological information was manually annotated. Each utterance unit is used as a sentence. Fillers are
  removed, and utterance units containing other tags (disfluencies, masked words, etc.) are not used.
- [UniDic](https://clrd.ninjal.ac.jp/unidic/) 3.1.1: surfaces and pronunciations in `lex_3_1.csv`, used as a dictionary.

The training data contain 929,573 words from BCCWJ and 482,722 words from CSJ.

### Pronunciations of spoken language

BCCWJ annotates some words with fixed pronunciations of written language that differ from how they are pronounced in
speech. BCCWJ pronunciations of the following words, listed in `untagged_words.txt`, are not used for training (the
words are still used for segmentation):

| Word | BCCWJ | CSJ |
| --- | --- | --- |
| 私 | ワタクシ | ワタシ |
| 日本 | ニッポン | ニホン |
| 言う | イウ | ユー |
| 本当 | ホントー | ホント |
| 女房 | ニョーボー | ニョーボ |
| 決して | ケッシテ | ケシテ |

These words are selected by the following criteria: words containing kanji that appear at least 3 times in BCCWJ and at
least 10 times in CSJ, have the same pronunciation in at least 90% of the BCCWJ occurrences, and have that pronunciation
in less than 30% of the CSJ occurrences. 鼻, 舌, and 筋 are manually excluded because their differences are caused by
the domains of the talks (e.g., phonetics).

## Evaluation

The following results are obtained by a model trained on the same data except for 5% of the CSJ talks, which are used
as test data (25,342 words). "Words with kanji" is the ratio of words containing kanji whose segmentation and
pronunciation are both correct.

| Model | Segmentation F1 | Segmentation + pronunciation F1 | Words with kanji |
| --- | --- | --- | --- |
| bccwj-suw+unidic_pos+pron | 97.66% | 94.43% | 94.38% |
| bccwj+csj-suw+unidic_pron | 98.83% | 98.20% | 97.22% |

## Building

`build.sh` expects the following files in `../../resources`:

- `bccwj_core/*.xml` (M-XML of the BCCWJ core data)
- `csj/SDB/` and `csj/fileList.csv` (`MORPH/SDB` and `fileList.csv` of CSJ)
- `lex_3_1.csv` (UniDic 3.1.1)

The model is trained with `--solver 5` and the default window sizes and n-gram lengths (3).

## License

This model is distributed under the BSD license. See [BSD](BSD).

This model was created using the following language resources:

- Kikuo Maekawa, Makoto Yamazaki, Toshinobu Ogiso, Takehiko Maruyama, Hideki Ogura, Wakako Kashino, Hanae Koiso,
  Masaya Yamaguchi, Makiro Tanaka, and Yasuharu Den. 2014. Balanced Corpus of Contemporary Written Japanese.
  Language Resources and Evaluation, 48(2):345–371.
- Kikuo Maekawa. 2003. Corpus of Spontaneous Japanese: Its design and evaluation. In Proceedings of the ISCA & IEEE
  Workshop on Spontaneous Speech Processing and Recognition (SSPR 2003).
- UniDic 3.1.1, The UniDic Consortium.
