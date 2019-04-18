#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SOC_HCI_COM_FUNC_EN {
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
pub struct SOC_HCI_WL_MACON_ENR {
    bits: bool,
}
impl SOC_HCI_WL_MACON_ENR {
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
pub struct SOC_HCI_SM_SELR {
    bits: bool,
}
impl SOC_HCI_SM_SELR {
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
pub struct SOC_HCI_MII_ENR {
    bits: bool,
}
impl SOC_HCI_MII_ENR {
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
pub struct SOC_HCI_OTG_RST_MUXR {
    bits: bool,
}
impl SOC_HCI_OTG_RST_MUXR {
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
pub struct SOC_HCI_OTG_ENR {
    bits: bool,
}
impl SOC_HCI_OTG_ENR {
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
pub struct SOC_HCI_SDIOD_ON_RST_MUXR {
    bits: bool,
}
impl SOC_HCI_SDIOD_ON_RST_MUXR {
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
pub struct SOC_HCI_SDIOH_ENR {
    bits: bool,
}
impl SOC_HCI_SDIOH_ENR {
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
pub struct SOC_HCI_SDIOD_OFF_ENR {
    bits: bool,
}
impl SOC_HCI_SDIOD_OFF_ENR {
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
pub struct SOC_HCI_SDIOD_ON_ENR {
    bits: bool,
}
impl SOC_HCI_SDIOD_ON_ENR {
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
pub struct _SOC_HCI_WL_MACON_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_HCI_WL_MACON_ENW<'a> {
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
pub struct _SOC_HCI_SM_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_HCI_SM_SELW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SOC_HCI_MII_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_HCI_MII_ENW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SOC_HCI_OTG_RST_MUXW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_HCI_OTG_RST_MUXW<'a> {
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
pub struct _SOC_HCI_OTG_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_HCI_OTG_ENW<'a> {
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
pub struct _SOC_HCI_SDIOD_ON_RST_MUXW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_HCI_SDIOD_ON_RST_MUXW<'a> {
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
pub struct _SOC_HCI_SDIOH_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_HCI_SDIOH_ENW<'a> {
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
pub struct _SOC_HCI_SDIOD_OFF_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_HCI_SDIOD_OFF_ENW<'a> {
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
pub struct _SOC_HCI_SDIOD_ON_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_HCI_SDIOD_ON_ENW<'a> {
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
    #[doc = "Bit 16"]
    #[inline]
    pub fn soc_hci_wl_macon_en(&self) -> SOC_HCI_WL_MACON_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_HCI_WL_MACON_ENR { bits }
    }
    #[doc = "Bit 13"]
    #[inline]
    pub fn soc_hci_sm_sel(&self) -> SOC_HCI_SM_SELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_HCI_SM_SELR { bits }
    }
    #[doc = "Bit 12"]
    #[inline]
    pub fn soc_hci_mii_en(&self) -> SOC_HCI_MII_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_HCI_MII_ENR { bits }
    }
    #[doc = "Bit 5"]
    #[inline]
    pub fn soc_hci_otg_rst_mux(&self) -> SOC_HCI_OTG_RST_MUXR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_HCI_OTG_RST_MUXR { bits }
    }
    #[doc = "Bit 4"]
    #[inline]
    pub fn soc_hci_otg_en(&self) -> SOC_HCI_OTG_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_HCI_OTG_ENR { bits }
    }
    #[doc = "Bit 3"]
    #[inline]
    pub fn soc_hci_sdiod_on_rst_mux(&self) -> SOC_HCI_SDIOD_ON_RST_MUXR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_HCI_SDIOD_ON_RST_MUXR { bits }
    }
    #[doc = "Bit 2"]
    #[inline]
    pub fn soc_hci_sdioh_en(&self) -> SOC_HCI_SDIOH_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_HCI_SDIOH_ENR { bits }
    }
    #[doc = "Bit 1"]
    #[inline]
    pub fn soc_hci_sdiod_off_en(&self) -> SOC_HCI_SDIOD_OFF_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_HCI_SDIOD_OFF_ENR { bits }
    }
    #[doc = "Bit 0"]
    #[inline]
    pub fn soc_hci_sdiod_on_en(&self) -> SOC_HCI_SDIOD_ON_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_HCI_SDIOD_ON_ENR { bits }
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
    #[doc = "Bit 16"]
    #[inline]
    pub fn soc_hci_wl_macon_en(&mut self) -> _SOC_HCI_WL_MACON_ENW {
        _SOC_HCI_WL_MACON_ENW { w: self }
    }
    #[doc = "Bit 13"]
    #[inline]
    pub fn soc_hci_sm_sel(&mut self) -> _SOC_HCI_SM_SELW {
        _SOC_HCI_SM_SELW { w: self }
    }
    #[doc = "Bit 12"]
    #[inline]
    pub fn soc_hci_mii_en(&mut self) -> _SOC_HCI_MII_ENW {
        _SOC_HCI_MII_ENW { w: self }
    }
    #[doc = "Bit 5"]
    #[inline]
    pub fn soc_hci_otg_rst_mux(&mut self) -> _SOC_HCI_OTG_RST_MUXW {
        _SOC_HCI_OTG_RST_MUXW { w: self }
    }
    #[doc = "Bit 4"]
    #[inline]
    pub fn soc_hci_otg_en(&mut self) -> _SOC_HCI_OTG_ENW {
        _SOC_HCI_OTG_ENW { w: self }
    }
    #[doc = "Bit 3"]
    #[inline]
    pub fn soc_hci_sdiod_on_rst_mux(&mut self) -> _SOC_HCI_SDIOD_ON_RST_MUXW {
        _SOC_HCI_SDIOD_ON_RST_MUXW { w: self }
    }
    #[doc = "Bit 2"]
    #[inline]
    pub fn soc_hci_sdioh_en(&mut self) -> _SOC_HCI_SDIOH_ENW {
        _SOC_HCI_SDIOH_ENW { w: self }
    }
    #[doc = "Bit 1"]
    #[inline]
    pub fn soc_hci_sdiod_off_en(&mut self) -> _SOC_HCI_SDIOD_OFF_ENW {
        _SOC_HCI_SDIOD_OFF_ENW { w: self }
    }
    #[doc = "Bit 0"]
    #[inline]
    pub fn soc_hci_sdiod_on_en(&mut self) -> _SOC_HCI_SDIOD_ON_ENW {
        _SOC_HCI_SDIOD_ON_ENW { w: self }
    }
}
