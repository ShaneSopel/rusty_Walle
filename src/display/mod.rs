pub mod st7735;
pub mod ssd3106_oled;

pub use st7735::St7735Display;

use crate::diag::TelemetryState;

pub trait DiagnosticsDisplay: Send {
    type Error;

    fn init(&mut self) -> Result<(), Self::Error>;
    fn render(&mut self, state: &TelemetryState) -> Result<(), Self::Error>;
}