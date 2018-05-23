use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power and clock gating control register"] pub otg_hs_pcgcr: OtgHsPcgcr,
}
#[doc = "Power and clock gating control register"]
pub struct OtgHsPcgcr {
    register: VolatileCell<u32>,
}
#[doc = "Power and clock gating control register"]
pub mod otg_hs_pcgcr;
