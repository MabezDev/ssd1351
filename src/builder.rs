//! Interface factory

use hal;
use hal::digital::OutputPin;

use super::properties::DisplayRotation;
use super::properties::DisplaySize;
use super::interface::{SpiInterface};
use super::display::Display;
use super::mode::displaymode::DisplayMode;
use super::mode::raw::RawMode;

/// Builder struct. Driver options and interface are set using its methods.
#[derive(Clone, Copy)]
pub struct Builder {
    display_size: DisplaySize,
    rotation: DisplayRotation,
    #[cfg(feature = "buffered")]
    buffer: [u8; 128 * 128 * 2],
}

// impl Default for Builder {
//     fn default() -> Self {
//         #[cfg(not(feature = "buffered"))]
//         Self::new()
//         #[cfg(feature = "buffered")]
//         Self::new()
//     }
// }

impl Builder {
    #[cfg(not(feature = "buffered"))]
    /// Create new builder with a default size of 128 x 128 pixels and no rotation.
    pub fn new() -> Self {
        Self {
            display_size: DisplaySize::Display128x128,
            rotation: DisplayRotation::Rotate0
        }
    }

    #[cfg(feature = "buffered")]
    /// Create new builder with a default size of 128 x 128 pixels and no rotation.
    pub fn new(buffer: [u8; 128 * 128 * 2]) -> Self {
        Self {
            display_size: DisplaySize::Display128x128,
            rotation: DisplayRotation::Rotate0,
            buffer: buffer
        }
    }

    /// Set the size of the display. Supported sizes are defined by [DisplaySize].
    pub fn with_size(&self, display_size: DisplaySize) -> Self {
        Self {
            display_size,
            ..*self
        }
    }

    /// Set the rotation of the display to one of four values. Defaults to no rotation. Note that
    /// 90º and 270º rotations are not supported by
    /// [`TerminalMode`](../mode/terminal/struct.TerminalMode.html).
    pub fn with_rotation(&self, rotation: DisplayRotation) -> Self {
        Self { rotation, ..*self }
    }

    /// Finish the builder and use SPI to communicate with the display
    pub fn connect_spi<SPI, DC>(
        &self,
        spi: SPI,
        dc: DC,
    ) -> DisplayMode<RawMode<SpiInterface<SPI, DC>>>
    where
        SPI: hal::blocking::spi::Transfer<u8> + hal::blocking::spi::Write<u8>,
        DC: OutputPin,
    {
        let properties =
            Display::new(SpiInterface::new(spi, dc), self.display_size, self.rotation);
        #[cfg(not(feature = "buffered"))]
        {
            return DisplayMode::<RawMode<SpiInterface<SPI, DC>>>::new(properties);
        }
        #[cfg(feature = "buffered")]
        {
            return DisplayMode::<RawMode<SpiInterface<SPI, DC>>>::new(properties, self.buffer);
        }
    }

}
