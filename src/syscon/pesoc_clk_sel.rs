#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PESOC_CLK_SEL {
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
pub struct PESOC_SPI1_SCLK_SELR {
    bits: bool,
}
impl PESOC_SPI1_SCLK_SELR {
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
pub struct PESOC_PERI_SCLK_SELR {
    bits: u8,
}
impl PESOC_PERI_SCLK_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PESOC_SDR_CK_SELR {
    bits: u8,
}
impl PESOC_SDR_CK_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PESOC_FLASH_CK_SELR {
    bits: u8,
}
impl PESOC_FLASH_CK_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PESOC_TRACE_CK_SELR {
    bits: u8,
}
impl PESOC_TRACE_CK_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PESOC_SPI1_SCLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _PESOC_SPI1_SCLK_SELW<'a> {
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
pub struct _PESOC_PERI_SCLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _PESOC_PERI_SCLK_SELW<'a> {
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
pub struct _PESOC_SDR_CK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _PESOC_SDR_CK_SELW<'a> {
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
pub struct _PESOC_FLASH_CK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _PESOC_FLASH_CK_SELW<'a> {
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
pub struct _PESOC_TRACE_CK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _PESOC_TRACE_CK_SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
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
    #[doc = "Bit 18"]
    #[inline]
    pub fn pesoc_spi1_sclk_sel(&self) -> PESOC_SPI1_SCLK_SELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PESOC_SPI1_SCLK_SELR { bits }
    }
    #[doc = "Bits 16:17"]
    #[inline]
    pub fn pesoc_peri_sclk_sel(&self) -> PESOC_PERI_SCLK_SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PESOC_PERI_SCLK_SELR { bits }
    }
    #[doc = "Bits 10:11"]
    #[inline]
    pub fn pesoc_sdr_ck_sel(&self) -> PESOC_SDR_CK_SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PESOC_SDR_CK_SELR { bits }
    }
    #[doc = "Bits 8:9"]
    #[inline]
    pub fn pesoc_flash_ck_sel(&self) -> PESOC_FLASH_CK_SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PESOC_FLASH_CK_SELR { bits }
    }
    #[doc = "Bits 4:5"]
    #[inline]
    pub fn pesoc_trace_ck_sel(&self) -> PESOC_TRACE_CK_SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PESOC_TRACE_CK_SELR { bits }
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
    #[doc = "Bit 18"]
    #[inline]
    pub fn pesoc_spi1_sclk_sel(&mut self) -> _PESOC_SPI1_SCLK_SELW {
        _PESOC_SPI1_SCLK_SELW { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline]
    pub fn pesoc_peri_sclk_sel(&mut self) -> _PESOC_PERI_SCLK_SELW {
        _PESOC_PERI_SCLK_SELW { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline]
    pub fn pesoc_sdr_ck_sel(&mut self) -> _PESOC_SDR_CK_SELW {
        _PESOC_SDR_CK_SELW { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline]
    pub fn pesoc_flash_ck_sel(&mut self) -> _PESOC_FLASH_CK_SELW {
        _PESOC_FLASH_CK_SELW { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline]
    pub fn pesoc_trace_ck_sel(&mut self) -> _PESOC_TRACE_CK_SELW {
        _PESOC_TRACE_CK_SELW { w: self }
    }
}
