/*
Simplified Embedded Rust: ESP Standard Library Edition
Programming Serial Communication - Interacting with an I2C Real-Time Clock Application Example
*/

use esp_idf_svc::hal::delay::{FreeRtos, BLOCK};
use esp_idf_svc::hal::i2c::*;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::prelude::*;

use nobcd::BcdNumber;

const DS1307_ADDR: u8 = 0x68;

fn main() {
    esp_idf_svc::sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio3;
    let scl = peripherals.pins.gpio2;

    let config = I2cConfig::new().baudrate(100.kHz().into());
    let mut ds1307 = I2cDriver::new(i2c, sda, scl, &config).unwrap();

    #[repr(u8)]
    enum DS1307 {
        Seconds,
        Minutes,
        Hours,
        Day,
        Date,
        Month,
        Year,
    }

    enum DAY {
        Sun = 1,
        Mon = 2,
        Tues = 3,
        Wed = 4,
        Thurs = 5,
        Fri = 6,
    }

    struct DateTime {
        sec: u8,
        min: u8,
        hrs: u8,
        day: u8,
        date: u8,
        mnth: u8,
        yr: u8,
    }

    let start_dt = DateTime {
        sec: 0,
        min: 0,
        hrs: 0,
        day: DAY::Fri as u8,
        date: 15,
        mnth: 5,
        yr: 15,
    };

    // Set Time
    // Set Seconds -> Also Activates Oscillator
    let secs: [u8; 1] = BcdNumber::new(start_dt.sec).unwrap().bcd_bytes();
    ds1307
        .write(DS1307_ADDR, &[DS1307::Seconds as u8, secs[0]], BLOCK)
        .unwrap();
    // Set Minutes
    let mins: [u8; 1] = BcdNumber::new(start_dt.min).unwrap().bcd_bytes();
    ds1307
        .write(DS1307_ADDR, &[DS1307::Minutes as u8, mins[0]], BLOCK)
        .unwrap();
    // Set Hours
    let hrs: [u8; 1] = BcdNumber::new(start_dt.hrs).unwrap().bcd_bytes();
    ds1307
        .write(DS1307_ADDR, &[DS1307::Hours as u8, hrs[0]], BLOCK)
        .unwrap();
    // Set Day of Week
    let dow: [u8; 1] = BcdNumber::new(start_dt.day).unwrap().bcd_bytes();
    ds1307
        .write(DS1307_ADDR, &[DS1307::Day as u8, dow[0]], BLOCK)
        .unwrap();
    // Set Day of Month
    let dom: [u8; 1] = BcdNumber::new(start_dt.date).unwrap().bcd_bytes();
    ds1307
        .write(DS1307_ADDR, &[DS1307::Date as u8, dom[0]], BLOCK)
        .unwrap();
    // Set Month
    let mnth: [u8; 1] = BcdNumber::new(start_dt.mnth).unwrap().bcd_bytes();
    ds1307
        .write(DS1307_ADDR, &[DS1307::Month as u8, mnth[0]], BLOCK)
        .unwrap();
    // Set Year
    let yr: [u8; 1] = BcdNumber::new(start_dt.yr).unwrap().bcd_bytes();
    ds1307
        .write(DS1307_ADDR, &[DS1307::Year as u8, yr[0]], BLOCK)
        .unwrap();

    loop {
        // Initialize Array that will buffer data read from the DS1307
        let mut data: [u8; 7] = [0_u8; 7];

        // Provide Starting Address (zero) to Read Data from DS1307
        ds1307.write(DS1307_ADDR, &[0_u8], BLOCK).unwrap();
        ds1307.read(DS1307_ADDR, &mut data, BLOCK).unwrap();

        println!("{:?}", data);

        let secs = BcdNumber::from_bcd_bytes([data[0] & 0x7f])
            .unwrap()
            .value::<u8>();
        let mins = BcdNumber::from_bcd_bytes([data[1]]).unwrap().value::<u8>();
        let hrs = BcdNumber::from_bcd_bytes([data[2] & 0x3f])
            .unwrap()
            .value::<u8>();
        let dom = BcdNumber::from_bcd_bytes([data[4]]).unwrap().value::<u8>();
        let mnth = BcdNumber::from_bcd_bytes([data[5]]).unwrap().value::<u8>();
        let yr = BcdNumber::from_bcd_bytes([data[6]]).unwrap().value::<u8>();
        let dow = match BcdNumber::from_bcd_bytes([data[3]]).unwrap().value::<u8>() {
            1 => "Sunday",
            2 => "Monday",
            3 => "Tuesday",
            4 => "Wednesday",
            5 => "Thursday",
            6 => "Friday",
            7 => "Saturday",
            _ => "",
        };

        println!(
            "{}, {}/{}/20{}, {:02}:{:02}:{:02}",
            dow, dom, mnth, yr, hrs, mins, secs
        );

        FreeRtos::delay_ms(1000_u32);
    }
}
