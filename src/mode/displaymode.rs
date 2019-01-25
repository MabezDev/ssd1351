//! Abstraction of different operating modes for the SSD1351

use interface::DisplayInterface;
use display::Display;

/// Display display abstraction
pub struct DisplayMode<MODE>{
    pub display: MODE
}

/// Trait with core functionality for display display switching
pub trait DisplayModeTrait<DI> {
    /// Allocate all required data and initialise display for display
    #[cfg(not(feature = "buffered"))]
    fn new(display: Display<DI>) -> Self;

    #[cfg(feature = "buffered")]
    fn new(display: Display<DI>, &'static mut [u8]) -> Self;

    /// Release resources for reuse with different display
    #[cfg(not(feature = "buffered"))]
    fn release(self) -> Display<DI>;

    #[cfg(feature = "buffered")]
    fn release(self) -> (Display<DI>, &'static mut [u8]);
}

impl<MODE> DisplayMode<MODE> {
    /// Setup display to run in requested display
    #[cfg(not(feature = "buffered"))]
    pub fn new<DI>(display: Display<DI>) -> Self
    where
        DI: DisplayInterface,
        MODE: DisplayModeTrait<DI>,
    {
        DisplayMode{
            display: MODE::new(display)
        }
    }

    #[cfg(feature = "buffered")]
    pub fn new<DI>(display: Display<DI>, buffer: &'static mut [u8]) -> Self
    where
        DI: DisplayInterface,
        MODE: DisplayModeTrait<DI>,
    {
        DisplayMode {
            display: MODE::new(display, buffer)
        }
    }

    /// Change into any display implementing DisplayModeTrait
    // TODO: Figure out how to stay as generic DisplayMode but act as particular display
    #[cfg(not(feature = "buffered"))]
    pub fn into<DI, NMODE: DisplayModeTrait<DI>>(self) -> NMODE
    where
        DI: DisplayInterface,
        MODE: DisplayModeTrait<DI>,
    {
        let display = self.display.release();
        NMODE::new(display)
    }

    #[cfg(feature = "buffered")]
    pub fn into<DI, NMODE: DisplayModeTrait<DI>>(self) -> NMODE
    where
        DI: DisplayInterface,
        MODE: DisplayModeTrait<DI>,
    {
        let (display, buffer) = self.display.release();
        NMODE::new(display, buffer)
    }
}
