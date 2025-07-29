use esp_idf_hal::gpio::*;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::prelude::*;
use esp_idf_hal::sys::link_patches;
use log::info;

fn main() {
    // Initialize ESP-IDF patches
    link_patches();

    // The log crate needs to be bound to a backend, and EspLogger does that for us here.
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("=== ESP32-S3 LED Blink Test ===");
    info!("Starting initialization...");

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    info!("Peripherals initialized successfully");


    let mut led = match PinDriver::output(pins.gpio48) {
        Ok(pin) => pin,
        Err(e) => {
            info!("Failed to initialize GPIO48: {:?}", e);
            return;
        }
    };
    
    info!("LED initialized successfully!");
    info!("Starting blink loop...");
    info!("Press Ctrl+C to stop");

    let mut counter = 0;
    loop {
        counter += 1;
        
        // Turn the LED on
        led.set_high().unwrap();
        info!("Cycle {}: LED ON", counter);
        FreeRtos::delay_ms(500);

        // Turn the LED off
        led.set_low().unwrap();
        info!("Cycle {}: LED OFF", counter);
        FreeRtos::delay_ms(500);

        // Turn the LED on
        led.set_high().unwrap();
        info!("Cycle {}: LED ON", counter);
        FreeRtos::delay_ms(500);

        // Turn the LED off
        led.set_low().unwrap();
        info!("Cycle {}: LED OFF", counter);
        FreeRtos::delay_ms(1000);
    }
}