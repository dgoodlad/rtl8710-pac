#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EFUSE_SYSCFG2 {
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
pub struct SYS_EEROM_ANAPAR_SPLL_24_15R {
    bits: u16,
}
impl SYS_EEROM_ANAPAR_SPLL_24_15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYS_EEROM_ANAPAR_SPLL_05_02R {
    bits: u8,
}
impl SYS_EEROM_ANAPAR_SPLL_05_02R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYS_EEROM_XTAL_STEL_SELR {
    bits: u8,
}
impl SYS_EEROM_XTAL_STEL_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYS_SYS_EEROM_XTAL_FREQ_SELR {
    bits: u8,
}
impl SYS_SYS_EEROM_XTAL_FREQ_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _SYS_EEROM_ANAPAR_SPLL_24_15W<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EEROM_ANAPAR_SPLL_24_15W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_EEROM_ANAPAR_SPLL_05_02W<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EEROM_ANAPAR_SPLL_05_02W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_EEROM_XTAL_STEL_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EEROM_XTAL_STEL_SELW<'a> {
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
pub struct _SYS_SYS_EEROM_XTAL_FREQ_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_SYS_EEROM_XTAL_FREQ_SELW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 21:30"]
    #[inline]
    pub fn sys_eerom_anapar_spll_24_15(&self) -> SYS_EEROM_ANAPAR_SPLL_24_15R {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SYS_EEROM_ANAPAR_SPLL_24_15R { bits }
    }
    #[doc = "Bits 16:19"]
    #[inline]
    pub fn sys_eerom_anapar_spll_05_02(&self) -> SYS_EEROM_ANAPAR_SPLL_05_02R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_EEROM_ANAPAR_SPLL_05_02R { bits }
    }
    #[doc = "Bits 12:13"]
    #[inline]
    pub fn sys_eerom_xtal_stel_sel(&self) -> SYS_EEROM_XTAL_STEL_SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_EEROM_XTAL_STEL_SELR { bits }
    }
    #[doc = "Bits 8:9"]
    #[inline]
    pub fn sys_sys_eerom_xtal_freq_sel(&self) -> SYS_SYS_EEROM_XTAL_FREQ_SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_SYS_EEROM_XTAL_FREQ_SELR { bits }
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
    #[doc = "Bits 21:30"]
    #[inline]
    pub fn sys_eerom_anapar_spll_24_15(&mut self) -> _SYS_EEROM_ANAPAR_SPLL_24_15W {
        _SYS_EEROM_ANAPAR_SPLL_24_15W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline]
    pub fn sys_eerom_anapar_spll_05_02(&mut self) -> _SYS_EEROM_ANAPAR_SPLL_05_02W {
        _SYS_EEROM_ANAPAR_SPLL_05_02W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline]
    pub fn sys_eerom_xtal_stel_sel(&mut self) -> _SYS_EEROM_XTAL_STEL_SELW {
        _SYS_EEROM_XTAL_STEL_SELW { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline]
    pub fn sys_sys_eerom_xtal_freq_sel(&mut self) -> _SYS_SYS_EEROM_XTAL_FREQ_SELW {
        _SYS_SYS_EEROM_XTAL_FREQ_SELW { w: self }
    }
}
