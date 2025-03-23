// Simplified Embedded Rust
// Standard Library Edition
// CH5-Q6 Wiring Template

// In the Wokwi components list there exists a PIR motion sensor component
// that simulates motion. The PIR sensor has three pins; a power, ground,
// and digital output (OUT). Triggering the sensor will drive the OUT pin
// high for 5 seconds, and then go low again. The sensor will ignore any
// further input for the next 1.2 seconds, and then start sensing for motion
// again. Create a project that upon detecting movement turns on a red LED.

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello world!");
}
