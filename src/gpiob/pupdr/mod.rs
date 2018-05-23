#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::Pupdr {
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
#[doc = "Possible values of the field `PUPDR15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Pupdr15R {
    #[doc = "No pull-up or pull-down."] None,
    #[doc = "Pull-up."] Pullup,
    #[doc = "Pull-down."] Pulldown,
    #[doc = r" Reserved"] _Reserved(u8),
}
impl Pupdr15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            Pupdr15R::None => 0,
            Pupdr15R::Pullup => 1,
            Pupdr15R::Pulldown => 2,
            Pupdr15R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(bits: u8) -> Pupdr15R {
        match bits {
            0 => Pupdr15R::None,
            1 => Pupdr15R::Pullup,
            2 => Pupdr15R::Pulldown,
            i => Pupdr15R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `None`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Pupdr15R::None
    }
    #[doc = "Checks if the value of the field is `Pullup`"]
    #[inline(always)]
    pub fn is_pullup(&self) -> bool {
        *self == Pupdr15R::Pullup
    }
    #[doc = "Checks if the value of the field is `Pulldown`"]
    #[inline(always)]
    pub fn is_pulldown(&self) -> bool {
        *self == Pupdr15R::Pulldown
    }
}
#[doc = "Possible values of the field `PUPDR14`"]
pub type Pupdr14R = Pupdr15R;
#[doc = "Possible values of the field `PUPDR13`"]
pub type Pupdr13R = Pupdr15R;
#[doc = "Possible values of the field `PUPDR12`"]
pub type Pupdr12R = Pupdr15R;
#[doc = "Possible values of the field `PUPDR11`"]
pub type Pupdr11R = Pupdr15R;
#[doc = "Possible values of the field `PUPDR10`"]
pub type Pupdr10R = Pupdr15R;
#[doc = "Possible values of the field `PUPDR9`"]
pub type Pupdr9R = Pupdr15R;
#[doc = "Possible values of the field `PUPDR8`"]
pub type Pupdr8R = Pupdr15R;
#[doc = "Possible values of the field `PUPDR7`"]
pub type Pupdr7R = Pupdr15R;
#[doc = "Possible values of the field `PUPDR6`"]
pub type Pupdr6R = Pupdr15R;
#[doc = "Possible values of the field `PUPDR5`"]
pub type Pupdr5R = Pupdr15R;
#[doc = "Possible values of the field `PUPDR4`"]
pub type Pupdr4R = Pupdr15R;
#[doc = "Possible values of the field `PUPDR3`"]
pub type Pupdr3R = Pupdr15R;
#[doc = "Possible values of the field `PUPDR2`"]
pub type Pupdr2R = Pupdr15R;
#[doc = "Possible values of the field `PUPDR1`"]
pub type Pupdr1R = Pupdr15R;
#[doc = "Possible values of the field `PUPDR0`"]
pub type Pupdr0R = Pupdr15R;
#[doc = "Values that can be written to the field `PUPDR15`"]
pub enum Pupdr15W {
    #[doc = "No pull-up or pull-down."] None,
    #[doc = "Pull-up."] Pullup,
    #[doc = "Pull-down."] Pulldown,
}
impl Pupdr15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            Pupdr15W::None => 0,
            Pupdr15W::Pullup => 1,
            Pupdr15W::Pulldown => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _Pupdr15W<'a> {
    w: &'a mut W,
}
impl<'a> _Pupdr15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Pupdr15W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No pull-up or pull-down."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(Pupdr15W::None)
    }
    #[doc = "Pull-up."]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut W {
        self.variant(Pupdr15W::Pullup)
    }
    #[doc = "Pull-down."]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut W {
        self.variant(Pupdr15W::Pulldown)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PUPDR14`"]
pub type Pupdr14W = Pupdr15W;
#[doc = r" Proxy"]
pub struct _Pupdr14W<'a> {
    w: &'a mut W,
}
impl<'a> _Pupdr14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Pupdr14W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No pull-up or pull-down."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(Pupdr15W::None)
    }
    #[doc = "Pull-up."]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut W {
        self.variant(Pupdr15W::Pullup)
    }
    #[doc = "Pull-down."]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut W {
        self.variant(Pupdr15W::Pulldown)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PUPDR13`"]
pub type Pupdr13W = Pupdr15W;
#[doc = r" Proxy"]
pub struct _Pupdr13W<'a> {
    w: &'a mut W,
}
impl<'a> _Pupdr13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Pupdr13W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No pull-up or pull-down."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(Pupdr15W::None)
    }
    #[doc = "Pull-up."]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut W {
        self.variant(Pupdr15W::Pullup)
    }
    #[doc = "Pull-down."]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut W {
        self.variant(Pupdr15W::Pulldown)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PUPDR12`"]
pub type Pupdr12W = Pupdr15W;
#[doc = r" Proxy"]
pub struct _Pupdr12W<'a> {
    w: &'a mut W,
}
impl<'a> _Pupdr12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Pupdr12W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No pull-up or pull-down."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(Pupdr15W::None)
    }
    #[doc = "Pull-up."]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut W {
        self.variant(Pupdr15W::Pullup)
    }
    #[doc = "Pull-down."]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut W {
        self.variant(Pupdr15W::Pulldown)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PUPDR11`"]
pub type Pupdr11W = Pupdr15W;
#[doc = r" Proxy"]
pub struct _Pupdr11W<'a> {
    w: &'a mut W,
}
impl<'a> _Pupdr11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Pupdr11W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No pull-up or pull-down."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(Pupdr15W::None)
    }
    #[doc = "Pull-up."]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut W {
        self.variant(Pupdr15W::Pullup)
    }
    #[doc = "Pull-down."]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut W {
        self.variant(Pupdr15W::Pulldown)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PUPDR10`"]
pub type Pupdr10W = Pupdr15W;
#[doc = r" Proxy"]
pub struct _Pupdr10W<'a> {
    w: &'a mut W,
}
impl<'a> _Pupdr10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Pupdr10W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No pull-up or pull-down."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(Pupdr15W::None)
    }
    #[doc = "Pull-up."]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut W {
        self.variant(Pupdr15W::Pullup)
    }
    #[doc = "Pull-down."]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut W {
        self.variant(Pupdr15W::Pulldown)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PUPDR9`"]
pub type Pupdr9W = Pupdr15W;
#[doc = r" Proxy"]
pub struct _Pupdr9W<'a> {
    w: &'a mut W,
}
impl<'a> _Pupdr9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Pupdr9W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No pull-up or pull-down."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(Pupdr15W::None)
    }
    #[doc = "Pull-up."]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut W {
        self.variant(Pupdr15W::Pullup)
    }
    #[doc = "Pull-down."]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut W {
        self.variant(Pupdr15W::Pulldown)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PUPDR8`"]
pub type Pupdr8W = Pupdr15W;
#[doc = r" Proxy"]
pub struct _Pupdr8W<'a> {
    w: &'a mut W,
}
impl<'a> _Pupdr8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Pupdr8W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No pull-up or pull-down."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(Pupdr15W::None)
    }
    #[doc = "Pull-up."]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut W {
        self.variant(Pupdr15W::Pullup)
    }
    #[doc = "Pull-down."]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut W {
        self.variant(Pupdr15W::Pulldown)
    }
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
#[doc = "Values that can be written to the field `PUPDR7`"]
pub type Pupdr7W = Pupdr15W;
#[doc = r" Proxy"]
pub struct _Pupdr7W<'a> {
    w: &'a mut W,
}
impl<'a> _Pupdr7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Pupdr7W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No pull-up or pull-down."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(Pupdr15W::None)
    }
    #[doc = "Pull-up."]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut W {
        self.variant(Pupdr15W::Pullup)
    }
    #[doc = "Pull-down."]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut W {
        self.variant(Pupdr15W::Pulldown)
    }
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
#[doc = "Values that can be written to the field `PUPDR6`"]
pub type Pupdr6W = Pupdr15W;
#[doc = r" Proxy"]
pub struct _Pupdr6W<'a> {
    w: &'a mut W,
}
impl<'a> _Pupdr6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Pupdr6W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No pull-up or pull-down."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(Pupdr15W::None)
    }
    #[doc = "Pull-up."]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut W {
        self.variant(Pupdr15W::Pullup)
    }
    #[doc = "Pull-down."]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut W {
        self.variant(Pupdr15W::Pulldown)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PUPDR5`"]
pub type Pupdr5W = Pupdr15W;
#[doc = r" Proxy"]
pub struct _Pupdr5W<'a> {
    w: &'a mut W,
}
impl<'a> _Pupdr5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Pupdr5W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No pull-up or pull-down."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(Pupdr15W::None)
    }
    #[doc = "Pull-up."]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut W {
        self.variant(Pupdr15W::Pullup)
    }
    #[doc = "Pull-down."]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut W {
        self.variant(Pupdr15W::Pulldown)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PUPDR4`"]
pub type Pupdr4W = Pupdr15W;
#[doc = r" Proxy"]
pub struct _Pupdr4W<'a> {
    w: &'a mut W,
}
impl<'a> _Pupdr4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Pupdr4W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No pull-up or pull-down."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(Pupdr15W::None)
    }
    #[doc = "Pull-up."]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut W {
        self.variant(Pupdr15W::Pullup)
    }
    #[doc = "Pull-down."]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut W {
        self.variant(Pupdr15W::Pulldown)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PUPDR3`"]
pub type Pupdr3W = Pupdr15W;
#[doc = r" Proxy"]
pub struct _Pupdr3W<'a> {
    w: &'a mut W,
}
impl<'a> _Pupdr3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Pupdr3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No pull-up or pull-down."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(Pupdr15W::None)
    }
    #[doc = "Pull-up."]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut W {
        self.variant(Pupdr15W::Pullup)
    }
    #[doc = "Pull-down."]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut W {
        self.variant(Pupdr15W::Pulldown)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PUPDR2`"]
pub type Pupdr2W = Pupdr15W;
#[doc = r" Proxy"]
pub struct _Pupdr2W<'a> {
    w: &'a mut W,
}
impl<'a> _Pupdr2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Pupdr2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No pull-up or pull-down."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(Pupdr15W::None)
    }
    #[doc = "Pull-up."]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut W {
        self.variant(Pupdr15W::Pullup)
    }
    #[doc = "Pull-down."]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut W {
        self.variant(Pupdr15W::Pulldown)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PUPDR1`"]
pub type Pupdr1W = Pupdr15W;
#[doc = r" Proxy"]
pub struct _Pupdr1W<'a> {
    w: &'a mut W,
}
impl<'a> _Pupdr1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Pupdr1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No pull-up or pull-down."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(Pupdr15W::None)
    }
    #[doc = "Pull-up."]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut W {
        self.variant(Pupdr15W::Pullup)
    }
    #[doc = "Pull-down."]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut W {
        self.variant(Pupdr15W::Pulldown)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PUPDR0`"]
pub type Pupdr0W = Pupdr15W;
#[doc = r" Proxy"]
pub struct _Pupdr0W<'a> {
    w: &'a mut W,
}
impl<'a> _Pupdr0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Pupdr0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No pull-up or pull-down."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(Pupdr15W::None)
    }
    #[doc = "Pull-up."]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut W {
        self.variant(Pupdr15W::Pullup)
    }
    #[doc = "Pull-down."]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut W {
        self.variant(Pupdr15W::Pulldown)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr15(&self) -> Pupdr15R {
        Pupdr15R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr14(&self) -> Pupdr14R {
        Pupdr14R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr13(&self) -> Pupdr13R {
        Pupdr13R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr12(&self) -> Pupdr12R {
        Pupdr12R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr11(&self) -> Pupdr11R {
        Pupdr11R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr10(&self) -> Pupdr10R {
        Pupdr10R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr9(&self) -> Pupdr9R {
        Pupdr9R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr8(&self) -> Pupdr8R {
        Pupdr8R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr7(&self) -> Pupdr7R {
        Pupdr7R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr6(&self) -> Pupdr6R {
        Pupdr6R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr5(&self) -> Pupdr5R {
        Pupdr5R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr4(&self) -> Pupdr4R {
        Pupdr4R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr3(&self) -> Pupdr3R {
        Pupdr3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr2(&self) -> Pupdr2R {
        Pupdr2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr1(&self) -> Pupdr1R {
        Pupdr1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr0(&self) -> Pupdr0R {
        Pupdr0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline(always)]
    pub fn reset_value() -> W {
        W { bits: 256 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr15(&mut self) -> _Pupdr15W {
        _Pupdr15W { w: self }
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr14(&mut self) -> _Pupdr14W {
        _Pupdr14W { w: self }
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr13(&mut self) -> _Pupdr13W {
        _Pupdr13W { w: self }
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr12(&mut self) -> _Pupdr12W {
        _Pupdr12W { w: self }
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr11(&mut self) -> _Pupdr11W {
        _Pupdr11W { w: self }
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr10(&mut self) -> _Pupdr10W {
        _Pupdr10W { w: self }
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr9(&mut self) -> _Pupdr9W {
        _Pupdr9W { w: self }
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr8(&mut self) -> _Pupdr8W {
        _Pupdr8W { w: self }
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr7(&mut self) -> _Pupdr7W {
        _Pupdr7W { w: self }
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr6(&mut self) -> _Pupdr6W {
        _Pupdr6W { w: self }
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr5(&mut self) -> _Pupdr5W {
        _Pupdr5W { w: self }
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr4(&mut self) -> _Pupdr4W {
        _Pupdr4W { w: self }
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr3(&mut self) -> _Pupdr3W {
        _Pupdr3W { w: self }
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr2(&mut self) -> _Pupdr2W {
        _Pupdr2W { w: self }
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr1(&mut self) -> _Pupdr1W {
        _Pupdr1W { w: self }
    }
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr0(&mut self) -> _Pupdr0W {
        _Pupdr0W { w: self }
    }
}
