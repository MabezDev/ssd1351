//! Raw mode for coercion into richer driver types
//!
//! A display driver instance without high level functionality used as a return type from the
//! builder. Used as a source to coerce the driver into richer modes like
//! [`GraphicsMode`](../graphics/index.html) and [`TerminalMode`](../terminal/index.html).

use interface::DisplayInterface;
use display::Display;

use mode::displaymode::DisplayModeTrait;

/// Raw display mode
pub struct RawMode<DI>
where
    DI: DisplayInterface,
{
    pub display: Display<DI>,
    #[cfg(feature = "buffered")]
    buffer: [u8; 128 * 128 * 2],
}

impl<DI> DisplayModeTrait<DI> for RawMode<DI>
where
    DI: DisplayInterface,
{
    #[cfg(not(feature = "buffered"))]
    /// Create new RawMode instance
    fn new(display: Display<DI>) -> Self {
        RawMode { 
            display: display
        }
    }

    #[cfg(feature = "buffered")]
    /// Create new GraphicsMode instance
    fn new(display: Display<DI>, buffer: [u8; 128 * 128 * 2]) -> Self {
        RawMode { 
            display,
            buffer
        }
    }

    #[cfg(not(feature = "buffered"))]
    /// Release all resources used by RawMode
    fn release(self) -> Display<DI> {
        self.display
    }

    #[cfg(feature = "buffered")]
    /// Release all resources used by GraphicsMode
    fn release(self) -> (Display<DI>, [u8; 128 * 128 * 2]) {
        (self.display, self.buffer)
    }
}

impl<DI: DisplayInterface> RawMode<DI> {
    #[cfg(not(feature = "buffered"))]
    /// Create a new raw display mode
    pub fn new(display: Display<DI>) -> Self {
        RawMode { 
            display: display,
        }
    }

    #[cfg(feature = "buffered")]
    /// Create a new raw display mode
    pub fn new(display: Display<DI>, buffer: [u8; 128 * 128 * 2]) -> Self {
        RawMode { 
            display: display,
            buffer: buffer
        }
    }
}
