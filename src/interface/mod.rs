
pub mod spi;

pub trait DisplayInterface {
    fn send_commands(&mut self, cmd: &[u8]) -> Result<(), ()>;
    fn send_data(&mut self, cmd: &[u8]) -> Result<(), ()>;
}

pub use self::spi::SpiInterface;