#![no_std]
#![no_main]

use core::cell::RefCell;

use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_executor::Spawner;
use embassy_rp::spi::Spi;
use embassy_rp::{gpio, spi};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use embassy_time::Delay;
use embedded_graphics::geometry::{Point, Size};
use embedded_graphics::image::Image;
use embedded_graphics::pixelcolor::{Rgb565, RgbColor};
use embedded_graphics::primitives::{Primitive, PrimitiveStyleBuilder, Rectangle};
use embedded_graphics::Drawable;
use gpio::{Level, Output};
use ssd1351::builder::Builder;
use ssd1351::mode::GraphicsMode;
use ssd1351::prelude::*;
use ssd1351::properties::{DisplayRotation, DisplaySize};
use tinybmp::Bmp;
use {defmt_rtt as _, panic_probe as _};

// static mut BUF: [u8; 32768] = [0; 32768];

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut config = spi::Config::default();
    config.frequency = 4_000_000;

    // 128x128 Display
    let clk = p.PIN_2;
    let mosi = p.PIN_3;
    let cs = Output::new(p.PIN_5, Level::Low);
    let dc = Output::new(p.PIN_6, Level::Low);
    let mut rst = Output::new(p.PIN_7, Level::Low);
    let spi = Spi::new_blocking_txonly(p.SPI0, clk, mosi, config.clone());
    let spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi));
    let spi_device = SpiDeviceWithConfig::new(&spi_bus, cs, config.clone());

    let interface = SPIInterface::new(spi_device, dc);
    let mut display_128: GraphicsMode<_> = Builder::new()
        .with_rotation(DisplayRotation::Rotate0)
        .connect_interface(
            interface, // , unsafe { &mut BUF}
        )
        .into();

    display_128.reset(&mut rst, &mut Delay).unwrap();
    display_128.init().unwrap();

    // 128x96 Display
    let clk = p.PIN_14;
    let mosi = p.PIN_15;
    let cs = Output::new(p.PIN_13, Level::Low);
    let dc = Output::new(p.PIN_16, Level::Low);
    let mut rst = Output::new(p.PIN_17, Level::Low);
    let spi = Spi::new_blocking_txonly(p.SPI1, clk, mosi, config.clone());
    let spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi));
    let spi_device = SpiDeviceWithConfig::new(&spi_bus, cs, config);

    let interface = SPIInterface::new(spi_device, dc);
    let mut display_96: GraphicsMode<_> = Builder::new()
        .with_rotation(DisplayRotation::Rotate0)
        .with_size(DisplaySize::Display128x96)
        .connect_interface(interface)
        .into();

    display_96.reset(&mut rst, &mut Delay).unwrap();
    display_96.init().unwrap();


    let image_data = include_bytes!("../128.bmp");
    let bmp = Bmp::<Rgb565>::from_slice(image_data).unwrap();
    let image = Image::new(&bmp, Point::new(0, 0));
    image.draw(&mut display_128).unwrap();

    let image_data = include_bytes!("../96.bmp");
    let bmp = Bmp::<Rgb565>::from_slice(image_data).unwrap();
    let image = Image::new(&bmp, Point::new(0, 0));
    image.draw(&mut display_96).unwrap();

    let rect_r = Rectangle::new(Point::new(0, 0), Size::new(40, 20)).into_styled(
        PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::RED)
            .build(),
    );

    let rect_g = Rectangle::new(Point::new(0, 20), Size::new(40, 20)).into_styled(
        PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::GREEN)
            .build(),
    );
    let rect_b = Rectangle::new(Point::new(0, 40), Size::new(40, 20)).into_styled(
        PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::BLUE)
            .build(),
    );

    rect_r.draw(&mut display_128).unwrap();
    rect_r.draw(&mut display_96).unwrap();
    rect_g.draw(&mut display_128).unwrap();
    rect_g.draw(&mut display_96).unwrap();
    rect_b.draw(&mut display_128).unwrap();
    rect_b.draw(&mut display_96).unwrap();

    loop {}
}
