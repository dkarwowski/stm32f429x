#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::Cdsr {
    #[doc = r" Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct HsyncsR {
    bits: u8,
}
impl HsyncsR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VsyncsR {
    bits: u8,
}
impl VsyncsR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HdesR {
    bits: u8,
}
impl HdesR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VdesR {
    bits: u8,
}
impl VdesR {
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
    #[doc = "Bit 3 - Horizontal Synchronization display Status"]
    #[inline(always)]
    pub fn hsyncs(&self) -> HsyncsR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HsyncsR { bits }
    }
    #[doc = "Bit 2 - Vertical Synchronization display Status"]
    #[inline(always)]
    pub fn vsyncs(&self) -> VsyncsR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VsyncsR { bits }
    }
    #[doc = "Bit 1 - Horizontal Data Enable display Status"]
    #[inline(always)]
    pub fn hdes(&self) -> HdesR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HdesR { bits }
    }
    #[doc = "Bit 0 - Vertical Data Enable display Status"]
    #[inline(always)]
    pub fn vdes(&self) -> VdesR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VdesR { bits }
    }
}
