use std::sync::{Arc,Mutex};
use esp_idf_sys::EspError;

use crate::actuators::{ServoActuator, ledc_servo::LedcServo, pololu_servo::{MaestroBus, MaestroChannel}};
use crate::diag::TelemetryState;
use super::WallEBody;


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
    pub neck_bot: JointCal,
    pub eye_left: JointCal,
    pub eye_right: JointCal,
    pub arm_left: JointCal,
    pub arm_right_cal: (u16,u16, bool, f32),
}

impl Default for WallEMap {
    fn default() -> Self {
        Self {
            head_rotation: JointCal {channel: 0, min_us: 600, max_us: 2400, inverted: false, home: 0.5},
            neck_top: JointCal {channel: 1, min_us: 600, max_us: 2400, inverted: false, home: 0.5},
            neck_bot: JointCal {channel: 2, min_us: 600, max_us: 2400, inverted: false, home: 0.5},
            eye_left: JointCal {channel: 3, min_us: 600, max_us: 2400, inverted: false, home: 0.5},
            eye_right: JointCal {channel: 4, min_us: 600, max_us: 2400, inverted: false, home: 0.5},
            arm_left: JointCal {channel: 5, min_us: 600, max_us: 2400, inverted: false, home: 0.5},
            arm_right_cal: (600, 2400, true, 0.5),
        }
    }
}

impl WallEMap {
pub fn build_body(
        &self,
        bus: &Arc<Mutex<MaestroBus>>,
        arm_right_servo: LedcServo<'static>,
        telemetry: Arc<Mutex<TelemetryState>>,
    ) -> WallEBody {
        let make_ch = |cal: JointCal| -> Box<dyn ServoActuator<Error = EspError>> {
            Box::new(MaestroChannel::new(
                Arc::clone(bus),
                cal.channel,
                cal.min_us,
                cal.max_us,
                cal.inverted,
            ))
        };

        WallEBody::new(
            make_ch(self.head_rotation),
            make_ch(self.neck_top),
            make_ch(self.neck_bot),
            make_ch(self.eye_left),
            make_ch(self.eye_right),
            make_ch(self.arm_left),
            Box::new(arm_right_servo),
            telemetry,
        )
    }
}

