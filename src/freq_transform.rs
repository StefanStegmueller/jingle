pub fn saw(x: f64, f: f64, duration: f64) -> f64 {
    if x < (duration) / 2.0 {
        2.0 * f * x / duration
    } else {
        -(2.0 * f / duration) + 2.0 * duration
    }
}

pub fn parabel(x: f64, f: f64, duration: f64) -> f64 {
    let a2 = (4.0 * f) / (duration * (duration - 3.0));
    let a1 = -((4.0 + f * duration) / (1.0 - (3.0 / duration)));
    let a0 = 0.0;

    a2 * x.powi(2) + a1 * x + a0
}
