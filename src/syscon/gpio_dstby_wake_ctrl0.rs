#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPIO_DSTBY_WAKE_CTRL0 {
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
pub struct SYS_GPIOE3_WEVENT_STSR {
    bits: bool,
}
impl SYS_GPIOE3_WEVENT_STSR {
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
pub struct SYS_GPIOD5_WEVENT_STSR {
    bits: bool,
}
impl SYS_GPIOD5_WEVENT_STSR {
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
pub struct SYS_GPIOC7_WEVENT_STSR {
    bits: bool,
}
impl SYS_GPIOC7_WEVENT_STSR {
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
pub struct SYS_GPIOA5_WEVENT_STSR {
    bits: bool,
}
impl SYS_GPIOA5_WEVENT_STSR {
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
pub struct SYS_GPIO_GPE3_PULL_CTRL_ENR {
    bits: bool,
}
impl SYS_GPIO_GPE3_PULL_CTRL_ENR {
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
pub struct SYS_GPIO_GPD5_PULL_CTRL_ENR {
    bits: bool,
}
impl SYS_GPIO_GPD5_PULL_CTRL_ENR {
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
pub struct SYS_GPIO_GPC7_PULL_CTRL_ENR {
    bits: bool,
}
impl SYS_GPIO_GPC7_PULL_CTRL_ENR {
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
pub struct SYS_GPIO_GPA5_PULL_CTRL_ENR {
    bits: bool,
}
impl SYS_GPIO_GPA5_PULL_CTRL_ENR {
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
pub struct SYS_GPIOE3_WINT_MODER {
    bits: bool,
}
impl SYS_GPIOE3_WINT_MODER {
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
pub struct SYS_GPIOD5_WINT_MODER {
    bits: bool,
}
impl SYS_GPIOD5_WINT_MODER {
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
pub struct SYS_GPIOC7_WINT_MODER {
    bits: bool,
}
impl SYS_GPIOC7_WINT_MODER {
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
pub struct SYS_GPIOA5_WINT_MODER {
    bits: bool,
}
impl SYS_GPIOA5_WINT_MODER {
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
pub struct SYS_GPIOE3_PIN_ENR {
    bits: bool,
}
impl SYS_GPIOE3_PIN_ENR {
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
pub struct SYS_GPIOD5_PIN_ENR {
    bits: bool,
}
impl SYS_GPIOD5_PIN_ENR {
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
pub struct SYS_GPIOC7_PIN_ENR {
    bits: bool,
}
impl SYS_GPIOC7_PIN_ENR {
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
pub struct SYS_GPIOA5_PIN_ENR {
    bits: bool,
}
impl SYS_GPIOA5_PIN_ENR {
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
pub struct _SYS_GPIOE3_WEVENT_STSW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_GPIOE3_WEVENT_STSW<'a> {
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
pub struct _SYS_GPIOD5_WEVENT_STSW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_GPIOD5_WEVENT_STSW<'a> {
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
pub struct _SYS_GPIOC7_WEVENT_STSW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_GPIOC7_WEVENT_STSW<'a> {
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
pub struct _SYS_GPIOA5_WEVENT_STSW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_GPIOA5_WEVENT_STSW<'a> {
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
pub struct _SYS_GPIO_GPE3_PULL_CTRL_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_GPIO_GPE3_PULL_CTRL_ENW<'a> {
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
pub struct _SYS_GPIO_GPD5_PULL_CTRL_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_GPIO_GPD5_PULL_CTRL_ENW<'a> {
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
pub struct _SYS_GPIO_GPC7_PULL_CTRL_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_GPIO_GPC7_PULL_CTRL_ENW<'a> {
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
pub struct _SYS_GPIO_GPA5_PULL_CTRL_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_GPIO_GPA5_PULL_CTRL_ENW<'a> {
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
pub struct _SYS_GPIOE3_WINT_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_GPIOE3_WINT_MODEW<'a> {
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
pub struct _SYS_GPIOD5_WINT_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_GPIOD5_WINT_MODEW<'a> {
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
pub struct _SYS_GPIOC7_WINT_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_GPIOC7_WINT_MODEW<'a> {
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
pub struct _SYS_GPIOA5_WINT_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_GPIOA5_WINT_MODEW<'a> {
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
pub struct _SYS_GPIOE3_PIN_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_GPIOE3_PIN_ENW<'a> {
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
pub struct _SYS_GPIOD5_PIN_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_GPIOD5_PIN_ENW<'a> {
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
pub struct _SYS_GPIOC7_PIN_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_GPIOC7_PIN_ENW<'a> {
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
pub struct _SYS_GPIOA5_PIN_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_GPIOA5_PIN_ENW<'a> {
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
    #[doc = "Bit 27"]
    #[inline]
    pub fn sys_gpioe3_wevent_sts(&self) -> SYS_GPIOE3_WEVENT_STSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_GPIOE3_WEVENT_STSR { bits }
    }
    #[doc = "Bit 26"]
    #[inline]
    pub fn sys_gpiod5_wevent_sts(&self) -> SYS_GPIOD5_WEVENT_STSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_GPIOD5_WEVENT_STSR { bits }
    }
    #[doc = "Bit 25"]
    #[inline]
    pub fn sys_gpioc7_wevent_sts(&self) -> SYS_GPIOC7_WEVENT_STSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_GPIOC7_WEVENT_STSR { bits }
    }
    #[doc = "Bit 24"]
    #[inline]
    pub fn sys_gpioa5_wevent_sts(&self) -> SYS_GPIOA5_WEVENT_STSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_GPIOA5_WEVENT_STSR { bits }
    }
    #[doc = "Bit 19"]
    #[inline]
    pub fn sys_gpio_gpe3_pull_ctrl_en(&self) -> SYS_GPIO_GPE3_PULL_CTRL_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_GPIO_GPE3_PULL_CTRL_ENR { bits }
    }
    #[doc = "Bit 18"]
    #[inline]
    pub fn sys_gpio_gpd5_pull_ctrl_en(&self) -> SYS_GPIO_GPD5_PULL_CTRL_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_GPIO_GPD5_PULL_CTRL_ENR { bits }
    }
    #[doc = "Bit 17"]
    #[inline]
    pub fn sys_gpio_gpc7_pull_ctrl_en(&self) -> SYS_GPIO_GPC7_PULL_CTRL_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_GPIO_GPC7_PULL_CTRL_ENR { bits }
    }
    #[doc = "Bit 16"]
    #[inline]
    pub fn sys_gpio_gpa5_pull_ctrl_en(&self) -> SYS_GPIO_GPA5_PULL_CTRL_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_GPIO_GPA5_PULL_CTRL_ENR { bits }
    }
    #[doc = "Bit 11"]
    #[inline]
    pub fn sys_gpioe3_wint_mode(&self) -> SYS_GPIOE3_WINT_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_GPIOE3_WINT_MODER { bits }
    }
    #[doc = "Bit 10"]
    #[inline]
    pub fn sys_gpiod5_wint_mode(&self) -> SYS_GPIOD5_WINT_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_GPIOD5_WINT_MODER { bits }
    }
    #[doc = "Bit 9"]
    #[inline]
    pub fn sys_gpioc7_wint_mode(&self) -> SYS_GPIOC7_WINT_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_GPIOC7_WINT_MODER { bits }
    }
    #[doc = "Bit 8"]
    #[inline]
    pub fn sys_gpioa5_wint_mode(&self) -> SYS_GPIOA5_WINT_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_GPIOA5_WINT_MODER { bits }
    }
    #[doc = "Bit 3"]
    #[inline]
    pub fn sys_gpioe3_pin_en(&self) -> SYS_GPIOE3_PIN_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_GPIOE3_PIN_ENR { bits }
    }
    #[doc = "Bit 2"]
    #[inline]
    pub fn sys_gpiod5_pin_en(&self) -> SYS_GPIOD5_PIN_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_GPIOD5_PIN_ENR { bits }
    }
    #[doc = "Bit 1"]
    #[inline]
    pub fn sys_gpioc7_pin_en(&self) -> SYS_GPIOC7_PIN_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_GPIOC7_PIN_ENR { bits }
    }
    #[doc = "Bit 0"]
    #[inline]
    pub fn sys_gpioa5_pin_en(&self) -> SYS_GPIOA5_PIN_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_GPIOA5_PIN_ENR { bits }
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
    #[doc = "Bit 27"]
    #[inline]
    pub fn sys_gpioe3_wevent_sts(&mut self) -> _SYS_GPIOE3_WEVENT_STSW {
        _SYS_GPIOE3_WEVENT_STSW { w: self }
    }
    #[doc = "Bit 26"]
    #[inline]
    pub fn sys_gpiod5_wevent_sts(&mut self) -> _SYS_GPIOD5_WEVENT_STSW {
        _SYS_GPIOD5_WEVENT_STSW { w: self }
    }
    #[doc = "Bit 25"]
    #[inline]
    pub fn sys_gpioc7_wevent_sts(&mut self) -> _SYS_GPIOC7_WEVENT_STSW {
        _SYS_GPIOC7_WEVENT_STSW { w: self }
    }
    #[doc = "Bit 24"]
    #[inline]
    pub fn sys_gpioa5_wevent_sts(&mut self) -> _SYS_GPIOA5_WEVENT_STSW {
        _SYS_GPIOA5_WEVENT_STSW { w: self }
    }
    #[doc = "Bit 19"]
    #[inline]
    pub fn sys_gpio_gpe3_pull_ctrl_en(&mut self) -> _SYS_GPIO_GPE3_PULL_CTRL_ENW {
        _SYS_GPIO_GPE3_PULL_CTRL_ENW { w: self }
    }
    #[doc = "Bit 18"]
    #[inline]
    pub fn sys_gpio_gpd5_pull_ctrl_en(&mut self) -> _SYS_GPIO_GPD5_PULL_CTRL_ENW {
        _SYS_GPIO_GPD5_PULL_CTRL_ENW { w: self }
    }
    #[doc = "Bit 17"]
    #[inline]
    pub fn sys_gpio_gpc7_pull_ctrl_en(&mut self) -> _SYS_GPIO_GPC7_PULL_CTRL_ENW {
        _SYS_GPIO_GPC7_PULL_CTRL_ENW { w: self }
    }
    #[doc = "Bit 16"]
    #[inline]
    pub fn sys_gpio_gpa5_pull_ctrl_en(&mut self) -> _SYS_GPIO_GPA5_PULL_CTRL_ENW {
        _SYS_GPIO_GPA5_PULL_CTRL_ENW { w: self }
    }
    #[doc = "Bit 11"]
    #[inline]
    pub fn sys_gpioe3_wint_mode(&mut self) -> _SYS_GPIOE3_WINT_MODEW {
        _SYS_GPIOE3_WINT_MODEW { w: self }
    }
    #[doc = "Bit 10"]
    #[inline]
    pub fn sys_gpiod5_wint_mode(&mut self) -> _SYS_GPIOD5_WINT_MODEW {
        _SYS_GPIOD5_WINT_MODEW { w: self }
    }
    #[doc = "Bit 9"]
    #[inline]
    pub fn sys_gpioc7_wint_mode(&mut self) -> _SYS_GPIOC7_WINT_MODEW {
        _SYS_GPIOC7_WINT_MODEW { w: self }
    }
    #[doc = "Bit 8"]
    #[inline]
    pub fn sys_gpioa5_wint_mode(&mut self) -> _SYS_GPIOA5_WINT_MODEW {
        _SYS_GPIOA5_WINT_MODEW { w: self }
    }
    #[doc = "Bit 3"]
    #[inline]
    pub fn sys_gpioe3_pin_en(&mut self) -> _SYS_GPIOE3_PIN_ENW {
        _SYS_GPIOE3_PIN_ENW { w: self }
    }
    #[doc = "Bit 2"]
    #[inline]
    pub fn sys_gpiod5_pin_en(&mut self) -> _SYS_GPIOD5_PIN_ENW {
        _SYS_GPIOD5_PIN_ENW { w: self }
    }
    #[doc = "Bit 1"]
    #[inline]
    pub fn sys_gpioc7_pin_en(&mut self) -> _SYS_GPIOC7_PIN_ENW {
        _SYS_GPIOC7_PIN_ENW { w: self }
    }
    #[doc = "Bit 0"]
    #[inline]
    pub fn sys_gpioa5_pin_en(&mut self) -> _SYS_GPIOA5_PIN_ENW {
        _SYS_GPIOA5_PIN_ENW { w: self }
    }
}
