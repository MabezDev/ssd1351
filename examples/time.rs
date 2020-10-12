//! Interfacing the on-board L3GD20 (gyroscope)
#![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate cortex_m;
extern crate embedded_graphics;
extern crate heapless;
extern crate panic_semihosting;
extern crate ssd1351;
extern crate stm32l4xx_hal as hal;

use core::fmt::Write;
use hal::datetime::{Date, Time};
use hal::delay::Delay;
use hal::prelude::*;
use hal::rtc::Rtc;
use hal::spi::Spi;
use hal::stm32l4::stm32l4x2;
use heapless::consts::*;
use heapless::String;
use rt::ExceptionFrame;
use ssd1351::builder::Builder;
use ssd1351::mode::GraphicsMode;
use ssd1351::prelude::*;

use embedded_graphics::fonts::{Font12x16, Font6x12, Text};
use embedded_graphics::pixelcolor::raw::RawU16;
use embedded_graphics::prelude::*;
use embedded_graphics::style::TextStyle;

/// SPI mode for

#[entry]
fn main() -> ! {
    let p = stm32l4x2::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();

    let clocks = rcc
        .cfgr
        .sysclk(80.mhz())
        .pclk1(80.mhz())
        .pclk2(80.mhz())
        .freeze(&mut flash.acr);

    let mut gpioa = p.GPIOA.split(&mut rcc.ahb2);
    let mut gpiob = p.GPIOB.split(&mut rcc.ahb2);

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = Delay::new(cp.SYST, clocks);

    let mut pwr = p.PWR.constrain(&mut rcc.apb1r1);
    let rtc = Rtc::rtc(p.RTC, &mut rcc.apb1r1, &mut rcc.bdcr, &mut pwr.cr1);

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
        SSD1351_SPI_MODE,
        4.mhz(),
        clocks,
        &mut rcc.apb2,
    );

    let mut time = Time::new(21.hours(), 57.minutes(), 32.seconds(), false);
    let mut date = Date::new(1.day(), 24.date(), 4.month(), 2018.year());

    rtc.set_time(&time);
    rtc.set_date(&date);

    let mut display: GraphicsMode<_> = Builder::new().connect_spi(spi, dc).into();
    display.reset(&mut rst, &mut delay);
    display.init().unwrap();

    // let mut buffer: [u8; 256] = [0; 256];
    let mut buffer: String<U16> = String::new();
    loop {
        time = rtc.get_time();
        date = rtc.get_date();
        {
            write!(
                buffer,
                "{:02}:{:02}:{:02}",
                time.hours, time.minutes, time.seconds
            )
            .unwrap();
            Text::new(buffer.as_str(), Point::new(10, 40))
                .into_styled(TextStyle::new(Font12x16, RawU16::new(0xF818_u16).into()))
                .draw(&mut display);
            buffer.clear(); // reset the buffer
            write!(
                buffer,
                "{:02}:{:02}:{:04}",
                date.date, date.month, date.year
            )
            .unwrap();
            Text::new(buffer.as_str(), Point::new(24, 60))
                .into_styled(TextStyle::new(Font6x12, RawU16::new(0x880B_u16).into()))
                .draw(&mut display);
            buffer.clear(); // reset the buffer
        }
    }
}
