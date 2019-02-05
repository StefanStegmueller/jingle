use clap::{App, Arg};
use std::error::Error;

pub struct Config {
    pub filename: String,
    pub gpio: u8,
    pub duty_cycle: u8,
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
            .get_matches();

        let filename = matches.value_of("JINGLEFILE").unwrap();
        let file_name_str = filename.to_string();

        let gpio = value_t!(matches, "gpio", u8).unwrap();

        let duty_cycle = value_t!(matches, "duty", u8).unwrap();

        Ok(Config {
            filename: file_name_str,
            gpio,
            duty_cycle,
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
