use hal::spi::{Mode, Phase, Polarity};

pub const SSD1351_SPI_MODE: Mode = Mode {
    phase: Phase::CaptureOnFirstTransition,
    polarity: Polarity::IdleLow,
};

pub use display_interface_spi::SPIInterface;
