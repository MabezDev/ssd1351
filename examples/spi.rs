//! Interfacing the on-board L3GD20 (gyroscope)
#![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate cortex_m;
extern crate embedded_hal as ehal;
extern crate panic_semihosting;
extern crate ssd1351;
extern crate stm32l4xx_hal as hal;

use cortex_m::asm;
use ehal::spi::{Mode, Phase, Polarity};
use hal::delay::Delay;
use hal::prelude::*;
use hal::spi::Spi;
use hal::stm32l4::stm32l4x2;
use rt::ExceptionFrame;
use ssd1351::builder::Builder;
use ssd1351::mode::RawMode;
use ssd1351::properties::DisplayRotation;

/// SPI mode
pub const MODE: Mode = Mode {
    phase: Phase::CaptureOnFirstTransition,
    polarity: Polarity::IdleLow,
};

#[entry]
fn main() -> ! {
    let p = stm32l4x2::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();

    // TRY the other clock configuration
    // let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let clocks = rcc
        .cfgr
        .sysclk(80.mhz())
        .pclk1(80.mhz())
        .pclk2(80.mhz())
        .freeze(&mut flash.acr);
    // let clocks = rcc.cfgr.sysclk(64.mhz()).pclk1(32.mhz()).freeze(&mut flash.acr);

    let mut gpioa = p.GPIOA.split(&mut rcc.ahb2);
    let mut gpiob = p.GPIOB.split(&mut rcc.ahb2);

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = Delay::new(cp.SYST, clocks);

    let mut rst = gpiob
        .pb0
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let dc = gpiob
        .pb1
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let sck = gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let miso = gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let mosi = gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl);

    let spi = Spi::spi1(
        p.SPI1,
        (sck, miso, mosi),
        MODE,
        1.mhz(),
        // 100.khz(),
        clocks,
        &mut rcc.apb2,
    );

    // reset the display
    rst.set_high().unwrap();
    delay.delay_ms(5_u16);
    rst.set_low().unwrap();
    delay.delay_ms(100_u16);
    rst.set_high().unwrap();
    delay.delay_ms(5_u16);

    let mut display: RawMode<_> = Builder::new().connect_spi(spi, dc).into();
    display.display.init().unwrap();

    let colour = 0xD90C; // 16 bit colour of choice
    let buffer = [(colour >> 8) as u8, colour as u8];
    let dimensions = display.display.get_size();
    let mut i = 0;
    display
        .display
        .set_rotation(DisplayRotation::Rotate270)
        .unwrap();
    display.display.set_draw_area((0, 0), (128, 128)).unwrap();
    display.display.draw(&buffer).unwrap();
    display.display.set_draw_area((64, 64), (128, 128)).unwrap();
    display.display.draw(&buffer).unwrap();

    // for _ in 0..128 { // draw a line
    //     display.display.draw(&buffer).unwrap();
    // }

    asm::bkpt();

    loop {}
}
