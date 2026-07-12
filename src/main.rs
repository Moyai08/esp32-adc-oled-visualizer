use embedded_graphics::{
    geometry::Point, pixelcolor::BinaryColor, prelude::*, primitives::Line,
    primitives::PrimitiveStyle,
};

use esp_idf_svc::hal::{
    i2c::{I2cConfig, I2cDriver},
    peripherals::Peripherals,
    units::*,
};
use ssd1306::{
    mode::DisplayConfig, prelude::DisplayRotation, size::DisplaySize128x64, I2CDisplayInterface,
    Ssd1306,
};

mod adc_task;

const DEFAULT_LINE_POINTS: [Point; 2] = [Point::new(0, 0), Point::new(127, 0)];
const ADC_MAX: f32 = 4095.0; // ADC driver currently only supports 12 bit values
const MAX_ANGLE: f32 = 180.0;
const DISPLAY_MAX_X: f32 = 127.0;

fn adc_to_angle(raw_adc: u16) -> f32 {
    raw_adc as f32 / ADC_MAX * MAX_ANGLE
}

fn angle_to_point(angle: f32) -> Point {
    let x = (angle / MAX_ANGLE * DISPLAY_MAX_X).round() as i32;

    Point::new(x, 0)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let cfg = I2cConfig::new().baudrate(400.kHz().into());

    let peripherals = Peripherals::take()?;
    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio21;
    let scl = peripherals.pins.gpio22;

    let adc1 = peripherals.adc1;
    let i2s0 = peripherals.i2s0;
    let adc_pin = peripherals.pins.gpio32;
    let pot = adc_task::spawn(adc1, i2s0, adc_pin)?;

    let i2c_driver = I2cDriver::new(i2c, sda, scl, &cfg)?;
    let display_interface = I2CDisplayInterface::new(i2c_driver);
    let mut display = Ssd1306::new(
        display_interface,
        DisplaySize128x64,
        DisplayRotation::Rotate0,
    )
    .into_buffered_graphics_mode();
    display.init().unwrap();

    let display_size = display.size();
    let starting_point = Point::new(
        ((display_size.width - 1) / 2) as i32,
        (display_size.height - 1) as i32,
    );
    loop {
        display.clear(BinaryColor::Off).unwrap();

        DEFAULT_LINE_POINTS.iter().for_each(|point| {
            let line = Line::new(starting_point, *point)
                .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1));
            line.draw(&mut display).unwrap();
        });

        let angle = adc_to_angle(pot.latest());
        let current_point = angle_to_point(angle);

        Line::new(starting_point, current_point)
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut display)
            .unwrap();

        display.flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
