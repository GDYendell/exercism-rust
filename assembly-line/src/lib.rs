// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

static MAX_CARS_PER_HOUR: f64 = 221.0;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    match speed {
        0..=4 => speed as f64 * MAX_CARS_PER_HOUR,
        5..=8 => speed as f64 * MAX_CARS_PER_HOUR * 0.9,
        9..=10 => speed as f64 * MAX_CARS_PER_HOUR * 0.77,
        _ => panic!("Illegal speed"),
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
