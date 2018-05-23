use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - master control register"] pub mcr: Mcr,
    #[doc = "0x04 - master status register"] pub msr: Msr,
    #[doc = "0x08 - transmit status register"] pub tsr: Tsr,
    #[doc = "0x0c - receive FIFO 0 register"] pub rf0r: Rf0r,
    #[doc = "0x10 - receive FIFO 1 register"] pub rf1r: Rf1r,
    #[doc = "0x14 - interrupt enable register"] pub ier: Ier,
    #[doc = "0x18 - interrupt enable register"] pub esr: Esr,
    #[doc = "0x1c - bit timing register"] pub btr: Btr,
    _reserved0: [u8; 352usize],
    #[doc = "0x180 - TX mailbox identifier register"] pub ti0r: Ti0r,
    #[doc = "0x184 - mailbox data length control and time stamp register"] pub tdt0r: Tdt0r,
    #[doc = "0x188 - mailbox data low register"] pub tdl0r: Tdl0r,
    #[doc = "0x18c - mailbox data high register"] pub tdh0r: Tdh0r,
    #[doc = "0x190 - mailbox identifier register"] pub ti1r: Ti1r,
    #[doc = "0x194 - mailbox data length control and time stamp register"] pub tdt1r: Tdt1r,
    #[doc = "0x198 - mailbox data low register"] pub tdl1r: Tdl1r,
    #[doc = "0x19c - mailbox data high register"] pub tdh1r: Tdh1r,
    #[doc = "0x1a0 - mailbox identifier register"] pub ti2r: Ti2r,
    #[doc = "0x1a4 - mailbox data length control and time stamp register"] pub tdt2r: Tdt2r,
    #[doc = "0x1a8 - mailbox data low register"] pub tdl2r: Tdl2r,
    #[doc = "0x1ac - mailbox data high register"] pub tdh2r: Tdh2r,
    #[doc = "0x1b0 - receive FIFO mailbox identifier register"] pub ri0r: Ri0r,
    #[doc = "0x1b4 - mailbox data high register"] pub rdt0r: Rdt0r,
    #[doc = "0x1b8 - mailbox data high register"] pub rdl0r: Rdl0r,
    #[doc = "0x1bc - receive FIFO mailbox data high register"] pub rdh0r: Rdh0r,
    #[doc = "0x1c0 - mailbox data high register"] pub ri1r: Ri1r,
    #[doc = "0x1c4 - mailbox data high register"] pub rdt1r: Rdt1r,
    #[doc = "0x1c8 - mailbox data high register"] pub rdl1r: Rdl1r,
    #[doc = "0x1cc - mailbox data high register"] pub rdh1r: Rdh1r,
    _reserved1: [u8; 48usize],
    #[doc = "0x200 - filter master register"] pub fmr: Fmr,
    #[doc = "0x204 - filter mode register"] pub fm1r: Fm1r,
    _reserved2: [u8; 4usize],
    #[doc = "0x20c - filter scale register"] pub fs1r: Fs1r,
    _reserved3: [u8; 4usize],
    #[doc = "0x214 - filter FIFO assignment register"] pub ffa1r: Ffa1r,
    _reserved4: [u8; 4usize],
    #[doc = "0x21c - filter activation register"] pub fa1r: Fa1r,
    _reserved5: [u8; 32usize],
    #[doc = "0x240 - Filter bank 0 register 1"] pub f0r1: F0r1,
    #[doc = "0x244 - Filter bank 0 register 2"] pub f0r2: F0r2,
    #[doc = "0x248 - Filter bank 1 register 1"] pub f1r1: F1r1,
    #[doc = "0x24c - Filter bank 1 register 2"] pub f1r2: F1r2,
    #[doc = "0x250 - Filter bank 2 register 1"] pub f2r1: F2r1,
    #[doc = "0x254 - Filter bank 2 register 2"] pub f2r2: F2r2,
    #[doc = "0x258 - Filter bank 3 register 1"] pub f3r1: F3r1,
    #[doc = "0x25c - Filter bank 3 register 2"] pub f3r2: F3r2,
    #[doc = "0x260 - Filter bank 4 register 1"] pub f4r1: F4r1,
    #[doc = "0x264 - Filter bank 4 register 2"] pub f4r2: F4r2,
    #[doc = "0x268 - Filter bank 5 register 1"] pub f5r1: F5r1,
    #[doc = "0x26c - Filter bank 5 register 2"] pub f5r2: F5r2,
    #[doc = "0x270 - Filter bank 6 register 1"] pub f6r1: F6r1,
    #[doc = "0x274 - Filter bank 6 register 2"] pub f6r2: F6r2,
    #[doc = "0x278 - Filter bank 7 register 1"] pub f7r1: F7r1,
    #[doc = "0x27c - Filter bank 7 register 2"] pub f7r2: F7r2,
    #[doc = "0x280 - Filter bank 8 register 1"] pub f8r1: F8r1,
    #[doc = "0x284 - Filter bank 8 register 2"] pub f8r2: F8r2,
    #[doc = "0x288 - Filter bank 9 register 1"] pub f9r1: F9r1,
    #[doc = "0x28c - Filter bank 9 register 2"] pub f9r2: F9r2,
    #[doc = "0x290 - Filter bank 10 register 1"] pub f10r1: F10r1,
    #[doc = "0x294 - Filter bank 10 register 2"] pub f10r2: F10r2,
    #[doc = "0x298 - Filter bank 11 register 1"] pub f11r1: F11r1,
    #[doc = "0x29c - Filter bank 11 register 2"] pub f11r2: F11r2,
    #[doc = "0x2a0 - Filter bank 4 register 1"] pub f12r1: F12r1,
    #[doc = "0x2a4 - Filter bank 12 register 2"] pub f12r2: F12r2,
    #[doc = "0x2a8 - Filter bank 13 register 1"] pub f13r1: F13r1,
    #[doc = "0x2ac - Filter bank 13 register 2"] pub f13r2: F13r2,
    #[doc = "0x2b0 - Filter bank 14 register 1"] pub f14r1: F14r1,
    #[doc = "0x2b4 - Filter bank 14 register 2"] pub f14r2: F14r2,
    #[doc = "0x2b8 - Filter bank 15 register 1"] pub f15r1: F15r1,
    #[doc = "0x2bc - Filter bank 15 register 2"] pub f15r2: F15r2,
    #[doc = "0x2c0 - Filter bank 16 register 1"] pub f16r1: F16r1,
    #[doc = "0x2c4 - Filter bank 16 register 2"] pub f16r2: F16r2,
    #[doc = "0x2c8 - Filter bank 17 register 1"] pub f17r1: F17r1,
    #[doc = "0x2cc - Filter bank 17 register 2"] pub f17r2: F17r2,
    #[doc = "0x2d0 - Filter bank 18 register 1"] pub f18r1: F18r1,
    #[doc = "0x2d4 - Filter bank 18 register 2"] pub f18r2: F18r2,
    #[doc = "0x2d8 - Filter bank 19 register 1"] pub f19r1: F19r1,
    #[doc = "0x2dc - Filter bank 19 register 2"] pub f19r2: F19r2,
    #[doc = "0x2e0 - Filter bank 20 register 1"] pub f20r1: F20r1,
    #[doc = "0x2e4 - Filter bank 20 register 2"] pub f20r2: F20r2,
    #[doc = "0x2e8 - Filter bank 21 register 1"] pub f21r1: F21r1,
    #[doc = "0x2ec - Filter bank 21 register 2"] pub f21r2: F21r2,
    #[doc = "0x2f0 - Filter bank 22 register 1"] pub f22r1: F22r1,
    #[doc = "0x2f4 - Filter bank 22 register 2"] pub f22r2: F22r2,
    #[doc = "0x2f8 - Filter bank 23 register 1"] pub f23r1: F23r1,
    #[doc = "0x2fc - Filter bank 23 register 2"] pub f23r2: F23r2,
    #[doc = "0x300 - Filter bank 24 register 1"] pub f24r1: F24r1,
    #[doc = "0x304 - Filter bank 24 register 2"] pub f24r2: F24r2,
    #[doc = "0x308 - Filter bank 25 register 1"] pub f25r1: F25r1,
    #[doc = "0x30c - Filter bank 25 register 2"] pub f25r2: F25r2,
    #[doc = "0x310 - Filter bank 26 register 1"] pub f26r1: F26r1,
    #[doc = "0x314 - Filter bank 26 register 2"] pub f26r2: F26r2,
    #[doc = "0x318 - Filter bank 27 register 1"] pub f27r1: F27r1,
    #[doc = "0x31c - Filter bank 27 register 2"] pub f27r2: F27r2,
}
#[doc = "master control register"]
pub struct Mcr {
    register: VolatileCell<u32>,
}
#[doc = "master control register"]
pub mod mcr;
#[doc = "master status register"]
pub struct Msr {
    register: VolatileCell<u32>,
}
#[doc = "master status register"]
pub mod msr;
#[doc = "transmit status register"]
pub struct Tsr {
    register: VolatileCell<u32>,
}
#[doc = "transmit status register"]
pub mod tsr;
#[doc = "receive FIFO 0 register"]
pub struct Rf0r {
    register: VolatileCell<u32>,
}
#[doc = "receive FIFO 0 register"]
pub mod rf0r;
#[doc = "receive FIFO 1 register"]
pub struct Rf1r {
    register: VolatileCell<u32>,
}
#[doc = "receive FIFO 1 register"]
pub mod rf1r;
#[doc = "interrupt enable register"]
pub struct Ier {
    register: VolatileCell<u32>,
}
#[doc = "interrupt enable register"]
pub mod ier;
#[doc = "interrupt enable register"]
pub struct Esr {
    register: VolatileCell<u32>,
}
#[doc = "interrupt enable register"]
pub mod esr;
#[doc = "bit timing register"]
pub struct Btr {
    register: VolatileCell<u32>,
}
#[doc = "bit timing register"]
pub mod btr;
#[doc = "TX mailbox identifier register"]
pub struct Ti0r {
    register: VolatileCell<u32>,
}
#[doc = "TX mailbox identifier register"]
pub mod ti0r;
#[doc = "mailbox data length control and time stamp register"]
pub struct Tdt0r {
    register: VolatileCell<u32>,
}
#[doc = "mailbox data length control and time stamp register"]
pub mod tdt0r;
#[doc = "mailbox data low register"]
pub struct Tdl0r {
    register: VolatileCell<u32>,
}
#[doc = "mailbox data low register"]
pub mod tdl0r;
#[doc = "mailbox data high register"]
pub struct Tdh0r {
    register: VolatileCell<u32>,
}
#[doc = "mailbox data high register"]
pub mod tdh0r;
#[doc = "mailbox identifier register"]
pub struct Ti1r {
    register: VolatileCell<u32>,
}
#[doc = "mailbox identifier register"]
pub mod ti1r;
#[doc = "mailbox data length control and time stamp register"]
pub struct Tdt1r {
    register: VolatileCell<u32>,
}
#[doc = "mailbox data length control and time stamp register"]
pub mod tdt1r;
#[doc = "mailbox data low register"]
pub struct Tdl1r {
    register: VolatileCell<u32>,
}
#[doc = "mailbox data low register"]
pub mod tdl1r;
#[doc = "mailbox data high register"]
pub struct Tdh1r {
    register: VolatileCell<u32>,
}
#[doc = "mailbox data high register"]
pub mod tdh1r;
#[doc = "mailbox identifier register"]
pub struct Ti2r {
    register: VolatileCell<u32>,
}
#[doc = "mailbox identifier register"]
pub mod ti2r;
#[doc = "mailbox data length control and time stamp register"]
pub struct Tdt2r {
    register: VolatileCell<u32>,
}
#[doc = "mailbox data length control and time stamp register"]
pub mod tdt2r;
#[doc = "mailbox data low register"]
pub struct Tdl2r {
    register: VolatileCell<u32>,
}
#[doc = "mailbox data low register"]
pub mod tdl2r;
#[doc = "mailbox data high register"]
pub struct Tdh2r {
    register: VolatileCell<u32>,
}
#[doc = "mailbox data high register"]
pub mod tdh2r;
#[doc = "receive FIFO mailbox identifier register"]
pub struct Ri0r {
    register: VolatileCell<u32>,
}
#[doc = "receive FIFO mailbox identifier register"]
pub mod ri0r;
#[doc = "mailbox data high register"]
pub struct Rdt0r {
    register: VolatileCell<u32>,
}
#[doc = "mailbox data high register"]
pub mod rdt0r;
#[doc = "mailbox data high register"]
pub struct Rdl0r {
    register: VolatileCell<u32>,
}
#[doc = "mailbox data high register"]
pub mod rdl0r;
#[doc = "receive FIFO mailbox data high register"]
pub struct Rdh0r {
    register: VolatileCell<u32>,
}
#[doc = "receive FIFO mailbox data high register"]
pub mod rdh0r;
#[doc = "mailbox data high register"]
pub struct Ri1r {
    register: VolatileCell<u32>,
}
#[doc = "mailbox data high register"]
pub mod ri1r;
#[doc = "mailbox data high register"]
pub struct Rdt1r {
    register: VolatileCell<u32>,
}
#[doc = "mailbox data high register"]
pub mod rdt1r;
#[doc = "mailbox data high register"]
pub struct Rdl1r {
    register: VolatileCell<u32>,
}
#[doc = "mailbox data high register"]
pub mod rdl1r;
#[doc = "mailbox data high register"]
pub struct Rdh1r {
    register: VolatileCell<u32>,
}
#[doc = "mailbox data high register"]
pub mod rdh1r;
#[doc = "filter master register"]
pub struct Fmr {
    register: VolatileCell<u32>,
}
#[doc = "filter master register"]
pub mod fmr;
#[doc = "filter mode register"]
pub struct Fm1r {
    register: VolatileCell<u32>,
}
#[doc = "filter mode register"]
pub mod fm1r;
#[doc = "filter scale register"]
pub struct Fs1r {
    register: VolatileCell<u32>,
}
#[doc = "filter scale register"]
pub mod fs1r;
#[doc = "filter FIFO assignment register"]
pub struct Ffa1r {
    register: VolatileCell<u32>,
}
#[doc = "filter FIFO assignment register"]
pub mod ffa1r;
#[doc = "filter activation register"]
pub struct Fa1r {
    register: VolatileCell<u32>,
}
#[doc = "filter activation register"]
pub mod fa1r;
#[doc = "Filter bank 0 register 1"]
pub struct F0r1 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 0 register 1"]
pub mod f0r1;
#[doc = "Filter bank 0 register 2"]
pub struct F0r2 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 0 register 2"]
pub mod f0r2;
#[doc = "Filter bank 1 register 1"]
pub struct F1r1 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 1 register 1"]
pub mod f1r1;
#[doc = "Filter bank 1 register 2"]
pub struct F1r2 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 1 register 2"]
pub mod f1r2;
#[doc = "Filter bank 2 register 1"]
pub struct F2r1 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 2 register 1"]
pub mod f2r1;
#[doc = "Filter bank 2 register 2"]
pub struct F2r2 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 2 register 2"]
pub mod f2r2;
#[doc = "Filter bank 3 register 1"]
pub struct F3r1 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 3 register 1"]
pub mod f3r1;
#[doc = "Filter bank 3 register 2"]
pub struct F3r2 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 3 register 2"]
pub mod f3r2;
#[doc = "Filter bank 4 register 1"]
pub struct F4r1 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 4 register 1"]
pub mod f4r1;
#[doc = "Filter bank 4 register 2"]
pub struct F4r2 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 4 register 2"]
pub mod f4r2;
#[doc = "Filter bank 5 register 1"]
pub struct F5r1 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 5 register 1"]
pub mod f5r1;
#[doc = "Filter bank 5 register 2"]
pub struct F5r2 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 5 register 2"]
pub mod f5r2;
#[doc = "Filter bank 6 register 1"]
pub struct F6r1 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 6 register 1"]
pub mod f6r1;
#[doc = "Filter bank 6 register 2"]
pub struct F6r2 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 6 register 2"]
pub mod f6r2;
#[doc = "Filter bank 7 register 1"]
pub struct F7r1 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 7 register 1"]
pub mod f7r1;
#[doc = "Filter bank 7 register 2"]
pub struct F7r2 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 7 register 2"]
pub mod f7r2;
#[doc = "Filter bank 8 register 1"]
pub struct F8r1 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 8 register 1"]
pub mod f8r1;
#[doc = "Filter bank 8 register 2"]
pub struct F8r2 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 8 register 2"]
pub mod f8r2;
#[doc = "Filter bank 9 register 1"]
pub struct F9r1 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 9 register 1"]
pub mod f9r1;
#[doc = "Filter bank 9 register 2"]
pub struct F9r2 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 9 register 2"]
pub mod f9r2;
#[doc = "Filter bank 10 register 1"]
pub struct F10r1 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 10 register 1"]
pub mod f10r1;
#[doc = "Filter bank 10 register 2"]
pub struct F10r2 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 10 register 2"]
pub mod f10r2;
#[doc = "Filter bank 11 register 1"]
pub struct F11r1 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 11 register 1"]
pub mod f11r1;
#[doc = "Filter bank 11 register 2"]
pub struct F11r2 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 11 register 2"]
pub mod f11r2;
#[doc = "Filter bank 4 register 1"]
pub struct F12r1 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 4 register 1"]
pub mod f12r1;
#[doc = "Filter bank 12 register 2"]
pub struct F12r2 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 12 register 2"]
pub mod f12r2;
#[doc = "Filter bank 13 register 1"]
pub struct F13r1 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 13 register 1"]
pub mod f13r1;
#[doc = "Filter bank 13 register 2"]
pub struct F13r2 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 13 register 2"]
pub mod f13r2;
#[doc = "Filter bank 14 register 1"]
pub struct F14r1 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 14 register 1"]
pub mod f14r1;
#[doc = "Filter bank 14 register 2"]
pub struct F14r2 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 14 register 2"]
pub mod f14r2;
#[doc = "Filter bank 15 register 1"]
pub struct F15r1 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 15 register 1"]
pub mod f15r1;
#[doc = "Filter bank 15 register 2"]
pub struct F15r2 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 15 register 2"]
pub mod f15r2;
#[doc = "Filter bank 16 register 1"]
pub struct F16r1 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 16 register 1"]
pub mod f16r1;
#[doc = "Filter bank 16 register 2"]
pub struct F16r2 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 16 register 2"]
pub mod f16r2;
#[doc = "Filter bank 17 register 1"]
pub struct F17r1 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 17 register 1"]
pub mod f17r1;
#[doc = "Filter bank 17 register 2"]
pub struct F17r2 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 17 register 2"]
pub mod f17r2;
#[doc = "Filter bank 18 register 1"]
pub struct F18r1 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 18 register 1"]
pub mod f18r1;
#[doc = "Filter bank 18 register 2"]
pub struct F18r2 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 18 register 2"]
pub mod f18r2;
#[doc = "Filter bank 19 register 1"]
pub struct F19r1 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 19 register 1"]
pub mod f19r1;
#[doc = "Filter bank 19 register 2"]
pub struct F19r2 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 19 register 2"]
pub mod f19r2;
#[doc = "Filter bank 20 register 1"]
pub struct F20r1 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 20 register 1"]
pub mod f20r1;
#[doc = "Filter bank 20 register 2"]
pub struct F20r2 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 20 register 2"]
pub mod f20r2;
#[doc = "Filter bank 21 register 1"]
pub struct F21r1 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 21 register 1"]
pub mod f21r1;
#[doc = "Filter bank 21 register 2"]
pub struct F21r2 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 21 register 2"]
pub mod f21r2;
#[doc = "Filter bank 22 register 1"]
pub struct F22r1 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 22 register 1"]
pub mod f22r1;
#[doc = "Filter bank 22 register 2"]
pub struct F22r2 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 22 register 2"]
pub mod f22r2;
#[doc = "Filter bank 23 register 1"]
pub struct F23r1 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 23 register 1"]
pub mod f23r1;
#[doc = "Filter bank 23 register 2"]
pub struct F23r2 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 23 register 2"]
pub mod f23r2;
#[doc = "Filter bank 24 register 1"]
pub struct F24r1 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 24 register 1"]
pub mod f24r1;
#[doc = "Filter bank 24 register 2"]
pub struct F24r2 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 24 register 2"]
pub mod f24r2;
#[doc = "Filter bank 25 register 1"]
pub struct F25r1 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 25 register 1"]
pub mod f25r1;
#[doc = "Filter bank 25 register 2"]
pub struct F25r2 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 25 register 2"]
pub mod f25r2;
#[doc = "Filter bank 26 register 1"]
pub struct F26r1 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 26 register 1"]
pub mod f26r1;
#[doc = "Filter bank 26 register 2"]
pub struct F26r2 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 26 register 2"]
pub mod f26r2;
#[doc = "Filter bank 27 register 1"]
pub struct F27r1 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 27 register 1"]
pub mod f27r1;
#[doc = "Filter bank 27 register 2"]
pub struct F27r2 {
    register: VolatileCell<u32>,
}
#[doc = "Filter bank 27 register 2"]
pub mod f27r2;
