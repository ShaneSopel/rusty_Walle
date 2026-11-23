use esp_idf_hal::io::Write;
use esp_idf_hal::uart::UartDriver;
use esp_idf_sys::EspError;
use std::sync::{Arc, Mutex};

use crate::actuators::ServoActuator;

pub struct MaestroChannel {
    // share the same uart bus with all 6 channels with freertos
    bus: Arc<Mutex<MaestroBus>>,
    // the actual channel
    channel: u8,
    // pulse width min and ax
    min_us: u16,
    max_us: u16,
    inverted: bool,
}

impl MaestroChannel {
    pub fn new(
        bus: Arc<Mutex<MaestroBus>>,
        channel: u8,
        min_us: u16,
        max_us: u16,
        inverted: bool,
    ) -> Self {
        Self {
            bus,
            channel,
            min_us,
            max_us,
            inverted,
        }
    }
}

impl ServoActuator for MaestroChannel {
    type Error = EspError;

    fn set_position(&mut self, norm: f32) -> Result<(), Self::Error> {
        // norm can be within the range of 0.0 - 1.0,
        //if outside of that range its max can be 1.0 or the lowest can be 0.0
        let clamped = norm.clamp(0.0, 1.0);
        //map incase inverted
        let mapped = if self.inverted {
            1.0 - clamped
        } else {
            clamped
        };

        let us = self.min_us as f32 + mapped * (self.max_us - self.min_us) as f32;

        let mut bus = self.bus.lock().unwrap();
        bus.set_target_us(self.channel, us.round() as u16)
    }

    fn set_speed(&mut self, speed: u16) -> Result<(), Self::Error> {
        self.bus.lock().unwrap().set_speed(self.channel, speed)
    }

    fn set_accel(&mut self, accel: u16) -> Result<(), Self::Error> {
        self.bus
            .lock()
            .unwrap()
            .set_acceleration(self.channel, accel)
    }

    fn limp(&mut self) -> Result<(), Self::Error> {
        self.bus.lock().unwrap().set_target_us(self.channel, 0)
    }
}

pub struct MaestroBus {
    uart: UartDriver<'static>,
}

impl MaestroBus {
    pub fn new(uart: UartDriver<'static>) -> Self {
        Self { uart }
    }

    pub fn init_auto_baud(&mut self) -> Result<(), EspError> {
       let _ = self.uart.write_all(&[0xAA]);
        Ok(())
    }

    /// Command 0x84: Set Target in microseconds (0 = stop pulses / relax)
    pub fn set_target_us(&mut self, channel: u8, us: u16) -> Result<(), EspError> {
        // Maestro expects quarter-microseconds (e.g., 1500 us -> 6000)
        let quarter_us = us.saturating_mul(4);
        let packet = [
            0x84,
            channel,
            (quarter_us & 0x7F) as u8,        // Lower 7 bits
            ((quarter_us >> 7) & 0x7F) as u8, // Upper 7 bits
        ];
        let _ = self.uart.write_all(&packet);
        Ok(())
    }

    /// Command 0x87: Set Speed (units of 0.25 us / 10 ms; 0 = unlimited)
    pub fn set_speed(&mut self, channel: u8, speed: u16) -> Result<(), EspError> {
        let packet = [
            0x87,
            channel,
            (speed & 0x7F) as u8,
            ((speed >> 7) & 0x7F) as u8,
        ];
        let _ = self.uart.write_all(&packet);
        Ok(())
    }

    /// Command 0x89: Set Acceleration (0..=255; 0 = unlimited)
    pub fn set_acceleration(&mut self, channel: u8, accel: u16) -> Result<(), EspError> {
        let packet = [
            0x89,
            channel,
            (accel & 0x7F) as u8,
            ((accel >> 7) & 0x7F) as u8,
        ];
        let _ =self.uart.write_all(&packet);
        Ok(())
    }
}
