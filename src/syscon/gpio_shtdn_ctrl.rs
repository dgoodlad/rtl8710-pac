#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPIO_SHTDN_CTRL {
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
pub struct GPIO_GPK_SHTDN_NR {
    bits: bool,
}
impl GPIO_GPK_SHTDN_NR {
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
pub struct GPIO_GPJ_SHTDN_NR {
    bits: bool,
}
impl GPIO_GPJ_SHTDN_NR {
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
pub struct GPIO_GPI_SHTDN_NR {
    bits: bool,
}
impl GPIO_GPI_SHTDN_NR {
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
pub struct GPIO_GPH_SHTDN_NR {
    bits: bool,
}
impl GPIO_GPH_SHTDN_NR {
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
pub struct GPIO_GPG_SHTDN_NR {
    bits: bool,
}
impl GPIO_GPG_SHTDN_NR {
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
pub struct GPIO_GPF_SHTDN_NR {
    bits: bool,
}
impl GPIO_GPF_SHTDN_NR {
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
pub struct GPIO_GPE_SHTDN_NR {
    bits: bool,
}
impl GPIO_GPE_SHTDN_NR {
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
pub struct GPIO_GPD_SHTDN_NR {
    bits: bool,
}
impl GPIO_GPD_SHTDN_NR {
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
pub struct GPIO_GPC_SHTDN_NR {
    bits: bool,
}
impl GPIO_GPC_SHTDN_NR {
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
pub struct GPIO_GPB_SHTDN_NR {
    bits: bool,
}
impl GPIO_GPB_SHTDN_NR {
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
pub struct GPIO_GPA_SHTDN_NR {
    bits: bool,
}
impl GPIO_GPA_SHTDN_NR {
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
pub struct _GPIO_GPK_SHTDN_NW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPK_SHTDN_NW<'a> {
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
pub struct _GPIO_GPJ_SHTDN_NW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPJ_SHTDN_NW<'a> {
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
pub struct _GPIO_GPI_SHTDN_NW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPI_SHTDN_NW<'a> {
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
pub struct _GPIO_GPH_SHTDN_NW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPH_SHTDN_NW<'a> {
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
pub struct _GPIO_GPG_SHTDN_NW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPG_SHTDN_NW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_GPF_SHTDN_NW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPF_SHTDN_NW<'a> {
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
pub struct _GPIO_GPE_SHTDN_NW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPE_SHTDN_NW<'a> {
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
pub struct _GPIO_GPD_SHTDN_NW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPD_SHTDN_NW<'a> {
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
pub struct _GPIO_GPC_SHTDN_NW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPC_SHTDN_NW<'a> {
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
pub struct _GPIO_GPB_SHTDN_NW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPB_SHTDN_NW<'a> {
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
pub struct _GPIO_GPA_SHTDN_NW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPA_SHTDN_NW<'a> {
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
    #[doc = "Bit 10"]
    #[inline]
    pub fn gpio_gpk_shtdn_n(&self) -> GPIO_GPK_SHTDN_NR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPIO_GPK_SHTDN_NR { bits }
    }
    #[doc = "Bit 9"]
    #[inline]
    pub fn gpio_gpj_shtdn_n(&self) -> GPIO_GPJ_SHTDN_NR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPIO_GPJ_SHTDN_NR { bits }
    }
    #[doc = "Bit 8"]
    #[inline]
    pub fn gpio_gpi_shtdn_n(&self) -> GPIO_GPI_SHTDN_NR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPIO_GPI_SHTDN_NR { bits }
    }
    #[doc = "Bit 7"]
    #[inline]
    pub fn gpio_gph_shtdn_n(&self) -> GPIO_GPH_SHTDN_NR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPIO_GPH_SHTDN_NR { bits }
    }
    #[doc = "Bit 6"]
    #[inline]
    pub fn gpio_gpg_shtdn_n(&self) -> GPIO_GPG_SHTDN_NR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPIO_GPG_SHTDN_NR { bits }
    }
    #[doc = "Bit 5"]
    #[inline]
    pub fn gpio_gpf_shtdn_n(&self) -> GPIO_GPF_SHTDN_NR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPIO_GPF_SHTDN_NR { bits }
    }
    #[doc = "Bit 4"]
    #[inline]
    pub fn gpio_gpe_shtdn_n(&self) -> GPIO_GPE_SHTDN_NR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPIO_GPE_SHTDN_NR { bits }
    }
    #[doc = "Bit 3"]
    #[inline]
    pub fn gpio_gpd_shtdn_n(&self) -> GPIO_GPD_SHTDN_NR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPIO_GPD_SHTDN_NR { bits }
    }
    #[doc = "Bit 2"]
    #[inline]
    pub fn gpio_gpc_shtdn_n(&self) -> GPIO_GPC_SHTDN_NR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPIO_GPC_SHTDN_NR { bits }
    }
    #[doc = "Bit 1"]
    #[inline]
    pub fn gpio_gpb_shtdn_n(&self) -> GPIO_GPB_SHTDN_NR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPIO_GPB_SHTDN_NR { bits }
    }
    #[doc = "Bit 0"]
    #[inline]
    pub fn gpio_gpa_shtdn_n(&self) -> GPIO_GPA_SHTDN_NR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPIO_GPA_SHTDN_NR { bits }
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
    #[doc = "Bit 10"]
    #[inline]
    pub fn gpio_gpk_shtdn_n(&mut self) -> _GPIO_GPK_SHTDN_NW {
        _GPIO_GPK_SHTDN_NW { w: self }
    }
    #[doc = "Bit 9"]
    #[inline]
    pub fn gpio_gpj_shtdn_n(&mut self) -> _GPIO_GPJ_SHTDN_NW {
        _GPIO_GPJ_SHTDN_NW { w: self }
    }
    #[doc = "Bit 8"]
    #[inline]
    pub fn gpio_gpi_shtdn_n(&mut self) -> _GPIO_GPI_SHTDN_NW {
        _GPIO_GPI_SHTDN_NW { w: self }
    }
    #[doc = "Bit 7"]
    #[inline]
    pub fn gpio_gph_shtdn_n(&mut self) -> _GPIO_GPH_SHTDN_NW {
        _GPIO_GPH_SHTDN_NW { w: self }
    }
    #[doc = "Bit 6"]
    #[inline]
    pub fn gpio_gpg_shtdn_n(&mut self) -> _GPIO_GPG_SHTDN_NW {
        _GPIO_GPG_SHTDN_NW { w: self }
    }
    #[doc = "Bit 5"]
    #[inline]
    pub fn gpio_gpf_shtdn_n(&mut self) -> _GPIO_GPF_SHTDN_NW {
        _GPIO_GPF_SHTDN_NW { w: self }
    }
    #[doc = "Bit 4"]
    #[inline]
    pub fn gpio_gpe_shtdn_n(&mut self) -> _GPIO_GPE_SHTDN_NW {
        _GPIO_GPE_SHTDN_NW { w: self }
    }
    #[doc = "Bit 3"]
    #[inline]
    pub fn gpio_gpd_shtdn_n(&mut self) -> _GPIO_GPD_SHTDN_NW {
        _GPIO_GPD_SHTDN_NW { w: self }
    }
    #[doc = "Bit 2"]
    #[inline]
    pub fn gpio_gpc_shtdn_n(&mut self) -> _GPIO_GPC_SHTDN_NW {
        _GPIO_GPC_SHTDN_NW { w: self }
    }
    #[doc = "Bit 1"]
    #[inline]
    pub fn gpio_gpb_shtdn_n(&mut self) -> _GPIO_GPB_SHTDN_NW {
        _GPIO_GPB_SHTDN_NW { w: self }
    }
    #[doc = "Bit 0"]
    #[inline]
    pub fn gpio_gpa_shtdn_n(&mut self) -> _GPIO_GPA_SHTDN_NW {
        _GPIO_GPA_SHTDN_NW { w: self }
    }
}
