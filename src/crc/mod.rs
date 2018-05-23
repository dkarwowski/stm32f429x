use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data register"] pub dr: Dr,
    #[doc = "0x04 - Independent Data register"] pub idr: Idr,
    #[doc = "0x08 - Control register"] pub cr: Cr,
}
#[doc = "Data register"]
pub struct Dr {
    register: VolatileCell<u32>,
}
#[doc = "Data register"]
pub mod dr;
#[doc = "Independent Data register"]
pub struct Idr {
    register: VolatileCell<u32>,
}
#[doc = "Independent Data register"]
pub mod idr;
#[doc = "Control register"]
pub struct Cr {
    register: VolatileCell<u32>,
}
#[doc = "Control register"]
pub mod cr;
