# Elisys ESP32 Weather Station (Rust)

Elisys ESP32 Weather Station is a weather station that reads data from sensors like light sensor, temperature sensor, humidity and pressure sensors and sends them values to a [server](https://github.com/goto-eof/elisys-home-automation-server-java).

## Features

- read data from light sensor (lux value);
- read temperature from a sensor (WIP);
- read humidity from a sensor (WIP);
- read pressure from a sensor (WIP).

# GPIO

| GPIO    | Description                     |
| ------- | ------------------------------- |
| GPIO5   | LED                             |
| GPIO15  | termometer and humidity sensor  |
| GPIO21  | SDA - light sensor              |
| GPIO22  | SCL - light sensor              |
| ------- | ------------------------------- |
