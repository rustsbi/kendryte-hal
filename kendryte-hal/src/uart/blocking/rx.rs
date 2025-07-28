use crate::iomux::FlexPad;
use crate::uart::blocking::{blocking_read, read_ready};
use crate::uart::{RegisterBlock, UartError};
use core::marker::PhantomData;

/// A UART receiver for blocking operations.
/// This struct implements blocking read operations for UART communication.
pub struct BlockingUartRx<'i, 'r> {
    /// Holds a reference to the UART register block.
    pub(crate) inner: &'static RegisterBlock,
    /// Contains a mutable handle to the RX pad.
    pub(crate) rx: FlexPad<'r>,
    /// Uses PhantomData for lifetime tracking.
    pub(crate) _marker: PhantomData<&'i ()>,
}

impl<'i, 'r> embedded_io::ErrorType for BlockingUartRx<'i, 'r> {
    type Error = UartError;
}

impl<'i, 'r> embedded_io::Read for BlockingUartRx<'i, 'r> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let mut count = 0_usize;
        for ch in buf {
            if self.inner.lsr.read().data_ready() {
                *ch = self.inner.rbr_thr_dll.read().receiver_buffer();
                count += 1;
            } else {
                break;
            }
        }
        Ok(count)
    }
}

impl<'i, 'r> embedded_hal_nb::serial::ErrorType for BlockingUartRx<'i, 'r> {
    type Error = UartError;
}

impl<'i, 'r> embedded_hal_nb::serial::Read for BlockingUartRx<'i, 'r> {
    fn read(&mut self) -> embedded_hal_nb::nb::Result<u8, Self::Error> {
        let mut buf = [0];
        let len = blocking_read(&self.inner, &mut buf);
        match len {
            0 => Err(embedded_hal_nb::nb::Error::WouldBlock),
            _ => Ok(buf[0]),
        }
    }
}

impl<'i, 'r> embedded_io::ReadReady for BlockingUartRx<'i, 'r> {
    fn read_ready(&mut self) -> Result<bool, Self::Error> {
        Ok(read_ready(&self.inner))
    }
}
