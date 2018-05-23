use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"] pub cr1: Cr1,
    _reserved0: [u8; 8usize],
    #[doc = "0x0c - DMA/Interrupt enable register"] pub dier: Dier,
    #[doc = "0x10 - status register"] pub sr: Sr,
    #[doc = "0x14 - event generation register"] pub egr: Egr,
    #[doc = "0x18 - capture/compare mode register 1 (output mode)"] pub ccmr1_output: Ccmr1Output,
    _reserved1: [u8; 4usize],
    #[doc = "0x20 - capture/compare enable register"] pub ccer: Ccer,
    #[doc = "0x24 - counter"] pub cnt: Cnt,
    #[doc = "0x28 - prescaler"] pub psc: Psc,
    #[doc = "0x2c - auto-reload register"] pub arr: Arr,
    _reserved2: [u8; 4usize],
    #[doc = "0x34 - capture/compare register 1"] pub ccr1: Ccr1,
    _reserved3: [u8; 24usize],
    #[doc = "0x50 - option register"] pub or: Or,
}
#[doc = "control register 1"]
pub struct Cr1 {
    register: VolatileCell<u32>,
}
#[doc = "control register 1"]
pub mod cr1;
#[doc = "DMA/Interrupt enable register"]
pub struct Dier {
    register: VolatileCell<u32>,
}
#[doc = "DMA/Interrupt enable register"]
pub mod dier;
#[doc = "status register"]
pub struct Sr {
    register: VolatileCell<u32>,
}
#[doc = "status register"]
pub mod sr;
#[doc = "event generation register"]
pub struct Egr {
    register: VolatileCell<u32>,
}
#[doc = "event generation register"]
pub mod egr;
#[doc = "capture/compare mode register 1 (output mode)"]
pub struct Ccmr1Output {
    register: VolatileCell<u32>,
}
#[doc = "capture/compare mode register 1 (output mode)"]
pub mod ccmr1_output;
#[doc = "capture/compare mode register 1 (input mode)"]
pub struct Ccmr1Input {
    register: VolatileCell<u32>,
}
#[doc = "capture/compare mode register 1 (input mode)"]
pub mod ccmr1_input;
#[doc = "capture/compare enable register"]
pub struct Ccer {
    register: VolatileCell<u32>,
}
#[doc = "capture/compare enable register"]
pub mod ccer;
#[doc = "counter"]
pub struct Cnt {
    register: VolatileCell<u32>,
}
#[doc = "counter"]
pub mod cnt;
#[doc = "prescaler"]
pub struct Psc {
    register: VolatileCell<u32>,
}
#[doc = "prescaler"]
pub mod psc;
#[doc = "auto-reload register"]
pub struct Arr {
    register: VolatileCell<u32>,
}
#[doc = "auto-reload register"]
pub mod arr;
#[doc = "capture/compare register 1"]
pub struct Ccr1 {
    register: VolatileCell<u32>,
}
#[doc = "capture/compare register 1"]
pub mod ccr1;
#[doc = "option register"]
pub struct Or {
    register: VolatileCell<u32>,
}
#[doc = "option register"]
pub mod or;
