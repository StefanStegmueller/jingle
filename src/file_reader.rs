extern crate csv;

use super::notes;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize)]
struct Record {
    note: String,
    duration: u64,
}

pub struct Note {
    pub hz: f64,
    pub duration: u64,
}

pub fn read(filename: &str) -> Result<(Vec<Note>), Box<Error>> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);
    let mut rdr = csv::Reader::from_reader(reader);

    let mut notes: Vec<Note> = Vec::new();

    for result in rdr.deserialize() {
        let record: Record = result?;

        let hz: f64 = notes::parse_note(&record.note).unwrap_or_else(|error| {
            panic!("failed to parse note [{}]: {:?}", record.note, error);
        });

        let note = Note {
            hz: hz,
            duration: record.duration,
        };

        notes.push(note);
    }

    Ok(notes)
}
