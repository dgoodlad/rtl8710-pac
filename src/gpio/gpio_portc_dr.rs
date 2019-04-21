#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPIO_PORTC_DR {
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
#[doc = "Possible values of the field `PC5`"]
pub type PC5R = ::gpio::gpio_porta_dr::PA5R;
#[doc = "Possible values of the field `PC4`"]
pub type PC4R = ::gpio::gpio_porta_dr::PA5R;
#[doc = "Possible values of the field `PC3`"]
pub type PC3R = ::gpio::gpio_porta_dr::PA5R;
#[doc = "Possible values of the field `PC2`"]
pub type PC2R = ::gpio::gpio_porta_dr::PA5R;
#[doc = "Possible values of the field `PC1`"]
pub type PC1R = ::gpio::gpio_porta_dr::PA5R;
#[doc = "Possible values of the field `PC0`"]
pub type PC0R = ::gpio::gpio_porta_dr::PA5R;
#[doc = "Values that can be written to the field `PC5`"]
pub type PC5W = ::gpio::gpio_porta_dr::PA5W;
#[doc = r" Proxy"]
pub struct _PC5W<'a> {
    w: &'a mut W,
}
impl<'a> _PC5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PC5W) -> &'a mut W {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PC4`"]
pub type PC4W = ::gpio::gpio_porta_dr::PA5W;
#[doc = r" Proxy"]
pub struct _PC4W<'a> {
    w: &'a mut W,
}
impl<'a> _PC4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PC4W) -> &'a mut W {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PC3`"]
pub type PC3W = ::gpio::gpio_porta_dr::PA5W;
#[doc = r" Proxy"]
pub struct _PC3W<'a> {
    w: &'a mut W,
}
impl<'a> _PC3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PC3W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `PC2`"]
pub type PC2W = ::gpio::gpio_porta_dr::PA5W;
#[doc = r" Proxy"]
pub struct _PC2W<'a> {
    w: &'a mut W,
}
impl<'a> _PC2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PC2W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `PC1`"]
pub type PC1W = ::gpio::gpio_porta_dr::PA5W;
#[doc = r" Proxy"]
pub struct _PC1W<'a> {
    w: &'a mut W,
}
impl<'a> _PC1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PC1W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `PC0`"]
pub type PC0W = ::gpio::gpio_porta_dr::PA5W;
#[doc = r" Proxy"]
pub struct _PC0W<'a> {
    w: &'a mut W,
}
impl<'a> _PC0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PC0W) -> &'a mut W {
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
    #[doc = "Bit 5 - GPIOC_5 Data"]
    #[inline]
    pub fn pc5(&self) -> PC5R {
        PC5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - GPIOC_4 Data"]
    #[inline]
    pub fn pc4(&self) -> PC4R {
        PC4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - GPIOC_3 Data"]
    #[inline]
    pub fn pc3(&self) -> PC3R {
        PC3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - GPIOC_2 Data"]
    #[inline]
    pub fn pc2(&self) -> PC2R {
        PC2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - GPIOC_1 Data"]
    #[inline]
    pub fn pc1(&self) -> PC1R {
        PC1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - GPIOC_0 Data"]
    #[inline]
    pub fn pc0(&self) -> PC0R {
        PC0R::_from({
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
    #[doc = "Bit 5 - GPIOC_5 Data"]
    #[inline]
    pub fn pc5(&mut self) -> _PC5W {
        _PC5W { w: self }
    }
    #[doc = "Bit 4 - GPIOC_4 Data"]
    #[inline]
    pub fn pc4(&mut self) -> _PC4W {
        _PC4W { w: self }
    }
    #[doc = "Bit 3 - GPIOC_3 Data"]
    #[inline]
    pub fn pc3(&mut self) -> _PC3W {
        _PC3W { w: self }
    }
    #[doc = "Bit 2 - GPIOC_2 Data"]
    #[inline]
    pub fn pc2(&mut self) -> _PC2W {
        _PC2W { w: self }
    }
    #[doc = "Bit 1 - GPIOC_1 Data"]
    #[inline]
    pub fn pc1(&mut self) -> _PC1W {
        _PC1W { w: self }
    }
    #[doc = "Bit 0 - GPIOC_0 Data"]
    #[inline]
    pub fn pc0(&mut self) -> _PC0W {
        _PC0W { w: self }
    }
}
