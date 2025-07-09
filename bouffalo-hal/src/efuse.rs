//! Electronic fuse peripheral.
use core::ops::Deref;

use volatile_register::RW;

/// Electronic fuse peripheral registers.
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x800],
    /// Efuse interface 0 control register.
    pub control: RW<EfuseControl>,
    /// Efuse interface 0 cycle config0 register.
    pub cycle_config0: RW<EfuseCycleConfig0>,
    /// Efuse interface 0 cycle config1 register.
    pub cycle_config1: RW<EfuseCycleConfig1>,
    /// Efuse interface 0 manual config1 register.
    pub if0_manual_config1: RW<EfuseIf0ManualConfig1>,
    /// Efuse interface 0 status register.
    pub if0_status: RW<u32>,
    /// Efuse interface 0 config0 register.
    pub config0: RW<EfuseConfig0>,
    _reserved1: [u8; 0xE6],
    /// Only bl808 has the registers below.
    /// Efuse interface 1 control1 register.
    pub control1: RW<EfuseControl1>,
    /// Efuse interface 1 manual config1 register.
    pub if1_manual_config1: RW<EfuseIf1ManualConfig1>,
    /// Efuse interface 1 status register.
    pub if1_status: RW<EfuseIf1Status>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EfuseControl(u32);

impl EfuseControl {
    const IF_PROT_CODE_CYC_MASK: u32 = 0xFF << 24;
    const IF_0_INT_SET_MASK: u32 = 0x1 << 22;
    const IF_0_INT_CLR_MASK: u32 = 0x1 << 21;
    const IF_0_INT_MASK: u32 = 0x1 << 20;
    const IF_CYC_MODIFY_LOCK_MASK: u32 = 0x1 << 19;
    const IF_AUTO_RD_EN_MASK: u32 = 0x1 << 18;
    const PCLK_FORCE_ON_MASK: u32 = 0x1 << 17;
    const CLK_SAHB_DATA_GATE_MASK: u32 = 0x1 << 17;
    const IF_POR_DIG_MASK: u32 = 0x1 << 16;
    const IF_PROT_CODE_CTRL_MASK: u32 = 0xFF << 8;
    const CLK_SAHB_DATA_SEL_MASK: u32 = 0x1 << 7;
    const IF_0_CYC_MODIFY_MASK: u32 = 0x1 << 6;
    const IF_0_MANUAL_EN_MASK: u32 = 0x1 << 5;
    const IF_0_TRIG_MASK: u32 = 0x1 << 4;
    const IF_0_RW_MASK: u32 = 0x1 << 3;
    const IF_0_BUSY_MASK: u32 = 0x1 << 2;
    const IF_0_AUTOLOAD_DONE_MASK: u32 = 0x1 << 1;
    const IF_0_AUTOLOAD_P1_DONE_MASK: u32 = 0x1 << 0;

    /// Set protect code cycle.
    #[inline]
    pub const fn set_prot_code_cyc(self, cycle: u8) -> Self {
        Self((self.0 & !Self::IF_PROT_CODE_CYC_MASK) | ((cycle as u32) << 24))
    }
    /// Get protect code cycle.
    #[inline]
    pub const fn prot_code_cyc(self) -> u8 {
        ((self.0 & Self::IF_PROT_CODE_CYC_MASK) >> 24) as u8
    }
    /// Set interface 0 interrupt set bit.
    #[inline]
    pub const fn set_if_0_int_set(self, val: u8) -> Self {
        Self((self.0 & !Self::IF_0_INT_SET_MASK) | (Self::IF_0_INT_SET_MASK & ((val as u32) << 22)))
    }
    /// Get interface 0 interrupt set bit.
    #[inline]
    pub const fn if_0_int_set(self) -> u8 {
        ((self.0 & Self::IF_0_INT_SET_MASK) >> 22) as u8
    }
    /// Set interface 0 interrupt clear bit.
    #[inline]
    pub const fn set_if_0_int_clr(self, val: u8) -> Self {
        Self((self.0 & !Self::IF_0_INT_CLR_MASK) | (Self::IF_0_INT_CLR_MASK & ((val as u32) << 21)))
    }
    /// Check if interface 0 interrupt occurs.
    #[inline]
    pub const fn if_0_int_occurs(self) -> bool {
        (self.0 & Self::IF_0_INT_MASK) != 0
    }
    /// Set cycle modify lock bit.
    #[inline]
    pub const fn set_cyc_modify_lock(self, val: u8) -> Self {
        Self((self.0 & !Self::IF_CYC_MODIFY_LOCK_MASK) | ((val as u32) << 19))
    }
    /// Get cycle modify lock bit.
    #[inline]
    pub const fn cyc_modify_lock(self) -> u8 {
        ((self.0 & Self::IF_CYC_MODIFY_LOCK_MASK) >> 19) as u8
    }
    /// Enable automatic read.
    #[inline]
    pub const fn enable_auto_read(self) -> Self {
        Self(self.0 | Self::IF_AUTO_RD_EN_MASK)
    }
    /// Disable automatic read.
    #[inline]
    pub const fn disable_auto_read(self) -> Self {
        Self(self.0 & !Self::IF_AUTO_RD_EN_MASK)
    }
    /// Check if automatic read is enabled.
    #[inline]
    pub const fn is_auto_read_enabled(self) -> bool {
        (self.0 & Self::IF_AUTO_RD_EN_MASK) != 0
    }
    /// Enable PCLK force on.
    #[inline]
    pub const fn enable_pclk_force_on(self) -> Self {
        Self(self.0 | Self::PCLK_FORCE_ON_MASK)
    }
    /// Disable PCLK force on.
    #[inline]
    pub const fn disable_pclk_force_on(self) -> Self {
        Self(self.0 & !Self::PCLK_FORCE_ON_MASK)
    }
    /// Check if PCLK force on is enabled.
    #[inline]
    pub const fn is_pclk_force_on_enabled(self) -> bool {
        (self.0 & Self::PCLK_FORCE_ON_MASK) != 0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EfuseCycleConfig0(u32);

impl EfuseCycleConfig0 {
    const CYC_PD_CS_S_MASK: u32 = 0xFF << 24;
    const CYC_CS_MASK: u32 = 0x3F << 18;
    const CYC_RD_ADR_MASK: u32 = 0x3F << 12;
    const CYC_RD_DAT_MASK: u32 = 0x3F << 6;
    const CYC_RD_DMY_MASK: u32 = 0x3F << 0;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EfuseCycleConfig1(u32);

impl EfuseCycleConfig1 {
    const CYC_PD_CS_H_MASK: u32 = 0x3F << 26;
    const CYC_PS_CS_MASK: u32 = 0x3F << 20;
    const CYC_WR_ADR_MASK: u32 = 0x3F << 14;
    const CYC_PP_MASK: u32 = 0xFF << 6;
    const CYC_PI_MASK: u32 = 0x3F << 0;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EfuseIf0ManualConfig1(u32);

impl EfuseIf0ManualConfig1 {
    const PROT_CODE_MANUAL_MASK: u32 = 0xFF << 24;
    const Q_MASK: u32 = 0xFF << 16;
    const CSB_MASK: u32 = 0x1 << 15;
    const LOAD_MASK: u32 = 0x1 << 14;
    const PGENB_MASK: u32 = 0x1 << 13;
    const STROBE_MASK: u32 = 0x1 << 12;
    const PS_MASK: u32 = 0x1 << 11;
    const PD_MASK: u32 = 0x1 << 10;
    const A_MASK: u32 = 0x3FF << 0;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EfuseConfig0(u32);

impl EfuseConfig0 {
    const DBG_MODE_MASK: u32 = 0xF << 28;
    const DBG_JTAG_0_DIS_MASK: u32 = 0x3 << 26;
    const DBG_JTAG_1_DIS_MASK: u32 = 0x3 << 24;
    const EFUSE_DBG_DIS_MASK: u32 = 0x1 << 23;
    const SE_DBG_DIS_MASK: u32 = 0x1 << 22;
    const CPU_RST_DBG_DIS_MASK: u32 = 0x1 << 21;
    const CPU1_DIS_MASK: u32 = 0x1 << 20;
    const M154_DIS_MASK: u32 = 0x1 << 19;
    const CAM_DIS_MASK: u32 = 0x1 << 18;
    const KEY_ENC_EN_MASK: u32 = 0x1 << 17;
    const WIFI_DIS_MASK: u32 = 0x1 << 16;
    const BTDM_DIS_MASK: u32 = 0x1 << 15;
    const SDU_DIS_MASK: u32 = 0x1 << 14;
    const SF_KEY_RE_SEL_MASK: u32 = 0x3 << 12;
    const M1542_DIS_MASK: u32 = 0x1 << 11;
    const BLE2_DIS_MASK: u32 = 0x1 << 10;
    const UART_DIS_MASK: u32 = 0xF << 6;
    const SBOOT_EN_MASK: u32 = 0x3 << 4;
    const CPU0_DIS_MASK: u32 = 0x1 << 3;
    const AI_DIS_MASK: u32 = 0x1 << 2;
    const SF_AES_MODE_MASK: u32 = 0x3 << 0;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EfuseControl1(u32);

impl EfuseControl1 {
    const SW_DBG_MODE_MASK: u32 = 0xF << 28;
    const SW_DBG_JTAG_0_DIS_MASK: u32 = 0x3 << 26;
    const SW_DBG_JTAG_1_DIS_MASK: u32 = 0x3 << 24;
    const SW_EFUSE_DBG_DIS_MASK: u32 = 0x1 << 23;
    const SW_SE_DBG_DIS_MASK: u32 = 0x1 << 22;
    const SW_CPU_RST_DBG_DIS_MASK: u32 = 0x1 << 21;
    const SW_CPU1_DIS_MASK: u32 = 0x1 << 20;
    const SW_M154_DIS_MASK: u32 = 0x1 << 19;
    const SW_CAM_DIS_MASK: u32 = 0x1 << 18;
    const SW_0_KEY_ENC_EN_MASK: u32 = 0x1 << 17;
    const SW_WIFI_DIS_MASK: u32 = 0x1 << 16;
    const SW_BTDM_DIS_MASK: u32 = 0x1 << 15;
    const SW_SDU_DIS_MASK: u32 = 0x1 << 14;
    const SW_SF_KEY_RE_SEL_MASK: u32 = 0x3 << 12;
    const SW_M1542_DIS_MASK: u32 = 0x1 << 11;
    const SW_BLE2_DIS_MASK: u32 = 0x1 << 10;
    const SW_UART_DIS_MASK: u32 = 0xF << 6;
    const SW_SBOOT_EN_MASK: u32 = 0x3 << 4;
    const SW_CPU0_DIS_MASK: u32 = 0x1 << 3;
    const SW_AI_DIS_MASK: u32 = 0x1 << 2;
    const SW_SF_AES_MODE_MASK: u32 = 0x3 << 0;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EfuseIf1ManualConfig1(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EfuseIf1Status(u32);

/// Set system clock divider.
#[inline(always)]
pub(crate) fn set_system_clk(glb: &super::glb::v2::RegisterBlock, hdiv: u8, bdiv: u8) {
    const GLB_REG_BCLK_DIS_ADDR: u32 = 0x40000FFC;

    unsafe {
        glb.sys_config0
            .modify(|val| val.set_bus_clk_div(bdiv).set_cpu_clk_div(hdiv));

        core::ptr::write_volatile(GLB_REG_BCLK_DIS_ADDR as *mut u32, 0x00000001);
        core::ptr::write_volatile(GLB_REG_BCLK_DIS_ADDR as *mut u32, 0x00000000);

        for _ in 0..8 {
            core::arch::asm! {"nop"};
        }

        glb.sys_config0
            .modify(|val| val.enable_bclk().enable_hclk());

        for _ in 0..8 {
            core::arch::asm! {"nop"};
        }
    }
}

/// Efuse read write switch clock save.
#[inline(always)]
pub(crate) fn efuse_switch_cpu_clock_save(
    glb: &super::glb::v2::RegisterBlock,
    hbn: &super::hbn::RegisterBlock,
) {
    use super::hbn::{RootClockSource1, RootClockSource2};

    let sys_cfg0 = glb.sys_config0.read();
    let (_bdiv, _hdiv, _rtclk) = (
        sys_cfg0.bus_clk_div(),
        sys_cfg0.cpu_clk_div(),
        sys_cfg0.root_clk_sel(),
    );
    let _xclk = hbn.global.read().root_clock();
    unsafe {
        hbn.global.modify(|val| {
            val.set_root_clock_1(RootClockSource1::RC32M)
                .set_root_clock_2(RootClockSource2::Xclk)
        });
    }

    // Why weren't the previous variables used?
    set_system_clk(glb, 0, 0);
}

/// Device name.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DeviceName {}

pub struct Efuse<E>
where
    E: Deref<Target = RegisterBlock>,
{
    efuse: E,
}

impl<E: Deref<Target = RegisterBlock>> Efuse<E> {
    #[inline]
    pub fn new(efuse: E) -> Self {
        Self { efuse }
    }

    /// Read device common trim.
    #[inline]
    pub(crate) fn efuse_read_common_trim(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use core::mem::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, control), 0x800);
        assert_eq!(offset_of!(RegisterBlock, cycle_config0), 0x804);
        assert_eq!(offset_of!(RegisterBlock, cycle_config1), 0x808);
        assert_eq!(offset_of!(RegisterBlock, if0_manual_config1), 0x80C);
        assert_eq!(offset_of!(RegisterBlock, if0_status), 0x810);
        assert_eq!(offset_of!(RegisterBlock, control1), 0x900);
        assert_eq!(offset_of!(RegisterBlock, if1_manual_config1), 0x904);
        assert_eq!(offset_of!(RegisterBlock, if1_status), 0x908);
    }
}
