use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRAM/NOR-Flash chip-select control register 1"] pub bcr1: Bcr1,
    #[doc = "0x04 - SRAM/NOR-Flash chip-select timing register 1"] pub btr1: Btr1,
    #[doc = "0x08 - SRAM/NOR-Flash chip-select control register 2"] pub bcr2: Bcr2,
    #[doc = "0x0c - SRAM/NOR-Flash chip-select timing register 2"] pub btr2: Btr2,
    #[doc = "0x10 - SRAM/NOR-Flash chip-select control register 3"] pub bcr3: Bcr3,
    #[doc = "0x14 - SRAM/NOR-Flash chip-select timing register 3"] pub btr3: Btr3,
    #[doc = "0x18 - SRAM/NOR-Flash chip-select control register 4"] pub bcr4: Bcr4,
    #[doc = "0x1c - SRAM/NOR-Flash chip-select timing register 4"] pub btr4: Btr4,
    _reserved0: [u8; 64usize],
    #[doc = "0x60 - PC Card/NAND Flash control register 2"] pub pcr2: Pcr2,
    #[doc = "0x64 - FIFO status and interrupt register 2"] pub sr2: Sr2,
    #[doc = "0x68 - Common memory space timing register 2"] pub pmem2: Pmem2,
    #[doc = "0x6c - Attribute memory space timing register 2"] pub patt2: Patt2,
    _reserved1: [u8; 4usize],
    #[doc = "0x74 - ECC result register 2"] pub eccr2: Eccr2,
    _reserved2: [u8; 8usize],
    #[doc = "0x80 - PC Card/NAND Flash control register 3"] pub pcr3: Pcr3,
    #[doc = "0x84 - FIFO status and interrupt register 3"] pub sr3: Sr3,
    #[doc = "0x88 - Common memory space timing register 3"] pub pmem3: Pmem3,
    #[doc = "0x8c - Attribute memory space timing register 3"] pub patt3: Patt3,
    _reserved3: [u8; 4usize],
    #[doc = "0x94 - ECC result register 3"] pub eccr3: Eccr3,
    _reserved4: [u8; 8usize],
    #[doc = "0xa0 - PC Card/NAND Flash control register 4"] pub pcr4: Pcr4,
    #[doc = "0xa4 - FIFO status and interrupt register 4"] pub sr4: Sr4,
    #[doc = "0xa8 - Common memory space timing register 4"] pub pmem4: Pmem4,
    #[doc = "0xac - Attribute memory space timing register 4"] pub patt4: Patt4,
    #[doc = "0xb0 - I/O space timing register 4"] pub pio4: Pio4,
    _reserved5: [u8; 80usize],
    #[doc = "0x104 - SRAM/NOR-Flash write timing registers 1"] pub bwtr1: Bwtr1,
    _reserved6: [u8; 4usize],
    #[doc = "0x10c - SRAM/NOR-Flash write timing registers 2"] pub bwtr2: Bwtr2,
    _reserved7: [u8; 4usize],
    #[doc = "0x114 - SRAM/NOR-Flash write timing registers 3"] pub bwtr3: Bwtr3,
    _reserved8: [u8; 4usize],
    #[doc = "0x11c - SRAM/NOR-Flash write timing registers 4"] pub bwtr4: Bwtr4,
    _reserved9: [u8; 32usize],
    #[doc = "0x140 - SDRAM Control Register 1"] pub sdcr1: Sdcr1,
    #[doc = "0x144 - SDRAM Control Register 2"] pub sdcr2: Sdcr2,
    #[doc = "0x148 - SDRAM Timing register 1"] pub sdtr1: Sdtr1,
    #[doc = "0x14c - SDRAM Timing register 2"] pub sdtr2: Sdtr2,
    #[doc = "0x150 - SDRAM Command Mode register"] pub sdcmr: Sdcmr,
    #[doc = "0x154 - SDRAM Refresh Timer register"] pub sdrtr: Sdrtr,
    #[doc = "0x158 - SDRAM Status register"] pub sdsr: Sdsr,
}
#[doc = "SRAM/NOR-Flash chip-select control register 1"]
pub struct Bcr1 {
    register: VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select control register 1"]
pub mod bcr1;
#[doc = "SRAM/NOR-Flash chip-select timing register 1"]
pub struct Btr1 {
    register: VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select timing register 1"]
pub mod btr1;
#[doc = "SRAM/NOR-Flash chip-select control register 2"]
pub struct Bcr2 {
    register: VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select control register 2"]
pub mod bcr2;
#[doc = "SRAM/NOR-Flash chip-select timing register 2"]
pub struct Btr2 {
    register: VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select timing register 2"]
pub mod btr2;
#[doc = "SRAM/NOR-Flash chip-select control register 3"]
pub struct Bcr3 {
    register: VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select control register 3"]
pub mod bcr3;
#[doc = "SRAM/NOR-Flash chip-select timing register 3"]
pub struct Btr3 {
    register: VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select timing register 3"]
pub mod btr3;
#[doc = "SRAM/NOR-Flash chip-select control register 4"]
pub struct Bcr4 {
    register: VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select control register 4"]
pub mod bcr4;
#[doc = "SRAM/NOR-Flash chip-select timing register 4"]
pub struct Btr4 {
    register: VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select timing register 4"]
pub mod btr4;
#[doc = "PC Card/NAND Flash control register 2"]
pub struct Pcr2 {
    register: VolatileCell<u32>,
}
#[doc = "PC Card/NAND Flash control register 2"]
pub mod pcr2;
#[doc = "FIFO status and interrupt register 2"]
pub struct Sr2 {
    register: VolatileCell<u32>,
}
#[doc = "FIFO status and interrupt register 2"]
pub mod sr2;
#[doc = "Common memory space timing register 2"]
pub struct Pmem2 {
    register: VolatileCell<u32>,
}
#[doc = "Common memory space timing register 2"]
pub mod pmem2;
#[doc = "Attribute memory space timing register 2"]
pub struct Patt2 {
    register: VolatileCell<u32>,
}
#[doc = "Attribute memory space timing register 2"]
pub mod patt2;
#[doc = "ECC result register 2"]
pub struct Eccr2 {
    register: VolatileCell<u32>,
}
#[doc = "ECC result register 2"]
pub mod eccr2;
#[doc = "PC Card/NAND Flash control register 3"]
pub struct Pcr3 {
    register: VolatileCell<u32>,
}
#[doc = "PC Card/NAND Flash control register 3"]
pub mod pcr3;
#[doc = "FIFO status and interrupt register 3"]
pub struct Sr3 {
    register: VolatileCell<u32>,
}
#[doc = "FIFO status and interrupt register 3"]
pub mod sr3;
#[doc = "Common memory space timing register 3"]
pub struct Pmem3 {
    register: VolatileCell<u32>,
}
#[doc = "Common memory space timing register 3"]
pub mod pmem3;
#[doc = "Attribute memory space timing register 3"]
pub struct Patt3 {
    register: VolatileCell<u32>,
}
#[doc = "Attribute memory space timing register 3"]
pub mod patt3;
#[doc = "ECC result register 3"]
pub struct Eccr3 {
    register: VolatileCell<u32>,
}
#[doc = "ECC result register 3"]
pub mod eccr3;
#[doc = "PC Card/NAND Flash control register 4"]
pub struct Pcr4 {
    register: VolatileCell<u32>,
}
#[doc = "PC Card/NAND Flash control register 4"]
pub mod pcr4;
#[doc = "FIFO status and interrupt register 4"]
pub struct Sr4 {
    register: VolatileCell<u32>,
}
#[doc = "FIFO status and interrupt register 4"]
pub mod sr4;
#[doc = "Common memory space timing register 4"]
pub struct Pmem4 {
    register: VolatileCell<u32>,
}
#[doc = "Common memory space timing register 4"]
pub mod pmem4;
#[doc = "Attribute memory space timing register 4"]
pub struct Patt4 {
    register: VolatileCell<u32>,
}
#[doc = "Attribute memory space timing register 4"]
pub mod patt4;
#[doc = "I/O space timing register 4"]
pub struct Pio4 {
    register: VolatileCell<u32>,
}
#[doc = "I/O space timing register 4"]
pub mod pio4;
#[doc = "SRAM/NOR-Flash write timing registers 1"]
pub struct Bwtr1 {
    register: VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash write timing registers 1"]
pub mod bwtr1;
#[doc = "SRAM/NOR-Flash write timing registers 2"]
pub struct Bwtr2 {
    register: VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash write timing registers 2"]
pub mod bwtr2;
#[doc = "SRAM/NOR-Flash write timing registers 3"]
pub struct Bwtr3 {
    register: VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash write timing registers 3"]
pub mod bwtr3;
#[doc = "SRAM/NOR-Flash write timing registers 4"]
pub struct Bwtr4 {
    register: VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash write timing registers 4"]
pub mod bwtr4;
#[doc = "SDRAM Control Register 1"]
pub struct Sdcr1 {
    register: VolatileCell<u32>,
}
#[doc = "SDRAM Control Register 1"]
pub mod sdcr1;
#[doc = "SDRAM Control Register 2"]
pub struct Sdcr2 {
    register: VolatileCell<u32>,
}
#[doc = "SDRAM Control Register 2"]
pub mod sdcr2;
#[doc = "SDRAM Timing register 1"]
pub struct Sdtr1 {
    register: VolatileCell<u32>,
}
#[doc = "SDRAM Timing register 1"]
pub mod sdtr1;
#[doc = "SDRAM Timing register 2"]
pub struct Sdtr2 {
    register: VolatileCell<u32>,
}
#[doc = "SDRAM Timing register 2"]
pub mod sdtr2;
#[doc = "SDRAM Command Mode register"]
pub struct Sdcmr {
    register: VolatileCell<u32>,
}
#[doc = "SDRAM Command Mode register"]
pub mod sdcmr;
#[doc = "SDRAM Refresh Timer register"]
pub struct Sdrtr {
    register: VolatileCell<u32>,
}
#[doc = "SDRAM Refresh Timer register"]
pub mod sdrtr;
#[doc = "SDRAM Status register"]
pub struct Sdsr {
    register: VolatileCell<u32>,
}
#[doc = "SDRAM Status register"]
pub mod sdsr;
