#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 12usize],
    #[doc = "0x0c - Data Register"]
    pub dr: DR,
    #[doc = "0x10 - Data Direction Register"]
    pub ddr: DDR,
    #[doc = "0x14 - Control"]
    pub ctrl: CTRL,
    _reserved1: [u8; 60usize],
    #[doc = "0x54 - GPIO IN read or OUT read back"]
    pub ext: EXT,
}
#[doc = "Data Register"]
pub struct DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Register"]
pub mod dr;
#[doc = "Data Direction Register"]
pub struct DDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Direction Register"]
pub mod ddr;
#[doc = "Control"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control"]
pub mod ctrl;
#[doc = "GPIO IN read or OUT read back"]
pub struct EXT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO IN read or OUT read back"]
pub mod ext;
