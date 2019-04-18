#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PESOC_CLK_CTRL {
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
pub struct SOC_SLPCK_BTCMD_ENR {
    bits: bool,
}
impl SOC_SLPCK_BTCMD_ENR {
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
pub struct SOC_ACTCK_BTCMD_ENR {
    bits: bool,
}
impl SOC_ACTCK_BTCMD_ENR {
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
pub struct SOC_SLPCK_GPIO_ENR {
    bits: bool,
}
impl SOC_SLPCK_GPIO_ENR {
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
pub struct SOC_ACTCK_GPIO_ENR {
    bits: bool,
}
impl SOC_ACTCK_GPIO_ENR {
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
pub struct SOC_SLPCK_GDMA1_ENR {
    bits: bool,
}
impl SOC_SLPCK_GDMA1_ENR {
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
pub struct SOC_ACTCK_GDMA1_ENR {
    bits: bool,
}
impl SOC_ACTCK_GDMA1_ENR {
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
pub struct SOC_SLPCK_GDMA0_ENR {
    bits: bool,
}
impl SOC_SLPCK_GDMA0_ENR {
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
pub struct SOC_ACTCK_GDMA0_ENR {
    bits: bool,
}
impl SOC_ACTCK_GDMA0_ENR {
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
pub struct SOC_SLPCK_TIMER_ENR {
    bits: bool,
}
impl SOC_SLPCK_TIMER_ENR {
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
pub struct SOC_ACTCK_TIMER_ENR {
    bits: bool,
}
impl SOC_ACTCK_TIMER_ENR {
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
pub struct SOC_SLPCK_LOG_UART_ENR {
    bits: bool,
}
impl SOC_SLPCK_LOG_UART_ENR {
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
pub struct SOC_ACTCK_LOG_UART_ENR {
    bits: bool,
}
impl SOC_ACTCK_LOG_UART_ENR {
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
pub struct SOC_SLPCK_SDR_ENR {
    bits: bool,
}
impl SOC_SLPCK_SDR_ENR {
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
pub struct SOC_ACTCK_SDR_ENR {
    bits: bool,
}
impl SOC_ACTCK_SDR_ENR {
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
pub struct SOC_SLPCK_FLASH_ENR {
    bits: bool,
}
impl SOC_SLPCK_FLASH_ENR {
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
pub struct SOC_ACTCK_FLASH_ENR {
    bits: bool,
}
impl SOC_ACTCK_FLASH_ENR {
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
pub struct SOC_SLPCK_VENDOR_REG_ENR {
    bits: bool,
}
impl SOC_SLPCK_VENDOR_REG_ENR {
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
pub struct SOC_ACTCK_VENDOR_REG_ENR {
    bits: bool,
}
impl SOC_ACTCK_VENDOR_REG_ENR {
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
pub struct SOC_SLPCK_TRACE_ENR {
    bits: bool,
}
impl SOC_SLPCK_TRACE_ENR {
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
pub struct SOC_ACTCK_TRACE_ENR {
    bits: bool,
}
impl SOC_ACTCK_TRACE_ENR {
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
pub struct SOC_CKE_PLFMR {
    bits: bool,
}
impl SOC_CKE_PLFMR {
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
pub struct SOC_CKE_OCPR {
    bits: bool,
}
impl SOC_CKE_OCPR {
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
pub struct _SOC_SLPCK_BTCMD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_SLPCK_BTCMD_ENW<'a> {
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
pub struct _SOC_ACTCK_BTCMD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_ACTCK_BTCMD_ENW<'a> {
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
pub struct _SOC_SLPCK_GPIO_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_SLPCK_GPIO_ENW<'a> {
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
pub struct _SOC_ACTCK_GPIO_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_ACTCK_GPIO_ENW<'a> {
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
pub struct _SOC_SLPCK_GDMA1_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_SLPCK_GDMA1_ENW<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SOC_ACTCK_GDMA1_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_ACTCK_GDMA1_ENW<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SOC_SLPCK_GDMA0_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_SLPCK_GDMA0_ENW<'a> {
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
pub struct _SOC_ACTCK_GDMA0_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_ACTCK_GDMA0_ENW<'a> {
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
pub struct _SOC_SLPCK_TIMER_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_SLPCK_TIMER_ENW<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SOC_ACTCK_TIMER_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_ACTCK_TIMER_ENW<'a> {
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SOC_SLPCK_LOG_UART_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_SLPCK_LOG_UART_ENW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SOC_ACTCK_LOG_UART_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_ACTCK_LOG_UART_ENW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SOC_SLPCK_SDR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_SLPCK_SDR_ENW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SOC_ACTCK_SDR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_ACTCK_SDR_ENW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SOC_SLPCK_FLASH_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_SLPCK_FLASH_ENW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SOC_ACTCK_FLASH_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_ACTCK_FLASH_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _SOC_SLPCK_VENDOR_REG_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_SLPCK_VENDOR_REG_ENW<'a> {
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
pub struct _SOC_ACTCK_VENDOR_REG_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_ACTCK_VENDOR_REG_ENW<'a> {
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
pub struct _SOC_SLPCK_TRACE_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_SLPCK_TRACE_ENW<'a> {
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
pub struct _SOC_ACTCK_TRACE_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_ACTCK_TRACE_ENW<'a> {
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
pub struct _SOC_CKE_PLFMW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_CKE_PLFMW<'a> {
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
pub struct _SOC_CKE_OCPW<'a> {
    w: &'a mut W,
}
impl<'a> _SOC_CKE_OCPW<'a> {
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
    pub fn soc_slpck_btcmd_en(&self) -> SOC_SLPCK_BTCMD_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_SLPCK_BTCMD_ENR { bits }
    }
    #[doc = "Bit 28"]
    #[inline]
    pub fn soc_actck_btcmd_en(&self) -> SOC_ACTCK_BTCMD_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_ACTCK_BTCMD_ENR { bits }
    }
    #[doc = "Bit 25"]
    #[inline]
    pub fn soc_slpck_gpio_en(&self) -> SOC_SLPCK_GPIO_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_SLPCK_GPIO_ENR { bits }
    }
    #[doc = "Bit 24"]
    #[inline]
    pub fn soc_actck_gpio_en(&self) -> SOC_ACTCK_GPIO_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_ACTCK_GPIO_ENR { bits }
    }
    #[doc = "Bit 19"]
    #[inline]
    pub fn soc_slpck_gdma1_en(&self) -> SOC_SLPCK_GDMA1_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_SLPCK_GDMA1_ENR { bits }
    }
    #[doc = "Bit 18"]
    #[inline]
    pub fn soc_actck_gdma1_en(&self) -> SOC_ACTCK_GDMA1_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_ACTCK_GDMA1_ENR { bits }
    }
    #[doc = "Bit 17"]
    #[inline]
    pub fn soc_slpck_gdma0_en(&self) -> SOC_SLPCK_GDMA0_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_SLPCK_GDMA0_ENR { bits }
    }
    #[doc = "Bit 16"]
    #[inline]
    pub fn soc_actck_gdma0_en(&self) -> SOC_ACTCK_GDMA0_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_ACTCK_GDMA0_ENR { bits }
    }
    #[doc = "Bit 15"]
    #[inline]
    pub fn soc_slpck_timer_en(&self) -> SOC_SLPCK_TIMER_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_SLPCK_TIMER_ENR { bits }
    }
    #[doc = "Bit 14"]
    #[inline]
    pub fn soc_actck_timer_en(&self) -> SOC_ACTCK_TIMER_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_ACTCK_TIMER_ENR { bits }
    }
    #[doc = "Bit 13"]
    #[inline]
    pub fn soc_slpck_log_uart_en(&self) -> SOC_SLPCK_LOG_UART_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_SLPCK_LOG_UART_ENR { bits }
    }
    #[doc = "Bit 12"]
    #[inline]
    pub fn soc_actck_log_uart_en(&self) -> SOC_ACTCK_LOG_UART_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_ACTCK_LOG_UART_ENR { bits }
    }
    #[doc = "Bit 11"]
    #[inline]
    pub fn soc_slpck_sdr_en(&self) -> SOC_SLPCK_SDR_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_SLPCK_SDR_ENR { bits }
    }
    #[doc = "Bit 10"]
    #[inline]
    pub fn soc_actck_sdr_en(&self) -> SOC_ACTCK_SDR_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_ACTCK_SDR_ENR { bits }
    }
    #[doc = "Bit 9"]
    #[inline]
    pub fn soc_slpck_flash_en(&self) -> SOC_SLPCK_FLASH_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_SLPCK_FLASH_ENR { bits }
    }
    #[doc = "Bit 8"]
    #[inline]
    pub fn soc_actck_flash_en(&self) -> SOC_ACTCK_FLASH_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_ACTCK_FLASH_ENR { bits }
    }
    #[doc = "Bit 7"]
    #[inline]
    pub fn soc_slpck_vendor_reg_en(&self) -> SOC_SLPCK_VENDOR_REG_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_SLPCK_VENDOR_REG_ENR { bits }
    }
    #[doc = "Bit 6"]
    #[inline]
    pub fn soc_actck_vendor_reg_en(&self) -> SOC_ACTCK_VENDOR_REG_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_ACTCK_VENDOR_REG_ENR { bits }
    }
    #[doc = "Bit 5"]
    #[inline]
    pub fn soc_slpck_trace_en(&self) -> SOC_SLPCK_TRACE_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_SLPCK_TRACE_ENR { bits }
    }
    #[doc = "Bit 4"]
    #[inline]
    pub fn soc_actck_trace_en(&self) -> SOC_ACTCK_TRACE_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_ACTCK_TRACE_ENR { bits }
    }
    #[doc = "Bit 2"]
    #[inline]
    pub fn soc_cke_plfm(&self) -> SOC_CKE_PLFMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_CKE_PLFMR { bits }
    }
    #[doc = "Bit 0"]
    #[inline]
    pub fn soc_cke_ocp(&self) -> SOC_CKE_OCPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOC_CKE_OCPR { bits }
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
    pub fn soc_slpck_btcmd_en(&mut self) -> _SOC_SLPCK_BTCMD_ENW {
        _SOC_SLPCK_BTCMD_ENW { w: self }
    }
    #[doc = "Bit 28"]
    #[inline]
    pub fn soc_actck_btcmd_en(&mut self) -> _SOC_ACTCK_BTCMD_ENW {
        _SOC_ACTCK_BTCMD_ENW { w: self }
    }
    #[doc = "Bit 25"]
    #[inline]
    pub fn soc_slpck_gpio_en(&mut self) -> _SOC_SLPCK_GPIO_ENW {
        _SOC_SLPCK_GPIO_ENW { w: self }
    }
    #[doc = "Bit 24"]
    #[inline]
    pub fn soc_actck_gpio_en(&mut self) -> _SOC_ACTCK_GPIO_ENW {
        _SOC_ACTCK_GPIO_ENW { w: self }
    }
    #[doc = "Bit 19"]
    #[inline]
    pub fn soc_slpck_gdma1_en(&mut self) -> _SOC_SLPCK_GDMA1_ENW {
        _SOC_SLPCK_GDMA1_ENW { w: self }
    }
    #[doc = "Bit 18"]
    #[inline]
    pub fn soc_actck_gdma1_en(&mut self) -> _SOC_ACTCK_GDMA1_ENW {
        _SOC_ACTCK_GDMA1_ENW { w: self }
    }
    #[doc = "Bit 17"]
    #[inline]
    pub fn soc_slpck_gdma0_en(&mut self) -> _SOC_SLPCK_GDMA0_ENW {
        _SOC_SLPCK_GDMA0_ENW { w: self }
    }
    #[doc = "Bit 16"]
    #[inline]
    pub fn soc_actck_gdma0_en(&mut self) -> _SOC_ACTCK_GDMA0_ENW {
        _SOC_ACTCK_GDMA0_ENW { w: self }
    }
    #[doc = "Bit 15"]
    #[inline]
    pub fn soc_slpck_timer_en(&mut self) -> _SOC_SLPCK_TIMER_ENW {
        _SOC_SLPCK_TIMER_ENW { w: self }
    }
    #[doc = "Bit 14"]
    #[inline]
    pub fn soc_actck_timer_en(&mut self) -> _SOC_ACTCK_TIMER_ENW {
        _SOC_ACTCK_TIMER_ENW { w: self }
    }
    #[doc = "Bit 13"]
    #[inline]
    pub fn soc_slpck_log_uart_en(&mut self) -> _SOC_SLPCK_LOG_UART_ENW {
        _SOC_SLPCK_LOG_UART_ENW { w: self }
    }
    #[doc = "Bit 12"]
    #[inline]
    pub fn soc_actck_log_uart_en(&mut self) -> _SOC_ACTCK_LOG_UART_ENW {
        _SOC_ACTCK_LOG_UART_ENW { w: self }
    }
    #[doc = "Bit 11"]
    #[inline]
    pub fn soc_slpck_sdr_en(&mut self) -> _SOC_SLPCK_SDR_ENW {
        _SOC_SLPCK_SDR_ENW { w: self }
    }
    #[doc = "Bit 10"]
    #[inline]
    pub fn soc_actck_sdr_en(&mut self) -> _SOC_ACTCK_SDR_ENW {
        _SOC_ACTCK_SDR_ENW { w: self }
    }
    #[doc = "Bit 9"]
    #[inline]
    pub fn soc_slpck_flash_en(&mut self) -> _SOC_SLPCK_FLASH_ENW {
        _SOC_SLPCK_FLASH_ENW { w: self }
    }
    #[doc = "Bit 8"]
    #[inline]
    pub fn soc_actck_flash_en(&mut self) -> _SOC_ACTCK_FLASH_ENW {
        _SOC_ACTCK_FLASH_ENW { w: self }
    }
    #[doc = "Bit 7"]
    #[inline]
    pub fn soc_slpck_vendor_reg_en(&mut self) -> _SOC_SLPCK_VENDOR_REG_ENW {
        _SOC_SLPCK_VENDOR_REG_ENW { w: self }
    }
    #[doc = "Bit 6"]
    #[inline]
    pub fn soc_actck_vendor_reg_en(&mut self) -> _SOC_ACTCK_VENDOR_REG_ENW {
        _SOC_ACTCK_VENDOR_REG_ENW { w: self }
    }
    #[doc = "Bit 5"]
    #[inline]
    pub fn soc_slpck_trace_en(&mut self) -> _SOC_SLPCK_TRACE_ENW {
        _SOC_SLPCK_TRACE_ENW { w: self }
    }
    #[doc = "Bit 4"]
    #[inline]
    pub fn soc_actck_trace_en(&mut self) -> _SOC_ACTCK_TRACE_ENW {
        _SOC_ACTCK_TRACE_ENW { w: self }
    }
    #[doc = "Bit 2"]
    #[inline]
    pub fn soc_cke_plfm(&mut self) -> _SOC_CKE_PLFMW {
        _SOC_CKE_PLFMW { w: self }
    }
    #[doc = "Bit 0"]
    #[inline]
    pub fn soc_cke_ocp(&mut self) -> _SOC_CKE_OCPW {
        _SOC_CKE_OCPW { w: self }
    }
}
