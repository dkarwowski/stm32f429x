#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::F8r2 {
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
pub struct Fb0R {
    bits: u8,
}
impl Fb0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fb1R {
    bits: u8,
}
impl Fb1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fb2R {
    bits: u8,
}
impl Fb2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fb3R {
    bits: u8,
}
impl Fb3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fb4R {
    bits: u8,
}
impl Fb4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fb5R {
    bits: u8,
}
impl Fb5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fb6R {
    bits: u8,
}
impl Fb6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fb7R {
    bits: u8,
}
impl Fb7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fb8R {
    bits: u8,
}
impl Fb8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fb9R {
    bits: u8,
}
impl Fb9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fb10R {
    bits: u8,
}
impl Fb10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fb11R {
    bits: u8,
}
impl Fb11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fb12R {
    bits: u8,
}
impl Fb12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fb13R {
    bits: u8,
}
impl Fb13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fb14R {
    bits: u8,
}
impl Fb14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fb15R {
    bits: u8,
}
impl Fb15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fb16R {
    bits: u8,
}
impl Fb16R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fb17R {
    bits: u8,
}
impl Fb17R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fb18R {
    bits: u8,
}
impl Fb18R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fb19R {
    bits: u8,
}
impl Fb19R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fb20R {
    bits: u8,
}
impl Fb20R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fb21R {
    bits: u8,
}
impl Fb21R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fb22R {
    bits: u8,
}
impl Fb22R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fb23R {
    bits: u8,
}
impl Fb23R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fb24R {
    bits: u8,
}
impl Fb24R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fb25R {
    bits: u8,
}
impl Fb25R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fb26R {
    bits: u8,
}
impl Fb26R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fb27R {
    bits: u8,
}
impl Fb27R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fb28R {
    bits: u8,
}
impl Fb28R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fb29R {
    bits: u8,
}
impl Fb29R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fb30R {
    bits: u8,
}
impl Fb30R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fb31R {
    bits: u8,
}
impl Fb31R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _Fb0W<'a> {
    w: &'a mut W,
}
impl<'a> _Fb0W<'a> {
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
pub struct _Fb1W<'a> {
    w: &'a mut W,
}
impl<'a> _Fb1W<'a> {
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
pub struct _Fb2W<'a> {
    w: &'a mut W,
}
impl<'a> _Fb2W<'a> {
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
pub struct _Fb3W<'a> {
    w: &'a mut W,
}
impl<'a> _Fb3W<'a> {
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
pub struct _Fb4W<'a> {
    w: &'a mut W,
}
impl<'a> _Fb4W<'a> {
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
pub struct _Fb5W<'a> {
    w: &'a mut W,
}
impl<'a> _Fb5W<'a> {
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
pub struct _Fb6W<'a> {
    w: &'a mut W,
}
impl<'a> _Fb6W<'a> {
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
pub struct _Fb7W<'a> {
    w: &'a mut W,
}
impl<'a> _Fb7W<'a> {
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
pub struct _Fb8W<'a> {
    w: &'a mut W,
}
impl<'a> _Fb8W<'a> {
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
pub struct _Fb9W<'a> {
    w: &'a mut W,
}
impl<'a> _Fb9W<'a> {
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
pub struct _Fb10W<'a> {
    w: &'a mut W,
}
impl<'a> _Fb10W<'a> {
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
pub struct _Fb11W<'a> {
    w: &'a mut W,
}
impl<'a> _Fb11W<'a> {
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
pub struct _Fb12W<'a> {
    w: &'a mut W,
}
impl<'a> _Fb12W<'a> {
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
pub struct _Fb13W<'a> {
    w: &'a mut W,
}
impl<'a> _Fb13W<'a> {
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
pub struct _Fb14W<'a> {
    w: &'a mut W,
}
impl<'a> _Fb14W<'a> {
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
pub struct _Fb15W<'a> {
    w: &'a mut W,
}
impl<'a> _Fb15W<'a> {
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
pub struct _Fb16W<'a> {
    w: &'a mut W,
}
impl<'a> _Fb16W<'a> {
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
pub struct _Fb17W<'a> {
    w: &'a mut W,
}
impl<'a> _Fb17W<'a> {
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
pub struct _Fb18W<'a> {
    w: &'a mut W,
}
impl<'a> _Fb18W<'a> {
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
pub struct _Fb19W<'a> {
    w: &'a mut W,
}
impl<'a> _Fb19W<'a> {
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
pub struct _Fb20W<'a> {
    w: &'a mut W,
}
impl<'a> _Fb20W<'a> {
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
pub struct _Fb21W<'a> {
    w: &'a mut W,
}
impl<'a> _Fb21W<'a> {
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
pub struct _Fb22W<'a> {
    w: &'a mut W,
}
impl<'a> _Fb22W<'a> {
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
pub struct _Fb23W<'a> {
    w: &'a mut W,
}
impl<'a> _Fb23W<'a> {
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
pub struct _Fb24W<'a> {
    w: &'a mut W,
}
impl<'a> _Fb24W<'a> {
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
pub struct _Fb25W<'a> {
    w: &'a mut W,
}
impl<'a> _Fb25W<'a> {
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
pub struct _Fb26W<'a> {
    w: &'a mut W,
}
impl<'a> _Fb26W<'a> {
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
pub struct _Fb27W<'a> {
    w: &'a mut W,
}
impl<'a> _Fb27W<'a> {
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
#[doc = r" Proxy"]
pub struct _Fb28W<'a> {
    w: &'a mut W,
}
impl<'a> _Fb28W<'a> {
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
pub struct _Fb29W<'a> {
    w: &'a mut W,
}
impl<'a> _Fb29W<'a> {
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
pub struct _Fb30W<'a> {
    w: &'a mut W,
}
impl<'a> _Fb30W<'a> {
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
pub struct _Fb31W<'a> {
    w: &'a mut W,
}
impl<'a> _Fb31W<'a> {
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
    #[doc = "Bit 0 - Filter bits"]
    #[inline(always)]
    pub fn fb0(&self) -> Fb0R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fb0R { bits }
    }
    #[doc = "Bit 1 - Filter bits"]
    #[inline(always)]
    pub fn fb1(&self) -> Fb1R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fb1R { bits }
    }
    #[doc = "Bit 2 - Filter bits"]
    #[inline(always)]
    pub fn fb2(&self) -> Fb2R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fb2R { bits }
    }
    #[doc = "Bit 3 - Filter bits"]
    #[inline(always)]
    pub fn fb3(&self) -> Fb3R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fb3R { bits }
    }
    #[doc = "Bit 4 - Filter bits"]
    #[inline(always)]
    pub fn fb4(&self) -> Fb4R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fb4R { bits }
    }
    #[doc = "Bit 5 - Filter bits"]
    #[inline(always)]
    pub fn fb5(&self) -> Fb5R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fb5R { bits }
    }
    #[doc = "Bit 6 - Filter bits"]
    #[inline(always)]
    pub fn fb6(&self) -> Fb6R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fb6R { bits }
    }
    #[doc = "Bit 7 - Filter bits"]
    #[inline(always)]
    pub fn fb7(&self) -> Fb7R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fb7R { bits }
    }
    #[doc = "Bit 8 - Filter bits"]
    #[inline(always)]
    pub fn fb8(&self) -> Fb8R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fb8R { bits }
    }
    #[doc = "Bit 9 - Filter bits"]
    #[inline(always)]
    pub fn fb9(&self) -> Fb9R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fb9R { bits }
    }
    #[doc = "Bit 10 - Filter bits"]
    #[inline(always)]
    pub fn fb10(&self) -> Fb10R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fb10R { bits }
    }
    #[doc = "Bit 11 - Filter bits"]
    #[inline(always)]
    pub fn fb11(&self) -> Fb11R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fb11R { bits }
    }
    #[doc = "Bit 12 - Filter bits"]
    #[inline(always)]
    pub fn fb12(&self) -> Fb12R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fb12R { bits }
    }
    #[doc = "Bit 13 - Filter bits"]
    #[inline(always)]
    pub fn fb13(&self) -> Fb13R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fb13R { bits }
    }
    #[doc = "Bit 14 - Filter bits"]
    #[inline(always)]
    pub fn fb14(&self) -> Fb14R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fb14R { bits }
    }
    #[doc = "Bit 15 - Filter bits"]
    #[inline(always)]
    pub fn fb15(&self) -> Fb15R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fb15R { bits }
    }
    #[doc = "Bit 16 - Filter bits"]
    #[inline(always)]
    pub fn fb16(&self) -> Fb16R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fb16R { bits }
    }
    #[doc = "Bit 17 - Filter bits"]
    #[inline(always)]
    pub fn fb17(&self) -> Fb17R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fb17R { bits }
    }
    #[doc = "Bit 18 - Filter bits"]
    #[inline(always)]
    pub fn fb18(&self) -> Fb18R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fb18R { bits }
    }
    #[doc = "Bit 19 - Filter bits"]
    #[inline(always)]
    pub fn fb19(&self) -> Fb19R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fb19R { bits }
    }
    #[doc = "Bit 20 - Filter bits"]
    #[inline(always)]
    pub fn fb20(&self) -> Fb20R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fb20R { bits }
    }
    #[doc = "Bit 21 - Filter bits"]
    #[inline(always)]
    pub fn fb21(&self) -> Fb21R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fb21R { bits }
    }
    #[doc = "Bit 22 - Filter bits"]
    #[inline(always)]
    pub fn fb22(&self) -> Fb22R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fb22R { bits }
    }
    #[doc = "Bit 23 - Filter bits"]
    #[inline(always)]
    pub fn fb23(&self) -> Fb23R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fb23R { bits }
    }
    #[doc = "Bit 24 - Filter bits"]
    #[inline(always)]
    pub fn fb24(&self) -> Fb24R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fb24R { bits }
    }
    #[doc = "Bit 25 - Filter bits"]
    #[inline(always)]
    pub fn fb25(&self) -> Fb25R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fb25R { bits }
    }
    #[doc = "Bit 26 - Filter bits"]
    #[inline(always)]
    pub fn fb26(&self) -> Fb26R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fb26R { bits }
    }
    #[doc = "Bit 27 - Filter bits"]
    #[inline(always)]
    pub fn fb27(&self) -> Fb27R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fb27R { bits }
    }
    #[doc = "Bit 28 - Filter bits"]
    #[inline(always)]
    pub fn fb28(&self) -> Fb28R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fb28R { bits }
    }
    #[doc = "Bit 29 - Filter bits"]
    #[inline(always)]
    pub fn fb29(&self) -> Fb29R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fb29R { bits }
    }
    #[doc = "Bit 30 - Filter bits"]
    #[inline(always)]
    pub fn fb30(&self) -> Fb30R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fb30R { bits }
    }
    #[doc = "Bit 31 - Filter bits"]
    #[inline(always)]
    pub fn fb31(&self) -> Fb31R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fb31R { bits }
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
    #[doc = "Bit 0 - Filter bits"]
    #[inline(always)]
    pub fn fb0(&mut self) -> _Fb0W {
        _Fb0W { w: self }
    }
    #[doc = "Bit 1 - Filter bits"]
    #[inline(always)]
    pub fn fb1(&mut self) -> _Fb1W {
        _Fb1W { w: self }
    }
    #[doc = "Bit 2 - Filter bits"]
    #[inline(always)]
    pub fn fb2(&mut self) -> _Fb2W {
        _Fb2W { w: self }
    }
    #[doc = "Bit 3 - Filter bits"]
    #[inline(always)]
    pub fn fb3(&mut self) -> _Fb3W {
        _Fb3W { w: self }
    }
    #[doc = "Bit 4 - Filter bits"]
    #[inline(always)]
    pub fn fb4(&mut self) -> _Fb4W {
        _Fb4W { w: self }
    }
    #[doc = "Bit 5 - Filter bits"]
    #[inline(always)]
    pub fn fb5(&mut self) -> _Fb5W {
        _Fb5W { w: self }
    }
    #[doc = "Bit 6 - Filter bits"]
    #[inline(always)]
    pub fn fb6(&mut self) -> _Fb6W {
        _Fb6W { w: self }
    }
    #[doc = "Bit 7 - Filter bits"]
    #[inline(always)]
    pub fn fb7(&mut self) -> _Fb7W {
        _Fb7W { w: self }
    }
    #[doc = "Bit 8 - Filter bits"]
    #[inline(always)]
    pub fn fb8(&mut self) -> _Fb8W {
        _Fb8W { w: self }
    }
    #[doc = "Bit 9 - Filter bits"]
    #[inline(always)]
    pub fn fb9(&mut self) -> _Fb9W {
        _Fb9W { w: self }
    }
    #[doc = "Bit 10 - Filter bits"]
    #[inline(always)]
    pub fn fb10(&mut self) -> _Fb10W {
        _Fb10W { w: self }
    }
    #[doc = "Bit 11 - Filter bits"]
    #[inline(always)]
    pub fn fb11(&mut self) -> _Fb11W {
        _Fb11W { w: self }
    }
    #[doc = "Bit 12 - Filter bits"]
    #[inline(always)]
    pub fn fb12(&mut self) -> _Fb12W {
        _Fb12W { w: self }
    }
    #[doc = "Bit 13 - Filter bits"]
    #[inline(always)]
    pub fn fb13(&mut self) -> _Fb13W {
        _Fb13W { w: self }
    }
    #[doc = "Bit 14 - Filter bits"]
    #[inline(always)]
    pub fn fb14(&mut self) -> _Fb14W {
        _Fb14W { w: self }
    }
    #[doc = "Bit 15 - Filter bits"]
    #[inline(always)]
    pub fn fb15(&mut self) -> _Fb15W {
        _Fb15W { w: self }
    }
    #[doc = "Bit 16 - Filter bits"]
    #[inline(always)]
    pub fn fb16(&mut self) -> _Fb16W {
        _Fb16W { w: self }
    }
    #[doc = "Bit 17 - Filter bits"]
    #[inline(always)]
    pub fn fb17(&mut self) -> _Fb17W {
        _Fb17W { w: self }
    }
    #[doc = "Bit 18 - Filter bits"]
    #[inline(always)]
    pub fn fb18(&mut self) -> _Fb18W {
        _Fb18W { w: self }
    }
    #[doc = "Bit 19 - Filter bits"]
    #[inline(always)]
    pub fn fb19(&mut self) -> _Fb19W {
        _Fb19W { w: self }
    }
    #[doc = "Bit 20 - Filter bits"]
    #[inline(always)]
    pub fn fb20(&mut self) -> _Fb20W {
        _Fb20W { w: self }
    }
    #[doc = "Bit 21 - Filter bits"]
    #[inline(always)]
    pub fn fb21(&mut self) -> _Fb21W {
        _Fb21W { w: self }
    }
    #[doc = "Bit 22 - Filter bits"]
    #[inline(always)]
    pub fn fb22(&mut self) -> _Fb22W {
        _Fb22W { w: self }
    }
    #[doc = "Bit 23 - Filter bits"]
    #[inline(always)]
    pub fn fb23(&mut self) -> _Fb23W {
        _Fb23W { w: self }
    }
    #[doc = "Bit 24 - Filter bits"]
    #[inline(always)]
    pub fn fb24(&mut self) -> _Fb24W {
        _Fb24W { w: self }
    }
    #[doc = "Bit 25 - Filter bits"]
    #[inline(always)]
    pub fn fb25(&mut self) -> _Fb25W {
        _Fb25W { w: self }
    }
    #[doc = "Bit 26 - Filter bits"]
    #[inline(always)]
    pub fn fb26(&mut self) -> _Fb26W {
        _Fb26W { w: self }
    }
    #[doc = "Bit 27 - Filter bits"]
    #[inline(always)]
    pub fn fb27(&mut self) -> _Fb27W {
        _Fb27W { w: self }
    }
    #[doc = "Bit 28 - Filter bits"]
    #[inline(always)]
    pub fn fb28(&mut self) -> _Fb28W {
        _Fb28W { w: self }
    }
    #[doc = "Bit 29 - Filter bits"]
    #[inline(always)]
    pub fn fb29(&mut self) -> _Fb29W {
        _Fb29W { w: self }
    }
    #[doc = "Bit 30 - Filter bits"]
    #[inline(always)]
    pub fn fb30(&mut self) -> _Fb30W {
        _Fb30W { w: self }
    }
    #[doc = "Bit 31 - Filter bits"]
    #[inline(always)]
    pub fn fb31(&mut self) -> _Fb31W {
        _Fb31W { w: self }
    }
}
