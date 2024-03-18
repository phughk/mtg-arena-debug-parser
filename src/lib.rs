use std::collections::BTreeMap;
use std::io::BufRead;
use log::info;
use rayon::prelude::*;


const INITIAL_SIZE: usize = 200_000;

fn parse_file() -> BTreeMap<usize, u32> {
    let file = std::fs::File::open("samples/Player.log").unwrap();
    let reader = std::io::BufReader::new(file);
    let mut buff = Vec::with_capacity(INITIAL_SIZE);
    for (line_number, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        buff.push(RawDataEntry {
            data: line,
            line_number,
        });
    }
    buff.par_iter().map(|line| parse_line(line))
        .map(|entry| {
            let mut map = BTreeMap::new();
            map.insert(entry.data().len(), 1);
            map
        })
        .reduce(|| BTreeMap::new(), |mut acc, mut map| {
            for (key, value) in map.into_iter() {
                *acc.entry(key).or_insert(0) += value;
            }
            acc
        })
}

#[derive(Debug)]
struct RawDataEntry {
    data: String,
    line_number: usize,
}

enum ParsedDataEntry<'a> {
    /// Uncategorised data
    Unknown(&'a RawDataEntry),
    /// Data that we know is useless
    Irrelevant(&'a RawDataEntry),
    /// Data can be JSON representation
    JSON(&'a RawDataEntry),
}

impl ParsedDataEntry<'_> {
    fn data(&self) -> &str {
        match self {
            ParsedDataEntry::Unknown(entry) => &entry.data,
            ParsedDataEntry::Irrelevant(entry) => &entry.data,
            ParsedDataEntry::JSON(entry) => &entry.data,
        }
    }
}

fn parse_line(input: &RawDataEntry) -> ParsedDataEntry{
    let jsoned = serde_json::from_str(&input.data);
    if jsoned.is_ok() {
        return ParsedDataEntry::JSON(input);
    }
    ParsedDataEntry::Unknown(input)
}

#[test]
fn main() {
    tracing_subscriber::fmt::init();
    info!("Started reading");
    let lines = parse_file();
    info!("Finished reading");
    println!("Number of lines: {}", lines.len());
    for (k,v) in lines.iter() {
        println!("{}: {}", k, v);
    }
}
