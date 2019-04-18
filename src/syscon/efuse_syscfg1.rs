#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EFUSE_SYSCFG1 {
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
pub struct SYS_PDSPL_STLR {
    bits: u8,
}
impl SYS_PDSPL_STLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYS_PDSOC_STLR {
    bits: u8,
}
impl SYS_PDSOC_STLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYS_PDPON_STLR {
    bits: u8,
}
impl SYS_PDPON_STLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYS_SWREG_XRTR {
    bits: u8,
}
impl SYS_SWREG_XRTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYS_SWSLC_STLR {
    bits: u8,
}
impl SYS_SWSLC_STLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYS_EEROM_SWR_PAR_46_45R {
    bits: u8,
}
impl SYS_EEROM_SWR_PAR_46_45R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYS_EEROM_SWR_PAR_40_39R {
    bits: u8,
}
impl SYS_EEROM_SWR_PAR_40_39R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYS_EEROM_SWR_PAR_33_26R {
    bits: u8,
}
impl SYS_EEROM_SWR_PAR_33_26R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYS_EEROM_SWSLD_VOLR {
    bits: u8,
}
impl SYS_EEROM_SWSLD_VOLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _SYS_PDSPL_STLW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_PDSPL_STLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_PDSOC_STLW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_PDSOC_STLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_PDPON_STLW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_PDPON_STLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_SWREG_XRTW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_SWREG_XRTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_SWSLC_STLW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_SWSLC_STLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_EEROM_SWR_PAR_46_45W<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EEROM_SWR_PAR_46_45W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_EEROM_SWR_PAR_40_39W<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EEROM_SWR_PAR_40_39W<'a> {
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
pub struct _SYS_EEROM_SWR_PAR_33_26W<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EEROM_SWR_PAR_33_26W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_EEROM_SWSLD_VOLW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EEROM_SWSLD_VOLW<'a> {
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
    #[doc = "Bits 24:25"]
    #[inline]
    pub fn sys_pdspl_stl(&self) -> SYS_PDSPL_STLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_PDSPL_STLR { bits }
    }
    #[doc = "Bits 22:23"]
    #[inline]
    pub fn sys_pdsoc_stl(&self) -> SYS_PDSOC_STLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_PDSOC_STLR { bits }
    }
    #[doc = "Bits 20:21"]
    #[inline]
    pub fn sys_pdpon_stl(&self) -> SYS_PDPON_STLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_PDPON_STLR { bits }
    }
    #[doc = "Bits 18:19"]
    #[inline]
    pub fn sys_swreg_xrt(&self) -> SYS_SWREG_XRTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_SWREG_XRTR { bits }
    }
    #[doc = "Bits 16:17"]
    #[inline]
    pub fn sys_swslc_stl(&self) -> SYS_SWSLC_STLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_SWSLC_STLR { bits }
    }
    #[doc = "Bits 14:15"]
    #[inline]
    pub fn sys_eerom_swr_par_46_45(&self) -> SYS_EEROM_SWR_PAR_46_45R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_EEROM_SWR_PAR_46_45R { bits }
    }
    #[doc = "Bits 12:13"]
    #[inline]
    pub fn sys_eerom_swr_par_40_39(&self) -> SYS_EEROM_SWR_PAR_40_39R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_EEROM_SWR_PAR_40_39R { bits }
    }
    #[doc = "Bits 4:11"]
    #[inline]
    pub fn sys_eerom_swr_par_33_26(&self) -> SYS_EEROM_SWR_PAR_33_26R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_EEROM_SWR_PAR_33_26R { bits }
    }
    #[doc = "Bits 0:2"]
    #[inline]
    pub fn sys_eerom_swsld_vol(&self) -> SYS_EEROM_SWSLD_VOLR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_EEROM_SWSLD_VOLR { bits }
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
    #[doc = "Bits 24:25"]
    #[inline]
    pub fn sys_pdspl_stl(&mut self) -> _SYS_PDSPL_STLW {
        _SYS_PDSPL_STLW { w: self }
    }
    #[doc = "Bits 22:23"]
    #[inline]
    pub fn sys_pdsoc_stl(&mut self) -> _SYS_PDSOC_STLW {
        _SYS_PDSOC_STLW { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline]
    pub fn sys_pdpon_stl(&mut self) -> _SYS_PDPON_STLW {
        _SYS_PDPON_STLW { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline]
    pub fn sys_swreg_xrt(&mut self) -> _SYS_SWREG_XRTW {
        _SYS_SWREG_XRTW { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline]
    pub fn sys_swslc_stl(&mut self) -> _SYS_SWSLC_STLW {
        _SYS_SWSLC_STLW { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline]
    pub fn sys_eerom_swr_par_46_45(&mut self) -> _SYS_EEROM_SWR_PAR_46_45W {
        _SYS_EEROM_SWR_PAR_46_45W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline]
    pub fn sys_eerom_swr_par_40_39(&mut self) -> _SYS_EEROM_SWR_PAR_40_39W {
        _SYS_EEROM_SWR_PAR_40_39W { w: self }
    }
    #[doc = "Bits 4:11"]
    #[inline]
    pub fn sys_eerom_swr_par_33_26(&mut self) -> _SYS_EEROM_SWR_PAR_33_26W {
        _SYS_EEROM_SWR_PAR_33_26W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline]
    pub fn sys_eerom_swsld_vol(&mut self) -> _SYS_EEROM_SWSLD_VOLW {
        _SYS_EEROM_SWSLD_VOLW { w: self }
    }
}
