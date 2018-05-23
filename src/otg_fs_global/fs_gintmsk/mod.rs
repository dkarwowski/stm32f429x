#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FsGintmsk {
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
pub struct MmismR {
    bits: u8,
}
impl MmismR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OtgintR {
    bits: u8,
}
impl OtgintR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SofmR {
    bits: u8,
}
impl SofmR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RxflvlmR {
    bits: u8,
}
impl RxflvlmR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NptxfemR {
    bits: u8,
}
impl NptxfemR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GinakeffmR {
    bits: u8,
}
impl GinakeffmR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GonakeffmR {
    bits: u8,
}
impl GonakeffmR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EsuspmR {
    bits: u8,
}
impl EsuspmR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct UsbsuspmR {
    bits: u8,
}
impl UsbsuspmR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct UsbrstR {
    bits: u8,
}
impl UsbrstR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EnumdnemR {
    bits: u8,
}
impl EnumdnemR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IsoodrpmR {
    bits: u8,
}
impl IsoodrpmR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EopfmR {
    bits: u8,
}
impl EopfmR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EpmismR {
    bits: u8,
}
impl EpmismR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IepintR {
    bits: u8,
}
impl IepintR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OepintR {
    bits: u8,
}
impl OepintR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IisoixfrmR {
    bits: u8,
}
impl IisoixfrmR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IpxfrmIisooxfrmR {
    bits: u8,
}
impl IpxfrmIisooxfrmR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PrtimR {
    bits: u8,
}
impl PrtimR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HcimR {
    bits: u8,
}
impl HcimR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PtxfemR {
    bits: u8,
}
impl PtxfemR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CidschgmR {
    bits: u8,
}
impl CidschgmR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DiscintR {
    bits: u8,
}
impl DiscintR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SrqimR {
    bits: u8,
}
impl SrqimR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WuimR {
    bits: u8,
}
impl WuimR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _MmismW<'a> {
    w: &'a mut W,
}
impl<'a> _MmismW<'a> {
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
pub struct _OtgintW<'a> {
    w: &'a mut W,
}
impl<'a> _OtgintW<'a> {
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
pub struct _SofmW<'a> {
    w: &'a mut W,
}
impl<'a> _SofmW<'a> {
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
pub struct _RxflvlmW<'a> {
    w: &'a mut W,
}
impl<'a> _RxflvlmW<'a> {
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
pub struct _NptxfemW<'a> {
    w: &'a mut W,
}
impl<'a> _NptxfemW<'a> {
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
pub struct _GinakeffmW<'a> {
    w: &'a mut W,
}
impl<'a> _GinakeffmW<'a> {
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
pub struct _GonakeffmW<'a> {
    w: &'a mut W,
}
impl<'a> _GonakeffmW<'a> {
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
pub struct _EsuspmW<'a> {
    w: &'a mut W,
}
impl<'a> _EsuspmW<'a> {
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
pub struct _UsbsuspmW<'a> {
    w: &'a mut W,
}
impl<'a> _UsbsuspmW<'a> {
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
pub struct _UsbrstW<'a> {
    w: &'a mut W,
}
impl<'a> _UsbrstW<'a> {
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
pub struct _EnumdnemW<'a> {
    w: &'a mut W,
}
impl<'a> _EnumdnemW<'a> {
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
pub struct _IsoodrpmW<'a> {
    w: &'a mut W,
}
impl<'a> _IsoodrpmW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EopfmW<'a> {
    w: &'a mut W,
}
impl<'a> _EopfmW<'a> {
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
pub struct _EpmismW<'a> {
    w: &'a mut W,
}
impl<'a> _EpmismW<'a> {
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
pub struct _IepintW<'a> {
    w: &'a mut W,
}
impl<'a> _IepintW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OepintW<'a> {
    w: &'a mut W,
}
impl<'a> _OepintW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IisoixfrmW<'a> {
    w: &'a mut W,
}
impl<'a> _IisoixfrmW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IpxfrmIisooxfrmW<'a> {
    w: &'a mut W,
}
impl<'a> _IpxfrmIisooxfrmW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HcimW<'a> {
    w: &'a mut W,
}
impl<'a> _HcimW<'a> {
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
pub struct _PtxfemW<'a> {
    w: &'a mut W,
}
impl<'a> _PtxfemW<'a> {
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
#[doc = r" Proxy"]
pub struct _CidschgmW<'a> {
    w: &'a mut W,
}
impl<'a> _CidschgmW<'a> {
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
pub struct _DiscintW<'a> {
    w: &'a mut W,
}
impl<'a> _DiscintW<'a> {
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
pub struct _SrqimW<'a> {
    w: &'a mut W,
}
impl<'a> _SrqimW<'a> {
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
pub struct _WuimW<'a> {
    w: &'a mut W,
}
impl<'a> _WuimW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - Mode mismatch interrupt mask"]
    #[inline(always)]
    pub fn mmism(&self) -> MmismR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MmismR { bits }
    }
    #[doc = "Bit 2 - OTG interrupt mask"]
    #[inline(always)]
    pub fn otgint(&self) -> OtgintR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OtgintR { bits }
    }
    #[doc = "Bit 3 - Start of frame mask"]
    #[inline(always)]
    pub fn sofm(&self) -> SofmR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SofmR { bits }
    }
    #[doc = "Bit 4 - Receive FIFO non-empty mask"]
    #[inline(always)]
    pub fn rxflvlm(&self) -> RxflvlmR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RxflvlmR { bits }
    }
    #[doc = "Bit 5 - Non-periodic TxFIFO empty mask"]
    #[inline(always)]
    pub fn nptxfem(&self) -> NptxfemR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NptxfemR { bits }
    }
    #[doc = "Bit 6 - Global non-periodic IN NAK effective mask"]
    #[inline(always)]
    pub fn ginakeffm(&self) -> GinakeffmR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GinakeffmR { bits }
    }
    #[doc = "Bit 7 - Global OUT NAK effective mask"]
    #[inline(always)]
    pub fn gonakeffm(&self) -> GonakeffmR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GonakeffmR { bits }
    }
    #[doc = "Bit 10 - Early suspend mask"]
    #[inline(always)]
    pub fn esuspm(&self) -> EsuspmR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EsuspmR { bits }
    }
    #[doc = "Bit 11 - USB suspend mask"]
    #[inline(always)]
    pub fn usbsuspm(&self) -> UsbsuspmR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        UsbsuspmR { bits }
    }
    #[doc = "Bit 12 - USB reset mask"]
    #[inline(always)]
    pub fn usbrst(&self) -> UsbrstR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        UsbrstR { bits }
    }
    #[doc = "Bit 13 - Enumeration done mask"]
    #[inline(always)]
    pub fn enumdnem(&self) -> EnumdnemR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EnumdnemR { bits }
    }
    #[doc = "Bit 14 - Isochronous OUT packet dropped interrupt mask"]
    #[inline(always)]
    pub fn isoodrpm(&self) -> IsoodrpmR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IsoodrpmR { bits }
    }
    #[doc = "Bit 15 - End of periodic frame interrupt mask"]
    #[inline(always)]
    pub fn eopfm(&self) -> EopfmR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EopfmR { bits }
    }
    #[doc = "Bit 17 - Endpoint mismatch interrupt mask"]
    #[inline(always)]
    pub fn epmism(&self) -> EpmismR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EpmismR { bits }
    }
    #[doc = "Bit 18 - IN endpoints interrupt mask"]
    #[inline(always)]
    pub fn iepint(&self) -> IepintR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IepintR { bits }
    }
    #[doc = "Bit 19 - OUT endpoints interrupt mask"]
    #[inline(always)]
    pub fn oepint(&self) -> OepintR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OepintR { bits }
    }
    #[doc = "Bit 20 - Incomplete isochronous IN transfer mask"]
    #[inline(always)]
    pub fn iisoixfrm(&self) -> IisoixfrmR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IisoixfrmR { bits }
    }
    # [ doc = "Bit 21 - Incomplete periodic transfer mask(Host mode)/Incomplete isochronous OUT transfer mask(Device mode)" ] # [ inline ( always ) ]
    pub fn ipxfrm_iisooxfrm(&self) -> IpxfrmIisooxfrmR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IpxfrmIisooxfrmR { bits }
    }
    #[doc = "Bit 24 - Host port interrupt mask"]
    #[inline(always)]
    pub fn prtim(&self) -> PrtimR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PrtimR { bits }
    }
    #[doc = "Bit 25 - Host channels interrupt mask"]
    #[inline(always)]
    pub fn hcim(&self) -> HcimR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HcimR { bits }
    }
    #[doc = "Bit 26 - Periodic TxFIFO empty mask"]
    #[inline(always)]
    pub fn ptxfem(&self) -> PtxfemR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PtxfemR { bits }
    }
    #[doc = "Bit 28 - Connector ID status change mask"]
    #[inline(always)]
    pub fn cidschgm(&self) -> CidschgmR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CidschgmR { bits }
    }
    #[doc = "Bit 29 - Disconnect detected interrupt mask"]
    #[inline(always)]
    pub fn discint(&self) -> DiscintR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DiscintR { bits }
    }
    #[doc = "Bit 30 - Session request/new session detected interrupt mask"]
    #[inline(always)]
    pub fn srqim(&self) -> SrqimR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SrqimR { bits }
    }
    #[doc = "Bit 31 - Resume/remote wakeup detected interrupt mask"]
    #[inline(always)]
    pub fn wuim(&self) -> WuimR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WuimR { bits }
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
    #[doc = "Bit 1 - Mode mismatch interrupt mask"]
    #[inline(always)]
    pub fn mmism(&mut self) -> _MmismW {
        _MmismW { w: self }
    }
    #[doc = "Bit 2 - OTG interrupt mask"]
    #[inline(always)]
    pub fn otgint(&mut self) -> _OtgintW {
        _OtgintW { w: self }
    }
    #[doc = "Bit 3 - Start of frame mask"]
    #[inline(always)]
    pub fn sofm(&mut self) -> _SofmW {
        _SofmW { w: self }
    }
    #[doc = "Bit 4 - Receive FIFO non-empty mask"]
    #[inline(always)]
    pub fn rxflvlm(&mut self) -> _RxflvlmW {
        _RxflvlmW { w: self }
    }
    #[doc = "Bit 5 - Non-periodic TxFIFO empty mask"]
    #[inline(always)]
    pub fn nptxfem(&mut self) -> _NptxfemW {
        _NptxfemW { w: self }
    }
    #[doc = "Bit 6 - Global non-periodic IN NAK effective mask"]
    #[inline(always)]
    pub fn ginakeffm(&mut self) -> _GinakeffmW {
        _GinakeffmW { w: self }
    }
    #[doc = "Bit 7 - Global OUT NAK effective mask"]
    #[inline(always)]
    pub fn gonakeffm(&mut self) -> _GonakeffmW {
        _GonakeffmW { w: self }
    }
    #[doc = "Bit 10 - Early suspend mask"]
    #[inline(always)]
    pub fn esuspm(&mut self) -> _EsuspmW {
        _EsuspmW { w: self }
    }
    #[doc = "Bit 11 - USB suspend mask"]
    #[inline(always)]
    pub fn usbsuspm(&mut self) -> _UsbsuspmW {
        _UsbsuspmW { w: self }
    }
    #[doc = "Bit 12 - USB reset mask"]
    #[inline(always)]
    pub fn usbrst(&mut self) -> _UsbrstW {
        _UsbrstW { w: self }
    }
    #[doc = "Bit 13 - Enumeration done mask"]
    #[inline(always)]
    pub fn enumdnem(&mut self) -> _EnumdnemW {
        _EnumdnemW { w: self }
    }
    #[doc = "Bit 14 - Isochronous OUT packet dropped interrupt mask"]
    #[inline(always)]
    pub fn isoodrpm(&mut self) -> _IsoodrpmW {
        _IsoodrpmW { w: self }
    }
    #[doc = "Bit 15 - End of periodic frame interrupt mask"]
    #[inline(always)]
    pub fn eopfm(&mut self) -> _EopfmW {
        _EopfmW { w: self }
    }
    #[doc = "Bit 17 - Endpoint mismatch interrupt mask"]
    #[inline(always)]
    pub fn epmism(&mut self) -> _EpmismW {
        _EpmismW { w: self }
    }
    #[doc = "Bit 18 - IN endpoints interrupt mask"]
    #[inline(always)]
    pub fn iepint(&mut self) -> _IepintW {
        _IepintW { w: self }
    }
    #[doc = "Bit 19 - OUT endpoints interrupt mask"]
    #[inline(always)]
    pub fn oepint(&mut self) -> _OepintW {
        _OepintW { w: self }
    }
    #[doc = "Bit 20 - Incomplete isochronous IN transfer mask"]
    #[inline(always)]
    pub fn iisoixfrm(&mut self) -> _IisoixfrmW {
        _IisoixfrmW { w: self }
    }
    # [ doc = "Bit 21 - Incomplete periodic transfer mask(Host mode)/Incomplete isochronous OUT transfer mask(Device mode)" ] # [ inline ( always ) ]
    pub fn ipxfrm_iisooxfrm(&mut self) -> _IpxfrmIisooxfrmW {
        _IpxfrmIisooxfrmW { w: self }
    }
    #[doc = "Bit 25 - Host channels interrupt mask"]
    #[inline(always)]
    pub fn hcim(&mut self) -> _HcimW {
        _HcimW { w: self }
    }
    #[doc = "Bit 26 - Periodic TxFIFO empty mask"]
    #[inline(always)]
    pub fn ptxfem(&mut self) -> _PtxfemW {
        _PtxfemW { w: self }
    }
    #[doc = "Bit 28 - Connector ID status change mask"]
    #[inline(always)]
    pub fn cidschgm(&mut self) -> _CidschgmW {
        _CidschgmW { w: self }
    }
    #[doc = "Bit 29 - Disconnect detected interrupt mask"]
    #[inline(always)]
    pub fn discint(&mut self) -> _DiscintW {
        _DiscintW { w: self }
    }
    #[doc = "Bit 30 - Session request/new session detected interrupt mask"]
    #[inline(always)]
    pub fn srqim(&mut self) -> _SrqimW {
        _SrqimW { w: self }
    }
    #[doc = "Bit 31 - Resume/remote wakeup detected interrupt mask"]
    #[inline(always)]
    pub fn wuim(&mut self) -> _WuimW {
        _WuimW { w: self }
    }
}
