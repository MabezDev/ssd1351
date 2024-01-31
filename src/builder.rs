//! Interface factory

use super::display::Display;
use super::mode::displaymode::DisplayMode;
use super::mode::raw::RawMode;
use super::properties::DisplayRotation;
use super::properties::DisplaySize;

use display_interface::WriteOnlyDataCommand;

/// Builder struct. Driver options and interface are set using its methods.
#[derive(Clone)]
pub struct Builder {
    display_size: DisplaySize,
    rotation: DisplayRotation,
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

impl Builder {
    /// Create new builder with a default size of 128 x 128 pixels and no rotation.
    pub fn new() -> Self {
        Self {
            display_size: DisplaySize::Display128x128,
            rotation: DisplayRotation::Rotate0,
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

    #[cfg(feature = "buffered")]
    /// Finish the builder and use the given interface to communicate with the display
    pub fn connect_interface<DI>(
        &self,
        display_interface: DI,
        buffer: &'static mut [u8],
    ) -> DisplayMode<RawMode<DI>>
    where
        DI: WriteOnlyDataCommand,
    {
        assert_eq!(buffer.len(), 128 * 128 * 2);
        let properties = Display::new(display_interface, self.display_size, self.rotation);
        DisplayMode::<RawMode<DI>>::new(properties, buffer)
    }

    #[cfg(not(feature = "buffered"))]
    /// Finish the builder and use the given interface to communicate with the display
    pub fn connect_interface<DI>(&self, display_interface: DI) -> DisplayMode<RawMode<DI>>
    where
        DI: WriteOnlyDataCommand,
    {
        let properties = Display::new(display_interface, self.display_size, self.rotation);
        DisplayMode::<RawMode<DI>>::new(properties)
    }
}
