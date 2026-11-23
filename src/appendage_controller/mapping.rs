use std::sync::{Arc,Mutex};
use esp_idf_hal::Sys::EspError;

use crate::actuators::{LedcServo, MaestroBus, MaestroChannel, ServoActuator};

#[derive(Clone, Copy)]
pub struct JointCal {
    pub channel: u8,
    pub min_us: u16,
    pub max_us: u16,
    pub inverted: bool,
    pub home: f32,
}

pub struct WallEMap {
    pub head_rotation: JointCal,
    pub neck_top: JointCal,
    pub neck_buttom: JointCal,
    pub eye_left: JointCal,
    pub eye_right: JointCal,
    pub arm_left: JointCal
    pub arm_right_cal: (u16,u16, bool, f32),
}

impl Default for WallEMap {
    
}