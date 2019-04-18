#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EFUSE_SYSCFG0 {
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
pub struct SYS_EEROM_SWR_PAR_05_00R {
    bits: u8,
}
impl SYS_EEROM_SWR_PAR_05_00R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYS_EEROM_LDO_PAR_07_04R {
    bits: u8,
}
impl SYS_EEROM_LDO_PAR_07_04R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYS_CHIPPDN_ENR {
    bits: bool,
}
impl SYS_CHIPPDN_ENR {
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
pub struct SYS_EEROM_B12V_ENR {
    bits: bool,
}
impl SYS_EEROM_B12V_ENR {
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
pub struct SYS_EEROM_VID1R {
    bits: u8,
}
impl SYS_EEROM_VID1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYS_EEROM_VID0R {
    bits: u8,
}
impl SYS_EEROM_VID0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _SYS_EEROM_SWR_PAR_05_00W<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EEROM_SWR_PAR_05_00W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_EEROM_LDO_PAR_07_04W<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EEROM_LDO_PAR_07_04W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_CHIPPDN_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_CHIPPDN_ENW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_EEROM_B12V_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EEROM_B12V_ENW<'a> {
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
pub struct _SYS_EEROM_VID1W<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EEROM_VID1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_EEROM_VID0W<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EEROM_VID0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bits 24:29"]
    #[inline]
    pub fn sys_eerom_swr_par_05_00(&self) -> SYS_EEROM_SWR_PAR_05_00R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_EEROM_SWR_PAR_05_00R { bits }
    }
    #[doc = "Bits 20:23"]
    #[inline]
    pub fn sys_eerom_ldo_par_07_04(&self) -> SYS_EEROM_LDO_PAR_07_04R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_EEROM_LDO_PAR_07_04R { bits }
    }
    #[doc = "Bit 17"]
    #[inline]
    pub fn sys_chippdn_en(&self) -> SYS_CHIPPDN_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_CHIPPDN_ENR { bits }
    }
    #[doc = "Bit 16"]
    #[inline]
    pub fn sys_eerom_b12v_en(&self) -> SYS_EEROM_B12V_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_EEROM_B12V_ENR { bits }
    }
    #[doc = "Bits 8:15"]
    #[inline]
    pub fn sys_eerom_vid1(&self) -> SYS_EEROM_VID1R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_EEROM_VID1R { bits }
    }
    #[doc = "Bits 0:7"]
    #[inline]
    pub fn sys_eerom_vid0(&self) -> SYS_EEROM_VID0R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_EEROM_VID0R { bits }
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
    #[doc = "Bits 24:29"]
    #[inline]
    pub fn sys_eerom_swr_par_05_00(&mut self) -> _SYS_EEROM_SWR_PAR_05_00W {
        _SYS_EEROM_SWR_PAR_05_00W { w: self }
    }
    #[doc = "Bits 20:23"]
    #[inline]
    pub fn sys_eerom_ldo_par_07_04(&mut self) -> _SYS_EEROM_LDO_PAR_07_04W {
        _SYS_EEROM_LDO_PAR_07_04W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline]
    pub fn sys_chippdn_en(&mut self) -> _SYS_CHIPPDN_ENW {
        _SYS_CHIPPDN_ENW { w: self }
    }
    #[doc = "Bit 16"]
    #[inline]
    pub fn sys_eerom_b12v_en(&mut self) -> _SYS_EEROM_B12V_ENW {
        _SYS_EEROM_B12V_ENW { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline]
    pub fn sys_eerom_vid1(&mut self) -> _SYS_EEROM_VID1W {
        _SYS_EEROM_VID1W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline]
    pub fn sys_eerom_vid0(&mut self) -> _SYS_EEROM_VID0W {
        _SYS_EEROM_VID0W { w: self }
    }
}
