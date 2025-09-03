mod rx;
mod tx;

pub use rx::BlockingUartRx;
pub use tx::BlockingUartTx;

use super::pad::FlexPad;
use crate::clocks::Clocks;
use crate::instance::Numbered;
use crate::uart::MmioRegisterBlock;
use crate::uart::config::{Config, set_divisor, set_parity_mode, set_stop_bits, set_word_length};
use crate::uart::config::{disable_fifo, enable_fifo};
use crate::uart::error::UartError;
use crate::uart::pad::{IntoUartSin, IntoUartSout};
use core::marker::PhantomData;

/// Checks if the UART is ready to read data.
pub(crate) fn read_ready(uart: &MmioRegisterBlock) -> bool {
    uart.read_lsr().data_ready()
}

/// Checks if the UART is ready to write data.
pub(crate) fn write_ready(uart: &mut MmioRegisterBlock) -> bool {
    uart.read_lsr().transmitter_empty() || uart.read_lsr().transmitter_holding_empty()
}

/// Reads data from UART in a blocking manner.
///
/// This function attempts to read data from the UART into the provided buffer.
/// It will read as much data as possible until either the buffer is full or no more data is available.
/// Returns the number of bytes actually read.
pub(crate) fn blocking_read(uart: &MmioRegisterBlock, buf: &mut [u8]) -> usize {
    let mut count = 0_usize;
    for ch in buf {
        if read_ready(uart) {
            *ch = uart.read_rbr_thr_dll().receiver_buffer();
            count += 1;
        } else {
            break;
        }
    }
    count
}

/// Writes data to UART in a blocking manner.
///
/// This function attempts to write data from the provided buffer to the UART.
/// It will write as much data as possible until either all data is written or the FIFO becomes full.
/// Returns the number of bytes actually written.
pub(crate) fn blocking_write(uart: &mut MmioRegisterBlock, buf: &[u8]) -> usize {
    let mut count = 0_usize;
    for ch in buf {
        if write_ready(uart) {
            unsafe {
                uart.modify_rbr_thr_dll(|r| r.with_transmitter_holding(*ch));
            }
            count += 1;
        } else {
            break;
        }
    }
    count
}

/// Flushes the UART transmitter by waiting until all data has been sent.
///
/// This function blocks until the transmitter is completely empty.
pub(crate) fn blocking_flush(uart: &mut MmioRegisterBlock) {
    while !uart.read_lsr().transmitter_empty() {
        core::hint::spin_loop();
    }
}

/// A wrapper struct for UART that provides blocking operations.
///
/// This struct implements blocking read and write operations for UART communication.
pub struct BlockingUart<'i, 't, 'r> {
    inner: MmioRegisterBlock<'static>,
    tx: Option<BlockingUartTx<'i, 't>>,
    rx: Option<BlockingUartRx<'i, 'r>>,
    _marker: PhantomData<&'i ()>,
}

impl<'i, 't, 'r> BlockingUart<'i, 't, 'r> {
    /// Creates a new BlockingUart instance with the specified configuration.
    ///
    /// This function initializes the UART with the provided configuration parameters.
    /// Returns a new BlockingUart instance.
    pub fn new<const N: usize>(
        instance: impl Numbered<'i, N, R = MmioRegisterBlock<'static>>,
        tx: Option<impl IntoUartSout<'t, N>>,
        rx: Option<impl IntoUartSin<'r, N>>,
        config: Config,
        clocks: Clocks,
    ) -> Self {
        let mut inner = instance.inner();
        Self::configure::<N>(&mut inner, config, clocks);

        let mut blocking_uart_tx = None;
        let mut blocking_uart_rx = None;

        if let Some(tx) = tx {
            let tx = tx.into_uart_sout();
            blocking_uart_tx = Some(BlockingUartTx {
                inner: unsafe { inner.clone() },
                tx,
                _marker: PhantomData,
            });
        }

        if let Some(rx) = rx {
            let rx = rx.into_uart_sin();
            blocking_uart_rx = Some(BlockingUartRx {
                inner: unsafe { inner.clone() },
                rx,
                _marker: PhantomData,
            })
        }

        BlockingUart {
            inner: unsafe { inner.clone() },
            tx: blocking_uart_tx,
            rx: blocking_uart_rx,
            _marker: PhantomData,
        }
    }

    /// Configures the UART peripheral with the specified settings.
    /// Disables all UART interrupts first.
    /// Sets the baud rate, parity, stop bits, word length, and FIFO mode.
    fn configure<const N: usize>(
        uart: &mut MmioRegisterBlock<'static>,
        config: Config,
        clocks: Clocks,
    ) {
        unsafe {
            uart.modify_ier_dlh(|r| {
                r.with_modem_status_interrupt_enable(false)
                    .with_transmit_empty_interrupt_enable(false)
                    .with_receive_data_available_interrupt_enable(false)
                    .with_receive_line_status_interrupt_enable(false)
                    .with_programmable_threshold_interrupt_enable(false)
            });
        }

        let divisor = clocks.uart_sclk::<N>().0 / (16_u32 * config.baud.0);
        set_divisor(uart, divisor as u16);
        set_parity_mode(uart, config.parity_mode);
        set_stop_bits(uart, config.stop_bits);
        set_word_length(uart, config.word_length);

        match config.fifo {
            true => enable_fifo(uart),
            false => disable_fifo(uart),
        }
    }

    /// Splits the BlockingUart into separate transmitter and receiver handles.
    /// Returns ownership of the transmitter and receiver, if available.
    pub fn split(
        self,
    ) -> (
        Option<BlockingUartTx<'i, 't>>,
        Option<BlockingUartRx<'i, 'r>>,
    ) {
        (self.tx, self.rx)
    }
}

impl<'i, 't, 'r> embedded_io::ErrorType for BlockingUart<'i, 't, 'r> {
    type Error = UartError;
}

impl<'i, 't, 'r> embedded_io::Read for BlockingUart<'i, 't, 'r> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.rx.as_mut().ok_or(UartError::NotFoundRx)?.read(buf)
    }
}

impl<'i, 't, 'r> embedded_io::Write for BlockingUart<'i, 't, 'r> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.tx.as_mut().ok_or(UartError::NotFoundRx)?.write(buf)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.tx.as_mut().ok_or(UartError::NotFoundRx)?.flush()
    }
    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        self.tx
            .as_mut()
            .ok_or(UartError::NotFoundRx)?
            .write_all(buf)
    }
}

impl<'i, 't, 'r> embedded_io::ReadReady for BlockingUart<'i, 't, 'r> {
    fn read_ready(&mut self) -> Result<bool, Self::Error> {
        self.rx.as_mut().ok_or(UartError::NotFoundRx)?.read_ready()
    }
}

impl<'i, 't, 'r> embedded_io::WriteReady for BlockingUart<'i, 't, 'r> {
    fn write_ready(&mut self) -> Result<bool, Self::Error> {
        self.tx.as_mut().ok_or(UartError::NotFoundRx)?.write_ready()
    }
}

impl<'i, 't, 'r> embedded_hal_nb::serial::ErrorType for BlockingUart<'i, 't, 'r> {
    type Error = UartError;
}

impl<'i, 't, 'r> embedded_hal_nb::serial::Read for BlockingUart<'i, 't, 'r> {
    fn read(&mut self) -> embedded_hal_nb::nb::Result<u8, Self::Error> {
        self.rx.as_mut().ok_or(UartError::NotFoundRx)?.read()
    }
}

impl<'i, 't, 'r> embedded_hal_nb::serial::Write for BlockingUart<'i, 't, 'r> {
    fn write(&mut self, word: u8) -> embedded_hal_nb::nb::Result<(), Self::Error> {
        self.tx.as_mut().ok_or(UartError::NotFoundRx)?.write(word)
    }

    fn flush(&mut self) -> embedded_hal_nb::nb::Result<(), Self::Error> {
        self.tx.as_mut().ok_or(UartError::NotFoundRx)?.flush()
    }
}
