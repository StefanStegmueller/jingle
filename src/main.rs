use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;

extern crate time;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const GPIO_AUDIO: u8 = 2;
const T_HALF: u64 = 1;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting audio output on pin {}.", GPIO_AUDIO);

    gen_frequency(500, 2000);
    gen_frequency(800, 2000);

    Ok(())
}

fn gen_frequency(hz: u64, duration_millis: u64){
    let t = 1 / hz;
    let t_micro = t * 1000000;
    let t_half_millis = t_micro / 2;

    let end_time = current_time_millis() + duration_millis;

    while current_time_millis() < end_time {
        gen_period(t_half_millis);
    } 
}

fn gen_period(t_half_millis: u64) -> Result<(), Box<dyn Error>>{
    let mut pin = Gpio::new()?.get(GPIO_AUDIO)?.into_output();

    pin.toggle();
    thread::sleep(Duration::from_millis(t_half_millis));

    Ok(())
}

fn current_time_millis() -> u64 {
    let current_time = time::get_time();

    //Calculate milliseconds
    (current_time.sec as u64 * 1000) + 
        (current_time.nsec as u64 / 1000 / 1000)
}