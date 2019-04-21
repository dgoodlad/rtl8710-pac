#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPIO_PULL_CTRL0 {
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
#[doc = "Possible values of the field `GPIO_GPB7_PULL_CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_GPB7_PULL_CTRLR {
    #[doc = "High-impedence (floating)"]
    HIGHZ,
    #[doc = "Pulled low"]
    PULL_LOW,
    #[doc = "Pulled high"]
    PULL_HIGH,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl GPIO_GPB7_PULL_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO_GPB7_PULL_CTRLR::HIGHZ => 0,
            GPIO_GPB7_PULL_CTRLR::PULL_LOW => 1,
            GPIO_GPB7_PULL_CTRLR::PULL_HIGH => 2,
            GPIO_GPB7_PULL_CTRLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO_GPB7_PULL_CTRLR {
        match value {
            0 => GPIO_GPB7_PULL_CTRLR::HIGHZ,
            1 => GPIO_GPB7_PULL_CTRLR::PULL_LOW,
            2 => GPIO_GPB7_PULL_CTRLR::PULL_HIGH,
            i => GPIO_GPB7_PULL_CTRLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `HIGHZ`"]
    #[inline]
    pub fn is_highz(&self) -> bool {
        *self == GPIO_GPB7_PULL_CTRLR::HIGHZ
    }
    #[doc = "Checks if the value of the field is `PULL_LOW`"]
    #[inline]
    pub fn is_pull_low(&self) -> bool {
        *self == GPIO_GPB7_PULL_CTRLR::PULL_LOW
    }
    #[doc = "Checks if the value of the field is `PULL_HIGH`"]
    #[inline]
    pub fn is_pull_high(&self) -> bool {
        *self == GPIO_GPB7_PULL_CTRLR::PULL_HIGH
    }
}
#[doc = "Possible values of the field `GPIO_GPB6_PULL_CTRL`"]
pub type GPIO_GPB6_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Possible values of the field `GPIO_GPB5_PULL_CTRL`"]
pub type GPIO_GPB5_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Possible values of the field `GPIO_GPB4_PULL_CTRL`"]
pub type GPIO_GPB4_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Possible values of the field `GPIO_GPB3_PULL_CTRL`"]
pub type GPIO_GPB3_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Possible values of the field `GPIO_GPB2_PULL_CTRL`"]
pub type GPIO_GPB2_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Possible values of the field `GPIO_GPB1_PULL_CTRL`"]
pub type GPIO_GPB1_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Possible values of the field `GPIO_GPB0_PULL_CTRL`"]
pub type GPIO_GPB0_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Possible values of the field `GPIO_GPA7_PULL_CTRL`"]
pub type GPIO_GPA7_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Possible values of the field `GPIO_GPA6_PULL_CTRL`"]
pub type GPIO_GPA6_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Possible values of the field `GPIO_GPA5_PULL_CTRL`"]
pub type GPIO_GPA5_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Possible values of the field `GPIO_GPA4_PULL_CTRL`"]
pub type GPIO_GPA4_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Possible values of the field `GPIO_GPA3_PULL_CTRL`"]
pub type GPIO_GPA3_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Possible values of the field `GPIO_GPA2_PULL_CTRL`"]
pub type GPIO_GPA2_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Possible values of the field `GPIO_GPA1_PULL_CTRL`"]
pub type GPIO_GPA1_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Possible values of the field `GPIO_GPA0_PULL_CTRL`"]
pub type GPIO_GPA0_PULL_CTRLR = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLR;
#[doc = "Values that can be written to the field `GPIO_GPB7_PULL_CTRL`"]
pub enum GPIO_GPB7_PULL_CTRLW {
    #[doc = "High-impedence (floating)"]
    HIGHZ,
    #[doc = "Pulled low"]
    PULL_LOW,
    #[doc = "Pulled high"]
    PULL_HIGH,
}
impl GPIO_GPB7_PULL_CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO_GPB7_PULL_CTRLW::HIGHZ => 0,
            GPIO_GPB7_PULL_CTRLW::PULL_LOW => 1,
            GPIO_GPB7_PULL_CTRLW::PULL_HIGH => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_GPB7_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPB7_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPB7_PULL_CTRLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "High-impedence (floating)"]
    #[inline]
    pub fn highz(self) -> &'a mut W {
        self.variant(GPIO_GPB7_PULL_CTRLW::HIGHZ)
    }
    #[doc = "Pulled low"]
    #[inline]
    pub fn pull_low(self) -> &'a mut W {
        self.variant(GPIO_GPB7_PULL_CTRLW::PULL_LOW)
    }
    #[doc = "Pulled high"]
    #[inline]
    pub fn pull_high(self) -> &'a mut W {
        self.variant(GPIO_GPB7_PULL_CTRLW::PULL_HIGH)
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
#[doc = "Values that can be written to the field `GPIO_GPB6_PULL_CTRL`"]
pub type GPIO_GPB6_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPB6_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPB6_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPB6_PULL_CTRLW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `GPIO_GPB5_PULL_CTRL`"]
pub type GPIO_GPB5_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPB5_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPB5_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPB5_PULL_CTRLW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `GPIO_GPB4_PULL_CTRL`"]
pub type GPIO_GPB4_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPB4_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPB4_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPB4_PULL_CTRLW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `GPIO_GPB3_PULL_CTRL`"]
pub type GPIO_GPB3_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPB3_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPB3_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPB3_PULL_CTRLW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `GPIO_GPB2_PULL_CTRL`"]
pub type GPIO_GPB2_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPB2_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPB2_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPB2_PULL_CTRLW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `GPIO_GPB1_PULL_CTRL`"]
pub type GPIO_GPB1_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPB1_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPB1_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPB1_PULL_CTRLW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `GPIO_GPB0_PULL_CTRL`"]
pub type GPIO_GPB0_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPB0_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPB0_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPB0_PULL_CTRLW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `GPIO_GPA7_PULL_CTRL`"]
pub type GPIO_GPA7_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPA7_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPA7_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPA7_PULL_CTRLW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `GPIO_GPA6_PULL_CTRL`"]
pub type GPIO_GPA6_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPA6_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPA6_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPA6_PULL_CTRLW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `GPIO_GPA5_PULL_CTRL`"]
pub type GPIO_GPA5_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPA5_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPA5_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPA5_PULL_CTRLW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `GPIO_GPA4_PULL_CTRL`"]
pub type GPIO_GPA4_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPA4_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPA4_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPA4_PULL_CTRLW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `GPIO_GPA3_PULL_CTRL`"]
pub type GPIO_GPA3_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPA3_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPA3_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPA3_PULL_CTRLW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `GPIO_GPA2_PULL_CTRL`"]
pub type GPIO_GPA2_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPA2_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPA2_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPA2_PULL_CTRLW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `GPIO_GPA1_PULL_CTRL`"]
pub type GPIO_GPA1_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPA1_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPA1_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPA1_PULL_CTRLW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `GPIO_GPA0_PULL_CTRL`"]
pub type GPIO_GPA0_PULL_CTRLW = ::syscon::gpio_pull_ctrl0::GPIO_GPB7_PULL_CTRLW;
#[doc = r" Proxy"]
pub struct _GPIO_GPA0_PULL_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GPA0_PULL_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GPA0_PULL_CTRLW) -> &'a mut W {
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
    pub fn gpio_gpb7_pull_ctrl(&self) -> GPIO_GPB7_PULL_CTRLR {
        GPIO_GPB7_PULL_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29"]
    #[inline]
    pub fn gpio_gpb6_pull_ctrl(&self) -> GPIO_GPB6_PULL_CTRLR {
        GPIO_GPB6_PULL_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27"]
    #[inline]
    pub fn gpio_gpb5_pull_ctrl(&self) -> GPIO_GPB5_PULL_CTRLR {
        GPIO_GPB5_PULL_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25"]
    #[inline]
    pub fn gpio_gpb4_pull_ctrl(&self) -> GPIO_GPB4_PULL_CTRLR {
        GPIO_GPB4_PULL_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23"]
    #[inline]
    pub fn gpio_gpb3_pull_ctrl(&self) -> GPIO_GPB3_PULL_CTRLR {
        GPIO_GPB3_PULL_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21"]
    #[inline]
    pub fn gpio_gpb2_pull_ctrl(&self) -> GPIO_GPB2_PULL_CTRLR {
        GPIO_GPB2_PULL_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19"]
    #[inline]
    pub fn gpio_gpb1_pull_ctrl(&self) -> GPIO_GPB1_PULL_CTRLR {
        GPIO_GPB1_PULL_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17"]
    #[inline]
    pub fn gpio_gpb0_pull_ctrl(&self) -> GPIO_GPB0_PULL_CTRLR {
        GPIO_GPB0_PULL_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15"]
    #[inline]
    pub fn gpio_gpa7_pull_ctrl(&self) -> GPIO_GPA7_PULL_CTRLR {
        GPIO_GPA7_PULL_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13"]
    #[inline]
    pub fn gpio_gpa6_pull_ctrl(&self) -> GPIO_GPA6_PULL_CTRLR {
        GPIO_GPA6_PULL_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11"]
    #[inline]
    pub fn gpio_gpa5_pull_ctrl(&self) -> GPIO_GPA5_PULL_CTRLR {
        GPIO_GPA5_PULL_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9"]
    #[inline]
    pub fn gpio_gpa4_pull_ctrl(&self) -> GPIO_GPA4_PULL_CTRLR {
        GPIO_GPA4_PULL_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7"]
    #[inline]
    pub fn gpio_gpa3_pull_ctrl(&self) -> GPIO_GPA3_PULL_CTRLR {
        GPIO_GPA3_PULL_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5"]
    #[inline]
    pub fn gpio_gpa2_pull_ctrl(&self) -> GPIO_GPA2_PULL_CTRLR {
        GPIO_GPA2_PULL_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3"]
    #[inline]
    pub fn gpio_gpa1_pull_ctrl(&self) -> GPIO_GPA1_PULL_CTRLR {
        GPIO_GPA1_PULL_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:1"]
    #[inline]
    pub fn gpio_gpa0_pull_ctrl(&self) -> GPIO_GPA0_PULL_CTRLR {
        GPIO_GPA0_PULL_CTRLR::_from({
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
    pub fn gpio_gpb7_pull_ctrl(&mut self) -> _GPIO_GPB7_PULL_CTRLW {
        _GPIO_GPB7_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 28:29"]
    #[inline]
    pub fn gpio_gpb6_pull_ctrl(&mut self) -> _GPIO_GPB6_PULL_CTRLW {
        _GPIO_GPB6_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 26:27"]
    #[inline]
    pub fn gpio_gpb5_pull_ctrl(&mut self) -> _GPIO_GPB5_PULL_CTRLW {
        _GPIO_GPB5_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline]
    pub fn gpio_gpb4_pull_ctrl(&mut self) -> _GPIO_GPB4_PULL_CTRLW {
        _GPIO_GPB4_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 22:23"]
    #[inline]
    pub fn gpio_gpb3_pull_ctrl(&mut self) -> _GPIO_GPB3_PULL_CTRLW {
        _GPIO_GPB3_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline]
    pub fn gpio_gpb2_pull_ctrl(&mut self) -> _GPIO_GPB2_PULL_CTRLW {
        _GPIO_GPB2_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline]
    pub fn gpio_gpb1_pull_ctrl(&mut self) -> _GPIO_GPB1_PULL_CTRLW {
        _GPIO_GPB1_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline]
    pub fn gpio_gpb0_pull_ctrl(&mut self) -> _GPIO_GPB0_PULL_CTRLW {
        _GPIO_GPB0_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline]
    pub fn gpio_gpa7_pull_ctrl(&mut self) -> _GPIO_GPA7_PULL_CTRLW {
        _GPIO_GPA7_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline]
    pub fn gpio_gpa6_pull_ctrl(&mut self) -> _GPIO_GPA6_PULL_CTRLW {
        _GPIO_GPA6_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline]
    pub fn gpio_gpa5_pull_ctrl(&mut self) -> _GPIO_GPA5_PULL_CTRLW {
        _GPIO_GPA5_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline]
    pub fn gpio_gpa4_pull_ctrl(&mut self) -> _GPIO_GPA4_PULL_CTRLW {
        _GPIO_GPA4_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline]
    pub fn gpio_gpa3_pull_ctrl(&mut self) -> _GPIO_GPA3_PULL_CTRLW {
        _GPIO_GPA3_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline]
    pub fn gpio_gpa2_pull_ctrl(&mut self) -> _GPIO_GPA2_PULL_CTRLW {
        _GPIO_GPA2_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline]
    pub fn gpio_gpa1_pull_ctrl(&mut self) -> _GPIO_GPA1_PULL_CTRLW {
        _GPIO_GPA1_PULL_CTRLW { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline]
    pub fn gpio_gpa0_pull_ctrl(&mut self) -> _GPIO_GPA0_PULL_CTRLW {
        _GPIO_GPA0_PULL_CTRLW { w: self }
    }
}
