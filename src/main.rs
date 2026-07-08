use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    esp_idf_svc::sys::link_patches(); // TODO: buy ESP32 with RISCV...
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

    let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);

    loop {
        display.clear(BinaryColor::Off).unwrap();

        Text::with_baseline("Potentiometer", Point::zero(), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        let value = format!("ADC: {}", pot.latest());
        Text::with_baseline(&value, Point::new(0, 16), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        display.flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
