use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - AConfiguration register 1"] pub acr1: Acr1,
    #[doc = "0x08 - AConfiguration register 2"] pub acr2: Acr2,
    #[doc = "0x0c - AFRCR"] pub afrcr: Afrcr,
    #[doc = "0x10 - ASlot register"] pub aslotr: Aslotr,
    #[doc = "0x14 - AInterrupt mask register2"] pub aim: Aim,
    #[doc = "0x18 - AStatus register"] pub asr: Asr,
    #[doc = "0x1c - AClear flag register"] pub aclrfr: Aclrfr,
    #[doc = "0x20 - AData register"] pub adr: Adr,
    #[doc = "0x24 - BConfiguration register 1"] pub bcr1: Bcr1,
    #[doc = "0x28 - BConfiguration register 2"] pub bcr2: Bcr2,
    #[doc = "0x2c - BFRCR"] pub bfrcr: Bfrcr,
    #[doc = "0x30 - BSlot register"] pub bslotr: Bslotr,
    #[doc = "0x34 - BInterrupt mask register2"] pub bim: Bim,
    #[doc = "0x38 - BStatus register"] pub bsr: Bsr,
    #[doc = "0x3c - BClear flag register"] pub bclrfr: Bclrfr,
    #[doc = "0x40 - BData register"] pub bdr: Bdr,
}
#[doc = "BConfiguration register 1"]
pub struct Bcr1 {
    register: VolatileCell<u32>,
}
#[doc = "BConfiguration register 1"]
pub mod bcr1;
#[doc = "BConfiguration register 2"]
pub struct Bcr2 {
    register: VolatileCell<u32>,
}
#[doc = "BConfiguration register 2"]
pub mod bcr2;
#[doc = "BFRCR"]
pub struct Bfrcr {
    register: VolatileCell<u32>,
}
#[doc = "BFRCR"]
pub mod bfrcr;
#[doc = "BSlot register"]
pub struct Bslotr {
    register: VolatileCell<u32>,
}
#[doc = "BSlot register"]
pub mod bslotr;
#[doc = "BInterrupt mask register2"]
pub struct Bim {
    register: VolatileCell<u32>,
}
#[doc = "BInterrupt mask register2"]
pub mod bim;
#[doc = "BStatus register"]
pub struct Bsr {
    register: VolatileCell<u32>,
}
#[doc = "BStatus register"]
pub mod bsr;
#[doc = "BClear flag register"]
pub struct Bclrfr {
    register: VolatileCell<u32>,
}
#[doc = "BClear flag register"]
pub mod bclrfr;
#[doc = "BData register"]
pub struct Bdr {
    register: VolatileCell<u32>,
}
#[doc = "BData register"]
pub mod bdr;
#[doc = "AConfiguration register 1"]
pub struct Acr1 {
    register: VolatileCell<u32>,
}
#[doc = "AConfiguration register 1"]
pub mod acr1;
#[doc = "AConfiguration register 2"]
pub struct Acr2 {
    register: VolatileCell<u32>,
}
#[doc = "AConfiguration register 2"]
pub mod acr2;
#[doc = "AFRCR"]
pub struct Afrcr {
    register: VolatileCell<u32>,
}
#[doc = "AFRCR"]
pub mod afrcr;
#[doc = "ASlot register"]
pub struct Aslotr {
    register: VolatileCell<u32>,
}
#[doc = "ASlot register"]
pub mod aslotr;
#[doc = "AInterrupt mask register2"]
pub struct Aim {
    register: VolatileCell<u32>,
}
#[doc = "AInterrupt mask register2"]
pub mod aim;
#[doc = "AStatus register"]
pub struct Asr {
    register: VolatileCell<u32>,
}
#[doc = "AStatus register"]
pub mod asr;
#[doc = "AClear flag register"]
pub struct Aclrfr {
    register: VolatileCell<u32>,
}
#[doc = "AClear flag register"]
pub mod aclrfr;
#[doc = "AData register"]
pub struct Adr {
    register: VolatileCell<u32>,
}
#[doc = "AData register"]
pub mod adr;
