#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWR_CTRL"]
    pub pwr_ctrl: PWR_CTRL,
    #[doc = "0x02 - ISO_CTRL"]
    pub iso_ctrl: ISO_CTRL,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - FUNC_EN"]
    pub func_en: FUNC_EN,
    _reserved1: [u8; 4usize],
    #[doc = "0x10 - CLK_CTRL0"]
    pub clk_ctrl0: CLK_CTRL0,
    #[doc = "0x14 - CLK_CTRL1"]
    pub clk_ctrl1: CLK_CTRL1,
    _reserved2: [u8; 8usize],
    #[doc = "0x20 - EFUSE_SYSCFG0"]
    pub efuse_syscfg0: EFUSE_SYSCFG0,
    #[doc = "0x24 - EFUSE_SYSCFG1"]
    pub efuse_syscfg1: EFUSE_SYSCFG1,
    #[doc = "0x28 - EFUSE_SYSCFG2"]
    pub efuse_syscfg2: EFUSE_SYSCFG2,
    #[doc = "0x2c - EFUSE_SYSCFG3"]
    pub efuse_syscfg3: EFUSE_SYSCFG3,
    #[doc = "0x30 - EFUSE_SYSCFG4"]
    pub efuse_syscfg4: EFUSE_SYSCFG4,
    #[doc = "0x34 - EFUSE_SYSCFG5"]
    pub efuse_syscfg5: EFUSE_SYSCFG5,
    #[doc = "0x38 - EFUSE_SYSCFG6"]
    pub efuse_syscfg6: EFUSE_SYSCFG6,
    #[doc = "0x3c - EFUSE_SYSCFG7"]
    pub efuse_syscfg7: EFUSE_SYSCFG7,
    #[doc = "0x40 - REGU_CTRL0"]
    pub regu_ctrl0: REGU_CTRL0,
    _reserved3: [u8; 4usize],
    #[doc = "0x48 - SWR_CTRL0"]
    pub swr_ctrl0: SWR_CTRL0,
    #[doc = "0x4c - SWR_CTRL1"]
    pub swr_ctrl1: SWR_CTRL1,
    _reserved4: [u8; 16usize],
    #[doc = "0x60 - XTAL_CTRL0"]
    pub xtal_ctrl0: XTAL_CTRL0,
    #[doc = "0x64 - XTAL_CTRL1"]
    pub xtal_ctrl1: XTAL_CTRL1,
    _reserved5: [u8; 8usize],
    #[doc = "0x70 - SYSPLL_CTRL0"]
    pub syspll_ctrl0: SYSPLL_CTRL0,
    #[doc = "0x74 - SYSPLL_CTRL1"]
    pub syspll_ctrl1: SYSPLL_CTRL1,
    #[doc = "0x78 - SYSPLL_CTRL2"]
    pub syspll_ctrl2: SYSPLL_CTRL2,
    _reserved6: [u8; 20usize],
    #[doc = "0x90 - ANA_TIM_CTRL"]
    pub ana_tim_ctrl: ANA_TIM_CTRL,
    #[doc = "0x94 - DSLP_TIM_CTRL"]
    pub dslp_tim_ctrl: DSLP_TIM_CTRL,
    #[doc = "0x98 - DSLP_TIM_CAL_CTRL"]
    pub dslp_tim_cal_ctrl: DSLP_TIM_CAL_CTRL,
    _reserved7: [u8; 4usize],
    #[doc = "0xa0 - DEBUG_CTRL"]
    pub debug_ctrl: DEBUG_CTRL,
    #[doc = "0xa4 - PINMUX_CTRL"]
    pub pinmux_ctrl: PINMUX_CTRL,
    #[doc = "0xa8 - GPIO_DSTBY_WAKE_CTRL0"]
    pub gpio_dstby_wake_ctrl0: GPIO_DSTBY_WAKE_CTRL0,
    #[doc = "0xac - GPIO_DSTBY_WAKE_CTRL1"]
    pub gpio_dstby_wake_ctrl1: GPIO_DSTBY_WAKE_CTRL1,
    _reserved8: [u8; 12usize],
    #[doc = "0xbc - DEBUG_REG"]
    pub debug_reg: DEBUG_REG,
    _reserved9: [u8; 32usize],
    #[doc = "0xe0 - EEPROM_CTRL0"]
    pub eeprom_ctrl0: EEPROM_CTRL0,
    #[doc = "0xe4 - EEPROM_CTRL1"]
    pub eeprom_ctrl1: EEPROM_CTRL1,
    #[doc = "0xe8 - EFUSE_CTRL"]
    pub efuse_ctrl: EFUSE_CTRL,
    #[doc = "0xec - EFUSE_TEST"]
    pub efuse_test: EFUSE_TEST,
    #[doc = "0xf0 - DSTBY_INFO0"]
    pub dstby_info0: DSTBY_INFO0,
    #[doc = "0xf4 - DSTBY_INFO1"]
    pub dstby_info1: DSTBY_INFO1,
    #[doc = "0xf8 - DSTBY_INFO2"]
    pub dstby_info2: DSTBY_INFO2,
    #[doc = "0xfc - DSTBY_INFO3"]
    pub dstby_info3: DSTBY_INFO3,
    #[doc = "0x100 - SLP_WAKE_EVENT_MSK0"]
    pub slp_wake_event_msk0: SLP_WAKE_EVENT_MSK0,
    #[doc = "0x104 - SLP_WAKE_EVENT_MSK1"]
    pub slp_wake_event_msk1: SLP_WAKE_EVENT_MSK1,
    #[doc = "0x108 - SLP_WAKE_EVENT_STATUS0"]
    pub slp_wake_event_status0: SLP_WAKE_EVENT_STATUS0,
    #[doc = "0x10c - SLP_WAKE_EVENT_STATUS1"]
    pub slp_wake_event_status1: SLP_WAKE_EVENT_STATUS1,
    #[doc = "0x110 - SNF_WAKE_EVENT_MSK0"]
    pub snf_wake_event_msk0: SNF_WAKE_EVENT_MSK0,
    #[doc = "0x114 - SNF_WAKE_EVENT_STATUS"]
    pub snf_wake_event_status: SNF_WAKE_EVENT_STATUS,
    #[doc = "0x118 - PWRMGT_CTRL"]
    pub pwrmgt_ctrl: PWRMGT_CTRL,
    _reserved10: [u8; 4usize],
    #[doc = "0x120 - PWRMGT_OPTION"]
    pub pwrmgt_option: PWRMGT_OPTION,
    #[doc = "0x124 - PWRMGT_OPTION_EXT"]
    pub pwrmgt_option_ext: PWRMGT_OPTION_EXT,
    _reserved11: [u8; 8usize],
    #[doc = "0x130 - DSLP_WEVENT"]
    pub dslp_wevent: DSLP_WEVENT,
    #[doc = "0x134 - PERI_MONITOR"]
    pub peri_monitor: PERI_MONITOR,
    _reserved12: [u8; 184usize],
    #[doc = "0x1f0 - SYSTEM_CFG0"]
    pub system_cfg0: SYSTEM_CFG0,
    #[doc = "0x1f4 - SYSTEM_CFG1"]
    pub system_cfg1: SYSTEM_CFG1,
    #[doc = "0x1f8 - SYSTEM_CFG2"]
    pub system_cfg2: SYSTEM_CFG2,
    _reserved13: [u8; 4usize],
    #[doc = "0x200 - PEON_PWR_CTRL"]
    pub peon_pwr_ctrl: PEON_PWR_CTRL,
    #[doc = "0x204 - PON_ISO_CTRL"]
    pub pon_iso_ctrl: PON_ISO_CTRL,
    _reserved14: [u8; 8usize],
    #[doc = "0x210 - SOC_FUNC_EN"]
    pub soc_func_en: SOC_FUNC_EN,
    #[doc = "0x214 - SOC_HCI_COM_FUNC_EN"]
    pub soc_hci_com_func_en: SOC_HCI_COM_FUNC_EN,
    #[doc = "0x218 - SOC_PERI_FUNC0_EN"]
    pub soc_peri_func0_en: SOC_PERI_FUNC0_EN,
    #[doc = "0x21c - SOC_PERI_FUNC1_EN"]
    pub soc_peri_func1_en: SOC_PERI_FUNC1_EN,
    #[doc = "0x220 - SOC_PERI_BD_FUNC0_EN"]
    pub soc_peri_bd_func0_en: SOC_PERI_BD_FUNC0_EN,
    _reserved15: [u8; 12usize],
    #[doc = "0x230 - PESOC_CLK_CTRL"]
    pub pesoc_clk_ctrl: PESOC_CLK_CTRL,
    #[doc = "0x234 - PESOC_PERI_CLK_CTRL0"]
    pub pesoc_peri_clk_ctrl0: PESOC_PERI_CLK_CTRL0,
    #[doc = "0x238 - PESOC_PERI_CLK_CTRL1"]
    pub pesoc_peri_clk_ctrl1: PESOC_PERI_CLK_CTRL1,
    #[doc = "0x23c - PESOC_CLK_CTRL3"]
    pub pesoc_clk_ctrl3: PESOC_CLK_CTRL3,
    #[doc = "0x240 - PESOC_HCI_CLK_CTRL0"]
    pub pesoc_hci_clk_ctrl0: PESOC_HCI_CLK_CTRL0,
    #[doc = "0x244 - PESOC_COM_CLK_CTRL1"]
    pub pesoc_com_clk_ctrl1: PESOC_COM_CLK_CTRL1,
    #[doc = "0x248 - PESOC_HW_ENG_CLK_CTRL"]
    pub pesoc_hw_eng_clk_ctrl: PESOC_HW_ENG_CLK_CTRL,
    _reserved16: [u8; 4usize],
    #[doc = "0x250 - PESOC_CLK_SEL"]
    pub pesoc_clk_sel: PESOC_CLK_SEL,
    _reserved17: [u8; 24usize],
    #[doc = "0x26c - SYS_ANACK_CAL_CTRL"]
    pub sys_anack_cal_ctrl: SYS_ANACK_CAL_CTRL,
    #[doc = "0x270 - OSC32K_CTRL"]
    pub osc32k_ctrl: OSC32K_CTRL,
    #[doc = "0x274 - OSC32K_REG_CTRL0"]
    pub osc32k_reg_ctrl0: OSC32K_REG_CTRL0,
    #[doc = "0x278 - OSC32K_REG_CTRL1"]
    pub osc32k_reg_ctrl1: OSC32K_REG_CTRL1,
    #[doc = "0x27c - THERMAL_METER_CTRL"]
    pub thermal_meter_ctrl: THERMAL_METER_CTRL,
    #[doc = "0x280 - UART_MUX_CTRL"]
    pub uart_mux_ctrl: UART_MUX_CTRL,
    #[doc = "0x284 - SPI_MUX_CTRL"]
    pub spi_mux_ctrl: SPI_MUX_CTRL,
    #[doc = "0x288 - I2C_MUX_CTRL"]
    pub i2c_mux_ctrl: I2C_MUX_CTRL,
    #[doc = "0x28c - I2S_MUX_CTRL"]
    pub i2s_mux_ctrl: I2S_MUX_CTRL,
    _reserved18: [u8; 16usize],
    #[doc = "0x2a0 - HCI_PINMUX_CTRL"]
    pub hci_pinmux_ctrl: HCI_PINMUX_CTRL,
    #[doc = "0x2a4 - WL_PINMUX_CTRL"]
    pub wl_pinmux_ctrl: WL_PINMUX_CTRL,
    #[doc = "0x2a8 - BT_PINMUX_CTRL"]
    pub bt_pinmux_ctrl: BT_PINMUX_CTRL,
    #[doc = "0x2ac - PWM_PINMUX_CTRL"]
    pub pwm_pinmux_ctrl: PWM_PINMUX_CTRL,
    _reserved19: [u8; 16usize],
    #[doc = "0x2c0 - CPU_PERIPHERAL_CTRL"]
    pub cpu_peripheral_ctrl: CPU_PERIPHERAL_CTRL,
    _reserved20: [u8; 28usize],
    #[doc = "0x2e0 - HCI_CTRL_STATUS_0"]
    pub hci_ctrl_status_0: HCI_CTRL_STATUS_0,
    #[doc = "0x2e4 - HCI_CTRL_STATUS_1"]
    pub hci_ctrl_status_1: HCI_CTRL_STATUS_1,
    _reserved21: [u8; 24usize],
    #[doc = "0x300 - PESOC_MEM_CTRL"]
    pub pesoc_mem_ctrl: PESOC_MEM_CTRL,
    #[doc = "0x304 - PESOC_SOC_CTRL"]
    pub pesoc_soc_ctrl: PESOC_SOC_CTRL,
    #[doc = "0x308 - PESOC_PERI_CTRL"]
    pub pesoc_peri_ctrl: PESOC_PERI_CTRL,
    _reserved22: [u8; 20usize],
    #[doc = "0x320 - GPIO_SHTDN_CTRL"]
    pub gpio_shtdn_ctrl: GPIO_SHTDN_CTRL,
    #[doc = "0x324 - GPIO_DRIVING_CTRL"]
    pub gpio_driving_ctrl: GPIO_DRIVING_CTRL,
    _reserved23: [u8; 8usize],
    #[doc = "0x330 - GPIO_PULL_CTRL0"]
    pub gpio_pull_ctrl0: GPIO_PULL_CTRL0,
    #[doc = "0x334 - GPIO_PULL_CTRL1"]
    pub gpio_pull_ctrl1: GPIO_PULL_CTRL1,
    #[doc = "0x338 - GPIO_PULL_CTRL2"]
    pub gpio_pull_ctrl2: GPIO_PULL_CTRL2,
    #[doc = "0x33c - GPIO_PULL_CTRL3"]
    pub gpio_pull_ctrl3: GPIO_PULL_CTRL3,
    #[doc = "0x340 - GPIO_PULL_CTRL4"]
    pub gpio_pull_ctrl4: GPIO_PULL_CTRL4,
    #[doc = "0x344 - GPIO_PULL_CTRL5"]
    pub gpio_pull_ctrl5: GPIO_PULL_CTRL5,
    #[doc = "0x348 - GPIO_PULL_CTRL6"]
    pub gpio_pull_ctrl6: GPIO_PULL_CTRL6,
    _reserved24: [u8; 20usize],
    #[doc = "0x360 - PERI_PWM0_CTRL"]
    pub peri_pwm0_ctrl: PERI_PWM0_CTRL,
    #[doc = "0x364 - PERI_PWM1_CTRL"]
    pub peri_pwm1_ctrl: PERI_PWM1_CTRL,
    #[doc = "0x368 - PERI_PWM2_CTRL"]
    pub peri_pwm2_ctrl: PERI_PWM2_CTRL,
    #[doc = "0x36c - PERI_PWM3_CTRL"]
    pub peri_pwm3_ctrl: PERI_PWM3_CTRL,
    #[doc = "0x370 - PERI_TIM_EVT_CTRL"]
    pub peri_tim_evt_ctrl: PERI_TIM_EVT_CTRL,
    #[doc = "0x374 - PERI_EGTIM_CTRL"]
    pub peri_egtim_ctrl: PERI_EGTIM_CTRL,
    _reserved25: [u8; 120usize],
    #[doc = "0x3f0 - PEON_CFG"]
    pub peon_cfg: PEON_CFG,
    #[doc = "0x3f4 - PEON_STATUS"]
    pub peon_status: PEON_STATUS,
}
#[doc = "PWR_CTRL"]
pub struct PWR_CTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PWR_CTRL"]
pub mod pwr_ctrl;
#[doc = "ISO_CTRL"]
pub struct ISO_CTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "ISO_CTRL"]
pub mod iso_ctrl;
#[doc = "FUNC_EN"]
pub struct FUNC_EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FUNC_EN"]
pub mod func_en;
#[doc = "CLK_CTRL0"]
pub struct CLK_CTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_CTRL0"]
pub mod clk_ctrl0;
#[doc = "CLK_CTRL1"]
pub struct CLK_CTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK_CTRL1"]
pub mod clk_ctrl1;
#[doc = "EFUSE_SYSCFG0"]
pub struct EFUSE_SYSCFG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EFUSE_SYSCFG0"]
pub mod efuse_syscfg0;
#[doc = "EFUSE_SYSCFG1"]
pub struct EFUSE_SYSCFG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EFUSE_SYSCFG1"]
pub mod efuse_syscfg1;
#[doc = "EFUSE_SYSCFG2"]
pub struct EFUSE_SYSCFG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EFUSE_SYSCFG2"]
pub mod efuse_syscfg2;
#[doc = "EFUSE_SYSCFG3"]
pub struct EFUSE_SYSCFG3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EFUSE_SYSCFG3"]
pub mod efuse_syscfg3;
#[doc = "EFUSE_SYSCFG4"]
pub struct EFUSE_SYSCFG4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EFUSE_SYSCFG4"]
pub mod efuse_syscfg4;
#[doc = "EFUSE_SYSCFG5"]
pub struct EFUSE_SYSCFG5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EFUSE_SYSCFG5"]
pub mod efuse_syscfg5;
#[doc = "EFUSE_SYSCFG6"]
pub struct EFUSE_SYSCFG6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EFUSE_SYSCFG6"]
pub mod efuse_syscfg6;
#[doc = "EFUSE_SYSCFG7"]
pub struct EFUSE_SYSCFG7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EFUSE_SYSCFG7"]
pub mod efuse_syscfg7;
#[doc = "REGU_CTRL0"]
pub struct REGU_CTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "REGU_CTRL0"]
pub mod regu_ctrl0;
#[doc = "SWR_CTRL0"]
pub struct SWR_CTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SWR_CTRL0"]
pub mod swr_ctrl0;
#[doc = "SWR_CTRL1"]
pub struct SWR_CTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SWR_CTRL1"]
pub mod swr_ctrl1;
#[doc = "XTAL_CTRL0"]
pub struct XTAL_CTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "XTAL_CTRL0"]
pub mod xtal_ctrl0;
#[doc = "XTAL_CTRL1"]
pub struct XTAL_CTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "XTAL_CTRL1"]
pub mod xtal_ctrl1;
#[doc = "SYSPLL_CTRL0"]
pub struct SYSPLL_CTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SYSPLL_CTRL0"]
pub mod syspll_ctrl0;
#[doc = "SYSPLL_CTRL1"]
pub struct SYSPLL_CTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SYSPLL_CTRL1"]
pub mod syspll_ctrl1;
#[doc = "SYSPLL_CTRL2"]
pub struct SYSPLL_CTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SYSPLL_CTRL2"]
pub mod syspll_ctrl2;
#[doc = "ANA_TIM_CTRL"]
pub struct ANA_TIM_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ANA_TIM_CTRL"]
pub mod ana_tim_ctrl;
#[doc = "DSLP_TIM_CTRL"]
pub struct DSLP_TIM_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSLP_TIM_CTRL"]
pub mod dslp_tim_ctrl;
#[doc = "DSLP_TIM_CAL_CTRL"]
pub struct DSLP_TIM_CAL_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSLP_TIM_CAL_CTRL"]
pub mod dslp_tim_cal_ctrl;
#[doc = "DEBUG_CTRL"]
pub struct DEBUG_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DEBUG_CTRL"]
pub mod debug_ctrl;
#[doc = "PINMUX_CTRL"]
pub struct PINMUX_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PINMUX_CTRL"]
pub mod pinmux_ctrl;
#[doc = "GPIO_DSTBY_WAKE_CTRL0"]
pub struct GPIO_DSTBY_WAKE_CTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO_DSTBY_WAKE_CTRL0"]
pub mod gpio_dstby_wake_ctrl0;
#[doc = "GPIO_DSTBY_WAKE_CTRL1"]
pub struct GPIO_DSTBY_WAKE_CTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO_DSTBY_WAKE_CTRL1"]
pub mod gpio_dstby_wake_ctrl1;
#[doc = "DEBUG_REG"]
pub struct DEBUG_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DEBUG_REG"]
pub mod debug_reg;
#[doc = "EEPROM_CTRL0"]
pub struct EEPROM_CTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM_CTRL0"]
pub mod eeprom_ctrl0;
#[doc = "EEPROM_CTRL1"]
pub struct EEPROM_CTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM_CTRL1"]
pub mod eeprom_ctrl1;
#[doc = "EFUSE_CTRL"]
pub struct EFUSE_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EFUSE_CTRL"]
pub mod efuse_ctrl;
#[doc = "EFUSE_TEST"]
pub struct EFUSE_TEST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EFUSE_TEST"]
pub mod efuse_test;
#[doc = "DSTBY_INFO0"]
pub struct DSTBY_INFO0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSTBY_INFO0"]
pub mod dstby_info0;
#[doc = "DSTBY_INFO1"]
pub struct DSTBY_INFO1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSTBY_INFO1"]
pub mod dstby_info1;
#[doc = "DSTBY_INFO2"]
pub struct DSTBY_INFO2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSTBY_INFO2"]
pub mod dstby_info2;
#[doc = "DSTBY_INFO3"]
pub struct DSTBY_INFO3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSTBY_INFO3"]
pub mod dstby_info3;
#[doc = "SLP_WAKE_EVENT_MSK0"]
pub struct SLP_WAKE_EVENT_MSK0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SLP_WAKE_EVENT_MSK0"]
pub mod slp_wake_event_msk0;
#[doc = "SLP_WAKE_EVENT_MSK1"]
pub struct SLP_WAKE_EVENT_MSK1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SLP_WAKE_EVENT_MSK1"]
pub mod slp_wake_event_msk1;
#[doc = "SLP_WAKE_EVENT_STATUS0"]
pub struct SLP_WAKE_EVENT_STATUS0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SLP_WAKE_EVENT_STATUS0"]
pub mod slp_wake_event_status0;
#[doc = "SLP_WAKE_EVENT_STATUS1"]
pub struct SLP_WAKE_EVENT_STATUS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SLP_WAKE_EVENT_STATUS1"]
pub mod slp_wake_event_status1;
#[doc = "SNF_WAKE_EVENT_MSK0"]
pub struct SNF_WAKE_EVENT_MSK0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SNF_WAKE_EVENT_MSK0"]
pub mod snf_wake_event_msk0;
#[doc = "SNF_WAKE_EVENT_STATUS"]
pub struct SNF_WAKE_EVENT_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SNF_WAKE_EVENT_STATUS"]
pub mod snf_wake_event_status;
#[doc = "PWRMGT_CTRL"]
pub struct PWRMGT_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWRMGT_CTRL"]
pub mod pwrmgt_ctrl;
#[doc = "PWRMGT_OPTION"]
pub struct PWRMGT_OPTION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWRMGT_OPTION"]
pub mod pwrmgt_option;
#[doc = "PWRMGT_OPTION_EXT"]
pub struct PWRMGT_OPTION_EXT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWRMGT_OPTION_EXT"]
pub mod pwrmgt_option_ext;
#[doc = "DSLP_WEVENT"]
pub struct DSLP_WEVENT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSLP_WEVENT"]
pub mod dslp_wevent;
#[doc = "PERI_MONITOR"]
pub struct PERI_MONITOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PERI_MONITOR"]
pub mod peri_monitor;
#[doc = "SYSTEM_CFG0"]
pub struct SYSTEM_CFG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SYSTEM_CFG0"]
pub mod system_cfg0;
#[doc = "SYSTEM_CFG1"]
pub struct SYSTEM_CFG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SYSTEM_CFG1"]
pub mod system_cfg1;
#[doc = "SYSTEM_CFG2"]
pub struct SYSTEM_CFG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SYSTEM_CFG2"]
pub mod system_cfg2;
#[doc = "PEON_PWR_CTRL"]
pub struct PEON_PWR_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PEON_PWR_CTRL"]
pub mod peon_pwr_ctrl;
#[doc = "PON_ISO_CTRL"]
pub struct PON_ISO_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PON_ISO_CTRL"]
pub mod pon_iso_ctrl;
#[doc = "SOC_FUNC_EN"]
pub struct SOC_FUNC_EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SOC_FUNC_EN"]
pub mod soc_func_en;
#[doc = "SOC_HCI_COM_FUNC_EN"]
pub struct SOC_HCI_COM_FUNC_EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SOC_HCI_COM_FUNC_EN"]
pub mod soc_hci_com_func_en;
#[doc = "SOC_PERI_FUNC0_EN"]
pub struct SOC_PERI_FUNC0_EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SOC_PERI_FUNC0_EN"]
pub mod soc_peri_func0_en;
#[doc = "SOC_PERI_FUNC1_EN"]
pub struct SOC_PERI_FUNC1_EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SOC_PERI_FUNC1_EN"]
pub mod soc_peri_func1_en;
#[doc = "SOC_PERI_BD_FUNC0_EN"]
pub struct SOC_PERI_BD_FUNC0_EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SOC_PERI_BD_FUNC0_EN"]
pub mod soc_peri_bd_func0_en;
#[doc = "PESOC_CLK_CTRL"]
pub struct PESOC_CLK_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PESOC_CLK_CTRL"]
pub mod pesoc_clk_ctrl;
#[doc = "PESOC_PERI_CLK_CTRL0"]
pub struct PESOC_PERI_CLK_CTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PESOC_PERI_CLK_CTRL0"]
pub mod pesoc_peri_clk_ctrl0;
#[doc = "PESOC_PERI_CLK_CTRL1"]
pub struct PESOC_PERI_CLK_CTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PESOC_PERI_CLK_CTRL1"]
pub mod pesoc_peri_clk_ctrl1;
#[doc = "PESOC_CLK_CTRL3"]
pub struct PESOC_CLK_CTRL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PESOC_CLK_CTRL3"]
pub mod pesoc_clk_ctrl3;
#[doc = "PESOC_HCI_CLK_CTRL0"]
pub struct PESOC_HCI_CLK_CTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PESOC_HCI_CLK_CTRL0"]
pub mod pesoc_hci_clk_ctrl0;
#[doc = "PESOC_COM_CLK_CTRL1"]
pub struct PESOC_COM_CLK_CTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PESOC_COM_CLK_CTRL1"]
pub mod pesoc_com_clk_ctrl1;
#[doc = "PESOC_HW_ENG_CLK_CTRL"]
pub struct PESOC_HW_ENG_CLK_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PESOC_HW_ENG_CLK_CTRL"]
pub mod pesoc_hw_eng_clk_ctrl;
#[doc = "PESOC_CLK_SEL"]
pub struct PESOC_CLK_SEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PESOC_CLK_SEL"]
pub mod pesoc_clk_sel;
#[doc = "SYS_ANACK_CAL_CTRL"]
pub struct SYS_ANACK_CAL_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SYS_ANACK_CAL_CTRL"]
pub mod sys_anack_cal_ctrl;
#[doc = "OSC32K_CTRL"]
pub struct OSC32K_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OSC32K_CTRL"]
pub mod osc32k_ctrl;
#[doc = "OSC32K_REG_CTRL0"]
pub struct OSC32K_REG_CTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OSC32K_REG_CTRL0"]
pub mod osc32k_reg_ctrl0;
#[doc = "OSC32K_REG_CTRL1"]
pub struct OSC32K_REG_CTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OSC32K_REG_CTRL1"]
pub mod osc32k_reg_ctrl1;
#[doc = "THERMAL_METER_CTRL"]
pub struct THERMAL_METER_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "THERMAL_METER_CTRL"]
pub mod thermal_meter_ctrl;
#[doc = "UART_MUX_CTRL"]
pub struct UART_MUX_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART_MUX_CTRL"]
pub mod uart_mux_ctrl;
#[doc = "SPI_MUX_CTRL"]
pub struct SPI_MUX_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI_MUX_CTRL"]
pub mod spi_mux_ctrl;
#[doc = "I2C_MUX_CTRL"]
pub struct I2C_MUX_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C_MUX_CTRL"]
pub mod i2c_mux_ctrl;
#[doc = "I2S_MUX_CTRL"]
pub struct I2S_MUX_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2S_MUX_CTRL"]
pub mod i2s_mux_ctrl;
#[doc = "HCI_PINMUX_CTRL"]
pub struct HCI_PINMUX_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HCI_PINMUX_CTRL"]
pub mod hci_pinmux_ctrl;
#[doc = "WL_PINMUX_CTRL"]
pub struct WL_PINMUX_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WL_PINMUX_CTRL"]
pub mod wl_pinmux_ctrl;
#[doc = "BT_PINMUX_CTRL"]
pub struct BT_PINMUX_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BT_PINMUX_CTRL"]
pub mod bt_pinmux_ctrl;
#[doc = "PWM_PINMUX_CTRL"]
pub struct PWM_PINMUX_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM_PINMUX_CTRL"]
pub mod pwm_pinmux_ctrl;
#[doc = "CPU_PERIPHERAL_CTRL"]
pub struct CPU_PERIPHERAL_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU_PERIPHERAL_CTRL"]
pub mod cpu_peripheral_ctrl;
#[doc = "HCI_CTRL_STATUS_0"]
pub struct HCI_CTRL_STATUS_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HCI_CTRL_STATUS_0"]
pub mod hci_ctrl_status_0;
#[doc = "HCI_CTRL_STATUS_1"]
pub struct HCI_CTRL_STATUS_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HCI_CTRL_STATUS_1"]
pub mod hci_ctrl_status_1;
#[doc = "PESOC_MEM_CTRL"]
pub struct PESOC_MEM_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PESOC_MEM_CTRL"]
pub mod pesoc_mem_ctrl;
#[doc = "PESOC_SOC_CTRL"]
pub struct PESOC_SOC_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PESOC_SOC_CTRL"]
pub mod pesoc_soc_ctrl;
#[doc = "PESOC_PERI_CTRL"]
pub struct PESOC_PERI_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PESOC_PERI_CTRL"]
pub mod pesoc_peri_ctrl;
#[doc = "GPIO_SHTDN_CTRL"]
pub struct GPIO_SHTDN_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO_SHTDN_CTRL"]
pub mod gpio_shtdn_ctrl;
#[doc = "GPIO_DRIVING_CTRL"]
pub struct GPIO_DRIVING_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO_DRIVING_CTRL"]
pub mod gpio_driving_ctrl;
#[doc = "GPIO_PULL_CTRL0"]
pub struct GPIO_PULL_CTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO_PULL_CTRL0"]
pub mod gpio_pull_ctrl0;
#[doc = "GPIO_PULL_CTRL1"]
pub struct GPIO_PULL_CTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO_PULL_CTRL1"]
pub mod gpio_pull_ctrl1;
#[doc = "GPIO_PULL_CTRL2"]
pub struct GPIO_PULL_CTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO_PULL_CTRL2"]
pub mod gpio_pull_ctrl2;
#[doc = "GPIO_PULL_CTRL3"]
pub struct GPIO_PULL_CTRL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO_PULL_CTRL3"]
pub mod gpio_pull_ctrl3;
#[doc = "GPIO_PULL_CTRL4"]
pub struct GPIO_PULL_CTRL4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO_PULL_CTRL4"]
pub mod gpio_pull_ctrl4;
#[doc = "GPIO_PULL_CTRL5"]
pub struct GPIO_PULL_CTRL5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO_PULL_CTRL5"]
pub mod gpio_pull_ctrl5;
#[doc = "GPIO_PULL_CTRL6"]
pub struct GPIO_PULL_CTRL6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO_PULL_CTRL6"]
pub mod gpio_pull_ctrl6;
#[doc = "PERI_PWM0_CTRL"]
pub struct PERI_PWM0_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PERI_PWM0_CTRL"]
pub mod peri_pwm0_ctrl;
#[doc = "PERI_PWM1_CTRL"]
pub struct PERI_PWM1_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PERI_PWM1_CTRL"]
pub mod peri_pwm1_ctrl;
#[doc = "PERI_PWM2_CTRL"]
pub struct PERI_PWM2_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PERI_PWM2_CTRL"]
pub mod peri_pwm2_ctrl;
#[doc = "PERI_PWM3_CTRL"]
pub struct PERI_PWM3_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PERI_PWM3_CTRL"]
pub mod peri_pwm3_ctrl;
#[doc = "PERI_TIM_EVT_CTRL"]
pub struct PERI_TIM_EVT_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PERI_TIM_EVT_CTRL"]
pub mod peri_tim_evt_ctrl;
#[doc = "PERI_EGTIM_CTRL"]
pub struct PERI_EGTIM_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PERI_EGTIM_CTRL"]
pub mod peri_egtim_ctrl;
#[doc = "PEON_CFG"]
pub struct PEON_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PEON_CFG"]
pub mod peon_cfg;
#[doc = "PEON_STATUS"]
pub struct PEON_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PEON_STATUS"]
pub mod peon_status;
