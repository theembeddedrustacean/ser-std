// Simplified Embedded Rust
// Standard Library Edition
// CH6-Q5 Wiring Template

// Using the LED bar and the rotating potentiometer, create an application that replicates a volume
// control dial. Meaning as the potentiometer dial is rotated clockwise, an increasing amount of LEDs
// light up indicating higher volume. Conversely, as the dial is rotated counter-clockwise, LEDs are
// turned off.

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello world!");
}
