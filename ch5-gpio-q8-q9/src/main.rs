// Simplified Embedded Rust
// Standard Library Edition
// CH5-Q8 & CH5-Q9 Wiring Templates

// Question 7
// In the Wokwi components, under inputs, you can find a keypad component. This is a matrix keypad,
// which arranges the buttons in rows and columns. Here’s how it works and button presses detected:
// • Matrix Layout: The buttons on the keypad are arranged in a grid pattern, typically with rows
// and columns. Each button is at the intersection of a row and a column, forming a matrix
// • Scanning Rows and Columns: To detect button presses, the microcontroller scans each row
// and column of the keypad in sequence. It sets one-row line as output (high) and the rest as
// input. Then, it checks the status of each column line. If a column line reads low, it indicates
// that a button in that row is pressed.
// • Multiplexing: The scanning process is repeated for each row, cycling through all rows one by
// one.
// • Button Detection and Character Mapping: When a button on the keypad is pressed, it creates
// a connection between the corresponding row and column lines. Once a button press is detected, the microcontroller maps the row and column of the pressed button to a specific
// character or function based on the keypad layout. For example, if row 1 and column 1 are
// connected, the microcontroller knows that the ”1” button is pressed.
// Based on the above, create a simple application that detects the button pressed and prints it to the
// console.

// Link to Keypad Datasheet:
// https://components101.com/misc/4x4-keypad-module-pinout-configuration-features-datasheet

// Question 8
// Using the keypad, create

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello world!");
}
