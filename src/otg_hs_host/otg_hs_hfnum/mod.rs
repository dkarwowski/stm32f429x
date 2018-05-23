#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::OtgHsHfnum {
    #[doc = r" Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct FrnumR {
    bits: u16,
}
impl FrnumR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FtremR {
    bits: u16,
}
impl FtremR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - Frame number"]
    #[inline(always)]
    pub fn frnum(&self) -> FrnumR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        FrnumR { bits }
    }
    #[doc = "Bits 16:31 - Frame time remaining"]
    #[inline(always)]
    pub fn ftrem(&self) -> FtremR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        FtremR { bits }
    }
}
