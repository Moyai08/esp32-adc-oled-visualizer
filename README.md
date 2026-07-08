# servo-visualizer-rs

Tiny ESP32 Rust project.

Right now it reads a potentiometer and shows the raw ADC value on an SSD1306
OLED using embedded-graphics.

Eventually this should become a small servo visualizer:

- read potentiometer value
- map it to a servo angle
- draw the angle on the OLED
- generate PWM
- move the servo

This is unfinished and mainly here so I can push code while figuring things out.

Target is currently xtensa-esp32-espidf, hopefully RISCV in the future.