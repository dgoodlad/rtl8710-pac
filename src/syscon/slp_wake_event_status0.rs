#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SLP_WAKE_EVENT_STATUS0 {
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
pub struct SYSON_WEVT_GPIO_DSTBY_STSR {
    bits: bool,
}
impl SYSON_WEVT_GPIO_DSTBY_STSR {
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
pub struct SYSON_WEVT_A33_STSR {
    bits: bool,
}
impl SYSON_WEVT_A33_STSR {
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
pub struct SYSON_WEVT_ADC_STSR {
    bits: bool,
}
impl SYSON_WEVT_ADC_STSR {
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
pub struct SYSON_WEVT_I2C_STSR {
    bits: bool,
}
impl SYSON_WEVT_I2C_STSR {
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
pub struct SYSON_WEVT_SPI_STSR {
    bits: bool,
}
impl SYSON_WEVT_SPI_STSR {
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
pub struct SYSON_WEVT_UART_STSR {
    bits: bool,
}
impl SYSON_WEVT_UART_STSR {
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
pub struct SYSON_WEVT_USB_STSR {
    bits: bool,
}
impl SYSON_WEVT_USB_STSR {
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
pub struct SYSON_WEVT_SDIO_STSR {
    bits: bool,
}
impl SYSON_WEVT_SDIO_STSR {
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
pub struct SYSON_WEVT_NFC_STSR {
    bits: bool,
}
impl SYSON_WEVT_NFC_STSR {
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
pub struct SYSON_WEVT_WLAN_STSR {
    bits: bool,
}
impl SYSON_WEVT_WLAN_STSR {
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
pub struct SYSON_WEVT_GPIO_STSR {
    bits: bool,
}
impl SYSON_WEVT_GPIO_STSR {
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
pub struct SYSON_WEVT_CHIP_EN_STSR {
    bits: bool,
}
impl SYSON_WEVT_CHIP_EN_STSR {
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
pub struct SYSON_WEVT_OVER_CURRENT_STSR {
    bits: bool,
}
impl SYSON_WEVT_OVER_CURRENT_STSR {
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
pub struct SYSON_WEVT_GTIM_STSR {
    bits: bool,
}
impl SYSON_WEVT_GTIM_STSR {
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
pub struct SYSON_WEVT_SYSTIM_STSR {
    bits: bool,
}
impl SYSON_WEVT_SYSTIM_STSR {
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
pub struct _SYSON_WEVT_GPIO_DSTBY_STSW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_WEVT_GPIO_DSTBY_STSW<'a> {
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
pub struct _SYSON_WEVT_A33_STSW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_WEVT_A33_STSW<'a> {
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
pub struct _SYSON_WEVT_ADC_STSW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_WEVT_ADC_STSW<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYSON_WEVT_I2C_STSW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_WEVT_I2C_STSW<'a> {
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
pub struct _SYSON_WEVT_SPI_STSW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_WEVT_SPI_STSW<'a> {
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
#[doc = r" Proxy"]
pub struct _SYSON_WEVT_UART_STSW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_WEVT_UART_STSW<'a> {
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
pub struct _SYSON_WEVT_USB_STSW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_WEVT_USB_STSW<'a> {
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
pub struct _SYSON_WEVT_SDIO_STSW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_WEVT_SDIO_STSW<'a> {
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
pub struct _SYSON_WEVT_NFC_STSW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_WEVT_NFC_STSW<'a> {
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
pub struct _SYSON_WEVT_WLAN_STSW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_WEVT_WLAN_STSW<'a> {
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
pub struct _SYSON_WEVT_GPIO_STSW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_WEVT_GPIO_STSW<'a> {
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
pub struct _SYSON_WEVT_CHIP_EN_STSW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_WEVT_CHIP_EN_STSW<'a> {
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
pub struct _SYSON_WEVT_OVER_CURRENT_STSW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_WEVT_OVER_CURRENT_STSW<'a> {
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
pub struct _SYSON_WEVT_GTIM_STSW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_WEVT_GTIM_STSW<'a> {
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
pub struct _SYSON_WEVT_SYSTIM_STSW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_WEVT_SYSTIM_STSW<'a> {
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
    pub fn syson_wevt_gpio_dstby_sts(&self) -> SYSON_WEVT_GPIO_DSTBY_STSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_WEVT_GPIO_DSTBY_STSR { bits }
    }
    #[doc = "Bit 28"]
    #[inline]
    pub fn syson_wevt_a33_sts(&self) -> SYSON_WEVT_A33_STSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_WEVT_A33_STSR { bits }
    }
    #[doc = "Bit 26"]
    #[inline]
    pub fn syson_wevt_adc_sts(&self) -> SYSON_WEVT_ADC_STSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_WEVT_ADC_STSR { bits }
    }
    #[doc = "Bit 24"]
    #[inline]
    pub fn syson_wevt_i2c_sts(&self) -> SYSON_WEVT_I2C_STSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_WEVT_I2C_STSR { bits }
    }
    #[doc = "Bit 22"]
    #[inline]
    pub fn syson_wevt_spi_sts(&self) -> SYSON_WEVT_SPI_STSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_WEVT_SPI_STSR { bits }
    }
    #[doc = "Bit 20"]
    #[inline]
    pub fn syson_wevt_uart_sts(&self) -> SYSON_WEVT_UART_STSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_WEVT_UART_STSR { bits }
    }
    #[doc = "Bit 16"]
    #[inline]
    pub fn syson_wevt_usb_sts(&self) -> SYSON_WEVT_USB_STSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_WEVT_USB_STSR { bits }
    }
    #[doc = "Bit 14"]
    #[inline]
    pub fn syson_wevt_sdio_sts(&self) -> SYSON_WEVT_SDIO_STSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_WEVT_SDIO_STSR { bits }
    }
    #[doc = "Bit 9"]
    #[inline]
    pub fn syson_wevt_nfc_sts(&self) -> SYSON_WEVT_NFC_STSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_WEVT_NFC_STSR { bits }
    }
    #[doc = "Bit 8"]
    #[inline]
    pub fn syson_wevt_wlan_sts(&self) -> SYSON_WEVT_WLAN_STSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_WEVT_WLAN_STSR { bits }
    }
    #[doc = "Bit 4"]
    #[inline]
    pub fn syson_wevt_gpio_sts(&self) -> SYSON_WEVT_GPIO_STSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_WEVT_GPIO_STSR { bits }
    }
    #[doc = "Bit 3"]
    #[inline]
    pub fn syson_wevt_chip_en_sts(&self) -> SYSON_WEVT_CHIP_EN_STSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_WEVT_CHIP_EN_STSR { bits }
    }
    #[doc = "Bit 2"]
    #[inline]
    pub fn syson_wevt_over_current_sts(&self) -> SYSON_WEVT_OVER_CURRENT_STSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_WEVT_OVER_CURRENT_STSR { bits }
    }
    #[doc = "Bit 1"]
    #[inline]
    pub fn syson_wevt_gtim_sts(&self) -> SYSON_WEVT_GTIM_STSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_WEVT_GTIM_STSR { bits }
    }
    #[doc = "Bit 0"]
    #[inline]
    pub fn syson_wevt_systim_sts(&self) -> SYSON_WEVT_SYSTIM_STSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_WEVT_SYSTIM_STSR { bits }
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
    pub fn syson_wevt_gpio_dstby_sts(&mut self) -> _SYSON_WEVT_GPIO_DSTBY_STSW {
        _SYSON_WEVT_GPIO_DSTBY_STSW { w: self }
    }
    #[doc = "Bit 28"]
    #[inline]
    pub fn syson_wevt_a33_sts(&mut self) -> _SYSON_WEVT_A33_STSW {
        _SYSON_WEVT_A33_STSW { w: self }
    }
    #[doc = "Bit 26"]
    #[inline]
    pub fn syson_wevt_adc_sts(&mut self) -> _SYSON_WEVT_ADC_STSW {
        _SYSON_WEVT_ADC_STSW { w: self }
    }
    #[doc = "Bit 24"]
    #[inline]
    pub fn syson_wevt_i2c_sts(&mut self) -> _SYSON_WEVT_I2C_STSW {
        _SYSON_WEVT_I2C_STSW { w: self }
    }
    #[doc = "Bit 22"]
    #[inline]
    pub fn syson_wevt_spi_sts(&mut self) -> _SYSON_WEVT_SPI_STSW {
        _SYSON_WEVT_SPI_STSW { w: self }
    }
    #[doc = "Bit 20"]
    #[inline]
    pub fn syson_wevt_uart_sts(&mut self) -> _SYSON_WEVT_UART_STSW {
        _SYSON_WEVT_UART_STSW { w: self }
    }
    #[doc = "Bit 16"]
    #[inline]
    pub fn syson_wevt_usb_sts(&mut self) -> _SYSON_WEVT_USB_STSW {
        _SYSON_WEVT_USB_STSW { w: self }
    }
    #[doc = "Bit 14"]
    #[inline]
    pub fn syson_wevt_sdio_sts(&mut self) -> _SYSON_WEVT_SDIO_STSW {
        _SYSON_WEVT_SDIO_STSW { w: self }
    }
    #[doc = "Bit 9"]
    #[inline]
    pub fn syson_wevt_nfc_sts(&mut self) -> _SYSON_WEVT_NFC_STSW {
        _SYSON_WEVT_NFC_STSW { w: self }
    }
    #[doc = "Bit 8"]
    #[inline]
    pub fn syson_wevt_wlan_sts(&mut self) -> _SYSON_WEVT_WLAN_STSW {
        _SYSON_WEVT_WLAN_STSW { w: self }
    }
    #[doc = "Bit 4"]
    #[inline]
    pub fn syson_wevt_gpio_sts(&mut self) -> _SYSON_WEVT_GPIO_STSW {
        _SYSON_WEVT_GPIO_STSW { w: self }
    }
    #[doc = "Bit 3"]
    #[inline]
    pub fn syson_wevt_chip_en_sts(&mut self) -> _SYSON_WEVT_CHIP_EN_STSW {
        _SYSON_WEVT_CHIP_EN_STSW { w: self }
    }
    #[doc = "Bit 2"]
    #[inline]
    pub fn syson_wevt_over_current_sts(&mut self) -> _SYSON_WEVT_OVER_CURRENT_STSW {
        _SYSON_WEVT_OVER_CURRENT_STSW { w: self }
    }
    #[doc = "Bit 1"]
    #[inline]
    pub fn syson_wevt_gtim_sts(&mut self) -> _SYSON_WEVT_GTIM_STSW {
        _SYSON_WEVT_GTIM_STSW { w: self }
    }
    #[doc = "Bit 0"]
    #[inline]
    pub fn syson_wevt_systim_sts(&mut self) -> _SYSON_WEVT_SYSTIM_STSW {
        _SYSON_WEVT_SYSTIM_STSW { w: self }
    }
}
