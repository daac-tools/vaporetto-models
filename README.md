# 🚤 Vaporetto models

This repository provides word segmentation models available in the fast tokenizer
[Vaporetto](https://github.com/daac-tools/vaporetto), as well as a set of programs for creating
each model.

The models are available on the [release page of Vaporetto](https://github.com/daac-tools/vaporetto/releases).
The models of this repository are for Vaporetto 0.7.0 or later.

## Usage

Clone this repository with the `vaporetto` submodule:

```
% git clone --recursive https://github.com/daac-tools/vaporetto-models
```

Create the `resources` directory directly under the repository root, copy the following files into it,
and then run `build.sh` in the `models` directory.

* `*.xml` files contained in the BCCWJ M-XML directory (models starting with `bccwj-`)
* `*.xml` files contained in the M-XML directory of the BCCWJ core data, placed in `resources/bccwj_core`
  (models starting with `bccwj+csj-`)
* `lex_3_1.csv` contained in [UniDic 3.1.1](https://clrd.ninjal.ac.jp/unidic/back_number.html#unidic_cwj)
  (models containing `unidic`)
* `MORPH/SDB` and `MORPH/LDB` directories and `fileList.csv` of [CSJ](https://clrd.ninjal.ac.jp/csj/), placed in
  `resources/csj` (models starting with `bccwj+csj-`)

```
resources/
├── *.xml
├── lex_3_1.csv
├── bccwj_core/
│   └── *.xml
└── csj/
    ├── fileList.csv
    ├── SDB/
    └── LDB/
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

See [the guidelines](./CONTRIBUTING.md).
