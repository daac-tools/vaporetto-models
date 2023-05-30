use std::fs::File;
use std::path::PathBuf;

use clap::Parser;
use csv::StringRecord;
use once_cell::sync::Lazy;
use regex::Regex;
use vaporetto::{CharacterBoundary, Sentence};

#[derive(Parser, Debug)]
#[clap(
    name = "convert_unidic_csv",
    about = "A program to parse XML files of BCCWJ."
)]
struct Args {
    /// Feature index to be used for tags./
    ///
    /// If multiple IDs are specified, separated by `|`, the value of the next attribute is used if
    /// the previous one does not exist.
    #[clap(long)]
    tag: Vec<String>,

    /// Input CSV file.
    csv_file: PathBuf,

    /// String representing null.
    #[clap(long, default_value = "*")]
    null_str: String,
}

const PLACE_HOLDER: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\{(\d+)\}").unwrap()
});


struct TagPattern {
    fragments: Vec<String>,
    indices: Vec<usize>,
}

impl TagPattern {
    fn new(pattern: &str) -> Self {
        let mut indices = vec![];
        for cap in PLACE_HOLDER.captures_iter(pattern) {
            indices.push(cap.get(1).unwrap().as_str().parse().unwrap());
        }
        let fragments = PLACE_HOLDER.split(pattern).map(String::from).collect();
        Self {
            fragments,
            indices,
        }
    }

    fn check(&self, record: &StringRecord, null_str: &str) -> bool {
        for &i in &self.indices {
            if let Some(feature) = record.get(i) {
                if feature == null_str {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    fn replace(&self, record: &StringRecord) -> String {
        let mut result = self.fragments[0].clone();
        for (&i, fragment) in self.indices.iter().zip(&self.fragments[1..]) {
            result.push_str(record.get(i).unwrap());
            result.push_str(fragment);
        }
        result
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let file = File::open(args.csv_file)?;
    let mut rdr = csv::Reader::from_reader(file);

    let patterns: Vec<Vec<TagPattern>> = args.tag.iter().map(|tag_pat| tag_pat.split('|').map(|pat| TagPattern::new(pat)).collect()).collect();

    let mut s = Sentence::default();
    let mut buf = String::new();
    for result in rdr.records() {
        let record = result?;
        if s.update_raw(record.get(0).unwrap().to_string()).is_err() {
            continue;
        }
        s.boundaries_mut().fill(CharacterBoundary::NotWordBoundary);
        s.reset_tags(args.tag.len());
        'a: for (i, patterns) in patterns.iter().enumerate() {
            for pattern in patterns {
                if !pattern.check(&record, &args.null_str) {
                    continue;
                }
                let len = s.tags().len();
                s.tags_mut()[len - args.tag.len() + i].replace(pattern.replace(&record).into());
                continue 'a;
            }
            panic!("Any tag candidate does not match");
        }
        s.write_tokenized_text(&mut buf);
        println!("{}", buf);
    }
    Ok(())
}
