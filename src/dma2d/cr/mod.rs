#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::Cr {
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
pub struct ModeR {
    bits: u8,
}
impl ModeR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CeieR {
    bits: u8,
}
impl CeieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CtcieR {
    bits: u8,
}
impl CtcieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CaeieR {
    bits: u8,
}
impl CaeieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TwieR {
    bits: u8,
}
impl TwieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TcieR {
    bits: u8,
}
impl TcieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TeieR {
    bits: u8,
}
impl TeieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AbortR {
    bits: u8,
}
impl AbortR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SuspR {
    bits: u8,
}
impl SuspR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct StartR {
    bits: u8,
}
impl StartR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _ModeW<'a> {
    w: &'a mut W,
}
impl<'a> _ModeW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CeieW<'a> {
    w: &'a mut W,
}
impl<'a> _CeieW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CtcieW<'a> {
    w: &'a mut W,
}
impl<'a> _CtcieW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CaeieW<'a> {
    w: &'a mut W,
}
impl<'a> _CaeieW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TwieW<'a> {
    w: &'a mut W,
}
impl<'a> _TwieW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TcieW<'a> {
    w: &'a mut W,
}
impl<'a> _TcieW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TeieW<'a> {
    w: &'a mut W,
}
impl<'a> _TeieW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AbortW<'a> {
    w: &'a mut W,
}
impl<'a> _AbortW<'a> {
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
pub struct _SuspW<'a> {
    w: &'a mut W,
}
impl<'a> _SuspW<'a> {
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
pub struct _StartW<'a> {
    w: &'a mut W,
}
impl<'a> _StartW<'a> {
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
    #[doc = "Bits 16:17 - DMA2D mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ModeR { bits }
    }
    #[doc = "Bit 13 - Configuration Error Interrupt Enable"]
    #[inline(always)]
    pub fn ceie(&self) -> CeieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CeieR { bits }
    }
    #[doc = "Bit 12 - CLUT transfer complete interrupt enable"]
    #[inline(always)]
    pub fn ctcie(&self) -> CtcieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CtcieR { bits }
    }
    #[doc = "Bit 11 - CLUT access error interrupt enable"]
    #[inline(always)]
    pub fn caeie(&self) -> CaeieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CaeieR { bits }
    }
    #[doc = "Bit 10 - Transfer watermark interrupt enable"]
    #[inline(always)]
    pub fn twie(&self) -> TwieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TwieR { bits }
    }
    #[doc = "Bit 9 - Transfer complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TcieR { bits }
    }
    #[doc = "Bit 8 - Transfer error interrupt enable"]
    #[inline(always)]
    pub fn teie(&self) -> TeieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TeieR { bits }
    }
    #[doc = "Bit 2 - Abort"]
    #[inline(always)]
    pub fn abort(&self) -> AbortR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AbortR { bits }
    }
    #[doc = "Bit 1 - Suspend"]
    #[inline(always)]
    pub fn susp(&self) -> SuspR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SuspR { bits }
    }
    #[doc = "Bit 0 - Start"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        StartR { bits }
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
    #[doc = "Bits 16:17 - DMA2D mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> _ModeW {
        _ModeW { w: self }
    }
    #[doc = "Bit 13 - Configuration Error Interrupt Enable"]
    #[inline(always)]
    pub fn ceie(&mut self) -> _CeieW {
        _CeieW { w: self }
    }
    #[doc = "Bit 12 - CLUT transfer complete interrupt enable"]
    #[inline(always)]
    pub fn ctcie(&mut self) -> _CtcieW {
        _CtcieW { w: self }
    }
    #[doc = "Bit 11 - CLUT access error interrupt enable"]
    #[inline(always)]
    pub fn caeie(&mut self) -> _CaeieW {
        _CaeieW { w: self }
    }
    #[doc = "Bit 10 - Transfer watermark interrupt enable"]
    #[inline(always)]
    pub fn twie(&mut self) -> _TwieW {
        _TwieW { w: self }
    }
    #[doc = "Bit 9 - Transfer complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&mut self) -> _TcieW {
        _TcieW { w: self }
    }
    #[doc = "Bit 8 - Transfer error interrupt enable"]
    #[inline(always)]
    pub fn teie(&mut self) -> _TeieW {
        _TeieW { w: self }
    }
    #[doc = "Bit 2 - Abort"]
    #[inline(always)]
    pub fn abort(&mut self) -> _AbortW {
        _AbortW { w: self }
    }
    #[doc = "Bit 1 - Suspend"]
    #[inline(always)]
    pub fn susp(&mut self) -> _SuspW {
        _SuspW { w: self }
    }
    #[doc = "Bit 0 - Start"]
    #[inline(always)]
    pub fn start(&mut self) -> _StartW {
        _StartW { w: self }
    }
}
