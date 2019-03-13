#![doc = "Peripheral access API for RTL8710 microcontrollers (generated using svd2rust v0.14.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.14.0/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
extern "C" {}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 0] = [];
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {}
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[doc = "General Purpose I/O"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioa::RegisterBlock {
        1073745920 as *const _
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &gpioa::RegisterBlock {
        unsafe { &*GPIOA::ptr() }
    }
}
#[doc = "General Purpose I/O"]
pub mod gpioa;
#[doc = "GPIOB"]
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioa::RegisterBlock {
        1073745932 as *const _
    }
}
impl Deref for GPIOB {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &gpioa::RegisterBlock {
        unsafe { &*GPIOB::ptr() }
    }
}
#[doc = "GPIOC"]
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioa::RegisterBlock {
        1073745944 as *const _
    }
}
impl Deref for GPIOC {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &gpioa::RegisterBlock {
        unsafe { &*GPIOC::ptr() }
    }
}
#[doc = "GPIOA Interrupt Control"]
pub struct GPIOA_INT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA_INT {}
impl GPIOA_INT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioa_int::RegisterBlock {
        1073745968 as *const _
    }
}
impl Deref for GPIOA_INT {
    type Target = gpioa_int::RegisterBlock;
    fn deref(&self) -> &gpioa_int::RegisterBlock {
        unsafe { &*GPIOA_INT::ptr() }
    }
}
#[doc = "GPIOA Interrupt Control"]
pub mod gpioa_int;
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "GPIOB"]
    pub GPIOB: GPIOB,
    #[doc = "GPIOC"]
    pub GPIOC: GPIOC,
    #[doc = "GPIOA_INT"]
    pub GPIOA_INT: GPIOA_INT,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            GPIOA: GPIOA {
                _marker: PhantomData,
            },
            GPIOB: GPIOB {
                _marker: PhantomData,
            },
            GPIOC: GPIOC {
                _marker: PhantomData,
            },
            GPIOA_INT: GPIOA_INT {
                _marker: PhantomData,
            },
        }
    }
}
