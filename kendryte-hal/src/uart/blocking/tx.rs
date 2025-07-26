use crate::iomux::FlexPad;
use crate::uart::blocking::{blocking_flush, blocking_write, write_ready};
use crate::uart::{RegisterBlock, UartError};
use core::marker::PhantomData;

/// A UART transmitter for blocking operations.
/// This struct implements blocking write operations for UART communication.
pub struct BlockingUartTx<'i, 't> {
    /// Holds a reference to the UART register block.
    pub(crate) inner: &'static RegisterBlock,
    /// Contains a mutable handle to the TX pad.
    pub(crate) tx: FlexPad<'t>,
    /// Uses PhantomData for lifetime tracking.
    pub(crate) _marker: PhantomData<&'i ()>,
}

impl<'i, 't> embedded_io::ErrorType for BlockingUartTx<'i, 't> {
    type Error = UartError;
}

impl<'i, 't> embedded_io::Write for BlockingUartTx<'i, 't> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        Ok(blocking_write(&self.inner, buf))
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        blocking_flush(&self.inner);
        Ok(())
    }
    fn write_all(&mut self, mut buf: &[u8]) -> Result<(), Self::Error> {
        while !buf.is_empty() {
            match self.write(buf) {
                Ok(n) => buf = &buf[n..],
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

impl<'i, 't> embedded_hal_nb::serial::ErrorType for BlockingUartTx<'i, 't> {
    type Error = UartError;
}

impl<'i, 't> embedded_hal_nb::serial::Write for BlockingUartTx<'i, 't> {
    fn write(&mut self, word: u8) -> embedded_hal_nb::nb::Result<(), Self::Error> {
        let len = blocking_write(&self.inner, &[word]);
        match len {
            0 => Err(embedded_hal_nb::nb::Error::WouldBlock),
            _ => Ok(()),
        }
    }

    fn flush(&mut self) -> embedded_hal_nb::nb::Result<(), Self::Error> {
        match self.inner.lsr.read().transmitter_empty() {
            true => Ok(()),
            false => Err(embedded_hal_nb::nb::Error::WouldBlock),
        }
    }
}

impl<'i, 't> embedded_io::WriteReady for BlockingUartTx<'i, 't> {
    fn write_ready(&mut self) -> Result<bool, Self::Error> {
        Ok(write_ready(&self.inner))
    }
}
