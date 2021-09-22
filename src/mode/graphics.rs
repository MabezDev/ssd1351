use crate::display::Display;
use hal::blocking::delay::DelayMs;
use hal::digital::v2::OutputPin;
use crate::interface::DisplayInterface;

use crate::mode::displaymode::DisplayModeTrait;
use crate::properties::DisplayRotation;

/// Graphics Mode for the display
pub struct GraphicsMode<DI>
where
    DI: DisplayInterface,
{
    display: Display<DI>,
    #[cfg(feature = "buffered")]
    pub buffer: &'static mut [u8],
}

impl<DI> DisplayModeTrait<DI> for GraphicsMode<DI>
where
    DI: DisplayInterface,
{
    #[cfg(not(feature = "buffered"))]
    /// Create new GraphicsMode instance
    fn new(display: Display<DI>) -> Self {
        GraphicsMode { display }
    }

    #[cfg(feature = "buffered")]
    fn new(display: Display<DI>, buffer: &'static mut [u8]) -> Self {
        GraphicsMode { display, buffer }
    }

    #[cfg(not(feature = "buffered"))]
    /// Release all resources used by GraphicsMode
    fn release(self) -> Display<DI> {
        self.display
    }

    #[cfg(feature = "buffered")]
    /// Release all resources used by GraphicsMode
    fn release(self) -> (Display<DI>, &'static mut [u8]) {
        (self.display, self.buffer)
    }
}

// impl<DI: DisplayInterface> GraphicsMode<DI> {
//     /// Create a new grahpics display interface
//     pub fn new(display: Display<DI>) -> Self {
//         GraphicsMode { display }
//     }
// }

impl<DI> GraphicsMode<DI>
where
    DI: DisplayInterface,
{
    #[cfg(not(feature = "buffered"))]
    /// Clear the display
    pub fn clear(&mut self) {
        self.display.clear().unwrap();
    }

    #[cfg(feature = "buffered")]
    /// Clear the display
    pub fn clear(&mut self, flush: bool) {
        for i in 0..self.buffer.len() {
            self.buffer[i] = 0u8;
        }
        if flush {
            self.flush();
        }
    }

    /// Reset display
    pub fn reset<RST, DELAY>(&mut self, rst: &mut RST, delay: &mut DELAY) -> Result<(), RST::Error>
    where
        RST: OutputPin,
        DELAY: DelayMs<u8>,
    {
        rst.set_high()?;
        delay.delay_ms(1);
        rst.set_low()?;
        delay.delay_ms(10);
        rst.set_high()?;
        Ok(())
    }

    #[cfg(feature = "buffered")]
    /// Access the framebuffer
    pub fn fb(&self) -> &[u8] {
        self.buffer
    }

    #[cfg(not(feature = "buffered"))]
    /// Turn a pixel on or off. A non-zero `value` is treated as on, `0` as off. If the X and Y
    /// coordinates are out of the bounds of the display, this method call is a noop.
    pub fn set_pixel(&mut self, x: u32, y: u32, color: u16) {
        let (display_width, display_height) = self.display.get_size().dimensions();
        let rot = self.display.get_rotation();
        let (nx, ny) = match rot {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => (x, y),
            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => (y, x),
        };
        self.display
            .set_draw_area((nx as u8, ny as u8), (display_width, display_height))
            .unwrap();
        self.display
            .draw(&[(color >> 8) as u8, color as u8])
            .unwrap();
    }

    #[cfg(feature = "buffered")]
    /// Turn a pixel on or off. A non-zero `value` is treated as on, `0` as off. If the X and Y
    /// coordinates are out of the bounds of the display, this method call is a noop.
    pub fn set_pixel(&mut self, x: u32, y: u32, color: u16) {
        let rot = self.display.get_rotation();
        let (nx, ny) = match rot {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => (x, y),
            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => (y, x),
        };
        // set bytes in buffer
        self.buffer[(ny as usize * 128usize + nx as usize) * 2] = (color >> 8) as u8;
        self.buffer[((ny as usize * 128usize + nx as usize) * 2) + 1usize] = color as u8;
    }

    #[cfg(feature = "buffered")]
    pub fn flush(&mut self) {
        let (display_width, display_height) = self.display.get_size().dimensions();
        self.display
            .set_draw_area((0, 0), (display_width, display_height))
            .unwrap();
        self.display.draw(self.buffer).unwrap();
    }

    /// Display is set up in column mode, i.e. a byte walks down a column of 8 pixels from
    /// column 0 on the left, to column _n_ on the right
    pub fn init(&mut self) -> Result<(), ()> {
        self.display.init()?;
        Ok(())
    }

    /// Set the display rotation
    pub fn set_rotation(&mut self, rot: DisplayRotation) -> Result<(), ()> {
        self.display.set_rotation(rot)
    }

    /// Get display dimensions, taking into account the current rotation of the display
    pub fn get_dimensions(&self) -> (u8, u8) {
        self.display.get_dimensions()
    }
}

#[cfg(feature = "graphics")]
extern crate embedded_graphics_core;
#[cfg(feature = "graphics")]
use self::embedded_graphics_core::prelude::{RawData, Size, OriginDimensions, DrawTarget, Dimensions, Pixel};
#[cfg(feature = "graphics")]
use self::embedded_graphics_core::pixelcolor::Rgb565;
#[cfg(feature = "graphics")]
use self::embedded_graphics_core::pixelcolor::raw::RawU16;

#[cfg(feature = "graphics")]
impl<DI: DisplayInterface> DrawTarget for GraphicsMode<DI> {
    type Color = Rgb565;
    type Error = ();

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error> where I: IntoIterator<Item=Pixel<Self::Color>> {
        let bb = self.bounding_box();

        pixels
            .into_iter()
            .filter(|Pixel(pos, _)| bb.contains(*pos))
            .for_each(|Pixel(pos, color)| {
                self.set_pixel(pos.x as u32, pos.y as u32, RawU16::from(color).into_inner())
            });

        Ok(())
    }
}

impl<DI: DisplayInterface> OriginDimensions for GraphicsMode<DI>  {
    fn size(&self) -> Size {
        let dim = self.display.get_size().dimensions();
        Size::from((dim.0 as u32, dim.1 as u32))
    }
}
