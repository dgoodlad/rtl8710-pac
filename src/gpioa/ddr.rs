#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DDR {
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
#[doc = "Possible values of the field `GPIOA_GC4_DDR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOA_GC4_DDRR {
    #[doc = "Configure pin as an INPUT"]
    INPUT,
    #[doc = "Configure pin as an OUTPUT"]
    OUTPUT,
}
impl GPIOA_GC4_DDRR {
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
            GPIOA_GC4_DDRR::INPUT => false,
            GPIOA_GC4_DDRR::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIOA_GC4_DDRR {
        match value {
            false => GPIOA_GC4_DDRR::INPUT,
            true => GPIOA_GC4_DDRR::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == GPIOA_GC4_DDRR::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == GPIOA_GC4_DDRR::OUTPUT
    }
}
#[doc = "Values that can be written to the field `GPIOA_GC4_DDR`"]
pub enum GPIOA_GC4_DDRW {
    #[doc = "Configure pin as an INPUT"]
    INPUT,
    #[doc = "Configure pin as an OUTPUT"]
    OUTPUT,
}
impl GPIOA_GC4_DDRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIOA_GC4_DDRW::INPUT => false,
            GPIOA_GC4_DDRW::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIOA_GC4_DDRW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOA_GC4_DDRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIOA_GC4_DDRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Configure pin as an INPUT"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(GPIOA_GC4_DDRW::INPUT)
    }
    #[doc = "Configure pin as an OUTPUT"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(GPIOA_GC4_DDRW::OUTPUT)
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
        const OFFSET: u8 = 8;
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
    #[doc = "Bit 8 - GPIOA GC4 Direction"]
    #[inline]
    pub fn gpioa_gc4_ddr(&self) -> GPIOA_GC4_DDRR {
        GPIOA_GC4_DDRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
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
    #[doc = "Bit 8 - GPIOA GC4 Direction"]
    #[inline]
    pub fn gpioa_gc4_ddr(&mut self) -> _GPIOA_GC4_DDRW {
        _GPIOA_GC4_DDRW { w: self }
    }
}
