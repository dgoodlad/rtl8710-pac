#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer Load Count"]
    pub timer_load_count: TIMER_LOAD_COUNT,
    #[doc = "0x04 - Current Value"]
    pub timer_current_val: TIMER_CURRENT_VAL,
    #[doc = "0x08 - Timer Control"]
    pub timer_ctl_reg: TIMER_CTL_REG,
    #[doc = "0x0c - Reading from this register returns b0, and clears the interrupt from PWM/Timer"]
    pub timer_eoi: TIMER_EOI,
    #[doc = "0x10 - Post-masking interrupt status of PWM/Timer"]
    pub timer_int_status: TIMER_INT_STATUS,
    #[doc = "0x14 - Timer Interval"]
    pub timer_interval: TIMER_INTERVAL,
    _reserved0: [u8; 136usize],
    #[doc = "0xa0 - Timers Interrupt Status"]
    pub timers_int_status: TIMERS_INT_STATUS,
    #[doc = "0xa4 - Timers End of Interrupt"]
    pub timers_eoi: TIMERS_EOI,
    #[doc = "0xa8 - Timers raw interrupt status"]
    pub timers_raw_int_status: TIMERS_RAW_INT_STATUS,
    #[doc = "0xac - ASCII string representing the version number of the timers logic"]
    pub timers_comp_ver: TIMERS_COMP_VER,
}
#[doc = "Timer Load Count"]
pub struct TIMER_LOAD_COUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Load Count"]
pub mod timer_load_count;
#[doc = "Current Value"]
pub struct TIMER_CURRENT_VAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Value"]
pub mod timer_current_val;
#[doc = "Timer Control"]
pub struct TIMER_CTL_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Control"]
pub mod timer_ctl_reg;
#[doc = "Reading from this register returns b0, and clears the interrupt from PWM/Timer"]
pub struct TIMER_EOI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reading from this register returns b0, and clears the interrupt from PWM/Timer"]
pub mod timer_eoi;
#[doc = "Post-masking interrupt status of PWM/Timer"]
pub struct TIMER_INT_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post-masking interrupt status of PWM/Timer"]
pub mod timer_int_status;
#[doc = "Timer Interval"]
pub struct TIMER_INTERVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Interval"]
pub mod timer_interval;
#[doc = "Timers Interrupt Status"]
pub struct TIMERS_INT_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timers Interrupt Status"]
pub mod timers_int_status;
#[doc = "Timers End of Interrupt"]
pub struct TIMERS_EOI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timers End of Interrupt"]
pub mod timers_eoi;
#[doc = "Timers raw interrupt status"]
pub struct TIMERS_RAW_INT_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timers raw interrupt status"]
pub mod timers_raw_int_status;
#[doc = "ASCII string representing the version number of the timers logic"]
pub struct TIMERS_COMP_VER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ASCII string representing the version number of the timers logic"]
pub mod timers_comp_ver;
