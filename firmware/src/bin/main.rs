#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use embedded_graphics::mono_font::ascii::FONT_6X13_BOLD;
use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use panic_rtt_target as _;
use rtt_target::rprintln;

use embedded_graphics::Drawable;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::{Dimensions, Point, Primitive};
use embedded_graphics::primitives::PrimitiveStyle;
use embedded_graphics::text::Text;
use esp_hal::analog::adc::{Adc, AdcConfig, Attenuation};
use esp_hal::delay::Delay;
use esp_hal::gpio::{DriveMode, Flex, InputConfig, OutputConfig, Pull};
use esp_hal::i2c::master::I2c;
use esp_hal::{i2c::master::Config, time::Rate};
use ssd1306::{I2CDisplayInterface, Ssd1306, prelude::*};
// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    // generator version: 1.3.0
    // generator parameters: --chip esp32c3 -o unstable-hal -o probe-rs -o panic-rtt-target -o vscode

    rtt_target::rtt_init_print!();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut delay = Delay::new();

    let i2c_config = Config::default().with_frequency(Rate::from_khz(100));

    let i2c = I2c::new(peripherals.I2C0, i2c_config)
        .unwrap()
        .with_sda(peripherals.GPIO8)
        .with_scl(peripherals.GPIO9);

    let mut adc1_config = AdcConfig::new();
    let mut pin = adc1_config.enable_pin(peripherals.GPIO0, Attenuation::_11dB);
    let mut adc1 = Adc::new(peripherals.ADC1, adc1_config);

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let style = MonoTextStyle::new(&FONT_6X13_BOLD, BinaryColor::On);

    Text::new("test", Point::new(0, 20), style)
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    rprintln!("Hello world!");
    loop {
        let pin_value: u16 = nb::block!(adc1.read_oneshot(&mut pin)).unwrap();
        rprintln!("ADC: {}", pin_value);

        delay.delay_millis(1500);
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.1.0/examples
}
