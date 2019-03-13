#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Enable"]
    pub en: EN,
    #[doc = "0x04 - Interrupt Mask"]
    pub mask: MASK,
    #[doc = "0x08 - Interrupt Type (level/edge)"]
    pub type_: TYPE,
    #[doc = "0x0c - Interrupt Polarity (low/high)"]
    pub polarity: POLARITY,
    #[doc = "0x10 - Interrupt Status"]
    pub status: STATUS,
    #[doc = "0x14 - Interrupt Status Without Mask"]
    pub raw_status: RAW_STATUS,
    #[doc = "0x18 - Interrupt Debounce"]
    pub debounce: DEBOUNCE,
    _reserved0: [u8; 16usize],
    #[doc = "0x2c - Interrupt Clear"]
    pub clear: CLEAR,
}
#[doc = "Interrupt Enable"]
pub struct EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable"]
pub mod en;
#[doc = "Interrupt Mask"]
pub struct MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask"]
pub mod mask;
#[doc = "Interrupt Type (level/edge)"]
pub struct TYPE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Type (level/edge)"]
pub mod type_;
#[doc = "Interrupt Polarity (low/high)"]
pub struct POLARITY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Polarity (low/high)"]
pub mod polarity;
#[doc = "Interrupt Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status"]
pub mod status;
#[doc = "Interrupt Status Without Mask"]
pub struct RAW_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Without Mask"]
pub mod raw_status;
#[doc = "Interrupt Debounce"]
pub struct DEBOUNCE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Debounce"]
pub mod debounce;
#[doc = "Interrupt Clear"]
pub struct CLEAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Clear"]
pub mod clear;
