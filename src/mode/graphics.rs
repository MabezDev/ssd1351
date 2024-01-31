use crate::display::Display;
use crate::interface::DisplayInterface;
use hal::delay::DelayNs;
use hal::digital::OutputPin;

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
        DELAY: DelayNs,
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

    #[cfg(feature = "buffered")]
    /// Access the framebuffer
    pub fn fb_mut(&mut self) -> &mut [u8] {
        &mut self.buffer
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
        // set bytes in buffer
        self.buffer[(y as usize * 128usize + x as usize) * 2] = (color >> 8) as u8;
        self.buffer[((y as usize * 128usize + x as usize) * 2) + 1usize] = color as u8;
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
use self::embedded_graphics_core::pixelcolor::raw::RawU16;
#[cfg(feature = "graphics")]
use self::embedded_graphics_core::pixelcolor::Rgb565;
#[cfg(feature = "graphics")]
use self::embedded_graphics_core::prelude::{
    Dimensions, DrawTarget, OriginDimensions, Pixel, PointsIter, RawData, Size,
};
#[cfg(feature = "graphics")]
use self::embedded_graphics_core::primitives::Rectangle;

#[cfg(feature = "graphics")]
impl<DI: DisplayInterface> DrawTarget for GraphicsMode<DI> {
    type Color = Rgb565;
    type Error = ();

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let bb = self.bounding_box();

        pixels
            .into_iter()
            .filter(|Pixel(pos, _)| bb.contains(*pos))
            .for_each(|Pixel(pos, color)| {
                self.set_pixel(pos.x as u32, pos.y as u32, RawU16::from(color).into_inner())
            });

        Ok(())
    }

    #[cfg(not(feature = "buffered"))]
    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        let drawable_area = area.intersection(&self.bounding_box());

        let rot = self.display.get_rotation();
        let sx = drawable_area.top_left.x as u8;
        let sy = drawable_area.top_left.y as u8;
        let ex = (drawable_area.top_left.x as u32 + drawable_area.size.width) as u8;
        let ey = (drawable_area.top_left.y as u32 + drawable_area.size.height) as u8;

        // Set the draw area to the size of the rectangle
        let (area_start, area_end) = match rot {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => ((sx, sy), (ex, ey)),
            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => ((sy, sx), (ey, ex)),
        };

        self.display.set_draw_area(area_start, area_end).unwrap();

        // Get an iterator of colours as u16
        // Check points for containment
        area.points()
            .zip(colors)
            .filter(|(pos, _)| drawable_area.contains(*pos))
            .map(|(_, color)| RawU16::from(color).into_inner())
            .for_each(|color| {
                self.display
                    .draw(&[(color >> 8) as u8, color as u8])
                    .unwrap()
            });

        Ok(())
    }
}

impl<DI: DisplayInterface> OriginDimensions for GraphicsMode<DI> {
    fn size(&self) -> Size {
        let dim = self.display.get_size().dimensions();
        Size::from((dim.0 as u32, dim.1 as u32))
    }
}
