/*
Simplified Embedded Rust: ESP Standard Library Edition
Programming GPIO - Blinky Application Example
*/

// Imports
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::peripherals::Peripherals;

fn main() {
    esp_idf_svc::sys::link_patches();

    // Take device peripherals
    let dp = Peripherals::take().unwrap();
    // Configure pin gpio1 to output
    let mut led = PinDriver::output(dp.pins.gpio1).unwrap();

    loop {
        // 1. Turn on LED
        led.set_high().unwrap();
        // 2. Delay for 1 second
        FreeRtos::delay_ms(1000_u32);
        // 3. Turn off LED
        led.set_low().unwrap();
        // 4. Delay for 1 second
        FreeRtos::delay_ms(1000_u32);
    }
}
