#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PWRMGT_OPTION {
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
pub struct SYSON_PMOPT_NORM_SYSCLK_SELR {
    bits: bool,
}
impl SYSON_PMOPT_NORM_SYSCLK_SELR {
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
pub struct SYSON_PMOPT_NORM_SYSPLL_ENR {
    bits: bool,
}
impl SYSON_PMOPT_NORM_SYSPLL_ENR {
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
pub struct SYSON_PMOPT_NORM_XTAL_ENR {
    bits: bool,
}
impl SYSON_PMOPT_NORM_XTAL_ENR {
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
pub struct SYSON_PMOPT_NORM_EN_SOCR {
    bits: bool,
}
impl SYSON_PMOPT_NORM_EN_SOCR {
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
pub struct SYSON_PMOPT_NORM_EN_PWMR {
    bits: bool,
}
impl SYSON_PMOPT_NORM_EN_PWMR {
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
pub struct SYSON_PMOPT_NORM_EN_SWRR {
    bits: bool,
}
impl SYSON_PMOPT_NORM_EN_SWRR {
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
pub struct SYSON_PMOPT_NORM_LPLDO_SELR {
    bits: bool,
}
impl SYSON_PMOPT_NORM_LPLDO_SELR {
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
pub struct SYSON_PMOPT_SNZ_SYSCLK_SELR {
    bits: bool,
}
impl SYSON_PMOPT_SNZ_SYSCLK_SELR {
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
pub struct SYSON_PMOPT_SNZ_SYSPLL_ENR {
    bits: bool,
}
impl SYSON_PMOPT_SNZ_SYSPLL_ENR {
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
pub struct SYSON_PMOPT_SNZ_XTAL_ENR {
    bits: bool,
}
impl SYSON_PMOPT_SNZ_XTAL_ENR {
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
pub struct SYSON_PMOPT_SNZ_EN_SOCR {
    bits: bool,
}
impl SYSON_PMOPT_SNZ_EN_SOCR {
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
pub struct SYSON_PMOPT_SNZ_EN_PWMR {
    bits: bool,
}
impl SYSON_PMOPT_SNZ_EN_PWMR {
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
pub struct SYSON_PMOPT_SNZ_EN_SWRR {
    bits: bool,
}
impl SYSON_PMOPT_SNZ_EN_SWRR {
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
pub struct SYSON_PMOPT_SNZ_LPLDO_SELR {
    bits: bool,
}
impl SYSON_PMOPT_SNZ_LPLDO_SELR {
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
pub struct SYSON_PMOPT_SLP_SYSCLK_SELR {
    bits: bool,
}
impl SYSON_PMOPT_SLP_SYSCLK_SELR {
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
pub struct SYSON_PMOPT_SLP_SYSPLL_ENR {
    bits: bool,
}
impl SYSON_PMOPT_SLP_SYSPLL_ENR {
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
pub struct SYSON_PMOPT_SLP_XTAL_ENR {
    bits: bool,
}
impl SYSON_PMOPT_SLP_XTAL_ENR {
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
pub struct SYSON_PMOPT_SLP_EN_SOCR {
    bits: bool,
}
impl SYSON_PMOPT_SLP_EN_SOCR {
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
pub struct SYSON_PMOPT_SLP_EN_PWMR {
    bits: bool,
}
impl SYSON_PMOPT_SLP_EN_PWMR {
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
pub struct SYSON_PMOPT_SLP_EN_SWRR {
    bits: bool,
}
impl SYSON_PMOPT_SLP_EN_SWRR {
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
pub struct SYSON_PMOPT_SLP_LPLDO_SELR {
    bits: bool,
}
impl SYSON_PMOPT_SLP_LPLDO_SELR {
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
pub struct SYSON_PMOPT_DSTBY_SYSCLK_SELR {
    bits: bool,
}
impl SYSON_PMOPT_DSTBY_SYSCLK_SELR {
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
pub struct SYSON_PMOPT_DSTBY_SYSPLL_ENR {
    bits: bool,
}
impl SYSON_PMOPT_DSTBY_SYSPLL_ENR {
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
pub struct SYSON_PMOPT_DSTBY_XTAL_ENR {
    bits: bool,
}
impl SYSON_PMOPT_DSTBY_XTAL_ENR {
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
pub struct SYSON_PMOPT_DSTBY_EN_SOCR {
    bits: bool,
}
impl SYSON_PMOPT_DSTBY_EN_SOCR {
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
pub struct SYSON_PMOPT_DSTBY_EN_PWMR {
    bits: bool,
}
impl SYSON_PMOPT_DSTBY_EN_PWMR {
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
pub struct SYSON_PMOPT_DSTBY_EN_SWRR {
    bits: bool,
}
impl SYSON_PMOPT_DSTBY_EN_SWRR {
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
pub struct SYSON_PMOPT_DSTBY_LPLDO_SELR {
    bits: bool,
}
impl SYSON_PMOPT_DSTBY_LPLDO_SELR {
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
pub struct _SYSON_PMOPT_NORM_SYSCLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_PMOPT_NORM_SYSCLK_SELW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYSON_PMOPT_NORM_SYSPLL_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_PMOPT_NORM_SYSPLL_ENW<'a> {
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
pub struct _SYSON_PMOPT_NORM_XTAL_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_PMOPT_NORM_XTAL_ENW<'a> {
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
pub struct _SYSON_PMOPT_NORM_EN_SOCW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_PMOPT_NORM_EN_SOCW<'a> {
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
pub struct _SYSON_PMOPT_NORM_EN_PWMW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_PMOPT_NORM_EN_PWMW<'a> {
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
pub struct _SYSON_PMOPT_NORM_EN_SWRW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_PMOPT_NORM_EN_SWRW<'a> {
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
pub struct _SYSON_PMOPT_NORM_LPLDO_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_PMOPT_NORM_LPLDO_SELW<'a> {
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
pub struct _SYSON_PMOPT_SNZ_SYSCLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_PMOPT_SNZ_SYSCLK_SELW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYSON_PMOPT_SNZ_SYSPLL_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_PMOPT_SNZ_SYSPLL_ENW<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYSON_PMOPT_SNZ_XTAL_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_PMOPT_SNZ_XTAL_ENW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYSON_PMOPT_SNZ_EN_SOCW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_PMOPT_SNZ_EN_SOCW<'a> {
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
pub struct _SYSON_PMOPT_SNZ_EN_PWMW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_PMOPT_SNZ_EN_PWMW<'a> {
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
pub struct _SYSON_PMOPT_SNZ_EN_SWRW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_PMOPT_SNZ_EN_SWRW<'a> {
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
#[doc = r" Proxy"]
pub struct _SYSON_PMOPT_SNZ_LPLDO_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_PMOPT_SNZ_LPLDO_SELW<'a> {
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
pub struct _SYSON_PMOPT_SLP_SYSCLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_PMOPT_SLP_SYSCLK_SELW<'a> {
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYSON_PMOPT_SLP_SYSPLL_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_PMOPT_SLP_SYSPLL_ENW<'a> {
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
pub struct _SYSON_PMOPT_SLP_XTAL_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_PMOPT_SLP_XTAL_ENW<'a> {
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
pub struct _SYSON_PMOPT_SLP_EN_SOCW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_PMOPT_SLP_EN_SOCW<'a> {
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
pub struct _SYSON_PMOPT_SLP_EN_PWMW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_PMOPT_SLP_EN_PWMW<'a> {
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
pub struct _SYSON_PMOPT_SLP_EN_SWRW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_PMOPT_SLP_EN_SWRW<'a> {
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
pub struct _SYSON_PMOPT_SLP_LPLDO_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_PMOPT_SLP_LPLDO_SELW<'a> {
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
pub struct _SYSON_PMOPT_DSTBY_SYSCLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_PMOPT_DSTBY_SYSCLK_SELW<'a> {
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
pub struct _SYSON_PMOPT_DSTBY_SYSPLL_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_PMOPT_DSTBY_SYSPLL_ENW<'a> {
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
pub struct _SYSON_PMOPT_DSTBY_XTAL_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_PMOPT_DSTBY_XTAL_ENW<'a> {
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
pub struct _SYSON_PMOPT_DSTBY_EN_SOCW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_PMOPT_DSTBY_EN_SOCW<'a> {
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
pub struct _SYSON_PMOPT_DSTBY_EN_PWMW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_PMOPT_DSTBY_EN_PWMW<'a> {
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
pub struct _SYSON_PMOPT_DSTBY_EN_SWRW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_PMOPT_DSTBY_EN_SWRW<'a> {
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
pub struct _SYSON_PMOPT_DSTBY_LPLDO_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_PMOPT_DSTBY_LPLDO_SELW<'a> {
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
    #[doc = "Bit 30"]
    #[inline]
    pub fn syson_pmopt_norm_sysclk_sel(&self) -> SYSON_PMOPT_NORM_SYSCLK_SELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_PMOPT_NORM_SYSCLK_SELR { bits }
    }
    #[doc = "Bit 29"]
    #[inline]
    pub fn syson_pmopt_norm_syspll_en(&self) -> SYSON_PMOPT_NORM_SYSPLL_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_PMOPT_NORM_SYSPLL_ENR { bits }
    }
    #[doc = "Bit 28"]
    #[inline]
    pub fn syson_pmopt_norm_xtal_en(&self) -> SYSON_PMOPT_NORM_XTAL_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_PMOPT_NORM_XTAL_ENR { bits }
    }
    #[doc = "Bit 27"]
    #[inline]
    pub fn syson_pmopt_norm_en_soc(&self) -> SYSON_PMOPT_NORM_EN_SOCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_PMOPT_NORM_EN_SOCR { bits }
    }
    #[doc = "Bit 26"]
    #[inline]
    pub fn syson_pmopt_norm_en_pwm(&self) -> SYSON_PMOPT_NORM_EN_PWMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_PMOPT_NORM_EN_PWMR { bits }
    }
    #[doc = "Bit 25"]
    #[inline]
    pub fn syson_pmopt_norm_en_swr(&self) -> SYSON_PMOPT_NORM_EN_SWRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_PMOPT_NORM_EN_SWRR { bits }
    }
    #[doc = "Bit 24"]
    #[inline]
    pub fn syson_pmopt_norm_lpldo_sel(&self) -> SYSON_PMOPT_NORM_LPLDO_SELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_PMOPT_NORM_LPLDO_SELR { bits }
    }
    #[doc = "Bit 22"]
    #[inline]
    pub fn syson_pmopt_snz_sysclk_sel(&self) -> SYSON_PMOPT_SNZ_SYSCLK_SELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_PMOPT_SNZ_SYSCLK_SELR { bits }
    }
    #[doc = "Bit 21"]
    #[inline]
    pub fn syson_pmopt_snz_syspll_en(&self) -> SYSON_PMOPT_SNZ_SYSPLL_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_PMOPT_SNZ_SYSPLL_ENR { bits }
    }
    #[doc = "Bit 20"]
    #[inline]
    pub fn syson_pmopt_snz_xtal_en(&self) -> SYSON_PMOPT_SNZ_XTAL_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_PMOPT_SNZ_XTAL_ENR { bits }
    }
    #[doc = "Bit 19"]
    #[inline]
    pub fn syson_pmopt_snz_en_soc(&self) -> SYSON_PMOPT_SNZ_EN_SOCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_PMOPT_SNZ_EN_SOCR { bits }
    }
    #[doc = "Bit 18"]
    #[inline]
    pub fn syson_pmopt_snz_en_pwm(&self) -> SYSON_PMOPT_SNZ_EN_PWMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_PMOPT_SNZ_EN_PWMR { bits }
    }
    #[doc = "Bit 17"]
    #[inline]
    pub fn syson_pmopt_snz_en_swr(&self) -> SYSON_PMOPT_SNZ_EN_SWRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_PMOPT_SNZ_EN_SWRR { bits }
    }
    #[doc = "Bit 16"]
    #[inline]
    pub fn syson_pmopt_snz_lpldo_sel(&self) -> SYSON_PMOPT_SNZ_LPLDO_SELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_PMOPT_SNZ_LPLDO_SELR { bits }
    }
    #[doc = "Bit 14"]
    #[inline]
    pub fn syson_pmopt_slp_sysclk_sel(&self) -> SYSON_PMOPT_SLP_SYSCLK_SELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_PMOPT_SLP_SYSCLK_SELR { bits }
    }
    #[doc = "Bit 13"]
    #[inline]
    pub fn syson_pmopt_slp_syspll_en(&self) -> SYSON_PMOPT_SLP_SYSPLL_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_PMOPT_SLP_SYSPLL_ENR { bits }
    }
    #[doc = "Bit 12"]
    #[inline]
    pub fn syson_pmopt_slp_xtal_en(&self) -> SYSON_PMOPT_SLP_XTAL_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_PMOPT_SLP_XTAL_ENR { bits }
    }
    #[doc = "Bit 11"]
    #[inline]
    pub fn syson_pmopt_slp_en_soc(&self) -> SYSON_PMOPT_SLP_EN_SOCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_PMOPT_SLP_EN_SOCR { bits }
    }
    #[doc = "Bit 10"]
    #[inline]
    pub fn syson_pmopt_slp_en_pwm(&self) -> SYSON_PMOPT_SLP_EN_PWMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_PMOPT_SLP_EN_PWMR { bits }
    }
    #[doc = "Bit 9"]
    #[inline]
    pub fn syson_pmopt_slp_en_swr(&self) -> SYSON_PMOPT_SLP_EN_SWRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_PMOPT_SLP_EN_SWRR { bits }
    }
    #[doc = "Bit 8"]
    #[inline]
    pub fn syson_pmopt_slp_lpldo_sel(&self) -> SYSON_PMOPT_SLP_LPLDO_SELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_PMOPT_SLP_LPLDO_SELR { bits }
    }
    #[doc = "Bit 6"]
    #[inline]
    pub fn syson_pmopt_dstby_sysclk_sel(&self) -> SYSON_PMOPT_DSTBY_SYSCLK_SELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_PMOPT_DSTBY_SYSCLK_SELR { bits }
    }
    #[doc = "Bit 5"]
    #[inline]
    pub fn syson_pmopt_dstby_syspll_en(&self) -> SYSON_PMOPT_DSTBY_SYSPLL_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_PMOPT_DSTBY_SYSPLL_ENR { bits }
    }
    #[doc = "Bit 4"]
    #[inline]
    pub fn syson_pmopt_dstby_xtal_en(&self) -> SYSON_PMOPT_DSTBY_XTAL_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_PMOPT_DSTBY_XTAL_ENR { bits }
    }
    #[doc = "Bit 3"]
    #[inline]
    pub fn syson_pmopt_dstby_en_soc(&self) -> SYSON_PMOPT_DSTBY_EN_SOCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_PMOPT_DSTBY_EN_SOCR { bits }
    }
    #[doc = "Bit 2"]
    #[inline]
    pub fn syson_pmopt_dstby_en_pwm(&self) -> SYSON_PMOPT_DSTBY_EN_PWMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_PMOPT_DSTBY_EN_PWMR { bits }
    }
    #[doc = "Bit 1"]
    #[inline]
    pub fn syson_pmopt_dstby_en_swr(&self) -> SYSON_PMOPT_DSTBY_EN_SWRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_PMOPT_DSTBY_EN_SWRR { bits }
    }
    #[doc = "Bit 0"]
    #[inline]
    pub fn syson_pmopt_dstby_lpldo_sel(&self) -> SYSON_PMOPT_DSTBY_LPLDO_SELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_PMOPT_DSTBY_LPLDO_SELR { bits }
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
    #[doc = "Bit 30"]
    #[inline]
    pub fn syson_pmopt_norm_sysclk_sel(&mut self) -> _SYSON_PMOPT_NORM_SYSCLK_SELW {
        _SYSON_PMOPT_NORM_SYSCLK_SELW { w: self }
    }
    #[doc = "Bit 29"]
    #[inline]
    pub fn syson_pmopt_norm_syspll_en(&mut self) -> _SYSON_PMOPT_NORM_SYSPLL_ENW {
        _SYSON_PMOPT_NORM_SYSPLL_ENW { w: self }
    }
    #[doc = "Bit 28"]
    #[inline]
    pub fn syson_pmopt_norm_xtal_en(&mut self) -> _SYSON_PMOPT_NORM_XTAL_ENW {
        _SYSON_PMOPT_NORM_XTAL_ENW { w: self }
    }
    #[doc = "Bit 27"]
    #[inline]
    pub fn syson_pmopt_norm_en_soc(&mut self) -> _SYSON_PMOPT_NORM_EN_SOCW {
        _SYSON_PMOPT_NORM_EN_SOCW { w: self }
    }
    #[doc = "Bit 26"]
    #[inline]
    pub fn syson_pmopt_norm_en_pwm(&mut self) -> _SYSON_PMOPT_NORM_EN_PWMW {
        _SYSON_PMOPT_NORM_EN_PWMW { w: self }
    }
    #[doc = "Bit 25"]
    #[inline]
    pub fn syson_pmopt_norm_en_swr(&mut self) -> _SYSON_PMOPT_NORM_EN_SWRW {
        _SYSON_PMOPT_NORM_EN_SWRW { w: self }
    }
    #[doc = "Bit 24"]
    #[inline]
    pub fn syson_pmopt_norm_lpldo_sel(&mut self) -> _SYSON_PMOPT_NORM_LPLDO_SELW {
        _SYSON_PMOPT_NORM_LPLDO_SELW { w: self }
    }
    #[doc = "Bit 22"]
    #[inline]
    pub fn syson_pmopt_snz_sysclk_sel(&mut self) -> _SYSON_PMOPT_SNZ_SYSCLK_SELW {
        _SYSON_PMOPT_SNZ_SYSCLK_SELW { w: self }
    }
    #[doc = "Bit 21"]
    #[inline]
    pub fn syson_pmopt_snz_syspll_en(&mut self) -> _SYSON_PMOPT_SNZ_SYSPLL_ENW {
        _SYSON_PMOPT_SNZ_SYSPLL_ENW { w: self }
    }
    #[doc = "Bit 20"]
    #[inline]
    pub fn syson_pmopt_snz_xtal_en(&mut self) -> _SYSON_PMOPT_SNZ_XTAL_ENW {
        _SYSON_PMOPT_SNZ_XTAL_ENW { w: self }
    }
    #[doc = "Bit 19"]
    #[inline]
    pub fn syson_pmopt_snz_en_soc(&mut self) -> _SYSON_PMOPT_SNZ_EN_SOCW {
        _SYSON_PMOPT_SNZ_EN_SOCW { w: self }
    }
    #[doc = "Bit 18"]
    #[inline]
    pub fn syson_pmopt_snz_en_pwm(&mut self) -> _SYSON_PMOPT_SNZ_EN_PWMW {
        _SYSON_PMOPT_SNZ_EN_PWMW { w: self }
    }
    #[doc = "Bit 17"]
    #[inline]
    pub fn syson_pmopt_snz_en_swr(&mut self) -> _SYSON_PMOPT_SNZ_EN_SWRW {
        _SYSON_PMOPT_SNZ_EN_SWRW { w: self }
    }
    #[doc = "Bit 16"]
    #[inline]
    pub fn syson_pmopt_snz_lpldo_sel(&mut self) -> _SYSON_PMOPT_SNZ_LPLDO_SELW {
        _SYSON_PMOPT_SNZ_LPLDO_SELW { w: self }
    }
    #[doc = "Bit 14"]
    #[inline]
    pub fn syson_pmopt_slp_sysclk_sel(&mut self) -> _SYSON_PMOPT_SLP_SYSCLK_SELW {
        _SYSON_PMOPT_SLP_SYSCLK_SELW { w: self }
    }
    #[doc = "Bit 13"]
    #[inline]
    pub fn syson_pmopt_slp_syspll_en(&mut self) -> _SYSON_PMOPT_SLP_SYSPLL_ENW {
        _SYSON_PMOPT_SLP_SYSPLL_ENW { w: self }
    }
    #[doc = "Bit 12"]
    #[inline]
    pub fn syson_pmopt_slp_xtal_en(&mut self) -> _SYSON_PMOPT_SLP_XTAL_ENW {
        _SYSON_PMOPT_SLP_XTAL_ENW { w: self }
    }
    #[doc = "Bit 11"]
    #[inline]
    pub fn syson_pmopt_slp_en_soc(&mut self) -> _SYSON_PMOPT_SLP_EN_SOCW {
        _SYSON_PMOPT_SLP_EN_SOCW { w: self }
    }
    #[doc = "Bit 10"]
    #[inline]
    pub fn syson_pmopt_slp_en_pwm(&mut self) -> _SYSON_PMOPT_SLP_EN_PWMW {
        _SYSON_PMOPT_SLP_EN_PWMW { w: self }
    }
    #[doc = "Bit 9"]
    #[inline]
    pub fn syson_pmopt_slp_en_swr(&mut self) -> _SYSON_PMOPT_SLP_EN_SWRW {
        _SYSON_PMOPT_SLP_EN_SWRW { w: self }
    }
    #[doc = "Bit 8"]
    #[inline]
    pub fn syson_pmopt_slp_lpldo_sel(&mut self) -> _SYSON_PMOPT_SLP_LPLDO_SELW {
        _SYSON_PMOPT_SLP_LPLDO_SELW { w: self }
    }
    #[doc = "Bit 6"]
    #[inline]
    pub fn syson_pmopt_dstby_sysclk_sel(&mut self) -> _SYSON_PMOPT_DSTBY_SYSCLK_SELW {
        _SYSON_PMOPT_DSTBY_SYSCLK_SELW { w: self }
    }
    #[doc = "Bit 5"]
    #[inline]
    pub fn syson_pmopt_dstby_syspll_en(&mut self) -> _SYSON_PMOPT_DSTBY_SYSPLL_ENW {
        _SYSON_PMOPT_DSTBY_SYSPLL_ENW { w: self }
    }
    #[doc = "Bit 4"]
    #[inline]
    pub fn syson_pmopt_dstby_xtal_en(&mut self) -> _SYSON_PMOPT_DSTBY_XTAL_ENW {
        _SYSON_PMOPT_DSTBY_XTAL_ENW { w: self }
    }
    #[doc = "Bit 3"]
    #[inline]
    pub fn syson_pmopt_dstby_en_soc(&mut self) -> _SYSON_PMOPT_DSTBY_EN_SOCW {
        _SYSON_PMOPT_DSTBY_EN_SOCW { w: self }
    }
    #[doc = "Bit 2"]
    #[inline]
    pub fn syson_pmopt_dstby_en_pwm(&mut self) -> _SYSON_PMOPT_DSTBY_EN_PWMW {
        _SYSON_PMOPT_DSTBY_EN_PWMW { w: self }
    }
    #[doc = "Bit 1"]
    #[inline]
    pub fn syson_pmopt_dstby_en_swr(&mut self) -> _SYSON_PMOPT_DSTBY_EN_SWRW {
        _SYSON_PMOPT_DSTBY_EN_SWRW { w: self }
    }
    #[doc = "Bit 0"]
    #[inline]
    pub fn syson_pmopt_dstby_lpldo_sel(&mut self) -> _SYSON_PMOPT_DSTBY_LPLDO_SELW {
        _SYSON_PMOPT_DSTBY_LPLDO_SELW { w: self }
    }
}
