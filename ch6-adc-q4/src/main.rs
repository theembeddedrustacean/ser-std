// Simplified Embedded Rust
// Standard Library Edition
// CH6-Q4 Wiring Template

// Wokwi has an analog joystick component that provides a range of analog values depending on the
// direction. Create an application that prints out the position of the joystick.

// Hint:
// Heres a link to the datasheet to help understand how it works
// https://components101.com/modules/joystick-module
// You can also click on the question mark icon when selecting the joystick for the Wokwi documentation

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello world!");
}
