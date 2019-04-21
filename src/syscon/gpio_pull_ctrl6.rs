#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPIO_PULL_CTRL6 {
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
#[doc = "Possible values of the field `GPIO_GPD9_PULL_CTRL`"]
pub type GPIO_GPD9_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Possible values of the field `GPIO_GPD8_PULL_CTRL`"]
pub type GPIO_GPD8_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Possible values of the field `GPIO_GPC9_PULL_CTRL`"]
pub type GPIO_GPC9_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Possible values of the field `GPIO_GPC8_PULL_CTRL`"]
pub type GPIO_GPC8_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Values that can be written to the field `GPIO_GPD9_PULL_CTRL`"]
pub type GPIO_GPD9_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPD9_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPD9_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPD9_PULL_CTRLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "High-impedence (floating)"]
    #[inline]
    pub fn highz(self) -> &'a mut W {
        self.variant(::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW::HIGHZ)
    }
    #[doc = "Pulled low"]
    #[inline]
    pub fn pull_low(self) -> &'a mut W {
        self.variant(::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW::PULL_LOW)
    }
    #[doc = "Pulled high"]
    #[inline]
    pub fn pull_high(self) -> &'a mut W {
        self.variant(::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW::PULL_HIGH)
    }
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
#[doc = "Values that can be written to the field `GPIO_GPD8_PULL_CTRL`"]
pub type GPIO_GPD8_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPD8_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPD8_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPD8_PULL_CTRLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "High-impedence (floating)"]
    #[inline]
    pub fn highz(self) -> &'a mut W {
        self.variant(::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW::HIGHZ)
    }
    #[doc = "Pulled low"]
    #[inline]
    pub fn pull_low(self) -> &'a mut W {
        self.variant(::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW::PULL_LOW)
    }
    #[doc = "Pulled high"]
    #[inline]
    pub fn pull_high(self) -> &'a mut W {
        self.variant(::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW::PULL_HIGH)
    }
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
#[doc = "Values that can be written to the field `GPIO_GPC9_PULL_CTRL`"]
pub type GPIO_GPC9_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPC9_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPC9_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPC9_PULL_CTRLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "High-impedence (floating)"]
    #[inline]
    pub fn highz(self) -> &'a mut W {
        self.variant(::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW::HIGHZ)
    }
    #[doc = "Pulled low"]
    #[inline]
    pub fn pull_low(self) -> &'a mut W {
        self.variant(::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW::PULL_LOW)
    }
    #[doc = "Pulled high"]
    #[inline]
    pub fn pull_high(self) -> &'a mut W {
        self.variant(::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW::PULL_HIGH)
    }
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
#[doc = "Values that can be written to the field `GPIO_GPC8_PULL_CTRL`"]
pub type GPIO_GPC8_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPC8_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPC8_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPC8_PULL_CTRLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "High-impedence (floating)"]
    #[inline]
    pub fn highz(self) -> &'a mut W {
        self.variant(::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW::HIGHZ)
    }
    #[doc = "Pulled low"]
    #[inline]
    pub fn pull_low(self) -> &'a mut W {
        self.variant(::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW::PULL_LOW)
    }
    #[doc = "Pulled high"]
    #[inline]
    pub fn pull_high(self) -> &'a mut W {
        self.variant(::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW::PULL_HIGH)
    }
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
    #[doc = "Bits 18:19"]
    #[inline]
    pub fn gpio_gpd9_pull_ctrl(&self) -> GPIO_GPD9_PULL_CTRLR {
        GPIO_GPD9_PULL_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17"]
    #[inline]
    pub fn gpio_gpd8_pull_ctrl(&self) -> GPIO_GPD8_PULL_CTRLR {
        GPIO_GPD8_PULL_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3"]
    #[inline]
    pub fn gpio_gpc9_pull_ctrl(&self) -> GPIO_GPC9_PULL_CTRLR {
        GPIO_GPC9_PULL_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:1"]
    #[inline]
    pub fn gpio_gpc8_pull_ctrl(&self) -> GPIO_GPC8_PULL_CTRLR {
        GPIO_GPC8_PULL_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bits 18:19"]
    #[inline]
    pub fn gpio_gpd9_pull_ctrl(&mut self) -> _GPIO_GPD9_PULL_CTRLW {
        _GPIO_GPD9_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline]
    pub fn gpio_gpd8_pull_ctrl(&mut self) -> _GPIO_GPD8_PULL_CTRLW {
        _GPIO_GPD8_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline]
    pub fn gpio_gpc9_pull_ctrl(&mut self) -> _GPIO_GPC9_PULL_CTRLW {
        _GPIO_GPC9_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline]
    pub fn gpio_gpc8_pull_ctrl(&mut self) -> _GPIO_GPC8_PULL_CTRLW {
        _GPIO_GPC8_PULL_CTRLW { w: self }
    }
}
