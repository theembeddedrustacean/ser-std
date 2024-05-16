/*
Simplified Embedded Rust: ESP Standard Library Edition
Programming ADCs - Simple Voltmeter Application Example
*/

use esp_idf_svc::hal::adc::attenuation::DB_11;
use esp_idf_svc::hal::adc::config::Resolution;
use esp_idf_svc::hal::adc::oneshot::config::AdcChannelConfig;
use esp_idf_svc::hal::adc::oneshot::*;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::peripherals::Peripherals;

fn main() -> ! {
    esp_idf_svc::sys::link_patches();

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
    let mut adc_chan = AdcChannelDriver::new(&adc1, peripherals.pins.gpio0, &ch_config).unwrap();

    loop {
        // Get ADC Reading
        let sample: u16 = adc_chan.read().unwrap();
        let raw_sample: u16 = adc_chan.read_raw().unwrap();

        // Print the temperature output
        println!("Raw Reading: {}, Voltage Reading: {}mV", raw_sample, sample);

        // Wait half a second before next sample
        FreeRtos::delay_ms(500);
    }
}
