#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPIO_PULL_CTRL4 {
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
pub struct GPIO_GPJ7_PULL_CTRLR {
    bits: u8,
}
impl GPIO_GPJ7_PULL_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GPIO_GPJ6_PULL_CTRLR {
    bits: u8,
}
impl GPIO_GPJ6_PULL_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GPIO_GPJ5_PULL_CTRLR {
    bits: u8,
}
impl GPIO_GPJ5_PULL_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GPIO_GPJ4_PULL_CTRLR {
    bits: u8,
}
impl GPIO_GPJ4_PULL_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GPIO_GPJ3_PULL_CTRLR {
    bits: u8,
}
impl GPIO_GPJ3_PULL_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GPIO_GPJ2_PULL_CTRLR {
    bits: u8,
}
impl GPIO_GPJ2_PULL_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GPIO_GPJ1_PULL_CTRLR {
    bits: u8,
}
impl GPIO_GPJ1_PULL_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GPIO_GPJ0_PULL_CTRLR {
    bits: u8,
}
impl GPIO_GPJ0_PULL_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GPIO_GPI7_PULL_CTRLR {
    bits: u8,
}
impl GPIO_GPI7_PULL_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GPIO_GPI6_PULL_CTRLR {
    bits: u8,
}
impl GPIO_GPI6_PULL_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GPIO_GPI5_PULL_CTRLR {
    bits: u8,
}
impl GPIO_GPI5_PULL_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GPIO_GPI4_PULL_CTRLR {
    bits: u8,
}
impl GPIO_GPI4_PULL_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GPIO_GPI3_PULL_CTRLR {
    bits: u8,
}
impl GPIO_GPI3_PULL_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GPIO_GPI2_PULL_CTRLR {
    bits: u8,
}
impl GPIO_GPI2_PULL_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GPIO_GPI1_PULL_CTRLR {
    bits: u8,
}
impl GPIO_GPI1_PULL_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GPIO_GPI0_PULL_CTRLR {
    bits: u8,
}
impl GPIO_GPI0_PULL_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_GPJ7_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPJ7_PULL_CTRLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_GPJ6_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPJ6_PULL_CTRLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_GPJ5_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPJ5_PULL_CTRLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_GPJ4_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPJ4_PULL_CTRLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_GPJ3_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPJ3_PULL_CTRLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_GPJ2_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPJ2_PULL_CTRLW<'a> {
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
pub struct _GPIO_GPJ1_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPJ1_PULL_CTRLW<'a> {
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
pub struct _GPIO_GPJ0_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPJ0_PULL_CTRLW<'a> {
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
pub struct _GPIO_GPI7_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPI7_PULL_CTRLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_GPI6_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPI6_PULL_CTRLW<'a> {
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
pub struct _GPIO_GPI5_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPI5_PULL_CTRLW<'a> {
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
pub struct _GPIO_GPI4_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPI4_PULL_CTRLW<'a> {
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
pub struct _GPIO_GPI3_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPI3_PULL_CTRLW<'a> {
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
pub struct _GPIO_GPI2_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPI2_PULL_CTRLW<'a> {
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
pub struct _GPIO_GPI1_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPI1_PULL_CTRLW<'a> {
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
pub struct _GPIO_GPI0_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPI0_PULL_CTRLW<'a> {
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
    #[doc = "Bits 30:31"]
    #[inline]
    pub fn gpio_gpj7_pull_ctrl(&self) -> GPIO_GPJ7_PULL_CTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GPIO_GPJ7_PULL_CTRLR { bits }
    }
    #[doc = "Bits 28:29"]
    #[inline]
    pub fn gpio_gpj6_pull_ctrl(&self) -> GPIO_GPJ6_PULL_CTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GPIO_GPJ6_PULL_CTRLR { bits }
    }
    #[doc = "Bits 26:27"]
    #[inline]
    pub fn gpio_gpj5_pull_ctrl(&self) -> GPIO_GPJ5_PULL_CTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GPIO_GPJ5_PULL_CTRLR { bits }
    }
    #[doc = "Bits 24:25"]
    #[inline]
    pub fn gpio_gpj4_pull_ctrl(&self) -> GPIO_GPJ4_PULL_CTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GPIO_GPJ4_PULL_CTRLR { bits }
    }
    #[doc = "Bits 22:23"]
    #[inline]
    pub fn gpio_gpj3_pull_ctrl(&self) -> GPIO_GPJ3_PULL_CTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GPIO_GPJ3_PULL_CTRLR { bits }
    }
    #[doc = "Bits 20:21"]
    #[inline]
    pub fn gpio_gpj2_pull_ctrl(&self) -> GPIO_GPJ2_PULL_CTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GPIO_GPJ2_PULL_CTRLR { bits }
    }
    #[doc = "Bits 18:19"]
    #[inline]
    pub fn gpio_gpj1_pull_ctrl(&self) -> GPIO_GPJ1_PULL_CTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GPIO_GPJ1_PULL_CTRLR { bits }
    }
    #[doc = "Bits 16:17"]
    #[inline]
    pub fn gpio_gpj0_pull_ctrl(&self) -> GPIO_GPJ0_PULL_CTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GPIO_GPJ0_PULL_CTRLR { bits }
    }
    #[doc = "Bits 14:15"]
    #[inline]
    pub fn gpio_gpi7_pull_ctrl(&self) -> GPIO_GPI7_PULL_CTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GPIO_GPI7_PULL_CTRLR { bits }
    }
    #[doc = "Bits 12:13"]
    #[inline]
    pub fn gpio_gpi6_pull_ctrl(&self) -> GPIO_GPI6_PULL_CTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GPIO_GPI6_PULL_CTRLR { bits }
    }
    #[doc = "Bits 10:11"]
    #[inline]
    pub fn gpio_gpi5_pull_ctrl(&self) -> GPIO_GPI5_PULL_CTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GPIO_GPI5_PULL_CTRLR { bits }
    }
    #[doc = "Bits 8:9"]
    #[inline]
    pub fn gpio_gpi4_pull_ctrl(&self) -> GPIO_GPI4_PULL_CTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GPIO_GPI4_PULL_CTRLR { bits }
    }
    #[doc = "Bits 6:7"]
    #[inline]
    pub fn gpio_gpi3_pull_ctrl(&self) -> GPIO_GPI3_PULL_CTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GPIO_GPI3_PULL_CTRLR { bits }
    }
    #[doc = "Bits 4:5"]
    #[inline]
    pub fn gpio_gpi2_pull_ctrl(&self) -> GPIO_GPI2_PULL_CTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GPIO_GPI2_PULL_CTRLR { bits }
    }
    #[doc = "Bits 2:3"]
    #[inline]
    pub fn gpio_gpi1_pull_ctrl(&self) -> GPIO_GPI1_PULL_CTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GPIO_GPI1_PULL_CTRLR { bits }
    }
    #[doc = "Bits 0:1"]
    #[inline]
    pub fn gpio_gpi0_pull_ctrl(&self) -> GPIO_GPI0_PULL_CTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GPIO_GPI0_PULL_CTRLR { bits }
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
    #[doc = "Bits 30:31"]
    #[inline]
    pub fn gpio_gpj7_pull_ctrl(&mut self) -> _GPIO_GPJ7_PULL_CTRLW {
        _GPIO_GPJ7_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 28:29"]
    #[inline]
    pub fn gpio_gpj6_pull_ctrl(&mut self) -> _GPIO_GPJ6_PULL_CTRLW {
        _GPIO_GPJ6_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 26:27"]
    #[inline]
    pub fn gpio_gpj5_pull_ctrl(&mut self) -> _GPIO_GPJ5_PULL_CTRLW {
        _GPIO_GPJ5_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline]
    pub fn gpio_gpj4_pull_ctrl(&mut self) -> _GPIO_GPJ4_PULL_CTRLW {
        _GPIO_GPJ4_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 22:23"]
    #[inline]
    pub fn gpio_gpj3_pull_ctrl(&mut self) -> _GPIO_GPJ3_PULL_CTRLW {
        _GPIO_GPJ3_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline]
    pub fn gpio_gpj2_pull_ctrl(&mut self) -> _GPIO_GPJ2_PULL_CTRLW {
        _GPIO_GPJ2_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline]
    pub fn gpio_gpj1_pull_ctrl(&mut self) -> _GPIO_GPJ1_PULL_CTRLW {
        _GPIO_GPJ1_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline]
    pub fn gpio_gpj0_pull_ctrl(&mut self) -> _GPIO_GPJ0_PULL_CTRLW {
        _GPIO_GPJ0_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline]
    pub fn gpio_gpi7_pull_ctrl(&mut self) -> _GPIO_GPI7_PULL_CTRLW {
        _GPIO_GPI7_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline]
    pub fn gpio_gpi6_pull_ctrl(&mut self) -> _GPIO_GPI6_PULL_CTRLW {
        _GPIO_GPI6_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline]
    pub fn gpio_gpi5_pull_ctrl(&mut self) -> _GPIO_GPI5_PULL_CTRLW {
        _GPIO_GPI5_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline]
    pub fn gpio_gpi4_pull_ctrl(&mut self) -> _GPIO_GPI4_PULL_CTRLW {
        _GPIO_GPI4_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline]
    pub fn gpio_gpi3_pull_ctrl(&mut self) -> _GPIO_GPI3_PULL_CTRLW {
        _GPIO_GPI3_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline]
    pub fn gpio_gpi2_pull_ctrl(&mut self) -> _GPIO_GPI2_PULL_CTRLW {
        _GPIO_GPI2_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline]
    pub fn gpio_gpi1_pull_ctrl(&mut self) -> _GPIO_GPI1_PULL_CTRLW {
        _GPIO_GPI1_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline]
    pub fn gpio_gpi0_pull_ctrl(&mut self) -> _GPIO_GPI0_PULL_CTRLW {
        _GPIO_GPI0_PULL_CTRLW { w: self }
    }
}