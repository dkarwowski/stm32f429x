use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - power control register"] pub power: Power,
    #[doc = "0x04 - SDI clock control register"] pub clkcr: Clkcr,
    #[doc = "0x08 - argument register"] pub arg: Arg,
    #[doc = "0x0c - command register"] pub cmd: Cmd,
    #[doc = "0x10 - command response register"] pub respcmd: Respcmd,
    #[doc = "0x14 - response 1..4 register"] pub resp1: Resp1,
    #[doc = "0x18 - response 1..4 register"] pub resp2: Resp2,
    #[doc = "0x1c - response 1..4 register"] pub resp3: Resp3,
    #[doc = "0x20 - response 1..4 register"] pub resp4: Resp4,
    #[doc = "0x24 - data timer register"] pub dtimer: Dtimer,
    #[doc = "0x28 - data length register"] pub dlen: Dlen,
    #[doc = "0x2c - data control register"] pub dctrl: Dctrl,
    #[doc = "0x30 - data counter register"] pub dcount: Dcount,
    #[doc = "0x34 - status register"] pub sta: Sta,
    #[doc = "0x38 - interrupt clear register"] pub icr: Icr,
    #[doc = "0x3c - mask register"] pub mask: Mask,
    _reserved0: [u8; 8usize],
    #[doc = "0x48 - FIFO counter register"] pub fifocnt: Fifocnt,
    _reserved1: [u8; 52usize],
    #[doc = "0x80 - data FIFO register"] pub fifo: Fifo,
}
#[doc = "power control register"]
pub struct Power {
    register: VolatileCell<u32>,
}
#[doc = "power control register"]
pub mod power;
#[doc = "SDI clock control register"]
pub struct Clkcr {
    register: VolatileCell<u32>,
}
#[doc = "SDI clock control register"]
pub mod clkcr;
#[doc = "argument register"]
pub struct Arg {
    register: VolatileCell<u32>,
}
#[doc = "argument register"]
pub mod arg;
#[doc = "command register"]
pub struct Cmd {
    register: VolatileCell<u32>,
}
#[doc = "command register"]
pub mod cmd;
#[doc = "command response register"]
pub struct Respcmd {
    register: VolatileCell<u32>,
}
#[doc = "command response register"]
pub mod respcmd;
#[doc = "response 1..4 register"]
pub struct Resp1 {
    register: VolatileCell<u32>,
}
#[doc = "response 1..4 register"]
pub mod resp1;
#[doc = "response 1..4 register"]
pub struct Resp2 {
    register: VolatileCell<u32>,
}
#[doc = "response 1..4 register"]
pub mod resp2;
#[doc = "response 1..4 register"]
pub struct Resp3 {
    register: VolatileCell<u32>,
}
#[doc = "response 1..4 register"]
pub mod resp3;
#[doc = "response 1..4 register"]
pub struct Resp4 {
    register: VolatileCell<u32>,
}
#[doc = "response 1..4 register"]
pub mod resp4;
#[doc = "data timer register"]
pub struct Dtimer {
    register: VolatileCell<u32>,
}
#[doc = "data timer register"]
pub mod dtimer;
#[doc = "data length register"]
pub struct Dlen {
    register: VolatileCell<u32>,
}
#[doc = "data length register"]
pub mod dlen;
#[doc = "data control register"]
pub struct Dctrl {
    register: VolatileCell<u32>,
}
#[doc = "data control register"]
pub mod dctrl;
#[doc = "data counter register"]
pub struct Dcount {
    register: VolatileCell<u32>,
}
#[doc = "data counter register"]
pub mod dcount;
#[doc = "status register"]
pub struct Sta {
    register: VolatileCell<u32>,
}
#[doc = "status register"]
pub mod sta;
#[doc = "interrupt clear register"]
pub struct Icr {
    register: VolatileCell<u32>,
}
#[doc = "interrupt clear register"]
pub mod icr;
#[doc = "mask register"]
pub struct Mask {
    register: VolatileCell<u32>,
}
#[doc = "mask register"]
pub mod mask;
#[doc = "FIFO counter register"]
pub struct Fifocnt {
    register: VolatileCell<u32>,
}
#[doc = "FIFO counter register"]
pub mod fifocnt;
#[doc = "data FIFO register"]
pub struct Fifo {
    register: VolatileCell<u32>,
}
#[doc = "data FIFO register"]
pub mod fifo;
