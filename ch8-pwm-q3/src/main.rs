// Simplified Embedded Rust
// Standard Library Edition
// CH8-Q3 Wiring Template

// Refactor the LED fading code to control the position of a servo motor connected to a pin using PWM.
// Implement functions to move the servo motor smoothly from its minimum position to its maximum
// position and back again over a specified duration. Assume access to PWM functions in your library.
// You can rotate the servo motor shaft from 0𝑜 to 180𝑜 by varying the PWM pulse width from 1 ms to
// 2 ms for a 50 Hz (20 ms period) signal. This corresponds to a 5-10% duty cycle on a 50 Hz waveform.

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello world!");
}
