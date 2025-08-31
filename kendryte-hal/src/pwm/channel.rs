use core::convert::Infallible;

use super::driver::Pwm;

// There are only 3 channels used (4 in total) so we define each as a separate struct.
/// PWM channel 1 (uses comparator 1)
pub struct Ch1<'a, 'i> {
    pub(crate) pwm: &'a Pwm<'i>,
}
/// PWM channel 2 (uses comparator 2)
pub struct Ch2<'a, 'i> {
    pub(crate) pwm: &'a Pwm<'i>,
}
/// PWM channel 3 (uses comparator 3)
pub struct Ch3<'a, 'i> {
    pub(crate) pwm: &'a Pwm<'i>,
}

macro_rules! impl_channel {
    ($Ty:ident, $idx:expr) => {
        impl<'a, 'i> embedded_hal::pwm::ErrorType for $Ty<'a, 'i> {
            type Error = Infallible;
        }
        impl<'a, 'i> embedded_hal::pwm::SetDutyCycle for $Ty<'a, 'i> {
            #[inline]
            fn max_duty_cycle(&self) -> u16 {
                self.pwm.top()
            }

            #[inline]
            fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
                let top = self.max_duty_cycle();
                let duty = duty.min(top);
                // Comparator outputs high when pwms >= cmpN.
                // For left-aligned PWM with top set in cmp0, a high width of `duty`
                // can be achieved by setting threshold = top - duty.
                let threshold = (top - duty) as u32;
                unsafe {
                    self.pwm.inner.pwm_cmpn[$idx]
                        .modify(|r| r.with_pwm_cpmn(arbitrary_int::u31::new(threshold)));
                }
                Ok(())
            }
        }
    };
}

impl_channel!(Ch1, 1);
impl_channel!(Ch2, 2);
impl_channel!(Ch3, 3);
