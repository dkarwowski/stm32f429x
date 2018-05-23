use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - power control register"] pub cr: Cr,
    #[doc = "0x04 - power control/status register"] pub csr: Csr,
}
#[doc = "power control register"]
pub struct Cr {
    register: VolatileCell<u32>,
}
#[doc = "power control register"]
pub mod cr;
#[doc = "power control/status register"]
pub struct Csr {
    register: VolatileCell<u32>,
}
#[doc = "power control/status register"]
pub mod csr;
