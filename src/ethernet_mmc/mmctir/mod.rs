#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::Mmctir {
    #[doc = r" Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct TgfscsR {
    bits: u8,
}
impl TgfscsR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TgfmscsR {
    bits: u8,
}
impl TgfmscsR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TgfsR {
    bits: u8,
}
impl TgfsR {
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
    #[doc = "Bit 14 - no description available"]
    #[inline(always)]
    pub fn tgfscs(&self) -> TgfscsR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TgfscsR { bits }
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    pub fn tgfmscs(&self) -> TgfmscsR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TgfmscsR { bits }
    }
    #[doc = "Bit 21 - no description available"]
    #[inline(always)]
    pub fn tgfs(&self) -> TgfsR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TgfsR { bits }
    }
}
