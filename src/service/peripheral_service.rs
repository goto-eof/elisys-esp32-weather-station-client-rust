use bh1750_ehal::BH1750;
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_hal::{
    delay::{self, Ets},
    gpio::{Gpio5, Output, PinDriver},
    i2c::{I2cConfig, I2cDriver},
    peripherals::Peripherals,
};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    nvs::EspDefaultNvsPartition,
    wifi::{BlockingWifi, EspWifi, WifiDeviceId},
};
use log::info;

use crate::util::thread_util;

const TIME_SHORT: u64 = 20;
const TIME_LONG: u64 = 1000;

pub struct PeripheralService {
    led: PinDriver<'static, Gpio5, Output>,
    bh1750: BH1750<I2cDriver<'static>, Ets>,
    wifi: BlockingWifi<EspWifi<'static>>,
    wifi_ssid: String,
    wifi_password: String,
}

impl PeripheralService {
    pub fn new(wifi_ssid: &str, wifi_password: &str) -> Self {
        let peripherals = Peripherals::take().unwrap();
        let led = PinDriver::output(peripherals.pins.gpio5).unwrap();

        let sys_loop = EspSystemEventLoop::take().unwrap();
        let nvs = EspDefaultNvsPartition::take().unwrap();

        let mut wifi = BlockingWifi::wrap(
            EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs)).unwrap(),
            sys_loop,
        )
        .unwrap();
        let mut wifi_connection = connect_wifi(&mut wifi, wifi_ssid, wifi_password);
        while wifi_connection.is_err() {
            thread_util::sleep_time(TIME_LONG);
            wifi_connection = connect_wifi(&mut wifi, wifi_ssid, wifi_password);
        }

        info!("configuring light sensor...");
        let sda = peripherals.pins.gpio21;
        let scl = peripherals.pins.gpio22;
        let config = I2cConfig::new().baudrate(400000.into());
        let i2c_instance = I2cDriver::new(peripherals.i2c0, sda, scl, &config).unwrap();
        let bh1750 =
            bh1750_ehal::BH1750::new(i2c_instance, delay::Ets, bh1750_ehal::Address::ADDR_L)
                .unwrap();
        info!("configuration of light sensor completed");

        let peripheral_service = PeripheralService {
            led,
            wifi,
            wifi_ssid: wifi_ssid.to_owned(),
            wifi_password: wifi_password.to_owned(),
            bh1750,
        };
        return peripheral_service;
    }

    pub fn retry_wifi_connection_if_necessary_and_return_status(&mut self) -> bool {
        if !self.wifi.is_connected().unwrap() {
            if connect_wifi(&mut self.wifi, &self.wifi_ssid, &self.wifi_password).is_err() {
                self.led_blink_3_time_long();
                return false;
            }
        }
        return true;
    }

    pub fn get_lux_measure(&mut self) -> u32 {
        self.bh1750
            .start_measurement(bh1750_ehal::ContinuesMeasurement::HIHGT_RES2);

        let value = self
            .bh1750
            .get_measurement(bh1750_ehal::ContinuesMeasurement::HIHGT_RES2);
        value
    }

    pub fn led_blink_3_time_short(&mut self) {
        self.led_blink_1_time(TIME_SHORT);
        self.led_blink_1_time(TIME_SHORT);
        self.led_blink_1_time(TIME_SHORT);
    }

    pub fn led_blink_3_time_long(&mut self) {
        self.led_blink_1_time(TIME_LONG);
        self.led_blink_1_time(TIME_LONG);
        self.led_blink_1_time(TIME_LONG);
    }

    pub fn led_blink_2_time_short(&mut self) {
        self.led_blink_1_time(TIME_SHORT);
        self.led_blink_1_time(TIME_SHORT);
    }

    pub fn led_blink_2_time_long(&mut self) {
        self.led_blink_1_time(TIME_LONG);
        self.led_blink_1_time(TIME_LONG);
    }

    pub fn led_blink_1_time_short(&mut self) {
        self.led_blink_1_time(TIME_SHORT);
    }

    pub fn led_blink_1_time_long(&mut self) {
        self.led_blink_1_time(TIME_LONG);
    }

    pub fn get_mac_address(&self) -> String {
        let mav = &self
            .wifi
            .wifi()
            .driver()
            .get_mac(WifiDeviceId::Sta)
            .unwrap();
        let mac_address_obj =
            macaddr::MacAddr6::new(mav[0], mav[1], mav[2], mav[3], mav[4], mav[5]);
        let mac_address_value = mac_address_obj.to_string();
        info!("MAC_ADDRESS: {:?}", mac_address_value);
        mac_address_value
    }

    fn led_blink_1_time(&mut self, time: u64) {
        self.led.set_high().unwrap();
        thread_util::sleep_time(time);
        self.led.set_low().unwrap();
        thread_util::sleep_time(time);
    }
}

fn connect_wifi(
    wifi: &mut BlockingWifi<EspWifi<'static>>,
    ssid: &str,
    password: &str,
) -> anyhow::Result<()> {
    let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
        ssid: ssid.into(),
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: password.into(),
        channel: None,
    });
    info!("Connecting to SSID: {}", ssid);
    wifi.set_configuration(&wifi_configuration)?;

    wifi.start()?;
    info!("Wifi started");

    wifi.connect()?;
    info!("Wifi connected: {}", ssid);

    wifi.wait_netif_up()?;
    info!("Wifi netif up");

    Ok(())
}
