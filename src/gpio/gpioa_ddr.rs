#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPIOA_DDR {
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
#[doc = "Possible values of the field `A0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum A0R {
    #[doc = "Configure pin as an INPUT"]
    INPUT,
    #[doc = "Configure pin as an OUTPUT"]
    OUTPUT,
}
impl A0R {
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
            A0R::INPUT => false,
            A0R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> A0R {
        match value {
            false => A0R::INPUT,
            true => A0R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == A0R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == A0R::OUTPUT
    }
}
#[doc = "Possible values of the field `A1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum A1R {
    #[doc = "Configure pin as an INPUT"]
    INPUT,
    #[doc = "Configure pin as an OUTPUT"]
    OUTPUT,
}
impl A1R {
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
            A1R::INPUT => false,
            A1R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> A1R {
        match value {
            false => A1R::INPUT,
            true => A1R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == A1R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == A1R::OUTPUT
    }
}
#[doc = "Possible values of the field `B3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum B3R {
    #[doc = "Configure pin as an INPUT"]
    INPUT,
    #[doc = "Configure pin as an OUTPUT"]
    OUTPUT,
}
impl B3R {
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
            B3R::INPUT => false,
            B3R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> B3R {
        match value {
            false => B3R::INPUT,
            true => B3R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == B3R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == B3R::OUTPUT
    }
}
#[doc = "Possible values of the field `B4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum B4R {
    #[doc = "Configure pin as an INPUT"]
    INPUT,
    #[doc = "Configure pin as an OUTPUT"]
    OUTPUT,
}
impl B4R {
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
            B4R::INPUT => false,
            B4R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> B4R {
        match value {
            false => B4R::INPUT,
            true => B4R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == B4R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == B4R::OUTPUT
    }
}
#[doc = "Possible values of the field `C1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C1R {
    #[doc = "Configure pin as an INPUT"]
    INPUT,
    #[doc = "Configure pin as an OUTPUT"]
    OUTPUT,
}
impl C1R {
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
            C1R::INPUT => false,
            C1R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> C1R {
        match value {
            false => C1R::INPUT,
            true => C1R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == C1R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == C1R::OUTPUT
    }
}
#[doc = "Possible values of the field `C3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C3R {
    #[doc = "Configure pin as an INPUT"]
    INPUT,
    #[doc = "Configure pin as an OUTPUT"]
    OUTPUT,
}
impl C3R {
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
            C3R::INPUT => false,
            C3R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> C3R {
        match value {
            false => C3R::INPUT,
            true => C3R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == C3R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == C3R::OUTPUT
    }
}
#[doc = "Possible values of the field `C4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C4R {
    #[doc = "Configure pin as an INPUT"]
    INPUT,
    #[doc = "Configure pin as an OUTPUT"]
    OUTPUT,
}
impl C4R {
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
            C4R::INPUT => false,
            C4R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> C4R {
        match value {
            false => C4R::INPUT,
            true => C4R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == C4R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == C4R::OUTPUT
    }
}
#[doc = "Possible values of the field `E1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E1R {
    #[doc = "Configure pin as an INPUT"]
    INPUT,
    #[doc = "Configure pin as an OUTPUT"]
    OUTPUT,
}
impl E1R {
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
            E1R::INPUT => false,
            E1R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> E1R {
        match value {
            false => E1R::INPUT,
            true => E1R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == E1R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == E1R::OUTPUT
    }
}
#[doc = "Possible values of the field `E2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E2R {
    #[doc = "Configure pin as an INPUT"]
    INPUT,
    #[doc = "Configure pin as an OUTPUT"]
    OUTPUT,
}
impl E2R {
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
            E2R::INPUT => false,
            E2R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> E2R {
        match value {
            false => E2R::INPUT,
            true => E2R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == E2R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == E2R::OUTPUT
    }
}
#[doc = "Possible values of the field `E3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E3R {
    #[doc = "Configure pin as an INPUT"]
    INPUT,
    #[doc = "Configure pin as an OUTPUT"]
    OUTPUT,
}
impl E3R {
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
            E3R::INPUT => false,
            E3R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> E3R {
        match value {
            false => E3R::INPUT,
            true => E3R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == E3R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == E3R::OUTPUT
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
#[doc = "Values that can be written to the field `A0`"]
pub enum A0W {
    #[doc = "Configure pin as an INPUT"]
    INPUT,
    #[doc = "Configure pin as an OUTPUT"]
    OUTPUT,
}
impl A0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            A0W::INPUT => false,
            A0W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _A0W<'a> {
    w: &'a mut W,
}
impl<'a> _A0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: A0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Configure pin as an INPUT"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(A0W::INPUT)
    }
    #[doc = "Configure pin as an OUTPUT"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(A0W::OUTPUT)
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
#[doc = "Values that can be written to the field `A1`"]
pub enum A1W {
    #[doc = "Configure pin as an INPUT"]
    INPUT,
    #[doc = "Configure pin as an OUTPUT"]
    OUTPUT,
}
impl A1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            A1W::INPUT => false,
            A1W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _A1W<'a> {
    w: &'a mut W,
}
impl<'a> _A1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: A1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Configure pin as an INPUT"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(A1W::INPUT)
    }
    #[doc = "Configure pin as an OUTPUT"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(A1W::OUTPUT)
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
#[doc = "Values that can be written to the field `B3`"]
pub enum B3W {
    #[doc = "Configure pin as an INPUT"]
    INPUT,
    #[doc = "Configure pin as an OUTPUT"]
    OUTPUT,
}
impl B3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            B3W::INPUT => false,
            B3W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _B3W<'a> {
    w: &'a mut W,
}
impl<'a> _B3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: B3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Configure pin as an INPUT"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(B3W::INPUT)
    }
    #[doc = "Configure pin as an OUTPUT"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(B3W::OUTPUT)
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
#[doc = "Values that can be written to the field `B4`"]
pub enum B4W {
    #[doc = "Configure pin as an INPUT"]
    INPUT,
    #[doc = "Configure pin as an OUTPUT"]
    OUTPUT,
}
impl B4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            B4W::INPUT => false,
            B4W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _B4W<'a> {
    w: &'a mut W,
}
impl<'a> _B4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: B4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Configure pin as an INPUT"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(B4W::INPUT)
    }
    #[doc = "Configure pin as an OUTPUT"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(B4W::OUTPUT)
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
#[doc = "Values that can be written to the field `C1`"]
pub enum C1W {
    #[doc = "Configure pin as an INPUT"]
    INPUT,
    #[doc = "Configure pin as an OUTPUT"]
    OUTPUT,
}
impl C1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            C1W::INPUT => false,
            C1W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _C1W<'a> {
    w: &'a mut W,
}
impl<'a> _C1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: C1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Configure pin as an INPUT"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(C1W::INPUT)
    }
    #[doc = "Configure pin as an OUTPUT"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(C1W::OUTPUT)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `C3`"]
pub enum C3W {
    #[doc = "Configure pin as an INPUT"]
    INPUT,
    #[doc = "Configure pin as an OUTPUT"]
    OUTPUT,
}
impl C3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            C3W::INPUT => false,
            C3W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _C3W<'a> {
    w: &'a mut W,
}
impl<'a> _C3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: C3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Configure pin as an INPUT"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(C3W::INPUT)
    }
    #[doc = "Configure pin as an OUTPUT"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(C3W::OUTPUT)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `C4`"]
pub enum C4W {
    #[doc = "Configure pin as an INPUT"]
    INPUT,
    #[doc = "Configure pin as an OUTPUT"]
    OUTPUT,
}
impl C4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            C4W::INPUT => false,
            C4W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _C4W<'a> {
    w: &'a mut W,
}
impl<'a> _C4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: C4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Configure pin as an INPUT"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(C4W::INPUT)
    }
    #[doc = "Configure pin as an OUTPUT"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(C4W::OUTPUT)
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
#[doc = "Values that can be written to the field `E1`"]
pub enum E1W {
    #[doc = "Configure pin as an INPUT"]
    INPUT,
    #[doc = "Configure pin as an OUTPUT"]
    OUTPUT,
}
impl E1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            E1W::INPUT => false,
            E1W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _E1W<'a> {
    w: &'a mut W,
}
impl<'a> _E1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: E1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Configure pin as an INPUT"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(E1W::INPUT)
    }
    #[doc = "Configure pin as an OUTPUT"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(E1W::OUTPUT)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `E2`"]
pub enum E2W {
    #[doc = "Configure pin as an INPUT"]
    INPUT,
    #[doc = "Configure pin as an OUTPUT"]
    OUTPUT,
}
impl E2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            E2W::INPUT => false,
            E2W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _E2W<'a> {
    w: &'a mut W,
}
impl<'a> _E2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: E2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Configure pin as an INPUT"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(E2W::INPUT)
    }
    #[doc = "Configure pin as an OUTPUT"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(E2W::OUTPUT)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `E3`"]
pub enum E3W {
    #[doc = "Configure pin as an INPUT"]
    INPUT,
    #[doc = "Configure pin as an OUTPUT"]
    OUTPUT,
}
impl E3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            E3W::INPUT => false,
            E3W::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _E3W<'a> {
    w: &'a mut W,
}
impl<'a> _E3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: E3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Configure pin as an INPUT"]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(E3W::INPUT)
    }
    #[doc = "Configure pin as an OUTPUT"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(E3W::OUTPUT)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[doc = "Bit 0 - GPIO pin A0 data direction"]
    #[inline]
    pub fn a0(&self) -> A0R {
        A0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Pin A1"]
    #[inline]
    pub fn a1(&self) -> A1R {
        A1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Pin B3"]
    #[inline]
    pub fn b3(&self) -> B3R {
        B3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Pin B4"]
    #[inline]
    pub fn b4(&self) -> B4R {
        B4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Pin C1"]
    #[inline]
    pub fn c1(&self) -> C1R {
        C1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Pin C3"]
    #[inline]
    pub fn c3(&self) -> C3R {
        C3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Pin C4"]
    #[inline]
    pub fn c4(&self) -> C4R {
        C4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Pin E1"]
    #[inline]
    pub fn e1(&self) -> E1R {
        E1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Pin E2"]
    #[inline]
    pub fn e2(&self) -> E2R {
        E2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Pin E3"]
    #[inline]
    pub fn e3(&self) -> E3R {
        E3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
    #[doc = "Bit 0 - GPIO pin A0 data direction"]
    #[inline]
    pub fn a0(&mut self) -> _A0W {
        _A0W { w: self }
    }
    #[doc = "Bit 1 - Pin A1"]
    #[inline]
    pub fn a1(&mut self) -> _A1W {
        _A1W { w: self }
    }
    #[doc = "Bit 2 - Pin B3"]
    #[inline]
    pub fn b3(&mut self) -> _B3W {
        _B3W { w: self }
    }
    #[doc = "Bit 3 - Pin B4"]
    #[inline]
    pub fn b4(&mut self) -> _B4W {
        _B4W { w: self }
    }
    #[doc = "Bit 6 - Pin C1"]
    #[inline]
    pub fn c1(&mut self) -> _C1W {
        _C1W { w: self }
    }
    #[doc = "Bit 7 - Pin C3"]
    #[inline]
    pub fn c3(&mut self) -> _C3W {
        _C3W { w: self }
    }
    #[doc = "Bit 8 - Pin C4"]
    #[inline]
    pub fn c4(&mut self) -> _C4W {
        _C4W { w: self }
    }
    #[doc = "Bit 21 - Pin E1"]
    #[inline]
    pub fn e1(&mut self) -> _E1W {
        _E1W { w: self }
    }
    #[doc = "Bit 22 - Pin E2"]
    #[inline]
    pub fn e2(&mut self) -> _E2W {
        _E2W { w: self }
    }
    #[doc = "Bit 23 - Pin E3"]
    #[inline]
    pub fn e3(&mut self) -> _E3W {
        _E3W { w: self }
    }
    #[doc = "Bit 8 - GPIOA GC4 Direction"]
    #[inline]
    pub fn gpioa_gc4_ddr(&mut self) -> _GPIOA_GC4_DDRW {
        _GPIOA_GC4_DDRW { w: self }
    }
}
