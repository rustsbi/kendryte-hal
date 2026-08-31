use core::fmt;

/// Indicate different error conditions that may occur during UART communication.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UartError {
    /// Framing error occurred.
    Framing,
    /// Parity error occurred.
    Parity,
    /// Overrun error occurred.
    Overrun,
    /// Transmit (TX) resource not found.
    NotFoundTx,
    /// Receive (RX) resource not found.
    NotFoundRx,
}

impl fmt::Display for UartError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Framing => write!(f, "UART framing error"),
            Self::Parity => write!(f, "UART parity error"),
            Self::Overrun => write!(f, "UART receive buffer overrun"),
            Self::NotFoundTx => write!(f, "UART transmit resource not found"),
            Self::NotFoundRx => write!(f, "UART receive resource not found"),
        }
    }
}

impl core::error::Error for UartError {}

impl embedded_io::Error for UartError {
    fn kind(&self) -> embedded_io::ErrorKind {
        embedded_io::ErrorKind::Other
    }
}

impl embedded_hal_nb::serial::Error for UartError {
    fn kind(&self) -> embedded_hal_nb::serial::ErrorKind {
        embedded_hal_nb::serial::ErrorKind::Other
    }
}
