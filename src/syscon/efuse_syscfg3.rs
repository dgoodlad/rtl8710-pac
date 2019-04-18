#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EFUSE_SYSCFG3 {
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
pub struct SYS_DBG_PINGP_ENR {
    bits: u8,
}
impl SYS_DBG_PINGP_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYS_DBG_SELR {
    bits: u16,
}
impl SYS_DBG_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYS_DBGBY3_LOC_SELR {
    bits: u8,
}
impl SYS_DBGBY3_LOC_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYS_DBGBY1_LOC_SELR {
    bits: u8,
}
impl SYS_DBGBY1_LOC_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYS_DBGBY0_LOC_SELR {
    bits: u8,
}
impl SYS_DBGBY0_LOC_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYS_EEROM_ANAPAR_SPLL_49R {
    bits: bool,
}
impl SYS_EEROM_ANAPAR_SPLL_49R {
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
pub struct SYS_EEROM_ANAPAR_SPLL_27_25R {
    bits: u8,
}
impl SYS_EEROM_ANAPAR_SPLL_27_25R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _SYS_DBG_PINGP_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_DBG_PINGP_ENW<'a> {
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
pub struct _SYS_DBG_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_DBG_SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_DBGBY3_LOC_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_DBGBY3_LOC_SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_DBGBY1_LOC_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_DBGBY1_LOC_SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_DBGBY0_LOC_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_DBGBY0_LOC_SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_EEROM_ANAPAR_SPLL_49W<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EEROM_ANAPAR_SPLL_49W<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_EEROM_ANAPAR_SPLL_27_25W<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EEROM_ANAPAR_SPLL_27_25W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    pub fn sys_dbg_pingp_en(&self) -> SYS_DBG_PINGP_ENR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_DBG_PINGP_ENR { bits }
    }
    #[doc = "Bits 16:27"]
    #[inline]
    pub fn sys_dbg_sel(&self) -> SYS_DBG_SELR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SYS_DBG_SELR { bits }
    }
    #[doc = "Bits 12:13"]
    #[inline]
    pub fn sys_dbgby3_loc_sel(&self) -> SYS_DBGBY3_LOC_SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_DBGBY3_LOC_SELR { bits }
    }
    #[doc = "Bits 10:11"]
    #[inline]
    pub fn sys_dbgby1_loc_sel(&self) -> SYS_DBGBY1_LOC_SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_DBGBY1_LOC_SELR { bits }
    }
    #[doc = "Bits 8:9"]
    #[inline]
    pub fn sys_dbgby0_loc_sel(&self) -> SYS_DBGBY0_LOC_SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_DBGBY0_LOC_SELR { bits }
    }
    #[doc = "Bit 3"]
    #[inline]
    pub fn sys_eerom_anapar_spll_49(&self) -> SYS_EEROM_ANAPAR_SPLL_49R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_EEROM_ANAPAR_SPLL_49R { bits }
    }
    #[doc = "Bits 0:2"]
    #[inline]
    pub fn sys_eerom_anapar_spll_27_25(&self) -> SYS_EEROM_ANAPAR_SPLL_27_25R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_EEROM_ANAPAR_SPLL_27_25R { bits }
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
    pub fn sys_dbg_pingp_en(&mut self) -> _SYS_DBG_PINGP_ENW {
        _SYS_DBG_PINGP_ENW { w: self }
    }
    #[doc = "Bits 16:27"]
    #[inline]
    pub fn sys_dbg_sel(&mut self) -> _SYS_DBG_SELW {
        _SYS_DBG_SELW { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline]
    pub fn sys_dbgby3_loc_sel(&mut self) -> _SYS_DBGBY3_LOC_SELW {
        _SYS_DBGBY3_LOC_SELW { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline]
    pub fn sys_dbgby1_loc_sel(&mut self) -> _SYS_DBGBY1_LOC_SELW {
        _SYS_DBGBY1_LOC_SELW { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline]
    pub fn sys_dbgby0_loc_sel(&mut self) -> _SYS_DBGBY0_LOC_SELW {
        _SYS_DBGBY0_LOC_SELW { w: self }
    }
    #[doc = "Bit 3"]
    #[inline]
    pub fn sys_eerom_anapar_spll_49(&mut self) -> _SYS_EEROM_ANAPAR_SPLL_49W {
        _SYS_EEROM_ANAPAR_SPLL_49W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline]
    pub fn sys_eerom_anapar_spll_27_25(&mut self) -> _SYS_EEROM_ANAPAR_SPLL_27_25W {
        _SYS_EEROM_ANAPAR_SPLL_27_25W { w: self }
    }
}
