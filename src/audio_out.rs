use super::file_reader;
use super::notes;

use rppal::gpio;
use rppal::gpio::Gpio;
use std::cell::RefCell;
use std::error::Error;
use std::thread;
use std::time::Duration;

pub struct DigitalOut {
    pin: RefCell<gpio::OutputPin>,
    duty: u8,
}

pub struct AnalogOut {
    i2c_address: u8,
    gpio_sda: u8,
    gpio_scl: u8,
}

pub trait AudioOut {
    fn play(&self, jingle: Vec<file_reader::Record>);
}

impl DigitalOut {
    pub fn new(gpio: u8, duty: u8) -> Result<(Box<DigitalOut>), Box<dyn Error>> {
        println!("Starting audio output on pin {}.", gpio);

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

        let start_time = self.current_time_millis();
        let end_time = start_time + duration_millis;

        while self.current_time_millis() < end_time {
            self.gen_period(t_high_micros, t_low_micros);
        }
    }

    fn gen_period(&self, t_high_micros: f64, t_low_micros: f64) {
        self.pin.borrow_mut().set_high();
        thread::sleep(Duration::from_micros(t_high_micros as u64));
        self.pin.borrow_mut().set_low();
        thread::sleep(Duration::from_micros(t_low_micros as u64));
    }

    fn current_time_millis(&self) -> u64 {
        let current_time = time::get_time();

        //Calculate milliseconds
        (current_time.sec as u64 * 1000) + (current_time.nsec as u64 / 1000 / 1000)
    }
}

impl AudioOut for DigitalOut {
    fn play(&self, jingle: Vec<file_reader::Record>) {
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
    }
}

impl AnalogOut {
    pub fn new(
        i2c_address: u8,
        gpio_sda: u8,
        gpio_scl: u8,
    ) -> Result<(Box<AnalogOut>), Box<dyn Error>> {
        Ok(Box::new(AnalogOut {
            i2c_address: i2c_address,
            gpio_sda: gpio_sda,
            gpio_scl: gpio_scl,
        }))
    }
}

impl AudioOut for AnalogOut {
    fn play(&self, jingle: Vec<file_reader::Record>) {}
}
