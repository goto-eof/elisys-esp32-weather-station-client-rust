# Elisys ESP32 Weather Station (Rust)

Elisys ESP32 Weather Station is a weather station, implemented using Rust programming language, that reads data from sensors like light sensor, temperature sensor, humidity and pressure sensors and sends then the values to a server like [Elisys Home Automation Server (Java)](https://github.com/goto-eof/elisys-home-automation-server-java). This software belongs to the [Elisys Home Automation Software suite](https://github.com/goto-eof/elisys-home-automation-server-java).

## Features

- register the device on the remote server;
- read data from light sensor (Lux value, sensor: BH1750);
- read temperature from a sensor (sensor: DHT11);
- read humidity from a sensor (sensor: DHT11);
- read pressure from a sensor (**WIP**).

# GPIO

| GPIO   | Description                     |
| ------ | ------------------------------- |
| GPIO5  | LED (device status)             |
| GPIO15 | thermometer and humidity sensor |
| GPIO21 | SDA - light sensor              |
| GPIO22 | SCL - light sensor              |

Tested on ESP32-DevKitC and developed on Linux (Ubuntu).

# Pictures

| Picture                       |
| ----------------------------- |
| ![DHT11](/images/DHT11.jpg)   |
| DHT11                         |
| ![BH1750](/images/BH1750.jpg) |
| BH1750                        |

# Known issues

Sometimes the DHT11 temperature and humidity sensor goes in timeout. It depends on the driver that is used in this application. I introduced an insistent check of the sensor data (it tries, if I am not wrong, about 100 times, before give up). If your DTH11 goes **always** in timeout, then I suggest you to changed the driver (try another rust crate: for DHT11 there are about 3-5 drivers) or write yourself a driver based on the official documentation of the device.

If you found a bug, please ping me [here](https://andre-i.eu/#contactme).
