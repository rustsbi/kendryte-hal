use crate::iomux::{SlewRate, Strength};
use arbitrary_int::{u1, u3};
use volatile_register::RW;

/// Pull-up/down configuration for a pad.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pull {
    None,
    Up,
    Down,
}

/// PadOps trait provides methods to operate and configure IO pads.
pub trait PadOps {
    /// Returns a reference to the underlying pad register.
    fn inner(&self) -> &RW<crate::iomux::Pad>;

    /// Set the pull-up or pull-down configuration for the pad.
    fn set_pull(&mut self, pull: Pull) -> &mut Self {
        unsafe {
            match pull {
                Pull::None => self
                    .inner()
                    .modify(|r| r.with_pull_up_enable(false).with_pull_down_enable(false)),
                Pull::Up => self
                    .inner()
                    .modify(|r| r.with_pull_up_enable(true).with_pull_down_enable(false)),
                Pull::Down => self
                    .inner()
                    .modify(|r| r.with_pull_up_enable(false).with_pull_down_enable(true)),
            };
        }
        self
    }

    /// Get the current pull-up or pull-down configuration of the pad.
    /// Returns Some(Pull) if only one is enabled, or None if both are enabled (invalid state).
    fn pull(&self) -> Option<Pull> {
        let is_pull_up = self.inner().read().pull_up_enable();
        let is_pull_down = self.inner().read().pull_down_enable();

        match (is_pull_up, is_pull_down) {
            (true, false) => Some(Pull::Up),
            (false, true) => Some(Pull::Down),
            (false, false) => Some(Pull::None),
            (true, true) => None,
        }
    }

    /// Enable the Schmitt trigger for the pad input.
    fn enable_schmitt_trigger(&mut self) -> &mut Self {
        unsafe {
            self.inner().modify(|r| r.with_schmitt_trigger_enable(true));
        }
        self
    }

    /// Disable the Schmitt trigger for the pad input.
    fn disable_schmitt_trigger(&mut self) -> &mut Self {
        unsafe {
            self.inner()
                .modify(|r| r.with_schmitt_trigger_enable(false));
        }
        self
    }

    /// Check if the Schmitt trigger is enabled for the pad input.
    fn is_schmitt_trigger_enabled(&self) -> bool {
        self.inner().read().schmitt_trigger_enable()
    }

    /// Set the slew rate for the pad output.
    fn set_slew_rate(&mut self, slew_rate: SlewRate) -> &mut Self {
        unsafe {
            self.inner().modify(|r| r.with_slew_rate(slew_rate));
        }
        self
    }

    /// Set the drive strength for the pad output.
    /// The drive_strength parameter controls the output current capability.
    fn set_drive_strength(&mut self, drive_strength: Strength) -> &mut Self {
        unsafe {
            self.inner()
                .modify(|r| r.with_drive_strength(drive_strength));
        }
        self
    }

    /// Set the function select value for the pad.
    fn set_function_select(&mut self, function_select: u3) -> &mut Self {
        unsafe {
            self.inner()
                .modify(|r| r.with_function_select(function_select));
        }
        self
    }

    /// Get the current slew rate setting of the pad.
    fn slew_rate(&self) -> SlewRate {
        self.inner().read().slew_rate()
    }

    /// Get the current drive strength setting of the pad.
    fn drive_strength(&self) -> Strength {
        self.inner().read().drive_strength()
    }

    /// Check if the pad input is enabled.
    fn is_input_enabled(&self) -> bool {
        self.inner().read().input_enable()
    }

    /// Check if the pad output is enabled.
    fn is_output_enabled(&self) -> bool {
        self.inner().read().output_enable()
    }

    /// Get the current function select value of the pad.
    fn function_select(&self) -> u3 {
        self.inner().read().function_select()
    }

    /// Read the input data from the pad.
    fn input_data(&self) -> u1 {
        self.inner().read().data_input()
    }

    /// Configure the pad as input only.
    /// This enables input and disables output.
    fn set_input(&mut self) -> &mut Self {
        unsafe {
            self.inner()
                .modify(|r| r.with_input_enable(true).with_output_enable(false));
        }
        self
    }

    /// Configure the pad as output only.
    /// This enables output and disables input.
    fn set_output(&mut self) -> &mut Self {
        unsafe {
            self.inner()
                .modify(|r| r.with_input_enable(false).with_output_enable(true));
        }
        self
    }

    /// Configure the pad as bidirectional (input and output enabled).
    fn set_bidirectional(&mut self) -> &mut Self {
        unsafe {
            self.inner()
                .modify(|r| r.with_input_enable(true).with_output_enable(true));
        }
        self
    }

    /// Disable both input and output for the pad.
    fn set_disabled(&mut self) -> &mut Self {
        unsafe {
            self.inner()
                .modify(|r| r.with_input_enable(false).with_output_enable(false));
        }
        self
    }
}
