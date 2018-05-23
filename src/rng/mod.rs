use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"] pub cr: Cr,
    #[doc = "0x04 - status register"] pub sr: Sr,
    #[doc = "0x08 - data register"] pub dr: Dr,
}
#[doc = "control register"]
pub struct Cr {
    register: VolatileCell<u32>,
}
#[doc = "control register"]
pub mod cr;
#[doc = "status register"]
pub struct Sr {
    register: VolatileCell<u32>,
}
#[doc = "status register"]
pub mod sr;
#[doc = "data register"]
pub struct Dr {
    register: VolatileCell<u32>,
}
#[doc = "data register"]
pub mod dr;
