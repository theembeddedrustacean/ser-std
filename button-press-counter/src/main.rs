/*
Simplified Embedded Rust: ESP Standard Library Edition
Programming GPIO - Button Press Counter Application Example
*/

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::peripherals::Peripherals;

static FLAG: AtomicBool = AtomicBool::new(false);

fn gpio_int_callback() {
    // Assert FLAG indicating a press button happened
    FLAG.store(true, Ordering::Relaxed);
}

fn main() -> ! {
    esp_idf_svc::sys::link_patches();

    // Take Peripherals
    let dp = Peripherals::take().unwrap();

    // Configure button pin as input
    let mut button = PinDriver::input(dp.pins.gpio0).unwrap();
    // Configure button pin with internal pull up
    button.set_pull(Pull::Up).unwrap();
    // Configure button pin to detect interrupts on a positive edge
    button.set_interrupt_type(InterruptType::PosEdge).unwrap();
    // Attach the ISR to the button interrupt
    unsafe { button.subscribe(gpio_int_callback).unwrap() }
    // Enable interrupts
    button.enable_interrupt().unwrap();

    // Set up a variable that keeps track of press button count
    let mut count = 0_u32;

    loop {
        // Check if global flag is asserted
        if FLAG.load(Ordering::Relaxed) {
            // Reset global flag
            FLAG.store(false, Ordering::Relaxed);
            // Enable button interrupts again
            button.enable_interrupt().unwrap();
            // Update Press count and print
            count = count.wrapping_add(1);
            println!("Press Count {}", count);
        }
    }
}
