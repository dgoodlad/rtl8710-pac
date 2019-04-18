#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EFUSE_CTRL {
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
pub struct SYS_EF_RWFLAGR {
    bits: bool,
}
impl SYS_EF_RWFLAGR {
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
pub struct SYS_EF_PGPDR {
    bits: u8,
}
impl SYS_EF_PGPDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYS_EF_RDTR {
    bits: u8,
}
impl SYS_EF_RDTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYS_EF_PGTSR {
    bits: u8,
}
impl SYS_EF_PGTSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYS_EF_PDWNR {
    bits: bool,
}
impl SYS_EF_PDWNR {
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
pub struct SYS_EF_ALDENR {
    bits: bool,
}
impl SYS_EF_ALDENR {
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
pub struct SYS_EF_ADDRR {
    bits: u16,
}
impl SYS_EF_ADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYS_EF_DATAR {
    bits: u8,
}
impl SYS_EF_DATAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _SYS_EF_RWFLAGW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EF_RWFLAGW<'a> {
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
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_EF_PGPDW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EF_PGPDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_EF_RDTW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EF_RDTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_EF_PGTSW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EF_PGTSW<'a> {
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
pub struct _SYS_EF_PDWNW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EF_PDWNW<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_EF_ALDENW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EF_ALDENW<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_EF_ADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EF_ADDRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_EF_DATAW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EF_DATAW<'a> {
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
    #[doc = "Bit 31"]
    #[inline]
    pub fn sys_ef_rwflag(&self) -> SYS_EF_RWFLAGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_EF_RWFLAGR { bits }
    }
    #[doc = "Bits 28:30"]
    #[inline]
    pub fn sys_ef_pgpd(&self) -> SYS_EF_PGPDR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_EF_PGPDR { bits }
    }
    #[doc = "Bits 24:26"]
    #[inline]
    pub fn sys_ef_rdt(&self) -> SYS_EF_RDTR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_EF_RDTR { bits }
    }
    #[doc = "Bits 20:23"]
    #[inline]
    pub fn sys_ef_pgts(&self) -> SYS_EF_PGTSR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_EF_PGTSR { bits }
    }
    #[doc = "Bit 19"]
    #[inline]
    pub fn sys_ef_pdwn(&self) -> SYS_EF_PDWNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_EF_PDWNR { bits }
    }
    #[doc = "Bit 18"]
    #[inline]
    pub fn sys_ef_alden(&self) -> SYS_EF_ALDENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_EF_ALDENR { bits }
    }
    #[doc = "Bits 8:17"]
    #[inline]
    pub fn sys_ef_addr(&self) -> SYS_EF_ADDRR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SYS_EF_ADDRR { bits }
    }
    #[doc = "Bits 0:7"]
    #[inline]
    pub fn sys_ef_data(&self) -> SYS_EF_DATAR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_EF_DATAR { bits }
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
    #[doc = "Bit 31"]
    #[inline]
    pub fn sys_ef_rwflag(&mut self) -> _SYS_EF_RWFLAGW {
        _SYS_EF_RWFLAGW { w: self }
    }
    #[doc = "Bits 28:30"]
    #[inline]
    pub fn sys_ef_pgpd(&mut self) -> _SYS_EF_PGPDW {
        _SYS_EF_PGPDW { w: self }
    }
    #[doc = "Bits 24:26"]
    #[inline]
    pub fn sys_ef_rdt(&mut self) -> _SYS_EF_RDTW {
        _SYS_EF_RDTW { w: self }
    }
    #[doc = "Bits 20:23"]
    #[inline]
    pub fn sys_ef_pgts(&mut self) -> _SYS_EF_PGTSW {
        _SYS_EF_PGTSW { w: self }
    }
    #[doc = "Bit 19"]
    #[inline]
    pub fn sys_ef_pdwn(&mut self) -> _SYS_EF_PDWNW {
        _SYS_EF_PDWNW { w: self }
    }
    #[doc = "Bit 18"]
    #[inline]
    pub fn sys_ef_alden(&mut self) -> _SYS_EF_ALDENW {
        _SYS_EF_ALDENW { w: self }
    }
    #[doc = "Bits 8:17"]
    #[inline]
    pub fn sys_ef_addr(&mut self) -> _SYS_EF_ADDRW {
        _SYS_EF_ADDRW { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline]
    pub fn sys_ef_data(&mut self) -> _SYS_EF_DATAW {
        _SYS_EF_DATAW { w: self }
    }
}
