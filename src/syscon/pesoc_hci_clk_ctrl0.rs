#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PESOC_HCI_CLK_CTRL0 {
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
pub struct SOC_SLPCK_MII_MPHY_ENR {
    bits: bool,
}
impl SOC_SLPCK_MII_MPHY_ENR {
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
pub struct SOC_ACTCK_MII_MPHY_ENR {
    bits: bool,
}
impl SOC_ACTCK_MII_MPHY_ENR {
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
pub struct SOC_SLPCK_OTG_ENR {
    bits: bool,
}
impl SOC_SLPCK_OTG_ENR {
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
pub struct SOC_ACTCK_OTG_ENR {
    bits: bool,
}
impl SOC_ACTCK_OTG_ENR {
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
pub struct SOC_SLPCK_SDIO_HST_ENR {
    bits: bool,
}
impl SOC_SLPCK_SDIO_HST_ENR {
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
pub struct SOC_ACTCK_SDIO_HST_ENR {
    bits: bool,
}
impl SOC_ACTCK_SDIO_HST_ENR {
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
pub struct SOC_SLPCK_SDIO_DEV_ENR {
    bits: bool,
}
impl SOC_SLPCK_SDIO_DEV_ENR {
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
pub struct SOC_ACTCK_SDIO_DEV_ENR {
    bits: bool,
}
impl SOC_ACTCK_SDIO_DEV_ENR {
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
pub struct _SOC_SLPCK_MII_MPHY_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_SLPCK_MII_MPHY_ENW<'a> {
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
pub struct _SOC_ACTCK_MII_MPHY_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_ACTCK_MII_MPHY_ENW<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SOC_SLPCK_OTG_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_SLPCK_OTG_ENW<'a> {
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
pub struct _SOC_ACTCK_OTG_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_ACTCK_OTG_ENW<'a> {
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
pub struct _SOC_SLPCK_SDIO_HST_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_SLPCK_SDIO_HST_ENW<'a> {
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
pub struct _SOC_ACTCK_SDIO_HST_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_ACTCK_SDIO_HST_ENW<'a> {
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
pub struct _SOC_SLPCK_SDIO_DEV_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_SLPCK_SDIO_DEV_ENW<'a> {
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
pub struct _SOC_ACTCK_SDIO_DEV_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_ACTCK_SDIO_DEV_ENW<'a> {
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
    #[doc = "Bit 25"]
    #[inline]
    pub fn soc_slpck_mii_mphy_en(&self) -> SOC_SLPCK_MII_MPHY_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_SLPCK_MII_MPHY_ENR { bits }
    }
    #[doc = "Bit 24"]
    #[inline]
    pub fn soc_actck_mii_mphy_en(&self) -> SOC_ACTCK_MII_MPHY_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_ACTCK_MII_MPHY_ENR { bits }
    }
    #[doc = "Bit 5"]
    #[inline]
    pub fn soc_slpck_otg_en(&self) -> SOC_SLPCK_OTG_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_SLPCK_OTG_ENR { bits }
    }
    #[doc = "Bit 4"]
    #[inline]
    pub fn soc_actck_otg_en(&self) -> SOC_ACTCK_OTG_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_ACTCK_OTG_ENR { bits }
    }
    #[doc = "Bit 3"]
    #[inline]
    pub fn soc_slpck_sdio_hst_en(&self) -> SOC_SLPCK_SDIO_HST_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_SLPCK_SDIO_HST_ENR { bits }
    }
    #[doc = "Bit 2"]
    #[inline]
    pub fn soc_actck_sdio_hst_en(&self) -> SOC_ACTCK_SDIO_HST_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_ACTCK_SDIO_HST_ENR { bits }
    }
    #[doc = "Bit 1"]
    #[inline]
    pub fn soc_slpck_sdio_dev_en(&self) -> SOC_SLPCK_SDIO_DEV_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_SLPCK_SDIO_DEV_ENR { bits }
    }
    #[doc = "Bit 0"]
    #[inline]
    pub fn soc_actck_sdio_dev_en(&self) -> SOC_ACTCK_SDIO_DEV_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_ACTCK_SDIO_DEV_ENR { bits }
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
    #[doc = "Bit 25"]
    #[inline]
    pub fn soc_slpck_mii_mphy_en(&mut self) -> _SOC_SLPCK_MII_MPHY_ENW {
        _SOC_SLPCK_MII_MPHY_ENW { w: self }
    }
    #[doc = "Bit 24"]
    #[inline]
    pub fn soc_actck_mii_mphy_en(&mut self) -> _SOC_ACTCK_MII_MPHY_ENW {
        _SOC_ACTCK_MII_MPHY_ENW { w: self }
    }
    #[doc = "Bit 5"]
    #[inline]
    pub fn soc_slpck_otg_en(&mut self) -> _SOC_SLPCK_OTG_ENW {
        _SOC_SLPCK_OTG_ENW { w: self }
    }
    #[doc = "Bit 4"]
    #[inline]
    pub fn soc_actck_otg_en(&mut self) -> _SOC_ACTCK_OTG_ENW {
        _SOC_ACTCK_OTG_ENW { w: self }
    }
    #[doc = "Bit 3"]
    #[inline]
    pub fn soc_slpck_sdio_hst_en(&mut self) -> _SOC_SLPCK_SDIO_HST_ENW {
        _SOC_SLPCK_SDIO_HST_ENW { w: self }
    }
    #[doc = "Bit 2"]
    #[inline]
    pub fn soc_actck_sdio_hst_en(&mut self) -> _SOC_ACTCK_SDIO_HST_ENW {
        _SOC_ACTCK_SDIO_HST_ENW { w: self }
    }
    #[doc = "Bit 1"]
    #[inline]
    pub fn soc_slpck_sdio_dev_en(&mut self) -> _SOC_SLPCK_SDIO_DEV_ENW {
        _SOC_SLPCK_SDIO_DEV_ENW { w: self }
    }
    #[doc = "Bit 0"]
    #[inline]
    pub fn soc_actck_sdio_dev_en(&mut self) -> _SOC_ACTCK_SDIO_DEV_ENW {
        _SOC_ACTCK_SDIO_DEV_ENW { w: self }
    }
}
