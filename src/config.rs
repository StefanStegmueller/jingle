use clap::{App, Arg};

pub struct Config {
    pub filename: String,
    pub duty_cycle: u32,
}

impl Config {
    pub fn new() -> Config {
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
                Arg::with_name("duty")
                    .short("d")
                    .long("duty")
                    .takes_value(true)
                    .default_value("50")
                    .help("Sets the duty cycle"),
            )
            .get_matches();

        let filename = matches.value_of("JINGLEFILE").unwrap();

        let duty_cycle = value_t!(matches, "duty", u32).unwrap();

        if duty_cycle == 0 || duty_cycle >= 100 {
            panic!("duty cycle has to be a value between 0 and 100");
        }

        let file_name_str = filename.to_string();

        Config {
            filename: file_name_str,
            duty_cycle,
        }
    }
}
