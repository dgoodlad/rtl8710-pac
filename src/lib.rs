#![doc = "Peripheral access API for RTL8710AF microcontrollers (generated using svd2rust v0.14.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.14.0/svd2rust/#peripheral-api"]
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
extern "C" {
    fn TIMER0();
    fn TIMER1();
    fn I2C3();
    fn TIMER2_7();
    fn SPI0();
    fn GPIO();
    fn UART0();
    fn SPI_FLASH();
    fn SDIO_HOST();
    fn SDIO_DEVICE();
    fn I2S0_PCM0();
    fn I2S1_PCM1();
    fn WL_DMA();
    fn WL_PROTOCOL();
    fn CRYPTO();
    fn GDMA0_CHANNEL0();
    fn GDMA0_CHANNEL1();
    fn GDMA0_CHANNEL2();
    fn GDMA0_CHANNEL3();
    fn GDMA0_CHANNEL4();
    fn GDMA0_CHANNEL5();
    fn GDMA1_CHANNEL0();
    fn GDMA1_CHANNEL1();
    fn GDMA1_CHANNEL2();
    fn GDMA1_CHANNEL3();
    fn GDMA1_CHANNEL4();
    fn GDMA1_CHANNEL5();
    fn I2C1();
    fn I2C2();
    fn SPI1();
    fn UART1();
    fn LOG_UART();
    fn ADC();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 90] = [
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: TIMER0 },
    Vector { _handler: TIMER1 },
    Vector { _handler: I2C3 },
    Vector { _handler: TIMER2_7 },
    Vector { _handler: SPI0 },
    Vector { _handler: GPIO },
    Vector { _handler: UART0 },
    Vector {
        _handler: SPI_FLASH,
    },
    Vector { _reserved: 0 },
    Vector {
        _handler: SDIO_HOST,
    },
    Vector {
        _handler: SDIO_DEVICE,
    },
    Vector {
        _handler: I2S0_PCM0,
    },
    Vector {
        _handler: I2S1_PCM1,
    },
    Vector { _handler: WL_DMA },
    Vector {
        _handler: WL_PROTOCOL,
    },
    Vector { _handler: CRYPTO },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: GDMA0_CHANNEL0,
    },
    Vector {
        _handler: GDMA0_CHANNEL1,
    },
    Vector {
        _handler: GDMA0_CHANNEL2,
    },
    Vector {
        _handler: GDMA0_CHANNEL3,
    },
    Vector {
        _handler: GDMA0_CHANNEL4,
    },
    Vector {
        _handler: GDMA0_CHANNEL5,
    },
    Vector {
        _handler: GDMA1_CHANNEL0,
    },
    Vector {
        _handler: GDMA1_CHANNEL1,
    },
    Vector {
        _handler: GDMA1_CHANNEL2,
    },
    Vector {
        _handler: GDMA1_CHANNEL3,
    },
    Vector {
        _handler: GDMA1_CHANNEL4,
    },
    Vector {
        _handler: GDMA1_CHANNEL5,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: I2C1 },
    Vector { _handler: I2C2 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: SPI1 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: UART1 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: LOG_UART },
    Vector { _handler: ADC },
];
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "2 - TIMER0"]
    TIMER0,
    #[doc = "3 - TIMER1"]
    TIMER1,
    #[doc = "4 - I2C3"]
    I2C3,
    #[doc = "5 - TIMER2_7"]
    TIMER2_7,
    #[doc = "6 - SPI0"]
    SPI0,
    #[doc = "7 - GPIO"]
    GPIO,
    #[doc = "8 - UART0"]
    UART0,
    #[doc = "9 - SPI_FLASH"]
    SPI_FLASH,
    #[doc = "11 - SDIO_HOST"]
    SDIO_HOST,
    #[doc = "12 - SDIO_DEVICE"]
    SDIO_DEVICE,
    #[doc = "13 - I2S0_PCM0"]
    I2S0_PCM0,
    #[doc = "14 - I2S1_PCM1"]
    I2S1_PCM1,
    #[doc = "15 - WL_DMA"]
    WL_DMA,
    #[doc = "16 - WL_PROTOCOL"]
    WL_PROTOCOL,
    #[doc = "17 - CRYPTO"]
    CRYPTO,
    #[doc = "20 - GDMA0_CHANNEL0"]
    GDMA0_CHANNEL0,
    #[doc = "21 - GDMA0_CHANNEL1"]
    GDMA0_CHANNEL1,
    #[doc = "22 - GDMA0_CHANNEL2"]
    GDMA0_CHANNEL2,
    #[doc = "23 - GDMA0_CHANNEL3"]
    GDMA0_CHANNEL3,
    #[doc = "24 - GDMA0_CHANNEL4"]
    GDMA0_CHANNEL4,
    #[doc = "25 - GDMA0_CHANNEL5"]
    GDMA0_CHANNEL5,
    #[doc = "26 - GDMA1_CHANNEL0"]
    GDMA1_CHANNEL0,
    #[doc = "27 - GDMA1_CHANNEL1"]
    GDMA1_CHANNEL1,
    #[doc = "28 - GDMA1_CHANNEL2"]
    GDMA1_CHANNEL2,
    #[doc = "29 - GDMA1_CHANNEL3"]
    GDMA1_CHANNEL3,
    #[doc = "30 - GDMA1_CHANNEL4"]
    GDMA1_CHANNEL4,
    #[doc = "31 - GDMA1_CHANNEL5"]
    GDMA1_CHANNEL5,
    #[doc = "65 - I2C1"]
    I2C1,
    #[doc = "66 - I2C2"]
    I2C2,
    #[doc = "72 - SPI1"]
    SPI1,
    #[doc = "80 - UART1"]
    UART1,
    #[doc = "88 - LOG_UART"]
    LOG_UART,
    #[doc = "89 - ADC"]
    ADC,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::TIMER0 => 2,
            Interrupt::TIMER1 => 3,
            Interrupt::I2C3 => 4,
            Interrupt::TIMER2_7 => 5,
            Interrupt::SPI0 => 6,
            Interrupt::GPIO => 7,
            Interrupt::UART0 => 8,
            Interrupt::SPI_FLASH => 9,
            Interrupt::SDIO_HOST => 11,
            Interrupt::SDIO_DEVICE => 12,
            Interrupt::I2S0_PCM0 => 13,
            Interrupt::I2S1_PCM1 => 14,
            Interrupt::WL_DMA => 15,
            Interrupt::WL_PROTOCOL => 16,
            Interrupt::CRYPTO => 17,
            Interrupt::GDMA0_CHANNEL0 => 20,
            Interrupt::GDMA0_CHANNEL1 => 21,
            Interrupt::GDMA0_CHANNEL2 => 22,
            Interrupt::GDMA0_CHANNEL3 => 23,
            Interrupt::GDMA0_CHANNEL4 => 24,
            Interrupt::GDMA0_CHANNEL5 => 25,
            Interrupt::GDMA1_CHANNEL0 => 26,
            Interrupt::GDMA1_CHANNEL1 => 27,
            Interrupt::GDMA1_CHANNEL2 => 28,
            Interrupt::GDMA1_CHANNEL3 => 29,
            Interrupt::GDMA1_CHANNEL4 => 30,
            Interrupt::GDMA1_CHANNEL5 => 31,
            Interrupt::I2C1 => 65,
            Interrupt::I2C2 => 66,
            Interrupt::SPI1 => 72,
            Interrupt::UART1 => 80,
            Interrupt::LOG_UART => 88,
            Interrupt::ADC => 89,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[doc = "System Control Registers"]
pub struct SYSCON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCON {}
impl SYSCON {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const syscon::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for SYSCON {
    type Target = syscon::RegisterBlock;
    fn deref(&self) -> &syscon::RegisterBlock {
        unsafe { &*SYSCON::ptr() }
    }
}
#[doc = "System Control Registers"]
pub mod syscon;
#[doc = "General Purpose I/O"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio::RegisterBlock {
        1073745920 as *const _
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    fn deref(&self) -> &gpio::RegisterBlock {
        unsafe { &*GPIO::ptr() }
    }
}
#[doc = "General Purpose I/O"]
pub mod gpio;
#[doc = "Timer Control"]
pub struct TIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER {}
impl TIMER {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer::RegisterBlock {
        1073750016 as *const _
    }
}
impl Deref for TIMER {
    type Target = timer::RegisterBlock;
    fn deref(&self) -> &timer::RegisterBlock {
        unsafe { &*TIMER::ptr() }
    }
}
#[doc = "Timer Control"]
pub mod timer;
#[doc = "UART for Log"]
pub struct LOG_UART {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LOG_UART {}
impl LOG_UART {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const log_uart::RegisterBlock {
        1073754112 as *const _
    }
}
impl Deref for LOG_UART {
    type Target = log_uart::RegisterBlock;
    fn deref(&self) -> &log_uart::RegisterBlock {
        unsafe { &*LOG_UART::ptr() }
    }
}
#[doc = "UART for Log"]
pub mod log_uart;
#[doc = "UART 0"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart0::RegisterBlock {
        1074003968 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &uart0::RegisterBlock {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "UART 0"]
pub mod uart0;
#[doc = "UART 1"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart1::RegisterBlock {
        1074006016 as *const _
    }
}
impl Deref for UART1 {
    type Target = uart1::RegisterBlock;
    fn deref(&self) -> &uart1::RegisterBlock {
        unsafe { &*UART1::ptr() }
    }
}
#[doc = "UART 1"]
pub mod uart1;
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "SYSCON"]
    pub SYSCON: SYSCON,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "TIMER"]
    pub TIMER: TIMER,
    #[doc = "LOG_UART"]
    pub LOG_UART: LOG_UART,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UART1"]
    pub UART1: UART1,
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
            SYSCON: SYSCON {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            TIMER: TIMER {
                _marker: PhantomData,
            },
            LOG_UART: LOG_UART {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
        }
    }
}
