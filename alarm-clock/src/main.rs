use anyhow::Result;
use bitmask_enum::bitmask;
use esp_idf_svc::hal::{
    delay::BLOCK,
    gpio::*,
    i2c::{I2cConfig, I2cDriver},
    ledc::{
        config::TimerConfig, LedcDriver, LedcTimerDriver,
    },
    peripherals::Peripherals,
    prelude::*,
    timer::{config::Config, TimerDriver},
};
use esp_idf_svc::{
    hal::delay::FreeRtos, log::EspLogger, sys::link_patches,
};
use std::sync::atomic::{AtomicBool, AtomicU32};

// ===================================================================
// Constants & Bit Masks
// ===================================================================
const DEBOUNCE_COUNT: u32 = 8;
const TCA6424_ADDR: u8 = 0x22;
const RTC_ADDR: u8 = 0x68;

const IN_PORT0: u8 = 0x80;
const IN_PORT1: u8 = 0x81;
const OUT_PORT0: u8 = 0x84;
const OUT_PORT1: u8 = 0x85;
const OUT_PORT2: u8 = 0x86;
const CONFIG_PORT0: u8 = 0x8C;

const PORT0_DIR: u8 = 0xFF; // All inputs on Port 0
const PORT1_DIR: u8 = 0xC0; // Bits 0-5 output, 6-7 input
const PORT2_DIR: u8 = 0x00; // All outputs (segments)

// I/O Expander Pin Definitions
#[bitmask(u8)]
pub enum IoExpPort0 {
    AlarmSwitchOn,  // Port 0 Pin 0
    AlarmSwitchOff, // Port 0 Pin 1
    P02,            // Port 0 Pin 2 (Unused)
    P03,            // Port 0 Pin 3 (Unused)
    AlarmButton,    // Port 0 Pin 4
    TimeButton,     // Port 0 Pin 5
    MinuteButton,   // Port 0 Pin 6
    HourButton,     // Port 0 Pin 7
}

#[bitmask(u8)]
pub enum IoExpPort1 {
    Digit1,         // Port 1 Pin 0
    Digit2,         // Port 1 Pin 1
    Digit3,         // Port 1 Pin 2
    Digit4,         // Port 1 Pin 3
    Led,            // Port 1 Pin 4
    PmLed,          // Port 1 Pin 5
    FormatSwitch12, // Port 1 Pin 6
    FormatSwitch24, // Port 1 Pin 7
}

#[bitmask(u8)]
pub enum IoExpPort2 {
    SegA, // Port 2 Pin 0
    SegB, // Port 2 Pin 1
    SegC, // Port 2 Pin 2
    SegD, // Port 2 Pin 3
    SegE, // Port 2 Pin 4
    SegF, // Port 2 Pin 5
    Dp,   // Port 2 Pin 6
    SegG, // Port 2 Pin 7
}

// ===================================================================
// Enums & Structures
// ===================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
enum Button {
    Pressed(u32),
    Released(u32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum AlarmSwitch {
    On(u32),
    Off(u32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum FormatSwitch {
    H12(u32),
    H24(u32),
}

#[derive(Debug, Clone, Copy)]
struct SystemEvents {
    time_but: Button,
    alarm_but: Button,
    hour_but: Button,
    min_but: Button,
    snooze_but: Button,
    alarm_sw: AlarmSwitch,
    fmt_sw: FormatSwitch,
}

impl Default for SystemEvents {
    fn default() -> Self {
        SystemEvents {
            time_but: Button::Released(0),
            alarm_but: Button::Released(0),
            hour_but: Button::Released(0),
            min_but: Button::Released(0),
            snooze_but: Button::Released(0),
            alarm_sw: AlarmSwitch::Off(0),
            fmt_sw: FormatSwitch::H24(0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Unarmed,
    Armed,
    Snoozing,
    Alarming,
    SetAlarm,
    SetTime,
}

#[derive(Debug, Clone, Copy)]
struct Time {
    hours: u8,
    minutes: u8,
    seconds: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum DisplayMode {
    ShowClock,
    ShowAlarm,
}

#[derive(Debug)]
struct TimeKeeper {
    clocktime: Time,
    alarmtime: Time,
    display_mode: DisplayMode,
}

impl TimeKeeper {
    fn new() -> Self {
        Self {
            clocktime: Time {
                hours: 0,
                minutes: 0,
                seconds: 0,
            },
            alarmtime: Time {
                hours: 0,
                minutes: 0,
                seconds: 0,
            },
            display_mode: DisplayMode::ShowClock,
        }
    }

    fn tick(&mut self) {
        self.clocktime.seconds += 1;
        if self.clocktime.seconds >= 60 {
            self.clocktime.seconds = 0;
            self.clocktime.minutes += 1;
            if self.clocktime.minutes >= 60 {
                self.clocktime.minutes = 0;
                self.clocktime.hours += 1;
                if self.clocktime.hours >= 24 {
                    self.clocktime.hours = 0;
                }
            }
        }
    }

    fn alarm_time_match(&self) -> bool {
        self.clocktime.hours == self.alarmtime.hours
            && self.clocktime.minutes
                == self.alarmtime.minutes
    }
    fn increment_time_hours(&mut self) {
        self.clocktime.hours =
            (self.clocktime.hours + 1) % 24;
    }
    fn increment_time_minutes(&mut self) {
        self.clocktime.minutes =
            (self.clocktime.minutes + 1) % 60;
    }
    fn increment_alarm_hours(&mut self) {
        self.alarmtime.hours =
            (self.alarmtime.hours + 1) % 24;
    }
    fn increment_alarm_minutes(&mut self) {
        self.alarmtime.minutes =
            (self.alarmtime.minutes + 1) % 60;
    }
    fn show_clock_time(&mut self) {
        self.display_mode = DisplayMode::ShowClock;
    }
    fn show_alarm_time(&mut self) {
        self.display_mode = DisplayMode::ShowAlarm;
    }
}

// ===================================================================
// Drivers
// ===================================================================

struct SharedBusDriver<'a> {
    i2c: I2cDriver<'a>,
    past_events: SystemEvents,
}

impl<'a> SharedBusDriver<'a> {
    fn new(i2c: I2cDriver<'a>) -> Self {
        Self {
            i2c,
            past_events: SystemEvents::default(),
        }
    }

    fn init_expander(&mut self) -> Result<()> {
        self.i2c.write(
            TCA6424_ADDR,
            &[
                CONFIG_PORT0,
                PORT0_DIR,
                PORT1_DIR,
                PORT2_DIR,
            ],
            BLOCK,
        )?;
        Ok(())
    }

    fn read_rtc_time(&mut self) -> Result<Time> {
        let mut buf = [0u8; 3];
        self.i2c.write_read(
            RTC_ADDR,
            &[0x00],
            &mut buf,
            BLOCK,
        )?;
        Ok(Time {
            seconds: bcd_to_bin(buf[0] & 0x7F),
            minutes: bcd_to_bin(buf[1]),
            hours: bcd_to_bin(buf[2] & 0x3F),
        })
    }

    fn write_rtc_time(&mut self, t: Time) -> Result<()> {
        let data = [
            0x00,
            bin_to_bcd(t.seconds),
            bin_to_bcd(t.minutes),
            bin_to_bcd(t.hours),
        ];
        self.i2c.write(RTC_ADDR, &data, BLOCK)?;
        Ok(())
    }

    fn pm_led_on(&mut self) -> Result<()> {
        let mut buf = [0u8; 1];
        self.i2c.write_read(
            TCA6424_ADDR,
            &[OUT_PORT1],
            &mut buf,
            BLOCK,
        )?;
        let new = buf[0] | IoExpPort1::PmLed.bits();
        self.i2c.write(
            TCA6424_ADDR,
            &[OUT_PORT1, new],
            BLOCK,
        )?;
        Ok(())
    }

    fn pm_led_off(&mut self) -> Result<()> {
        let mut buf = [0u8; 1];
        self.i2c.write_read(
            TCA6424_ADDR,
            &[OUT_PORT1],
            &mut buf,
            BLOCK,
        )?;
        let new = buf[0] | !IoExpPort1::PmLed.bits();
        self.i2c.write(
            TCA6424_ADDR,
            &[OUT_PORT1, new],
            BLOCK,
        )?;
        Ok(())
    }

    fn display_digit(
        &mut self,
        idx: u8,
        value: u8,
    ) -> Result<()> {
        let digit_masks = [
            IoExpPort1::Digit1,
            IoExpPort1::Digit2,
            IoExpPort1::Digit3,
            IoExpPort1::Digit4,
        ];

        // 1. Blank all digits
        let mut port1 = [0u8; 1];
        self.i2c.write_read(
            TCA6424_ADDR,
            &[OUT_PORT1],
            &mut port1,
            BLOCK,
        )?;
        let blank = (port1[0] & 0xF0) | 0x0F;
        self.i2c.write(
            TCA6424_ADDR,
            &[OUT_PORT1, blank],
            BLOCK,
        )?;

        // 2. Write segments
        let seg = digit_to_segments(value)
            | IoExpPort2::Dp.bits();
        self.i2c.write(
            TCA6424_ADDR,
            &[OUT_PORT2, seg],
            BLOCK,
        )?;

        // 3. Activate selected digit
        let active = (port1[0] & 0xF0)
            | (!digit_masks[idx as usize].bits() & 0x0F);
        self.i2c.write(
            TCA6424_ADDR,
            &[OUT_PORT1, active],
            BLOCK,
        )?;
        Ok(())
    }

    fn get_expander_events(&mut self) -> SystemEvents {
        let mut ports = [0u8; 2];

        let mut buf = [0u8];

        for (i, reg) in
            [IN_PORT0, IN_PORT1].iter().enumerate()
        {
            self.i2c
                .write_read(
                    TCA6424_ADDR,
                    &[*reg],
                    &mut buf,
                    BLOCK,
                )
                .unwrap();
            ports[i] = buf[0];
        }

        let events = SystemEvents {
            time_but: if ports[0]
                & IoExpPort0::TimeButton.bits()
                == 0
            {
                match self.past_events.time_but {
                    Button::Pressed(count) => {
                        Button::Pressed(
                            count.wrapping_add(1),
                        )
                    }
                    Button::Released(_) => {
                        Button::Pressed(1)
                    }
                }
            } else {
                match self.past_events.time_but {
                    Button::Pressed(_) => {
                        Button::Released(1)
                    }
                    Button::Released(count) => {
                        Button::Released(
                            count.wrapping_add(1),
                        )
                    }
                }
            },
            alarm_but: if ports[0]
                & IoExpPort0::AlarmButton.bits()
                == 0
            {
                match self.past_events.alarm_but {
                    Button::Pressed(count) => {
                        Button::Pressed(
                            count.wrapping_add(1),
                        )
                    }
                    Button::Released(_) => {
                        Button::Pressed(1)
                    }
                }
            } else {
                match self.past_events.alarm_but {
                    Button::Pressed(_) => {
                        Button::Released(1)
                    }
                    Button::Released(count) => {
                        Button::Released(
                            count.wrapping_add(1),
                        )
                    }
                }
            },
            hour_but: if ports[0]
                & IoExpPort0::HourButton.bits()
                == 0
            {
                match self.past_events.hour_but {
                    Button::Pressed(count) => {
                        Button::Pressed(
                            count.wrapping_add(1),
                        )
                    }
                    Button::Released(_) => {
                        Button::Pressed(1)
                    }
                }
            } else {
                match self.past_events.hour_but {
                    Button::Pressed(_) => {
                        Button::Released(1)
                    }
                    Button::Released(count) => {
                        Button::Released(
                            count.wrapping_add(1),
                        )
                    }
                }
            },
            min_but: if ports[0]
                & IoExpPort0::MinuteButton.bits()
                == 0
            {
                match self.past_events.min_but {
                    Button::Pressed(count) => {
                        Button::Pressed(
                            count.wrapping_add(1),
                        )
                    }
                    Button::Released(_) => {
                        Button::Pressed(1)
                    }
                }
            } else {
                match self.past_events.min_but {
                    Button::Pressed(_) => {
                        Button::Released(1)
                    }
                    Button::Released(count) => {
                        Button::Released(
                            count.wrapping_add(1),
                        )
                    }
                }
            },
            // Snooze button handled in separate GPIO driver
            snooze_but: self.past_events.snooze_but,
            alarm_sw: if ports[0]
                & IoExpPort0::AlarmSwitchOn.bits()
                == 0
            {
                match self.past_events.alarm_sw {
                    AlarmSwitch::On(count) => {
                        AlarmSwitch::On(
                            count.wrapping_add(1),
                        )
                    }
                    AlarmSwitch::Off(_) => {
                        AlarmSwitch::On(1)
                    }
                }
            } else {
                match self.past_events.alarm_sw {
                    AlarmSwitch::On(_) => {
                        AlarmSwitch::Off(1)
                    }
                    AlarmSwitch::Off(count) => {
                        AlarmSwitch::Off(
                            count.wrapping_add(1),
                        )
                    }
                }
            },
            fmt_sw: if ports[1]
                & IoExpPort1::FormatSwitch12.bits()
                == 0
            {
                match self.past_events.fmt_sw {
                    FormatSwitch::H12(count) => {
                        FormatSwitch::H12(
                            count.wrapping_add(1),
                        )
                    }
                    FormatSwitch::H24(_) => {
                        FormatSwitch::H12(1)
                    }
                }
            } else {
                match self.past_events.fmt_sw {
                    FormatSwitch::H12(_) => {
                        FormatSwitch::H24(1)
                    }
                    FormatSwitch::H24(count) => {
                        FormatSwitch::H24(
                            count.wrapping_add(1),
                        )
                    }
                }
            },
        };
        self.past_events = events;
        events
    }
}

// ===================================================================
// Global Flags and Timer Callback
// ===================================================================
static FLAG_5MS: AtomicBool = AtomicBool::new(false);
static FLAG_1S: AtomicBool = AtomicBool::new(false);
static FLAG_5M: AtomicBool = AtomicBool::new(false);
static TICK: AtomicU32 = AtomicU32::new(0);

fn timer_int_callback() {
    let tick = TICK
        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

    // 5ms Flag
    if tick % 1 == 0 {
        FLAG_5MS.store(
            true,
            std::sync::atomic::Ordering::Relaxed,
        );
    }

    // 1s Flag
    if tick % 200 == 0 {
        FLAG_1S.store(
            true,
            std::sync::atomic::Ordering::Relaxed,
        );
    }

    // 5 Minute Flag
    // 60000 ticks of 5ms = 5 minutes
    if tick % 10000 == 0 {
        FLAG_5M.store(
            true,
            std::sync::atomic::Ordering::Relaxed,
        );
    }
}

// ===================================================================
// Main
// ===================================================================

fn main() -> Result<()> {
    link_patches();
    EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    // Alarm LED Driver
    let mut alarm_led =
        PinDriver::output(peripherals.pins.gpio3)?;
    alarm_led.set_low()?;

    // Snooze Button Driver
    let mut snooze_button =
        PinDriver::input(peripherals.pins.gpio5)?;
    snooze_button.set_pull(Pull::Up)?;

    // Buzzer Driver
    let buzzer_timer = LedcTimerDriver::new(
        peripherals.ledc.timer0,
        &TimerConfig::default()
            .resolution(
                esp_idf_svc::hal::ledc::Resolution::Bits14,
            )
            .frequency(2700.Hz()),
    )?;
    let mut buzzer = LedcDriver::new(
        peripherals.ledc.channel0,
        &buzzer_timer,
        peripherals.pins.gpio4,
    )?;
    buzzer.set_duty(0)?;

    // I2C Driver
    let i2c = I2cDriver::new(
        peripherals.i2c0,
        peripherals.pins.gpio6,
        peripherals.pins.gpio7,
        &I2cConfig::new().baudrate(100.kHz().into()),
    )?;

    // SharedBusDriver Instance
    let mut i2c_bus_driver = SharedBusDriver::new(i2c);

    // Initialize I/O Expander
    i2c_bus_driver.init_expander()?;

    // TimeKeeper Instance
    let mut tk = TimeKeeper::new();
    // System Events Instance
    let mut events = SystemEvents::default();

    // Load RTC time at startup
    if let Ok(t) = i2c_bus_driver.read_rtc_time() {
        tk.clocktime = t;
    }

    // Initial State, Digit Index, and Snooze Button State
    let mut state = State::Unarmed;
    let mut digit_idx: u8 = 0;
    let mut past_snooze_button: Button =
        Button::Released(0);

    // Timer Driver for scheduling application operations
    let config = Config::new().auto_reload(true);
    let mut timer1 =
        TimerDriver::new(peripherals.timer00, &config)
            .unwrap();
    // Set timer alarm for 5 ms
    timer1.set_alarm(timer1.tick_hz() / 500).unwrap();

    // Subscribe to timer interrupt callback
    unsafe { timer1.subscribe(timer_int_callback).unwrap() }

    // Enable Timer interrupt
    timer1.enable_interrupt().unwrap();

    // Enable Timer Alarm
    timer1.enable_alarm(true).unwrap();

    // Enable Counting
    timer1.enable(true).unwrap();
    loop {
        // 1 Second Tasks
        if FLAG_1S
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            FLAG_1S.store(
                false,
                std::sync::atomic::Ordering::Relaxed,
            );

            // Timekeeping
            tk.tick();
            i2c_bus_driver.write_rtc_time(tk.clocktime)?;
            // log::info!("Time: {:?}", tk.clocktime);
        }

        // 5 ms Tasks
        if FLAG_5MS
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            FLAG_5MS.store(
                false,
                std::sync::atomic::Ordering::Relaxed,
            );

            // Input Polling
            events = i2c_bus_driver.get_expander_events();
            events.snooze_but = if snooze_button.is_low() {
                // Button physically pressed
                past_snooze_button =
                    match past_snooze_button {
                        Button::Pressed(c) => {
                            Button::Pressed(
                                c.wrapping_add(1),
                            )
                        }
                        Button::Released(_) => {
                            Button::Pressed(1)
                        }
                    };
                past_snooze_button
            } else {
                // Button physically released
                past_snooze_button =
                    match past_snooze_button {
                        Button::Pressed(_) => {
                            Button::Released(1)
                        }
                        Button::Released(c) => {
                            Button::Released(
                                c.wrapping_add(1),
                            )
                        }
                    };
                past_snooze_button
            };

            let (h, m) = match tk.display_mode {
                DisplayMode::ShowClock => (
                    tk.clocktime.hours,
                    tk.clocktime.minutes,
                ),
                DisplayMode::ShowAlarm => (
                    tk.alarmtime.hours,
                    tk.alarmtime.minutes,
                ),
            };

            // PM Indicator & 12/24 Hour Format Handling
            let mut dh = h;
            match events.fmt_sw {
                FormatSwitch::H12(count) => {
                    if count >= DEBOUNCE_COUNT {
                        if h == 0 {
                            dh = 12;
                            i2c_bus_driver.pm_led_off()?;
                        } else if h == 12 {
                            dh = 12;
                            i2c_bus_driver.pm_led_on()?;
                        } else if h > 12 {
                            dh = h - 12;
                            i2c_bus_driver.pm_led_on()?;
                        } else {
                            i2c_bus_driver.pm_led_off()?;
                        }
                    } else {
                        i2c_bus_driver.pm_led_off()?;
                    }
                }
                _ => {
                    i2c_bus_driver.pm_led_off()?;
                }
            }

            // Display Multiplexing
            let digits = [dh / 10, dh % 10, m / 10, m % 10];
            i2c_bus_driver.display_digit(
                digit_idx,
                digits[digit_idx as usize],
            )?;
            digit_idx = (digit_idx + 1) % 4;
        }

        // State Machine Evaluation
        state = match state {
            State::Unarmed => {
                log::info!("{:?}", state);
                alarm_led.set_low()?;
                tk.show_clock_time();
                buzzer.set_duty(0).unwrap();
                match events.alarm_sw {
                    AlarmSwitch::On(count) => {
                        if count == DEBOUNCE_COUNT {
                            State::Armed
                        } else {
                            State::Unarmed
                        }
                    }
                    _ => match events.alarm_but {
                        Button::Pressed(count) => {
                            if count == DEBOUNCE_COUNT {
                                State::SetAlarm
                            } else {
                                State::Unarmed
                            }
                        }
                        _ => match events.time_but {
                            Button::Pressed(count) => {
                                if count == DEBOUNCE_COUNT {
                                    State::SetTime
                                } else {
                                    State::Unarmed
                                }
                            }
                            _ => State::Unarmed,
                        },
                    },
                }
            }
            State::Armed => {
                log::info!("{:?}", state);
                alarm_led.set_high()?;
                match events.alarm_sw {
                    AlarmSwitch::Off(count) => {
                        if count == DEBOUNCE_COUNT {
                            State::Unarmed
                        } else {
                            State::Armed
                        }
                    }
                    _ => match events.alarm_but {
                        Button::Pressed(count) => {
                            if count == DEBOUNCE_COUNT {
                                State::SetAlarm
                            } else {
                                State::Armed
                            }
                        }
                        _ => match events.time_but {
                            Button::Pressed(count) => {
                                if count == DEBOUNCE_COUNT {
                                    State::SetTime
                                } else {
                                    State::Armed
                                }
                            }
                            _ => {
                                if tk.alarm_time_match() {
                                    State::Alarming
                                } else {
                                    State::Armed
                                }
                            }
                        },
                    },
                }
            }
            State::SetTime => {
                log::info!("{:?}", state);
                tk.show_clock_time();
                if let Button::Pressed(count) =
                    events.hour_but
                {
                    if count == DEBOUNCE_COUNT {
                        tk.increment_time_hours();
                    }
                }

                if let Button::Pressed(count) =
                    events.min_but
                {
                    if count == DEBOUNCE_COUNT {
                        tk.increment_time_minutes();
                    }
                }

                if let Button::Released(count) =
                    events.time_but
                {
                    if count == DEBOUNCE_COUNT {
                        State::Unarmed
                    } else {
                        State::SetTime
                    }
                } else {
                    State::SetTime
                }
            }
            State::SetAlarm => {
                log::info!("{:?}", state);
                tk.show_alarm_time();

                // Adjust alarm hours/minutes on short press
                if let Button::Pressed(count) =
                    events.hour_but
                {
                    if count == DEBOUNCE_COUNT {
                        tk.increment_alarm_hours();
                    }
                }
                if let Button::Pressed(count) =
                    events.min_but
                {
                    if count == DEBOUNCE_COUNT {
                        tk.increment_alarm_minutes();
                    }
                }

                if let Button::Released(count) =
                    events.alarm_but
                {
                    if count == DEBOUNCE_COUNT {
                        State::Unarmed
                    } else {
                        State::SetAlarm
                    }
                } else {
                    State::SetAlarm
                }
            }
            State::Alarming => {
                log::info!("{:?}", state);
                buzzer.set_duty(50).unwrap();
                match events.alarm_sw {
                    AlarmSwitch::Off(count) => {
                        if count == DEBOUNCE_COUNT {
                            State::Unarmed
                        } else {
                            State::Alarming
                        }
                    }
                    _ => match events.snooze_but {
                        Button::Pressed(count) => {
                            if count == DEBOUNCE_COUNT {
                                // Reset Snooze Flag
                                FLAG_5M.store(
                                    false,
                                    std::sync::atomic::Ordering::Relaxed,
                                );
                                // Transition to Snooze State
                                State::Snoozing
                            } else {
                                // Or continue Alarming
                                State::Alarming
                            }
                        }
                        // No action, continue alarming
                        _ => State::Alarming,
                    },
                }
            }
            State::Snoozing => {
                log::info!("{:?}", state);
                buzzer.set_duty(0)?;
                // Check if 5 minute snooze period has elapsed
                if FLAG_5M.load(
                    std::sync::atomic::Ordering::Relaxed,
                ) {
                    State::Alarming
                } else {
                    // Check if Alarm Switch turned off
                    match events.alarm_sw {
                        AlarmSwitch::Off(count) => {
                            if count == DEBOUNCE_COUNT {
                                State::Unarmed
                            } else {
                                State::Snoozing
                            }
                        }
                        _ => State::Snoozing,
                    }
                }
            }
        };

        FreeRtos::delay_ms(2);
    }
}

// Helper functions
fn bcd_to_bin(v: u8) -> u8 {
    ((v >> 4) * 10) + (v & 0x0F)
}
fn bin_to_bcd(v: u8) -> u8 {
    ((v / 10) << 4) | (v % 10)
}
fn digit_to_segments(d: u8) -> u8 {
    match d {
        0 => {
            IoExpPort2::SegA.bits()
                | IoExpPort2::SegB.bits()
                | IoExpPort2::SegC.bits()
                | IoExpPort2::SegD.bits()
                | IoExpPort2::SegE.bits()
                | IoExpPort2::SegF.bits()
        }
        1 => {
            IoExpPort2::SegB.bits()
                | IoExpPort2::SegC.bits()
        }
        2 => {
            IoExpPort2::SegA.bits()
                | IoExpPort2::SegB.bits()
                | IoExpPort2::SegD.bits()
                | IoExpPort2::SegE.bits()
                | IoExpPort2::SegG.bits()
        }
        3 => {
            IoExpPort2::SegA.bits()
                | IoExpPort2::SegB.bits()
                | IoExpPort2::SegC.bits()
                | IoExpPort2::SegD.bits()
                | IoExpPort2::SegG.bits()
        }
        4 => {
            IoExpPort2::SegB.bits()
                | IoExpPort2::SegC.bits()
                | IoExpPort2::SegF.bits()
                | IoExpPort2::SegG.bits()
        }
        5 => {
            IoExpPort2::SegA.bits()
                | IoExpPort2::SegC.bits()
                | IoExpPort2::SegD.bits()
                | IoExpPort2::SegF.bits()
                | IoExpPort2::SegG.bits()
        }
        6 => {
            IoExpPort2::SegA.bits()
                | IoExpPort2::SegC.bits()
                | IoExpPort2::SegD.bits()
                | IoExpPort2::SegE.bits()
                | IoExpPort2::SegF.bits()
                | IoExpPort2::SegG.bits()
        }
        7 => {
            IoExpPort2::SegA.bits()
                | IoExpPort2::SegB.bits()
                | IoExpPort2::SegC.bits()
        }
        8 => {
            IoExpPort2::SegA.bits()
                | IoExpPort2::SegB.bits()
                | IoExpPort2::SegC.bits()
                | IoExpPort2::SegD.bits()
                | IoExpPort2::SegE.bits()
                | IoExpPort2::SegF.bits()
                | IoExpPort2::SegG.bits()
        }
        9 => {
            IoExpPort2::SegA.bits()
                | IoExpPort2::SegB.bits()
                | IoExpPort2::SegC.bits()
                | IoExpPort2::SegD.bits()
                | IoExpPort2::SegF.bits()
                | IoExpPort2::SegG.bits()
        }
        _ => 0,
    }
}
