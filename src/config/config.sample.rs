// rename the file in config.rs
// customize your settings by editing this variables
// ------------------------------------------------------------------
// wifi name
pub const WIFI_SSID: &str = "wifi name";
// wifi password
pub const WIFI_PASS: &str = "wifi password";
// endpoint that is used to send an alert after a movement detection
pub const DEFAULT_ALERT_URL: &str = "http://192.168.1.102:8080/api/v1/weather-sensor/submit";
// endpoint on which the server is informed that the device is alive
pub const DEFAULT_I_AM_ALIVE_URL: &str = "http://192.168.1.102:8080/api/v1/i-am-alive/notify";
// time interval between is alive requests
// pub const DEFAULT_I_AM_ALIVE_INTERVAL_SECONDS: u64 = 30;
// endpoint for configuration download
pub const CONFIGURATION_URL: &str = "http://192.168.1.102:8080/api/v1/weather-sensor/configuration";
// the unit of measure of the temperature sensor - could be "F" or "C"
pub const TEMPERATURE_SENSOR_UNIT_OF_MEASURE: &str = "C";
// if enabled, if cannot download configuration then will terminate the application
pub const IS_REMOTE_CONFIGURATION_MANDATORY: bool = false;
// the time interval between the communication with the server
pub const WEATHER_SENSOR_SUPPLY_INTERVAL_SECONDS: u64 = 30;
// This is the default crontab value if server value is wrong
// pub const DEFAULT_CRONTAB: &str =
// "0-59   0-59   0-23     1-31       Jan-Dec  Mon,Tue,Wed,Thu,Fri,Sat,Sun  2023-2100";
