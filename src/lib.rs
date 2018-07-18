#![no_std]

extern crate embedded_hal as hal;

pub mod command;
pub mod interface;
pub mod builder;
mod display;
mod properties;
mod mode;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
