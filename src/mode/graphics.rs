use interface::DisplayInterface;
use display::Display;
use hal::blocking::delay::DelayMs;
use hal::digital::OutputPin;

use mode::displaymode::DisplayModeTrait;

/// Raw display mode
pub struct GraphicsMode<DI>
where
    DI: DisplayInterface,
{
    display: Display<DI>,
}

impl<DI> DisplayModeTrait<DI> for GraphicsMode<DI>
where
    DI: DisplayInterface,
{
    /// Create new GraphicsMode instance
    fn new(display: Display<DI>) -> Self {
        GraphicsMode { display }
    }

    /// Release all resources used by GraphicsMode
    fn release(self) -> Display<DI> {
        self.display
    }
}

impl<DI: DisplayInterface> GraphicsMode<DI> {
    /// Create a new raw display mode
    pub fn new(display: Display<DI>) -> Self {
        GraphicsMode { display }
    }
}

impl<DI> GraphicsMode<DI>
where
    DI: DisplayInterface,
{
    /// Clear the display buffer. You need to call `disp.flush()` for any effect on the screen
    pub fn clear(&mut self) {
        self.display.clear().unwrap();
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
        let (display_width, display_height) = self.display.get_size().dimensions();
        //TODO rotation
        // let display_rotation = self.display.get_rotation();
        self.display.set_draw_area((y as u8, x as u8), (display_width, display_height)).unwrap();
        self.display.draw(&[(color >> 8) as u8, color as u8]).unwrap();
    }

    /// Display is set up in column mode, i.e. a byte walks down a column of 8 pixels from
    /// column 0 on the left, to column _n_ on the right
    pub fn init(&mut self) -> Result<(), ()> {
        self.display.init()?;
        Ok(())
    }

    /// Set the display rotation
    // pub fn set_rotation(&mut self, rot: DisplayRotation) -> Result<(), ()> {
    //     self.display.set_rotation(rot)
    // }

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
        for drawable::Pixel(UnsignedCoord(x, y), color) in item_pixels {
            self.set_pixel(x, y, color.into_inner());
        }
    }
}