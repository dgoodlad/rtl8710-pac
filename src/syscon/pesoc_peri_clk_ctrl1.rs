#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PESOC_PERI_CLK_CTRL1 {
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
pub struct SOC_SLPCK_DAC_ENR {
    bits: bool,
}
impl SOC_SLPCK_DAC_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct SOC_ACTCK_DAC_ENR {
    bits: bool,
}
impl SOC_ACTCK_DAC_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct SOC_SLPCK_ADC_ENR {
    bits: bool,
}
impl SOC_SLPCK_ADC_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct SOC_ACTCK_ADC_ENR {
    bits: bool,
}
impl SOC_ACTCK_ADC_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct SOC_SLPCK_PCM_ENR {
    bits: bool,
}
impl SOC_SLPCK_PCM_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct SOC_ACTCK_PCM_ENR {
    bits: bool,
}
impl SOC_ACTCK_PCM_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct SOC_SLPCK_I2S_ENR {
    bits: bool,
}
impl SOC_SLPCK_I2S_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct SOC_ACTCK_I2S_ENR {
    bits: bool,
}
impl SOC_ACTCK_I2S_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct SOC_SLPCK_I2C3_ENR {
    bits: bool,
}
impl SOC_SLPCK_I2C3_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct SOC_ACTCK_I2C3_ENR {
    bits: bool,
}
impl SOC_ACTCK_I2C3_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct SOC_SLPCK_I2C2_ENR {
    bits: bool,
}
impl SOC_SLPCK_I2C2_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct SOC_ACTCK_I2C2_ENR {
    bits: bool,
}
impl SOC_ACTCK_I2C2_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct SOC_SLPCK_I2C1_ENR {
    bits: bool,
}
impl SOC_SLPCK_I2C1_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct SOC_ACTCK_I2C1_ENR {
    bits: bool,
}
impl SOC_ACTCK_I2C1_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct SOC_SLPCK_I2C0_ENR {
    bits: bool,
}
impl SOC_SLPCK_I2C0_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct SOC_ACTCK_I2C0_ENR {
    bits: bool,
}
impl SOC_ACTCK_I2C0_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Proxy"]
pub struct _SOC_SLPCK_DAC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_SLPCK_DAC_ENW<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SOC_ACTCK_DAC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_ACTCK_DAC_ENW<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SOC_SLPCK_ADC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_SLPCK_ADC_ENW<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SOC_ACTCK_ADC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_ACTCK_ADC_ENW<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SOC_SLPCK_PCM_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_SLPCK_PCM_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _SOC_ACTCK_PCM_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_ACTCK_PCM_ENW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SOC_SLPCK_I2S_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_SLPCK_I2S_ENW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SOC_ACTCK_I2S_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_ACTCK_I2S_ENW<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SOC_SLPCK_I2C3_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_SLPCK_I2C3_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _SOC_ACTCK_I2C3_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_ACTCK_I2C3_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _SOC_SLPCK_I2C2_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_SLPCK_I2C2_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _SOC_ACTCK_I2C2_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_ACTCK_I2C2_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _SOC_SLPCK_I2C1_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_SLPCK_I2C1_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _SOC_ACTCK_I2C1_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_ACTCK_I2C1_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _SOC_SLPCK_I2C0_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_SLPCK_I2C0_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _SOC_ACTCK_I2C0_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_ACTCK_I2C0_ENW<'a> {
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
    #[doc = "Bit 29"]
    #[inline]
    pub fn soc_slpck_dac_en(&self) -> SOC_SLPCK_DAC_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_SLPCK_DAC_ENR { bits }
    }
    #[doc = "Bit 28"]
    #[inline]
    pub fn soc_actck_dac_en(&self) -> SOC_ACTCK_DAC_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_ACTCK_DAC_ENR { bits }
    }
    #[doc = "Bit 25"]
    #[inline]
    pub fn soc_slpck_adc_en(&self) -> SOC_SLPCK_ADC_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_SLPCK_ADC_ENR { bits }
    }
    #[doc = "Bit 24"]
    #[inline]
    pub fn soc_actck_adc_en(&self) -> SOC_ACTCK_ADC_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_ACTCK_ADC_ENR { bits }
    }
    #[doc = "Bit 21"]
    #[inline]
    pub fn soc_slpck_pcm_en(&self) -> SOC_SLPCK_PCM_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_SLPCK_PCM_ENR { bits }
    }
    #[doc = "Bit 20"]
    #[inline]
    pub fn soc_actck_pcm_en(&self) -> SOC_ACTCK_PCM_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_ACTCK_PCM_ENR { bits }
    }
    #[doc = "Bit 17"]
    #[inline]
    pub fn soc_slpck_i2s_en(&self) -> SOC_SLPCK_I2S_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_SLPCK_I2S_ENR { bits }
    }
    #[doc = "Bit 16"]
    #[inline]
    pub fn soc_actck_i2s_en(&self) -> SOC_ACTCK_I2S_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_ACTCK_I2S_ENR { bits }
    }
    #[doc = "Bit 7"]
    #[inline]
    pub fn soc_slpck_i2c3_en(&self) -> SOC_SLPCK_I2C3_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_SLPCK_I2C3_ENR { bits }
    }
    #[doc = "Bit 6"]
    #[inline]
    pub fn soc_actck_i2c3_en(&self) -> SOC_ACTCK_I2C3_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_ACTCK_I2C3_ENR { bits }
    }
    #[doc = "Bit 5"]
    #[inline]
    pub fn soc_slpck_i2c2_en(&self) -> SOC_SLPCK_I2C2_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_SLPCK_I2C2_ENR { bits }
    }
    #[doc = "Bit 4"]
    #[inline]
    pub fn soc_actck_i2c2_en(&self) -> SOC_ACTCK_I2C2_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_ACTCK_I2C2_ENR { bits }
    }
    #[doc = "Bit 3"]
    #[inline]
    pub fn soc_slpck_i2c1_en(&self) -> SOC_SLPCK_I2C1_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_SLPCK_I2C1_ENR { bits }
    }
    #[doc = "Bit 2"]
    #[inline]
    pub fn soc_actck_i2c1_en(&self) -> SOC_ACTCK_I2C1_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_ACTCK_I2C1_ENR { bits }
    }
    #[doc = "Bit 1"]
    #[inline]
    pub fn soc_slpck_i2c0_en(&self) -> SOC_SLPCK_I2C0_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_SLPCK_I2C0_ENR { bits }
    }
    #[doc = "Bit 0"]
    #[inline]
    pub fn soc_actck_i2c0_en(&self) -> SOC_ACTCK_I2C0_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_ACTCK_I2C0_ENR { bits }
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
    #[doc = "Bit 29"]
    #[inline]
    pub fn soc_slpck_dac_en(&mut self) -> _SOC_SLPCK_DAC_ENW {
        _SOC_SLPCK_DAC_ENW { w: self }
    }
    #[doc = "Bit 28"]
    #[inline]
    pub fn soc_actck_dac_en(&mut self) -> _SOC_ACTCK_DAC_ENW {
        _SOC_ACTCK_DAC_ENW { w: self }
    }
    #[doc = "Bit 25"]
    #[inline]
    pub fn soc_slpck_adc_en(&mut self) -> _SOC_SLPCK_ADC_ENW {
        _SOC_SLPCK_ADC_ENW { w: self }
    }
    #[doc = "Bit 24"]
    #[inline]
    pub fn soc_actck_adc_en(&mut self) -> _SOC_ACTCK_ADC_ENW {
        _SOC_ACTCK_ADC_ENW { w: self }
    }
    #[doc = "Bit 21"]
    #[inline]
    pub fn soc_slpck_pcm_en(&mut self) -> _SOC_SLPCK_PCM_ENW {
        _SOC_SLPCK_PCM_ENW { w: self }
    }
    #[doc = "Bit 20"]
    #[inline]
    pub fn soc_actck_pcm_en(&mut self) -> _SOC_ACTCK_PCM_ENW {
        _SOC_ACTCK_PCM_ENW { w: self }
    }
    #[doc = "Bit 17"]
    #[inline]
    pub fn soc_slpck_i2s_en(&mut self) -> _SOC_SLPCK_I2S_ENW {
        _SOC_SLPCK_I2S_ENW { w: self }
    }
    #[doc = "Bit 16"]
    #[inline]
    pub fn soc_actck_i2s_en(&mut self) -> _SOC_ACTCK_I2S_ENW {
        _SOC_ACTCK_I2S_ENW { w: self }
    }
    #[doc = "Bit 7"]
    #[inline]
    pub fn soc_slpck_i2c3_en(&mut self) -> _SOC_SLPCK_I2C3_ENW {
        _SOC_SLPCK_I2C3_ENW { w: self }
    }
    #[doc = "Bit 6"]
    #[inline]
    pub fn soc_actck_i2c3_en(&mut self) -> _SOC_ACTCK_I2C3_ENW {
        _SOC_ACTCK_I2C3_ENW { w: self }
    }
    #[doc = "Bit 5"]
    #[inline]
    pub fn soc_slpck_i2c2_en(&mut self) -> _SOC_SLPCK_I2C2_ENW {
        _SOC_SLPCK_I2C2_ENW { w: self }
    }
    #[doc = "Bit 4"]
    #[inline]
    pub fn soc_actck_i2c2_en(&mut self) -> _SOC_ACTCK_I2C2_ENW {
        _SOC_ACTCK_I2C2_ENW { w: self }
    }
    #[doc = "Bit 3"]
    #[inline]
    pub fn soc_slpck_i2c1_en(&mut self) -> _SOC_SLPCK_I2C1_ENW {
        _SOC_SLPCK_I2C1_ENW { w: self }
    }
    #[doc = "Bit 2"]
    #[inline]
    pub fn soc_actck_i2c1_en(&mut self) -> _SOC_ACTCK_I2C1_ENW {
        _SOC_ACTCK_I2C1_ENW { w: self }
    }
    #[doc = "Bit 1"]
    #[inline]
    pub fn soc_slpck_i2c0_en(&mut self) -> _SOC_SLPCK_I2C0_ENW {
        _SOC_SLPCK_I2C0_ENW { w: self }
    }
    #[doc = "Bit 0"]
    #[inline]
    pub fn soc_actck_i2c0_en(&mut self) -> _SOC_ACTCK_I2C0_ENW {
        _SOC_ACTCK_I2C0_ENW { w: self }
    }
}
