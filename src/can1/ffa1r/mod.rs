#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::Ffa1r {
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
pub struct Ffa0R {
    bits: u8,
}
impl Ffa0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Ffa1R {
    bits: u8,
}
impl Ffa1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Ffa2R {
    bits: u8,
}
impl Ffa2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Ffa3R {
    bits: u8,
}
impl Ffa3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Ffa4R {
    bits: u8,
}
impl Ffa4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Ffa5R {
    bits: u8,
}
impl Ffa5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Ffa6R {
    bits: u8,
}
impl Ffa6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Ffa7R {
    bits: u8,
}
impl Ffa7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Ffa8R {
    bits: u8,
}
impl Ffa8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Ffa9R {
    bits: u8,
}
impl Ffa9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Ffa10R {
    bits: u8,
}
impl Ffa10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Ffa11R {
    bits: u8,
}
impl Ffa11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Ffa12R {
    bits: u8,
}
impl Ffa12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Ffa13R {
    bits: u8,
}
impl Ffa13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Ffa14R {
    bits: u8,
}
impl Ffa14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Ffa15R {
    bits: u8,
}
impl Ffa15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Ffa16R {
    bits: u8,
}
impl Ffa16R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Ffa17R {
    bits: u8,
}
impl Ffa17R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Ffa18R {
    bits: u8,
}
impl Ffa18R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Ffa19R {
    bits: u8,
}
impl Ffa19R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Ffa20R {
    bits: u8,
}
impl Ffa20R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Ffa21R {
    bits: u8,
}
impl Ffa21R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Ffa22R {
    bits: u8,
}
impl Ffa22R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Ffa23R {
    bits: u8,
}
impl Ffa23R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Ffa24R {
    bits: u8,
}
impl Ffa24R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Ffa25R {
    bits: u8,
}
impl Ffa25R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Ffa26R {
    bits: u8,
}
impl Ffa26R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Ffa27R {
    bits: u8,
}
impl Ffa27R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _Ffa0W<'a> {
    w: &'a mut W,
}
impl<'a> _Ffa0W<'a> {
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
pub struct _Ffa1W<'a> {
    w: &'a mut W,
}
impl<'a> _Ffa1W<'a> {
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
pub struct _Ffa2W<'a> {
    w: &'a mut W,
}
impl<'a> _Ffa2W<'a> {
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
pub struct _Ffa3W<'a> {
    w: &'a mut W,
}
impl<'a> _Ffa3W<'a> {
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
pub struct _Ffa4W<'a> {
    w: &'a mut W,
}
impl<'a> _Ffa4W<'a> {
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
pub struct _Ffa5W<'a> {
    w: &'a mut W,
}
impl<'a> _Ffa5W<'a> {
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
pub struct _Ffa6W<'a> {
    w: &'a mut W,
}
impl<'a> _Ffa6W<'a> {
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
pub struct _Ffa7W<'a> {
    w: &'a mut W,
}
impl<'a> _Ffa7W<'a> {
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
pub struct _Ffa8W<'a> {
    w: &'a mut W,
}
impl<'a> _Ffa8W<'a> {
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
pub struct _Ffa9W<'a> {
    w: &'a mut W,
}
impl<'a> _Ffa9W<'a> {
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
pub struct _Ffa10W<'a> {
    w: &'a mut W,
}
impl<'a> _Ffa10W<'a> {
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
pub struct _Ffa11W<'a> {
    w: &'a mut W,
}
impl<'a> _Ffa11W<'a> {
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
pub struct _Ffa12W<'a> {
    w: &'a mut W,
}
impl<'a> _Ffa12W<'a> {
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
pub struct _Ffa13W<'a> {
    w: &'a mut W,
}
impl<'a> _Ffa13W<'a> {
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
pub struct _Ffa14W<'a> {
    w: &'a mut W,
}
impl<'a> _Ffa14W<'a> {
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
pub struct _Ffa15W<'a> {
    w: &'a mut W,
}
impl<'a> _Ffa15W<'a> {
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
pub struct _Ffa16W<'a> {
    w: &'a mut W,
}
impl<'a> _Ffa16W<'a> {
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
pub struct _Ffa17W<'a> {
    w: &'a mut W,
}
impl<'a> _Ffa17W<'a> {
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
pub struct _Ffa18W<'a> {
    w: &'a mut W,
}
impl<'a> _Ffa18W<'a> {
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
pub struct _Ffa19W<'a> {
    w: &'a mut W,
}
impl<'a> _Ffa19W<'a> {
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
pub struct _Ffa20W<'a> {
    w: &'a mut W,
}
impl<'a> _Ffa20W<'a> {
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
pub struct _Ffa21W<'a> {
    w: &'a mut W,
}
impl<'a> _Ffa21W<'a> {
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
pub struct _Ffa22W<'a> {
    w: &'a mut W,
}
impl<'a> _Ffa22W<'a> {
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
#[doc = r" Proxy"]
pub struct _Ffa23W<'a> {
    w: &'a mut W,
}
impl<'a> _Ffa23W<'a> {
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
pub struct _Ffa24W<'a> {
    w: &'a mut W,
}
impl<'a> _Ffa24W<'a> {
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
pub struct _Ffa25W<'a> {
    w: &'a mut W,
}
impl<'a> _Ffa25W<'a> {
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
pub struct _Ffa26W<'a> {
    w: &'a mut W,
}
impl<'a> _Ffa26W<'a> {
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
pub struct _Ffa27W<'a> {
    w: &'a mut W,
}
impl<'a> _Ffa27W<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Filter FIFO assignment for filter 0"]
    #[inline(always)]
    pub fn ffa0(&self) -> Ffa0R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Ffa0R { bits }
    }
    #[doc = "Bit 1 - Filter FIFO assignment for filter 1"]
    #[inline(always)]
    pub fn ffa1(&self) -> Ffa1R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Ffa1R { bits }
    }
    #[doc = "Bit 2 - Filter FIFO assignment for filter 2"]
    #[inline(always)]
    pub fn ffa2(&self) -> Ffa2R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Ffa2R { bits }
    }
    #[doc = "Bit 3 - Filter FIFO assignment for filter 3"]
    #[inline(always)]
    pub fn ffa3(&self) -> Ffa3R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Ffa3R { bits }
    }
    #[doc = "Bit 4 - Filter FIFO assignment for filter 4"]
    #[inline(always)]
    pub fn ffa4(&self) -> Ffa4R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Ffa4R { bits }
    }
    #[doc = "Bit 5 - Filter FIFO assignment for filter 5"]
    #[inline(always)]
    pub fn ffa5(&self) -> Ffa5R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Ffa5R { bits }
    }
    #[doc = "Bit 6 - Filter FIFO assignment for filter 6"]
    #[inline(always)]
    pub fn ffa6(&self) -> Ffa6R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Ffa6R { bits }
    }
    #[doc = "Bit 7 - Filter FIFO assignment for filter 7"]
    #[inline(always)]
    pub fn ffa7(&self) -> Ffa7R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Ffa7R { bits }
    }
    #[doc = "Bit 8 - Filter FIFO assignment for filter 8"]
    #[inline(always)]
    pub fn ffa8(&self) -> Ffa8R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Ffa8R { bits }
    }
    #[doc = "Bit 9 - Filter FIFO assignment for filter 9"]
    #[inline(always)]
    pub fn ffa9(&self) -> Ffa9R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Ffa9R { bits }
    }
    #[doc = "Bit 10 - Filter FIFO assignment for filter 10"]
    #[inline(always)]
    pub fn ffa10(&self) -> Ffa10R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Ffa10R { bits }
    }
    #[doc = "Bit 11 - Filter FIFO assignment for filter 11"]
    #[inline(always)]
    pub fn ffa11(&self) -> Ffa11R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Ffa11R { bits }
    }
    #[doc = "Bit 12 - Filter FIFO assignment for filter 12"]
    #[inline(always)]
    pub fn ffa12(&self) -> Ffa12R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Ffa12R { bits }
    }
    #[doc = "Bit 13 - Filter FIFO assignment for filter 13"]
    #[inline(always)]
    pub fn ffa13(&self) -> Ffa13R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Ffa13R { bits }
    }
    #[doc = "Bit 14 - Filter FIFO assignment for filter 14"]
    #[inline(always)]
    pub fn ffa14(&self) -> Ffa14R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Ffa14R { bits }
    }
    #[doc = "Bit 15 - Filter FIFO assignment for filter 15"]
    #[inline(always)]
    pub fn ffa15(&self) -> Ffa15R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Ffa15R { bits }
    }
    #[doc = "Bit 16 - Filter FIFO assignment for filter 16"]
    #[inline(always)]
    pub fn ffa16(&self) -> Ffa16R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Ffa16R { bits }
    }
    #[doc = "Bit 17 - Filter FIFO assignment for filter 17"]
    #[inline(always)]
    pub fn ffa17(&self) -> Ffa17R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Ffa17R { bits }
    }
    #[doc = "Bit 18 - Filter FIFO assignment for filter 18"]
    #[inline(always)]
    pub fn ffa18(&self) -> Ffa18R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Ffa18R { bits }
    }
    #[doc = "Bit 19 - Filter FIFO assignment for filter 19"]
    #[inline(always)]
    pub fn ffa19(&self) -> Ffa19R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Ffa19R { bits }
    }
    #[doc = "Bit 20 - Filter FIFO assignment for filter 20"]
    #[inline(always)]
    pub fn ffa20(&self) -> Ffa20R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Ffa20R { bits }
    }
    #[doc = "Bit 21 - Filter FIFO assignment for filter 21"]
    #[inline(always)]
    pub fn ffa21(&self) -> Ffa21R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Ffa21R { bits }
    }
    #[doc = "Bit 22 - Filter FIFO assignment for filter 22"]
    #[inline(always)]
    pub fn ffa22(&self) -> Ffa22R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Ffa22R { bits }
    }
    #[doc = "Bit 23 - Filter FIFO assignment for filter 23"]
    #[inline(always)]
    pub fn ffa23(&self) -> Ffa23R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Ffa23R { bits }
    }
    #[doc = "Bit 24 - Filter FIFO assignment for filter 24"]
    #[inline(always)]
    pub fn ffa24(&self) -> Ffa24R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Ffa24R { bits }
    }
    #[doc = "Bit 25 - Filter FIFO assignment for filter 25"]
    #[inline(always)]
    pub fn ffa25(&self) -> Ffa25R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Ffa25R { bits }
    }
    #[doc = "Bit 26 - Filter FIFO assignment for filter 26"]
    #[inline(always)]
    pub fn ffa26(&self) -> Ffa26R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Ffa26R { bits }
    }
    #[doc = "Bit 27 - Filter FIFO assignment for filter 27"]
    #[inline(always)]
    pub fn ffa27(&self) -> Ffa27R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Ffa27R { bits }
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
    #[doc = "Bit 0 - Filter FIFO assignment for filter 0"]
    #[inline(always)]
    pub fn ffa0(&mut self) -> _Ffa0W {
        _Ffa0W { w: self }
    }
    #[doc = "Bit 1 - Filter FIFO assignment for filter 1"]
    #[inline(always)]
    pub fn ffa1(&mut self) -> _Ffa1W {
        _Ffa1W { w: self }
    }
    #[doc = "Bit 2 - Filter FIFO assignment for filter 2"]
    #[inline(always)]
    pub fn ffa2(&mut self) -> _Ffa2W {
        _Ffa2W { w: self }
    }
    #[doc = "Bit 3 - Filter FIFO assignment for filter 3"]
    #[inline(always)]
    pub fn ffa3(&mut self) -> _Ffa3W {
        _Ffa3W { w: self }
    }
    #[doc = "Bit 4 - Filter FIFO assignment for filter 4"]
    #[inline(always)]
    pub fn ffa4(&mut self) -> _Ffa4W {
        _Ffa4W { w: self }
    }
    #[doc = "Bit 5 - Filter FIFO assignment for filter 5"]
    #[inline(always)]
    pub fn ffa5(&mut self) -> _Ffa5W {
        _Ffa5W { w: self }
    }
    #[doc = "Bit 6 - Filter FIFO assignment for filter 6"]
    #[inline(always)]
    pub fn ffa6(&mut self) -> _Ffa6W {
        _Ffa6W { w: self }
    }
    #[doc = "Bit 7 - Filter FIFO assignment for filter 7"]
    #[inline(always)]
    pub fn ffa7(&mut self) -> _Ffa7W {
        _Ffa7W { w: self }
    }
    #[doc = "Bit 8 - Filter FIFO assignment for filter 8"]
    #[inline(always)]
    pub fn ffa8(&mut self) -> _Ffa8W {
        _Ffa8W { w: self }
    }
    #[doc = "Bit 9 - Filter FIFO assignment for filter 9"]
    #[inline(always)]
    pub fn ffa9(&mut self) -> _Ffa9W {
        _Ffa9W { w: self }
    }
    #[doc = "Bit 10 - Filter FIFO assignment for filter 10"]
    #[inline(always)]
    pub fn ffa10(&mut self) -> _Ffa10W {
        _Ffa10W { w: self }
    }
    #[doc = "Bit 11 - Filter FIFO assignment for filter 11"]
    #[inline(always)]
    pub fn ffa11(&mut self) -> _Ffa11W {
        _Ffa11W { w: self }
    }
    #[doc = "Bit 12 - Filter FIFO assignment for filter 12"]
    #[inline(always)]
    pub fn ffa12(&mut self) -> _Ffa12W {
        _Ffa12W { w: self }
    }
    #[doc = "Bit 13 - Filter FIFO assignment for filter 13"]
    #[inline(always)]
    pub fn ffa13(&mut self) -> _Ffa13W {
        _Ffa13W { w: self }
    }
    #[doc = "Bit 14 - Filter FIFO assignment for filter 14"]
    #[inline(always)]
    pub fn ffa14(&mut self) -> _Ffa14W {
        _Ffa14W { w: self }
    }
    #[doc = "Bit 15 - Filter FIFO assignment for filter 15"]
    #[inline(always)]
    pub fn ffa15(&mut self) -> _Ffa15W {
        _Ffa15W { w: self }
    }
    #[doc = "Bit 16 - Filter FIFO assignment for filter 16"]
    #[inline(always)]
    pub fn ffa16(&mut self) -> _Ffa16W {
        _Ffa16W { w: self }
    }
    #[doc = "Bit 17 - Filter FIFO assignment for filter 17"]
    #[inline(always)]
    pub fn ffa17(&mut self) -> _Ffa17W {
        _Ffa17W { w: self }
    }
    #[doc = "Bit 18 - Filter FIFO assignment for filter 18"]
    #[inline(always)]
    pub fn ffa18(&mut self) -> _Ffa18W {
        _Ffa18W { w: self }
    }
    #[doc = "Bit 19 - Filter FIFO assignment for filter 19"]
    #[inline(always)]
    pub fn ffa19(&mut self) -> _Ffa19W {
        _Ffa19W { w: self }
    }
    #[doc = "Bit 20 - Filter FIFO assignment for filter 20"]
    #[inline(always)]
    pub fn ffa20(&mut self) -> _Ffa20W {
        _Ffa20W { w: self }
    }
    #[doc = "Bit 21 - Filter FIFO assignment for filter 21"]
    #[inline(always)]
    pub fn ffa21(&mut self) -> _Ffa21W {
        _Ffa21W { w: self }
    }
    #[doc = "Bit 22 - Filter FIFO assignment for filter 22"]
    #[inline(always)]
    pub fn ffa22(&mut self) -> _Ffa22W {
        _Ffa22W { w: self }
    }
    #[doc = "Bit 23 - Filter FIFO assignment for filter 23"]
    #[inline(always)]
    pub fn ffa23(&mut self) -> _Ffa23W {
        _Ffa23W { w: self }
    }
    #[doc = "Bit 24 - Filter FIFO assignment for filter 24"]
    #[inline(always)]
    pub fn ffa24(&mut self) -> _Ffa24W {
        _Ffa24W { w: self }
    }
    #[doc = "Bit 25 - Filter FIFO assignment for filter 25"]
    #[inline(always)]
    pub fn ffa25(&mut self) -> _Ffa25W {
        _Ffa25W { w: self }
    }
    #[doc = "Bit 26 - Filter FIFO assignment for filter 26"]
    #[inline(always)]
    pub fn ffa26(&mut self) -> _Ffa26W {
        _Ffa26W { w: self }
    }
    #[doc = "Bit 27 - Filter FIFO assignment for filter 27"]
    #[inline(always)]
    pub fn ffa27(&mut self) -> _Ffa27W {
        _Ffa27W { w: self }
    }
}
