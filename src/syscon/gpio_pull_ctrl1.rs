#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPIO_PULL_CTRL1 {
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
#[doc = "Possible values of the field `GPIO_GPD7_PULL_CTRL`"]
pub type GPIO_GPD7_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Possible values of the field `GPIO_GPD6_PULL_CTRL`"]
pub type GPIO_GPD6_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Possible values of the field `GPIO_GPD5_PULL_CTRL`"]
pub type GPIO_GPD5_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Possible values of the field `GPIO_GPD4_PULL_CTRL`"]
pub type GPIO_GPD4_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Possible values of the field `GPIO_GPD3_PULL_CTRL`"]
pub type GPIO_GPD3_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Possible values of the field `GPIO_GPD2_PULL_CTRL`"]
pub type GPIO_GPD2_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Possible values of the field `GPIO_GPD1_PULL_CTRL`"]
pub type GPIO_GPD1_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Possible values of the field `GPIO_GPD0_PULL_CTRL`"]
pub type GPIO_GPD0_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Possible values of the field `GPIO_GPC7_PULL_CTRL`"]
pub type GPIO_GPC7_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Possible values of the field `GPIO_GPC6_PULL_CTRL`"]
pub type GPIO_GPC6_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Possible values of the field `GPIO_GPC5_PULL_CTRL`"]
pub type GPIO_GPC5_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Possible values of the field `GPIO_GPC4_PULL_CTRL`"]
pub type GPIO_GPC4_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Possible values of the field `GPIO_GPC3_PULL_CTRL`"]
pub type GPIO_GPC3_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Possible values of the field `GPIO_GPC2_PULL_CTRL`"]
pub type GPIO_GPC2_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Possible values of the field `GPIO_GPC1_PULL_CTRL`"]
pub type GPIO_GPC1_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Possible values of the field `GPIO_GPC0_PULL_CTRL`"]
pub type GPIO_GPC0_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Values that can be written to the field `GPIO_GPD7_PULL_CTRL`"]
pub type GPIO_GPD7_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPD7_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPD7_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPD7_PULL_CTRLW) -> &'a mut W {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIO_GPD6_PULL_CTRL`"]
pub type GPIO_GPD6_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPD6_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPD6_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPD6_PULL_CTRLW) -> &'a mut W {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIO_GPD5_PULL_CTRL`"]
pub type GPIO_GPD5_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPD5_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPD5_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPD5_PULL_CTRLW) -> &'a mut W {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIO_GPD4_PULL_CTRL`"]
pub type GPIO_GPD4_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPD4_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPD4_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPD4_PULL_CTRLW) -> &'a mut W {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIO_GPD3_PULL_CTRL`"]
pub type GPIO_GPD3_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPD3_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPD3_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPD3_PULL_CTRLW) -> &'a mut W {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIO_GPD2_PULL_CTRL`"]
pub type GPIO_GPD2_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPD2_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPD2_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPD2_PULL_CTRLW) -> &'a mut W {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIO_GPD1_PULL_CTRL`"]
pub type GPIO_GPD1_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPD1_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPD1_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPD1_PULL_CTRLW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `GPIO_GPD0_PULL_CTRL`"]
pub type GPIO_GPD0_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPD0_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPD0_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPD0_PULL_CTRLW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `GPIO_GPC7_PULL_CTRL`"]
pub type GPIO_GPC7_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPC7_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPC7_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPC7_PULL_CTRLW) -> &'a mut W {
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIO_GPC6_PULL_CTRL`"]
pub type GPIO_GPC6_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPC6_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPC6_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPC6_PULL_CTRLW) -> &'a mut W {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIO_GPC5_PULL_CTRL`"]
pub type GPIO_GPC5_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPC5_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPC5_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPC5_PULL_CTRLW) -> &'a mut W {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIO_GPC4_PULL_CTRL`"]
pub type GPIO_GPC4_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPC4_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPC4_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPC4_PULL_CTRLW) -> &'a mut W {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIO_GPC3_PULL_CTRL`"]
pub type GPIO_GPC3_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPC3_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPC3_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPC3_PULL_CTRLW) -> &'a mut W {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIO_GPC2_PULL_CTRL`"]
pub type GPIO_GPC2_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPC2_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPC2_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPC2_PULL_CTRLW) -> &'a mut W {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIO_GPC1_PULL_CTRL`"]
pub type GPIO_GPC1_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPC1_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPC1_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPC1_PULL_CTRLW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `GPIO_GPC0_PULL_CTRL`"]
pub type GPIO_GPC0_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPC0_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPC0_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPC0_PULL_CTRLW) -> &'a mut W {
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
    #[doc = "Bits 30:31"]
    #[inline]
    pub fn gpio_gpd7_pull_ctrl(&self) -> GPIO_GPD7_PULL_CTRLR {
        GPIO_GPD7_PULL_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29"]
    #[inline]
    pub fn gpio_gpd6_pull_ctrl(&self) -> GPIO_GPD6_PULL_CTRLR {
        GPIO_GPD6_PULL_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27"]
    #[inline]
    pub fn gpio_gpd5_pull_ctrl(&self) -> GPIO_GPD5_PULL_CTRLR {
        GPIO_GPD5_PULL_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25"]
    #[inline]
    pub fn gpio_gpd4_pull_ctrl(&self) -> GPIO_GPD4_PULL_CTRLR {
        GPIO_GPD4_PULL_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23"]
    #[inline]
    pub fn gpio_gpd3_pull_ctrl(&self) -> GPIO_GPD3_PULL_CTRLR {
        GPIO_GPD3_PULL_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21"]
    #[inline]
    pub fn gpio_gpd2_pull_ctrl(&self) -> GPIO_GPD2_PULL_CTRLR {
        GPIO_GPD2_PULL_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19"]
    #[inline]
    pub fn gpio_gpd1_pull_ctrl(&self) -> GPIO_GPD1_PULL_CTRLR {
        GPIO_GPD1_PULL_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17"]
    #[inline]
    pub fn gpio_gpd0_pull_ctrl(&self) -> GPIO_GPD0_PULL_CTRLR {
        GPIO_GPD0_PULL_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15"]
    #[inline]
    pub fn gpio_gpc7_pull_ctrl(&self) -> GPIO_GPC7_PULL_CTRLR {
        GPIO_GPC7_PULL_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13"]
    #[inline]
    pub fn gpio_gpc6_pull_ctrl(&self) -> GPIO_GPC6_PULL_CTRLR {
        GPIO_GPC6_PULL_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11"]
    #[inline]
    pub fn gpio_gpc5_pull_ctrl(&self) -> GPIO_GPC5_PULL_CTRLR {
        GPIO_GPC5_PULL_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9"]
    #[inline]
    pub fn gpio_gpc4_pull_ctrl(&self) -> GPIO_GPC4_PULL_CTRLR {
        GPIO_GPC4_PULL_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7"]
    #[inline]
    pub fn gpio_gpc3_pull_ctrl(&self) -> GPIO_GPC3_PULL_CTRLR {
        GPIO_GPC3_PULL_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5"]
    #[inline]
    pub fn gpio_gpc2_pull_ctrl(&self) -> GPIO_GPC2_PULL_CTRLR {
        GPIO_GPC2_PULL_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3"]
    #[inline]
    pub fn gpio_gpc1_pull_ctrl(&self) -> GPIO_GPC1_PULL_CTRLR {
        GPIO_GPC1_PULL_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:1"]
    #[inline]
    pub fn gpio_gpc0_pull_ctrl(&self) -> GPIO_GPC0_PULL_CTRLR {
        GPIO_GPC0_PULL_CTRLR::_from({
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
    #[doc = "Bits 30:31"]
    #[inline]
    pub fn gpio_gpd7_pull_ctrl(&mut self) -> _GPIO_GPD7_PULL_CTRLW {
        _GPIO_GPD7_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 28:29"]
    #[inline]
    pub fn gpio_gpd6_pull_ctrl(&mut self) -> _GPIO_GPD6_PULL_CTRLW {
        _GPIO_GPD6_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 26:27"]
    #[inline]
    pub fn gpio_gpd5_pull_ctrl(&mut self) -> _GPIO_GPD5_PULL_CTRLW {
        _GPIO_GPD5_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline]
    pub fn gpio_gpd4_pull_ctrl(&mut self) -> _GPIO_GPD4_PULL_CTRLW {
        _GPIO_GPD4_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 22:23"]
    #[inline]
    pub fn gpio_gpd3_pull_ctrl(&mut self) -> _GPIO_GPD3_PULL_CTRLW {
        _GPIO_GPD3_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline]
    pub fn gpio_gpd2_pull_ctrl(&mut self) -> _GPIO_GPD2_PULL_CTRLW {
        _GPIO_GPD2_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline]
    pub fn gpio_gpd1_pull_ctrl(&mut self) -> _GPIO_GPD1_PULL_CTRLW {
        _GPIO_GPD1_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline]
    pub fn gpio_gpd0_pull_ctrl(&mut self) -> _GPIO_GPD0_PULL_CTRLW {
        _GPIO_GPD0_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline]
    pub fn gpio_gpc7_pull_ctrl(&mut self) -> _GPIO_GPC7_PULL_CTRLW {
        _GPIO_GPC7_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline]
    pub fn gpio_gpc6_pull_ctrl(&mut self) -> _GPIO_GPC6_PULL_CTRLW {
        _GPIO_GPC6_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline]
    pub fn gpio_gpc5_pull_ctrl(&mut self) -> _GPIO_GPC5_PULL_CTRLW {
        _GPIO_GPC5_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline]
    pub fn gpio_gpc4_pull_ctrl(&mut self) -> _GPIO_GPC4_PULL_CTRLW {
        _GPIO_GPC4_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline]
    pub fn gpio_gpc3_pull_ctrl(&mut self) -> _GPIO_GPC3_PULL_CTRLW {
        _GPIO_GPC3_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline]
    pub fn gpio_gpc2_pull_ctrl(&mut self) -> _GPIO_GPC2_PULL_CTRLW {
        _GPIO_GPC2_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline]
    pub fn gpio_gpc1_pull_ctrl(&mut self) -> _GPIO_GPC1_PULL_CTRLW {
        _GPIO_GPC1_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline]
    pub fn gpio_gpc0_pull_ctrl(&mut self) -> _GPIO_GPC0_PULL_CTRLW {
        _GPIO_GPC0_PULL_CTRLW { w: self }
    }
}
