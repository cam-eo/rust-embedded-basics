use esp_idf_hal::gpio::*;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::prelude::*;
use esp_idf_hal::sys::link_patches;

fn main() {
    // Initialize ESP-IDF patches
    link_patches();

    esp_println::println!("=== ESP32-S3 LED Blink Test ===");
    esp_println::println!("Starting initialization...");

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    esp_println::println!("Peripherals initialized successfully");


    let mut led = match PinDriver::output(pins.gpio48) {
        Ok(pin) => pin,
        Err(e) => {
            esp_println::println!("Failed to initialize GPIO48: {:?}", e);
            return;
        }
    };
    
    esp_println::println!("LED initialized successfully!");
    esp_println::println!("Starting blink loop...");
    esp_println::println!("Press Ctrl+C to stop");

    let mut counter = 0;
    loop {
        counter += 1;
        
        // Turn the LED on
        led.set_high().unwrap();
        esp_println::println!("Cycle {}: LED ON", counter);
        FreeRtos::delay_ms(500);

        // Turn the LED off
        led.set_low().unwrap();
        esp_println::println!("Cycle {}: LED OFF", counter);
        FreeRtos::delay_ms(500);

        // Turn the LED on
        led.set_high().unwrap();
        esp_println::println!("Cycle {}: LED ON", counter);
        FreeRtos::delay_ms(500);

        // Turn the LED off
        led.set_low().unwrap();
        esp_println::println!("Cycle {}: LED OFF", counter);
        FreeRtos::delay_ms(1000);
    }
}