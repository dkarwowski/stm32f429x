use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC Common status register"] pub csr: Csr,
    #[doc = "0x04 - ADC common control register"] pub ccr: Ccr,
    #[doc = "0x08 - ADC common regular data register for dual and triple modes"] pub cdr: Cdr,
}
#[doc = "ADC Common status register"]
pub struct Csr {
    register: VolatileCell<u32>,
}
#[doc = "ADC Common status register"]
pub mod csr;
#[doc = "ADC common control register"]
pub struct Ccr {
    register: VolatileCell<u32>,
}
#[doc = "ADC common control register"]
pub mod ccr;
#[doc = "ADC common regular data register for dual and triple modes"]
pub struct Cdr {
    register: VolatileCell<u32>,
}
#[doc = "ADC common regular data register for dual and triple modes"]
pub mod cdr;
