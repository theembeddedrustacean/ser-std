/*
Simplified Embedded Rust: ESP Standard Library Edition
Programming PWM - LED Fading Application Example
*/

use esp_idf_svc::hal::delay::Ets;
use esp_idf_svc::hal::ledc::{config::TimerConfig, LedcDriver, LedcTimerDriver, Resolution};
use esp_idf_svc::hal::prelude::*;

fn main() {
    esp_idf_svc::sys::link_patches();

    // Take Peripherals
    let peripherals = Peripherals::take().unwrap();

    // Configure and Initialize LEDC Timer Driver
    let timer_driver = LedcTimerDriver::new(
        peripherals.ledc.timer0,
        &TimerConfig::default()
            .frequency(1000.Hz())
            .resolution(Resolution::Bits14),
    )
    .unwrap();

    // Configure and Initialize LEDC Driver
    let mut pwm_driver = LedcDriver::new(
        peripherals.ledc.channel0,
        timer_driver,
        peripherals.pins.gpio7,
    )
    .unwrap();

    // Get the PWM Max Duty Cycle
    let max_duty = pwm_driver.get_max_duty();
    // Set the PWM Max Duty Cycle
    let min_duty = 0;

    // Initalize starting Duty Cycle
    pwm_driver.set_duty(0).unwrap();

    // Enable PWM
    pwm_driver.enable().unwrap();

    loop {
        // Sweep from 0% Duty to Maximum Duty (100%)
        for duty in min_duty..max_duty {
            // Set Duty
            pwm_driver.set_duty(duty).unwrap();
            // Delay to create fading effect
            Ets::delay_us(10);
        }

        // Sweep from Maximum Duty (100%) to 0% Duty
        for duty in (min_duty..max_duty).rev() {
            // Set Duty
            pwm_driver.set_duty(duty).unwrap();
            // Delay to create fading effect
            Ets::delay_us(10);
        }
    }
}
