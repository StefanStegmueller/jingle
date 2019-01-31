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

// notes frequences

pub const G3: f64 = 196.00;
pub const C4: f64 = 261.63;
pub const E4: f64 = 329.63;
pub const G4: f64 = 392.00;
pub const MUTE: f64 = 0.0;

pub fn parse_note(note: &str) -> Result<(f64), ParseError> {
    match note {
        "G3" => Ok(G3),
        "C4" => Ok(C4),
        "E4" => Ok(E4),
        "G4" => Ok(G4),
        "MUTE" => Ok(MUTE),
        &_ => {
            let error_msg: &str = "Can't parse note description: ";
            Err(ParseError::new(&format!("{}{}", error_msg, note)))
        }
    }
}
