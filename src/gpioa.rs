#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data Register"]
    pub dr: DR,
    #[doc = "0x04 - Data Direction Register"]
    pub ddr: DDR,
    #[doc = "0x08 - Control"]
    pub ctrl: CTRL,
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
