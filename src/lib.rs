#![no_std]
#![allow(clippy::result_unit_err)]

extern crate embedded_hal as hal;

pub mod builder;
pub mod command;
pub mod display;
pub mod interface;
pub mod mode;
pub mod prelude;
pub mod properties;
