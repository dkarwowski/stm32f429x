#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::Macdbgr {
    #[doc = r" Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct CrR {
    bits: u8,
}
impl CrR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CsrR {
    bits: u8,
}
impl CsrR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RorR {
    bits: u8,
}
impl RorR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct McfR {
    bits: u8,
}
impl McfR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct McpR {
    bits: u8,
}
impl McpR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct McfhpR {
    bits: u8,
}
impl McfhpR {
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
    #[doc = "Bit 0 - CR"]
    #[inline(always)]
    pub fn cr(&self) -> CrR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CrR { bits }
    }
    #[doc = "Bit 1 - CSR"]
    #[inline(always)]
    pub fn csr(&self) -> CsrR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CsrR { bits }
    }
    #[doc = "Bit 2 - ROR"]
    #[inline(always)]
    pub fn ror(&self) -> RorR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RorR { bits }
    }
    #[doc = "Bit 3 - MCF"]
    #[inline(always)]
    pub fn mcf(&self) -> McfR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        McfR { bits }
    }
    #[doc = "Bit 4 - MCP"]
    #[inline(always)]
    pub fn mcp(&self) -> McpR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        McpR { bits }
    }
    #[doc = "Bit 5 - MCFHP"]
    #[inline(always)]
    pub fn mcfhp(&self) -> McfhpR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        McfhpR { bits }
    }
}
