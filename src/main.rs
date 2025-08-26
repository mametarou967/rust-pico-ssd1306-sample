#![no_std]
#![no_main]

use fugit::RateExtU32;
use cortex_m_rt::entry;
use panic_halt as _;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use rp_pico::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
    gpio::FunctionI2C,
    i2c::I2C,
    prelude::*,
};

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = Sio::new(pac.SIO);

    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // I2C1を GPIO14(SDA), GPIO15(SCL) に設定
    // let sda_pin = pins.gpio14.into_mode::<FunctionI2C>();
    // let scl_pin = pins.gpio15.into_mode::<FunctionI2C>();

    let i2c = I2C::i2c1(
        pac.I2C1,
        pins.gpio14.reconfigure(),
        pins.gpio15.reconfigure(),
	400.kHz(),
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
    );

    let interface = I2CDisplayInterface::new(i2c);

    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.init().unwrap();
    display.clear(BinaryColor::Off);

    let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    Text::new("RasPico", Point::new(0, 16), text_style)
        .draw(&mut display)
        .unwrap();
    Text::new("SSD1306", Point::new(0, 32), text_style)
        .draw(&mut display)
        .unwrap();
    Text::new("hello", Point::new(0, 48), text_style)
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    loop {
        cortex_m::asm::wfi();
    }
}
