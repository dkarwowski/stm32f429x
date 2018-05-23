#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::Ier {
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
pub struct SlkieR {
    bits: u8,
}
impl SlkieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WkuieR {
    bits: u8,
}
impl WkuieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ErrieR {
    bits: u8,
}
impl ErrieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LecieR {
    bits: u8,
}
impl LecieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BofieR {
    bits: u8,
}
impl BofieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EpvieR {
    bits: u8,
}
impl EpvieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EwgieR {
    bits: u8,
}
impl EwgieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fovie1R {
    bits: u8,
}
impl Fovie1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Ffie1R {
    bits: u8,
}
impl Ffie1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fmpie1R {
    bits: u8,
}
impl Fmpie1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fovie0R {
    bits: u8,
}
impl Fovie0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Ffie0R {
    bits: u8,
}
impl Ffie0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fmpie0R {
    bits: u8,
}
impl Fmpie0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TmeieR {
    bits: u8,
}
impl TmeieR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _SlkieW<'a> {
    w: &'a mut W,
}
impl<'a> _SlkieW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WkuieW<'a> {
    w: &'a mut W,
}
impl<'a> _WkuieW<'a> {
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
pub struct _ErrieW<'a> {
    w: &'a mut W,
}
impl<'a> _ErrieW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LecieW<'a> {
    w: &'a mut W,
}
impl<'a> _LecieW<'a> {
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
pub struct _BofieW<'a> {
    w: &'a mut W,
}
impl<'a> _BofieW<'a> {
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
pub struct _EpvieW<'a> {
    w: &'a mut W,
}
impl<'a> _EpvieW<'a> {
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
pub struct _EwgieW<'a> {
    w: &'a mut W,
}
impl<'a> _EwgieW<'a> {
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
pub struct _Fovie1W<'a> {
    w: &'a mut W,
}
impl<'a> _Fovie1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _Ffie1W<'a> {
    w: &'a mut W,
}
impl<'a> _Ffie1W<'a> {
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
pub struct _Fmpie1W<'a> {
    w: &'a mut W,
}
impl<'a> _Fmpie1W<'a> {
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
pub struct _Fovie0W<'a> {
    w: &'a mut W,
}
impl<'a> _Fovie0W<'a> {
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
pub struct _Ffie0W<'a> {
    w: &'a mut W,
}
impl<'a> _Ffie0W<'a> {
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
pub struct _Fmpie0W<'a> {
    w: &'a mut W,
}
impl<'a> _Fmpie0W<'a> {
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
pub struct _TmeieW<'a> {
    w: &'a mut W,
}
impl<'a> _TmeieW<'a> {
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
    #[doc = "Bit 17 - SLKIE"]
    #[inline(always)]
    pub fn slkie(&self) -> SlkieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SlkieR { bits }
    }
    #[doc = "Bit 16 - WKUIE"]
    #[inline(always)]
    pub fn wkuie(&self) -> WkuieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WkuieR { bits }
    }
    #[doc = "Bit 15 - ERRIE"]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ErrieR { bits }
    }
    #[doc = "Bit 11 - LECIE"]
    #[inline(always)]
    pub fn lecie(&self) -> LecieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LecieR { bits }
    }
    #[doc = "Bit 10 - BOFIE"]
    #[inline(always)]
    pub fn bofie(&self) -> BofieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BofieR { bits }
    }
    #[doc = "Bit 9 - EPVIE"]
    #[inline(always)]
    pub fn epvie(&self) -> EpvieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EpvieR { bits }
    }
    #[doc = "Bit 8 - EWGIE"]
    #[inline(always)]
    pub fn ewgie(&self) -> EwgieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EwgieR { bits }
    }
    #[doc = "Bit 6 - FOVIE1"]
    #[inline(always)]
    pub fn fovie1(&self) -> Fovie1R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fovie1R { bits }
    }
    #[doc = "Bit 5 - FFIE1"]
    #[inline(always)]
    pub fn ffie1(&self) -> Ffie1R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Ffie1R { bits }
    }
    #[doc = "Bit 4 - FMPIE1"]
    #[inline(always)]
    pub fn fmpie1(&self) -> Fmpie1R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fmpie1R { bits }
    }
    #[doc = "Bit 3 - FOVIE0"]
    #[inline(always)]
    pub fn fovie0(&self) -> Fovie0R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fovie0R { bits }
    }
    #[doc = "Bit 2 - FFIE0"]
    #[inline(always)]
    pub fn ffie0(&self) -> Ffie0R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Ffie0R { bits }
    }
    #[doc = "Bit 1 - FMPIE0"]
    #[inline(always)]
    pub fn fmpie0(&self) -> Fmpie0R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fmpie0R { bits }
    }
    #[doc = "Bit 0 - TMEIE"]
    #[inline(always)]
    pub fn tmeie(&self) -> TmeieR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TmeieR { bits }
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
    #[doc = "Bit 17 - SLKIE"]
    #[inline(always)]
    pub fn slkie(&mut self) -> _SlkieW {
        _SlkieW { w: self }
    }
    #[doc = "Bit 16 - WKUIE"]
    #[inline(always)]
    pub fn wkuie(&mut self) -> _WkuieW {
        _WkuieW { w: self }
    }
    #[doc = "Bit 15 - ERRIE"]
    #[inline(always)]
    pub fn errie(&mut self) -> _ErrieW {
        _ErrieW { w: self }
    }
    #[doc = "Bit 11 - LECIE"]
    #[inline(always)]
    pub fn lecie(&mut self) -> _LecieW {
        _LecieW { w: self }
    }
    #[doc = "Bit 10 - BOFIE"]
    #[inline(always)]
    pub fn bofie(&mut self) -> _BofieW {
        _BofieW { w: self }
    }
    #[doc = "Bit 9 - EPVIE"]
    #[inline(always)]
    pub fn epvie(&mut self) -> _EpvieW {
        _EpvieW { w: self }
    }
    #[doc = "Bit 8 - EWGIE"]
    #[inline(always)]
    pub fn ewgie(&mut self) -> _EwgieW {
        _EwgieW { w: self }
    }
    #[doc = "Bit 6 - FOVIE1"]
    #[inline(always)]
    pub fn fovie1(&mut self) -> _Fovie1W {
        _Fovie1W { w: self }
    }
    #[doc = "Bit 5 - FFIE1"]
    #[inline(always)]
    pub fn ffie1(&mut self) -> _Ffie1W {
        _Ffie1W { w: self }
    }
    #[doc = "Bit 4 - FMPIE1"]
    #[inline(always)]
    pub fn fmpie1(&mut self) -> _Fmpie1W {
        _Fmpie1W { w: self }
    }
    #[doc = "Bit 3 - FOVIE0"]
    #[inline(always)]
    pub fn fovie0(&mut self) -> _Fovie0W {
        _Fovie0W { w: self }
    }
    #[doc = "Bit 2 - FFIE0"]
    #[inline(always)]
    pub fn ffie0(&mut self) -> _Ffie0W {
        _Ffie0W { w: self }
    }
    #[doc = "Bit 1 - FMPIE0"]
    #[inline(always)]
    pub fn fmpie0(&mut self) -> _Fmpie0W {
        _Fmpie0W { w: self }
    }
    #[doc = "Bit 0 - TMEIE"]
    #[inline(always)]
    pub fn tmeie(&mut self) -> _TmeieW {
        _TmeieW { w: self }
    }
}
