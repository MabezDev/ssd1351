//! Abstraction of different operating modes for the SSD1351

use interface::DisplayInterface;
use display::Display;

/// Display mode abstraction
pub struct DisplayMode<MODE>(pub MODE);

/// Trait with core functionality for display mode switching
pub trait DisplayModeTrait<DI> {
    /// Allocate all required data and initialise display for mode
    fn new(properties: Display<DI>) -> Self;

    /// Release resources for reuse with different mode
    fn release(self) -> Display<DI>;
}

impl<MODE> DisplayMode<MODE> {
    /// Setup display to run in requested mode
    pub fn new<DI>(properties: Display<DI>) -> Self
    where
        DI: DisplayInterface,
        MODE: DisplayModeTrait<DI>,
    {
        DisplayMode(MODE::new(properties))
    }

    /// Change into any mode implementing DisplayModeTrait
    // TODO: Figure out how to stay as generic DisplayMode but act as particular mode
    pub fn into<DI, NMODE: DisplayModeTrait<DI>>(self) -> NMODE
    where
        DI: DisplayInterface,
        MODE: DisplayModeTrait<DI>,
    {
        let properties = self.0.release();
        NMODE::new(properties)
    }
}
