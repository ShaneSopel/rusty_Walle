
pub mod pololuChannel;
pub mod ledc_channel;

pub trait ServoActuactor {
    type Error;

    // Move Servo to a position in bounds of [0.0 - 1.0]
    fn set_position(&mut self, norm: f32) -> Result<(), Self::Error>;

    // optional motion profiling 
    fn set_speed(&mut self, _speed: u16) -> Result<(), Self:Error>;

    fn set_accel(&mut self,_accel: u16) -> Result<(), Self:Error>;

    fn limp(&mut self) -> Result<(), Self:Error>;
}

pub trait DcMotor {
    type Error;

}