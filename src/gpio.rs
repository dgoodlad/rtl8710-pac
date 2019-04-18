#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIOA Data Register"]
    pub gpioa_dr: GPIOA_DR,
    #[doc = "0x04 - GPIOA Data Direction Register"]
    pub gpioa_ddr: GPIOA_DDR,
    #[doc = "0x08 - GPIOA Control Register"]
    pub gpioa_ctrl: GPIOA_CTRL,
    #[doc = "0x0c - GPIOA Data Register"]
    pub gpiob_dr: GPIOB_DR,
    #[doc = "0x10 - GPIOB Data Direction Register"]
    pub gpiob_ddr: GPIOB_DDR,
    #[doc = "0x14 - GPIOB Control Register"]
    pub gpiob_ctrl: GPIOB_CTRL,
    _reserved0: [u8; 24usize],
    #[doc = "0x30 - Interrupt Enable"]
    pub int_en: INT_EN,
    #[doc = "0x34 - Interrupt Mask"]
    pub int_mask: INT_MASK,
    #[doc = "0x38 - Interrupt Type (level/edge)"]
    pub int_type: INT_TYPE,
    #[doc = "0x3c - Interrupt Polarity (low/high)"]
    pub int_polarity: INT_POLARITY,
    #[doc = "0x40 - Interrupt Status"]
    pub int_status: INT_STATUS,
    #[doc = "0x44 - Interrupt Status Without Mask"]
    pub int_raw_status: INT_RAW_STATUS,
    #[doc = "0x48 - Interrupt Debounce"]
    pub int_debounce: INT_DEBOUNCE,
    #[doc = "0x4c - Interrupt Clear"]
    pub int_clear: INT_CLEAR,
    #[doc = "0x50 - GPIO IN read or OUT read back"]
    pub ext: EXT,
    _reserved1: [u8; 12usize],
    #[doc = "0x60 - Is level-sensitive interrupt being sync with PCLK"]
    pub int_sync: INT_SYNC,
}
#[doc = "GPIOA Data Register"]
pub struct GPIOA_DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOA Data Register"]
pub mod gpioa_dr;
#[doc = "GPIOA Data Direction Register"]
pub struct GPIOA_DDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOA Data Direction Register"]
pub mod gpioa_ddr;
#[doc = "GPIOB Data Direction Register"]
pub struct GPIOB_DDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOB Data Direction Register"]
pub mod gpiob_ddr;
#[doc = "GPIOA Control Register"]
pub struct GPIOA_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOA Control Register"]
pub mod gpioa_ctrl;
#[doc = "GPIOB Control Register"]
pub struct GPIOB_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOB Control Register"]
pub mod gpiob_ctrl;
#[doc = "GPIOA Data Register"]
pub struct GPIOB_DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIOA Data Register"]
pub mod gpiob_dr;
#[doc = "Interrupt Enable"]
pub struct INT_EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable"]
pub mod int_en;
#[doc = "Interrupt Mask"]
pub struct INT_MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask"]
pub mod int_mask;
#[doc = "Interrupt Type (level/edge)"]
pub struct INT_TYPE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Type (level/edge)"]
pub mod int_type;
#[doc = "Interrupt Polarity (low/high)"]
pub struct INT_POLARITY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Polarity (low/high)"]
pub mod int_polarity;
#[doc = "Interrupt Status"]
pub struct INT_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status"]
pub mod int_status;
#[doc = "Interrupt Status Without Mask"]
pub struct INT_RAW_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Without Mask"]
pub mod int_raw_status;
#[doc = "Interrupt Debounce"]
pub struct INT_DEBOUNCE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Debounce"]
pub mod int_debounce;
#[doc = "Interrupt Clear"]
pub struct INT_CLEAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Clear"]
pub mod int_clear;
#[doc = "GPIO IN read or OUT read back"]
pub struct EXT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO IN read or OUT read back"]
pub mod ext;
#[doc = "Is level-sensitive interrupt being sync with PCLK"]
pub struct INT_SYNC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Is level-sensitive interrupt being sync with PCLK"]
pub mod int_sync;
