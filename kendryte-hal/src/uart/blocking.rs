use crate::clocks::Clocks;
use crate::instance::ExclusiveInstance;
use crate::pad::function::uart::{UartSinFunction, UartSoutFunction};
use crate::pad::{FlexPad, Pad};
use crate::uart::config::{Config, set_divisor, set_parity_mode, set_stop_bits, set_word_length};
use crate::uart::config::{disable_fifo, enable_fifo};
use crate::uart::error::UartError;
use crate::uart::{Instance, RegisterBlock};
use core::marker::PhantomData;

/// Checks if the UART is ready to read data.
pub(crate) fn read_ready(uart: &RegisterBlock) -> bool {
    uart.lsr.read().data_ready()
}

/// Checks if the UART is ready to write data.
pub(crate) fn write_ready(uart: &RegisterBlock) -> bool {
    uart.lsr.read().transmitter_empty() || uart.lsr.read().transmitter_holding_empty()
}

/// Reads data from UART in a blocking manner.
///
/// This function attempts to read data from the UART into the provided buffer.
/// It will read as much data as possible until either the buffer is full or no more data is available.
/// Returns the number of bytes actually read.
pub(crate) fn blocking_read(uart: &RegisterBlock, buf: &mut [u8]) -> usize {
    let mut count = 0_usize;
    for ch in buf {
        if read_ready(uart) {
            *ch = uart.rbr_thr_dll.read().receiver_buffer();
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
pub(crate) fn blocking_write(uart: &RegisterBlock, buf: &[u8]) -> usize {
    let mut count = 0_usize;
    for ch in buf {
        if write_ready(uart) {
            unsafe {
                uart.rbr_thr_dll.modify(|r| r.with_transmitter_holding(*ch));
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
pub(crate) fn blocking_flush(uart: &RegisterBlock) {
    while !uart.lsr.read().transmitter_empty() {
        core::hint::spin_loop();
    }
}

/// A wrapper struct for UART that provides blocking operations.
///
/// This struct implements blocking read and write operations for UART communication.
pub struct BlockingUart<'tx, 'rx, 'd> {
    inner: &'static RegisterBlock,
    tx: Option<BlockingUartTx<'tx, 'd>>,
    rx: Option<BlockingUartRx<'rx, 'd>>,
    _marker: PhantomData<&'d ()>,
}

impl<'tx, 'rx, 'd> BlockingUart<'tx, 'rx, 'd> {
    /// Creates a new BlockingUart instance with the specified configuration.
    ///
    /// This function initializes the UART with the provided configuration parameters.
    /// Returns a new BlockingUart instance.
    pub fn new<const UART_NUM: usize, const TX_PAD_NUM: usize, const RX_PAD_NUM: usize>(
        instance: &'d mut Instance<UART_NUM>,
        tx: Option<&'tx mut Pad<TX_PAD_NUM>>,
        rx: Option<&'rx mut Pad<RX_PAD_NUM>>,
        config: Config,
        clocks: Clocks,
    ) -> Self
    where
        Pad<TX_PAD_NUM>: UartSoutFunction<UART_NUM>,
        Pad<RX_PAD_NUM>: UartSinFunction<UART_NUM>,
    {
        Self::configure::<UART_NUM>(instance.inner(), config, clocks);

        let mut blocking_uart_tx = None;
        let mut blocking_uart_rx = None;

        if let Some(tx) = tx {
            tx.set_uart_sout_function();
            blocking_uart_tx = Some(BlockingUartTx {
                inner: instance.inner(),
                tx: tx.as_flexible_mut(),
                _marker: PhantomData,
            });
        }

        if let Some(rx) = rx {
            rx.set_uart_sin_function();
            blocking_uart_rx = Some(BlockingUartRx {
                inner: instance.inner(),
                rx: rx.as_flexible_mut(),
                _marker: PhantomData,
            })
        }

        BlockingUart {
            inner: instance.inner(),
            tx: blocking_uart_tx,
            rx: blocking_uart_rx,
            _marker: PhantomData,
        }
    }

    /// Configures the UART peripheral with the specified settings.
    /// Disables all UART interrupts first.
    /// Sets the baud rate, parity, stop bits, word length, and FIFO mode.
    fn configure<const UART_NUM: usize>(
        uart: &'static RegisterBlock,
        config: Config,
        clocks: Clocks,
    ) {
        unsafe {
            uart.ier_dlh.modify(|r| {
                r.with_modem_status_interrupt_enable(false)
                    .with_transmit_empty_interrupt_enable(false)
                    .with_receive_data_available_interrupt_enable(false)
                    .with_receive_line_status_interrupt_enable(false)
                    .with_programmable_threshold_interrupt_enable(false)
            });
        }

        let divisor = clocks.uart_sclk::<UART_NUM>().0 / (16_u32 * config.baud.0);
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
        Option<BlockingUartTx<'tx, 'd>>,
        Option<BlockingUartRx<'rx, 'd>>,
    ) {
        (self.tx, self.rx)
    }
}

impl<'tx, 'rx, 'd> embedded_io::ErrorType for BlockingUart<'tx, 'rx, 'd> {
    type Error = UartError;
}

impl<'tx, 'rx, 'd> embedded_io::Read for BlockingUart<'tx, 'rx, 'd> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.rx.as_mut().ok_or(UartError::NotFoundRx)?.read(buf)
    }
}

impl<'tx, 'rx, 'd> embedded_io::Write for BlockingUart<'tx, 'rx, 'd> {
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

impl<'tx, 'rx, 'd> embedded_io::ReadReady for BlockingUart<'tx, 'rx, 'd> {
    fn read_ready(&mut self) -> Result<bool, Self::Error> {
        self.rx.as_mut().ok_or(UartError::NotFoundRx)?.read_ready()
    }
}

impl<'tx, 'rx, 'd> embedded_io::WriteReady for BlockingUart<'tx, 'rx, 'd> {
    fn write_ready(&mut self) -> Result<bool, Self::Error> {
        self.tx.as_mut().ok_or(UartError::NotFoundRx)?.write_ready()
    }
}

impl<'tx, 'rx, 'd> embedded_hal_nb::serial::ErrorType for BlockingUart<'tx, 'rx, 'd> {
    type Error = UartError;
}

impl<'tx, 'rx, 'd> embedded_hal_nb::serial::Read for BlockingUart<'tx, 'rx, 'd> {
    fn read(&mut self) -> embedded_hal_nb::nb::Result<u8, Self::Error> {
        self.rx.as_mut().ok_or(UartError::NotFoundRx)?.read()
    }
}

impl<'tx, 'rx, 'd> embedded_hal_nb::serial::Write for BlockingUart<'tx, 'rx, 'd> {
    fn write(&mut self, word: u8) -> embedded_hal_nb::nb::Result<(), Self::Error> {
        self.tx.as_mut().ok_or(UartError::NotFoundRx)?.write(word)
    }

    fn flush(&mut self) -> embedded_hal_nb::nb::Result<(), Self::Error> {
        self.tx.as_mut().ok_or(UartError::NotFoundRx)?.flush()
    }
}

/// A UART transmitter for blocking operations.
/// This struct implements blocking write operations for UART communication.
pub struct BlockingUartTx<'tx, 'd> {
    /// Holds a reference to the UART register block.
    inner: &'static RegisterBlock,
    /// Contains a mutable handle to the TX pad.
    tx: &'tx mut FlexPad,
    /// Uses PhantomData for lifetime tracking.
    _marker: PhantomData<&'d ()>,
}

impl<'tx, 'd> embedded_io::ErrorType for BlockingUartTx<'tx, 'd> {
    type Error = UartError;
}

impl<'tx, 'd> embedded_io::Write for BlockingUartTx<'tx, 'd> {
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

impl<'tx, 'd> embedded_hal_nb::serial::ErrorType for BlockingUartTx<'tx, 'd> {
    type Error = UartError;
}

impl<'tx, 'd> embedded_hal_nb::serial::Write for BlockingUartTx<'tx, 'd> {
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

impl<'tx, 'd> embedded_io::WriteReady for BlockingUartTx<'tx, 'd> {
    fn write_ready(&mut self) -> Result<bool, Self::Error> {
        Ok(write_ready(&self.inner))
    }
}

/// A UART receiver for blocking operations.
/// This struct implements blocking read operations for UART communication.
pub struct BlockingUartRx<'rx, 'd> {
    /// Holds a reference to the UART register block.
    inner: &'static RegisterBlock,
    /// Contains a mutable handle to the RX pad.
    rx: &'rx mut FlexPad,
    /// Uses PhantomData for lifetime tracking.
    _marker: PhantomData<&'d ()>,
}

impl<'rx, 'd> embedded_io::ErrorType for BlockingUartRx<'rx, 'd> {
    type Error = UartError;
}

impl<'rx, 'd> embedded_io::Read for BlockingUartRx<'rx, 'd> {
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

impl<'rx, 'd> embedded_hal_nb::serial::ErrorType for BlockingUartRx<'rx, 'd> {
    type Error = UartError;
}

impl<'rx, 'd> embedded_hal_nb::serial::Read for BlockingUartRx<'rx, 'd> {
    fn read(&mut self) -> embedded_hal_nb::nb::Result<u8, Self::Error> {
        let mut buf = [0];
        let len = blocking_read(&self.inner, &mut buf);
        match len {
            0 => Err(embedded_hal_nb::nb::Error::WouldBlock),
            _ => Ok(buf[0]),
        }
    }
}

impl<'rx, 'd> embedded_io::ReadReady for BlockingUartRx<'rx, 'd> {
    fn read_ready(&mut self) -> Result<bool, Self::Error> {
        Ok(read_ready(&self.inner))
    }
}
