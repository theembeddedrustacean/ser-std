// Simplified Embedded Rust
// Standard Library Edition
// CH5-Q6 Wiring Template

// Develop a program to generate a custom LED pattern on three GPIO pins.
// The pattern should repeat indefinitely, cycling through turning on and
// off each LED in sequence (e.g., LED1 on, LED2 on, LED3 on, LED1 off,
// LED2 off, LED3 off, repeat).

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello world!");
}
