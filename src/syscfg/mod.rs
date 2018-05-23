use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - memory remap register"] pub memrm: Memrm,
    #[doc = "0x04 - peripheral mode configuration register"] pub pmc: Pmc,
    #[doc = "0x08 - external interrupt configuration register 1"] pub exticr1: Exticr1,
    #[doc = "0x0c - external interrupt configuration register 2"] pub exticr2: Exticr2,
    #[doc = "0x10 - external interrupt configuration register 3"] pub exticr3: Exticr3,
    #[doc = "0x14 - external interrupt configuration register 4"] pub exticr4: Exticr4,
    _reserved0: [u8; 8usize],
    #[doc = "0x20 - Compensation cell control register"] pub cmpcr: Cmpcr,
}
#[doc = "memory remap register"]
pub struct Memrm {
    register: VolatileCell<u32>,
}
#[doc = "memory remap register"]
pub mod memrm;
#[doc = "peripheral mode configuration register"]
pub struct Pmc {
    register: VolatileCell<u32>,
}
#[doc = "peripheral mode configuration register"]
pub mod pmc;
#[doc = "external interrupt configuration register 1"]
pub struct Exticr1 {
    register: VolatileCell<u32>,
}
#[doc = "external interrupt configuration register 1"]
pub mod exticr1;
#[doc = "external interrupt configuration register 2"]
pub struct Exticr2 {
    register: VolatileCell<u32>,
}
#[doc = "external interrupt configuration register 2"]
pub mod exticr2;
#[doc = "external interrupt configuration register 3"]
pub struct Exticr3 {
    register: VolatileCell<u32>,
}
#[doc = "external interrupt configuration register 3"]
pub mod exticr3;
#[doc = "external interrupt configuration register 4"]
pub struct Exticr4 {
    register: VolatileCell<u32>,
}
#[doc = "external interrupt configuration register 4"]
pub mod exticr4;
#[doc = "Compensation cell control register"]
pub struct Cmpcr {
    register: VolatileCell<u32>,
}
#[doc = "Compensation cell control register"]
pub mod cmpcr;
