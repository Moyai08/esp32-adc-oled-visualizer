# Servo Visualizer in Rust

Small ESP32 project in Rust. A potentiometer controls an angle indicator on a
128×64 SSD1306 OLED. The next step is to use that calculated angle to generate a PWM signal for a servo.

At the moment it:

- reads the potentiometer with the ESP32 ADC
- maps the ADC value to a range from 0° to 180°
- draws the two limits and the current position on the OLED

## Wiring

- OLED: SDA to GPIO21, SCL to GPIO22, plus 3.3 V and GND
- Potentiometer: middle pin to GPIO32, outer pins to 3.3 V and GND

## Run

With the ESP Rust toolchain and `espflash` installed:

```sh
cargo run
```

## Next up
- use the calculated angle to generate the PWM signal and move the servo