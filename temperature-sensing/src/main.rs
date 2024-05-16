/*
Simplified Embedded Rust: ESP Standard Library Edition
Programming ADCs - Temperature Sensing Application Example
*/

use esp_idf_svc::hal::adc::attenuation::DB_11;
use esp_idf_svc::hal::adc::config::Resolution;
use esp_idf_svc::hal::adc::oneshot::config::AdcChannelConfig;
use esp_idf_svc::hal::adc::oneshot::*;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::peripherals::Peripherals;
use libm::log;

fn main() -> ! {
    esp_idf_svc::sys::link_patches();

    // Take the Peripherals
    let peripherals = Peripherals::take().unwrap();

    // Instantiate ADC Driver
    let adc1 = AdcDriver::new(peripherals.adc1).unwrap();

    // Configure ADC Channel
    let ch_config = AdcChannelConfig {
        attenuation: DB_11,
        calibration: true,
        resolution: Resolution::Resolution12Bit,
    };

    // Instantiate ADC Channel
    let mut adc_chan = AdcChannelDriver::new(&adc1, peripherals.pins.gpio4, &ch_config).unwrap();

    const B: f64 = 3950.0; // B value of the thermistor
    const VMAX: f64 = 4095.0; // Full Range Voltage

    // Algorithm
    // 1) Get adc reading
    // 2) Convert to temperature
    // 3) Send over Serial
    // 4) Go Back to step 1

    loop {
        // Get ADC Reading
        let sample: u16 = adc_chan.read_raw().unwrap();

        //Convert to temperature
        let temperature = 1. / (log(1. / (VMAX / sample as f64 - 1.)) / B + 1.0 / 298.15) - 273.15;

        // Print the temperature output
        println!("Temperature {:02} Celcius\r", temperature);

        // Wait half a second before next sample
        FreeRtos::delay_ms(500);
    }
}
