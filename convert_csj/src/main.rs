use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use clap::Parser;
use md5::{Digest, Md5};
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Parser, Debug)]
#[clap(
    name = "convert_csj",
    about = "A program to convert SDB or LDB files of CSJ."
)]
struct Args {
    /// fileList.csv of CSJ.
    ///
    /// Only talks whose morphological information was manually annotated are used.
    #[clap(long)]
    file_list: PathBuf,

    /// Adds pronunciation tags.
    #[clap(long)]
    pron: bool,

    /// Uses only talks not included in the test set.
    ///
    /// The test set consists of 5% of the talks, determined by the MD5 hashes of the talk IDs.
    #[clap(long, conflicts_with = "test")]
    train: bool,

    /// Uses only talks included in the test set.
    #[clap(long)]
    test: bool,

    /// Input SDB or LDB files.
    input_files: Vec<PathBuf>,
}

static UNIT_ID: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d{4}) \d+\.\d+-\d+\.\d+").unwrap());
static TAG: Lazy<Regex> = Lazy::new(|| Regex::new(r"[()<>×;\u{FFFD}]").unwrap());
static FILLER: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\(F [^()]*\)$").unwrap());

struct Unit {
    tokens: Vec<String>,
    valid: bool,
}

fn read_shift_jis(path: &Path) -> std::io::Result<String> {
    let bytes = fs::read(path)?;
    Ok(encoding_rs::SHIFT_JIS.decode(&bytes).0.into_owned())
}

fn manual_talks(path: &Path) -> std::io::Result<HashSet<String>> {
    let mut talks = HashSet::new();
    for line in read_shift_jis(path)?.lines().skip(1) {
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() > 6 && cols[6] == "手動" {
            talks.insert(cols[0].to_string());
        }
    }
    Ok(talks)
}

fn is_test(talk: &str) -> bool {
    let hash = Md5::digest(talk.as_bytes());
    u128::from_be_bytes(hash.into()) % 100 < 5
}

fn is_valid(s: &str) -> bool {
    !s.contains([' ', '/', '\\', '\u{3000}'])
}

fn parse_talk(text: &str, pron: bool) -> Vec<String> {
    let mut ids = vec![];
    let mut units: HashMap<&str, Unit> = HashMap::new();
    for line in text.lines() {
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() < 12 {
            continue;
        }
        let id = match UNIT_ID.captures(cols[3]) {
            Some(cap) => cap.get(1).unwrap().as_str(),
            None => continue,
        };
        let unit = units.entry(id).or_insert_with(|| {
            ids.push(id);
            Unit {
                tokens: vec![],
                valid: true,
            }
        });
        let surface = cols[7];
        let p = cols[10];
        if FILLER.is_match(p) {
            continue;
        }
        if TAG.is_match(surface) || TAG.is_match(p) {
            unit.valid = false;
            continue;
        }
        if surface.is_empty() || !is_valid(surface) || !is_valid(p) {
            unit.valid = false;
            continue;
        }
        if pron && !p.is_empty() {
            unit.tokens.push(format!("{}/{}", surface, p));
        } else {
            unit.tokens.push(surface.to_string());
        }
    }
    let mut results = vec![];
    for id in ids {
        let unit = &units[id];
        if unit.valid && !unit.tokens.is_empty() {
            results.push(unit.tokens.join(" "));
        }
    }
    results
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let talks = manual_talks(&args.file_list)?;

    for path in args.input_files {
        let talk = match path.file_stem().and_then(|s| s.to_str()) {
            Some(talk) => talk,
            None => continue,
        };
        if !talks.contains(talk) {
            continue;
        }
        if args.train && is_test(talk) || args.test && !is_test(talk) {
            continue;
        }
        let results = parse_talk(&read_shift_jis(&path)?, args.pron);
        for result in results {
            println!("{}", result);
        }
    }
    Ok(())
}
