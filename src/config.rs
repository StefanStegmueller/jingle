use clap::{App, Arg};
use std::error::Error;

extern crate hex;

arg_enum! {
    #[derive(Debug)]
    pub enum Mode {
        Digital,
        Analog
    }
}

pub struct Config {
    pub filename: String,
    pub mode: Mode,
    pub gpio: u8,
    pub duty_cycle: u8,
    pub i2c_address: u16,
}

impl Config {
    pub fn new() -> Result<Config, Box<Error>> {
        let matches = App::new("jinglepi")
            .version(crate_version!())
            .author(crate_authors!())
            .about("Plays jingle with raspberry pi")
            .arg(
                Arg::with_name("JINGLEFILE")
                    .required(true)
                    .takes_value(true)
                    .help("Sets the jingle file to use"),
            )
            .arg(Arg::from_usage("<MODE> 'Set output mode'").possible_values(&Mode::variants()))
            .arg(
                Arg::with_name("gpio")
                    .short("g")
                    .long("gpio")
                    .takes_value(true)
                    .default_value("2")
                    .validator(is_gpio)
                    .help("Sets the gpio pin to use"),
            )
            .arg(
                Arg::with_name("duty")
                    .short("d")
                    .long("duty")
                    .takes_value(true)
                    .default_value("50")
                    .validator(is_duty)
                    .help("Sets the duty cycle"),
            )
            .arg(
                Arg::with_name("i2caddress")
                    .short("i")
                    .long("i2caddress")
                    .takes_value(true)
                    .default_value("62")
                    .help("Sets the i2c address (hex) for dac to use"),
            )
            .get_matches();

        let filename = matches.value_of("JINGLEFILE").unwrap();
        let file_name_str = filename.to_string();

        let mode = value_t!(matches.value_of("MODE"), Mode).unwrap();

        let gpio = value_t!(matches, "gpio", u8).unwrap();

        let duty_cycle = value_t!(matches, "duty", u8).unwrap();

        let i2c_address_str = matches.value_of("i2caddress").unwrap();
        let i2c_address = hex::decode(i2c_address_str)
            .unwrap()
            .iter()
            .fold(0, |acc, x| acc * 10 + x) as u16;

        Ok(Config {
            filename: file_name_str,
            mode,
            gpio,
            duty_cycle,
            i2c_address,
        })
    }
}

fn is_duty(val: String) -> Result<(), String> {
    let duty_cycle = match val.parse::<u8>() {
        Ok(i) => i,
        Err(_err) => {
            return Err(String::from(
                "Duty cycle has to be an unsigned integer (u8).",
            ));
        }
    };

    match duty_cycle == 0 || duty_cycle >= 100 {
        true => Err(String::from(
            "Duty cycle has to be a value between 0 and 100",
        )),
        false => Ok(()),
    }
}

fn is_gpio(val: String) -> Result<(), String> {
    let _gpio = match val.parse::<u8>() {
        Ok(i) => i,
        Err(_err) => return Err(String::from("ggio has to be an unsigned integer (u8).")),
    };

    Ok(())
}
