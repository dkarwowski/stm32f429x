#![doc = "Peripheral access API for STM32F429X microcontrollers (generated using svd2rust v0.7.0)"]
#![deny(missing_docs)]
#![deny(warnings)]
#![feature(const_fn)]
#![no_std]
extern crate cortex_m;
extern crate vcell;
use core::ops::Deref;
use cortex_m::peripheral::Peripheral;
#[doc = r" Interrupts"]
pub mod interrupt;
pub use cortex_m::peripheral::Cpuid;
pub use cortex_m::peripheral::CPUID;
pub use cortex_m::peripheral::Dcb;
pub use cortex_m::peripheral::DCB;
pub use cortex_m::peripheral::Dwt;
pub use cortex_m::peripheral::DWT;
pub use cortex_m::peripheral::Fpb;
pub use cortex_m::peripheral::FPB;
pub use cortex_m::peripheral::Fpu;
pub use cortex_m::peripheral::FPU;
pub use cortex_m::peripheral::Itm;
pub use cortex_m::peripheral::ITM;
pub use cortex_m::peripheral::Mpu;
pub use cortex_m::peripheral::MPU;
pub use cortex_m::peripheral::Nvic;
pub use cortex_m::peripheral::NVIC;
pub use cortex_m::peripheral::Scb;
pub use cortex_m::peripheral::SCB;
pub use cortex_m::peripheral::Syst;
pub use cortex_m::peripheral::SYST;
pub use cortex_m::peripheral::Tpiu;
pub use cortex_m::peripheral::TPIU;
#[doc = "Random number generator"]
pub const RNG: Peripheral<Rng> = unsafe { Peripheral::new(1342572544) };
#[doc = "Random number generator"]
pub mod rng;
#[doc = "Random number generator"]
pub struct Rng {
    register_block: rng::RegisterBlock,
}
impl Deref for Rng {
    type Target = rng::RegisterBlock;
    fn deref(&self) -> &rng::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Digital camera interface"]
pub const DCMI: Peripheral<Dcmi> = unsafe { Peripheral::new(1342504960) };
#[doc = "Digital camera interface"]
pub mod dcmi;
#[doc = "Digital camera interface"]
pub struct Dcmi {
    register_block: dcmi::RegisterBlock,
}
impl Deref for Dcmi {
    type Target = dcmi::RegisterBlock;
    fn deref(&self) -> &dcmi::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Flexible memory controller"]
pub const FMC: Peripheral<Fmc> = unsafe { Peripheral::new(2684354560) };
#[doc = "Flexible memory controller"]
pub mod fmc;
#[doc = "Flexible memory controller"]
pub struct Fmc {
    register_block: fmc::RegisterBlock,
}
impl Deref for Fmc {
    type Target = fmc::RegisterBlock;
    fn deref(&self) -> &fmc::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Debug support"]
pub const DBG: Peripheral<Dbg> = unsafe { Peripheral::new(3758366720) };
#[doc = "Debug support"]
pub mod dbg;
#[doc = "Debug support"]
pub struct Dbg {
    register_block: dbg::RegisterBlock,
}
impl Deref for Dbg {
    type Target = dbg::RegisterBlock;
    fn deref(&self) -> &dbg::RegisterBlock {
        &self.register_block
    }
}
#[doc = "DMA controller"]
pub const DMA2: Peripheral<Dma2> = unsafe { Peripheral::new(1073898496) };
#[doc = "DMA controller"]
pub mod dma2;
#[doc = "DMA controller"]
pub struct Dma2 {
    register_block: dma2::RegisterBlock,
}
impl Deref for Dma2 {
    type Target = dma2::RegisterBlock;
    fn deref(&self) -> &dma2::RegisterBlock {
        &self.register_block
    }
}
#[doc = "DMA1"]
pub const DMA1: Peripheral<Dma1> = unsafe { Peripheral::new(1073897472) };
#[doc = r" Register block"]
pub struct Dma1 {
    register_block: dma2::RegisterBlock,
}
impl Deref for Dma1 {
    type Target = dma2::RegisterBlock;
    fn deref(&self) -> &dma2::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Reset and clock control"]
pub const RCC: Peripheral<Rcc> = unsafe { Peripheral::new(1073887232) };
#[doc = "Reset and clock control"]
pub mod rcc;
#[doc = "Reset and clock control"]
pub struct Rcc {
    register_block: rcc::RegisterBlock,
}
impl Deref for Rcc {
    type Target = rcc::RegisterBlock;
    fn deref(&self) -> &rcc::RegisterBlock {
        &self.register_block
    }
}
#[doc = "General-purpose I/Os"]
pub const GPIOK: Peripheral<Gpiok> = unsafe { Peripheral::new(1073883136) };
#[doc = "General-purpose I/Os"]
pub mod gpiok;
#[doc = "General-purpose I/Os"]
pub struct Gpiok {
    register_block: gpiok::RegisterBlock,
}
impl Deref for Gpiok {
    type Target = gpiok::RegisterBlock;
    fn deref(&self) -> &gpiok::RegisterBlock {
        &self.register_block
    }
}
#[doc = "GPIOJ"]
pub const GPIOJ: Peripheral<Gpioj> = unsafe { Peripheral::new(1073882112) };
#[doc = r" Register block"]
pub struct Gpioj {
    register_block: gpiok::RegisterBlock,
}
impl Deref for Gpioj {
    type Target = gpiok::RegisterBlock;
    fn deref(&self) -> &gpiok::RegisterBlock {
        &self.register_block
    }
}
#[doc = "GPIOI"]
pub const GPIOI: Peripheral<Gpioi> = unsafe { Peripheral::new(1073881088) };
#[doc = r" Register block"]
pub struct Gpioi {
    register_block: gpiok::RegisterBlock,
}
impl Deref for Gpioi {
    type Target = gpiok::RegisterBlock;
    fn deref(&self) -> &gpiok::RegisterBlock {
        &self.register_block
    }
}
#[doc = "GPIOH"]
pub const GPIOH: Peripheral<Gpioh> = unsafe { Peripheral::new(1073880064) };
#[doc = r" Register block"]
pub struct Gpioh {
    register_block: gpiok::RegisterBlock,
}
impl Deref for Gpioh {
    type Target = gpiok::RegisterBlock;
    fn deref(&self) -> &gpiok::RegisterBlock {
        &self.register_block
    }
}
#[doc = "GPIOG"]
pub const GPIOG: Peripheral<Gpiog> = unsafe { Peripheral::new(1073879040) };
#[doc = r" Register block"]
pub struct Gpiog {
    register_block: gpiok::RegisterBlock,
}
impl Deref for Gpiog {
    type Target = gpiok::RegisterBlock;
    fn deref(&self) -> &gpiok::RegisterBlock {
        &self.register_block
    }
}
#[doc = "GPIOF"]
pub const GPIOF: Peripheral<Gpiof> = unsafe { Peripheral::new(1073878016) };
#[doc = r" Register block"]
pub struct Gpiof {
    register_block: gpiok::RegisterBlock,
}
impl Deref for Gpiof {
    type Target = gpiok::RegisterBlock;
    fn deref(&self) -> &gpiok::RegisterBlock {
        &self.register_block
    }
}
#[doc = "GPIOE"]
pub const GPIOE: Peripheral<Gpioe> = unsafe { Peripheral::new(1073876992) };
#[doc = r" Register block"]
pub struct Gpioe {
    register_block: gpiok::RegisterBlock,
}
impl Deref for Gpioe {
    type Target = gpiok::RegisterBlock;
    fn deref(&self) -> &gpiok::RegisterBlock {
        &self.register_block
    }
}
#[doc = "GPIOD"]
pub const GPIOD: Peripheral<Gpiod> = unsafe { Peripheral::new(1073875968) };
#[doc = r" Register block"]
pub struct Gpiod {
    register_block: gpiok::RegisterBlock,
}
impl Deref for Gpiod {
    type Target = gpiok::RegisterBlock;
    fn deref(&self) -> &gpiok::RegisterBlock {
        &self.register_block
    }
}
#[doc = "GPIOC"]
pub const GPIOC: Peripheral<Gpioc> = unsafe { Peripheral::new(1073874944) };
#[doc = r" Register block"]
pub struct Gpioc {
    register_block: gpiok::RegisterBlock,
}
impl Deref for Gpioc {
    type Target = gpiok::RegisterBlock;
    fn deref(&self) -> &gpiok::RegisterBlock {
        &self.register_block
    }
}
#[doc = "General-purpose I/Os"]
pub const GPIOB: Peripheral<Gpiob> = unsafe { Peripheral::new(1073873920) };
#[doc = "General-purpose I/Os"]
pub mod gpiob;
#[doc = "General-purpose I/Os"]
pub struct Gpiob {
    register_block: gpiob::RegisterBlock,
}
impl Deref for Gpiob {
    type Target = gpiob::RegisterBlock;
    fn deref(&self) -> &gpiob::RegisterBlock {
        &self.register_block
    }
}
#[doc = "General-purpose I/Os"]
pub const GPIOA: Peripheral<Gpioa> = unsafe { Peripheral::new(1073872896) };
#[doc = "General-purpose I/Os"]
pub mod gpioa;
#[doc = "General-purpose I/Os"]
pub struct Gpioa {
    register_block: gpioa::RegisterBlock,
}
impl Deref for Gpioa {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &gpioa::RegisterBlock {
        &self.register_block
    }
}
#[doc = "System configuration controller"]
pub const SYSCFG: Peripheral<Syscfg> = unsafe { Peripheral::new(1073821696) };
#[doc = "System configuration controller"]
pub mod syscfg;
#[doc = "System configuration controller"]
pub struct Syscfg {
    register_block: syscfg::RegisterBlock,
}
impl Deref for Syscfg {
    type Target = syscfg::RegisterBlock;
    fn deref(&self) -> &syscfg::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Serial peripheral interface"]
pub const SPI1: Peripheral<Spi1> = unsafe { Peripheral::new(1073819648) };
#[doc = "Serial peripheral interface"]
pub mod spi1;
#[doc = "Serial peripheral interface"]
pub struct Spi1 {
    register_block: spi1::RegisterBlock,
}
impl Deref for Spi1 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "SPI2"]
pub const SPI2: Peripheral<Spi2> = unsafe { Peripheral::new(1073756160) };
#[doc = r" Register block"]
pub struct Spi2 {
    register_block: spi1::RegisterBlock,
}
impl Deref for Spi2 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "SPI3"]
pub const SPI3: Peripheral<Spi3> = unsafe { Peripheral::new(1073757184) };
#[doc = r" Register block"]
pub struct Spi3 {
    register_block: spi1::RegisterBlock,
}
impl Deref for Spi3 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "I2S2ext"]
pub const I2S2EXT: Peripheral<I2s2ext> = unsafe { Peripheral::new(1073755136) };
#[doc = r" Register block"]
pub struct I2s2ext {
    register_block: spi1::RegisterBlock,
}
impl Deref for I2s2ext {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "I2S3ext"]
pub const I2S3EXT: Peripheral<I2s3ext> = unsafe { Peripheral::new(1073758208) };
#[doc = r" Register block"]
pub struct I2s3ext {
    register_block: spi1::RegisterBlock,
}
impl Deref for I2s3ext {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "SPI4"]
pub const SPI4: Peripheral<Spi4> = unsafe { Peripheral::new(1073820672) };
#[doc = r" Register block"]
pub struct Spi4 {
    register_block: spi1::RegisterBlock,
}
impl Deref for Spi4 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "SPI5"]
pub const SPI5: Peripheral<Spi5> = unsafe { Peripheral::new(1073827840) };
#[doc = r" Register block"]
pub struct Spi5 {
    register_block: spi1::RegisterBlock,
}
impl Deref for Spi5 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "SPI6"]
pub const SPI6: Peripheral<Spi6> = unsafe { Peripheral::new(1073828864) };
#[doc = r" Register block"]
pub struct Spi6 {
    register_block: spi1::RegisterBlock,
}
impl Deref for Spi6 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Secure digital input/output interface"]
pub const SDIO: Peripheral<Sdio> = unsafe { Peripheral::new(1073818624) };
#[doc = "Secure digital input/output interface"]
pub mod sdio;
#[doc = "Secure digital input/output interface"]
pub struct Sdio {
    register_block: sdio::RegisterBlock,
}
impl Deref for Sdio {
    type Target = sdio::RegisterBlock;
    fn deref(&self) -> &sdio::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Analog-to-digital converter"]
pub const ADC1: Peripheral<Adc1> = unsafe { Peripheral::new(1073815552) };
#[doc = "Analog-to-digital converter"]
pub mod adc1;
#[doc = "Analog-to-digital converter"]
pub struct Adc1 {
    register_block: adc1::RegisterBlock,
}
impl Deref for Adc1 {
    type Target = adc1::RegisterBlock;
    fn deref(&self) -> &adc1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "ADC2"]
pub const ADC2: Peripheral<Adc2> = unsafe { Peripheral::new(1073815808) };
#[doc = r" Register block"]
pub struct Adc2 {
    register_block: adc1::RegisterBlock,
}
impl Deref for Adc2 {
    type Target = adc1::RegisterBlock;
    fn deref(&self) -> &adc1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "ADC3"]
pub const ADC3: Peripheral<Adc3> = unsafe { Peripheral::new(1073816064) };
#[doc = r" Register block"]
pub struct Adc3 {
    register_block: adc1::RegisterBlock,
}
impl Deref for Adc3 {
    type Target = adc1::RegisterBlock;
    fn deref(&self) -> &adc1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub const USART6: Peripheral<Usart6> = unsafe { Peripheral::new(1073812480) };
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub mod usart6;
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct Usart6 {
    register_block: usart6::RegisterBlock,
}
impl Deref for Usart6 {
    type Target = usart6::RegisterBlock;
    fn deref(&self) -> &usart6::RegisterBlock {
        &self.register_block
    }
}
#[doc = "USART1"]
pub const USART1: Peripheral<Usart1> = unsafe { Peripheral::new(1073811456) };
#[doc = r" Register block"]
pub struct Usart1 {
    register_block: usart6::RegisterBlock,
}
impl Deref for Usart1 {
    type Target = usart6::RegisterBlock;
    fn deref(&self) -> &usart6::RegisterBlock {
        &self.register_block
    }
}
#[doc = "USART2"]
pub const USART2: Peripheral<Usart2> = unsafe { Peripheral::new(1073759232) };
#[doc = r" Register block"]
pub struct Usart2 {
    register_block: usart6::RegisterBlock,
}
impl Deref for Usart2 {
    type Target = usart6::RegisterBlock;
    fn deref(&self) -> &usart6::RegisterBlock {
        &self.register_block
    }
}
#[doc = "USART3"]
pub const USART3: Peripheral<Usart3> = unsafe { Peripheral::new(1073760256) };
#[doc = r" Register block"]
pub struct Usart3 {
    register_block: usart6::RegisterBlock,
}
impl Deref for Usart3 {
    type Target = usart6::RegisterBlock;
    fn deref(&self) -> &usart6::RegisterBlock {
        &self.register_block
    }
}
#[doc = "UART7"]
pub const UART7: Peripheral<Uart7> = unsafe { Peripheral::new(1073772544) };
#[doc = r" Register block"]
pub struct Uart7 {
    register_block: usart6::RegisterBlock,
}
impl Deref for Uart7 {
    type Target = usart6::RegisterBlock;
    fn deref(&self) -> &usart6::RegisterBlock {
        &self.register_block
    }
}
#[doc = "UART8"]
pub const UART8: Peripheral<Uart8> = unsafe { Peripheral::new(1073773568) };
#[doc = r" Register block"]
pub struct Uart8 {
    register_block: usart6::RegisterBlock,
}
impl Deref for Uart8 {
    type Target = usart6::RegisterBlock;
    fn deref(&self) -> &usart6::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Digital-to-analog converter"]
pub const DAC: Peripheral<Dac> = unsafe { Peripheral::new(1073771520) };
#[doc = "Digital-to-analog converter"]
pub mod dac;
#[doc = "Digital-to-analog converter"]
pub struct Dac {
    register_block: dac::RegisterBlock,
}
impl Deref for Dac {
    type Target = dac::RegisterBlock;
    fn deref(&self) -> &dac::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Inter-integrated circuit"]
pub const I2C3: Peripheral<I2c3> = unsafe { Peripheral::new(1073765376) };
#[doc = "Inter-integrated circuit"]
pub mod i2c3;
#[doc = "Inter-integrated circuit"]
pub struct I2c3 {
    register_block: i2c3::RegisterBlock,
}
impl Deref for I2c3 {
    type Target = i2c3::RegisterBlock;
    fn deref(&self) -> &i2c3::RegisterBlock {
        &self.register_block
    }
}
#[doc = "I2C2"]
pub const I2C2: Peripheral<I2c2> = unsafe { Peripheral::new(1073764352) };
#[doc = r" Register block"]
pub struct I2c2 {
    register_block: i2c3::RegisterBlock,
}
impl Deref for I2c2 {
    type Target = i2c3::RegisterBlock;
    fn deref(&self) -> &i2c3::RegisterBlock {
        &self.register_block
    }
}
#[doc = "I2C1"]
pub const I2C1: Peripheral<I2c1> = unsafe { Peripheral::new(1073763328) };
#[doc = r" Register block"]
pub struct I2c1 {
    register_block: i2c3::RegisterBlock,
}
impl Deref for I2c1 {
    type Target = i2c3::RegisterBlock;
    fn deref(&self) -> &i2c3::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Independent watchdog"]
pub const IWDG: Peripheral<Iwdg> = unsafe { Peripheral::new(1073754112) };
#[doc = "Independent watchdog"]
pub mod iwdg;
#[doc = "Independent watchdog"]
pub struct Iwdg {
    register_block: iwdg::RegisterBlock,
}
impl Deref for Iwdg {
    type Target = iwdg::RegisterBlock;
    fn deref(&self) -> &iwdg::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Window watchdog"]
pub const WWDG: Peripheral<Wwdg> = unsafe { Peripheral::new(1073753088) };
#[doc = "Window watchdog"]
pub mod wwdg;
#[doc = "Window watchdog"]
pub struct Wwdg {
    register_block: wwdg::RegisterBlock,
}
impl Deref for Wwdg {
    type Target = wwdg::RegisterBlock;
    fn deref(&self) -> &wwdg::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Real-time clock"]
pub const RTC: Peripheral<Rtc> = unsafe { Peripheral::new(1073752064) };
#[doc = "Real-time clock"]
pub mod rtc;
#[doc = "Real-time clock"]
pub struct Rtc {
    register_block: rtc::RegisterBlock,
}
impl Deref for Rtc {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub const UART4: Peripheral<Uart4> = unsafe { Peripheral::new(1073761280) };
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub mod uart4;
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct Uart4 {
    register_block: uart4::RegisterBlock,
}
impl Deref for Uart4 {
    type Target = uart4::RegisterBlock;
    fn deref(&self) -> &uart4::RegisterBlock {
        &self.register_block
    }
}
#[doc = "UART5"]
pub const UART5: Peripheral<Uart5> = unsafe { Peripheral::new(1073762304) };
#[doc = r" Register block"]
pub struct Uart5 {
    register_block: uart4::RegisterBlock,
}
impl Deref for Uart5 {
    type Target = uart4::RegisterBlock;
    fn deref(&self) -> &uart4::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Common ADC registers"]
pub const C_ADC: Peripheral<CAdc> = unsafe { Peripheral::new(1073816320) };
#[doc = "Common ADC registers"]
pub mod c_adc;
#[doc = "Common ADC registers"]
pub struct CAdc {
    register_block: c_adc::RegisterBlock,
}
impl Deref for CAdc {
    type Target = c_adc::RegisterBlock;
    fn deref(&self) -> &c_adc::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Advanced-timers"]
pub const TIM1: Peripheral<Tim1> = unsafe { Peripheral::new(1073807360) };
#[doc = "Advanced-timers"]
pub mod tim1;
#[doc = "Advanced-timers"]
pub struct Tim1 {
    register_block: tim1::RegisterBlock,
}
impl Deref for Tim1 {
    type Target = tim1::RegisterBlock;
    fn deref(&self) -> &tim1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "TIM8"]
pub const TIM8: Peripheral<Tim8> = unsafe { Peripheral::new(1073808384) };
#[doc = r" Register block"]
pub struct Tim8 {
    register_block: tim1::RegisterBlock,
}
impl Deref for Tim8 {
    type Target = tim1::RegisterBlock;
    fn deref(&self) -> &tim1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "General purpose timers"]
pub const TIM2: Peripheral<Tim2> = unsafe { Peripheral::new(1073741824) };
#[doc = "General purpose timers"]
pub mod tim2;
#[doc = "General purpose timers"]
pub struct Tim2 {
    register_block: tim2::RegisterBlock,
}
impl Deref for Tim2 {
    type Target = tim2::RegisterBlock;
    fn deref(&self) -> &tim2::RegisterBlock {
        &self.register_block
    }
}
#[doc = "General purpose timers"]
pub const TIM3: Peripheral<Tim3> = unsafe { Peripheral::new(1073742848) };
#[doc = "General purpose timers"]
pub mod tim3;
#[doc = "General purpose timers"]
pub struct Tim3 {
    register_block: tim3::RegisterBlock,
}
impl Deref for Tim3 {
    type Target = tim3::RegisterBlock;
    fn deref(&self) -> &tim3::RegisterBlock {
        &self.register_block
    }
}
#[doc = "TIM4"]
pub const TIM4: Peripheral<Tim4> = unsafe { Peripheral::new(1073743872) };
#[doc = r" Register block"]
pub struct Tim4 {
    register_block: tim3::RegisterBlock,
}
impl Deref for Tim4 {
    type Target = tim3::RegisterBlock;
    fn deref(&self) -> &tim3::RegisterBlock {
        &self.register_block
    }
}
#[doc = "General-purpose-timers"]
pub const TIM5: Peripheral<Tim5> = unsafe { Peripheral::new(1073744896) };
#[doc = "General-purpose-timers"]
pub mod tim5;
#[doc = "General-purpose-timers"]
pub struct Tim5 {
    register_block: tim5::RegisterBlock,
}
impl Deref for Tim5 {
    type Target = tim5::RegisterBlock;
    fn deref(&self) -> &tim5::RegisterBlock {
        &self.register_block
    }
}
#[doc = "General purpose timers"]
pub const TIM9: Peripheral<Tim9> = unsafe { Peripheral::new(1073823744) };
#[doc = "General purpose timers"]
pub mod tim9;
#[doc = "General purpose timers"]
pub struct Tim9 {
    register_block: tim9::RegisterBlock,
}
impl Deref for Tim9 {
    type Target = tim9::RegisterBlock;
    fn deref(&self) -> &tim9::RegisterBlock {
        &self.register_block
    }
}
#[doc = "TIM12"]
pub const TIM12: Peripheral<Tim12> = unsafe { Peripheral::new(1073747968) };
#[doc = r" Register block"]
pub struct Tim12 {
    register_block: tim9::RegisterBlock,
}
impl Deref for Tim12 {
    type Target = tim9::RegisterBlock;
    fn deref(&self) -> &tim9::RegisterBlock {
        &self.register_block
    }
}
#[doc = "General-purpose-timers"]
pub const TIM10: Peripheral<Tim10> = unsafe { Peripheral::new(1073824768) };
#[doc = "General-purpose-timers"]
pub mod tim10;
#[doc = "General-purpose-timers"]
pub struct Tim10 {
    register_block: tim10::RegisterBlock,
}
impl Deref for Tim10 {
    type Target = tim10::RegisterBlock;
    fn deref(&self) -> &tim10::RegisterBlock {
        &self.register_block
    }
}
#[doc = "TIM13"]
pub const TIM13: Peripheral<Tim13> = unsafe { Peripheral::new(1073748992) };
#[doc = r" Register block"]
pub struct Tim13 {
    register_block: tim10::RegisterBlock,
}
impl Deref for Tim13 {
    type Target = tim10::RegisterBlock;
    fn deref(&self) -> &tim10::RegisterBlock {
        &self.register_block
    }
}
#[doc = "TIM14"]
pub const TIM14: Peripheral<Tim14> = unsafe { Peripheral::new(1073750016) };
#[doc = r" Register block"]
pub struct Tim14 {
    register_block: tim10::RegisterBlock,
}
impl Deref for Tim14 {
    type Target = tim10::RegisterBlock;
    fn deref(&self) -> &tim10::RegisterBlock {
        &self.register_block
    }
}
#[doc = "General-purpose-timers"]
pub const TIM11: Peripheral<Tim11> = unsafe { Peripheral::new(1073825792) };
#[doc = "General-purpose-timers"]
pub mod tim11;
#[doc = "General-purpose-timers"]
pub struct Tim11 {
    register_block: tim11::RegisterBlock,
}
impl Deref for Tim11 {
    type Target = tim11::RegisterBlock;
    fn deref(&self) -> &tim11::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Basic timers"]
pub const TIM6: Peripheral<Tim6> = unsafe { Peripheral::new(1073745920) };
#[doc = "Basic timers"]
pub mod tim6;
#[doc = "Basic timers"]
pub struct Tim6 {
    register_block: tim6::RegisterBlock,
}
impl Deref for Tim6 {
    type Target = tim6::RegisterBlock;
    fn deref(&self) -> &tim6::RegisterBlock {
        &self.register_block
    }
}
#[doc = "TIM7"]
pub const TIM7: Peripheral<Tim7> = unsafe { Peripheral::new(1073746944) };
#[doc = r" Register block"]
pub struct Tim7 {
    register_block: tim6::RegisterBlock,
}
impl Deref for Tim7 {
    type Target = tim6::RegisterBlock;
    fn deref(&self) -> &tim6::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Ethernet: media access control (MAC)"]
pub const ETHERNET_MAC: Peripheral<EthernetMac> = unsafe { Peripheral::new(1073905664) };
#[doc = "Ethernet: media access control (MAC)"]
pub mod ethernet_mac;
#[doc = "Ethernet: media access control (MAC)"]
pub struct EthernetMac {
    register_block: ethernet_mac::RegisterBlock,
}
impl Deref for EthernetMac {
    type Target = ethernet_mac::RegisterBlock;
    fn deref(&self) -> &ethernet_mac::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Ethernet: MAC management counters"]
pub const ETHERNET_MMC: Peripheral<EthernetMmc> = unsafe { Peripheral::new(1073905920) };
#[doc = "Ethernet: MAC management counters"]
pub mod ethernet_mmc;
#[doc = "Ethernet: MAC management counters"]
pub struct EthernetMmc {
    register_block: ethernet_mmc::RegisterBlock,
}
impl Deref for EthernetMmc {
    type Target = ethernet_mmc::RegisterBlock;
    fn deref(&self) -> &ethernet_mmc::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Ethernet: Precision time protocol"]
pub const ETHERNET_PTP: Peripheral<EthernetPtp> = unsafe { Peripheral::new(1073907456) };
#[doc = "Ethernet: Precision time protocol"]
pub mod ethernet_ptp;
#[doc = "Ethernet: Precision time protocol"]
pub struct EthernetPtp {
    register_block: ethernet_ptp::RegisterBlock,
}
impl Deref for EthernetPtp {
    type Target = ethernet_ptp::RegisterBlock;
    fn deref(&self) -> &ethernet_ptp::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Ethernet: DMA controller operation"]
pub const ETHERNET_DMA: Peripheral<EthernetDma> = unsafe { Peripheral::new(1073909760) };
#[doc = "Ethernet: DMA controller operation"]
pub mod ethernet_dma;
#[doc = "Ethernet: DMA controller operation"]
pub struct EthernetDma {
    register_block: ethernet_dma::RegisterBlock,
}
impl Deref for EthernetDma {
    type Target = ethernet_dma::RegisterBlock;
    fn deref(&self) -> &ethernet_dma::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Cryptographic processor"]
pub const CRC: Peripheral<Crc> = unsafe { Peripheral::new(1073885184) };
#[doc = "Cryptographic processor"]
pub mod crc;
#[doc = "Cryptographic processor"]
pub struct Crc {
    register_block: crc::RegisterBlock,
}
impl Deref for Crc {
    type Target = crc::RegisterBlock;
    fn deref(&self) -> &crc::RegisterBlock {
        &self.register_block
    }
}
#[doc = "USB on the go full speed"]
pub const OTG_FS_GLOBAL: Peripheral<OtgFsGlobal> = unsafe { Peripheral::new(1342177280) };
#[doc = "USB on the go full speed"]
pub mod otg_fs_global;
#[doc = "USB on the go full speed"]
pub struct OtgFsGlobal {
    register_block: otg_fs_global::RegisterBlock,
}
impl Deref for OtgFsGlobal {
    type Target = otg_fs_global::RegisterBlock;
    fn deref(&self) -> &otg_fs_global::RegisterBlock {
        &self.register_block
    }
}
#[doc = "USB on the go full speed"]
pub const OTG_FS_HOST: Peripheral<OtgFsHost> = unsafe { Peripheral::new(1342178304) };
#[doc = "USB on the go full speed"]
pub mod otg_fs_host;
#[doc = "USB on the go full speed"]
pub struct OtgFsHost {
    register_block: otg_fs_host::RegisterBlock,
}
impl Deref for OtgFsHost {
    type Target = otg_fs_host::RegisterBlock;
    fn deref(&self) -> &otg_fs_host::RegisterBlock {
        &self.register_block
    }
}
#[doc = "USB on the go full speed"]
pub const OTG_FS_DEVICE: Peripheral<OtgFsDevice> = unsafe { Peripheral::new(1342179328) };
#[doc = "USB on the go full speed"]
pub mod otg_fs_device;
#[doc = "USB on the go full speed"]
pub struct OtgFsDevice {
    register_block: otg_fs_device::RegisterBlock,
}
impl Deref for OtgFsDevice {
    type Target = otg_fs_device::RegisterBlock;
    fn deref(&self) -> &otg_fs_device::RegisterBlock {
        &self.register_block
    }
}
#[doc = "USB on the go full speed"]
pub const OTG_FS_PWRCLK: Peripheral<OtgFsPwrclk> = unsafe { Peripheral::new(1342180864) };
#[doc = "USB on the go full speed"]
pub mod otg_fs_pwrclk;
#[doc = "USB on the go full speed"]
pub struct OtgFsPwrclk {
    register_block: otg_fs_pwrclk::RegisterBlock,
}
impl Deref for OtgFsPwrclk {
    type Target = otg_fs_pwrclk::RegisterBlock;
    fn deref(&self) -> &otg_fs_pwrclk::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Controller area network"]
pub const CAN1: Peripheral<Can1> = unsafe { Peripheral::new(1073767424) };
#[doc = "Controller area network"]
pub mod can1;
#[doc = "Controller area network"]
pub struct Can1 {
    register_block: can1::RegisterBlock,
}
impl Deref for Can1 {
    type Target = can1::RegisterBlock;
    fn deref(&self) -> &can1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "CAN2"]
pub const CAN2: Peripheral<Can2> = unsafe { Peripheral::new(1073768448) };
#[doc = r" Register block"]
pub struct Can2 {
    register_block: can1::RegisterBlock,
}
impl Deref for Can2 {
    type Target = can1::RegisterBlock;
    fn deref(&self) -> &can1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "FLASH"]
pub const FLASH: Peripheral<Flash> = unsafe { Peripheral::new(1073888256) };
#[doc = "FLASH"]
pub mod flash;
#[doc = "FLASH"]
pub struct Flash {
    register_block: flash::RegisterBlock,
}
impl Deref for Flash {
    type Target = flash::RegisterBlock;
    fn deref(&self) -> &flash::RegisterBlock {
        &self.register_block
    }
}
#[doc = "External interrupt/event controller"]
pub const EXTI: Peripheral<Exti> = unsafe { Peripheral::new(1073822720) };
#[doc = "External interrupt/event controller"]
pub mod exti;
#[doc = "External interrupt/event controller"]
pub struct Exti {
    register_block: exti::RegisterBlock,
}
impl Deref for Exti {
    type Target = exti::RegisterBlock;
    fn deref(&self) -> &exti::RegisterBlock {
        &self.register_block
    }
}
#[doc = "USB on the go high speed"]
pub const OTG_HS_GLOBAL: Peripheral<OtgHsGlobal> = unsafe { Peripheral::new(1074003968) };
#[doc = "USB on the go high speed"]
pub mod otg_hs_global;
#[doc = "USB on the go high speed"]
pub struct OtgHsGlobal {
    register_block: otg_hs_global::RegisterBlock,
}
impl Deref for OtgHsGlobal {
    type Target = otg_hs_global::RegisterBlock;
    fn deref(&self) -> &otg_hs_global::RegisterBlock {
        &self.register_block
    }
}
#[doc = "USB on the go high speed"]
pub const OTG_HS_HOST: Peripheral<OtgHsHost> = unsafe { Peripheral::new(1074004992) };
#[doc = "USB on the go high speed"]
pub mod otg_hs_host;
#[doc = "USB on the go high speed"]
pub struct OtgHsHost {
    register_block: otg_hs_host::RegisterBlock,
}
impl Deref for OtgHsHost {
    type Target = otg_hs_host::RegisterBlock;
    fn deref(&self) -> &otg_hs_host::RegisterBlock {
        &self.register_block
    }
}
#[doc = "USB on the go high speed"]
pub const OTG_HS_DEVICE: Peripheral<OtgHsDevice> = unsafe { Peripheral::new(1074006016) };
#[doc = "USB on the go high speed"]
pub mod otg_hs_device;
#[doc = "USB on the go high speed"]
pub struct OtgHsDevice {
    register_block: otg_hs_device::RegisterBlock,
}
impl Deref for OtgHsDevice {
    type Target = otg_hs_device::RegisterBlock;
    fn deref(&self) -> &otg_hs_device::RegisterBlock {
        &self.register_block
    }
}
#[doc = "USB on the go high speed"]
pub const OTG_HS_PWRCLK: Peripheral<OtgHsPwrclk> = unsafe { Peripheral::new(1074007552) };
#[doc = "USB on the go high speed"]
pub mod otg_hs_pwrclk;
#[doc = "USB on the go high speed"]
pub struct OtgHsPwrclk {
    register_block: otg_hs_pwrclk::RegisterBlock,
}
impl Deref for OtgHsPwrclk {
    type Target = otg_hs_pwrclk::RegisterBlock;
    fn deref(&self) -> &otg_hs_pwrclk::RegisterBlock {
        &self.register_block
    }
}
#[doc = "LCD-TFT Controller"]
pub const LTDC: Peripheral<Ltdc> = unsafe { Peripheral::new(1073833984) };
#[doc = "LCD-TFT Controller"]
pub mod ltdc;
#[doc = "LCD-TFT Controller"]
pub struct Ltdc {
    register_block: ltdc::RegisterBlock,
}
impl Deref for Ltdc {
    type Target = ltdc::RegisterBlock;
    fn deref(&self) -> &ltdc::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Serial audio interface"]
pub const SAI: Peripheral<Sai> = unsafe { Peripheral::new(1073829888) };
#[doc = "Serial audio interface"]
pub mod sai;
#[doc = "Serial audio interface"]
pub struct Sai {
    register_block: sai::RegisterBlock,
}
impl Deref for Sai {
    type Target = sai::RegisterBlock;
    fn deref(&self) -> &sai::RegisterBlock {
        &self.register_block
    }
}
#[doc = "DMA2D controller"]
pub const DMA2D: Peripheral<Dma2d> = unsafe { Peripheral::new(1073917952) };
#[doc = "DMA2D controller"]
pub mod dma2d;
#[doc = "DMA2D controller"]
pub struct Dma2d {
    register_block: dma2d::RegisterBlock,
}
impl Deref for Dma2d {
    type Target = dma2d::RegisterBlock;
    fn deref(&self) -> &dma2d::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Power control"]
pub const PWR: Peripheral<Pwr> = unsafe { Peripheral::new(1073770496) };
#[doc = "Power control"]
pub mod pwr;
#[doc = "Power control"]
pub struct Pwr {
    register_block: pwr::RegisterBlock,
}
impl Deref for Pwr {
    type Target = pwr::RegisterBlock;
    fn deref(&self) -> &pwr::RegisterBlock {
        &self.register_block
    }
}
