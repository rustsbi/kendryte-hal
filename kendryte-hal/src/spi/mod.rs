mod register;
pub use register::*;

mod driver;
pub use driver::*;

pub mod pad;
pub use pad::{IntoPads, IntoSpiClk, IntoSpiCs, IntoSpiMiso, IntoSpiMosi, IntoTransmitOnly};
