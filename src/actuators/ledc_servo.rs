pub struct ledc_channel{
    
}

impl ServoActuactor for ledc_channel {
    fn set_position(&mut self, norm:f32) -> Result<(), Self::Error> {

    }

    fn set_speed(&mut self, speed:f16) -> Result<(), Self::Error> {
        
    }

    fn set_accel(&mut self, accel:f16) -> Result<(), Self::Error> {
        
    }

    fn relax(&mut self) -> Result<(), Self::Error> {
    }
}