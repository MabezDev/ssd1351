//! Raw mode for coercion into richer driver types
//!
//! A display driver instance without high level functionality used as a return type from the
//! builder. Used as a source to coerce the driver into richer modes like
//! [`GraphicsMode`](../graphics/index.html) and [`TerminalMode`](../terminal/index.html).

use crate::display::Display;
use crate::interface::DisplayInterface;

use crate::mode::displaymode::DisplayModeTrait;

/// Raw display mode
pub struct RawMode<DI>
where
    DI: DisplayInterface,
{
    pub display: Display<DI>,
    #[cfg(feature = "buffered")]
    pub buffer: &'static mut [u8],
}

impl<DI> DisplayModeTrait<DI> for RawMode<DI>
where
    DI: DisplayInterface,
{
    /// Create new RawMode instance
    #[cfg(not(feature = "buffered"))]
    fn new(display: Display<DI>) -> Self {
        RawMode { display }
    }

    #[cfg(feature = "buffered")]
    fn new(display: Display<DI>, buffer: &'static mut [u8]) -> Self {
        RawMode { display, buffer }
    }

    #[cfg(not(feature = "buffered"))]
    /// Release all resources used by RawMode
    fn release(self) -> Display<DI> {
        self.display
    }

    #[cfg(feature = "buffered")]
    /// Release all resources used by RawMode
    fn release(self) -> (Display<DI>, &'static mut [u8]) {
        (self.display, self.buffer)
    }
}

// impl<DI: DisplayInterface> RawMode<DI> {
//     /// Create a new raw display mode
//     pub fn new(display: Display<DI>) -> Self {
//         RawMode { display }
//     }
// }
