use esp_idf_hal::gpio::OutputPin;
use esp_idf_hal::ledc::{
    config::TimerConfig, LedcChannel, LedcDriver, LedcTimer, LedcTimerDriver, Resolution,
};
use esp_idf_hal::sys::EspError;
use esp_idf_hal::units::Hertz;

use super::ServoActuator;

const SERVO_FREQ_HZ: u32 = 50;
const PERIOD_US: u32 = 1_000_000 / SERVO_FREQ_HZ; // 20,000 us

pub struct LedcServo<'d> {
    driver: LedcDriver<'d>,
    min_us: u16,
    max_us: u16,
    inverted: bool,
}

impl<'d> LedcServo<'d> {

    pub fn new<T, C, P>(
        timer: T,
        channel: C,
        pin: P,
        min_us: u16,
        max_us: u16,
        inverted: bool,
    ) -> Result<Self, EspError>
    where
        T: LedcTimer + 'd,
        C: LedcChannel<SpeedMode = T::SpeedMode> + 'd,
        P: OutputPin + 'd,
    {
        let timer_config = TimerConfig::new()
            .frequency(Hertz(SERVO_FREQ_HZ))
            .resolution(Resolution::Bits14);

        let timer_driver = LedcTimerDriver::new(timer, &timer_config)?;
        let driver = LedcDriver::new(channel, timer_driver, pin)?;

        Ok(Self {
            driver,
            min_us,
            max_us,
            inverted,
        })
    }

    pub fn set_target_us(&mut self, pulse_us: u16) -> Result<(), EspError> {
        let clamped_us = pulse_us.clamp(self.min_us, self.max_us) as u32;
        let max_duty = self.driver.get_max_duty();
        let duty = (clamped_us * max_duty) / PERIOD_US;
        self.driver.set_duty(duty)
    }
}

impl<'d> ServoActuator for LedcServo<'d> {
    type Error = EspError;
    
    fn set_position(&mut self, position: f32) -> Result<(), Self::Error> {
        let clamped = position.clamp(0.0, 1.0);
        let normalized = if self.inverted { 1.0 - clamped } else { clamped };
        let span = (self.max_us - self.min_us) as f32;
        let target_us = self.min_us + (normalized * span).round() as u16;
        self.set_target_us(target_us)
    }

    fn set_speed(&mut self, _speed: u16) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_accel(&mut self, _accel: u16) -> Result<(), Self::Error> {
        Ok(())
    }

    fn limp(&mut self) -> Result<(), Self::Error> {
        self.driver.set_duty(0)
    }
}