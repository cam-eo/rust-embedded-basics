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

    // Try different GPIO pins in order of preference
    let mut led = None;
    
    // List of GPIO pins to try (in order of preference)
    let gpio_pins = [2, 48, 10, 3, 1, 0];
    
    for &pin_num in &gpio_pins {
        esp_println::println!("Trying GPIO{}...", pin_num);
        
        let result = match pin_num {
            2 => PinDriver::output(pins.gpio2),
            48 => PinDriver::output(pins.gpio48),
            10 => PinDriver::output(pins.gpio10),
            3 => PinDriver::output(pins.gpio3),
            1 => PinDriver::output(pins.gpio1),
            0 => PinDriver::output(pins.gpio0),
            _ => continue,
        };
        
        match result {
            Ok(pin) => {
                esp_println::println!("Successfully initialized GPIO{}", pin_num);
                led = Some(pin);
                break;
            }
            Err(e) => {
                esp_println::println!("Failed to initialize GPIO{}: {:?}", pin_num, e);
            }
        }
    }
    
    let mut led = led.expect("Could not initialize any GPIO pin for LED");
    
    esp_println::println!("LED initialized successfully!");
    esp_println::println!("Starting blink loop (1 second intervals)...");
    esp_println::println!("Press Ctrl+C to stop");

    let mut counter = 0;
    loop {
        counter += 1;
        
        // Turn the LED on
        led.set_high().unwrap();
        esp_println::println!("Cycle {}: LED ON", counter);
        FreeRtos::delay_ms(1000);

        // Turn the LED off
        led.set_low().unwrap();
        esp_println::println!("Cycle {}: LED OFF", counter);
        FreeRtos::delay_ms(1000);
    }
}