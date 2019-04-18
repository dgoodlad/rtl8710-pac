#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPIO_PULL_CTRL5 {
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
pub struct GPIO_GPEA_PULL_CTRLR {
    bits: u8,
}
impl GPIO_GPEA_PULL_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GPIO_GPE9_PULL_CTRLR {
    bits: u8,
}
impl GPIO_GPE9_PULL_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GPIO_GPE8_PULL_CTRLR {
    bits: u8,
}
impl GPIO_GPE8_PULL_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GPIO_GPK7_PULL_CTRLR {
    bits: u8,
}
impl GPIO_GPK7_PULL_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GPIO_GPK5_PULL_CTRLR {
    bits: u8,
}
impl GPIO_GPK5_PULL_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GPIO_GPK4_PULL_CTRLR {
    bits: u8,
}
impl GPIO_GPK4_PULL_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GPIO_GPK3_PULL_CTRLR {
    bits: u8,
}
impl GPIO_GPK3_PULL_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GPIO_GPK2_PULL_CTRLR {
    bits: u8,
}
impl GPIO_GPK2_PULL_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GPIO_GPK1_PULL_CTRLR {
    bits: u8,
}
impl GPIO_GPK1_PULL_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GPIO_GPK0_PULL_CTRLR {
    bits: u8,
}
impl GPIO_GPK0_PULL_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_GPEA_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPEA_PULL_CTRLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_GPE9_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPE9_PULL_CTRLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_GPE8_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPE8_PULL_CTRLW<'a> {
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
pub struct _GPIO_GPK7_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPK7_PULL_CTRLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_GPK5_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPK5_PULL_CTRLW<'a> {
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
pub struct _GPIO_GPK4_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPK4_PULL_CTRLW<'a> {
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
pub struct _GPIO_GPK3_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPK3_PULL_CTRLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_GPK2_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPK2_PULL_CTRLW<'a> {
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
#[doc = r" Proxy"]
pub struct _GPIO_GPK1_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPK1_PULL_CTRLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_GPK0_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPK0_PULL_CTRLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 20:21"]
    #[inline]
    pub fn gpio_gpea_pull_ctrl(&self) -> GPIO_GPEA_PULL_CTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GPIO_GPEA_PULL_CTRLR { bits }
    }
    #[doc = "Bits 18:19"]
    #[inline]
    pub fn gpio_gpe9_pull_ctrl(&self) -> GPIO_GPE9_PULL_CTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GPIO_GPE9_PULL_CTRLR { bits }
    }
    #[doc = "Bits 16:17"]
    #[inline]
    pub fn gpio_gpe8_pull_ctrl(&self) -> GPIO_GPE8_PULL_CTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GPIO_GPE8_PULL_CTRLR { bits }
    }
    #[doc = "Bits 12:13"]
    #[inline]
    pub fn gpio_gpk7_pull_ctrl(&self) -> GPIO_GPK7_PULL_CTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GPIO_GPK7_PULL_CTRLR { bits }
    }
    #[doc = "Bits 10:11"]
    #[inline]
    pub fn gpio_gpk5_pull_ctrl(&self) -> GPIO_GPK5_PULL_CTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GPIO_GPK5_PULL_CTRLR { bits }
    }
    #[doc = "Bits 8:9"]
    #[inline]
    pub fn gpio_gpk4_pull_ctrl(&self) -> GPIO_GPK4_PULL_CTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GPIO_GPK4_PULL_CTRLR { bits }
    }
    #[doc = "Bits 6:7"]
    #[inline]
    pub fn gpio_gpk3_pull_ctrl(&self) -> GPIO_GPK3_PULL_CTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GPIO_GPK3_PULL_CTRLR { bits }
    }
    #[doc = "Bits 4:5"]
    #[inline]
    pub fn gpio_gpk2_pull_ctrl(&self) -> GPIO_GPK2_PULL_CTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GPIO_GPK2_PULL_CTRLR { bits }
    }
    #[doc = "Bits 2:3"]
    #[inline]
    pub fn gpio_gpk1_pull_ctrl(&self) -> GPIO_GPK1_PULL_CTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GPIO_GPK1_PULL_CTRLR { bits }
    }
    #[doc = "Bits 0:1"]
    #[inline]
    pub fn gpio_gpk0_pull_ctrl(&self) -> GPIO_GPK0_PULL_CTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GPIO_GPK0_PULL_CTRLR { bits }
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
    #[doc = "Bits 20:21"]
    #[inline]
    pub fn gpio_gpea_pull_ctrl(&mut self) -> _GPIO_GPEA_PULL_CTRLW {
        _GPIO_GPEA_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline]
    pub fn gpio_gpe9_pull_ctrl(&mut self) -> _GPIO_GPE9_PULL_CTRLW {
        _GPIO_GPE9_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline]
    pub fn gpio_gpe8_pull_ctrl(&mut self) -> _GPIO_GPE8_PULL_CTRLW {
        _GPIO_GPE8_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline]
    pub fn gpio_gpk7_pull_ctrl(&mut self) -> _GPIO_GPK7_PULL_CTRLW {
        _GPIO_GPK7_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline]
    pub fn gpio_gpk5_pull_ctrl(&mut self) -> _GPIO_GPK5_PULL_CTRLW {
        _GPIO_GPK5_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline]
    pub fn gpio_gpk4_pull_ctrl(&mut self) -> _GPIO_GPK4_PULL_CTRLW {
        _GPIO_GPK4_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline]
    pub fn gpio_gpk3_pull_ctrl(&mut self) -> _GPIO_GPK3_PULL_CTRLW {
        _GPIO_GPK3_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline]
    pub fn gpio_gpk2_pull_ctrl(&mut self) -> _GPIO_GPK2_PULL_CTRLW {
        _GPIO_GPK2_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline]
    pub fn gpio_gpk1_pull_ctrl(&mut self) -> _GPIO_GPK1_PULL_CTRLW {
        _GPIO_GPK1_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline]
    pub fn gpio_gpk0_pull_ctrl(&mut self) -> _GPIO_GPK0_PULL_CTRLW {
        _GPIO_GPK0_PULL_CTRLW { w: self }
    }
}
