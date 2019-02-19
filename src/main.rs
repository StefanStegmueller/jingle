#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate time;

mod audio_out;
mod config;
mod file_reader;
mod notes;

use audio_out::{AnalogOut, AudioOut, DigitalOut};
use config::Mode;
use std::error::Error;
use std::process;
use std::thread;
use std::time::Duration;

fn main() {
    let config = config::Config::new().unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });;

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let output: Box<AudioOut> = match config.mode {
        Mode::Digital => DigitalOut::new(config.gpio, config.duty_cycle)?,
        Mode::Analog => AnalogOut::new(config.i2c_address, config.wave)?,
    };

    println!("Using jingle of file {}.", config.filename);

    loop {
        let jingle = file_reader::read(&config.filename)?;
        output.play(jingle)?;

        thread::sleep(Duration::from_secs(2));
    }
}
