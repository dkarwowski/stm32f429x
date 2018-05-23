#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::Dmabmr {
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
pub struct SrR {
    bits: u8,
}
impl SrR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DaR {
    bits: u8,
}
impl DaR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DslR {
    bits: u8,
}
impl DslR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EdfeR {
    bits: u8,
}
impl EdfeR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PblR {
    bits: u8,
}
impl PblR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RtprR {
    bits: u8,
}
impl RtprR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FbR {
    bits: u8,
}
impl FbR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RdpR {
    bits: u8,
}
impl RdpR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct UspR {
    bits: u8,
}
impl UspR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FpmR {
    bits: u8,
}
impl FpmR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AabR {
    bits: u8,
}
impl AabR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MbR {
    bits: u8,
}
impl MbR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _SrW<'a> {
    w: &'a mut W,
}
impl<'a> _SrW<'a> {
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
#[doc = r" Proxy"]
pub struct _DaW<'a> {
    w: &'a mut W,
}
impl<'a> _DaW<'a> {
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
pub struct _DslW<'a> {
    w: &'a mut W,
}
impl<'a> _DslW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EdfeW<'a> {
    w: &'a mut W,
}
impl<'a> _EdfeW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PblW<'a> {
    w: &'a mut W,
}
impl<'a> _PblW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RtprW<'a> {
    w: &'a mut W,
}
impl<'a> _RtprW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FbW<'a> {
    w: &'a mut W,
}
impl<'a> _FbW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RdpW<'a> {
    w: &'a mut W,
}
impl<'a> _RdpW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _UspW<'a> {
    w: &'a mut W,
}
impl<'a> _UspW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FpmW<'a> {
    w: &'a mut W,
}
impl<'a> _FpmW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AabW<'a> {
    w: &'a mut W,
}
impl<'a> _AabW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MbW<'a> {
    w: &'a mut W,
}
impl<'a> _MbW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 26;
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
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn sr(&self) -> SrR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SrR { bits }
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn da(&self) -> DaR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DaR { bits }
    }
    #[doc = "Bits 2:6 - no description available"]
    #[inline(always)]
    pub fn dsl(&self) -> DslR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DslR { bits }
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn edfe(&self) -> EdfeR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EdfeR { bits }
    }
    #[doc = "Bits 8:13 - no description available"]
    #[inline(always)]
    pub fn pbl(&self) -> PblR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PblR { bits }
    }
    #[doc = "Bits 14:15 - no description available"]
    #[inline(always)]
    pub fn rtpr(&self) -> RtprR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RtprR { bits }
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    pub fn fb(&self) -> FbR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FbR { bits }
    }
    #[doc = "Bits 17:22 - no description available"]
    #[inline(always)]
    pub fn rdp(&self) -> RdpR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RdpR { bits }
    }
    #[doc = "Bit 23 - no description available"]
    #[inline(always)]
    pub fn usp(&self) -> UspR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        UspR { bits }
    }
    #[doc = "Bit 24 - no description available"]
    #[inline(always)]
    pub fn fpm(&self) -> FpmR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FpmR { bits }
    }
    #[doc = "Bit 25 - no description available"]
    #[inline(always)]
    pub fn aab(&self) -> AabR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AabR { bits }
    }
    #[doc = "Bit 26 - no description available"]
    #[inline(always)]
    pub fn mb(&self) -> MbR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MbR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline(always)]
    pub fn reset_value() -> W {
        W { bits: 8449 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn sr(&mut self) -> _SrW {
        _SrW { w: self }
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn da(&mut self) -> _DaW {
        _DaW { w: self }
    }
    #[doc = "Bits 2:6 - no description available"]
    #[inline(always)]
    pub fn dsl(&mut self) -> _DslW {
        _DslW { w: self }
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn edfe(&mut self) -> _EdfeW {
        _EdfeW { w: self }
    }
    #[doc = "Bits 8:13 - no description available"]
    #[inline(always)]
    pub fn pbl(&mut self) -> _PblW {
        _PblW { w: self }
    }
    #[doc = "Bits 14:15 - no description available"]
    #[inline(always)]
    pub fn rtpr(&mut self) -> _RtprW {
        _RtprW { w: self }
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    pub fn fb(&mut self) -> _FbW {
        _FbW { w: self }
    }
    #[doc = "Bits 17:22 - no description available"]
    #[inline(always)]
    pub fn rdp(&mut self) -> _RdpW {
        _RdpW { w: self }
    }
    #[doc = "Bit 23 - no description available"]
    #[inline(always)]
    pub fn usp(&mut self) -> _UspW {
        _UspW { w: self }
    }
    #[doc = "Bit 24 - no description available"]
    #[inline(always)]
    pub fn fpm(&mut self) -> _FpmW {
        _FpmW { w: self }
    }
    #[doc = "Bit 25 - no description available"]
    #[inline(always)]
    pub fn aab(&mut self) -> _AabW {
        _AabW { w: self }
    }
    #[doc = "Bit 26 - no description available"]
    #[inline(always)]
    pub fn mb(&mut self) -> _MbW {
        _MbW { w: self }
    }
}
