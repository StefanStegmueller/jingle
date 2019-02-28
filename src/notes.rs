use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ParseError {
    details: String,
}

impl ParseError {
    fn new(msg: &str) -> ParseError {
        ParseError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        &self.details
    }
}

lazy_static! {
    static ref NOTE_FREQUENCIES: HashMap<&'static str, f64> = [
        ("MUTE", 0.0),
        ("C0", 16.35),
        ("CSHARP0_Db0", 17.32),
        ("D0", 18.35),
        ("DSHARP0_Eb0", 19.45),
        ("E0", 20.60),
        ("F0", 21.83),
        ("FSHARP0_Gb0", 23.12),
        ("G0", 24.50),
        ("GSHARP0_Ab0", 25.96),
        ("A0", 27.50),
        ("ASHARP0_Bb0", 29.14),
        ("B0", 30.87),
        ("C1", 32.70),
        ("CSHARP1_Db1", 34.65),
        ("D1", 36.71),
        ("DSHARP1_Eb1", 38.89),
        ("E1", 41.20),
        ("F1", 43.65),
        ("FSHARP1_Gb1", 46.25),
        ("G1", 49.00),
        ("GSHARP1_Ab1", 51.91),
        ("A1", 55.00),
        ("ASHARP1_Bb1", 58.27),
        ("B1", 61.74),
        ("C2", 65.41),
        ("CSHARP2_Db2", 69.30),
        ("D2", 73.42),
        ("DSHARP2_Eb2", 77.78),
        ("E2", 82.41),
        ("F2", 87.31),
        ("FSHARP2_Gb2", 92.50),
        ("G2", 98.00),
        ("GSHARP2_Ab2", 103.83),
        ("A2", 110.00),
        ("ASHARP2_Bb2", 116.54),
        ("B2", 123.47),
        ("C3", 130.81),
        ("CSHARP3_Db3", 138.59),
        ("D3", 146.83),
        ("DSHARP3_Eb3", 155.56),
        ("E3", 164.81),
        ("F3", 174.61),
        ("FSHARP3_Gb3", 185.00),
        ("G3", 196.00),
        ("GSHARP3_Ab3", 207.65),
        ("A3", 220.00),
        ("ASHARP3_Bb3", 233.08),
        ("B3", 246.94),
        ("C4", 261.63),
        ("CSHARP4_Db4", 277.18),
        ("D4", 293.66),
        ("DSHARP4_Eb4", 311.13),
        ("E4", 329.63),
        ("F4", 349.23),
        ("FSHARP4_Gb4", 369.99),
        ("G4", 392.00),
        ("GSHARP4_Ab4", 415.30),
        ("A4", 440.00),
        ("ASHARP4_Bb4", 466.16),
        ("B4", 493.88),
        ("C5", 523.25),
        ("CSHARP5_Db5", 554.37),
        ("D5", 587.33),
        ("DSHARP5_Eb5", 622.25),
        ("E5", 659.25),
        ("F5", 698.46),
        ("FSHARP5_Gb5", 739.99),
        ("G5", 783.99),
        ("GSHARP5_Ab5", 830.61),
        ("A5", 880.00),
        ("ASHARP5_Bb5", 932.33),
        ("B5", 987.77),
        ("C6", 1046.50),
        ("CSHARP6_Db6", 1108.73),
        ("D6", 1174.66),
        ("DSHARP6_Eb6", 1244.51),
        ("E6", 1318.51),
        ("F6", 1396.91),
        ("FSHARP6_Gb6", 1479.98),
        ("G6", 1567.98),
        ("GSHARP6_Ab6", 1661.22),
        ("A6", 1760.00),
        ("ASHARP6_Bb6", 1864.66),
        ("B6", 1975.53),
        ("C7", 2093.00),
        ("CSHARP7_Db7", 2217.46),
        ("D7", 2349.32),
        ("DSHARP7_Eb7", 2489.02),
        ("E7", 2637.02),
        ("F7", 2793.83),
        ("FSHARP7_Gb7", 2959.96),
        ("G7", 3135.96),
        ("GSHARP7_Ab7", 3322.44),
        ("A7", 3520.00),
        ("ASHARP7_Bb7", 3729.31),
        ("B7", 3951.07),
        ("C8", 4186.01),
        ("CSHARP8_Db8", 4434.92),
        ("D8", 4698.63),
        ("DSHARP8_Eb8", 4978.03),
        ("E8", 5274.04),
        ("F8", 5587.65),
        ("FSHARP8_Gb8", 5919.91),
        ("G8", 6271.93),
        ("GSHARP8_Ab8", 6644.88),
        ("A8", 7040.00),
        ("ASHARP8_Bb8", 7458.62),
        ("B8", 7902.13),
    ]
    .iter()
    .cloned()
    .collect();
}

pub fn parse_note(note: &str) -> Result<(f64), ParseError> {
    match NOTE_FREQUENCIES.get(note) {
        Some(f) => Ok(*f),
        None => {
            let error_msg: &str = "Can't parse note description: ";
            Err(ParseError::new(&format!("{}{}", error_msg, note)))
        }
    }
}

pub fn is_mute(note_frequency: f64) -> bool {
    note_frequency == 0.0
}
