// Simplified Embedded Rust
// Core Library Edition
// CH6-Q6 & Q7 Wiring Template

// Illumination/light level is measured in lux. Moreover, a light-dependent resistor or LDR is a resistor
// that changes its value depending on the light level. In the components list, Wokwi contains a photoresistor
// module that contains an LDR. The circuit configuration is more or less similar to an NTC
// that was introduced in the application example, however, the conversion formulas are different. To
// find out the formulas, you’ll need to refer to the documentation of the LDR. In Wokwi, when clicking
// on a component there is a question mark icon that appears and you can click on it to take you to the
// documentation. Check out the LDR documentation and replace the temperature sensing example
// NTC module with an LDR module. After that refactor the code to measure the level of illumination
// in lux. Additionally, print out the light condition.

// Based on the previous example, design a simple light sensor application. This means that as the
// illumination level drops turn on one or more LEDs. For example, if a twilight condition is detected,
// then turn on one LED, if deep twilight is detected, turn on two LEDs, finally, if full moon is detected
// then turn on three LEDs.

// Hint: Check Wokwi Datasheet for converstion formulas https://docs.wokwi.com/parts/wokwi-photoresistor-sensor

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello world!");
}
