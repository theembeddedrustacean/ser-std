// Simplified Embedded Rust
// Standard Library Edition
// CH8-Q6 Wiring Template

// Build a door lock system using the keypad you built in the GPIO questions, a servo, a red LED, and
// a green LED. The system should prompt the user to enter a 4-digit code using the keypad. Upon
// entering the code, the system should compare it with a code hardcoded in your application. If the
// entered code matches the hardcoded code, rotate the servo 90 degrees, then light up the green LED, indicating
// that the door is unlocked. Otherwise, if the entered code does not match the hardcoded code,
// the red LED should light up, indicating that the door is still locked.

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello world!");
}
