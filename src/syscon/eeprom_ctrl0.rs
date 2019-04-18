#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EEPROM_CTRL0 {
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
pub struct SYS_EFUSE_UNLOCKR {
    bits: u8,
}
impl SYS_EFUSE_UNLOCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYS_EFUSE_LDALLR {
    bits: bool,
}
impl SYS_EFUSE_LDALLR {
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
pub struct SYS_EEPROM_VPDIDXR {
    bits: u8,
}
impl SYS_EEPROM_VPDIDXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYS_EEPROM_MDR {
    bits: u8,
}
impl SYS_EEPROM_MDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYS_AUTOLOAD_SUSR {
    bits: bool,
}
impl SYS_AUTOLOAD_SUSR {
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
pub struct SYS_EEPROM_SELR {
    bits: bool,
}
impl SYS_EEPROM_SELR {
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
pub struct SYS_EEPROM_EECSR {
    bits: bool,
}
impl SYS_EEPROM_EECSR {
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
pub struct SYS_EEPROM_EESKR {
    bits: bool,
}
impl SYS_EEPROM_EESKR {
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
pub struct SYS_EEPROM_EEDIR {
    bits: bool,
}
impl SYS_EEPROM_EEDIR {
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
pub struct SYS_EEPROM_EEDOR {
    bits: bool,
}
impl SYS_EEPROM_EEDOR {
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
pub struct _SYS_EFUSE_UNLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EFUSE_UNLOCKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_EFUSE_LDALLW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EFUSE_LDALLW<'a> {
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
pub struct _SYS_EEPROM_VPDIDXW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EEPROM_VPDIDXW<'a> {
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
pub struct _SYS_EEPROM_MDW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EEPROM_MDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_AUTOLOAD_SUSW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_AUTOLOAD_SUSW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_EEPROM_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EEPROM_SELW<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_EEPROM_EECSW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EEPROM_EECSW<'a> {
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
pub struct _SYS_EEPROM_EESKW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EEPROM_EESKW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_EEPROM_EEDIW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EEPROM_EEDIW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_EEPROM_EEDOW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_EEPROM_EEDOW<'a> {
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
    #[doc = "Bits 24:31"]
    #[inline]
    pub fn sys_efuse_unlock(&self) -> SYS_EFUSE_UNLOCKR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_EFUSE_UNLOCKR { bits }
    }
    #[doc = "Bit 16"]
    #[inline]
    pub fn sys_efuse_ldall(&self) -> SYS_EFUSE_LDALLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_EFUSE_LDALLR { bits }
    }
    #[doc = "Bits 8:15"]
    #[inline]
    pub fn sys_eeprom_vpdidx(&self) -> SYS_EEPROM_VPDIDXR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_EEPROM_VPDIDXR { bits }
    }
    #[doc = "Bits 6:7"]
    #[inline]
    pub fn sys_eeprom_md(&self) -> SYS_EEPROM_MDR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_EEPROM_MDR { bits }
    }
    #[doc = "Bit 5"]
    #[inline]
    pub fn sys_autoload_sus(&self) -> SYS_AUTOLOAD_SUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_AUTOLOAD_SUSR { bits }
    }
    #[doc = "Bit 4"]
    #[inline]
    pub fn sys_eeprom_sel(&self) -> SYS_EEPROM_SELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_EEPROM_SELR { bits }
    }
    #[doc = "Bit 3"]
    #[inline]
    pub fn sys_eeprom_eecs(&self) -> SYS_EEPROM_EECSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_EEPROM_EECSR { bits }
    }
    #[doc = "Bit 2"]
    #[inline]
    pub fn sys_eeprom_eesk(&self) -> SYS_EEPROM_EESKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_EEPROM_EESKR { bits }
    }
    #[doc = "Bit 1"]
    #[inline]
    pub fn sys_eeprom_eedi(&self) -> SYS_EEPROM_EEDIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_EEPROM_EEDIR { bits }
    }
    #[doc = "Bit 0"]
    #[inline]
    pub fn sys_eeprom_eedo(&self) -> SYS_EEPROM_EEDOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_EEPROM_EEDOR { bits }
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
    #[doc = "Bits 24:31"]
    #[inline]
    pub fn sys_efuse_unlock(&mut self) -> _SYS_EFUSE_UNLOCKW {
        _SYS_EFUSE_UNLOCKW { w: self }
    }
    #[doc = "Bit 16"]
    #[inline]
    pub fn sys_efuse_ldall(&mut self) -> _SYS_EFUSE_LDALLW {
        _SYS_EFUSE_LDALLW { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline]
    pub fn sys_eeprom_vpdidx(&mut self) -> _SYS_EEPROM_VPDIDXW {
        _SYS_EEPROM_VPDIDXW { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline]
    pub fn sys_eeprom_md(&mut self) -> _SYS_EEPROM_MDW {
        _SYS_EEPROM_MDW { w: self }
    }
    #[doc = "Bit 5"]
    #[inline]
    pub fn sys_autoload_sus(&mut self) -> _SYS_AUTOLOAD_SUSW {
        _SYS_AUTOLOAD_SUSW { w: self }
    }
    #[doc = "Bit 4"]
    #[inline]
    pub fn sys_eeprom_sel(&mut self) -> _SYS_EEPROM_SELW {
        _SYS_EEPROM_SELW { w: self }
    }
    #[doc = "Bit 3"]
    #[inline]
    pub fn sys_eeprom_eecs(&mut self) -> _SYS_EEPROM_EECSW {
        _SYS_EEPROM_EECSW { w: self }
    }
    #[doc = "Bit 2"]
    #[inline]
    pub fn sys_eeprom_eesk(&mut self) -> _SYS_EEPROM_EESKW {
        _SYS_EEPROM_EESKW { w: self }
    }
    #[doc = "Bit 1"]
    #[inline]
    pub fn sys_eeprom_eedi(&mut self) -> _SYS_EEPROM_EEDIW {
        _SYS_EEPROM_EEDIW { w: self }
    }
    #[doc = "Bit 0"]
    #[inline]
    pub fn sys_eeprom_eedo(&mut self) -> _SYS_EEPROM_EEDOW {
        _SYS_EEPROM_EEDOW { w: self }
    }
}
