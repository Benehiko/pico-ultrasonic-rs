#![no_std]
#![no_main]

pub mod hc_sr04;

use core::str::{self, from_utf8, FromStr};
use core::{env, option_env};
use cyw43_pio::PioSpi;
use defmt::unwrap;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_net::driver::Driver as NetDriver;
use embassy_net::tcp::TcpSocket;
use embassy_net::{Config, Ipv4Address, Stack, StackResources};
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIN_23, PIN_25, PIO0, USB};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::usb::{Driver, InterruptHandler as USBInterruptHandler};
use embassy_rp::watchdog::Watchdog;
use embassy_time::{Duration, Instant, Timer};
use hc_sr04::HCSR04;
use rust_mqtt::client::client::MqttClient;
use rust_mqtt::client::client_config::ClientConfig;
use rust_mqtt::utils::rng_generator::CountingRng;
use static_cell::StaticCell;

#[cfg(debug_assertions)]
use panic_semihosting as _;

#[cfg(not(debug_assertions))]
use panic_reset as _;

type Device = cyw43::NetDriver<'static>;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => USBInterruptHandler<USB>;
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

const WIFI_NETWORK: &'static str = env!("RP_WIFI_NETWORK");
const WIFI_PASSWORD: &'static str = env!("RP_WIFI_PASSWORD");
const MQTT_SERVER_IP: &'static str = env!("RP_MQTT_SERVER_IP");
const MQTT_SERVER_PORT: &'static str = match option_env!("RP_MQTT_SERVER_PORT") {
    Some(port) => port,
    _ => "1883",
};
const MQTT_USERNAME: &'static str = match option_env!("RP_MQTT_USERNAME") {
    Some(username) => username,
    _ => "",
};
const MQTT_PASSWORD: &'static str = match option_env!("RP_MQTT_PASSWORD") {
    Some(password) => password,
    _ => "",
};

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Debug, driver);
}

#[embassy_executor::task]
async fn wifi_task(
    runner: cyw43::Runner<
        'static,
        Output<'static, PIN_23>,
        PioSpi<'static, PIN_25, PIO0, 0, DMA_CH0>,
    >,
) {
    runner.run().await;
}

async fn blink_led(
    control: &mut cyw43::Control<'static>,
    w: &mut Watchdog,
    led_on_duration: Duration,
    between_cycles_duration: Duration,
    max_cycles: u8,
) {
    let mut current_cycle: u8 = 0;
    loop {
        w.feed();
        control.gpio_set(0, true).await;
        Timer::after(led_on_duration).await;
        control.gpio_set(0, false).await;
        Timer::after(between_cycles_duration).await;
        current_cycle += 1;
        if current_cycle >= max_cycles {
            break;
        }
    }
}

#[embassy_executor::task]
async fn net_task(stack: &'static Stack<Device>) -> ! {
    stack.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let fw = include_bytes!("../../embassy/cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../../embassy/cyw43-firmware/43439A0_clm.bin");

    let mut watchdog = Watchdog::new(p.WATCHDOG);

    // Initialize driver.
    let mut ultrasonic = HCSR04::new(p.PIN_2, p.PIN_3).unwrap();

    // Setup logging.
    let usb = p.USB;
    let driver = Driver::new(usb, Irqs);
    spawner.spawn(logger_task(driver)).unwrap();

    log::info!("Pico starting up!");

    // Setup wifi
    let pwr = Output::new(p.PIN_23, Level::High);
    let cs = Output::new(p.PIN_25, Level::High);
    let mut pio = Pio::new(p.PIO0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        pio.irq0,
        cs,
        p.PIN_24,
        p.PIN_29,
        p.DMA_CH0,
    );

    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;

    let mac_addr: &str = match net_device.hardware_address() {
        embassy_net::driver::HardwareAddress::Ethernet(addr) => from_utf8(addr.into()),
        embassy_net::driver::HardwareAddress::Ieee802154(addr) => addr,
        _ => panic!("Failed to get mac address"),
    };

    log::info!("MAC address: {:x}", mac_addr);

    log::info!("Starting wifi task");
    unwrap!(spawner.spawn(wifi_task(runner)));

    control.init(clm).await;

    // get the device up and running as fast as possible
    log::info!("setting power management mode: performance");
    control
        .set_power_management(cyw43::PowerManagementMode::Performance)
        .await;

    {
        log::info!("Scanning for AP: {}", WIFI_NETWORK);
        let mut scanner = control.scan().await;
        while let Some(bss) = scanner.next().await {
            if let Ok(ssid_str) = str::from_utf8(&bss.ssid) {
                log::info!(
                    "AP: {}, capability: {}, beacon_period: {}",
                    ssid_str,
                    bss.capability,
                    bss.beacon_period
                );
                if ssid_str.eq(WIFI_NETWORK) {
                    log::info!(
                        "Found AP: {}, capability: {}, beacon_period: {}",
                        ssid_str,
                        bss.capability,
                        bss.beacon_period
                    );
                    break;
                }
            }
        }
    }

    log::info!("starting watchdog");
    // watchdog.start(Duration::from_secs(8));

    log::info!("blink 2x long blinks");
    blink_led(
        &mut control,
        &mut watchdog,
        Duration::from_secs(2),
        Duration::from_secs(1),
        2,
    )
    .await;

    let dhcp_config = Config::dhcpv4(Default::default());

    let seed = 0x0123_4567_89ab_cdef; // chosen by fair dice roll. guarenteed to be random.

    static STACK: StaticCell<Stack<cyw43::NetDriver>> = StaticCell::new();
    static RESOURCES: StaticCell<StackResources<2>> = StaticCell::new();
    let stack = &*STACK.init(Stack::new(
        net_device,
        dhcp_config,
        RESOURCES.init(StackResources::<2>::new()),
        seed,
    ));

    log::info!("starting net task");
    watchdog.feed();
    unwrap!(spawner.spawn(net_task(&stack)));

    log::info!("blink 5x short blinks");
    blink_led(
        &mut control,
        &mut watchdog,
        Duration::from_secs(1),
        Duration::from_millis(500),
        5,
    )
    .await;

    log::info!("waiting 2 seconds...");
    Timer::after_secs(2).await;

    // infinite try to connect to wifi
    const MAX_RETRIES: usize = 10;
    let mut current: usize = 0;
    loop {
        watchdog.feed();
        blink_led(
            &mut control,
            &mut watchdog,
            Duration::from_secs(1),
            Duration::from_secs(1),
            1,
        )
        .await;

        match control.join_wpa2(WIFI_NETWORK, WIFI_PASSWORD).await {
            Ok(_) => {
                log::info!("connected!");
                break;
            }
            Err(e) => {
                log::error!("join failed: {}", e.status);
                Timer::after_millis(500).await;
            }
        }
        if current >= MAX_RETRIES {
            panic!("Failed to connect to wifi");
        }
        current += 1;
    }

    watchdog.feed();

    log::info!("Wifi connected!");
    log::info!("Waiting 2 seconds...");
    Timer::after_secs(2).await;

    control.gpio_set(0, false).await;
    // wait forever to get ip address
    // Wait for DHCP, not necessary when using static IP
    log::info!("waiting for DHCP...");
    let cfg = wait_for_config(stack, &mut control, &mut watchdog).await;

    control.gpio_set(0, true).await;
    let local_addr = cfg.address.address();
    log::info!("successfully got assigned address {} via dhcp.", local_addr);

    watchdog.feed();
    // go into power save mode
    // log::info!("setting power management mode: power save");
    // control
    //     .set_power_management(cyw43::PowerManagementMode::PowerSave)
    //     .await;

    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];
    let server_port: u16 = MQTT_SERVER_PORT.parse().unwrap();
    let host_addr = Ipv4Address::from_str(MQTT_SERVER_IP).unwrap();
    let addr = (host_addr, server_port);
    log::info!("got server address: {:?}", addr);

    // get sensor data and send to server
    let mut base_line: f64 = 8.0;
    let mut counter: i8 = 0;
    let mut buffer = ryu::Buffer::new();

    let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
    socket.set_timeout(Some(Duration::from_secs(10)));

    // try to connect to the server
    for _ in 0..50 {
        watchdog.feed();
        log::info!("connecting...");
        blink_led(
            &mut control,
            &mut watchdog,
            Duration::from_millis(200),
            Duration::from_millis(200),
            1,
        )
        .await;

        if let Err(e) = socket.connect(addr).await {
            log::warn!("connect error: {:?}", e);
            Timer::after_millis(200).await;
            continue;
        }
        log::info!("Connected to {:?}", socket.remote_endpoint());
        break;
    }

    // we restart the pico if we can't connect to the server
    if socket.remote_endpoint() == None {
        log::error!("Failed to connect to remote server");
        watchdog.trigger_reset();
        return;
    }

    let mut config = ClientConfig::new(
        rust_mqtt::client::client_config::MqttVersion::MQTTv5,
        CountingRng(20000),
    );

    config.add_max_subscribe_qos(rust_mqtt::packet::v5::publish_packet::QualityOfService::QoS1);
    config.add_client_id("pico-");
    config.add_username(MQTT_USERNAME);
    config.add_password(MQTT_PASSWORD);

    config.max_packet_size = 100;
    let mut recv_buffer = [0; 80];
    let mut write_buffer = [0; 80];

    let mut client = MqttClient::<&mut TcpSocket, 5, _>::new(
        &mut socket,
        &mut write_buffer,
        80,
        &mut recv_buffer,
        80,
        config,
    );
    client.connect_to_broker().await.unwrap();

    let mut instant: Instant;
    let mut unit: f64;
    let mut msg: &[u8];

    loop {
        watchdog.feed();

        blink_led(
            &mut control,
            &mut watchdog,
            Duration::from_millis(100),
            Duration::from_millis(100),
            1,
        )
        .await;

        instant = Instant::now();

        unit = match ultrasonic.measure().await {
            Ok(unit) => unit.millimeters,
            Err(_) => -1.0,
        };
        watchdog.feed();

        log::info!("distance: {}mm", unit);
        log::info!("time: {}ms", instant.elapsed().as_millis() as f64);

        if unit == -1.0 {
            log::error!("Failed to measure distance");
            Timer::after_millis(100).await;
            continue;
        }
        if unit > base_line {
            watchdog.feed();
            base_line = unit;
            Timer::after_millis(100).await;
            continue;
        }
        if unit < base_line - 200.0 {
            log::info!("base_line has changed from {}mm to {}mm", base_line, unit);
            watchdog.feed();

            counter += 1;
            if counter > 10 {
                base_line = unit;
                counter = 0;
            }
            msg = buffer.format(unit).as_bytes();

            let mut failed_count = 0;
            loop {
                match client
                    .send_message(
                        "pico",
                        msg,
                        rust_mqtt::packet::v5::publish_packet::QualityOfService::QoS1,
                        true,
                    )
                    .await
                {
                    Err(e) => {
                        failed_count += 1;
                        if failed_count > 10 {
                            log::error!(
                                "failed to send message more than 10 retries. Restarting pico"
                            );
                            watchdog.trigger_reset();
                            break;
                        }
                        log::error!("failed to send message: {:?}", e)
                    }
                    _ => break,
                };
            }

            Timer::after_millis(100).await;
        }
    }
}

async fn wait_for_config(
    stack: &'static Stack<Device>,
    control: &mut cyw43::Control<'static>,
    w: &mut Watchdog,
) -> embassy_net::StaticConfigV4 {
    const MAX_RETRIES: usize = 20;
    let mut current: usize = 0;
    loop {
        w.feed();
        control.gpio_set(0, true).await;
        if let Some(config) = stack.config_v4() {
            break config.clone();
        }
        Timer::after_millis(500).await;
        control.gpio_set(0, false).await;
        if current >= MAX_RETRIES {
            log::error!("Failed to get IP address");
            w.trigger_reset();
        }
        current += 1;
    }
}
