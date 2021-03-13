#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate embedded_graphics;
extern crate embedded_hal as ehal;
extern crate panic_semihosting;
extern crate ssd1351;
extern crate stm32l4xx_hal as hal;

use ehal::spi::{Mode, Phase, Polarity};
use hal::delay::Delay;
use hal::prelude::*;
use hal::spi::Spi;
use hal::stm32l4::stm32l4x2;
use cortex_m_rt::{entry, ExceptionFrame};
use ssd1351::builder::Builder;
use ssd1351::mode::GraphicsMode;

use embedded_graphics::pixelcolor::raw::RawU16;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line};
use embedded_graphics::style::PrimitiveStyle;

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

    let mut display: GraphicsMode<_> = Builder::new().connect_spi(spi, dc).into();
    display.reset(&mut rst, &mut delay).unwrap();
    display.init().unwrap();

    Line::new(Point::new(0, 0), Point::new(74, 74))
        .into_styled(PrimitiveStyle::with_stroke(
            RawU16::new(0x0D85_u16).into(),
            1,
        ))
        .draw(&mut display)
        .unwrap();
    Circle::new(Point::new(64, 64), 8)
        .into_styled(PrimitiveStyle::with_stroke(
            RawU16::new(0xF1FA_u16).into(),
            1,
        ))
        .draw(&mut display)
        .unwrap();

    loop {}
}
