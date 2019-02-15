use super::file_reader;
use super::notes;

use rppal::gpio::{Gpio, OutputPin};
use rppal::i2c::I2c;
use std::cell::RefCell;
use std::error::Error;
use std::f64::consts::PI;
use std::thread;
use std::time::Duration;

pub struct DigitalOut {
    pin: RefCell<OutputPin>,
    duty: u8,
}

pub struct AnalogOut {
    i2c: RefCell<I2c>,
}

pub trait AudioOut {
    fn play(&self, jingle: Vec<file_reader::Record>) -> Result<(), Box<dyn Error>>;
}

impl DigitalOut {
    pub fn new(gpio: u8, duty: u8) -> Result<(Box<DigitalOut>), Box<dyn Error>> {
        println!("Starting audio output using pin {}.", gpio);

        let mut pin = Gpio::new()?.get(gpio)?.into_output();

        // init pin
        pin.toggle();
        thread::sleep(Duration::from_millis(1000));
        pin.toggle();

        Ok(Box::new(DigitalOut {
            pin: RefCell::new(pin),
            duty: duty,
        }))
    }

    fn gen_frequency(&self, f_hz: f64, duration_millis: u64) {
        println!(
            "Generate frequency with {} hz for {} ms",
            f_hz, duration_millis
        );

        let t = 1.00 / f_hz;
        let t_micros = t * 1000000.0;
        let t_high_micros = t_micros * (self.duty as f64) / 100.00;
        let t_low_micros = t_micros * (100.00 - (self.duty as f64)) / 100.00;

        let start_time = current_time_millis();
        let end_time = start_time + duration_millis;

        while current_time_millis() < end_time {
            self.gen_period(t_high_micros, t_low_micros);
        }
    }

    fn gen_period(&self, t_high_micros: f64, t_low_micros: f64) {
        self.pin.borrow_mut().set_high();
        thread::sleep(Duration::from_micros(t_high_micros as u64));
        self.pin.borrow_mut().set_low();
        thread::sleep(Duration::from_micros(t_low_micros as u64));
    }
}

impl AudioOut for DigitalOut {
    fn play(&self, jingle: Vec<file_reader::Record>) -> Result<(), Box<dyn Error>> {
        for record in jingle.iter() {
            let hz = notes::parse_note(&record.note).unwrap_or_else(|error| {
                panic!("failed to parse note [{}]: {:?}", record.note, error);
            });
            let duration_millis = record.duration;

            if notes::is_mute(hz) {
                println!("Pause for {} ms", duration_millis);
                thread::sleep(Duration::from_millis(duration_millis as u64));
            } else {
                self.gen_frequency(hz, duration_millis);
            }
        }

        Ok(())
    }
}

impl AnalogOut {
    pub fn new(i2c_address: u16) -> Result<(Box<AnalogOut>), Box<dyn Error>> {
        println!("Starting audio output using i2c address {}.", i2c_address);

        let mut i2c = I2c::new()?;
        i2c.set_slave_address(i2c_address)?;

        Ok(Box::new(AnalogOut {
            i2c: RefCell::new(i2c),
        }))
    }

    fn dec_to_regdata(&self, dec: u16) -> [u8; 2] {
        [((dec >> 4) & 0xFF) as u8, ((dec << 4) & 0xFF) as u8]
    }

    fn gen_frequency(&self, f_hz: f64, duration_millis: u64) -> Result<(), Box<dyn Error>> {
        println!(
            "Generate frequency with {} hz for {} ms",
            f_hz, duration_millis
        );

        let t = 1.00 / f_hz;
        let t_micros = t * 1000000.0;

        let start_time = current_time_millis();
        let end_time = start_time + duration_millis;

        while current_time_millis() < end_time {
            let x = current_time_millis() - start_time;

            self.gen_voltage(x, t_micros)?;
        }
        Ok(())
    }

    fn gen_voltage(&self, x: u64, t_micros: f64) -> Result<(), Box<dyn Error>> {
        let raw_value = 2047.0 * (2.0 * PI / t_micros * (x as f64)).sin() + 2047.0;

        let reg_data: [u8; 2] = self.dec_to_regdata(raw_value as u16);

        self.i2c.borrow_mut().block_write(0, &reg_data)?;
        Ok(())
    }
}

impl AudioOut for AnalogOut {
    fn play(&self, jingle: Vec<file_reader::Record>) -> Result<(), Box<dyn Error>> {
        for record in jingle.iter() {
            let hz = notes::parse_note(&record.note).unwrap_or_else(|error| {
                panic!("failed to parse note [{}]: {:?}", record.note, error);
            });
            let duration_millis = record.duration;

            if notes::is_mute(hz) {
                println!("Pause for {} ms", duration_millis);
                thread::sleep(Duration::from_millis(duration_millis as u64));
            } else {
                self.gen_frequency(hz, duration_millis)?;
            }
        }

        Ok(())
    }
}

fn current_time_millis() -> u64 {
    let current_time = time::get_time();

    //Calculate milliseconds
    (current_time.sec as u64 * 1000) + (current_time.nsec as u64 / 1000 / 1000)
}
