#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::Sr2 {
    #[doc = r" Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct PecR {
    bits: u8,
}
impl PecR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DualfR {
    bits: u8,
}
impl DualfR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SmbhostR {
    bits: u8,
}
impl SmbhostR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SmbdefaultR {
    bits: u8,
}
impl SmbdefaultR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GencallR {
    bits: u8,
}
impl GencallR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TraR {
    bits: u8,
}
impl TraR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BusyR {
    bits: u8,
}
impl BusyR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MslR {
    bits: u8,
}
impl MslR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 8:15 - acket error checking register"]
    #[inline(always)]
    pub fn pec(&self) -> PecR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PecR { bits }
    }
    #[doc = "Bit 7 - Dual flag (Slave mode)"]
    #[inline(always)]
    pub fn dualf(&self) -> DualfR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DualfR { bits }
    }
    #[doc = "Bit 6 - SMBus host header (Slave mode)"]
    #[inline(always)]
    pub fn smbhost(&self) -> SmbhostR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SmbhostR { bits }
    }
    #[doc = "Bit 5 - SMBus device default address (Slave mode)"]
    #[inline(always)]
    pub fn smbdefault(&self) -> SmbdefaultR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SmbdefaultR { bits }
    }
    #[doc = "Bit 4 - General call address (Slave mode)"]
    #[inline(always)]
    pub fn gencall(&self) -> GencallR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GencallR { bits }
    }
    #[doc = "Bit 2 - Transmitter/receiver"]
    #[inline(always)]
    pub fn tra(&self) -> TraR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TraR { bits }
    }
    #[doc = "Bit 1 - Bus busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BusyR { bits }
    }
    #[doc = "Bit 0 - Master/slave"]
    #[inline(always)]
    pub fn msl(&self) -> MslR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MslR { bits }
    }
}
