#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::Bsrr {
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
#[doc = "Values that can be written to the field `BR15`"]
pub enum Br15W {
    #[doc = "Reset the corresponding ODRx bit."] Reset,
}
impl Br15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            Br15W::Reset => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _Br15W<'a> {
    w: &'a mut W,
}
impl<'a> _Br15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Br15W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Reset the corresponding ODRx bit."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(Br15W::Reset)
    }
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
#[doc = "Values that can be written to the field `BR14`"]
pub type Br14W = Br15W;
#[doc = r" Proxy"]
pub struct _Br14W<'a> {
    w: &'a mut W,
}
impl<'a> _Br14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Br14W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Reset the corresponding ODRx bit."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(Br15W::Reset)
    }
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
#[doc = "Values that can be written to the field `BR13`"]
pub type Br13W = Br15W;
#[doc = r" Proxy"]
pub struct _Br13W<'a> {
    w: &'a mut W,
}
impl<'a> _Br13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Br13W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Reset the corresponding ODRx bit."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(Br15W::Reset)
    }
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
#[doc = "Values that can be written to the field `BR12`"]
pub type Br12W = Br15W;
#[doc = r" Proxy"]
pub struct _Br12W<'a> {
    w: &'a mut W,
}
impl<'a> _Br12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Br12W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Reset the corresponding ODRx bit."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(Br15W::Reset)
    }
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
#[doc = "Values that can be written to the field `BR11`"]
pub type Br11W = Br15W;
#[doc = r" Proxy"]
pub struct _Br11W<'a> {
    w: &'a mut W,
}
impl<'a> _Br11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Br11W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Reset the corresponding ODRx bit."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(Br15W::Reset)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BR10`"]
pub type Br10W = Br15W;
#[doc = r" Proxy"]
pub struct _Br10W<'a> {
    w: &'a mut W,
}
impl<'a> _Br10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Br10W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Reset the corresponding ODRx bit."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(Br15W::Reset)
    }
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
#[doc = "Values that can be written to the field `BR9`"]
pub type Br9W = Br15W;
#[doc = r" Proxy"]
pub struct _Br9W<'a> {
    w: &'a mut W,
}
impl<'a> _Br9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Br9W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Reset the corresponding ODRx bit."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(Br15W::Reset)
    }
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
#[doc = "Values that can be written to the field `BR8`"]
pub type Br8W = Br15W;
#[doc = r" Proxy"]
pub struct _Br8W<'a> {
    w: &'a mut W,
}
impl<'a> _Br8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Br8W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Reset the corresponding ODRx bit."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(Br15W::Reset)
    }
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
#[doc = "Values that can be written to the field `BR7`"]
pub type Br7W = Br15W;
#[doc = r" Proxy"]
pub struct _Br7W<'a> {
    w: &'a mut W,
}
impl<'a> _Br7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Br7W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Reset the corresponding ODRx bit."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(Br15W::Reset)
    }
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
#[doc = "Values that can be written to the field `BR6`"]
pub type Br6W = Br15W;
#[doc = r" Proxy"]
pub struct _Br6W<'a> {
    w: &'a mut W,
}
impl<'a> _Br6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Br6W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Reset the corresponding ODRx bit."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(Br15W::Reset)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BR5`"]
pub type Br5W = Br15W;
#[doc = r" Proxy"]
pub struct _Br5W<'a> {
    w: &'a mut W,
}
impl<'a> _Br5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Br5W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Reset the corresponding ODRx bit."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(Br15W::Reset)
    }
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
#[doc = "Values that can be written to the field `BR4`"]
pub type Br4W = Br15W;
#[doc = r" Proxy"]
pub struct _Br4W<'a> {
    w: &'a mut W,
}
impl<'a> _Br4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Br4W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Reset the corresponding ODRx bit."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(Br15W::Reset)
    }
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
#[doc = "Values that can be written to the field `BR3`"]
pub type Br3W = Br15W;
#[doc = r" Proxy"]
pub struct _Br3W<'a> {
    w: &'a mut W,
}
impl<'a> _Br3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Br3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Reset the corresponding ODRx bit."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(Br15W::Reset)
    }
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
#[doc = "Values that can be written to the field `BR2`"]
pub type Br2W = Br15W;
#[doc = r" Proxy"]
pub struct _Br2W<'a> {
    w: &'a mut W,
}
impl<'a> _Br2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Br2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Reset the corresponding ODRx bit."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(Br15W::Reset)
    }
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
#[doc = "Values that can be written to the field `BR1`"]
pub type Br1W = Br15W;
#[doc = r" Proxy"]
pub struct _Br1W<'a> {
    w: &'a mut W,
}
impl<'a> _Br1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Br1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Reset the corresponding ODRx bit."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(Br15W::Reset)
    }
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
#[doc = "Values that can be written to the field `BR0`"]
pub type Br0W = Br15W;
#[doc = r" Proxy"]
pub struct _Br0W<'a> {
    w: &'a mut W,
}
impl<'a> _Br0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Br0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Reset the corresponding ODRx bit."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(Br15W::Reset)
    }
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
#[doc = "Values that can be written to the field `BS15`"]
pub enum Bs15W {
    #[doc = "Set the corresponding ODRx bit."] Set,
}
impl Bs15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            Bs15W::Set => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _Bs15W<'a> {
    w: &'a mut W,
}
impl<'a> _Bs15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Bs15W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set the corresponding ODRx bit."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(Bs15W::Set)
    }
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
#[doc = "Values that can be written to the field `BS14`"]
pub type Bs14W = Bs15W;
#[doc = r" Proxy"]
pub struct _Bs14W<'a> {
    w: &'a mut W,
}
impl<'a> _Bs14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Bs14W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set the corresponding ODRx bit."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(Bs15W::Set)
    }
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
#[doc = "Values that can be written to the field `BS13`"]
pub type Bs13W = Bs15W;
#[doc = r" Proxy"]
pub struct _Bs13W<'a> {
    w: &'a mut W,
}
impl<'a> _Bs13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Bs13W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set the corresponding ODRx bit."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(Bs15W::Set)
    }
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
#[doc = "Values that can be written to the field `BS12`"]
pub type Bs12W = Bs15W;
#[doc = r" Proxy"]
pub struct _Bs12W<'a> {
    w: &'a mut W,
}
impl<'a> _Bs12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Bs12W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set the corresponding ODRx bit."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(Bs15W::Set)
    }
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
#[doc = "Values that can be written to the field `BS11`"]
pub type Bs11W = Bs15W;
#[doc = r" Proxy"]
pub struct _Bs11W<'a> {
    w: &'a mut W,
}
impl<'a> _Bs11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Bs11W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set the corresponding ODRx bit."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(Bs15W::Set)
    }
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
#[doc = "Values that can be written to the field `BS10`"]
pub type Bs10W = Bs15W;
#[doc = r" Proxy"]
pub struct _Bs10W<'a> {
    w: &'a mut W,
}
impl<'a> _Bs10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Bs10W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set the corresponding ODRx bit."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(Bs15W::Set)
    }
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
#[doc = "Values that can be written to the field `BS9`"]
pub type Bs9W = Bs15W;
#[doc = r" Proxy"]
pub struct _Bs9W<'a> {
    w: &'a mut W,
}
impl<'a> _Bs9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Bs9W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set the corresponding ODRx bit."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(Bs15W::Set)
    }
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
#[doc = "Values that can be written to the field `BS8`"]
pub type Bs8W = Bs15W;
#[doc = r" Proxy"]
pub struct _Bs8W<'a> {
    w: &'a mut W,
}
impl<'a> _Bs8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Bs8W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set the corresponding ODRx bit."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(Bs15W::Set)
    }
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
#[doc = "Values that can be written to the field `BS7`"]
pub type Bs7W = Bs15W;
#[doc = r" Proxy"]
pub struct _Bs7W<'a> {
    w: &'a mut W,
}
impl<'a> _Bs7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Bs7W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set the corresponding ODRx bit."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(Bs15W::Set)
    }
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
#[doc = "Values that can be written to the field `BS6`"]
pub type Bs6W = Bs15W;
#[doc = r" Proxy"]
pub struct _Bs6W<'a> {
    w: &'a mut W,
}
impl<'a> _Bs6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Bs6W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set the corresponding ODRx bit."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(Bs15W::Set)
    }
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
#[doc = "Values that can be written to the field `BS5`"]
pub type Bs5W = Bs15W;
#[doc = r" Proxy"]
pub struct _Bs5W<'a> {
    w: &'a mut W,
}
impl<'a> _Bs5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Bs5W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set the corresponding ODRx bit."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(Bs15W::Set)
    }
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
#[doc = "Values that can be written to the field `BS4`"]
pub type Bs4W = Bs15W;
#[doc = r" Proxy"]
pub struct _Bs4W<'a> {
    w: &'a mut W,
}
impl<'a> _Bs4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Bs4W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set the corresponding ODRx bit."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(Bs15W::Set)
    }
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
#[doc = "Values that can be written to the field `BS3`"]
pub type Bs3W = Bs15W;
#[doc = r" Proxy"]
pub struct _Bs3W<'a> {
    w: &'a mut W,
}
impl<'a> _Bs3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Bs3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set the corresponding ODRx bit."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(Bs15W::Set)
    }
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
#[doc = "Values that can be written to the field `BS2`"]
pub type Bs2W = Bs15W;
#[doc = r" Proxy"]
pub struct _Bs2W<'a> {
    w: &'a mut W,
}
impl<'a> _Bs2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Bs2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set the corresponding ODRx bit."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(Bs15W::Set)
    }
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
#[doc = "Values that can be written to the field `BS1`"]
pub type Bs1W = Bs15W;
#[doc = r" Proxy"]
pub struct _Bs1W<'a> {
    w: &'a mut W,
}
impl<'a> _Bs1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Bs1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set the corresponding ODRx bit."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(Bs15W::Set)
    }
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
#[doc = "Values that can be written to the field `BS0`"]
pub type Bs0W = Bs15W;
#[doc = r" Proxy"]
pub struct _Bs0W<'a> {
    w: &'a mut W,
}
impl<'a> _Bs0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Bs0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set the corresponding ODRx bit."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(Bs15W::Set)
    }
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
    #[doc = "Bit 31 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br15(&mut self) -> _Br15W {
        _Br15W { w: self }
    }
    #[doc = "Bit 30 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br14(&mut self) -> _Br14W {
        _Br14W { w: self }
    }
    #[doc = "Bit 29 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br13(&mut self) -> _Br13W {
        _Br13W { w: self }
    }
    #[doc = "Bit 28 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br12(&mut self) -> _Br12W {
        _Br12W { w: self }
    }
    #[doc = "Bit 27 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br11(&mut self) -> _Br11W {
        _Br11W { w: self }
    }
    #[doc = "Bit 26 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br10(&mut self) -> _Br10W {
        _Br10W { w: self }
    }
    #[doc = "Bit 25 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br9(&mut self) -> _Br9W {
        _Br9W { w: self }
    }
    #[doc = "Bit 24 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br8(&mut self) -> _Br8W {
        _Br8W { w: self }
    }
    #[doc = "Bit 23 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br7(&mut self) -> _Br7W {
        _Br7W { w: self }
    }
    #[doc = "Bit 22 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br6(&mut self) -> _Br6W {
        _Br6W { w: self }
    }
    #[doc = "Bit 21 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br5(&mut self) -> _Br5W {
        _Br5W { w: self }
    }
    #[doc = "Bit 20 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br4(&mut self) -> _Br4W {
        _Br4W { w: self }
    }
    #[doc = "Bit 19 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br3(&mut self) -> _Br3W {
        _Br3W { w: self }
    }
    #[doc = "Bit 18 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br2(&mut self) -> _Br2W {
        _Br2W { w: self }
    }
    #[doc = "Bit 17 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br1(&mut self) -> _Br1W {
        _Br1W { w: self }
    }
    #[doc = "Bit 16 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn br0(&mut self) -> _Br0W {
        _Br0W { w: self }
    }
    #[doc = "Bit 15 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs15(&mut self) -> _Bs15W {
        _Bs15W { w: self }
    }
    #[doc = "Bit 14 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs14(&mut self) -> _Bs14W {
        _Bs14W { w: self }
    }
    #[doc = "Bit 13 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs13(&mut self) -> _Bs13W {
        _Bs13W { w: self }
    }
    #[doc = "Bit 12 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs12(&mut self) -> _Bs12W {
        _Bs12W { w: self }
    }
    #[doc = "Bit 11 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs11(&mut self) -> _Bs11W {
        _Bs11W { w: self }
    }
    #[doc = "Bit 10 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs10(&mut self) -> _Bs10W {
        _Bs10W { w: self }
    }
    #[doc = "Bit 9 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs9(&mut self) -> _Bs9W {
        _Bs9W { w: self }
    }
    #[doc = "Bit 8 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs8(&mut self) -> _Bs8W {
        _Bs8W { w: self }
    }
    #[doc = "Bit 7 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs7(&mut self) -> _Bs7W {
        _Bs7W { w: self }
    }
    #[doc = "Bit 6 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs6(&mut self) -> _Bs6W {
        _Bs6W { w: self }
    }
    #[doc = "Bit 5 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs5(&mut self) -> _Bs5W {
        _Bs5W { w: self }
    }
    #[doc = "Bit 4 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs4(&mut self) -> _Bs4W {
        _Bs4W { w: self }
    }
    #[doc = "Bit 3 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs3(&mut self) -> _Bs3W {
        _Bs3W { w: self }
    }
    #[doc = "Bit 2 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs2(&mut self) -> _Bs2W {
        _Bs2W { w: self }
    }
    #[doc = "Bit 1 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs1(&mut self) -> _Bs1W {
        _Bs1W { w: self }
    }
    #[doc = "Bit 0 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs0(&mut self) -> _Bs0W {
        _Bs0W { w: self }
    }
}
