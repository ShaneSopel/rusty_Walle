use embedded_graphics::{
    mono_font::{iso_8859_5::FONT_6X10, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
    text::{Baseline, Text},
};
use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio::{Output, PinDriver};
use esp_idf_hal::spi::{SpiDeviceDriver, SpiDriver};
use st7735_lcd::{Orientation, ST7735};

use super::DiagnosticsDisplay;
use crate::diag::TelemetryState;

#[derive(Debug)]
pub enum DisplayError {
    HardwareInit,
    RenderFailed,
}

pub struct St7735Display<'d>
{
    tft: ST7735<
        SpiDeviceDriver<'d, SpiDriver<'d>>,
        PinDriver<'d, Output>,
        PinDriver<'d, Output>,
    >,
}

impl<'d> St7735Display<'d>
{
    pub fn new(
        spi: SpiDeviceDriver<'d, SpiDriver<'d>>,
        dc: PinDriver<'d,  Output>,
        rst: PinDriver<'d,  Output>,
        width: u32,
        height: u32,
        rgb: bool,
    ) -> Self {
        let tft = ST7735::new(spi, dc, rst, rgb, false, width, height);
        Self { tft }
    }

    fn brand_style() -> MonoTextStyle<'static, Rgb565> {
        MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(Rgb565::CYAN)
            .background_color(Rgb565::BLACK)
            .build()
    }

    fn header_style() -> MonoTextStyle<'static, Rgb565> {
        MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(Rgb565::YELLOW)
            .background_color(Rgb565::BLACK)
            .build()
    }

    fn data_style() -> MonoTextStyle<'static, Rgb565> {
        MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(Rgb565::WHITE)
            .background_color(Rgb565::BLACK)
            .build()
    }
}

impl<'d> DiagnosticsDisplay for St7735Display<'d> {
    type Error = DisplayError;

    fn init(&mut self) -> Result<(), Self::Error> {
        let mut delay = Ets;
        self.tft.init(&mut delay).map_err(|_| DisplayError::HardwareInit)?;
        self.tft
            .set_orientation(&Orientation::Landscape)
            .map_err(|_| DisplayError::HardwareInit)?;
        self.tft.set_offset(0, 0);
        self.tft
            .clear(Rgb565::BLACK)
            .map_err(|_| DisplayError::HardwareInit)?;
        Ok(())
    }

    fn render(&mut self, state: &TelemetryState) -> Result<(), Self::Error> {
        //Title banner ("SW@GL0RD WALL-E" - 15 chars -> centered at x = 35)
        Text::with_baseline(
            "SW@GL0яD WALL-E",
            Point::new(35, 4),
            Self::brand_style(),
            Baseline::Top,
        )
        .draw(&mut self.tft)
        .map_err(|_| DisplayError::RenderFailed)?;

        let filled_bars = (state.charge_pct / 10).min(10);
        for i in 0..10u8 {
            let x = 6 + (i as i32 * 15);
            let color = if i < filled_bars {
                if i < 2 {
                    Rgb565::RED
                } else if i < 5 {
                    Rgb565::YELLOW
                } else {
                    Rgb565::GREEN
                }
            } else {
                Rgb565::CSS_DARK_SLATE_GRAY
            };

            Rectangle::new(Point::new(x, 18), Size::new(12, 12))
                .into_styled(PrimitiveStyleBuilder::new().fill_color(color).build())
                .draw(&mut self.tft)
                .map_err(|_| DisplayError::RenderFailed)?;
        }

        let header = Self::header_style();
        let data = Self::data_style();

        let lines = [
            (36, format!("POSE: {:<19}", state.active_pose), header),
            (
                50,
                format!(
                    "BUS : {:<19}",
                    if state.maestro_ok {
                        "MAESTRO ONLINE"
                    } else {
                        "UART ERROR"
                    }
                ),
                data,
            ),
            (66, format!("HEAD: {:.2}  ARM_R: {:.2}   ", state.head_yaw, state.arm_r), data),
            (80, format!("NECK: T{:.2} B{:.2}        ", state.neck_top, state.neck_bot), data),
            (94, format!("EYES: L{:.2} R{:.2}        ", state.eye_l, state.eye_r), data),
            (108, format!("ARM_L:{:.2}  CHG:{:>3}%     ", state.arm_l, state.charge_pct), data),
        ];

        for (y, text, style) in lines {
            Text::with_baseline(&text, Point::new(6, y), style, Baseline::Top)
                .draw(&mut self.tft)
                .map_err(|_| DisplayError::RenderFailed)?;
        }

        Ok(())
    }
}