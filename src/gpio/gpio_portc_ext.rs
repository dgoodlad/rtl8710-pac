#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::GPIO_PORTC_EXT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `PC5`"]
pub type PC5R = ::gpio::gpio_porta_ext::PA5R;
#[doc = "Possible values of the field `PC4`"]
pub type PC4R = ::gpio::gpio_porta_ext::PA5R;
#[doc = "Possible values of the field `PC3`"]
pub type PC3R = ::gpio::gpio_porta_ext::PA5R;
#[doc = "Possible values of the field `PC2`"]
pub type PC2R = ::gpio::gpio_porta_ext::PA5R;
#[doc = "Possible values of the field `PC1`"]
pub type PC1R = ::gpio::gpio_porta_ext::PA5R;
#[doc = "Possible values of the field `PC0`"]
pub type PC0R = ::gpio::gpio_porta_ext::PA5R;
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
