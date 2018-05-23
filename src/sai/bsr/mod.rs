#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::Bsr {
    #[doc = r" Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct FlvlR {
    bits: u8,
}
impl FlvlR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LfsdetR {
    bits: u8,
}
impl LfsdetR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AfsdetR {
    bits: u8,
}
impl AfsdetR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CnrdyR {
    bits: u8,
}
impl CnrdyR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FreqR {
    bits: u8,
}
impl FreqR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WckcfgR {
    bits: u8,
}
impl WckcfgR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MutedetR {
    bits: u8,
}
impl MutedetR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OvrudrR {
    bits: u8,
}
impl OvrudrR {
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
    #[doc = "Bits 16:18 - FIFO level threshold"]
    #[inline(always)]
    pub fn flvl(&self) -> FlvlR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FlvlR { bits }
    }
    #[doc = "Bit 6 - Late frame synchronization detection"]
    #[inline(always)]
    pub fn lfsdet(&self) -> LfsdetR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LfsdetR { bits }
    }
    #[doc = "Bit 5 - Anticipated frame synchronization detection"]
    #[inline(always)]
    pub fn afsdet(&self) -> AfsdetR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AfsdetR { bits }
    }
    #[doc = "Bit 4 - Codec not ready"]
    #[inline(always)]
    pub fn cnrdy(&self) -> CnrdyR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CnrdyR { bits }
    }
    #[doc = "Bit 3 - FIFO request"]
    #[inline(always)]
    pub fn freq(&self) -> FreqR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FreqR { bits }
    }
    #[doc = "Bit 2 - Wrong clock configuration flag"]
    #[inline(always)]
    pub fn wckcfg(&self) -> WckcfgR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WckcfgR { bits }
    }
    #[doc = "Bit 1 - Mute detection"]
    #[inline(always)]
    pub fn mutedet(&self) -> MutedetR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MutedetR { bits }
    }
    #[doc = "Bit 0 - Overrun / underrun"]
    #[inline(always)]
    pub fn ovrudr(&self) -> OvrudrR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OvrudrR { bits }
    }
}
