// Simplified Embedded Rust
// Standard Library Edition
// CH7-Q2 Wiring Template

// Expand the real-time timer example to operate as a stopwatch. Add three input buttons; start, stop,
// and reset.

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello world!");
}
