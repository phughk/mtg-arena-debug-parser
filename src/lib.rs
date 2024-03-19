use std::borrow::Cow;
use std::collections::BTreeMap;
use std::io::BufRead;
use log::info;
use rayon::prelude::*;


const INITIAL_SIZE: usize = 200_000;

// MacOS: /Users/hugh/Library/ApplicationSupport/com.wizards.mtga/Logs/Logs/UTC_Log - *.log

pub struct LogFileData {
    metadata: Metadata,
    decks: Vec<Deck>,
    games: Vec<Game>,
}

pub struct Metadata {}
pub struct Deck {}
pub struct Game {}

fn parse_file() -> LogFileData {
    let file = std::fs::File::open("samples/Player.log").unwrap();
    let reader = std::io::BufReader::new(file);
    let mut buff = Vec::with_capacity(INITIAL_SIZE);
    for (line_number, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        buff.push({
            let mut d = RawDataEntry {
                data: line,
                line_number,
                parsed: None,
            };
            d.parse_line(&mut d).unwrap();
            d
        });
    }
    let v: Vec<ParsedDataEntry> = buff.into_par_iter().map(|line| parse_line(line)).collect();
    v.iter().find(|v| {
        match v {
            ParsedDataEntry::Unknown(_) => true,
            _ => false,
        }
    }).iter().next().unwrap();
    LogFileData {
        metadata: Metadata {},
        decks: vec![],
        games: vec![],
    }
}

#[derive(Debug)]
struct RawDataEntry {
    data: String,
    line_number: usize,
    parsed: Option<ParsedDataEntry>,
}

impl RawDataEntry {
    fn parse_line(&mut self, input: &mut RawDataEntry) -> Result<&Self, ()>{
        if let Ok(jsoned) = serde_json::from_str(&input.data) {
            match jsoned {
                serde_json::Value::Object(_) =>  {
                    self.parsed = Some(ParsedDataEntry::JSON);
                    return Ok(self);
                }
                serde_json::Value::Array(_) => {
                    self.parsed=Some(ParsedDataEntry::JSON);
                    return Ok(self);
                },
                _ => panic!("Unexpected JSON type, {}", jsoned),

            }
        }
        self.parsed = Some(ParsedDataEntry::Unknown);
        Err(())
    }
}

enum ParsedDataEntry {
    /// Uncategorised data
    Unknown,
    /// Data that we know is useless
    Irrelevant,
    /// Data can be JSON representation
    JSON,
}


#[test]
fn main() {
    tracing_subscriber::fmt::init();
    info!("Started reading");
    let data = parse_file();
    info!("Finished reading");
}
