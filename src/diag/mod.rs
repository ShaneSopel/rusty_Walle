pub mod telemetry;

pub use telemetry::TelemetryState;

use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::display::DiagnosticsDisplay;

pub fn spawn_diagnostics_task<E: Debug + 'static>(
    mut display: Box<dyn DiagnosticsDisplay<Error = E> + 'static>,
    telemetry: Arc<Mutex<TelemetryState>>,
    refresh_ms: u64,
){
    thread::spawn(move || {
        if let Err(e) = display.init() {
            log::error!("Diagnostics display init failed: {:?}", e);
            return;
        }

        loop {
            let snapshot = telemetry.lock().unwrap().clone();
            if let Err(e) = display.render(&snapshot) {
                log::warn!("Diagnostics display render error: {:?}", e);
            }
            thread::sleep(Duration::from_millis(refresh_ms));
        }
    });
}