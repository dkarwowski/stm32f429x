use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"] pub cr: Cr,
    #[doc = "0x04 - software trigger register"] pub swtrigr: Swtrigr,
    #[doc = "0x08 - channel1 12-bit right-aligned data holding register"] pub dhr12r1: Dhr12r1,
    #[doc = "0x0c - channel1 12-bit left aligned data holding register"] pub dhr12l1: Dhr12l1,
    #[doc = "0x10 - channel1 8-bit right aligned data holding register"] pub dhr8r1: Dhr8r1,
    #[doc = "0x14 - channel2 12-bit right aligned data holding register"] pub dhr12r2: Dhr12r2,
    #[doc = "0x18 - channel2 12-bit left aligned data holding register"] pub dhr12l2: Dhr12l2,
    #[doc = "0x1c - channel2 8-bit right-aligned data holding register"] pub dhr8r2: Dhr8r2,
    #[doc = "0x20 - Dual DAC 12-bit right-aligned data holding register"] pub dhr12rd: Dhr12rd,
    #[doc = "0x24 - DUAL DAC 12-bit left aligned data holding register"] pub dhr12ld: Dhr12ld,
    #[doc = "0x28 - DUAL DAC 8-bit right aligned data holding register"] pub dhr8rd: Dhr8rd,
    #[doc = "0x2c - channel1 data output register"] pub dor1: Dor1,
    #[doc = "0x30 - channel2 data output register"] pub dor2: Dor2,
    #[doc = "0x34 - status register"] pub sr: Sr,
}
#[doc = "control register"]
pub struct Cr {
    register: VolatileCell<u32>,
}
#[doc = "control register"]
pub mod cr;
#[doc = "software trigger register"]
pub struct Swtrigr {
    register: VolatileCell<u32>,
}
#[doc = "software trigger register"]
pub mod swtrigr;
#[doc = "channel1 12-bit right-aligned data holding register"]
pub struct Dhr12r1 {
    register: VolatileCell<u32>,
}
#[doc = "channel1 12-bit right-aligned data holding register"]
pub mod dhr12r1;
#[doc = "channel1 12-bit left aligned data holding register"]
pub struct Dhr12l1 {
    register: VolatileCell<u32>,
}
#[doc = "channel1 12-bit left aligned data holding register"]
pub mod dhr12l1;
#[doc = "channel1 8-bit right aligned data holding register"]
pub struct Dhr8r1 {
    register: VolatileCell<u32>,
}
#[doc = "channel1 8-bit right aligned data holding register"]
pub mod dhr8r1;
#[doc = "channel2 12-bit right aligned data holding register"]
pub struct Dhr12r2 {
    register: VolatileCell<u32>,
}
#[doc = "channel2 12-bit right aligned data holding register"]
pub mod dhr12r2;
#[doc = "channel2 12-bit left aligned data holding register"]
pub struct Dhr12l2 {
    register: VolatileCell<u32>,
}
#[doc = "channel2 12-bit left aligned data holding register"]
pub mod dhr12l2;
#[doc = "channel2 8-bit right-aligned data holding register"]
pub struct Dhr8r2 {
    register: VolatileCell<u32>,
}
#[doc = "channel2 8-bit right-aligned data holding register"]
pub mod dhr8r2;
#[doc = "Dual DAC 12-bit right-aligned data holding register"]
pub struct Dhr12rd {
    register: VolatileCell<u32>,
}
#[doc = "Dual DAC 12-bit right-aligned data holding register"]
pub mod dhr12rd;
#[doc = "DUAL DAC 12-bit left aligned data holding register"]
pub struct Dhr12ld {
    register: VolatileCell<u32>,
}
#[doc = "DUAL DAC 12-bit left aligned data holding register"]
pub mod dhr12ld;
#[doc = "DUAL DAC 8-bit right aligned data holding register"]
pub struct Dhr8rd {
    register: VolatileCell<u32>,
}
#[doc = "DUAL DAC 8-bit right aligned data holding register"]
pub mod dhr8rd;
#[doc = "channel1 data output register"]
pub struct Dor1 {
    register: VolatileCell<u32>,
}
#[doc = "channel1 data output register"]
pub mod dor1;
#[doc = "channel2 data output register"]
pub struct Dor2 {
    register: VolatileCell<u32>,
}
#[doc = "channel2 data output register"]
pub mod dor2;
#[doc = "status register"]
pub struct Sr {
    register: VolatileCell<u32>,
}
#[doc = "status register"]
pub mod sr;
