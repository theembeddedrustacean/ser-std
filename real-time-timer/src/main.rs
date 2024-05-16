/*
Simplified Embedded Rust: ESP Standard Library Edition
Programming Timers and Counters - Real-Time Timer Example
*/

use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::timer::config::Config;
use esp_idf_svc::hal::timer::TimerDriver;
use std::ops::Add;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

static FLAG: AtomicBool = AtomicBool::new(false);

struct Time {
    seconds: u32,
    minutes: u32,
    hours: u32,
}

fn timer_int_callback() {
    // Assert FLAG indicating a 1Hz interrupt Occured
    FLAG.store(true, Ordering::Relaxed);
}

fn main() {
    esp_idf_svc::sys::link_patches();

    // Take the peripherals
    let peripherals = Peripherals::take().unwrap();

    // Configure and Initialize Timer Driver
    let config = Config::new().auto_reload(true);
    let mut timer1 = TimerDriver::new(peripherals.timer00, &config).unwrap();

    // Set timer alarm for 1 second
    timer1.set_alarm(timer1.tick_hz()).unwrap();

    // Subscribe to timer interrupt callback
    unsafe { timer1.subscribe(timer_int_callback).unwrap() }

    // Enable Timer interrupt
    timer1.enable_interrupt().unwrap();

    // Enable Timer Alarm
    timer1.enable_alarm(true).unwrap();

    // Enable Counter
    timer1.enable(true).unwrap();

    // Set up a Time struct to keep track of time
    let mut time = Time {
        seconds: 0_u32,
        minutes: 0_u32,
        hours: 0_u32,
    };

    loop {
        // Check if flag asserted by interrupt
        if FLAG.load(Ordering::Relaxed) {
            // Reset global flag
            FLAG.store(false, Ordering::Relaxed);
            // Update Press count and print
            time.seconds = time.seconds.wrapping_add(1);
            if time.seconds > 59 {
                time.minutes = time.minutes.add(1);
            }
            if time.minutes > 59 {
                time.hours = time.hours.add(1);
            }
            if time.hours > 23 {
                time.seconds = 0;
                time.minutes = 0;
                time.hours = 0;
            }
            println!(
                "Elapsed Time {:0>2}:{:0>2}:{:0>2}",
                time.hours, time.minutes, time.seconds
            );
        }
    }
}
