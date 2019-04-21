#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPIO_PORTB_DR {
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
#[doc = "Possible values of the field `PB3`"]
pub type PB3R = ::gpio::gpio_porta_dr::PA5R;
#[doc = "Possible values of the field `PB2`"]
pub type PB2R = ::gpio::gpio_porta_dr::PA5R;
#[doc = "Possible values of the field `PB1`"]
pub type PB1R = ::gpio::gpio_porta_dr::PA5R;
#[doc = "Possible values of the field `PB0`"]
pub type PB0R = ::gpio::gpio_porta_dr::PA5R;
#[doc = "Values that can be written to the field `PB3`"]
pub type PB3W = ::gpio::gpio_porta_dr::PA5W;
#[doc = r" Proxy"]
pub struct _PB3W<'a> {
    w: &'a mut W,
}
impl<'a> _PB3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PB3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low value"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(::gpio::gpio_porta_dr::PA5W::LOW)
    }
    #[doc = "High value"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(::gpio::gpio_porta_dr::PA5W::HIGH)
    }
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
#[doc = "Values that can be written to the field `PB2`"]
pub type PB2W = ::gpio::gpio_porta_dr::PA5W;
#[doc = r" Proxy"]
pub struct _PB2W<'a> {
    w: &'a mut W,
}
impl<'a> _PB2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PB2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low value"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(::gpio::gpio_porta_dr::PA5W::LOW)
    }
    #[doc = "High value"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(::gpio::gpio_porta_dr::PA5W::HIGH)
    }
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
#[doc = "Values that can be written to the field `PB1`"]
pub type PB1W = ::gpio::gpio_porta_dr::PA5W;
#[doc = r" Proxy"]
pub struct _PB1W<'a> {
    w: &'a mut W,
}
impl<'a> _PB1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PB1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low value"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(::gpio::gpio_porta_dr::PA5W::LOW)
    }
    #[doc = "High value"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(::gpio::gpio_porta_dr::PA5W::HIGH)
    }
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
#[doc = "Values that can be written to the field `PB0`"]
pub type PB0W = ::gpio::gpio_porta_dr::PA5W;
#[doc = r" Proxy"]
pub struct _PB0W<'a> {
    w: &'a mut W,
}
impl<'a> _PB0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PB0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low value"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(::gpio::gpio_porta_dr::PA5W::LOW)
    }
    #[doc = "High value"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(::gpio::gpio_porta_dr::PA5W::HIGH)
    }
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
    #[doc = "Bit 3 - GPIOB_3 Data"]
    #[inline]
    pub fn pb3(&self) -> PB3R {
        PB3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - GPIOB_2 Data"]
    #[inline]
    pub fn pb2(&self) -> PB2R {
        PB2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - GPIOB_1 Data"]
    #[inline]
    pub fn pb1(&self) -> PB1R {
        PB1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - GPIOB_0 Data"]
    #[inline]
    pub fn pb0(&self) -> PB0R {
        PB0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bit 3 - GPIOB_3 Data"]
    #[inline]
    pub fn pb3(&mut self) -> _PB3W {
        _PB3W { w: self }
    }
    #[doc = "Bit 2 - GPIOB_2 Data"]
    #[inline]
    pub fn pb2(&mut self) -> _PB2W {
        _PB2W { w: self }
    }
    #[doc = "Bit 1 - GPIOB_1 Data"]
    #[inline]
    pub fn pb1(&mut self) -> _PB1W {
        _PB1W { w: self }
    }
    #[doc = "Bit 0 - GPIOB_0 Data"]
    #[inline]
    pub fn pb0(&mut self) -> _PB0W {
        _PB0W { w: self }
    }
}
