#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::Sscgr {
    #[doc = r" Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
}
#[doc = r" Value of the field"]
pub struct SscgenR {
    bits: u8,
}
impl SscgenR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SpreadselR {
    bits: u8,
}
impl SpreadselR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IncstepR {
    bits: u16,
}
impl IncstepR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ModperR {
    bits: u16,
}
impl ModperR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _SscgenW<'a> {
    w: &'a mut W,
}
impl<'a> _SscgenW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SpreadselW<'a> {
    w: &'a mut W,
}
impl<'a> _SpreadselW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IncstepW<'a> {
    w: &'a mut W,
}
impl<'a> _IncstepW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u16) -> &'a mut W {
        const MASK: u16 = 32767;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ModperW<'a> {
    w: &'a mut W,
}
impl<'a> _ModperW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u16) -> &'a mut W {
        const MASK: u16 = 8191;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 31 - Spread spectrum modulation enable"]
    #[inline(always)]
    pub fn sscgen(&self) -> SscgenR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SscgenR { bits }
    }
    #[doc = "Bit 30 - Spread Select"]
    #[inline(always)]
    pub fn spreadsel(&self) -> SpreadselR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SpreadselR { bits }
    }
    #[doc = "Bits 13:27 - Incrementation step"]
    #[inline(always)]
    pub fn incstep(&self) -> IncstepR {
        let bits = {
            const MASK: u16 = 32767;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        IncstepR { bits }
    }
    #[doc = "Bits 0:12 - Modulation period"]
    #[inline(always)]
    pub fn modper(&self) -> ModperR {
        let bits = {
            const MASK: u16 = 8191;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        ModperR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline(always)]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 31 - Spread spectrum modulation enable"]
    #[inline(always)]
    pub fn sscgen(&mut self) -> _SscgenW {
        _SscgenW { w: self }
    }
    #[doc = "Bit 30 - Spread Select"]
    #[inline(always)]
    pub fn spreadsel(&mut self) -> _SpreadselW {
        _SpreadselW { w: self }
    }
    #[doc = "Bits 13:27 - Incrementation step"]
    #[inline(always)]
    pub fn incstep(&mut self) -> _IncstepW {
        _IncstepW { w: self }
    }
    #[doc = "Bits 0:12 - Modulation period"]
    #[inline(always)]
    pub fn modper(&mut self) -> _ModperW {
        _ModperW { w: self }
    }
}
