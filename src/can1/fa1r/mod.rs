#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::Fa1r {
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
pub struct Fact0R {
    bits: u8,
}
impl Fact0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fact1R {
    bits: u8,
}
impl Fact1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fact2R {
    bits: u8,
}
impl Fact2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fact3R {
    bits: u8,
}
impl Fact3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fact4R {
    bits: u8,
}
impl Fact4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fact5R {
    bits: u8,
}
impl Fact5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fact6R {
    bits: u8,
}
impl Fact6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fact7R {
    bits: u8,
}
impl Fact7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fact8R {
    bits: u8,
}
impl Fact8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fact9R {
    bits: u8,
}
impl Fact9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fact10R {
    bits: u8,
}
impl Fact10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fact11R {
    bits: u8,
}
impl Fact11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fact12R {
    bits: u8,
}
impl Fact12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fact13R {
    bits: u8,
}
impl Fact13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fact14R {
    bits: u8,
}
impl Fact14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fact15R {
    bits: u8,
}
impl Fact15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fact16R {
    bits: u8,
}
impl Fact16R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fact17R {
    bits: u8,
}
impl Fact17R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fact18R {
    bits: u8,
}
impl Fact18R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fact19R {
    bits: u8,
}
impl Fact19R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fact20R {
    bits: u8,
}
impl Fact20R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fact21R {
    bits: u8,
}
impl Fact21R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fact22R {
    bits: u8,
}
impl Fact22R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fact23R {
    bits: u8,
}
impl Fact23R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fact24R {
    bits: u8,
}
impl Fact24R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fact25R {
    bits: u8,
}
impl Fact25R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fact26R {
    bits: u8,
}
impl Fact26R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Fact27R {
    bits: u8,
}
impl Fact27R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _Fact0W<'a> {
    w: &'a mut W,
}
impl<'a> _Fact0W<'a> {
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
pub struct _Fact1W<'a> {
    w: &'a mut W,
}
impl<'a> _Fact1W<'a> {
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
pub struct _Fact2W<'a> {
    w: &'a mut W,
}
impl<'a> _Fact2W<'a> {
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
pub struct _Fact3W<'a> {
    w: &'a mut W,
}
impl<'a> _Fact3W<'a> {
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
pub struct _Fact4W<'a> {
    w: &'a mut W,
}
impl<'a> _Fact4W<'a> {
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
pub struct _Fact5W<'a> {
    w: &'a mut W,
}
impl<'a> _Fact5W<'a> {
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
pub struct _Fact6W<'a> {
    w: &'a mut W,
}
impl<'a> _Fact6W<'a> {
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
pub struct _Fact7W<'a> {
    w: &'a mut W,
}
impl<'a> _Fact7W<'a> {
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
pub struct _Fact8W<'a> {
    w: &'a mut W,
}
impl<'a> _Fact8W<'a> {
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
pub struct _Fact9W<'a> {
    w: &'a mut W,
}
impl<'a> _Fact9W<'a> {
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
pub struct _Fact10W<'a> {
    w: &'a mut W,
}
impl<'a> _Fact10W<'a> {
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
pub struct _Fact11W<'a> {
    w: &'a mut W,
}
impl<'a> _Fact11W<'a> {
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
pub struct _Fact12W<'a> {
    w: &'a mut W,
}
impl<'a> _Fact12W<'a> {
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
pub struct _Fact13W<'a> {
    w: &'a mut W,
}
impl<'a> _Fact13W<'a> {
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
pub struct _Fact14W<'a> {
    w: &'a mut W,
}
impl<'a> _Fact14W<'a> {
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
pub struct _Fact15W<'a> {
    w: &'a mut W,
}
impl<'a> _Fact15W<'a> {
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
pub struct _Fact16W<'a> {
    w: &'a mut W,
}
impl<'a> _Fact16W<'a> {
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
pub struct _Fact17W<'a> {
    w: &'a mut W,
}
impl<'a> _Fact17W<'a> {
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
pub struct _Fact18W<'a> {
    w: &'a mut W,
}
impl<'a> _Fact18W<'a> {
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
pub struct _Fact19W<'a> {
    w: &'a mut W,
}
impl<'a> _Fact19W<'a> {
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
pub struct _Fact20W<'a> {
    w: &'a mut W,
}
impl<'a> _Fact20W<'a> {
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
pub struct _Fact21W<'a> {
    w: &'a mut W,
}
impl<'a> _Fact21W<'a> {
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
pub struct _Fact22W<'a> {
    w: &'a mut W,
}
impl<'a> _Fact22W<'a> {
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
pub struct _Fact23W<'a> {
    w: &'a mut W,
}
impl<'a> _Fact23W<'a> {
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
pub struct _Fact24W<'a> {
    w: &'a mut W,
}
impl<'a> _Fact24W<'a> {
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
pub struct _Fact25W<'a> {
    w: &'a mut W,
}
impl<'a> _Fact25W<'a> {
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
pub struct _Fact26W<'a> {
    w: &'a mut W,
}
impl<'a> _Fact26W<'a> {
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
pub struct _Fact27W<'a> {
    w: &'a mut W,
}
impl<'a> _Fact27W<'a> {
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
    #[doc = "Bit 0 - Filter active"]
    #[inline(always)]
    pub fn fact0(&self) -> Fact0R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fact0R { bits }
    }
    #[doc = "Bit 1 - Filter active"]
    #[inline(always)]
    pub fn fact1(&self) -> Fact1R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fact1R { bits }
    }
    #[doc = "Bit 2 - Filter active"]
    #[inline(always)]
    pub fn fact2(&self) -> Fact2R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fact2R { bits }
    }
    #[doc = "Bit 3 - Filter active"]
    #[inline(always)]
    pub fn fact3(&self) -> Fact3R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fact3R { bits }
    }
    #[doc = "Bit 4 - Filter active"]
    #[inline(always)]
    pub fn fact4(&self) -> Fact4R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fact4R { bits }
    }
    #[doc = "Bit 5 - Filter active"]
    #[inline(always)]
    pub fn fact5(&self) -> Fact5R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fact5R { bits }
    }
    #[doc = "Bit 6 - Filter active"]
    #[inline(always)]
    pub fn fact6(&self) -> Fact6R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fact6R { bits }
    }
    #[doc = "Bit 7 - Filter active"]
    #[inline(always)]
    pub fn fact7(&self) -> Fact7R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fact7R { bits }
    }
    #[doc = "Bit 8 - Filter active"]
    #[inline(always)]
    pub fn fact8(&self) -> Fact8R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fact8R { bits }
    }
    #[doc = "Bit 9 - Filter active"]
    #[inline(always)]
    pub fn fact9(&self) -> Fact9R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fact9R { bits }
    }
    #[doc = "Bit 10 - Filter active"]
    #[inline(always)]
    pub fn fact10(&self) -> Fact10R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fact10R { bits }
    }
    #[doc = "Bit 11 - Filter active"]
    #[inline(always)]
    pub fn fact11(&self) -> Fact11R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fact11R { bits }
    }
    #[doc = "Bit 12 - Filter active"]
    #[inline(always)]
    pub fn fact12(&self) -> Fact12R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fact12R { bits }
    }
    #[doc = "Bit 13 - Filter active"]
    #[inline(always)]
    pub fn fact13(&self) -> Fact13R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fact13R { bits }
    }
    #[doc = "Bit 14 - Filter active"]
    #[inline(always)]
    pub fn fact14(&self) -> Fact14R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fact14R { bits }
    }
    #[doc = "Bit 15 - Filter active"]
    #[inline(always)]
    pub fn fact15(&self) -> Fact15R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fact15R { bits }
    }
    #[doc = "Bit 16 - Filter active"]
    #[inline(always)]
    pub fn fact16(&self) -> Fact16R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fact16R { bits }
    }
    #[doc = "Bit 17 - Filter active"]
    #[inline(always)]
    pub fn fact17(&self) -> Fact17R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fact17R { bits }
    }
    #[doc = "Bit 18 - Filter active"]
    #[inline(always)]
    pub fn fact18(&self) -> Fact18R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fact18R { bits }
    }
    #[doc = "Bit 19 - Filter active"]
    #[inline(always)]
    pub fn fact19(&self) -> Fact19R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fact19R { bits }
    }
    #[doc = "Bit 20 - Filter active"]
    #[inline(always)]
    pub fn fact20(&self) -> Fact20R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fact20R { bits }
    }
    #[doc = "Bit 21 - Filter active"]
    #[inline(always)]
    pub fn fact21(&self) -> Fact21R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fact21R { bits }
    }
    #[doc = "Bit 22 - Filter active"]
    #[inline(always)]
    pub fn fact22(&self) -> Fact22R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fact22R { bits }
    }
    #[doc = "Bit 23 - Filter active"]
    #[inline(always)]
    pub fn fact23(&self) -> Fact23R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fact23R { bits }
    }
    #[doc = "Bit 24 - Filter active"]
    #[inline(always)]
    pub fn fact24(&self) -> Fact24R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fact24R { bits }
    }
    #[doc = "Bit 25 - Filter active"]
    #[inline(always)]
    pub fn fact25(&self) -> Fact25R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fact25R { bits }
    }
    #[doc = "Bit 26 - Filter active"]
    #[inline(always)]
    pub fn fact26(&self) -> Fact26R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fact26R { bits }
    }
    #[doc = "Bit 27 - Filter active"]
    #[inline(always)]
    pub fn fact27(&self) -> Fact27R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Fact27R { bits }
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
    #[doc = "Bit 0 - Filter active"]
    #[inline(always)]
    pub fn fact0(&mut self) -> _Fact0W {
        _Fact0W { w: self }
    }
    #[doc = "Bit 1 - Filter active"]
    #[inline(always)]
    pub fn fact1(&mut self) -> _Fact1W {
        _Fact1W { w: self }
    }
    #[doc = "Bit 2 - Filter active"]
    #[inline(always)]
    pub fn fact2(&mut self) -> _Fact2W {
        _Fact2W { w: self }
    }
    #[doc = "Bit 3 - Filter active"]
    #[inline(always)]
    pub fn fact3(&mut self) -> _Fact3W {
        _Fact3W { w: self }
    }
    #[doc = "Bit 4 - Filter active"]
    #[inline(always)]
    pub fn fact4(&mut self) -> _Fact4W {
        _Fact4W { w: self }
    }
    #[doc = "Bit 5 - Filter active"]
    #[inline(always)]
    pub fn fact5(&mut self) -> _Fact5W {
        _Fact5W { w: self }
    }
    #[doc = "Bit 6 - Filter active"]
    #[inline(always)]
    pub fn fact6(&mut self) -> _Fact6W {
        _Fact6W { w: self }
    }
    #[doc = "Bit 7 - Filter active"]
    #[inline(always)]
    pub fn fact7(&mut self) -> _Fact7W {
        _Fact7W { w: self }
    }
    #[doc = "Bit 8 - Filter active"]
    #[inline(always)]
    pub fn fact8(&mut self) -> _Fact8W {
        _Fact8W { w: self }
    }
    #[doc = "Bit 9 - Filter active"]
    #[inline(always)]
    pub fn fact9(&mut self) -> _Fact9W {
        _Fact9W { w: self }
    }
    #[doc = "Bit 10 - Filter active"]
    #[inline(always)]
    pub fn fact10(&mut self) -> _Fact10W {
        _Fact10W { w: self }
    }
    #[doc = "Bit 11 - Filter active"]
    #[inline(always)]
    pub fn fact11(&mut self) -> _Fact11W {
        _Fact11W { w: self }
    }
    #[doc = "Bit 12 - Filter active"]
    #[inline(always)]
    pub fn fact12(&mut self) -> _Fact12W {
        _Fact12W { w: self }
    }
    #[doc = "Bit 13 - Filter active"]
    #[inline(always)]
    pub fn fact13(&mut self) -> _Fact13W {
        _Fact13W { w: self }
    }
    #[doc = "Bit 14 - Filter active"]
    #[inline(always)]
    pub fn fact14(&mut self) -> _Fact14W {
        _Fact14W { w: self }
    }
    #[doc = "Bit 15 - Filter active"]
    #[inline(always)]
    pub fn fact15(&mut self) -> _Fact15W {
        _Fact15W { w: self }
    }
    #[doc = "Bit 16 - Filter active"]
    #[inline(always)]
    pub fn fact16(&mut self) -> _Fact16W {
        _Fact16W { w: self }
    }
    #[doc = "Bit 17 - Filter active"]
    #[inline(always)]
    pub fn fact17(&mut self) -> _Fact17W {
        _Fact17W { w: self }
    }
    #[doc = "Bit 18 - Filter active"]
    #[inline(always)]
    pub fn fact18(&mut self) -> _Fact18W {
        _Fact18W { w: self }
    }
    #[doc = "Bit 19 - Filter active"]
    #[inline(always)]
    pub fn fact19(&mut self) -> _Fact19W {
        _Fact19W { w: self }
    }
    #[doc = "Bit 20 - Filter active"]
    #[inline(always)]
    pub fn fact20(&mut self) -> _Fact20W {
        _Fact20W { w: self }
    }
    #[doc = "Bit 21 - Filter active"]
    #[inline(always)]
    pub fn fact21(&mut self) -> _Fact21W {
        _Fact21W { w: self }
    }
    #[doc = "Bit 22 - Filter active"]
    #[inline(always)]
    pub fn fact22(&mut self) -> _Fact22W {
        _Fact22W { w: self }
    }
    #[doc = "Bit 23 - Filter active"]
    #[inline(always)]
    pub fn fact23(&mut self) -> _Fact23W {
        _Fact23W { w: self }
    }
    #[doc = "Bit 24 - Filter active"]
    #[inline(always)]
    pub fn fact24(&mut self) -> _Fact24W {
        _Fact24W { w: self }
    }
    #[doc = "Bit 25 - Filter active"]
    #[inline(always)]
    pub fn fact25(&mut self) -> _Fact25W {
        _Fact25W { w: self }
    }
    #[doc = "Bit 26 - Filter active"]
    #[inline(always)]
    pub fn fact26(&mut self) -> _Fact26W {
        _Fact26W { w: self }
    }
    #[doc = "Bit 27 - Filter active"]
    #[inline(always)]
    pub fn fact27(&mut self) -> _Fact27W {
        _Fact27W { w: self }
    }
}
