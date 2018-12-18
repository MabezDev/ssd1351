//! Abstraction of different operating modes for the SSD1351

use interface::DisplayInterface;
use display::Display;

/// Display mode abstraction
pub struct DisplayMode<MODE>{
    pub mode: MODE
}

/// Trait with core functionality for display mode switching
pub trait DisplayModeTrait<DI> {
    #[cfg(not(feature = "buffered"))]
    /// Allocate all required data and initialise display for mode
    fn new(properties: Display<DI>) -> Self;

    #[cfg(feature = "buffered")]
    /// Create a display object using a pre-allocated frame buffer
    fn new(display: Display<DI>, buffer: [u8; 128 * 128 * 2]) -> Self;

    #[cfg(not(feature = "buffered"))]
    /// Release resources for reuse with different mode
    fn release(self) -> Display<DI>;
    #[cfg(feature = "buffered")]
    /// Release resources for reuse with different mode
    fn release(self) -> (Display<DI>, [u8; 128 * 128 * 2]);
}

impl<MODE> DisplayMode<MODE> {
    #[cfg(not(feature = "buffered"))]
    /// Setup display to run in requested mode
    pub fn new<DI>(properties: Display<DI>) -> Self
    where
        DI: DisplayInterface,
        MODE: DisplayModeTrait<DI>,
    {
        DisplayMode {
            mode: MODE::new(properties),
        }
    }

    #[cfg(feature = "buffered")]
    /// Create new GraphicsMode instance
    pub fn new<DI>(display: Display<DI>, buffer: [u8; 128 * 128 * 2]) -> Self 
    where
        DI: DisplayInterface,
        MODE: DisplayModeTrait<DI>,
    {
        DisplayMode { 
            mode: MODE::new(display, buffer)
        }
    }

    /// Change into any mode implementing DisplayModeTrait
    // TODO: Figure out how to stay as generic DisplayMode but act as particular mode
    pub fn into<DI, NMODE: DisplayModeTrait<DI>>(self) -> NMODE
    where
        DI: DisplayInterface,
        MODE: DisplayModeTrait<DI>,
    {
        #[cfg(not(feature = "buffered"))]
        {
            let properties = self.mode.release();
            return NMODE::new(properties);
        }
        #[cfg(feature = "buffered")]
        {
            let properties = self.mode.release();
            return NMODE::new(properties.0, properties.1);
        }
    }
}
