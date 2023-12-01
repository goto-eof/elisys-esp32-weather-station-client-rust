use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Configuration {
    #[serde(rename = "alertEndpoint")]
    pub alert_endpoint: String,
    #[serde(rename = "iAmAliveEndpoint")]
    pub i_am_alive_endpoint: String,
    // #[serde(rename = "iAmAliveIntervalSeconds")]
    // pub i_am_alive_interval_seconds: u64,
    // pub crontab: String,
    // #[serde(rename = "timezoneOffsetSec")]
    // pub timezone_offset: i32,
    #[serde(rename = "temperatureSensorUnitOfMeasure")]
    pub temperature_sensor_unit_of_measure: String,
    #[serde(rename = "weatherSensorSupplyIntervalSeconds")]
    pub weather_sensor_supply_interval_seconds: u64,
}
