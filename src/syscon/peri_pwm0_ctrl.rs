#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PERI_PWM0_CTRL {
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
pub struct PERI_PWM0_ENR {
    bits: bool,
}
impl PERI_PWM0_ENR {
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
pub struct PERI_PWM0_GT_SELR {
    bits: u8,
}
impl PERI_PWM0_GT_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PERI_PWM0_DUTYR {
    bits: u16,
}
impl PERI_PWM0_DUTYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PERI_PWM0_PERIODR {
    bits: u16,
}
impl PERI_PWM0_PERIODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PERI_PWM0_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PERI_PWM0_ENW<'a> {
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
pub struct _PERI_PWM0_GT_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _PERI_PWM0_GT_SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PERI_PWM0_DUTYW<'a> {
    w: &'a mut W,
}
impl<'a> _PERI_PWM0_DUTYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PERI_PWM0_PERIODW<'a> {
    w: &'a mut W,
}
impl<'a> _PERI_PWM0_PERIODW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
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
    #[doc = "Bit 31"]
    #[inline]
    pub fn peri_pwm0_en(&self) -> PERI_PWM0_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PERI_PWM0_ENR { bits }
    }
    #[doc = "Bits 24:27"]
    #[inline]
    pub fn peri_pwm0_gt_sel(&self) -> PERI_PWM0_GT_SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PERI_PWM0_GT_SELR { bits }
    }
    #[doc = "Bits 12:21"]
    #[inline]
    pub fn peri_pwm0_duty(&self) -> PERI_PWM0_DUTYR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        PERI_PWM0_DUTYR { bits }
    }
    #[doc = "Bits 0:9"]
    #[inline]
    pub fn peri_pwm0_period(&self) -> PERI_PWM0_PERIODR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        PERI_PWM0_PERIODR { bits }
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
    pub fn peri_pwm0_en(&mut self) -> _PERI_PWM0_ENW {
        _PERI_PWM0_ENW { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline]
    pub fn peri_pwm0_gt_sel(&mut self) -> _PERI_PWM0_GT_SELW {
        _PERI_PWM0_GT_SELW { w: self }
    }
    #[doc = "Bits 12:21"]
    #[inline]
    pub fn peri_pwm0_duty(&mut self) -> _PERI_PWM0_DUTYW {
        _PERI_PWM0_DUTYW { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline]
    pub fn peri_pwm0_period(&mut self) -> _PERI_PWM0_PERIODW {
        _PERI_PWM0_PERIODW { w: self }
    }
}
