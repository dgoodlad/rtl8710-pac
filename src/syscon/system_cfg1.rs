#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYSTEM_CFG1 {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct SYSCFG_TRP_ICFGR {
    bits: u8,
}
impl SYSCFG_TRP_ICFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYSCFG_TRP_BOOT_SEL_R {
    bits: bool,
}
impl SYSCFG_TRP_BOOT_SEL_R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct SYSCFG_TRP_SPSLDO_SELR {
    bits: bool,
}
impl SYSCFG_TRP_SPSLDO_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct V15_VLDR {
    bits: bool,
}
impl V15_VLDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct SYS_SYSPLL_CLK_RDYR {
    bits: bool,
}
impl SYS_SYSPLL_CLK_RDYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct SYS_XCLK_VLDR {
    bits: bool,
}
impl SYS_XCLK_VLDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct SYSCFG_ALDN_STSR {
    bits: bool,
}
impl SYSCFG_ALDN_STSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Proxy"]
pub struct _SYSCFG_TRP_ICFGW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCFG_TRP_ICFGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYSCFG_TRP_BOOT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCFG_TRP_BOOT_SEL_W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYSCFG_TRP_SPSLDO_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCFG_TRP_SPSLDO_SELW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _V15_VLDW<'a> {
    w: &'a mut W,
}
impl<'a> _V15_VLDW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_SYSPLL_CLK_RDYW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_SYSPLL_CLK_RDYW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_XCLK_VLDW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_XCLK_VLDW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYSCFG_ALDN_STSW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCFG_ALDN_STSW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 28:31"]
    #[inline]
    pub fn syscfg_trp_icfg(&self) -> SYSCFG_TRP_ICFGR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYSCFG_TRP_ICFGR { bits }
    }
    #[doc = "Bit 27"]
    #[inline]
    pub fn syscfg_trp_boot_sel_(&self) -> SYSCFG_TRP_BOOT_SEL_R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSCFG_TRP_BOOT_SEL_R { bits }
    }
    #[doc = "Bit 26"]
    #[inline]
    pub fn syscfg_trp_spsldo_sel(&self) -> SYSCFG_TRP_SPSLDO_SELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSCFG_TRP_SPSLDO_SELR { bits }
    }
    #[doc = "Bit 16"]
    #[inline]
    pub fn v15_vld(&self) -> V15_VLDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        V15_VLDR { bits }
    }
    #[doc = "Bit 9"]
    #[inline]
    pub fn sys_syspll_clk_rdy(&self) -> SYS_SYSPLL_CLK_RDYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_SYSPLL_CLK_RDYR { bits }
    }
    #[doc = "Bit 8"]
    #[inline]
    pub fn sys_xclk_vld(&self) -> SYS_XCLK_VLDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_XCLK_VLDR { bits }
    }
    #[doc = "Bit 0"]
    #[inline]
    pub fn syscfg_aldn_sts(&self) -> SYSCFG_ALDN_STSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSCFG_ALDN_STSR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 28:31"]
    #[inline]
    pub fn syscfg_trp_icfg(&mut self) -> _SYSCFG_TRP_ICFGW {
        _SYSCFG_TRP_ICFGW { w: self }
    }
    #[doc = "Bit 27"]
    #[inline]
    pub fn syscfg_trp_boot_sel_(&mut self) -> _SYSCFG_TRP_BOOT_SEL_W {
        _SYSCFG_TRP_BOOT_SEL_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline]
    pub fn syscfg_trp_spsldo_sel(&mut self) -> _SYSCFG_TRP_SPSLDO_SELW {
        _SYSCFG_TRP_SPSLDO_SELW { w: self }
    }
    #[doc = "Bit 16"]
    #[inline]
    pub fn v15_vld(&mut self) -> _V15_VLDW {
        _V15_VLDW { w: self }
    }
    #[doc = "Bit 9"]
    #[inline]
    pub fn sys_syspll_clk_rdy(&mut self) -> _SYS_SYSPLL_CLK_RDYW {
        _SYS_SYSPLL_CLK_RDYW { w: self }
    }
    #[doc = "Bit 8"]
    #[inline]
    pub fn sys_xclk_vld(&mut self) -> _SYS_XCLK_VLDW {
        _SYS_XCLK_VLDW { w: self }
    }
    #[doc = "Bit 0"]
    #[inline]
    pub fn syscfg_aldn_sts(&mut self) -> _SYSCFG_ALDN_STSW {
        _SYSCFG_ALDN_STSW { w: self }
    }
}
