use esp_idf_hal::gpio::*;
use esp_idf_hal::prelude::*;
// use esp_idf_sys as _;

use std::{thread, time::Duration};

fn main() {
    let peripherals = Peripherals::take().unwrap();

    let mut red = PinDriver::output(peripherals.pins.gpio48).unwrap();
    let mut green = PinDriver::output(peripherals.pins.gpio47).unwrap();
    let mut blue = PinDriver::output(peripherals.pins.gpio21).unwrap();

    let blink_duration = Duration::from_millis(500);

    loop {
        // RED
        red.set_high().unwrap();
        green.set_low().unwrap();
        blue.set_low().unwrap();
        thread::sleep(blink_duration);

        // GREEN
        red.set_low().unwrap();
        green.set_high().unwrap();
        blue.set_low().unwrap();
        thread::sleep(blink_duration);

        // BLUE
        red.set_low().unwrap();
        green.set_low().unwrap();
        blue.set_high().unwrap();
        thread::sleep(blink_duration);

        // ALL OFF
        red.set_low().unwrap();
        green.set_low().unwrap();
        blue.set_low().unwrap();
        thread::sleep(blink_duration);
    }
}
