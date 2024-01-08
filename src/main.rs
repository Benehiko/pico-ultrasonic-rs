#![no_std]
#![no_main]

pub mod hc_sr04;

use core::str::{self, FromStr};
use core::{env, option_env};
use cyw43::Control;
use cyw43_pio::PioSpi;
use defmt::*;
use embassy_executor::Spawner;
use embassy_futures::yield_now;
use embassy_net::{Config, Stack, StackResources};
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIN_23, PIN_25, PIO0, USB};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::usb::{Driver, InterruptHandler as USBInterruptHandler};
use embassy_time::{Duration, Timer};
use hc_sr04::HCSR04;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

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
) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn net_task(stack: &'static Stack<Device>) -> ! {
    stack.run().await
}

#[embassy_executor::task]
async fn hc_sr04_task(mut hc_sr04: HCSR04, stack: &'static Stack<Device>) {
    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];
    let server_port: u16 = SERVER_PORT.parse().unwrap();

    loop {
        let mut socket = embassy_net::tcp::TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
        socket.set_timeout(Some(Duration::from_secs(10)));

        log::info!("Connecting...");
        let host_addr = embassy_net::Ipv4Address::from_str(SERVER_IP).unwrap();
        if let Err(e) = socket.connect((host_addr, server_port)).await {
            log::warn!("connect error: {:?}", e);
            continue;
        }
        log::info!("Connected to {:?}", socket.remote_endpoint());

        match hc_sr04.measure().await {
            Ok(unit) => {
                let mut buffer = ryu::Buffer::new();
                let msg = buffer.format(unit.meters);
                if let Err(e) = socket.write(msg.as_bytes()).await {
                    log::warn!("write error: {:?}", e);
                }
            }
            Err(e) => {
                log::error!("Error: {}", e);
            }
        };
        Timer::after(Duration::from_secs(1)).await;
    }
}

#[embassy_executor::task]
async fn alive_task(stack: &'static Stack<Device>) {
    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];

    let server_port: u16 = SERVER_PORT.parse().unwrap();

    loop {
        let mut socket = embassy_net::tcp::TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
        socket.set_timeout(Some(Duration::from_secs(10)));

        log::info!("Connecting...");
        let host_addr = embassy_net::Ipv4Address::from_str(SERVER_IP).unwrap();
        if let Err(e) = socket.connect((host_addr, server_port)).await {
            log::warn!("connect error: {:?}", e);
            continue;
        }
        log::info!("Connected to {:?}", socket.remote_endpoint());

        let msg = b"Hello world!\n";
        loop {
            if let Err(e) = socket.write(msg).await {
                log::warn!("write error: {:?}", e);
                break;
            }
            log::info!("txd: {}", core::str::from_utf8(msg).unwrap());
            Timer::after_secs(1).await;
        }
    }
}

#[embassy_executor::task]
async fn blink_task(mut control: Control<'static>) {
    let delay = Duration::from_secs(1);
    loop {
        control.gpio_set(0, true).await;
        Timer::after(delay).await;
        control.gpio_set(0, false).await;
        Timer::after(delay).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let fw = include_bytes!("../../embassy/cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../../embassy/cyw43-firmware/43439A0_clm.bin");

    // Initialize driver.
    let ultrasonic = HCSR04::new(p.PIN_2, p.PIN_3).unwrap();

    let usb = p.USB;
    let driver = Driver::new(usb, Irqs);
    spawner.spawn(logger_task(driver)).unwrap();
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
    unwrap!(spawner.spawn(wifi_task(runner)));

    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
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

    unwrap!(spawner.spawn(net_task(&stack)));

    let delay = Duration::from_secs(1);

    loop {
        match control.join_wpa2(WIFI_NETWORK, WIFI_PASSWORD).await {
            Ok(_) => {
                log::info!("connected!");
                break;
            }
            Err(e) => {
                log::error!("join failed: {}", e.status);
                Timer::after(delay).await;
            }
        }
    }

    // Wait for DHCP, not necessary when using static IP
    log::info!("waiting for DHCP...");
    let cfg = wait_for_config(stack).await;
    log::info!("DHCP is now up!");
    let local_addr = cfg.address.address();
    log::info!("got address: {}", local_addr);

    unwrap!(spawner.spawn(hc_sr04_task(ultrasonic, &stack)));

    // let mut scanner = control.scan().await;
    // while let Some(bss) = scanner.next().await {
    //     if let Ok(ssid_str) = str::from_utf8(&bss.ssid) {
    //         log::info!("scanned {}", ssid_str);
    //     }
    // }

    // loop {
    //     info!("led on!");
    //     control.gpio_set(0, true).await;
    //     Timer::after(delay).await;
    //
    //     info!("led off!");
    //     control.gpio_set(0, false).await;
    //     Timer::after(delay).await;
    // }
}

async fn wait_for_config(stack: &'static Stack<Device>) -> embassy_net::StaticConfigV4 {
    loop {
        if let Some(config) = stack.config_v4() {
            return config.clone();
        }
        yield_now().await;
    }
}
