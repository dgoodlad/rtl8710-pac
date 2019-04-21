#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::GPIO_PORTB_EXT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `PB3`"]
pub type PB3R = ::gpio::gpio_porta_ext::PA5R;
#[doc = "Possible values of the field `PB2`"]
pub type PB2R = ::gpio::gpio_porta_ext::PA5R;
#[doc = "Possible values of the field `PB1`"]
pub type PB1R = ::gpio::gpio_porta_ext::PA5R;
#[doc = "Possible values of the field `PB0`"]
pub type PB0R = ::gpio::gpio_porta_ext::PA5R;
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
