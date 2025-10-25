use crate::soc::k230::PWM0;
use crate::soc::k230::pads::Pad;
use arbitrary_int::u3;
use kendryte_hal::instance::Instance;
use kendryte_hal::iomux::ops::PadOps;
use kendryte_hal::iomux::{FlexPad, IntoFlexPad};
use kendryte_hal::pwm::RegisterBlock;
use kendryte_hal::pwm::pad::IntoPwmOut;

impl Instance<'static> for PWM0 {
    type R = RegisterBlock;
    #[inline]
    fn inner(self) -> &'static Self::R {
        unsafe { &*PWM0::ptr() }
    }
}

impl<'i> Instance<'i> for &'i PWM0 {
    type R = RegisterBlock;
    #[inline]
    fn inner(self) -> &'static Self::R {
        unsafe { &*PWM0::ptr() }
    }
}

impl<'i> Instance<'i> for &'i mut PWM0 {
    type R = RegisterBlock;
    #[inline]
    fn inner(self) -> &'static Self::R {
        unsafe { &*PWM0::ptr() }
    }
}

// Map PWM outputs to pads based on the datasheet table.
// Table rows (all Direction: O):
// pwm_pwm_pins_1_io_pins_pwm_0_o_oval: PAD_IO_60(sel=1); PAD_IO_42(sel=2); PAD_IO_54(sel=3)
// pwm_pwm_pins_1_io_pins_pwm_1_o_oval: PAD_IO_61(sel=1); PAD_IO_43(sel=2); PAD_IO_55(sel=3)
// pwm_pwm_pins_1_io_pins_pwm_2_o_oval: PAD_IO_7(sel=1);  PAD_IO_46(sel=2); PAD_IO_56(sel=3)
// pwm_pwm_pins_1_io_pins_pwm_3_o_oval: PAD_IO_8(sel=1);  PAD_IO_47(sel=2); PAD_IO_57(sel=3)
// pwm_pwm_pins_1_io_pins_pwm_4_o_oval: PAD_IO_9(sel=1);  PAD_IO_52(sel=2); PAD_IO_58(sel=3)
// pwm_pwm_pins_1_io_pins_pwm_5_o_oval: PAD_IO_25(sel=1); PAD_IO_53(sel=2); PAD_IO_59(sel=3)

macro_rules! pad_pwm_out {
    (
        $( ($pad_num:expr, $function_select:expr, $pwm_out:expr) ),+ $(,)?
    ) => {
        $(
            impl IntoPwmOut<'static, $pwm_out> for Pad<$pad_num> {
                #[inline]
                fn into_pwm_out(self) -> FlexPad<'static> {
                    self.set_output()
                        .set_function_select(u3::new($function_select));
                    self.into_flex_pad()
                }
            }

            impl<'p> IntoPwmOut<'p, $pwm_out> for &'p mut Pad<$pad_num> {
                #[inline]
                fn into_pwm_out(self) -> FlexPad<'p> {
                    self.set_output()
                        .set_function_select(u3::new($function_select));
                    self.into_flex_pad()
                }
            }
        )+
    };
}

pad_pwm_out! {
    // pwm_0 outputs
    (60, 1, 0), (42, 2, 0), (54, 3, 0),
    // pwm_1 outputs
    (61, 1, 1), (43, 2, 1), (55, 3, 1),
    // pwm_2 outputs
    (7,  1, 2), (46, 2, 2), (56, 3, 2),
    // pwm_3 outputs
    (8,  1, 3), (47, 2, 3), (57, 3, 3),
    // pwm_4 outputs
    (9,  1, 4), (52, 2, 4), (58, 3, 4),
    // pwm_5 outputs
    (25, 1, 5), (53, 2, 5), (59, 3, 5),
}
