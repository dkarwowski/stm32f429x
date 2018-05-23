#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::Gcr {
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
pub struct HspolR {
    bits: u8,
}
impl HspolR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VspolR {
    bits: u8,
}
impl VspolR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DepolR {
    bits: u8,
}
impl DepolR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PcpolR {
    bits: u8,
}
impl PcpolR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DenR {
    bits: u8,
}
impl DenR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DrwR {
    bits: u8,
}
impl DrwR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DgwR {
    bits: u8,
}
impl DgwR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DbwR {
    bits: u8,
}
impl DbwR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LtdcenR {
    bits: u8,
}
impl LtdcenR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _HspolW<'a> {
    w: &'a mut W,
}
impl<'a> _HspolW<'a> {
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
pub struct _VspolW<'a> {
    w: &'a mut W,
}
impl<'a> _VspolW<'a> {
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
pub struct _DepolW<'a> {
    w: &'a mut W,
}
impl<'a> _DepolW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PcpolW<'a> {
    w: &'a mut W,
}
impl<'a> _PcpolW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DenW<'a> {
    w: &'a mut W,
}
impl<'a> _DenW<'a> {
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
pub struct _LtdcenW<'a> {
    w: &'a mut W,
}
impl<'a> _LtdcenW<'a> {
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
    #[doc = "Bit 31 - Horizontal Synchronization Polarity"]
    #[inline(always)]
    pub fn hspol(&self) -> HspolR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HspolR { bits }
    }
    #[doc = "Bit 30 - Vertical Synchronization Polarity"]
    #[inline(always)]
    pub fn vspol(&self) -> VspolR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VspolR { bits }
    }
    #[doc = "Bit 29 - Data Enable Polarity"]
    #[inline(always)]
    pub fn depol(&self) -> DepolR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DepolR { bits }
    }
    #[doc = "Bit 28 - Pixel Clock Polarity"]
    #[inline(always)]
    pub fn pcpol(&self) -> PcpolR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PcpolR { bits }
    }
    #[doc = "Bit 16 - Dither Enable"]
    #[inline(always)]
    pub fn den(&self) -> DenR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DenR { bits }
    }
    #[doc = "Bits 12:14 - Dither Red Width"]
    #[inline(always)]
    pub fn drw(&self) -> DrwR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DrwR { bits }
    }
    #[doc = "Bits 8:10 - Dither Green Width"]
    #[inline(always)]
    pub fn dgw(&self) -> DgwR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DgwR { bits }
    }
    #[doc = "Bits 4:6 - Dither Blue Width"]
    #[inline(always)]
    pub fn dbw(&self) -> DbwR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DbwR { bits }
    }
    #[doc = "Bit 0 - LCD-TFT controller enable bit"]
    #[inline(always)]
    pub fn ltdcen(&self) -> LtdcenR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LtdcenR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline(always)]
    pub fn reset_value() -> W {
        W { bits: 8736 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 31 - Horizontal Synchronization Polarity"]
    #[inline(always)]
    pub fn hspol(&mut self) -> _HspolW {
        _HspolW { w: self }
    }
    #[doc = "Bit 30 - Vertical Synchronization Polarity"]
    #[inline(always)]
    pub fn vspol(&mut self) -> _VspolW {
        _VspolW { w: self }
    }
    #[doc = "Bit 29 - Data Enable Polarity"]
    #[inline(always)]
    pub fn depol(&mut self) -> _DepolW {
        _DepolW { w: self }
    }
    #[doc = "Bit 28 - Pixel Clock Polarity"]
    #[inline(always)]
    pub fn pcpol(&mut self) -> _PcpolW {
        _PcpolW { w: self }
    }
    #[doc = "Bit 16 - Dither Enable"]
    #[inline(always)]
    pub fn den(&mut self) -> _DenW {
        _DenW { w: self }
    }
    #[doc = "Bit 0 - LCD-TFT controller enable bit"]
    #[inline(always)]
    pub fn ltdcen(&mut self) -> _LtdcenW {
        _LtdcenW { w: self }
    }
}
