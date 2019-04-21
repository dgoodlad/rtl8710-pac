#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPIO_PORTA_DDR {
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
#[doc = "Possible values of the field `PA5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PA5R {
    #[doc = "Configure the pin as an input"]
    INPUT,
    #[doc = "Configure the pin as an output"]
    OUTPUT,
}
impl PA5R {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PA5R::INPUT => false,
            PA5R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PA5R {
        match value {
            false => PA5R::INPUT,
            true => PA5R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == PA5R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == PA5R::OUTPUT
    }
}
#[doc = "Possible values of the field `PA4`"]
pub type PA4R = ::gpio::gpio_porta_ddr::PA5R;
#[doc = "Possible values of the field `PA0`"]
pub type PA0R = ::gpio::gpio_porta_ddr::PA5R;
#[doc = "Values that can be written to the field `PA5`"]
pub enum PA5W {
    #[doc = "Configure the pin as an input"]
    INPUT,
    #[doc = "Configure the pin as an output"]
    OUTPUT,
}
impl PA5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PA5W::INPUT => false,
            PA5W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PA5W<'a> {
    w: &'a mut W,
}
impl<'a> _PA5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PA5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Configure the pin as an input"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(PA5W::INPUT)
    }
    #[doc = "Configure the pin as an output"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(PA5W::OUTPUT)
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
#[doc = "Values that can be written to the field `PA4`"]
pub type PA4W = ::gpio::gpio_porta_ddr::PA5W;
#[doc = r" Proxy"]
pub struct _PA4W<'a> {
    w: &'a mut W,
}
impl<'a> _PA4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PA4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Configure the pin as an input"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(::gpio::gpio_porta_ddr::PA5W::INPUT)
    }
    #[doc = "Configure the pin as an output"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(::gpio::gpio_porta_ddr::PA5W::OUTPUT)
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
#[doc = "Values that can be written to the field `PA0`"]
pub type PA0W = ::gpio::gpio_porta_ddr::PA5W;
#[doc = r" Proxy"]
pub struct _PA0W<'a> {
    w: &'a mut W,
}
impl<'a> _PA0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PA0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Configure the pin as an input"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(::gpio::gpio_porta_ddr::PA5W::INPUT)
    }
    #[doc = "Configure the pin as an output"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(::gpio::gpio_porta_ddr::PA5W::OUTPUT)
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
    #[doc = "Bit 5 - GPIOA_5 Data Direction"]
    #[inline]
    pub fn pa5(&self) -> PA5R {
        PA5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - GPIOA_4 Data Direction"]
    #[inline]
    pub fn pa4(&self) -> PA4R {
        PA4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - GPIOA_0 Data Direction"]
    #[inline]
    pub fn pa0(&self) -> PA0R {
        PA0R::_from({
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
    #[doc = "Bit 5 - GPIOA_5 Data Direction"]
    #[inline]
    pub fn pa5(&mut self) -> _PA5W {
        _PA5W { w: self }
    }
    #[doc = "Bit 4 - GPIOA_4 Data Direction"]
    #[inline]
    pub fn pa4(&mut self) -> _PA4W {
        _PA4W { w: self }
    }
    #[doc = "Bit 0 - GPIOA_0 Data Direction"]
    #[inline]
    pub fn pa0(&mut self) -> _PA0W {
        _PA0W { w: self }
    }
}
