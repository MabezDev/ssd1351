use interface::DisplayInterface;
use display::Display;
use hal::blocking::delay::DelayMs;
use hal::digital::OutputPin;

use mode::displaymode::DisplayModeTrait;
use properties::DisplayRotation;
#[cfg(feature = "buffered")]
use properties::DisplaySize;

/// Graphics Mode for the display
pub struct GraphicsMode<DI>
where
    DI: DisplayInterface,
{
    display: Display<DI>,
    #[cfg(feature = "buffered")]
    buffer: [u8; 128 * 128 * 2]
}

impl<DI> DisplayModeTrait<DI> for GraphicsMode<DI>
where
    DI: DisplayInterface,
{
    #[cfg(not(feature = "buffered"))]
    /// Create new GraphicsMode instance
    fn new(display: Display<DI>) -> Self {
        GraphicsMode { 
            display
        }
    }

    #[cfg(feature = "buffered")]
    /// Create new GraphicsMode instance
    fn new(display: Display<DI>, buffer: [u8; 128 * 128 * 2]) -> Self {
        GraphicsMode { 
            display,
            buffer
        }
    }

    #[cfg(not(feature = "buffered"))]
    /// Release all resources used by GraphicsMode
    fn release(self) -> Display<DI> {
        self.display
    }

    #[cfg(feature = "buffered")]
    /// Release all resources used by GraphicsMode
    fn release(self) -> (Display<DI>, [u8; 128 * 128 * 2]) {
        (self.display, self.buffer)
    }
}

impl<DI: DisplayInterface> GraphicsMode<DI> {
    #[cfg(not(feature = "buffered"))]
    /// Create a new grahpics display interface
    pub fn new(display: Display<DI>) -> Self {
        GraphicsMode { 
            display
        }
    }

    #[cfg(feature = "buffered")]
    /// Create a new grahpics display interface
    pub fn new(display: Display<DI>, buffer: [u8; 128 * 128 * 2]) -> Self {
        GraphicsMode { 
            display,
            buffer
        }
    }
}

impl<DI> GraphicsMode<DI>
where
    DI: DisplayInterface,
{
    /// Clear the display
    pub fn clear(&mut self) {
        #[cfg(feature = "buffered")]
        {
            self.buffer = [0; 128 * 128 * 2];
        }
        #[cfg(not(feature = "buffered"))]
        {
            self.display.clear().unwrap();
        }
    }

    /// Reset display
    pub fn reset<RST, DELAY>(&mut self, rst: &mut RST, delay: &mut DELAY)
    where
        RST: OutputPin,
        DELAY: DelayMs<u8>,
    {
        rst.set_high();
        delay.delay_ms(1);
        rst.set_low();
        delay.delay_ms(10);
        rst.set_high();
    }

    /// Turn a pixel on or off. A non-zero `value` is treated as on, `0` as off. If the X and Y
    /// coordinates are out of the bounds of the display, this method call is a noop.
    pub fn set_pixel(&mut self, x: u32, y: u32, color: u16) {
        let rot = self.display.get_rotation();
        let (nx, ny) = match rot {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => (x, y),
            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => (y, x),
        };
        let color_buffer = [(color >> 8) as u8, color as u8];

        #[cfg(feature = "buffered")]
        {
            let (_n_, display_height) = self.display.get_size().dimensions();
            self.buffer[(((ny * display_height as u32) + nx) * 2) as usize] = color_buffer[1];
            self.buffer[(((ny * display_height as u32) + nx) * 2) as usize + 1usize] = color_buffer[0];
        }
        #[cfg(not(feature = "buffered"))]
        {
            let (display_width, display_height) = self.display.get_size().dimensions();
            self.display.set_draw_area((nx as u8, ny as u8), (display_width, display_height)).unwrap();
            self.display.draw(&color_buffer).unwrap();
        }
    }

    #[cfg(feature = "buffered")]
    /// Write out data to display
    pub fn flush(&mut self) -> Result<(), ()> {
        let display_size = self.display.get_size();

        // Ensure the display buffer is at the origin of the display before we send the full frame
        // to prevent accidental offsets
        let (display_width, display_height) = display_size.dimensions();
        self.display
            .set_draw_area((0, 0), (display_width, display_height))?;

        match display_size {
            DisplaySize::Display128x128 => self.display.draw(&self.buffer),
            DisplaySize::Display128x64 => self.display.draw(&self.buffer[0..16384]), 
        }
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
extern crate embedded_graphics;
#[cfg(feature = "graphics")]
use self::embedded_graphics::drawable;
#[cfg(feature = "graphics")]
use self::embedded_graphics::Drawing;
#[cfg(feature = "graphics")]
use self::embedded_graphics::pixelcolor::PixelColorU16;
#[cfg(feature = "graphics")]
use self::embedded_graphics::unsignedcoord::UnsignedCoord;

#[cfg(feature = "graphics")]
impl<DI> Drawing<PixelColorU16> for GraphicsMode<DI> 
    where
    DI: DisplayInterface,
{
    fn draw<T>(&mut self, item_pixels: T)
    where
        T: Iterator<Item = drawable::Pixel<PixelColorU16>>,
    {
        let (width, height) = self.display.get_size().dimensions();
        for drawable::Pixel(UnsignedCoord(x, y), color) in item_pixels {
            if x <= width.into() && y <= height.into() {
                self.set_pixel(x, y, color.into_inner());
            }
        }
    }
}