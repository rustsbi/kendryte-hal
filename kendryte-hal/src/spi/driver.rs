use core::marker::PhantomData;

use crate::clocks::Clocks;
use crate::instance::Numbered;
use crate::iomux::FlexPad;
use crate::spi::pad::{IntoPads, IntoTransmitOnly};
use crate::spi::register::*;
use arbitrary_int::{u2, u5, u14, u15, u30};

/// Simple error type for SPI operations.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SpiError {
    BusyTimeout,
    FifoOverflow,
    FifoUnderflow,
}

impl embedded_hal::spi::Error for SpiError {
    fn kind(&self) -> embedded_hal::spi::ErrorKind {
        match self {
            SpiError::BusyTimeout => embedded_hal::spi::ErrorKind::Other,
            SpiError::FifoOverflow => embedded_hal::spi::ErrorKind::Overrun,
            SpiError::FifoUnderflow => embedded_hal::spi::ErrorKind::Other,
        }
    }
}

/// SPI mode (CPOL/CPHA)
pub type Mode = embedded_hal::spi::Mode;

/// Blocking SPI master implementing embedded-hal 1.0 `SpiBus<u8>`.
pub struct Spi<'i> {
    regs: &'static RegisterBlock,
    _pads: PhantomData<FlexPad<'i>>,
}

/// Configuration for SPI
#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub frequency: u32,
    pub mode: Mode,
    /// data frame size in bits (4..=16 typical, controller supports up to 32). We use 8 by default
    pub data_bits: u8,
    /// slave select bit index (0-based)
    pub ss_index: u8,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            frequency: 1_000_000,
            mode: embedded_hal::spi::MODE_0,
            data_bits: 8,
            ss_index: 0,
        }
    }
}

impl<'i> Spi<'i> {
    /// Create and configure an SPI master instance for numbered instance N.
    pub fn new<const N: usize>(
        instance: impl Numbered<'i, N, R = RegisterBlock>,
        cfg: Config,
        clocks: Clocks,
    ) -> Self {
        let regs = instance.inner();
        Self::configure::<N>(regs, cfg, clocks);
        Spi {
            regs,
            _pads: PhantomData,
        }
    }

    /// Create a new SPI with full-duplex pads (bouffalo-hal style API).
    #[inline]
    pub fn with_pads<const N: usize>(
        instance: impl Numbered<'i, N, R = RegisterBlock>,
        pads: impl IntoPads<'i, N>,
        cfg: Config,
        clocks: Clocks,
    ) -> Self {
        let pads = pads.into_full_duplex_pads();
        core::mem::forget(pads);
        Self::new(instance, cfg, clocks)
    }

    /// Create a new SPI in transmit-only mode with pads.
    #[inline]
    pub fn transmit_only<const N: usize>(
        instance: impl Numbered<'i, N, R = RegisterBlock>,
        pads: impl IntoTransmitOnly<'i, N>,
        cfg: Config,
        clocks: Clocks,
    ) -> Self {
        let pads = pads.into_transmit_only_pads();
        core::mem::forget(pads);
        let regs = instance.inner();
        Self::configure::<N>(regs, cfg, clocks);
        unsafe {
            regs.ctrlr0
                .modify(|r| r.with_transfer_mode(TransferMode::TransmitOnly));
        }
        Spi {
            regs,
            _pads: PhantomData,
        }
    }

    /// Create from a raw register pointer and a known source clock (Hz).
    /// Safety: caller must ensure `regs` points to a valid SPI RegisterBlock.
    pub unsafe fn from_regs_with_src_clock(
        regs: &'static RegisterBlock,
        src_clock_hz: u32,
        cfg: Config,
    ) -> Self {
        // Temporarily emulate a Clocks value by computing divider directly
        // Disable controller before changing config
        unsafe { regs.ssienr.modify(|r| r.with_ssi_enable(false)) };

        // Frame format and clock mode
        let (scpol, scph) = match (cfg.mode.polarity, cfg.mode.phase) {
            (
                embedded_hal::spi::Polarity::IdleLow,
                embedded_hal::spi::Phase::CaptureOnFirstTransition,
            ) => (SerialClockPolarity::Low, SerialClockPhase::Middle),
            (
                embedded_hal::spi::Polarity::IdleLow,
                embedded_hal::spi::Phase::CaptureOnSecondTransition,
            ) => (SerialClockPolarity::Low, SerialClockPhase::Start),
            (
                embedded_hal::spi::Polarity::IdleHigh,
                embedded_hal::spi::Phase::CaptureOnFirstTransition,
            ) => (SerialClockPolarity::High, SerialClockPhase::Middle),
            (
                embedded_hal::spi::Polarity::IdleHigh,
                embedded_hal::spi::Phase::CaptureOnSecondTransition,
            ) => (SerialClockPolarity::High, SerialClockPhase::Start),
        };
        let dfs = u5::new((cfg.data_bits.saturating_sub(1)).min(31));
        unsafe {
            regs.ctrlr0.modify(|r| {
                r.with_frame_format(FrameFormat::MotorolaSpi)
                    .with_serial_clock_polarity(scpol)
                    .with_serial_clock_phase(scph)
                    .with_transfer_mode(TransferMode::TransmitAndReceive)
                    .with_slave_output_enable(false)
                    .with_shift_register_loop(false)
                    .with_slave_select_toggle_enable(false)
                    .with_spi_frame_format(SpiFrameFormat::Standard)
                    .with_ssi_is_master(WorkingMode::Master)
                    .with_data_frame_size(dfs)
            })
        };

        let mut div2 = src_clock_hz / cfg.frequency;
        if div2 < 2 {
            div2 = 2;
        }
        if div2 % 2 == 1 {
            div2 += 1;
        }
        let sckdv = u15::new(((div2 / 2) as u16).max(1));
        unsafe { regs.baudr.modify(|r| r.with_ssi_clock_divider(sckdv)) };
        unsafe {
            regs.txftlr.modify(|r| {
                r.with_transmit_fifo_threshold(u2::new(0))
                    .with_transfer_start_fifo_level(u14::new(0))
            })
        };
        unsafe { regs.rxftlr.modify(|r| r.with_receive_fifo_threshold(0u8)) };
        let ser = (1u32 << (cfg.ss_index as u32)) & 0x3FFF_FFFF;
        unsafe {
            regs.ser
                .modify(|r| r.with_slave_select_enable(u30::new(ser)))
        };
        unsafe { regs.icr.modify(|r| r.with_interrupt_clear(true)) };
        unsafe { regs.ssienr.modify(|r| r.with_ssi_enable(true)) };

        Spi {
            regs,
            _pads: PhantomData,
        }
    }

    fn configure<const N: usize>(regs: &'static RegisterBlock, cfg: Config, clocks: Clocks) {
        // Disable controller before changing config
        unsafe { regs.ssienr.modify(|r| r.with_ssi_enable(false)) };

        // Frame format and clock mode
        let (scpol, scph) = match (cfg.mode.polarity, cfg.mode.phase) {
            (
                embedded_hal::spi::Polarity::IdleLow,
                embedded_hal::spi::Phase::CaptureOnFirstTransition,
            ) => (SerialClockPolarity::Low, SerialClockPhase::Middle),
            (
                embedded_hal::spi::Polarity::IdleLow,
                embedded_hal::spi::Phase::CaptureOnSecondTransition,
            ) => (SerialClockPolarity::Low, SerialClockPhase::Start),
            (
                embedded_hal::spi::Polarity::IdleHigh,
                embedded_hal::spi::Phase::CaptureOnFirstTransition,
            ) => (SerialClockPolarity::High, SerialClockPhase::Middle),
            (
                embedded_hal::spi::Polarity::IdleHigh,
                embedded_hal::spi::Phase::CaptureOnSecondTransition,
            ) => (SerialClockPolarity::High, SerialClockPhase::Start),
        };

        // data frame size is encoded as n-1 per Synopsys SSI, map 8 -> 7
        let dfs = u5::new((cfg.data_bits.saturating_sub(1)).min(31));

        unsafe {
            regs.ctrlr0.modify(|r| {
                r.with_frame_format(FrameFormat::MotorolaSpi)
                    .with_serial_clock_polarity(scpol)
                    .with_serial_clock_phase(scph)
                    .with_transfer_mode(TransferMode::TransmitAndReceive)
                    .with_slave_output_enable(false)
                    .with_shift_register_loop(false)
                    .with_slave_select_toggle_enable(false)
                    .with_spi_frame_format(SpiFrameFormat::Standard)
                    .with_ssi_is_master(WorkingMode::Master)
                    .with_data_frame_size(dfs)
            })
        };

        // Program baud rate divider: Fsclk = Fssi_clk / (2 * ssi_clock_divider)
        let src = clocks.uart_sclk::<N>().0; // reuse UART clock until a dedicated clock API is available
        let mut div2 = src / cfg.frequency;
        if div2 < 2 {
            div2 = 2;
        }
        if div2 % 2 == 1 {
            div2 += 1;
        } // ensure even
        let sckdv = u15::new(((div2 / 2) as u16).max(1));
        unsafe { regs.baudr.modify(|r| r.with_ssi_clock_divider(sckdv)) };

        // Default thresholds: start when at least 1 entry, RX trigger at 1
        unsafe {
            regs.txftlr.modify(|r| {
                r.with_transmit_fifo_threshold(u2::new(0))
                    .with_transfer_start_fifo_level(u14::new(0))
            })
        };
        unsafe { regs.rxftlr.modify(|r| r.with_receive_fifo_threshold(0u8)) };

        // Select slave
        let ser = (1u32 << (cfg.ss_index as u32)) & 0x3FFF_FFFF;
        unsafe {
            regs.ser
                .modify(|r| r.with_slave_select_enable(u30::new(ser)))
        };

        // Clear interrupts and enable
        unsafe { regs.icr.modify(|r| r.with_interrupt_clear(true)) };
        unsafe { regs.ssienr.modify(|r| r.with_ssi_enable(true)) };
    }

    #[inline]
    fn wait_tfnf(&self) {
        while !self.regs.sr.read().transmit_fifo_not_full() {
            core::hint::spin_loop();
        }
    }

    #[inline]
    fn wait_rfne(&self) {
        while !self.regs.sr.read().receive_fifo_not_empty() {
            core::hint::spin_loop();
        }
    }

    #[inline]
    fn wait_idle(&self) {
        while self.regs.sr.read().busy() {
            core::hint::spin_loop();
        }
    }
}

impl embedded_hal::spi::ErrorType for Spi<'_> {
    type Error = SpiError;
}

impl embedded_hal::spi::SpiBus<u8> for Spi<'_> {
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        for b in words.iter_mut() {
            // write dummy to generate clock
            self.wait_tfnf();
            unsafe { self.regs.dr_ssi_ctrl[0].modify(|r| r.with_data(0)) };
            self.wait_rfne();
            *b = self.regs.dr_ssi_ctrl[0].read().data() as u8;
        }
        Ok(())
    }

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        for &b in words.iter() {
            self.wait_tfnf();
            unsafe { self.regs.dr_ssi_ctrl[0].modify(|r| r.with_data(b as u32)) };
            // read and drop if data is received to keep FIFO balanced in full-duplex
            if self.regs.sr.read().receive_fifo_not_empty() {
                let _ = self.regs.dr_ssi_ctrl[0].read().data();
            }
        }
        self.wait_idle();
        Ok(())
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        assert_eq!(read.len(), write.len());
        for (rb, &wb) in read.iter_mut().zip(write.iter()) {
            self.wait_tfnf();
            unsafe { self.regs.dr_ssi_ctrl[0].modify(|r| r.with_data(wb as u32)) };
            self.wait_rfne();
            *rb = self.regs.dr_ssi_ctrl[0].read().data() as u8;
        }
        Ok(())
    }

    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        for w in words.iter_mut() {
            let wb = *w;
            self.wait_tfnf();
            unsafe { self.regs.dr_ssi_ctrl[0].modify(|r| r.with_data(wb as u32)) };
            self.wait_rfne();
            *w = self.regs.dr_ssi_ctrl[0].read().data() as u8;
        }
        Ok(())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.wait_idle();
        Ok(())
    }
}

impl embedded_hal_nb::spi::FullDuplex<u8> for Spi<'_> {
    fn read(&mut self) -> embedded_hal_nb::nb::Result<u8, Self::Error> {
        if self.regs.sr.read().receive_fifo_not_empty() {
            Ok(self.regs.dr_ssi_ctrl[0].read().data() as u8)
        } else {
            Err(embedded_hal_nb::nb::Error::WouldBlock)
        }
    }

    fn write(&mut self, word: u8) -> embedded_hal_nb::nb::Result<(), Self::Error> {
        if self.regs.sr.read().transmit_fifo_not_full() {
            unsafe { self.regs.dr_ssi_ctrl[0].modify(|r| r.with_data(word as u32)) };
            Ok(())
        } else {
            Err(embedded_hal_nb::nb::Error::WouldBlock)
        }
    }
}

impl embedded_hal::spi::SpiDevice<u8> for Spi<'_> {
    fn transaction<'a>(
        &mut self,
        operations: &mut [embedded_hal::spi::Operation<'a, u8>],
    ) -> Result<(), Self::Error> {
        for op in operations {
            match op {
                embedded_hal::spi::Operation::Read(buf) => {
                    embedded_hal::spi::SpiBus::read(self, buf)?
                }
                embedded_hal::spi::Operation::Write(buf) => {
                    embedded_hal::spi::SpiBus::write(self, buf)?
                }
                embedded_hal::spi::Operation::Transfer(read, write) => {
                    embedded_hal::spi::SpiBus::transfer(self, read, write)?
                }
                embedded_hal::spi::Operation::TransferInPlace(buf) => {
                    embedded_hal::spi::SpiBus::transfer_in_place(self, buf)?
                }
                embedded_hal::spi::Operation::DelayNs(delay) => {
                    for _ in 0..*delay {
                        core::hint::spin_loop();
                    }
                }
            }
        }
        Ok(())
    }
}
