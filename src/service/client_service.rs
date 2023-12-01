use crate::{
    config::config::{
        DEFAULT_ALERT_URL, DEFAULT_I_AM_ALIVE_URL, TEMPERATURE_SENSOR_UNIT_OF_MEASURE,
        WEATHER_SENSOR_SUPPLY_INTERVAL_SECONDS,
    },
    dto::{
        config_request::ConfigRequest, config_response::Configuration,
        request_submit::RequestSubmit, request_i_am_alive::RequestIAmAlive,
    },
};
use anyhow::{Error, Ok};
use embedded_svc::{http::client::Client as HttpClient, io::Write, utils::io};
use esp_idf_svc::http::client::EspHttpConnection;
use esp_idf_sys as _;
use log::{error, info};
use std::result::Result::Ok as StandardOk;
pub struct ClientService {
    alert_url: String,
    i_am_alive_url: String,
}

impl ClientService {
    pub fn new(alert_url: &str, i_am_alive_url: &str) -> ClientService {
        ClientService {
            alert_url: alert_url.to_owned(),
            i_am_alive_url: i_am_alive_url.to_owned(),
        }
    }

    pub fn send_alert(
        &self,
        mac_address: &str,
        temperature: Option<f32>,
        humidity: Option<f32>,
        pressure: Option<f32>,
        lux: Option<f32>,
        light: Option<bool>,
    ) -> anyhow::Result<(), anyhow::Error> {
        let client = HttpClient::wrap(EspHttpConnection::new(&Default::default())?);

        let payload = serde_json::to_string(&RequestSubmit::new(
            mac_address.to_owned(),
            temperature,
            humidity,
            pressure,
            lux,
            light,
        ))
        .unwrap();
        let payload = payload.as_bytes();

        info!("trying to send data...");
        let result = post_request(payload, client, &self.alert_url);
        info!("data sent? {}", !result.is_err());
        return match result {
            Err(e) => Err(e.into()),
            StandardOk(_) => Ok(()),
        };
    }

    pub fn send_i_am_alive(&self, mac_address: &str) -> anyhow::Result<(), anyhow::Error> {
        let client = HttpClient::wrap(EspHttpConnection::new(&Default::default())?);
        let payload = serde_json::to_string(&RequestIAmAlive::new(mac_address.to_owned())).unwrap();
        let payload = payload.as_bytes();

        info!("trying to send is alive ack...");
        let result = post_request(payload, client, &self.i_am_alive_url);
        info!("ack sent? {}", !result.is_err());
        return match result {
            Err(e) => Err(e.into()),
            StandardOk(_) => Ok(()),
        };
    }
}

pub fn get_configuration(
    configuration_uri: &str,
    mac_address: &str,
) -> anyhow::Result<Configuration, anyhow::Error> {
    let client = HttpClient::wrap(EspHttpConnection::new(&Default::default())?);
    let payload = serde_json::to_string(&ConfigRequest::new(mac_address.to_owned())).unwrap();
    let payload = payload.as_bytes();

    info!("[config downloader]: trying to get remote configuration...");
    let result = post_request(payload, client, configuration_uri);
    info!(
        "[config downloader]: configuration retrieved with success? {}",
        !result.is_err()
    );

    match result {
        StandardOk(body_string) => {
            let configuration: Result<Configuration, serde_json::Error> =
                serde_json::from_str(&body_string);
            info!("{:?}", configuration);

            if configuration.is_err() {
                let err = configuration.err().unwrap();
                error!(
            "[config downloader]: error while trying to parse the configuration response: {}",
            &err
        );
                return Err(err.into());
            }

            let configuration = configuration.unwrap();
            info!(
                "[config downloader]: Remote configuration loaded successfully: {:?}",
                configuration
            );
            return Ok(configuration);
        }
        Err(e) => {
            error!("[config downloader]: Error decoding response body: {}", e);
            return Err(e.into());
        }
    }
}

fn post_request(
    payload: &[u8],
    mut client: HttpClient<EspHttpConnection>,
    url: &str,
) -> Result<String, Error> {
    let content_length_header = format!("{}", payload.len());
    let headers = [
        ("content-type", "application/json"),
        ("content-length", &*content_length_header),
    ];

    let request = client.post(url, &headers);

    if request.is_err() {
        let message = format!("connection error: {:?}", request.err());
        error!("{}", message);
        return Err(Error::msg(message));
    }
    let mut request = request.unwrap();

    if request.write_all(payload).is_err() {
        let message = format!("connection error while trying to write all");
        error!("{}", message);
        return Err(Error::msg(message));
    }
    if request.flush().is_err() {
        let message = format!("connection error while trying to flush");
        error!("{}", message);
        return Err(Error::msg(message));
    }
    info!("-> POST {}", url);
    let response = request.submit();
    if response.is_err() {
        let message = format!("connection error while trying to read response");
        error!("{}", message);
        return Err(Error::msg(message));
    }
    let mut response = response.unwrap();

    let status = response.status();
    info!("<- {}", status);
    let mut buf = [0u8; 4086];
    let bytes_read = io::try_read_full(&mut response, &mut buf).map_err(|e| e.0);

    if bytes_read.is_err() {
        let message = format!(
            "connection error while trying to read response: {:?}",
            bytes_read.err()
        );
        error!("{}", message);
        return Err(Error::msg(message));
    } else {
        let bytes_read = bytes_read.unwrap();
        return match std::str::from_utf8(&buf[0..bytes_read]) {
            Err(e) => Err(Error::msg(format!("{:?}", e))),
            StandardOk(str) => Ok(str.to_owned()),
        };
    }
}

pub fn get_default_configuration(e: Error) -> Configuration {
    error!(
        "Error while trying to load configuration from remote server: {:?}",
        e
    );
    Configuration {
        alert_endpoint: DEFAULT_ALERT_URL.to_owned(),
        i_am_alive_endpoint: DEFAULT_I_AM_ALIVE_URL.to_owned(),
        temperature_sensor_unit_of_measure: TEMPERATURE_SENSOR_UNIT_OF_MEASURE.to_owned(),
        weather_sensor_supply_interval_seconds: WEATHER_SENSOR_SUPPLY_INTERVAL_SECONDS,
    }
}
