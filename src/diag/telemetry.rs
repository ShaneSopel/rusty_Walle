#[derive(Clone, Debug)]
pub struct TelemetryState {
    pub active_pose: &'static str,
    pub head_yaw: f32,
    pub neck_top: f32,
    pub neck_bot: f32,
    pub eye_l: f32,
    pub eye_r: f32, 
    pub arm_l: f32,
    pub arm_r: f32,
    pub charge_pct: u8,
    pub maestro_ok: bool,
}

impl Default for TelemetryState {
    fn default() -> Self {
        Self {
            active_pose: "IDLE / HOME",
            head_yaw: 0.5,
            neck_top: 0.5,
            neck_bot: 0.5,
            eye_l: 0.5,
            eye_r: 0.5,
            arm_l: 0.5,
            arm_r: 0.5,
            charge_pct: 100,
            maestro_ok: true,
        }
    }
}
