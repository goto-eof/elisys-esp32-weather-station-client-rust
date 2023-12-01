use serde::Serialize;

#[derive(Serialize)]
#[warn(non_snake_case)]
pub struct RequestSubmit {
    #[serde(rename = "macAddress")]
    mac_address: String,
    temperature: Option<f32>,
    humidity: Option<f32>,
    pressure: Option<f32>,
    lux: Option<f32>,
    light: Option<bool>,
}

impl RequestSubmit {
    pub fn new(
        mac_address: String,
        temperature: Option<f32>,
        humidity: Option<f32>,
        pressure: Option<f32>,
        lux: Option<f32>,
        light: Option<bool>,
    ) -> RequestSubmit {
        RequestSubmit {
            mac_address,
            temperature,
            humidity,
            pressure,
            lux,
            light,
        }
    }
}
