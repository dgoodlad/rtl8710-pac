#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO Port A Data Register"]
    pub gpio_porta_dr: GPIO_PORTA_DR,
    #[doc = "0x04 - GPIO Port A Data Direction Register"]
    pub gpio_porta_ddr: GPIO_PORTA_DDR,
    #[doc = "0x08 - GPIO Port A Data Source Control"]
    pub gpio_porta_ctrl: GPIO_PORTA_CTRL,
    #[doc = "0x0c - GPIO Port B Data Register"]
    pub gpio_portb_dr: GPIO_PORTB_DR,
    #[doc = "0x10 - GPIO Port B Data Direction Register"]
    pub gpio_portb_ddr: GPIO_PORTB_DDR,
    #[doc = "0x14 - GPIO Port B Data Source Control"]
    pub gpio_portb_ctrl: GPIO_PORTB_CTRL,
    #[doc = "0x18 - GPIO Port C Data Register"]
    pub gpio_portc_dr: GPIO_PORTC_DR,
    #[doc = "0x1c - GPIO Port C Data Direction Register"]
    pub gpio_portc_ddr: GPIO_PORTC_DDR,
    #[doc = "0x20 - GPIO Port C Data Source Control"]
    pub gpio_portc_ctrl: GPIO_PORTC_CTRL,
    _reserved0: [u8; 12usize],
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
    #[doc = "0x50 - GPIO Port A External Data"]
    pub gpio_porta_ext: GPIO_PORTA_EXT,
    #[doc = "0x54 - GPIO Port B External Data"]
    pub gpio_portb_ext: GPIO_PORTB_EXT,
    #[doc = "0x58 - GPIO Port C External Data"]
    pub gpio_portc_ext: GPIO_PORTC_EXT,
    _reserved1: [u8; 4usize],
    #[doc = "0x60 - Is level-sensitive interrupt being sync with PCLK"]
    pub int_sync: INT_SYNC,
}
#[doc = "GPIO Port A Data Register"]
pub struct GPIO_PORTA_DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Port A Data Register"]
pub mod gpio_porta_dr;
#[doc = "GPIO Port A Data Direction Register"]
pub struct GPIO_PORTA_DDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Port A Data Direction Register"]
pub mod gpio_porta_ddr;
#[doc = "GPIO Port A Data Source Control"]
pub struct GPIO_PORTA_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Port A Data Source Control"]
pub mod gpio_porta_ctrl;
#[doc = "GPIO Port B Data Register"]
pub struct GPIO_PORTB_DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Port B Data Register"]
pub mod gpio_portb_dr;
#[doc = "GPIO Port B Data Direction Register"]
pub struct GPIO_PORTB_DDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Port B Data Direction Register"]
pub mod gpio_portb_ddr;
#[doc = "GPIO Port B Data Source Control"]
pub struct GPIO_PORTB_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Port B Data Source Control"]
pub mod gpio_portb_ctrl;
#[doc = "GPIO Port C Data Register"]
pub struct GPIO_PORTC_DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Port C Data Register"]
pub mod gpio_portc_dr;
#[doc = "GPIO Port C Data Direction Register"]
pub struct GPIO_PORTC_DDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Port C Data Direction Register"]
pub mod gpio_portc_ddr;
#[doc = "GPIO Port C Data Source Control"]
pub struct GPIO_PORTC_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Port C Data Source Control"]
pub mod gpio_portc_ctrl;
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
#[doc = "GPIO Port A External Data"]
pub struct GPIO_PORTA_EXT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Port A External Data"]
pub mod gpio_porta_ext;
#[doc = "GPIO Port B External Data"]
pub struct GPIO_PORTB_EXT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Port B External Data"]
pub mod gpio_portb_ext;
#[doc = "GPIO Port C External Data"]
pub struct GPIO_PORTC_EXT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Port C External Data"]
pub mod gpio_portc_ext;
#[doc = "Is level-sensitive interrupt being sync with PCLK"]
pub struct INT_SYNC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Is level-sensitive interrupt being sync with PCLK"]
pub mod int_sync;
