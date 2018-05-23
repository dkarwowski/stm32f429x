#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::Ifcr {
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
pub struct CceifR {
    bits: u8,
}
impl CceifR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CctcifR {
    bits: u8,
}
impl CctcifR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CaecifR {
    bits: u8,
}
impl CaecifR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CtwifR {
    bits: u8,
}
impl CtwifR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CtcifR {
    bits: u8,
}
impl CtcifR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CteifR {
    bits: u8,
}
impl CteifR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CceifW<'a> {
    w: &'a mut W,
}
impl<'a> _CceifW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CctcifW<'a> {
    w: &'a mut W,
}
impl<'a> _CctcifW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CaecifW<'a> {
    w: &'a mut W,
}
impl<'a> _CaecifW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CtwifW<'a> {
    w: &'a mut W,
}
impl<'a> _CtwifW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CtcifW<'a> {
    w: &'a mut W,
}
impl<'a> _CtcifW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CteifW<'a> {
    w: &'a mut W,
}
impl<'a> _CteifW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
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
    #[doc = "Bit 5 - Clear configuration error interrupt flag"]
    #[inline(always)]
    pub fn cceif(&self) -> CceifR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CceifR { bits }
    }
    #[doc = "Bit 4 - Clear CLUT transfer complete interrupt flag"]
    #[inline(always)]
    pub fn cctcif(&self) -> CctcifR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CctcifR { bits }
    }
    #[doc = "Bit 3 - Clear CLUT access error interrupt flag"]
    #[inline(always)]
    pub fn caecif(&self) -> CaecifR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CaecifR { bits }
    }
    #[doc = "Bit 2 - Clear transfer watermark interrupt flag"]
    #[inline(always)]
    pub fn ctwif(&self) -> CtwifR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CtwifR { bits }
    }
    #[doc = "Bit 1 - Clear transfer complete interrupt flag"]
    #[inline(always)]
    pub fn ctcif(&self) -> CtcifR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CtcifR { bits }
    }
    #[doc = "Bit 0 - Clear Transfer error interrupt flag"]
    #[inline(always)]
    pub fn cteif(&self) -> CteifR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CteifR { bits }
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
    #[doc = "Bit 5 - Clear configuration error interrupt flag"]
    #[inline(always)]
    pub fn cceif(&mut self) -> _CceifW {
        _CceifW { w: self }
    }
    #[doc = "Bit 4 - Clear CLUT transfer complete interrupt flag"]
    #[inline(always)]
    pub fn cctcif(&mut self) -> _CctcifW {
        _CctcifW { w: self }
    }
    #[doc = "Bit 3 - Clear CLUT access error interrupt flag"]
    #[inline(always)]
    pub fn caecif(&mut self) -> _CaecifW {
        _CaecifW { w: self }
    }
    #[doc = "Bit 2 - Clear transfer watermark interrupt flag"]
    #[inline(always)]
    pub fn ctwif(&mut self) -> _CtwifW {
        _CtwifW { w: self }
    }
    #[doc = "Bit 1 - Clear transfer complete interrupt flag"]
    #[inline(always)]
    pub fn ctcif(&mut self) -> _CtcifW {
        _CtcifW { w: self }
    }
    #[doc = "Bit 0 - Clear Transfer error interrupt flag"]
    #[inline(always)]
    pub fn cteif(&mut self) -> _CteifW {
        _CteifW { w: self }
    }
}
