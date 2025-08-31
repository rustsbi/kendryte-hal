use crate::instance::Instance;
use core::convert::Infallible;

use super::channel::{Ch1, Ch2, Ch3};
use super::register::RegisterBlock;

/// PWM peripheral abstraction.
///
/// This wraps a [`RegisterBlock`] and provides a safe(ish) API plus
/// an embedded-hal implementation for channels 1-3.
pub struct Pwm<'i> {
    pub(crate) inner: &'static RegisterBlock,
    pub(crate) top: core::cell::Cell<u16>,
    _marker: core::marker::PhantomData<&'i ()>,
}

impl<'i> Pwm<'i> {
    /// Create a new PWM driver from a static register block reference.
    ///
    /// Safety: `inner` must point to the PWM peripheral's memory-mapped registers.
    #[inline]
    pub const unsafe fn from_raw(inner: &'static RegisterBlock) -> Self {
        Self {
            inner,
            top: core::cell::Cell::new(0),
            _marker: core::marker::PhantomData,
        }
    }

    /// Access the raw registers.
    #[inline]
    pub fn regs(&self) -> &'static RegisterBlock {
        self.inner
    }

    /// Construct from a peripheral instance that implements [`Instance`].
    #[inline]
    pub fn new<'a>(instance: impl Instance<'a, R = RegisterBlock>) -> Self {
        // Safe because Instance::inner yields a &'static to the MMIO block defined by SoC.
        unsafe { Self::from_raw(instance.inner()) }
    }

    /// Reset basic configuration to a known state.
    /// - scale = 0 (no prescale)
    /// - sticky = AutoClear
    /// - deglitch = Disabled
    /// - zero compare = Enabled (cmp0 defines period)
    /// - enable_always = Disabled (stopped)
    /// - alignment = Left for all channels
    /// - gang = Disabled for all channels
    pub fn reset_config(&mut self) {
        unsafe {
            self.inner.pwm_cfg.modify(|r| {
                r.with_pwm_scale(arbitrary_int::u4::new(0))
                    .with_pwm_sticky(super::register::StickyMode::AutoClear)
                    .with_pwm_zero_cmp(super::register::Enable::Enabled)
                    .with_pwm_deglitch(super::register::Enable::Disabled)
                    .with_pwm_en_always(super::register::Enable::Disabled)
                    .with_pwm_en_oneshot(super::register::Enable::Disabled)
                    .with_pwm_cmp0_center(super::register::Alignment::Left)
                    .with_pwm_cmp1_center(super::register::Alignment::Left)
                    .with_pwm_cmp2_center(super::register::Alignment::Left)
                    .with_pwm_cmp3_center(super::register::Alignment::Left)
                    .with_pwm_cmp0_gang(super::register::Enable::Disabled)
                    .with_pwm_cmp1_gang(super::register::Enable::Disabled)
                    .with_pwm_cmp2_gang(super::register::Enable::Disabled)
                    .with_pwm_cmp3_gang(super::register::Enable::Disabled)
            });
        }
    }

    /// Set prescaler (0..=15). Each increment divides by 2^n before compare.
    pub fn set_scale(&mut self, scale: u8) {
        let s = if scale > 15 { 15 } else { scale };
        unsafe {
            self.inner
                .pwm_cfg
                .modify(|r| r.with_pwm_scale(arbitrary_int::u4::new(s)));
        }
    }

    /// Set period (top) via comparator 0 when zero-compare mode is enabled.
    /// This value also becomes the embedded-hal max_duty for channels.
    pub fn set_period(&mut self, top: u16) {
        self.top.set(top);
        unsafe {
            self.inner.pwm_cmpn[0].modify(|r| r.with_pwm_cpmn(arbitrary_int::u31::new(top as u32)));
        }
    }

    /// Start free-running counter.
    pub fn start(&mut self) {
        unsafe {
            self.inner
                .pwm_cfg
                .modify(|r| r.with_pwm_en_always(super::register::Enable::Enabled));
        }
    }

    /// Stop counter.
    pub fn stop(&mut self) {
        unsafe {
            self.inner
                .pwm_cfg
                .modify(|r| r.with_pwm_en_always(super::register::Enable::Disabled));
        }
    }

    /// Get current top value (period counts) from cmp0.
    #[inline]
    pub fn top(&self) -> u16 {
        self.top.get()
    }

    /// Split into three channels (1,2,3). Comparator 0 is reserved for period/top.
    #[inline]
    pub fn split(&mut self) -> (Ch1<'_, 'i>, Ch2<'_, 'i>, Ch3<'_, 'i>) {
        (Ch1 { pwm: self }, Ch2 { pwm: self }, Ch3 { pwm: self })
    }
}
