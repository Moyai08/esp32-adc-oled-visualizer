# ESP32 ADC OLED Visualizer

Small ESP32 project in Rust. An analog input is read continuously through the ESP32 ADC. Its value is mapped to an angle between 0° and 180° and visualized live on a 128×64 SSD1306 OLED display.

The project targets the Xtensa-based ESP32.

At the moment it:

- reads an analog input with the ESP32 ADC
- maps the ADC value to a range from 0° to 180°
- draws the range limits and the current position on the OLED

## Wiring

- OLED: SDA to GPIO21, SCL to GPIO22, plus 3.3 V and GND
- Analog input source (currently a potentiometer): GPIO32

## Run

With the ESP Rust toolchain and `espflash` installed:

```sh
cargo run
```
