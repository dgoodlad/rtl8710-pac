#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Divisor Latch Low"]
    pub dll: DLL,
    #[doc = "0x04 - Interrupt Enable / Divisor Latch High"]
    pub ier_dlh: IER_DLH,
    #[doc = "0x08 - Interrupt Identification / FIFO Control"]
    pub iir_fcr: IIR_FCR,
    #[doc = "0x0c - Line Control Register"]
    pub lcr: LCR,
    #[doc = "0x10 - Modem Control Register"]
    pub mcr: MCR,
    #[doc = "0x14 - Line Status Register"]
    pub lsr: LSR,
    #[doc = "0x18 - Modem Status Register"]
    pub msr: MSR,
    #[doc = "0x1c - Scratch Pad"]
    pub scr: SCR,
    #[doc = "0x20 - STS"]
    pub sts: STS,
    #[doc = "0x24 - Receive Buffer / Transmission Hold"]
    pub recv_buf_tran_hold: RECV_BUF_TRAN_HOLD,
    #[doc = "0x28 - Misc Control"]
    pub misc_ctl: MISC_CTL,
    _reserved0: [u8; 16usize],
    #[doc = "0x3c - UART Debug"]
    pub debug: DEBUG,
}
#[doc = "Divisor Latch Low"]
pub struct DLL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Divisor Latch Low"]
pub mod dll;
#[doc = "Interrupt Enable / Divisor Latch High"]
pub struct IER_DLH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable / Divisor Latch High"]
pub mod ier_dlh;
#[doc = "Interrupt Identification / FIFO Control"]
pub struct IIR_FCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Identification / FIFO Control"]
pub mod iir_fcr;
#[doc = "Line Control Register"]
pub struct LCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Line Control Register"]
pub mod lcr;
#[doc = "Modem Control Register"]
pub struct MCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Modem Control Register"]
pub mod mcr;
#[doc = "Line Status Register"]
pub struct LSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Line Status Register"]
pub mod lsr;
#[doc = "Modem Status Register"]
pub struct MSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Modem Status Register"]
pub mod msr;
#[doc = "Scratch Pad"]
pub struct SCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scratch Pad"]
pub mod scr;
#[doc = "STS"]
pub struct STS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "STS"]
pub mod sts;
#[doc = "Receive Buffer / Transmission Hold"]
pub struct RECV_BUF_TRAN_HOLD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Buffer / Transmission Hold"]
pub mod recv_buf_tran_hold;
#[doc = "Misc Control"]
pub struct MISC_CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Misc Control"]
pub mod misc_ctl;
#[doc = "UART Debug"]
pub struct DEBUG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART Debug"]
pub mod debug;
