#![no_std]
#![no_main]

pub mod hc_sr04;

use core::str::{self, FromStr};
use core::{env, option_env};
use cyw43_pio::PioSpi;
use defmt::unwrap;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_net::{Config, Stack, StackResources};
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIN_23, PIN_25, PIO0, USB};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::usb::{Driver, InterruptHandler as USBInterruptHandler};
use embassy_rp::watchdog::Watchdog;
use embassy_time::{Duration, Instant, Timer};
use hc_sr04::HCSR04;
use static_cell::StaticCell;

// #[cfg(debug_assertions)]
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
const SERVER_IP: &'static str = env!("RP_SERVER_IP");
const SERVER_PORT: &'static str = match option_env!("RP_SERVER_PORT") {
    Some(port) => port,
    _ => "1883",
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
    log::info!("DHCP is now up!");
    let local_addr = cfg.address.address();
    log::info!("got address: {}", local_addr);

    watchdog.feed();
    // go into power save mode
    // log::info!("setting power management mode: power save");
    // control
    //     .set_power_management(cyw43::PowerManagementMode::PowerSave)
    //     .await;

    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];
    let server_port: u16 = SERVER_PORT.parse().unwrap();
    let host_addr = embassy_net::Ipv4Address::from_str(SERVER_IP).unwrap();

    // get sensor data and send to server
    let mut base_line: f64 = 8.0;
    let mut counter: i8 = 0;
    let mut buffer = ryu::Buffer::new();
    let mut socket = embassy_net::tcp::TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
    socket.set_timeout(Some(Duration::from_secs(10)));

    loop {
        watchdog.feed();

        blink_led(
            &mut control,
            &mut watchdog,
            Duration::from_millis(500),
            Duration::from_millis(500),
            1,
        )
        .await;

        let instant = Instant::now();

        let unit = match ultrasonic.measure().await {
            Ok(unit) => unit.millimeters,
            Err(_) => -1.0,
        };
        watchdog.feed();

        log::info!("distance: {}mm", unit);
        log::info!("time: {}ms", instant.elapsed().as_millis() as f64);

        if unit == -1.0 {
            log::error!("Failed to measure distance");
            Timer::after_millis(500).await;
            continue;
        }
        if unit > base_line {
            watchdog.feed();
            base_line = unit;
            Timer::after_millis(500).await;
            continue;
        }
        if unit < base_line - 100.0 {
            log::info!("base_line has changed from {}mm to {}mm", base_line, unit);
            watchdog.feed();

            for _ in 0..50 {
                watchdog.feed();
                log::info!("Connecting...");
                if let Err(e) = socket.connect((host_addr, server_port)).await {
                    log::warn!("connect error: {:?}", e);
                    Timer::after_millis(200).await;
                    continue;
                }
                log::info!("Connected to {:?}", socket.remote_endpoint());
                break;
            }

            if socket.remote_endpoint() == None {
                log::error!("Failed to connect to remote server");
                watchdog.trigger_reset();
                return;
            }
            watchdog.feed();
            counter += 1;
            if counter > 10 {
                base_line = unit;
                counter = 0;
            }
            let msg = buffer.format(base_line);
            if let Err(e) = socket.write(msg.as_bytes()).await {
                log::warn!("write error: {:?}", e);
            }
            socket.close();
            Timer::after_millis(500).await;
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
