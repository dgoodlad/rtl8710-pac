#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SLP_WAKE_EVENT_MSK0 {
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
pub struct SYSON_WEVT_GPIO_DSTBY_MSKR {
    bits: bool,
}
impl SYSON_WEVT_GPIO_DSTBY_MSKR {
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
pub struct SYSON_WEVT_A33_MSKR {
    bits: bool,
}
impl SYSON_WEVT_A33_MSKR {
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
pub struct SYSON_WEVT_ADC_MSKR {
    bits: bool,
}
impl SYSON_WEVT_ADC_MSKR {
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
pub struct SYSON_WEVT_I2C_MSKR {
    bits: bool,
}
impl SYSON_WEVT_I2C_MSKR {
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
pub struct SYSON_WEVT_SPI_MSKR {
    bits: bool,
}
impl SYSON_WEVT_SPI_MSKR {
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
pub struct SYSON_WEVT_UART_MSKR {
    bits: bool,
}
impl SYSON_WEVT_UART_MSKR {
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
pub struct SYSON_WEVT_USB_MSKR {
    bits: bool,
}
impl SYSON_WEVT_USB_MSKR {
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
pub struct SYSON_WEVT_SDIO_MSKR {
    bits: bool,
}
impl SYSON_WEVT_SDIO_MSKR {
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
pub struct SYSON_WEVT_NFC_MSKR {
    bits: bool,
}
impl SYSON_WEVT_NFC_MSKR {
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
pub struct SYSON_WEVT_WLAN_MSKR {
    bits: bool,
}
impl SYSON_WEVT_WLAN_MSKR {
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
pub struct SYSON_WEVT_GPIO_MSKR {
    bits: bool,
}
impl SYSON_WEVT_GPIO_MSKR {
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
pub struct SYSON_WEVT_CHIP_EN_MSKR {
    bits: bool,
}
impl SYSON_WEVT_CHIP_EN_MSKR {
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
pub struct SYSON_WEVT_OVER_CURRENT_MSKR {
    bits: bool,
}
impl SYSON_WEVT_OVER_CURRENT_MSKR {
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
pub struct SYSON_WEVT_GTIM_MSKR {
    bits: bool,
}
impl SYSON_WEVT_GTIM_MSKR {
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
pub struct SYSON_WEVT_SYSTIM_MSKR {
    bits: bool,
}
impl SYSON_WEVT_SYSTIM_MSKR {
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
pub struct _SYSON_WEVT_GPIO_DSTBY_MSKW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_WEVT_GPIO_DSTBY_MSKW<'a> {
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
pub struct _SYSON_WEVT_A33_MSKW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_WEVT_A33_MSKW<'a> {
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
pub struct _SYSON_WEVT_ADC_MSKW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_WEVT_ADC_MSKW<'a> {
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
pub struct _SYSON_WEVT_I2C_MSKW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_WEVT_I2C_MSKW<'a> {
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
pub struct _SYSON_WEVT_SPI_MSKW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_WEVT_SPI_MSKW<'a> {
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
pub struct _SYSON_WEVT_UART_MSKW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_WEVT_UART_MSKW<'a> {
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
pub struct _SYSON_WEVT_USB_MSKW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_WEVT_USB_MSKW<'a> {
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
pub struct _SYSON_WEVT_SDIO_MSKW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_WEVT_SDIO_MSKW<'a> {
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
pub struct _SYSON_WEVT_NFC_MSKW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_WEVT_NFC_MSKW<'a> {
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
pub struct _SYSON_WEVT_WLAN_MSKW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_WEVT_WLAN_MSKW<'a> {
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
pub struct _SYSON_WEVT_GPIO_MSKW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_WEVT_GPIO_MSKW<'a> {
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
pub struct _SYSON_WEVT_CHIP_EN_MSKW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_WEVT_CHIP_EN_MSKW<'a> {
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
pub struct _SYSON_WEVT_OVER_CURRENT_MSKW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_WEVT_OVER_CURRENT_MSKW<'a> {
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
pub struct _SYSON_WEVT_GTIM_MSKW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_WEVT_GTIM_MSKW<'a> {
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
pub struct _SYSON_WEVT_SYSTIM_MSKW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSON_WEVT_SYSTIM_MSKW<'a> {
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
    pub fn syson_wevt_gpio_dstby_msk(&self) -> SYSON_WEVT_GPIO_DSTBY_MSKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_WEVT_GPIO_DSTBY_MSKR { bits }
    }
    #[doc = "Bit 28"]
    #[inline]
    pub fn syson_wevt_a33_msk(&self) -> SYSON_WEVT_A33_MSKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_WEVT_A33_MSKR { bits }
    }
    #[doc = "Bit 26"]
    #[inline]
    pub fn syson_wevt_adc_msk(&self) -> SYSON_WEVT_ADC_MSKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_WEVT_ADC_MSKR { bits }
    }
    #[doc = "Bit 24"]
    #[inline]
    pub fn syson_wevt_i2c_msk(&self) -> SYSON_WEVT_I2C_MSKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_WEVT_I2C_MSKR { bits }
    }
    #[doc = "Bit 22"]
    #[inline]
    pub fn syson_wevt_spi_msk(&self) -> SYSON_WEVT_SPI_MSKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_WEVT_SPI_MSKR { bits }
    }
    #[doc = "Bit 20"]
    #[inline]
    pub fn syson_wevt_uart_msk(&self) -> SYSON_WEVT_UART_MSKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_WEVT_UART_MSKR { bits }
    }
    #[doc = "Bit 16"]
    #[inline]
    pub fn syson_wevt_usb_msk(&self) -> SYSON_WEVT_USB_MSKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_WEVT_USB_MSKR { bits }
    }
    #[doc = "Bit 14"]
    #[inline]
    pub fn syson_wevt_sdio_msk(&self) -> SYSON_WEVT_SDIO_MSKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_WEVT_SDIO_MSKR { bits }
    }
    #[doc = "Bit 9"]
    #[inline]
    pub fn syson_wevt_nfc_msk(&self) -> SYSON_WEVT_NFC_MSKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_WEVT_NFC_MSKR { bits }
    }
    #[doc = "Bit 8"]
    #[inline]
    pub fn syson_wevt_wlan_msk(&self) -> SYSON_WEVT_WLAN_MSKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_WEVT_WLAN_MSKR { bits }
    }
    #[doc = "Bit 4"]
    #[inline]
    pub fn syson_wevt_gpio_msk(&self) -> SYSON_WEVT_GPIO_MSKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_WEVT_GPIO_MSKR { bits }
    }
    #[doc = "Bit 3"]
    #[inline]
    pub fn syson_wevt_chip_en_msk(&self) -> SYSON_WEVT_CHIP_EN_MSKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_WEVT_CHIP_EN_MSKR { bits }
    }
    #[doc = "Bit 2"]
    #[inline]
    pub fn syson_wevt_over_current_msk(&self) -> SYSON_WEVT_OVER_CURRENT_MSKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_WEVT_OVER_CURRENT_MSKR { bits }
    }
    #[doc = "Bit 1"]
    #[inline]
    pub fn syson_wevt_gtim_msk(&self) -> SYSON_WEVT_GTIM_MSKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_WEVT_GTIM_MSKR { bits }
    }
    #[doc = "Bit 0"]
    #[inline]
    pub fn syson_wevt_systim_msk(&self) -> SYSON_WEVT_SYSTIM_MSKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSON_WEVT_SYSTIM_MSKR { bits }
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
    pub fn syson_wevt_gpio_dstby_msk(&mut self) -> _SYSON_WEVT_GPIO_DSTBY_MSKW {
        _SYSON_WEVT_GPIO_DSTBY_MSKW { w: self }
    }
    #[doc = "Bit 28"]
    #[inline]
    pub fn syson_wevt_a33_msk(&mut self) -> _SYSON_WEVT_A33_MSKW {
        _SYSON_WEVT_A33_MSKW { w: self }
    }
    #[doc = "Bit 26"]
    #[inline]
    pub fn syson_wevt_adc_msk(&mut self) -> _SYSON_WEVT_ADC_MSKW {
        _SYSON_WEVT_ADC_MSKW { w: self }
    }
    #[doc = "Bit 24"]
    #[inline]
    pub fn syson_wevt_i2c_msk(&mut self) -> _SYSON_WEVT_I2C_MSKW {
        _SYSON_WEVT_I2C_MSKW { w: self }
    }
    #[doc = "Bit 22"]
    #[inline]
    pub fn syson_wevt_spi_msk(&mut self) -> _SYSON_WEVT_SPI_MSKW {
        _SYSON_WEVT_SPI_MSKW { w: self }
    }
    #[doc = "Bit 20"]
    #[inline]
    pub fn syson_wevt_uart_msk(&mut self) -> _SYSON_WEVT_UART_MSKW {
        _SYSON_WEVT_UART_MSKW { w: self }
    }
    #[doc = "Bit 16"]
    #[inline]
    pub fn syson_wevt_usb_msk(&mut self) -> _SYSON_WEVT_USB_MSKW {
        _SYSON_WEVT_USB_MSKW { w: self }
    }
    #[doc = "Bit 14"]
    #[inline]
    pub fn syson_wevt_sdio_msk(&mut self) -> _SYSON_WEVT_SDIO_MSKW {
        _SYSON_WEVT_SDIO_MSKW { w: self }
    }
    #[doc = "Bit 9"]
    #[inline]
    pub fn syson_wevt_nfc_msk(&mut self) -> _SYSON_WEVT_NFC_MSKW {
        _SYSON_WEVT_NFC_MSKW { w: self }
    }
    #[doc = "Bit 8"]
    #[inline]
    pub fn syson_wevt_wlan_msk(&mut self) -> _SYSON_WEVT_WLAN_MSKW {
        _SYSON_WEVT_WLAN_MSKW { w: self }
    }
    #[doc = "Bit 4"]
    #[inline]
    pub fn syson_wevt_gpio_msk(&mut self) -> _SYSON_WEVT_GPIO_MSKW {
        _SYSON_WEVT_GPIO_MSKW { w: self }
    }
    #[doc = "Bit 3"]
    #[inline]
    pub fn syson_wevt_chip_en_msk(&mut self) -> _SYSON_WEVT_CHIP_EN_MSKW {
        _SYSON_WEVT_CHIP_EN_MSKW { w: self }
    }
    #[doc = "Bit 2"]
    #[inline]
    pub fn syson_wevt_over_current_msk(&mut self) -> _SYSON_WEVT_OVER_CURRENT_MSKW {
        _SYSON_WEVT_OVER_CURRENT_MSKW { w: self }
    }
    #[doc = "Bit 1"]
    #[inline]
    pub fn syson_wevt_gtim_msk(&mut self) -> _SYSON_WEVT_GTIM_MSKW {
        _SYSON_WEVT_GTIM_MSKW { w: self }
    }
    #[doc = "Bit 0"]
    #[inline]
    pub fn syson_wevt_systim_msk(&mut self) -> _SYSON_WEVT_SYSTIM_MSKW {
        _SYSON_WEVT_SYSTIM_MSKW { w: self }
    }
}
