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
use esp_hal::peripherals::ADC1;
use heapless::String;
use panic_rtt_target as _;
use profont::{PROFONT_14_POINT, PROFONT_18_POINT, PROFONT_24_POINT};
use rtt_target::rprintln;
use temp_calc::{Action, State, Symbol};

use core::fmt::Write;
use embedded_graphics::Drawable;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::{Dimensions, Point, Primitive};
use embedded_graphics::primitives::PrimitiveStyle;
use embedded_graphics::text::Text;
use esp_hal::analog::adc::{Adc, AdcCalBasic, AdcConfig, Attenuation};
use esp_hal::delay::Delay;
use esp_hal::gpio::{Input, InputConfig, Pull};
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

    let button = Input::new(
        peripherals.GPIO21,
        InputConfig::default().with_pull(Pull::Up),
    );

    let delay = Delay::new();

    let i2c_config = Config::default().with_frequency(Rate::from_khz(400));

    let i2c = I2c::new(peripherals.I2C0, i2c_config)
        .unwrap()
        .with_sda(peripherals.GPIO8)
        .with_scl(peripherals.GPIO9);

    let mut adc1_config = AdcConfig::new();
    let mut pot1 = adc1_config
        .enable_pin_with_cal::<_, AdcCalBasic<ADC1>>(peripherals.GPIO0, Attenuation::_0dB);
    let mut pot2 = adc1_config
        .enable_pin_with_cal::<_, AdcCalBasic<ADC1>>(peripherals.GPIO1, Attenuation::_0dB);
    let mut adc1 = Adc::new(peripherals.ADC1, adc1_config);

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let style = MonoTextStyle::new(&PROFONT_14_POINT, BinaryColor::On);

    display.flush().unwrap();
    const POT_DIV_DIGIT: u16 = 4000 / 10;
    const POT_DIV_ACTION: u16 = 4000 / 8;

    let mut buf: String<32> = String::new();
    let mut buf_op: String<2> = String::new();
    let mut state = State::init();

    let mut was_pressed = false;
    loop {
        let pot1_val: u16 = nb::block!(adc1.read_oneshot(&mut pot1)).unwrap();
        let pot2_val: u16 = nb::block!(adc1.read_oneshot(&mut pot2)).unwrap();

        let digit = pot1_val / POT_DIV_DIGIT;
        rprintln!("adc1 : {} digit: {}", pot1_val, digit);

        let action = match pot2_val / POT_DIV_ACTION {
            0 => Action::Calculate,
            1 => Action::Insert(Symbol::Addition),
            2 => Action::Insert(Symbol::Subtraction),
            3 => Action::Insert(Symbol::Multiplication),
            4 => Action::Insert(Symbol::Division),
            5 => Action::Insert(Symbol::Number(digit as i32)),
            6 => Action::Delete,
            _ => Action::AllClear,
        };
        rprintln!("adc2: {}, action: {}", pot2_val, action);

        write!(buf_op, "{}", action).unwrap();
        let action_text = Text::new(
            &buf_op,
            Point::new(108, 24),
            MonoTextStyle::new(&PROFONT_24_POINT, BinaryColor::On),
        );
        action_text.draw(&mut display).unwrap();

        let number_line_text = &state.get_calculation_as_string().clone();

        let number_line = Text::new(number_line_text, Point::new(0, 10), style);
        number_line.draw(&mut display).unwrap();
        if let Some(result) = state.get_last_result() {
            write!(buf, "{}", result).unwrap();
        }
        let result_field = Text::new(
            &buf,
            Point::new(0, 60),
            MonoTextStyle::new(&PROFONT_18_POINT, BinaryColor::On),
        );
        result_field.draw(&mut display).unwrap();
        display.flush().unwrap();

        let pressed = button.is_low();

        if pressed && !was_pressed {
            state.action(action);
        }

        was_pressed = pressed;

        delay.delay_millis(20);
        number_line
            .bounding_box()
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off))
            .draw(&mut display)
            .unwrap();
        action_text
            .bounding_box()
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off))
            .draw(&mut display)
            .unwrap();
        result_field
            .bounding_box()
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off))
            .draw(&mut display)
            .unwrap();
        buf.clear();
        buf_op.clear();
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.1.0/examples
}
