#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::Sr {
    #[doc = r" Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct RvuR {
    bits: u8,
}
impl RvuR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PvuR {
    bits: u8,
}
impl PvuR {
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
    #[doc = "Bit 1 - Watchdog counter reload value update"]
    #[inline(always)]
    pub fn rvu(&self) -> RvuR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RvuR { bits }
    }
    #[doc = "Bit 0 - Watchdog prescaler value update"]
    #[inline(always)]
    pub fn pvu(&self) -> PvuR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PvuR { bits }
    }
}
