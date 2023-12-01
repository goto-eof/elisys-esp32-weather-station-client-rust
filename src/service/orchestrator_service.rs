use super::{
    client_service::{self, get_configuration},
    peripheral_service::PeripheralService,
};
use crate::{
    config::config::{self, CONFIGURATION_URL},
    dto::config_response::Configuration,
    service::client_service::get_default_configuration,
    util::thread_util,
};
use core::result::Result::Ok as StandardOk;
use esp_idf_svc::sntp::{self, SyncStatus};
use log::{error, info};
pub fn orchestrate() {
    let mut peripheral_service = PeripheralService::new(config::WIFI_SSID, config::WIFI_PASS);
    let mac_address = peripheral_service.get_mac_address();

    let configuration: Result<Configuration, anyhow::Error> =
        get_configuration(CONFIGURATION_URL, &mac_address);

    let configuration = match configuration {
        Err(e) => Some({
            if config::IS_REMOTE_CONFIGURATION_MANDATORY {
                error!("Could not download the remote configuration. REMOTE CONFIGURATION DOWNLOAD IS MANDATORY. Terminating the application...");
                return;
            }
            peripheral_service.led_blink_3_time_short();
            get_default_configuration(e)
        }),
        StandardOk(config) => Some(config),
    };

    let configuration = configuration.unwrap();
    info!("{}", format!("configuration: {:?}", &configuration));
    let client_service = client_service::ClientService::new(
        &configuration.alert_endpoint,
        &configuration.i_am_alive_endpoint,
    );

    peripheral_service.led_blink_1_time_long();

    synchronize_clock();

    loop {
        while !peripheral_service.retry_wifi_connection_if_necessary_and_return_status() {
            peripheral_service.led_blink_3_time_long();
            thread_util::sleep_short();
        }
        info!("sending I AM ALIVE message...");
        send_i_am_alive(&client_service, &mac_address, &mut peripheral_service);

        info!("---<< Gathering information from sensors >>---");
        let lux = peripheral_service.get_lux_measure() as f32;
        info!("lux: {:?}", lux);
        info!("submiting data...");
        if client_service
            .send_alert(&mac_address, None, None, None, Some(lux), None)
            .is_err()
        {
            error!("cannot send data to server");
            peripheral_service.led_blink_2_time_long();
        } else {
            info!("data sent to server successfully!");
            peripheral_service.led_blink_1_time_short();
        }

        thread_util::sleep_time(configuration.weather_sensor_supply_interval_seconds * 1000);
    }
}

fn send_i_am_alive(
    client_service: &client_service::ClientService,
    mac_address: &String,
    peripheral_service: &mut PeripheralService,
) {
    if client_service.send_i_am_alive(mac_address).is_err() {
        log::error!("failed to send is alive ack");
        peripheral_service.led_blink_2_time_short();
    }
}

fn synchronize_clock() {
    let sntp = sntp::EspSntp::new_default();
    if sntp.is_err() {
        error!("unable to set system time");
        return;
    }
    let sntp = sntp.unwrap();
    info!("SNTP initialized, waiting for status!");
    while sntp.get_sync_status() != SyncStatus::Completed {
        thread_util::sleep_short();
    }
}
