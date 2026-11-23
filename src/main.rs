use esp_idf_hal::uart::config::Config;
//use crate::uart::UartNode;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::AnyIOPin;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::uart::UartDriver;
use esp_idf_svc::hal::units::Hertz;

use std::sync::{Arc, Mutex};

mod actuators;
use crate::actuators::{ServoActuator, ledc_servo::LedcServo, pololu_servo::{MaestroBus, MaestroChannel}};

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;

    let config = Config::new().baudrate(Hertz(115_200));
    let uart = UartDriver::new(
        peripherals.uart1,
        peripherals.pins.gpio18,
        peripherals.pins.gpio19,
        Option::<AnyIOPin>::None,
        Option::<AnyIOPin>::None,
        &config,
    )?;

    let mut servo7 = LedcServo::new(
        peripherals.ledc.timer0,
        peripherals.ledc.channel0,
        peripherals.pins.gpio25,
        500,
        2500,
        false,
    )?;

    println!("Walter Project: UART Node Initialized. Starting Tasks...");

    let bus = Arc::new(Mutex::new(MaestroBus::new(uart)));

    FreeRtos::delay_ms(50);
    bus.lock().unwrap().init_auto_baud()?;
    FreeRtos::delay_ms(10);

    let mut channels: [MaestroChannel; 6] = core::array::from_fn(|ch| {
    MaestroChannel::new(Arc::clone(&bus), ch as u8, 500, 2500, false)
    });

    log::info!("Starting Servo Test");

    loop {

        log::info!("Moving servo7 to 0.0");
    servo7.set_position(0.0)?;
    FreeRtos::delay_ms(1200);

    log::info!("Moving servo7 to 0.5");
    servo7.set_position(0.5)?;
    FreeRtos::delay_ms(1200);

    log::info!("Moving servo7 to 1.0");
    servo7.set_position(1.0)?;
    FreeRtos::delay_ms(1200);

        println!("Runnin on dunkin");
        /*for (i, channel) in channels.iter_mut().enumerate() {
            log::info!("testing channel {i}");
             channel.set_speed(40)?;
            channel.set_accel(20)?;
            log::info!("Moving to 0.0 (min)");
            channel.set_position(0.0)?;
            FreeRtos::delay_ms(1200);
            log::info!("Moving to 0.5 (min)");
            channel.set_position(0.5)?;
            FreeRtos::delay_ms(1200);
            log::info!("Moving to 1 (min)");
            channel.set_position(1.0)?;
            FreeRtos::delay_ms(1200);
            channel.limp()?;
        }*/
    }
}
