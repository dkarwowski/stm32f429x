#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::OtgHsGnptxsts {
    #[doc = r" Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct NptxfsavR {
    bits: u16,
}
impl NptxfsavR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NptqxsavR {
    bits: u8,
}
impl NptqxsavR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NptxqtopR {
    bits: u8,
}
impl NptxqtopR {
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
    #[doc = "Bits 0:15 - Nonperiodic TxFIFO space available"]
    #[inline(always)]
    pub fn nptxfsav(&self) -> NptxfsavR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        NptxfsavR { bits }
    }
    #[doc = "Bits 16:23 - Nonperiodic transmit request queue space available"]
    #[inline(always)]
    pub fn nptqxsav(&self) -> NptqxsavR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NptqxsavR { bits }
    }
    #[doc = "Bits 24:30 - Top of the nonperiodic transmit request queue"]
    #[inline(always)]
    pub fn nptxqtop(&self) -> NptxqtopR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NptxqtopR { bits }
    }
}
