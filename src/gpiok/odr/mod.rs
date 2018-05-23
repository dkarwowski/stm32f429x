#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::Odr {
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
pub struct Odr15R {
    bits: u8,
}
impl Odr15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Odr14R {
    bits: u8,
}
impl Odr14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Odr13R {
    bits: u8,
}
impl Odr13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Odr12R {
    bits: u8,
}
impl Odr12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Odr11R {
    bits: u8,
}
impl Odr11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Odr10R {
    bits: u8,
}
impl Odr10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Odr9R {
    bits: u8,
}
impl Odr9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Odr8R {
    bits: u8,
}
impl Odr8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Odr7R {
    bits: u8,
}
impl Odr7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Odr6R {
    bits: u8,
}
impl Odr6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Odr5R {
    bits: u8,
}
impl Odr5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Odr4R {
    bits: u8,
}
impl Odr4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Odr3R {
    bits: u8,
}
impl Odr3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Odr2R {
    bits: u8,
}
impl Odr2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Odr1R {
    bits: u8,
}
impl Odr1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Odr0R {
    bits: u8,
}
impl Odr0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _Odr15W<'a> {
    w: &'a mut W,
}
impl<'a> _Odr15W<'a> {
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
pub struct _Odr14W<'a> {
    w: &'a mut W,
}
impl<'a> _Odr14W<'a> {
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
pub struct _Odr13W<'a> {
    w: &'a mut W,
}
impl<'a> _Odr13W<'a> {
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
pub struct _Odr12W<'a> {
    w: &'a mut W,
}
impl<'a> _Odr12W<'a> {
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
pub struct _Odr11W<'a> {
    w: &'a mut W,
}
impl<'a> _Odr11W<'a> {
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
pub struct _Odr10W<'a> {
    w: &'a mut W,
}
impl<'a> _Odr10W<'a> {
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
pub struct _Odr9W<'a> {
    w: &'a mut W,
}
impl<'a> _Odr9W<'a> {
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
pub struct _Odr8W<'a> {
    w: &'a mut W,
}
impl<'a> _Odr8W<'a> {
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
pub struct _Odr7W<'a> {
    w: &'a mut W,
}
impl<'a> _Odr7W<'a> {
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
pub struct _Odr6W<'a> {
    w: &'a mut W,
}
impl<'a> _Odr6W<'a> {
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
pub struct _Odr5W<'a> {
    w: &'a mut W,
}
impl<'a> _Odr5W<'a> {
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
pub struct _Odr4W<'a> {
    w: &'a mut W,
}
impl<'a> _Odr4W<'a> {
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
pub struct _Odr3W<'a> {
    w: &'a mut W,
}
impl<'a> _Odr3W<'a> {
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
pub struct _Odr2W<'a> {
    w: &'a mut W,
}
impl<'a> _Odr2W<'a> {
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
pub struct _Odr1W<'a> {
    w: &'a mut W,
}
impl<'a> _Odr1W<'a> {
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
pub struct _Odr0W<'a> {
    w: &'a mut W,
}
impl<'a> _Odr0W<'a> {
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
    #[doc = "Bit 15 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr15(&self) -> Odr15R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Odr15R { bits }
    }
    #[doc = "Bit 14 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr14(&self) -> Odr14R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Odr14R { bits }
    }
    #[doc = "Bit 13 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr13(&self) -> Odr13R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Odr13R { bits }
    }
    #[doc = "Bit 12 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr12(&self) -> Odr12R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Odr12R { bits }
    }
    #[doc = "Bit 11 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr11(&self) -> Odr11R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Odr11R { bits }
    }
    #[doc = "Bit 10 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr10(&self) -> Odr10R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Odr10R { bits }
    }
    #[doc = "Bit 9 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr9(&self) -> Odr9R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Odr9R { bits }
    }
    #[doc = "Bit 8 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr8(&self) -> Odr8R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Odr8R { bits }
    }
    #[doc = "Bit 7 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr7(&self) -> Odr7R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Odr7R { bits }
    }
    #[doc = "Bit 6 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr6(&self) -> Odr6R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Odr6R { bits }
    }
    #[doc = "Bit 5 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr5(&self) -> Odr5R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Odr5R { bits }
    }
    #[doc = "Bit 4 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr4(&self) -> Odr4R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Odr4R { bits }
    }
    #[doc = "Bit 3 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr3(&self) -> Odr3R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Odr3R { bits }
    }
    #[doc = "Bit 2 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr2(&self) -> Odr2R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Odr2R { bits }
    }
    #[doc = "Bit 1 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr1(&self) -> Odr1R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Odr1R { bits }
    }
    #[doc = "Bit 0 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr0(&self) -> Odr0R {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Odr0R { bits }
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
    #[doc = "Bit 15 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr15(&mut self) -> _Odr15W {
        _Odr15W { w: self }
    }
    #[doc = "Bit 14 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr14(&mut self) -> _Odr14W {
        _Odr14W { w: self }
    }
    #[doc = "Bit 13 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr13(&mut self) -> _Odr13W {
        _Odr13W { w: self }
    }
    #[doc = "Bit 12 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr12(&mut self) -> _Odr12W {
        _Odr12W { w: self }
    }
    #[doc = "Bit 11 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr11(&mut self) -> _Odr11W {
        _Odr11W { w: self }
    }
    #[doc = "Bit 10 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr10(&mut self) -> _Odr10W {
        _Odr10W { w: self }
    }
    #[doc = "Bit 9 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr9(&mut self) -> _Odr9W {
        _Odr9W { w: self }
    }
    #[doc = "Bit 8 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr8(&mut self) -> _Odr8W {
        _Odr8W { w: self }
    }
    #[doc = "Bit 7 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr7(&mut self) -> _Odr7W {
        _Odr7W { w: self }
    }
    #[doc = "Bit 6 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr6(&mut self) -> _Odr6W {
        _Odr6W { w: self }
    }
    #[doc = "Bit 5 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr5(&mut self) -> _Odr5W {
        _Odr5W { w: self }
    }
    #[doc = "Bit 4 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr4(&mut self) -> _Odr4W {
        _Odr4W { w: self }
    }
    #[doc = "Bit 3 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr3(&mut self) -> _Odr3W {
        _Odr3W { w: self }
    }
    #[doc = "Bit 2 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr2(&mut self) -> _Odr2W {
        _Odr2W { w: self }
    }
    #[doc = "Bit 1 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr1(&mut self) -> _Odr1W {
        _Odr1W { w: self }
    }
    #[doc = "Bit 0 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr0(&mut self) -> _Odr0W {
        _Odr0W { w: self }
    }
}
