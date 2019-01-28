use rppal::gpio::Gpio;
use std::error::Error;
use std::thread;
use std::time::Duration;
extern crate time;

mod freq_transform;

// notes

const NOTE_G3: f64 = 196.00;
const NOTE_C4: f64 = 261.63;
const NOTE_E4: f64 = 329.63;
const NOTE_G4: f64 = 392.00;
const NOTE_MUTE: f64 = 0.0;

const GPIO_AUDIO: u8 = 2;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting audio output on pin {}.", GPIO_AUDIO);

    let jingle = [
        (NOTE_E4, 100),
        (NOTE_MUTE, 100),
        (NOTE_E4, 100),
        (NOTE_MUTE, 100),
        (NOTE_E4, 100),
        (NOTE_MUTE, 200),
        (NOTE_C4, 100),
        (NOTE_MUTE, 200),
        (NOTE_E4, 100),
        (NOTE_MUTE, 150),
        (NOTE_G4, 100),
        (NOTE_MUTE, 400),
        (NOTE_G3, 200),
    ];

    init_pin().expect("error");

    for note in jingle.iter() {
        let hz = note.0;
        let duration_millis = note.1;

        if hz == 0.0 {
            println!("Pause for {} ms", duration_millis);
            thread::sleep(Duration::from_millis(duration_millis as u64));
        } else {
            gen_frequency(hz, duration_millis);
        }
    }

    Ok(())
}

fn init_pin() -> Result<(), Box<dyn Error>> {
    let mut pin = Gpio::new()?.get(GPIO_AUDIO)?.into_output();

    pin.toggle();
    thread::sleep(Duration::from_millis(1000));
    pin.toggle();

    Ok(())
}

fn gen_frequency(t_hz: f64, duration_millis: u64) {
    println!(
        "Generate frequency with {} hz for {} ms",
        t_hz, duration_millis
    );

    let f = 1.00 / t_hz;
    let f_micros = f * 1000000.0;

    println!("f: {}, f_micros: {}", f, f_micros);

    let start_time = current_time_millis();
    let end_time = start_time + duration_millis;
    let duration_micros = (duration_millis * 1000) as f64;

    while current_time_millis() < end_time {
        let x_micros = ((current_time_millis() - start_time) * 1000) as f64;

        let f_trans = freq_transform::parabel(x_micros, f_micros, duration_micros);
        gen_period(f_trans).expect("error");
    }
}

fn gen_period(f_micros: f64) -> Result<(), Box<dyn Error>> {
    let f_half_micros = f_micros / 2.0;

    let mut pin = Gpio::new()?.get(GPIO_AUDIO)?.into_output();

    pin.toggle();
    thread::sleep(Duration::from_micros(f_half_micros as u64));

    Ok(())
}

fn current_time_millis() -> u64 {
    let current_time = time::get_time();

    //Calculate milliseconds
    (current_time.sec as u64 * 1000) + (current_time.nsec as u64 / 1000 / 1000)
}
