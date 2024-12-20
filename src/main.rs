#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    analog::adc::{Adc, AdcConfig, Attenuation},
    delay::Delay,
    gpio::Io,
    prelude::*,
};

#[entry]
fn main() -> ! {
    #[allow(unused)]
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let analog_pin = io.pins.gpio2;

    let mut adc2_config = AdcConfig::new();
    let mut pin = adc2_config.enable_pin(analog_pin, Attenuation::Attenuation11dB);

    let mut adc1 = Adc::new(peripherals.ADC2, adc2_config);

    esp_println::logger::init_logger_from_env();

    let delay = Delay::new();

    loop {
        delay.delay_millis(1000);

        let pin_value: u16 = nb::block!(adc1.read_oneshot(&mut pin)).unwrap();

        let voltage = pin_value as f32 * 3.3 / 4095.0;

        esp_println::println!("ADC value: {}, Voltage: {:.2} V", pin_value, voltage);
    }
}
