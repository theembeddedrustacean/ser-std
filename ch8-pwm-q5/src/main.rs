// Simplified Embedded Rust
// Standard Library Edition
// CH8-Q5 Wiring Template

// Create a program to generate audio tones using PWM for a simple buzzer. A buzzer has two terminals,
// one terminal hooks up to the PWM output and the other to ground. Implement functions to
// play different musical notes by generating corresponding PWM signals with appropriate frequencies.

// Wiring
// The buzzer second terminal (PWM input) is connected to gpio 0.
// The buzzer first terminal is connected to ground.

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello world!");
}
