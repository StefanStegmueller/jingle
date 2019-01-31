extern crate csv;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize)]
pub struct Record {
    pub note: String,
    pub duration: u64,
}

pub fn read(filename: &str) -> Result<(Vec<Record>), Box<Error>> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);
    let mut rdr = csv::Reader::from_reader(reader);

    let mut records: Vec<Record> = Vec::new();

    for result in rdr.deserialize() {
        let record: Record = result?;
        records.push(record);
    }
    Ok(records)
}
