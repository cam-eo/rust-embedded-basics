use esp_idf_hal::gpio::*;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::prelude::*;

fn main() {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    let mut led = PinDriver::output(pins.gpio10).unwrap();

    loop {
        led.set_high().unwrap();
        FreeRtos::delay_ms(500);

        led.set_low().unwrap();
        FreeRtos::delay_ms(500);
    }
}
