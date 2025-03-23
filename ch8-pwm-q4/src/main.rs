// Simplified Embedded Rust
// Standard Library Edition
// CH8-Q4 Wiring Template

// Refactor the LED fading code such that the level of fading can be controlled by an input potentiometer
// (same as the one used in the ADC questions).

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello world!");
}
