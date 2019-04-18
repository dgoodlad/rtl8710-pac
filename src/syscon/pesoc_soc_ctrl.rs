#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PESOC_SOC_CTRL {
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
pub struct PESOC_GDMA_CFGR {
    bits: u16,
}
impl PESOC_GDMA_CFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PESOC_MII_LX_SLV_SWAP_SELR {
    bits: bool,
}
impl PESOC_MII_LX_SLV_SWAP_SELR {
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
pub struct PESOC_MII_LX_MST_SWAP_SELR {
    bits: bool,
}
impl PESOC_MII_LX_MST_SWAP_SELR {
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
pub struct PESOC_MII_LX_WRAPPER_ENR {
    bits: bool,
}
impl PESOC_MII_LX_WRAPPER_ENR {
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
pub struct PESOC_LX_SLV_SWAP_SELR {
    bits: bool,
}
impl PESOC_LX_SLV_SWAP_SELR {
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
pub struct PESOC_LX_MST_SWAP_SELR {
    bits: bool,
}
impl PESOC_LX_MST_SWAP_SELR {
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
pub struct PESOC_LX_WL_SWAP_SELR {
    bits: bool,
}
impl PESOC_LX_WL_SWAP_SELR {
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
pub struct PESOC_SRAM_MUX_CFGR {
    bits: u8,
}
impl PESOC_SRAM_MUX_CFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PESOC_GDMA_CFGW<'a> {
    w: &'a mut W,
}
impl<'a> _PESOC_GDMA_CFGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 8191;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PESOC_MII_LX_SLV_SWAP_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _PESOC_MII_LX_SLV_SWAP_SELW<'a> {
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
pub struct _PESOC_MII_LX_MST_SWAP_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _PESOC_MII_LX_MST_SWAP_SELW<'a> {
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
pub struct _PESOC_MII_LX_WRAPPER_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PESOC_MII_LX_WRAPPER_ENW<'a> {
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
pub struct _PESOC_LX_SLV_SWAP_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _PESOC_LX_SLV_SWAP_SELW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PESOC_LX_MST_SWAP_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _PESOC_LX_MST_SWAP_SELW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PESOC_LX_WL_SWAP_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _PESOC_LX_WL_SWAP_SELW<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PESOC_SRAM_MUX_CFGW<'a> {
    w: &'a mut W,
}
impl<'a> _PESOC_SRAM_MUX_CFGW<'a> {
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
    #[doc = "Bits 16:28"]
    #[inline]
    pub fn pesoc_gdma_cfg(&self) -> PESOC_GDMA_CFGR {
        let bits = {
            const MASK: u16 = 8191;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        PESOC_GDMA_CFGR { bits }
    }
    #[doc = "Bit 13"]
    #[inline]
    pub fn pesoc_mii_lx_slv_swap_sel(&self) -> PESOC_MII_LX_SLV_SWAP_SELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PESOC_MII_LX_SLV_SWAP_SELR { bits }
    }
    #[doc = "Bit 12"]
    #[inline]
    pub fn pesoc_mii_lx_mst_swap_sel(&self) -> PESOC_MII_LX_MST_SWAP_SELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PESOC_MII_LX_MST_SWAP_SELR { bits }
    }
    #[doc = "Bit 11"]
    #[inline]
    pub fn pesoc_mii_lx_wrapper_en(&self) -> PESOC_MII_LX_WRAPPER_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PESOC_MII_LX_WRAPPER_ENR { bits }
    }
    #[doc = "Bit 10"]
    #[inline]
    pub fn pesoc_lx_slv_swap_sel(&self) -> PESOC_LX_SLV_SWAP_SELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PESOC_LX_SLV_SWAP_SELR { bits }
    }
    #[doc = "Bit 9"]
    #[inline]
    pub fn pesoc_lx_mst_swap_sel(&self) -> PESOC_LX_MST_SWAP_SELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PESOC_LX_MST_SWAP_SELR { bits }
    }
    #[doc = "Bit 8"]
    #[inline]
    pub fn pesoc_lx_wl_swap_sel(&self) -> PESOC_LX_WL_SWAP_SELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PESOC_LX_WL_SWAP_SELR { bits }
    }
    #[doc = "Bits 0:2"]
    #[inline]
    pub fn pesoc_sram_mux_cfg(&self) -> PESOC_SRAM_MUX_CFGR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PESOC_SRAM_MUX_CFGR { bits }
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
    #[doc = "Bits 16:28"]
    #[inline]
    pub fn pesoc_gdma_cfg(&mut self) -> _PESOC_GDMA_CFGW {
        _PESOC_GDMA_CFGW { w: self }
    }
    #[doc = "Bit 13"]
    #[inline]
    pub fn pesoc_mii_lx_slv_swap_sel(&mut self) -> _PESOC_MII_LX_SLV_SWAP_SELW {
        _PESOC_MII_LX_SLV_SWAP_SELW { w: self }
    }
    #[doc = "Bit 12"]
    #[inline]
    pub fn pesoc_mii_lx_mst_swap_sel(&mut self) -> _PESOC_MII_LX_MST_SWAP_SELW {
        _PESOC_MII_LX_MST_SWAP_SELW { w: self }
    }
    #[doc = "Bit 11"]
    #[inline]
    pub fn pesoc_mii_lx_wrapper_en(&mut self) -> _PESOC_MII_LX_WRAPPER_ENW {
        _PESOC_MII_LX_WRAPPER_ENW { w: self }
    }
    #[doc = "Bit 10"]
    #[inline]
    pub fn pesoc_lx_slv_swap_sel(&mut self) -> _PESOC_LX_SLV_SWAP_SELW {
        _PESOC_LX_SLV_SWAP_SELW { w: self }
    }
    #[doc = "Bit 9"]
    #[inline]
    pub fn pesoc_lx_mst_swap_sel(&mut self) -> _PESOC_LX_MST_SWAP_SELW {
        _PESOC_LX_MST_SWAP_SELW { w: self }
    }
    #[doc = "Bit 8"]
    #[inline]
    pub fn pesoc_lx_wl_swap_sel(&mut self) -> _PESOC_LX_WL_SWAP_SELW {
        _PESOC_LX_WL_SWAP_SELW { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline]
    pub fn pesoc_sram_mux_cfg(&mut self) -> _PESOC_SRAM_MUX_CFGW {
        _PESOC_SRAM_MUX_CFGW { w: self }
    }
}
