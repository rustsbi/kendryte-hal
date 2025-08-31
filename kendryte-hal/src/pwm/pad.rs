pub(crate) use crate::iomux::FlexPad;

/// Convert a Pad into a PWM output for a specific PWM pin number.
///
/// N is the PWM output index as defined by the SoC's IOMUX (e.g., 0..=5).
pub trait IntoPwmOut<'p, const N: usize> {
    fn into_pwm_out(self) -> FlexPad<'p>;
}
