#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::Ris {
    #[doc = r" Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct LineRisR {
    bits: u8,
}
impl LineRisR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VsyncRisR {
    bits: u8,
}
impl VsyncRisR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ErrRisR {
    bits: u8,
}
impl ErrRisR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OvrRisR {
    bits: u8,
}
impl OvrRisR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FrameRisR {
    bits: u8,
}
impl FrameRisR {
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
    #[doc = "Bit 4 - Line raw interrupt status"]
    #[inline(always)]
    pub fn line_ris(&self) -> LineRisR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LineRisR { bits }
    }
    #[doc = "Bit 3 - VSYNC raw interrupt status"]
    #[inline(always)]
    pub fn vsync_ris(&self) -> VsyncRisR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VsyncRisR { bits }
    }
    #[doc = "Bit 2 - Synchronization error raw interrupt status"]
    #[inline(always)]
    pub fn err_ris(&self) -> ErrRisR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ErrRisR { bits }
    }
    #[doc = "Bit 1 - Overrun raw interrupt status"]
    #[inline(always)]
    pub fn ovr_ris(&self) -> OvrRisR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OvrRisR { bits }
    }
    #[doc = "Bit 0 - Capture complete raw interrupt status"]
    #[inline(always)]
    pub fn frame_ris(&self) -> FrameRisR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FrameRisR { bits }
    }
}
