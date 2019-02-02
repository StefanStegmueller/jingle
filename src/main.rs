#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate time;

use rppal::gpio;
use rppal::gpio::Gpio;
use std::env;
use std::error::Error;
use std::process;
use std::thread;
use std::time::Duration;

mod config;
mod file_reader;
mod freq_transform;
mod notes;

const GPIO_AUDIO: u8 = 2;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = config::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });;

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    println!("Starting audio output on pin {}.", GPIO_AUDIO);
    let mut pin = init_pin().expect("failed to initialize pin");

    println!("Using jingle of file {}.", config.filename);

    loop {
        let jingle = file_reader::read(&config.filename).expect("error reading file");
        play_jinge(&mut pin, jingle);

        thread::sleep(Duration::from_secs(2));
    }
}

fn init_pin() -> Result<(gpio::OutputPin), Box<dyn Error>> {
    let mut pin = Gpio::new()?.get(GPIO_AUDIO)?.into_output();

    pin.toggle();
    thread::sleep(Duration::from_millis(1000));
    pin.toggle();

    Ok(pin)
}

fn play_jinge(mut pin: &mut gpio::OutputPin, jingle: Vec<file_reader::Record>) {
    for record in jingle.iter() {
        let hz = notes::parse_note(&record.note).unwrap_or_else(|error| {
            panic!("failed to parse note [{}]: {:?}", record.note, error);
        });
        let duration_millis = record.duration;

        if notes::is_mute(hz) {
            println!("Pause for {} ms", duration_millis);
            thread::sleep(Duration::from_millis(duration_millis as u64));
        } else {
            gen_frequency(&mut pin, hz, duration_millis);
        }
    }
}

fn gen_frequency(mut pin: &mut gpio::OutputPin, f_hz: f64, duration_millis: u64) {
    println!(
        "Generate frequency with {} hz for {} ms",
        f_hz, duration_millis
    );

    let t = 1.00 / f_hz;
    let t_micros = t * 1000000.0;

    let start_time = current_time_millis();
    let end_time = start_time + duration_millis;

    while current_time_millis() < end_time {
        gen_period(&mut pin, t_micros);
    }
}

fn gen_period(pin: &mut gpio::OutputPin, t_micros: f64) {
    let t_half_micros = t_micros / 2.0;

    pin.set_high();
    thread::sleep(Duration::from_micros(t_half_micros as u64));
    pin.set_low();
    thread::sleep(Duration::from_micros(t_half_micros as u64));
}

fn current_time_millis() -> u64 {
    let current_time = time::get_time();

    //Calculate milliseconds
    (current_time.sec as u64 * 1000) + (current_time.nsec as u64 / 1000 / 1000)
}
