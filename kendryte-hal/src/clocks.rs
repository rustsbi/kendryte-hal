use embedded_time::rate::{Extensions, Hertz};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Clocks;

impl Clocks {
    pub fn uart_sclk<const N: usize>(&self) -> Hertz {
        assert!(N <= 4, "N must be less than or equal to 4");
        50_000_000.Hz()
    }
}
