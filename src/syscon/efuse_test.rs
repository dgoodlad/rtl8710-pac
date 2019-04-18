#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EFUSE_TEST {
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
pub struct SYS_EF_CRES_SELR {
    bits: bool,
}
impl SYS_EF_CRES_SELR {
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
pub struct SYS_EF_SCAN_STARTR {
    bits: u16,
}
impl SYS_EF_SCAN_STARTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYS_EF_SCAN_ENDR {
    bits: u8,
}
impl SYS_EF_SCAN_ENDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYS_EF_FORCE_PGMENR {
    bits: bool,
}
impl SYS_EF_FORCE_PGMENR {
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
pub struct SYS_EF_CELL_SELR {
    bits: u8,
}
impl SYS_EF_CELL_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYS_EF_TRPTR {
    bits: bool,
}
impl SYS_EF_TRPTR {
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
pub struct SYS_EF_SCAN_TTHDR {
    bits: u8,
}
impl SYS_EF_SCAN_TTHDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _SYS_EF_CRES_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EF_CRES_SELW<'a> {
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
pub struct _SYS_EF_SCAN_STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EF_SCAN_STARTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_EF_SCAN_ENDW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EF_SCAN_ENDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_EF_FORCE_PGMENW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EF_FORCE_PGMENW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_EF_CELL_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EF_CELL_SELW<'a> {
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
pub struct _SYS_EF_TRPTW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EF_TRPTW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_EF_SCAN_TTHDW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EF_SCAN_TTHDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
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
    #[doc = "Bit 26"]
    #[inline]
    pub fn sys_ef_cres_sel(&self) -> SYS_EF_CRES_SELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_EF_CRES_SELR { bits }
    }
    #[doc = "Bits 16:24"]
    #[inline]
    pub fn sys_ef_scan_start(&self) -> SYS_EF_SCAN_STARTR {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SYS_EF_SCAN_STARTR { bits }
    }
    #[doc = "Bits 12:15"]
    #[inline]
    pub fn sys_ef_scan_end(&self) -> SYS_EF_SCAN_ENDR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_EF_SCAN_ENDR { bits }
    }
    #[doc = "Bit 11"]
    #[inline]
    pub fn sys_ef_force_pgmen(&self) -> SYS_EF_FORCE_PGMENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_EF_FORCE_PGMENR { bits }
    }
    #[doc = "Bits 8:9"]
    #[inline]
    pub fn sys_ef_cell_sel(&self) -> SYS_EF_CELL_SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_EF_CELL_SELR { bits }
    }
    #[doc = "Bit 7"]
    #[inline]
    pub fn sys_ef_trpt(&self) -> SYS_EF_TRPTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_EF_TRPTR { bits }
    }
    #[doc = "Bits 0:6"]
    #[inline]
    pub fn sys_ef_scan_tthd(&self) -> SYS_EF_SCAN_TTHDR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_EF_SCAN_TTHDR { bits }
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
    #[doc = "Bit 26"]
    #[inline]
    pub fn sys_ef_cres_sel(&mut self) -> _SYS_EF_CRES_SELW {
        _SYS_EF_CRES_SELW { w: self }
    }
    #[doc = "Bits 16:24"]
    #[inline]
    pub fn sys_ef_scan_start(&mut self) -> _SYS_EF_SCAN_STARTW {
        _SYS_EF_SCAN_STARTW { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline]
    pub fn sys_ef_scan_end(&mut self) -> _SYS_EF_SCAN_ENDW {
        _SYS_EF_SCAN_ENDW { w: self }
    }
    #[doc = "Bit 11"]
    #[inline]
    pub fn sys_ef_force_pgmen(&mut self) -> _SYS_EF_FORCE_PGMENW {
        _SYS_EF_FORCE_PGMENW { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline]
    pub fn sys_ef_cell_sel(&mut self) -> _SYS_EF_CELL_SELW {
        _SYS_EF_CELL_SELW { w: self }
    }
    #[doc = "Bit 7"]
    #[inline]
    pub fn sys_ef_trpt(&mut self) -> _SYS_EF_TRPTW {
        _SYS_EF_TRPTW { w: self }
    }
    #[doc = "Bits 0:6"]
    #[inline]
    pub fn sys_ef_scan_tthd(&mut self) -> _SYS_EF_SCAN_TTHDW {
        _SYS_EF_SCAN_TTHDW { w: self }
    }
}
