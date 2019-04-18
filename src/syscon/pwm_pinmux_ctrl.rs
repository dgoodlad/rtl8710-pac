#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PWM_PINMUX_CTRL {
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
pub struct ETE3_PIN_SELR {
    bits: u8,
}
impl ETE3_PIN_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ETE3_PIN_ENR {
    bits: bool,
}
impl ETE3_PIN_ENR {
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
pub struct ETE2_PIN_SELR {
    bits: u8,
}
impl ETE2_PIN_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ETE2_PIN_ENR {
    bits: bool,
}
impl ETE2_PIN_ENR {
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
pub struct ETE1_PIN_SELR {
    bits: u8,
}
impl ETE1_PIN_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ETE1_PIN_ENR {
    bits: bool,
}
impl ETE1_PIN_ENR {
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
pub struct ETE0_PIN_SELR {
    bits: u8,
}
impl ETE0_PIN_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ETE0_PIN_ENR {
    bits: bool,
}
impl ETE0_PIN_ENR {
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
pub struct PWM3_PIN_SELR {
    bits: u8,
}
impl PWM3_PIN_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PWM3_PIN_ENR {
    bits: bool,
}
impl PWM3_PIN_ENR {
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
pub struct PWM2_PIN_SELR {
    bits: u8,
}
impl PWM2_PIN_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PWM2_PIN_ENR {
    bits: bool,
}
impl PWM2_PIN_ENR {
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
pub struct PWM1_PIN_SELR {
    bits: u8,
}
impl PWM1_PIN_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PWM1_PIN_ENR {
    bits: bool,
}
impl PWM1_PIN_ENR {
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
pub struct PWM0_PIN_SELR {
    bits: u8,
}
impl PWM0_PIN_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PWM0_PIN_ENR {
    bits: bool,
}
impl PWM0_PIN_ENR {
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
pub struct _ETE3_PIN_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _ETE3_PIN_SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ETE3_PIN_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ETE3_PIN_ENW<'a> {
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
pub struct _ETE2_PIN_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _ETE2_PIN_SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ETE2_PIN_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ETE2_PIN_ENW<'a> {
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
pub struct _ETE1_PIN_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _ETE1_PIN_SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ETE1_PIN_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ETE1_PIN_ENW<'a> {
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
pub struct _ETE0_PIN_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _ETE0_PIN_SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ETE0_PIN_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ETE0_PIN_ENW<'a> {
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
pub struct _PWM3_PIN_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM3_PIN_SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PWM3_PIN_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM3_PIN_ENW<'a> {
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
pub struct _PWM2_PIN_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM2_PIN_SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PWM2_PIN_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM2_PIN_ENW<'a> {
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
pub struct _PWM1_PIN_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM1_PIN_SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PWM1_PIN_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM1_PIN_ENW<'a> {
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
pub struct _PWM0_PIN_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM0_PIN_SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PWM0_PIN_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM0_PIN_ENW<'a> {
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
    #[doc = "Bits 29:30"]
    #[inline]
    pub fn ete3_pin_sel(&self) -> ETE3_PIN_SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ETE3_PIN_SELR { bits }
    }
    #[doc = "Bit 28"]
    #[inline]
    pub fn ete3_pin_en(&self) -> ETE3_PIN_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ETE3_PIN_ENR { bits }
    }
    #[doc = "Bits 25:26"]
    #[inline]
    pub fn ete2_pin_sel(&self) -> ETE2_PIN_SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ETE2_PIN_SELR { bits }
    }
    #[doc = "Bit 24"]
    #[inline]
    pub fn ete2_pin_en(&self) -> ETE2_PIN_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ETE2_PIN_ENR { bits }
    }
    #[doc = "Bits 21:22"]
    #[inline]
    pub fn ete1_pin_sel(&self) -> ETE1_PIN_SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ETE1_PIN_SELR { bits }
    }
    #[doc = "Bit 20"]
    #[inline]
    pub fn ete1_pin_en(&self) -> ETE1_PIN_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ETE1_PIN_ENR { bits }
    }
    #[doc = "Bits 17:18"]
    #[inline]
    pub fn ete0_pin_sel(&self) -> ETE0_PIN_SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ETE0_PIN_SELR { bits }
    }
    #[doc = "Bit 16"]
    #[inline]
    pub fn ete0_pin_en(&self) -> ETE0_PIN_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ETE0_PIN_ENR { bits }
    }
    #[doc = "Bits 13:14"]
    #[inline]
    pub fn pwm3_pin_sel(&self) -> PWM3_PIN_SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PWM3_PIN_SELR { bits }
    }
    #[doc = "Bit 12"]
    #[inline]
    pub fn pwm3_pin_en(&self) -> PWM3_PIN_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PWM3_PIN_ENR { bits }
    }
    #[doc = "Bits 9:10"]
    #[inline]
    pub fn pwm2_pin_sel(&self) -> PWM2_PIN_SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PWM2_PIN_SELR { bits }
    }
    #[doc = "Bit 8"]
    #[inline]
    pub fn pwm2_pin_en(&self) -> PWM2_PIN_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PWM2_PIN_ENR { bits }
    }
    #[doc = "Bits 5:6"]
    #[inline]
    pub fn pwm1_pin_sel(&self) -> PWM1_PIN_SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PWM1_PIN_SELR { bits }
    }
    #[doc = "Bit 4"]
    #[inline]
    pub fn pwm1_pin_en(&self) -> PWM1_PIN_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PWM1_PIN_ENR { bits }
    }
    #[doc = "Bits 1:2"]
    #[inline]
    pub fn pwm0_pin_sel(&self) -> PWM0_PIN_SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PWM0_PIN_SELR { bits }
    }
    #[doc = "Bit 0"]
    #[inline]
    pub fn pwm0_pin_en(&self) -> PWM0_PIN_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PWM0_PIN_ENR { bits }
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
    #[doc = "Bits 29:30"]
    #[inline]
    pub fn ete3_pin_sel(&mut self) -> _ETE3_PIN_SELW {
        _ETE3_PIN_SELW { w: self }
    }
    #[doc = "Bit 28"]
    #[inline]
    pub fn ete3_pin_en(&mut self) -> _ETE3_PIN_ENW {
        _ETE3_PIN_ENW { w: self }
    }
    #[doc = "Bits 25:26"]
    #[inline]
    pub fn ete2_pin_sel(&mut self) -> _ETE2_PIN_SELW {
        _ETE2_PIN_SELW { w: self }
    }
    #[doc = "Bit 24"]
    #[inline]
    pub fn ete2_pin_en(&mut self) -> _ETE2_PIN_ENW {
        _ETE2_PIN_ENW { w: self }
    }
    #[doc = "Bits 21:22"]
    #[inline]
    pub fn ete1_pin_sel(&mut self) -> _ETE1_PIN_SELW {
        _ETE1_PIN_SELW { w: self }
    }
    #[doc = "Bit 20"]
    #[inline]
    pub fn ete1_pin_en(&mut self) -> _ETE1_PIN_ENW {
        _ETE1_PIN_ENW { w: self }
    }
    #[doc = "Bits 17:18"]
    #[inline]
    pub fn ete0_pin_sel(&mut self) -> _ETE0_PIN_SELW {
        _ETE0_PIN_SELW { w: self }
    }
    #[doc = "Bit 16"]
    #[inline]
    pub fn ete0_pin_en(&mut self) -> _ETE0_PIN_ENW {
        _ETE0_PIN_ENW { w: self }
    }
    #[doc = "Bits 13:14"]
    #[inline]
    pub fn pwm3_pin_sel(&mut self) -> _PWM3_PIN_SELW {
        _PWM3_PIN_SELW { w: self }
    }
    #[doc = "Bit 12"]
    #[inline]
    pub fn pwm3_pin_en(&mut self) -> _PWM3_PIN_ENW {
        _PWM3_PIN_ENW { w: self }
    }
    #[doc = "Bits 9:10"]
    #[inline]
    pub fn pwm2_pin_sel(&mut self) -> _PWM2_PIN_SELW {
        _PWM2_PIN_SELW { w: self }
    }
    #[doc = "Bit 8"]
    #[inline]
    pub fn pwm2_pin_en(&mut self) -> _PWM2_PIN_ENW {
        _PWM2_PIN_ENW { w: self }
    }
    #[doc = "Bits 5:6"]
    #[inline]
    pub fn pwm1_pin_sel(&mut self) -> _PWM1_PIN_SELW {
        _PWM1_PIN_SELW { w: self }
    }
    #[doc = "Bit 4"]
    #[inline]
    pub fn pwm1_pin_en(&mut self) -> _PWM1_PIN_ENW {
        _PWM1_PIN_ENW { w: self }
    }
    #[doc = "Bits 1:2"]
    #[inline]
    pub fn pwm0_pin_sel(&mut self) -> _PWM0_PIN_SELW {
        _PWM0_PIN_SELW { w: self }
    }
    #[doc = "Bit 0"]
    #[inline]
    pub fn pwm0_pin_en(&mut self) -> _PWM0_PIN_ENW {
        _PWM0_PIN_ENW { w: self }
    }
}
