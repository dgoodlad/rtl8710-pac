#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EFUSE_SYSCFG7 {
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
pub struct SYS_MEM_RMV_SIGNR {
    bits: bool,
}
impl SYS_MEM_RMV_SIGNR {
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
pub struct SYS_MEM_RMV_1PRF1R {
    bits: bool,
}
impl SYS_MEM_RMV_1PRF1R {
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
pub struct SYS_MEM_RMV_1PRF0R {
    bits: bool,
}
impl SYS_MEM_RMV_1PRF0R {
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
pub struct SYS_MEM_RMV_1PSRR {
    bits: bool,
}
impl SYS_MEM_RMV_1PSRR {
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
pub struct SYS_MEM_RMV_1PHSRR {
    bits: bool,
}
impl SYS_MEM_RMV_1PHSRR {
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
pub struct SYS_MEM_RMV_ROMR {
    bits: bool,
}
impl SYS_MEM_RMV_ROMR {
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
pub struct SYS_MEM_RME_CPUR {
    bits: u8,
}
impl SYS_MEM_RME_CPUR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYS_MEM_RME_WLANR {
    bits: u8,
}
impl SYS_MEM_RME_WLANR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYS_MEM_RME_USBR {
    bits: bool,
}
impl SYS_MEM_RME_USBR {
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
pub struct SYS_MEM_RME_SDIOR {
    bits: bool,
}
impl SYS_MEM_RME_SDIOR {
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
pub struct _SYS_MEM_RMV_SIGNW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_MEM_RMV_SIGNW<'a> {
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
pub struct _SYS_MEM_RMV_1PRF1W<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_MEM_RMV_1PRF1W<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_MEM_RMV_1PRF0W<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_MEM_RMV_1PRF0W<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_MEM_RMV_1PSRW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_MEM_RMV_1PSRW<'a> {
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
pub struct _SYS_MEM_RMV_1PHSRW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_MEM_RMV_1PHSRW<'a> {
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
pub struct _SYS_MEM_RMV_ROMW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_MEM_RMV_ROMW<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_MEM_RME_CPUW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_MEM_RME_CPUW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_MEM_RME_WLANW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_MEM_RME_WLANW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_MEM_RME_USBW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_MEM_RME_USBW<'a> {
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
pub struct _SYS_MEM_RME_SDIOW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_MEM_RME_SDIOW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 31"]
    #[inline]
    pub fn sys_mem_rmv_sign(&self) -> SYS_MEM_RMV_SIGNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_MEM_RMV_SIGNR { bits }
    }
    #[doc = "Bit 29"]
    #[inline]
    pub fn sys_mem_rmv_1prf1(&self) -> SYS_MEM_RMV_1PRF1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_MEM_RMV_1PRF1R { bits }
    }
    #[doc = "Bit 28"]
    #[inline]
    pub fn sys_mem_rmv_1prf0(&self) -> SYS_MEM_RMV_1PRF0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_MEM_RMV_1PRF0R { bits }
    }
    #[doc = "Bit 27"]
    #[inline]
    pub fn sys_mem_rmv_1psr(&self) -> SYS_MEM_RMV_1PSRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_MEM_RMV_1PSRR { bits }
    }
    #[doc = "Bit 26"]
    #[inline]
    pub fn sys_mem_rmv_1phsr(&self) -> SYS_MEM_RMV_1PHSRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_MEM_RMV_1PHSRR { bits }
    }
    #[doc = "Bit 25"]
    #[inline]
    pub fn sys_mem_rmv_rom(&self) -> SYS_MEM_RMV_ROMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_MEM_RMV_ROMR { bits }
    }
    #[doc = "Bits 22:24"]
    #[inline]
    pub fn sys_mem_rme_cpu(&self) -> SYS_MEM_RME_CPUR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_MEM_RME_CPUR { bits }
    }
    #[doc = "Bits 19:21"]
    #[inline]
    pub fn sys_mem_rme_wlan(&self) -> SYS_MEM_RME_WLANR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_MEM_RME_WLANR { bits }
    }
    #[doc = "Bit 18"]
    #[inline]
    pub fn sys_mem_rme_usb(&self) -> SYS_MEM_RME_USBR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_MEM_RME_USBR { bits }
    }
    #[doc = "Bit 17"]
    #[inline]
    pub fn sys_mem_rme_sdio(&self) -> SYS_MEM_RME_SDIOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_MEM_RME_SDIOR { bits }
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
    pub fn sys_mem_rmv_sign(&mut self) -> _SYS_MEM_RMV_SIGNW {
        _SYS_MEM_RMV_SIGNW { w: self }
    }
    #[doc = "Bit 29"]
    #[inline]
    pub fn sys_mem_rmv_1prf1(&mut self) -> _SYS_MEM_RMV_1PRF1W {
        _SYS_MEM_RMV_1PRF1W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline]
    pub fn sys_mem_rmv_1prf0(&mut self) -> _SYS_MEM_RMV_1PRF0W {
        _SYS_MEM_RMV_1PRF0W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline]
    pub fn sys_mem_rmv_1psr(&mut self) -> _SYS_MEM_RMV_1PSRW {
        _SYS_MEM_RMV_1PSRW { w: self }
    }
    #[doc = "Bit 26"]
    #[inline]
    pub fn sys_mem_rmv_1phsr(&mut self) -> _SYS_MEM_RMV_1PHSRW {
        _SYS_MEM_RMV_1PHSRW { w: self }
    }
    #[doc = "Bit 25"]
    #[inline]
    pub fn sys_mem_rmv_rom(&mut self) -> _SYS_MEM_RMV_ROMW {
        _SYS_MEM_RMV_ROMW { w: self }
    }
    #[doc = "Bits 22:24"]
    #[inline]
    pub fn sys_mem_rme_cpu(&mut self) -> _SYS_MEM_RME_CPUW {
        _SYS_MEM_RME_CPUW { w: self }
    }
    #[doc = "Bits 19:21"]
    #[inline]
    pub fn sys_mem_rme_wlan(&mut self) -> _SYS_MEM_RME_WLANW {
        _SYS_MEM_RME_WLANW { w: self }
    }
    #[doc = "Bit 18"]
    #[inline]
    pub fn sys_mem_rme_usb(&mut self) -> _SYS_MEM_RME_USBW {
        _SYS_MEM_RME_USBW { w: self }
    }
    #[doc = "Bit 17"]
    #[inline]
    pub fn sys_mem_rme_sdio(&mut self) -> _SYS_MEM_RME_SDIOW {
        _SYS_MEM_RME_SDIOW { w: self }
    }
}
