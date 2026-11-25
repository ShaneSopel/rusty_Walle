mod actuators;
mod appendage_controller;
mod diag;
mod display;

use esp_idf_hal::uart::config::Config;
//use crate::uart::UartNode;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::{AnyIOPin, PinDriver};
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::uart::UartDriver;
use esp_idf_hal::spi::{config::Config as SpiConfig, SpiDeviceDriver, SpiDriver, SpiDriverConfig};
use esp_idf_svc::hal::units::Hertz;

use std::sync::{Arc, Mutex};

use diag::{spawn_diagnostics_task, TelemetryState};
use crate::appendage_controller::WallEMap;
use display::St7735Display;
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

    let map = WallEMap::default();
    let (arm_r_min, arm_r_max, arm_r_inv, _arm_r_home) = map.arm_right_cal;
    let servo7 = LedcServo::new(
        peripherals.ledc.timer0,
        peripherals.ledc.channel0,
        peripherals.pins.gpio25,
        500,
        2500,
        false,
    )?;

    let spi_driver = SpiDriver::new(
    peripherals.spi2,
    peripherals.pins.gpio14, // SCLK
    peripherals.pins.gpio12, // MOSI
    None::<AnyIOPin>,        // MISO unused for display
    &SpiDriverConfig::new(),
    )?;

    let spi_config = SpiConfig::new().baudrate(Hertz(26_000_000));
    let spi_device = SpiDeviceDriver::new(
    spi_driver,
    Some(peripherals.pins.gpio15), // CS
    &spi_config,
)   ?;

    let dc = PinDriver::output(peripherals.pins.gpio26)?;
    let rst = PinDriver::output(peripherals.pins.gpio17)?;

    let st7735 = St7735Display::new(spi_device, dc, rst, 160, 128, true);
    let telemetry = Arc::new(Mutex::new(TelemetryState::default()));

    spawn_diagnostics_task(Box::new(st7735), Arc::clone(&telemetry), 100);
    println!("Walter Project: UART Node Initialized. Starting Tasks...");

    let bus = Arc::new(Mutex::new(MaestroBus::new(uart)));
    FreeRtos::delay_ms(50);
    let baud_ok = bus.lock().unwrap().init_auto_baud().is_ok();
    telemetry.lock().unwrap().maestro_ok = baud_ok;
    FreeRtos::delay_ms(10);

    let mut walle = map.build_body(&bus, servo7, Arc::clone(&telemetry));

    log::info!("WALL-E initialized. Starting diagnostic sweep...");

    loop {
        // Home all joints
        walle.home_all()?;
        {
            let mut t = telemetry.lock().unwrap();
            t.active_pose = "HOME ALL";
            t.head_yaw = 0.5;
            t.neck_top = 0.5;
            t.neck_bot = 0.5;
            t.eye_l = 0.5;
            t.eye_r = 0.5;
            t.arm_l = 0.5;
            t.arm_r = 0.5;
        }
        FreeRtos::delay_ms(1200);

        for pos in [0.2_f32, 0.8_f32, 0.5_f32] {
            walle.head_rotation.set_position(pos)?;
            {
                let mut t = telemetry.lock().unwrap();
                t.active_pose = "HEAD SCAN";
                t.head_yaw = pos;
            }
            FreeRtos::delay_ms(900);
        }

        for pos in [0.1_f32, 0.9_f32, 0.5_f32] {
            walle.arm_right.set_position(pos)?;
            {
                let mut t = telemetry.lock().unwrap();
                t.active_pose = "ARM R TEST";
                t.arm_r = pos;
            }
            FreeRtos::delay_ms(900);
        }

        println!("Runnin on dunkin");
    }

}
