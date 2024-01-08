use core::fmt::Error;

pub type Result<T> = core::result::Result<T, Error>;

// This is pretty much a rewrite of
// https://github.com/marcoradocchia/hc-sr04/tree/master
// for embedded systems, especially the Raspberry Pi Pico
// We avoid using the std library and instead use the
// embassy crate for async programming
use embassy_rp::{
    gpio::{Input, Level, Output, Pull},
    peripherals::{PIN_2, PIN_3},
};
use embassy_time::{Duration, Instant, Timer};

pub struct HCSR04 {
    trigger: Output<'static, PIN_2>,
    echo: Input<'static, PIN_3>,
}

pub struct Unit {
    pub millimeters: f64,
    pub centimeters: f64,
    pub decimeters: f64,
    pub meters: f64,
}

impl HCSR04 {
    pub fn new(trigger_pin: PIN_2, echo_pin: PIN_3) -> Result<Self> {
        let mut trigger = Output::new(trigger_pin, Level::Low);
        let echo = Input::new(echo_pin, Pull::None);
        trigger.set_low();

        Ok(Self { trigger, echo })
    }
    fn calculate_speed(&mut self, duration: Duration) -> Unit {
        // 343 m/s
        // 0.0343 cm/microsecond
        const SPEED_OF_SOUND: f64 = 0.0343;

        // divide by 2 since the signal travels
        // to the object and back
        let distance = (SPEED_OF_SOUND * (duration.as_micros() as f64)) / 2f64;

        return Unit {
            millimeters: distance * 10.0,
            centimeters: distance,
            decimeters: distance / 10.0,
            meters: distance / 100.0,
        };
    }
    pub async fn measure(&mut self) -> Result<Unit> {
        self.trigger.set_high();
        Timer::after(Duration::from_micros(10)).await;
        self.trigger.set_low();

        self.echo.wait_for_high().await;
        let instant = Instant::now();
        self.echo.wait_for_low().await;

        return Ok(self.calculate_speed(instant.elapsed()));
    }
}
