use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IDCODE"] pub dbgmcu_idcode: DbgmcuIdcode,
    #[doc = "0x04 - Control Register"] pub dbgmcu_cr: DbgmcuCr,
    #[doc = "0x08 - Debug MCU APB1 Freeze registe"] pub dbgmcu_apb1_fz: DbgmcuApb1Fz,
    #[doc = "0x0c - Debug MCU APB2 Freeze registe"] pub dbgmcu_apb2_fz: DbgmcuApb2Fz,
}
#[doc = "IDCODE"]
pub struct DbgmcuIdcode {
    register: VolatileCell<u32>,
}
#[doc = "IDCODE"]
pub mod dbgmcu_idcode;
#[doc = "Control Register"]
pub struct DbgmcuCr {
    register: VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod dbgmcu_cr;
#[doc = "Debug MCU APB1 Freeze registe"]
pub struct DbgmcuApb1Fz {
    register: VolatileCell<u32>,
}
#[doc = "Debug MCU APB1 Freeze registe"]
pub mod dbgmcu_apb1_fz;
#[doc = "Debug MCU APB2 Freeze registe"]
pub struct DbgmcuApb2Fz {
    register: VolatileCell<u32>,
}
#[doc = "Debug MCU APB2 Freeze registe"]
pub mod dbgmcu_apb2_fz;
