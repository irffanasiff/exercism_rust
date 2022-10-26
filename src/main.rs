#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    (speed * 221) as f64
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    match speed {
        1 | 2 | 3 | 4 => production_rate_per_hour(speed) as u32 % 60,
        5 | 6 | 7 | 8 => production_rate_per_hour(speed) as u32 * 90 % 6000,
        9 | 10 => production_rate_per_hour(speed) as u32 * 77 % 6000,
        _ => production_rate_per_hour(speed) as u32,
    }
}

fn main() {
    println!("{}", working_items_per_minute(6));
}
