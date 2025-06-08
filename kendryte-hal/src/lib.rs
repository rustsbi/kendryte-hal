//! SoC peripheral support for Cannan Kendryte chips.
#![no_std]
#![allow(unused)]
pub mod clocks;
pub mod gpio;
pub mod i2c;
pub mod instance;
pub mod iomux;
pub mod lsadc;
pub mod pad;
pub mod pwm;
pub mod spi;
pub mod uart;
