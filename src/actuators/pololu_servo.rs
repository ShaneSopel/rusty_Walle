
pub struct pololuChannel {
    // share the same uart bus with all 6 channels with freertos
    bus: Arc<Mutex<MaestroBus>>,
    // the actual channel
    channel: u8,
    // pulse width min and ax
    min_us: u16,
    max_us: u16,
    inverted: bool,
}

impl ServoActuactor for pololuChannel {
    type Error = EspError;

    fn set_position(&mut self, norm: f32) -> Result<(), Self:Error> {
        // norm can be within the range of 0.0 - 1.0, 
        //if outside of that range its max can be 1.0 or the lowest can be 0.0
        let clamped = norm.clamp(0.0, 1.0);
        //map incase inverted
        let mapped = if self.inverted {1.0 - clamped} else {clamped};

        let us = self.min_us as f32 + mapped*(self.max_us-self.min_us) as f32;

        let mut bus = self.bus.lock().unwrap();
        bus.set_target_us(self.channel, us.round() as u16)
    }

    fn set_speed(&mut self, speed: u16) -> Result<(), Self:Error> {
        self.bus.lock().unwrap.set_speed(self.channel, speed)
    }

    fn set_accel(&mut self, accel: u16) -> Result<(), Self:Error>{
        self.bus.lock().unwrap().set_accel(self.channel, accel)
    }

    fn relax(&mut self) -> Result<(), Self:Error> {
        self.bus.lock().unwrap().set_target_us(self.channel, 0)
    }

}
