use super::config::Wave;
use super::file_reader::Note;
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
    wave_func: Box<Fn(f64, f64) -> f64>,
}

pub trait AudioOut {
    fn play(&self, jingle: Vec<Note>) -> Result<(), Box<dyn Error>>;
    fn gen_frequency(&self, f_hz: f64, duration_millis: u64) -> Result<(), Box<dyn Error>>;
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

    fn gen_period(&self, t_high_micros: f64, t_low_micros: f64) {
        self.pin.borrow_mut().set_high();
        thread::sleep(Duration::from_micros(t_high_micros as u64));
        self.pin.borrow_mut().set_low();
        thread::sleep(Duration::from_micros(t_low_micros as u64));
    }
}

impl AudioOut for DigitalOut {
    fn play(&self, jingle: Vec<Note>) -> Result<(), Box<dyn Error>> {
        iter_notes(self, jingle)?;
        Ok(())
    }

    fn gen_frequency(&self, f_hz: f64, duration_millis: u64) -> Result<(), Box<dyn Error>> {
        let t = 1.00 / f_hz;
        let t_micros = t * 1000000.0;
        let t_high_micros = t_micros * (self.duty as f64) / 100.00;
        let t_low_micros = t_micros * (100.00 - (self.duty as f64)) / 100.00;

        let start_time = current_time_millis();
        let end_time = start_time + duration_millis;

        while current_time_millis() < end_time {
            self.gen_period(t_high_micros, t_low_micros);
        }

        Ok(())
    }
}

impl AnalogOut {
    pub fn new(i2c_address: u16, wave: Wave) -> Result<(Box<AnalogOut>), Box<dyn Error>> {
        println!("Starting audio output using i2c address {}.", i2c_address);

        let wave_func = Box::new(match wave {
            Wave::Rectangle => rectangle_wave,
            Wave::Sine => sine_wave,
            Wave::Triangle => triangle_wave,
            Wave::Saw => saw_wave,
        });

        let mut i2c = I2c::new()?;
        i2c.set_slave_address(i2c_address)?;

        Ok(Box::new(AnalogOut {
            i2c: RefCell::new(i2c),
            wave_func,
        }))
    }

    /// Generates signal using i2c-bus with 0V to 3.3V.
    /// raw_value = 0       => 0V,
    /// raw_value = 2047    => ~1.65V,
    /// raw_value = 4095    => 3.3V
    fn gen_voltage(&self, x: u64, t_micros: f64) -> Result<(), Box<dyn Error>> {
        let raw_value = (*self.wave_func)(x as f64, t_micros).round();

        let reg_data: [u8; 2] = self.dec_to_regdata(raw_value as u16);

        self.i2c.borrow_mut().block_write(0, &reg_data)?;
        Ok(())
    }

    fn dec_to_regdata(&self, dec: u16) -> [u8; 2] {
        [((dec >> 4) & 0xFF) as u8, ((dec << 4) & 0xFF) as u8]
    }
}

impl AudioOut for AnalogOut {
    fn play(&self, jingle: Vec<Note>) -> Result<(), Box<dyn Error>> {
        iter_notes(self, jingle)?;
        Ok(())
    }

    fn gen_frequency(&self, f_hz: f64, duration_millis: u64) -> Result<(), Box<dyn Error>> {
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
}

fn iter_notes(audio_out: &impl AudioOut, jingle: Vec<Note>) -> Result<(), Box<dyn Error>> {
    for note in jingle.iter() {
        let hz = note.hz;
        let duration_millis = note.duration;

        if notes::is_mute(hz) {
            println!("Pause for {} ms", duration_millis);
            thread::sleep(Duration::from_millis(duration_millis as u64));
        } else {
            println!(
                "Generate frequency with {} hz for {} ms",
                hz, duration_millis
            );
            audio_out.gen_frequency(hz, duration_millis)?;
        }
    }

    Ok(())
}

fn current_time_millis() -> u64 {
    let current_time = time::get_time();

    //Calculate milliseconds
    (current_time.sec as u64 * 1000) + (current_time.nsec as u64 / 1000 / 1000)
}

fn rectangle_wave(x: f64, t: f64) -> f64 {
    if x < (t / 2.0) {
        4095.0
    } else {
        0.0
    }
}

fn sine_wave(x: f64, t: f64) -> f64 {
    2047.0 * (2.0 * PI / t * (x)).sin() + 2047.0
}

fn triangle_wave(x: f64, t: f64) -> f64 {
    (4095.0 / PI) * ((2.0 * t.powi(-1) * PI * x).sin()).asin() + 2047.0
}

fn saw_wave(x: f64, t: f64) -> f64 {
    if x < (t / 2.0) {
        4095.0 * x / t + 2047.0
    } else {
        4095.0 * x / t - 2047.0
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_sine_wave() {
        let t: f64 = 3.0;

        assert_eq!(sine_wave(0.0, t), 2047.0);
        assert_eq!(sine_wave(t * (1.0 / 4.0), t).round(), 4094.0);
        assert_eq!(sine_wave(t / 2.0, t).round(), 2047.0);
        assert_eq!(sine_wave(t * (3.0 / 4.0), t).round(), 0.0);
        assert_eq!(sine_wave(t, t).round(), 2047.0);
    }

    #[test]
    fn test_triangle_wave() {
        let t: f64 = 3.0;

        assert_eq!(triangle_wave(0.0, t), 2047.0);
        assert_eq!(
            triangle_wave(t * (1.0 / 8.0), t).round(),
            ((4095.0 - (2047.5 / 2.0)) as f64).round()
        );
        assert_eq!(triangle_wave(t * (1.0 / 4.0), t).round(), 4095.0);
        assert_eq!(triangle_wave(t / 2.0, t).round(), 2047.0);
        assert_eq!(triangle_wave(t * (3.0 / 4.0), t).round(), 0.0);
        assert_eq!(triangle_wave(t, t).round(), 2047.0);
    }

    #[test]
    fn test_saw_wave() {
        let t: f64 = 3.0;

        assert_eq!(saw_wave(0.0, t), 2047.0);
        assert_eq!(
            saw_wave(t * (1.0 / 4.0), t).round(),
            ((4095.0 - (2047.5 / 2.0)) as f64).round()
        );
        assert_eq!(saw_wave(t / 2.0, t), 0.5);
        assert_eq!(saw_wave(t, t), 2048.0);
    }
}
