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

use cortex_m::asm;
use hal::prelude::*;
use hal::spi::Spi;
use hal::stm32l4::stm32l4x2;
use rt::ExceptionFrame;
use ehal::spi::{FullDuplex, Mode, Phase, Polarity};
use ssd1351::builder::Builder;
use ssd1351::mode::RawMode;
use hal::delay::Delay;

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
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    // let clocks = rcc.cfgr.sysclk(64.mhz()).pclk1(32.mhz()).freeze(&mut flash.acr);

    let mut gpioa = p.GPIOA.split(&mut rcc.ahb2);
    let mut gpiob = p.GPIOB.split(&mut rcc.ahb2);

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = Delay::new(cp.SYST, clocks);

    let mut rst = gpioa
        .pa8
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    let mut dc = gpiob
        .pb1
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let sck = gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let miso = gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let mosi = gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl);

    let spi = Spi::spi1(
        p.SPI1,
        (sck, miso, mosi),
        MODE,
        // 1.mhz(),
        100.khz(),
        clocks,
        &mut rcc.apb2,
    );


    // let sck = gpiob.pb3.into_af5(&mut gpiob.moder, &mut gpiob.afrl);
    // let miso = gpiob.pb4.into_af5(&mut gpiob.moder, &mut gpiob.afrl);
    // let mosi = gpiob.pb5.into_af5(&mut gpiob.moder, &mut gpiob.afrl);

    // let spi = Spi::spi3(
    //     p.SPI3,
    //     (sck, miso, mosi),
    //     MODE,
    //     // 1.mhz(),
    //     100.khz(),
    //     clocks,
    //     &mut rcc.apb1r1,
    // );

    // nss.set_low(); // only one device, always select

    // reset the display
    rst.set_high();
    delay.delay_ms(500_u16);
    rst.set_low();
    delay.delay_ms(500_u16);
    rst.set_high();
    delay.delay_ms(500_u16);
    
    // TODO
    let mut display: RawMode<_> = Builder::new().connect_spi(spi, dc).into();
    display.display.init();

    let colour = 0xFFFF; // white
    let buffer = [(colour >> 8) as u8, colour as u8];
    let dimensions = display.display.get_size();
    let i = 0;
    // display.display.set_draw_area((i, i),(128, 128)).unwrap();
    display.display.draw(&buffer).unwrap();
    // for i in 0..128 {
        
    // }
    

    // when you reach this breakpoint you'll be able to inspect the variable `_m` which contains the
    // gyroscope and the temperature sensor readings
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
