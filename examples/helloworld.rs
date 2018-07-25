//! Interfacing the on-board L3GD20 (gyroscope)
#![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate cortex_m;
extern crate panic_semihosting;
extern crate embedded_hal as ehal;
extern crate stm32l432xx_hal as hal;
extern crate ssd1351;
extern crate embedded_graphics;

use cortex_m::asm;
use hal::prelude::*;
use hal::spi::Spi;
use hal::stm32l4::stm32l4x2;
use rt::ExceptionFrame;
use ehal::spi::{Mode, Phase, Polarity};
use ssd1351::builder::Builder;
use ssd1351::mode::{GraphicsMode};
use hal::delay::Delay;

use embedded_graphics::prelude::*;
// use embedded_graphics::fonts::Font6x8;
// use embedded_graphics::fonts::Font8x16;
use embedded_graphics::fonts::Font12x16;

/// SPI mode
pub const MODE: Mode = Mode {
    phase: Phase::CaptureOnFirstTransition,
    polarity: Polarity::IdleLow,
};

entry!(main);

fn main() -> ! {
    let p = stm32l4x2::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();

    // TRY the other clock configuration
    // let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let clocks = rcc.cfgr.sysclk(80.mhz()).pclk1(80.mhz()).pclk2(80.mhz()).freeze(&mut flash.acr);
    // let clocks = rcc.cfgr.sysclk(64.mhz()).pclk1(32.mhz()).freeze(&mut flash.acr);

    let mut gpioa = p.GPIOA.split(&mut rcc.ahb2);
    let mut gpiob = p.GPIOB.split(&mut rcc.ahb2);

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = Delay::new(cp.SYST, clocks);

    let mut rst = gpioa
        .pa8
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

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
        24.mhz(),
        // 100.khz(),
        clocks,
        &mut rcc.apb2,
    );

    // reset the display
    rst.set_high();
    delay.delay_ms(5_u16);
    rst.set_low();
    delay.delay_ms(100_u16);
    rst.set_high();
    delay.delay_ms(5_u16);
    
    let mut display: GraphicsMode<_> = Builder::new().connect_spi(spi, dc).into();
    display.init().unwrap();

    let mut i = 0;
    loop {
        display.draw(Font12x16::render_str("Wavey!", i).into_iter());
        // display.clear();
        delay.delay_ms(32_u16);
        i+=1;
        if i == 255 {
            i = 0;
        }
    }
    
    // display.draw(
    //     Font6x8::render_str("Hello Rust!", 0x84)
    //         .translate((0, 16))
    //         .into_iter(),
    // );
    
    asm::bkpt();

    loop {}
}

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
