use anyhow::{anyhow, Result};
use embedded_graphics::{
    mono_font::{ascii::FONT_5X8, ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Line, PrimitiveStyle, Rectangle},
    text::{Baseline, Text},
};
use esp_idf_hal::i2c::I2cDriver;
use ssd1306::{
    mode::BufferedGraphicsMode,
    prelude::*,
    I2CDisplayInterface,
    Ssd1306,
};

use crate::diag::telemetry::TelemetryState;
use crate::display::DiagnosticsDisplay;

/// Concrete 0.96" 128x64 I2C SSD1306 OLED implementation of `DiagnosticsDisplay`.
pub struct Ssd1306Display<'d> {
    display: Ssd1306<
        I2CInterface<I2cDriver<'d>>,
        DisplaySize128x64,
        BufferedGraphicsMode<DisplaySize128x64>,
    >,
}

impl<'d> Ssd1306Display<'d> {
    /// Wraps the I2C driver in an uninitialized SSD1306 128x64 buffered display.
    /// Call `.init()` (via `DiagnosticsDisplay`) before rendering.
    pub fn new(i2c: I2cDriver<'d>) -> Self {
        let interface = I2CDisplayInterface::new(i2c);
        let display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();

        Self { display }
    }

    /// Draws WALL-E's 10-segment solar charge meter across y = 14..30.
    fn draw_solar_meter(&mut self, charge_pct: u8) -> Result<()> {
        let clamped = charge_pct.min(100);
        let active_bars = ((clamped as u16 * 10 + 50) / 100).min(10) as i32;

        let label_style = MonoTextStyleBuilder::new()
            .font(&FONT_5X8)
            .text_color(BinaryColor::On)
            .build();

        Text::with_baseline("SOLAR", Point::new(0, 14), label_style, Baseline::Top)
            .draw(&mut self.display)
            .map_err(|e| anyhow!("Draw error: {:?}", e))?;

        let start_x = 30;
        let bar_y = 14;
        let bar_w = 6;
        let bar_h = 13;
        let step_x = 8;

        let outline = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
        let filled = PrimitiveStyle::with_fill(BinaryColor::On);

        for i in 0..10 {
            let x = start_x + (i * step_x);
            let rect = Rectangle::new(Point::new(x, bar_y), Size::new(bar_w as u32, bar_h as u32));
            if i < active_bars {
                rect.into_styled(filled)
                    .draw(&mut self.display)
                    .map_err(|e| anyhow!("Draw error: {:?}", e))?;
            } else {
                rect.into_styled(outline)
                    .draw(&mut self.display)
                    .map_err(|e| anyhow!("Draw error: {:?}", e))?;
            }
        }

        let pct_str = format!("{:>3}%", clamped);
        Text::with_baseline(&pct_str, Point::new(108, 17), label_style, Baseline::Top)
            .draw(&mut self.display)
            .map_err(|e| anyhow!("Draw error: {:?}", e))?;

        Ok(())
    }
}

impl<'d> DiagnosticsDisplay for Ssd1306Display<'d> {
    type Error = anyhow::Error;

    fn init(&mut self) -> Result<(), Self::Error> {
        self.display
            .init()
            .map_err(|e| anyhow!("SSD1306 OLED init failed: {:?}", e))?;
        self.display.clear_buffer();
        self.display
            .flush()
            .map_err(|e| anyhow!("SSD1306 initial flush failed: {:?}", e))?;
        Ok(())
    }

    fn render(&mut self, state: &TelemetryState) -> Result<(), Self::Error> {
        self.display.clear_buffer();

        // 1. Top Inverted Header Banner (128 x 11 px)
        Rectangle::new(Point::new(0, 0), Size::new(128, 11))
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(&mut self.display)
            .map_err(|e| anyhow!("Draw error: {:?}", e))?;

        let banner_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(BinaryColor::Off)
            .build();

        let bus_tag = if state.maestro_ok { "OK" } else { "ER" };
        let header = format!("SW@GL0RD WALL-E [{}]", bus_tag);
        Text::with_baseline(&header, Point::new(2, 1), banner_style, Baseline::Top)
            .draw(&mut self.display)
            .map_err(|e| anyhow!("Draw error: {:?}", e))?;

        // 2. Solar Charge Bar (10 segments)
        self.draw_solar_meter(state.charge_pct)?;

        // Horizontal divider
        Line::new(Point::new(0, 31), Point::new(127, 31))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut self.display)
            .map_err(|e| anyhow!("Draw error: {:?}", e))?;

        // 3. Active Pose / Expression Line
        let body_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(BinaryColor::On)
            .build();

        let pose_line = format!("POSE: {}", state.active_pose);
        Text::with_baseline(&pose_line, Point::new(0, 35), body_style, Baseline::Top)
            .draw(&mut self.display)
            .map_err(|e| anyhow!("Draw error: {:?}", e))?;

        // 4. Compact Joint Position Readout (Normalized 0..100%)
        let small_style = MonoTextStyleBuilder::new()
            .font(&FONT_5X8)
            .text_color(BinaryColor::On)
            .build();

        let joints_line1 = format!(
            "HD:{:02} NT:{:02} NB:{:02}",
            (state.head_yaw* 99.0) as u8,
            (state.neck_top * 99.0) as u8,
            (state.neck_bot * 99.0) as u8,
        );
        Text::with_baseline(&joints_line1, Point::new(0, 47), small_style, Baseline::Top)
            .draw(&mut self.display)
            .map_err(|e| anyhow!("Draw error: {:?}", e))?;

        let joints_line2 = format!(
            "E:{:02}/{:02} A:{:02}/{:02}",
            (state.eye_l * 99.0) as u8,
            (state.eye_r * 99.0) as u8,
            (state.arm_l * 99.0) as u8,
            (state.arm_r * 99.0) as u8,
        );
        Text::with_baseline(&joints_line2, Point::new(0, 56), small_style, Baseline::Top)
            .draw(&mut self.display)
            .map_err(|e| anyhow!("Draw error: {:?}", e))?;

        // Push framebuffer to SSD1306 over I2C
        self.display
            .flush()
            .map_err(|e| anyhow!("SSD1306 flush error: {:?}", e))?;

        Ok(())
    }
}