//! Display attributes

/// Display rotation.
///
/// Note that 90º and 270º rotations are not supported by
// [`TerminalMode`](../mode/terminal/struct.TerminalMode.html).
#[derive(Clone, Copy)]
pub enum DisplayRotation {
    /// No rotation, normal display
    Rotate0,
    /// Rotate by 90 degress clockwise
    Rotate90,
    /// Rotate by 180 degress clockwise
    Rotate180,
    /// Rotate 270 degress clockwise
    Rotate270,
}

/// Display size enumeration
#[derive(Clone, Copy)]
pub enum DisplaySize {
    /// 128 by 128 pixels
    Display128x128,
    /// 128 by 96 pixels
    Display128x96,
}

impl DisplaySize {
    /// Get integral dimensions from DisplaySize
    // TODO: Use whatever vec2 impl I decide to use here
    pub fn dimensions(&self) -> (u8, u8) {
        match *self {
            DisplaySize::Display128x128 => (128, 128),
            DisplaySize::Display128x96 => (128, 96),
        }
    }
}

