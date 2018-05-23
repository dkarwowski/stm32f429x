use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    #[doc = "0x08 - Synchronization Size Configuration Register"] pub sscr: Sscr,
    #[doc = "0x0c - Back Porch Configuration Register"] pub bpcr: Bpcr,
    #[doc = "0x10 - Active Width Configuration Register"] pub awcr: Awcr,
    #[doc = "0x14 - Total Width Configuration Register"] pub twcr: Twcr,
    #[doc = "0x18 - Global Control Register"] pub gcr: Gcr,
    _reserved1: [u8; 8usize],
    #[doc = "0x24 - Shadow Reload Configuration Register"] pub srcr: Srcr,
    _reserved2: [u8; 4usize],
    #[doc = "0x2c - Background Color Configuration Register"] pub bccr: Bccr,
    _reserved3: [u8; 4usize],
    #[doc = "0x34 - Interrupt Enable Register"] pub ier: Ier,
    #[doc = "0x38 - Interrupt Status Register"] pub isr: Isr,
    #[doc = "0x3c - Interrupt Clear Register"] pub icr: Icr,
    #[doc = "0x40 - Line Interrupt Position Configuration Register"] pub lipcr: Lipcr,
    #[doc = "0x44 - Current Position Status Register"] pub cpsr: Cpsr,
    #[doc = "0x48 - Current Display Status Register"] pub cdsr: Cdsr,
    _reserved4: [u8; 56usize],
    #[doc = "0x84 - Layerx Control Register"] pub l1cr: L1cr,
    #[doc = "0x88 - Layerx Window Horizontal Position Configuration Register"] pub l1whpcr: L1whpcr,
    #[doc = "0x8c - Layerx Window Vertical Position Configuration Register"] pub l1wvpcr: L1wvpcr,
    #[doc = "0x90 - Layerx Color Keying Configuration Register"] pub l1ckcr: L1ckcr,
    #[doc = "0x94 - Layerx Pixel Format Configuration Register"] pub l1pfcr: L1pfcr,
    #[doc = "0x98 - Layerx Constant Alpha Configuration Register"] pub l1cacr: L1cacr,
    #[doc = "0x9c - Layerx Default Color Configuration Register"] pub l1dccr: L1dccr,
    #[doc = "0xa0 - Layerx Blending Factors Configuration Register"] pub l1bfcr: L1bfcr,
    _reserved5: [u8; 8usize],
    #[doc = "0xac - Layerx Color Frame Buffer Address Register"] pub l1cfbar: L1cfbar,
    #[doc = "0xb0 - Layerx Color Frame Buffer Length Register"] pub l1cfblr: L1cfblr,
    #[doc = "0xb4 - Layerx ColorFrame Buffer Line Number Register"] pub l1cfblnr: L1cfblnr,
    _reserved6: [u8; 12usize],
    #[doc = "0xc4 - Layerx CLUT Write Register"] pub l1clutwr: L1clutwr,
    _reserved7: [u8; 60usize],
    #[doc = "0x104 - Layerx Control Register"] pub l2cr: L2cr,
    #[doc = "0x108 - Layerx Window Horizontal Position Configuration Register"]
    pub l2whpcr: L2whpcr,
    #[doc = "0x10c - Layerx Window Vertical Position Configuration Register"] pub l2wvpcr: L2wvpcr,
    #[doc = "0x110 - Layerx Color Keying Configuration Register"] pub l2ckcr: L2ckcr,
    #[doc = "0x114 - Layerx Pixel Format Configuration Register"] pub l2pfcr: L2pfcr,
    #[doc = "0x118 - Layerx Constant Alpha Configuration Register"] pub l2cacr: L2cacr,
    #[doc = "0x11c - Layerx Default Color Configuration Register"] pub l2dccr: L2dccr,
    #[doc = "0x120 - Layerx Blending Factors Configuration Register"] pub l2bfcr: L2bfcr,
    _reserved8: [u8; 8usize],
    #[doc = "0x12c - Layerx Color Frame Buffer Address Register"] pub l2cfbar: L2cfbar,
    #[doc = "0x130 - Layerx Color Frame Buffer Length Register"] pub l2cfblr: L2cfblr,
    #[doc = "0x134 - Layerx ColorFrame Buffer Line Number Register"] pub l2cfblnr: L2cfblnr,
    _reserved9: [u8; 12usize],
    #[doc = "0x144 - Layerx CLUT Write Register"] pub l2clutwr: L2clutwr,
}
#[doc = "Synchronization Size Configuration Register"]
pub struct Sscr {
    register: VolatileCell<u32>,
}
#[doc = "Synchronization Size Configuration Register"]
pub mod sscr;
#[doc = "Back Porch Configuration Register"]
pub struct Bpcr {
    register: VolatileCell<u32>,
}
#[doc = "Back Porch Configuration Register"]
pub mod bpcr;
#[doc = "Active Width Configuration Register"]
pub struct Awcr {
    register: VolatileCell<u32>,
}
#[doc = "Active Width Configuration Register"]
pub mod awcr;
#[doc = "Total Width Configuration Register"]
pub struct Twcr {
    register: VolatileCell<u32>,
}
#[doc = "Total Width Configuration Register"]
pub mod twcr;
#[doc = "Global Control Register"]
pub struct Gcr {
    register: VolatileCell<u32>,
}
#[doc = "Global Control Register"]
pub mod gcr;
#[doc = "Shadow Reload Configuration Register"]
pub struct Srcr {
    register: VolatileCell<u32>,
}
#[doc = "Shadow Reload Configuration Register"]
pub mod srcr;
#[doc = "Background Color Configuration Register"]
pub struct Bccr {
    register: VolatileCell<u32>,
}
#[doc = "Background Color Configuration Register"]
pub mod bccr;
#[doc = "Interrupt Enable Register"]
pub struct Ier {
    register: VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "Interrupt Status Register"]
pub struct Isr {
    register: VolatileCell<u32>,
}
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "Interrupt Clear Register"]
pub struct Icr {
    register: VolatileCell<u32>,
}
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "Line Interrupt Position Configuration Register"]
pub struct Lipcr {
    register: VolatileCell<u32>,
}
#[doc = "Line Interrupt Position Configuration Register"]
pub mod lipcr;
#[doc = "Current Position Status Register"]
pub struct Cpsr {
    register: VolatileCell<u32>,
}
#[doc = "Current Position Status Register"]
pub mod cpsr;
#[doc = "Current Display Status Register"]
pub struct Cdsr {
    register: VolatileCell<u32>,
}
#[doc = "Current Display Status Register"]
pub mod cdsr;
#[doc = "Layerx Control Register"]
pub struct L1cr {
    register: VolatileCell<u32>,
}
#[doc = "Layerx Control Register"]
pub mod l1cr;
#[doc = "Layerx Window Horizontal Position Configuration Register"]
pub struct L1whpcr {
    register: VolatileCell<u32>,
}
#[doc = "Layerx Window Horizontal Position Configuration Register"]
pub mod l1whpcr;
#[doc = "Layerx Window Vertical Position Configuration Register"]
pub struct L1wvpcr {
    register: VolatileCell<u32>,
}
#[doc = "Layerx Window Vertical Position Configuration Register"]
pub mod l1wvpcr;
#[doc = "Layerx Color Keying Configuration Register"]
pub struct L1ckcr {
    register: VolatileCell<u32>,
}
#[doc = "Layerx Color Keying Configuration Register"]
pub mod l1ckcr;
#[doc = "Layerx Pixel Format Configuration Register"]
pub struct L1pfcr {
    register: VolatileCell<u32>,
}
#[doc = "Layerx Pixel Format Configuration Register"]
pub mod l1pfcr;
#[doc = "Layerx Constant Alpha Configuration Register"]
pub struct L1cacr {
    register: VolatileCell<u32>,
}
#[doc = "Layerx Constant Alpha Configuration Register"]
pub mod l1cacr;
#[doc = "Layerx Default Color Configuration Register"]
pub struct L1dccr {
    register: VolatileCell<u32>,
}
#[doc = "Layerx Default Color Configuration Register"]
pub mod l1dccr;
#[doc = "Layerx Blending Factors Configuration Register"]
pub struct L1bfcr {
    register: VolatileCell<u32>,
}
#[doc = "Layerx Blending Factors Configuration Register"]
pub mod l1bfcr;
#[doc = "Layerx Color Frame Buffer Address Register"]
pub struct L1cfbar {
    register: VolatileCell<u32>,
}
#[doc = "Layerx Color Frame Buffer Address Register"]
pub mod l1cfbar;
#[doc = "Layerx Color Frame Buffer Length Register"]
pub struct L1cfblr {
    register: VolatileCell<u32>,
}
#[doc = "Layerx Color Frame Buffer Length Register"]
pub mod l1cfblr;
#[doc = "Layerx ColorFrame Buffer Line Number Register"]
pub struct L1cfblnr {
    register: VolatileCell<u32>,
}
#[doc = "Layerx ColorFrame Buffer Line Number Register"]
pub mod l1cfblnr;
#[doc = "Layerx CLUT Write Register"]
pub struct L1clutwr {
    register: VolatileCell<u32>,
}
#[doc = "Layerx CLUT Write Register"]
pub mod l1clutwr;
#[doc = "Layerx Control Register"]
pub struct L2cr {
    register: VolatileCell<u32>,
}
#[doc = "Layerx Control Register"]
pub mod l2cr;
#[doc = "Layerx Window Horizontal Position Configuration Register"]
pub struct L2whpcr {
    register: VolatileCell<u32>,
}
#[doc = "Layerx Window Horizontal Position Configuration Register"]
pub mod l2whpcr;
#[doc = "Layerx Window Vertical Position Configuration Register"]
pub struct L2wvpcr {
    register: VolatileCell<u32>,
}
#[doc = "Layerx Window Vertical Position Configuration Register"]
pub mod l2wvpcr;
#[doc = "Layerx Color Keying Configuration Register"]
pub struct L2ckcr {
    register: VolatileCell<u32>,
}
#[doc = "Layerx Color Keying Configuration Register"]
pub mod l2ckcr;
#[doc = "Layerx Pixel Format Configuration Register"]
pub struct L2pfcr {
    register: VolatileCell<u32>,
}
#[doc = "Layerx Pixel Format Configuration Register"]
pub mod l2pfcr;
#[doc = "Layerx Constant Alpha Configuration Register"]
pub struct L2cacr {
    register: VolatileCell<u32>,
}
#[doc = "Layerx Constant Alpha Configuration Register"]
pub mod l2cacr;
#[doc = "Layerx Default Color Configuration Register"]
pub struct L2dccr {
    register: VolatileCell<u32>,
}
#[doc = "Layerx Default Color Configuration Register"]
pub mod l2dccr;
#[doc = "Layerx Blending Factors Configuration Register"]
pub struct L2bfcr {
    register: VolatileCell<u32>,
}
#[doc = "Layerx Blending Factors Configuration Register"]
pub mod l2bfcr;
#[doc = "Layerx Color Frame Buffer Address Register"]
pub struct L2cfbar {
    register: VolatileCell<u32>,
}
#[doc = "Layerx Color Frame Buffer Address Register"]
pub mod l2cfbar;
#[doc = "Layerx Color Frame Buffer Length Register"]
pub struct L2cfblr {
    register: VolatileCell<u32>,
}
#[doc = "Layerx Color Frame Buffer Length Register"]
pub mod l2cfblr;
#[doc = "Layerx ColorFrame Buffer Line Number Register"]
pub struct L2cfblnr {
    register: VolatileCell<u32>,
}
#[doc = "Layerx ColorFrame Buffer Line Number Register"]
pub mod l2cfblnr;
#[doc = "Layerx CLUT Write Register"]
pub struct L2clutwr {
    register: VolatileCell<u32>,
}
#[doc = "Layerx CLUT Write Register"]
pub mod l2clutwr;
