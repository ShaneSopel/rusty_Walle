use std::sync::{Arc, Mutex};
use esp_idf_hal::sys::EspError;

use crate::actuators::ServoActuator;
use crate::diag::TelemetryState;

pub type Joint = Box<dyn ServoActuator<Error = EspError>>;

pub struct WallEBody {
    pub head_rotation: Joint,
    pub neck_top: Joint,
    pub neck_bottom: Joint,
    pub eye_left: Joint,
    pub eye_right: Joint,
    pub arm_left: Joint,
    pub arm_right: Joint,
    telemetry: Arc<Mutex<TelemetryState>>,
}

impl WallEBody {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        head_rotation: Joint,
        neck_top: Joint,
        neck_bottom: Joint,
        eye_left: Joint,
        eye_right: Joint,
        arm_left: Joint,
        arm_right: Joint,
        telemetry: Arc<Mutex<TelemetryState>>,
    ) -> Self {
        Self {
            head_rotation,
            neck_top,
            neck_bottom,
            eye_left,
            eye_right,
            arm_left,
            arm_right,
            telemetry,
        }
    }

    pub fn home_all(&mut self) -> Result<(), EspError> {
        for joint in self.all_joints_mut() {
            joint.set_speed(40)?;
            joint.set_accel(20)?;
            joint.set_position(0.5)?;
        }

        let mut t = self.telemetry.lock().unwrap();
        t.active_pose = "HOME ALL";
        t.head_yaw = 0.5;
        t.neck_top = 0.5;
        t.neck_bot = 0.5;
        t.eye_l = 0.5;
        t.eye_r = 0.5;
        t.arm_l = 0.5;
        t.arm_r = 0.5;
        Ok(())
    }

    pub fn look_at(&mut self, yaw: f32, pitch_top: f32, pitch_bot: f32) -> Result<(), EspError> {
        let yaw = yaw.clamp(0.0, 1.0);
        let pitch_top = pitch_top.clamp(0.0, 1.0);
        let pitch_bot = pitch_bot.clamp(0.0, 1.0);

        self.head_rotation.set_position(yaw)?;
        self.neck_top.set_position(pitch_top)?;
        self.neck_bottom.set_position(pitch_bot)?;

        let mut t = self.telemetry.lock().unwrap();
        t.active_pose = "LOOK AT";
        t.head_yaw = yaw;
        t.neck_top = pitch_top;
        t.neck_bot = pitch_bot;
        Ok(())
    }

    pub fn set_eyes(&mut self, left: f32, right: f32) -> Result<(), EspError> {
        let left = left.clamp(0.0, 1.0);
        let right = right.clamp(0.0, 1.0);

        self.eye_left.set_position(left)?;
        self.eye_right.set_position(right)?;

        let mut t = self.telemetry.lock().unwrap();
        t.active_pose = "TILT EYES";
        t.eye_l = left;
        t.eye_r = right;
        Ok(())
    }

    pub fn set_arms(&mut self, left: f32, right: f32) -> Result<(), EspError> {
        let left = left.clamp(0.0, 1.0);
        let right = right.clamp(0.0, 1.0);

        self.arm_left.set_position(left)?;
        self.arm_right.set_position(right)?;

        let mut t = self.telemetry.lock().unwrap();
        t.active_pose = "ARM MOVE";
        t.arm_l = left;
        t.arm_r = right;
        Ok(())
    }

    pub fn limp_all(&mut self) -> Result<(), EspError> {
        for joint in self.all_joints_mut() {
            joint.limp()?;
        }
        self.telemetry.lock().unwrap().active_pose = "LIMP / RELAXED";
        Ok(())
    }

    fn all_joints_mut(&mut self) -> [&mut Joint; 7] {
        [
            &mut self.head_rotation,
            &mut self.neck_top,
            &mut self.neck_bottom,
            &mut self.eye_left,
            &mut self.eye_right,
            &mut self.arm_left,
            &mut self.arm_right,
        ]
    }
}