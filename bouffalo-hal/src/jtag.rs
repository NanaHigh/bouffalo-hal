//! JTAG interface feature support.

#[cfg(any(doc, feature = "glb-v2"))]
use crate::glb::{
    self,
    v2::{Function, GpioConfig},
    Drive, Pull,
};
use crate::gpio::Alternate;
#[cfg(any(doc, feature = "glb-v2"))]
use crate::gpio::Pad;
#[cfg(any(doc, feature = "glb-v2"))]
use core::marker::PhantomData;
#[cfg(any(doc, feature = "glb-v2"))]
use core::ops::Deref;

/// D0 core JTAG mode (type state).
pub struct JtagD0;

/// M0 core JTAG mode (type state).
pub struct JtagM0;

/// LP core JTAG mode (type state).
pub struct JtagLp;

impl Alternate for JtagD0 {
    #[cfg(feature = "glb-v2")]
    const F: Function = Function::JtagD0;
}

impl Alternate for JtagM0 {
    #[cfg(feature = "glb-v2")]
    const F: Function = Function::JtagM0;
}

impl Alternate for JtagLp {
    #[cfg(feature = "glb-v2")]
    const F: Function = Function::JtagLp;
}

// requires to set `.set_function(Function::JtagXx)` before use.
#[cfg(feature = "glb-v2")]
const JTAG_GPIO_CONFIG: GpioConfig = GpioConfig::RESET_VALUE
    .enable_input()
    .disable_output()
    .enable_schmitt()
    .set_drive(Drive::Drive0)
    .set_pull(Pull::None);

#[cfg(any(doc, feature = "glb-v2"))]
impl<GLB: Deref<Target = glb::v2::RegisterBlock>, const N: usize, M: Alternate> Pad<GLB, N, M> {
    /// Configures the pin to operate as D0 core JTAG.
    #[inline]
    pub fn into_jtag_d0(self) -> Pad<GLB, N, JtagD0> {
        let config = JTAG_GPIO_CONFIG.set_function(Function::JtagD0);
        unsafe { self.base.gpio_config[N].write(config) };
        Pad {
            base: self.base,
            _mode: PhantomData,
        }
    }
    /// Configures the pin to operate as M0 core JTAG.
    #[inline]
    pub fn into_jtag_m0(self) -> Pad<GLB, N, JtagM0> {
        let config = JTAG_GPIO_CONFIG.set_function(Function::JtagM0);
        unsafe { self.base.gpio_config[N].write(config) };
        Pad {
            base: self.base,
            _mode: PhantomData,
        }
    }
    /// Configures the pin to operate as LP core JTAG.
    #[inline]
    pub fn into_jtag_lp(self) -> Pad<GLB, N, JtagLp> {
        let config = JTAG_GPIO_CONFIG.set_function(Function::JtagLp);
        unsafe { self.base.gpio_config[N].write(config) };
        Pad {
            base: self.base,
            _mode: PhantomData,
        }
    }
}
