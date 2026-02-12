// < 1 expo, 1 linear, > 1 log
// Essentially just graph 2^exponent and grab value at x
pub fn exp_map(x: u16, exponent: f32) -> u16 {
    let x_norm = x as f32 / 255.0;
    let y_norm = x_norm.powf(exponent);

    (y_norm * 255.0).round().clamp(0.0, 255.0) as u16
}

// Copied from the evded documentation, altered so 1 device will auto choose.
pub fn get_device() -> evdev::Device {
    use std::io::prelude::*;
    let mut devices = evdev::enumerate().map(|t| t.1).collect::<Vec<_>>();
    devices.reverse();
    let choice;
    if devices.len() == 0 {
        choice = 0;
    }
    else {
        println!("Select Device [0-{}]: ", devices.len() - 1);
        let _ = std::io::stdout().flush();
        let mut chosen = String::new();
        std::io::stdin().read_line(&mut chosen).unwrap();
        choice = chosen.trim().parse::<usize>().unwrap();
    }
    devices.into_iter().nth(choice).unwrap()
}