#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::Ahb1lpenr {
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
#[doc = "Possible values of the field `GPIOALPEN`"]
pub type GpioalpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `GPIOBLPEN`"]
pub type GpioblpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `GPIOCLPEN`"]
pub type GpioclpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `GPIODLPEN`"]
pub type GpiodlpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `GPIOELPEN`"]
pub type GpioelpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `GPIOFLPEN`"]
pub type GpioflpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `GPIOGLPEN`"]
pub type GpioglpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `GPIOHLPEN`"]
pub type GpiohlpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `GPIOILPEN`"]
pub type GpioilpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `GPIOJLPEN`"]
pub type GpiojlpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `GPIOKLPEN`"]
pub type GpioklpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `CRCLPEN`"]
pub type CrclpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `FLITFLPEN`"]
pub type FlitflpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `SRAM1LPEN`"]
pub type Sram1lpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `SRAM2LPEN`"]
pub type Sram2lpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `BKPSRAMLPEN`"]
pub type BkpsramlpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `SRAM3LPEN`"]
pub type Sram3lpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `DMA1LPEN`"]
pub type Dma1lpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `DMA2LPEN`"]
pub type Dma2lpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `DMA2DLPEN`"]
pub type Dma2dlpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `ETHMACLPEN`"]
pub type EthmaclpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `ETHMACTXLPEN`"]
pub type EthmactxlpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `ETHMACRXLPEN`"]
pub type EthmacrxlpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `ETHMACPTPLPEN`"]
pub type EthmacptplpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `OTGHSLPEN`"]
pub type OtghslpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `OTGHSULPILPEN`"]
pub type OtghsulpilpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Values that can be written to the field `GPIOALPEN`"]
pub type GpioalpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _GpioalpenW<'a> {
    w: &'a mut W,
}
impl<'a> _GpioalpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GpioalpenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIOBLPEN`"]
pub type GpioblpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _GpioblpenW<'a> {
    w: &'a mut W,
}
impl<'a> _GpioblpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GpioblpenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIOCLPEN`"]
pub type GpioclpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _GpioclpenW<'a> {
    w: &'a mut W,
}
impl<'a> _GpioclpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GpioclpenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIODLPEN`"]
pub type GpiodlpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _GpiodlpenW<'a> {
    w: &'a mut W,
}
impl<'a> _GpiodlpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GpiodlpenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIOELPEN`"]
pub type GpioelpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _GpioelpenW<'a> {
    w: &'a mut W,
}
impl<'a> _GpioelpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GpioelpenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIOFLPEN`"]
pub type GpioflpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _GpioflpenW<'a> {
    w: &'a mut W,
}
impl<'a> _GpioflpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GpioflpenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIOGLPEN`"]
pub type GpioglpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _GpioglpenW<'a> {
    w: &'a mut W,
}
impl<'a> _GpioglpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GpioglpenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIOHLPEN`"]
pub type GpiohlpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _GpiohlpenW<'a> {
    w: &'a mut W,
}
impl<'a> _GpiohlpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GpiohlpenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIOILPEN`"]
pub type GpioilpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _GpioilpenW<'a> {
    w: &'a mut W,
}
impl<'a> _GpioilpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GpioilpenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIOJLPEN`"]
pub type GpiojlpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _GpiojlpenW<'a> {
    w: &'a mut W,
}
impl<'a> _GpiojlpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GpiojlpenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIOKLPEN`"]
pub type GpioklpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _GpioklpenW<'a> {
    w: &'a mut W,
}
impl<'a> _GpioklpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GpioklpenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CRCLPEN`"]
pub type CrclpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _CrclpenW<'a> {
    w: &'a mut W,
}
impl<'a> _CrclpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CrclpenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FLITFLPEN`"]
pub type FlitflpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _FlitflpenW<'a> {
    w: &'a mut W,
}
impl<'a> _FlitflpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FlitflpenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRAM1LPEN`"]
pub type Sram1lpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Sram1lpenW<'a> {
    w: &'a mut W,
}
impl<'a> _Sram1lpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Sram1lpenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRAM2LPEN`"]
pub type Sram2lpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Sram2lpenW<'a> {
    w: &'a mut W,
}
impl<'a> _Sram2lpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Sram2lpenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BKPSRAMLPEN`"]
pub type BkpsramlpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _BkpsramlpenW<'a> {
    w: &'a mut W,
}
impl<'a> _BkpsramlpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BkpsramlpenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRAM3LPEN`"]
pub type Sram3lpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Sram3lpenW<'a> {
    w: &'a mut W,
}
impl<'a> _Sram3lpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Sram3lpenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMA1LPEN`"]
pub type Dma1lpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Dma1lpenW<'a> {
    w: &'a mut W,
}
impl<'a> _Dma1lpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Dma1lpenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMA2LPEN`"]
pub type Dma2lpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Dma2lpenW<'a> {
    w: &'a mut W,
}
impl<'a> _Dma2lpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Dma2lpenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMA2DLPEN`"]
pub type Dma2dlpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Dma2dlpenW<'a> {
    w: &'a mut W,
}
impl<'a> _Dma2dlpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Dma2dlpenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ETHMACLPEN`"]
pub type EthmaclpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _EthmaclpenW<'a> {
    w: &'a mut W,
}
impl<'a> _EthmaclpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EthmaclpenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ETHMACTXLPEN`"]
pub type EthmactxlpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _EthmactxlpenW<'a> {
    w: &'a mut W,
}
impl<'a> _EthmactxlpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EthmactxlpenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ETHMACRXLPEN`"]
pub type EthmacrxlpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _EthmacrxlpenW<'a> {
    w: &'a mut W,
}
impl<'a> _EthmacrxlpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EthmacrxlpenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ETHMACPTPLPEN`"]
pub type EthmacptplpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _EthmacptplpenW<'a> {
    w: &'a mut W,
}
impl<'a> _EthmacptplpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EthmacptplpenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OTGHSLPEN`"]
pub type OtghslpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _OtghslpenW<'a> {
    w: &'a mut W,
}
impl<'a> _OtghslpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OtghslpenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OTGHSULPILPEN`"]
pub type OtghsulpilpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _OtghsulpilpenW<'a> {
    w: &'a mut W,
}
impl<'a> _OtghsulpilpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OtghsulpilpenW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(super::ahb1enr::OtghsulpienW::Disabled)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, bits: u8) -> &'a mut W {
        const MASK: u8 = 1;
        const OFFSET: u8 = 30;
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
    #[doc = "Bit 0 - IO port A clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioalpen(&self) -> GpioalpenR {
        GpioalpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 1 - IO port B clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioblpen(&self) -> GpioblpenR {
        GpioblpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - IO port C clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioclpen(&self) -> GpioclpenR {
        GpioclpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - IO port D clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiodlpen(&self) -> GpiodlpenR {
        GpiodlpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - IO port E clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioelpen(&self) -> GpioelpenR {
        GpioelpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - IO port F clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioflpen(&self) -> GpioflpenR {
        GpioflpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - IO port G clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioglpen(&self) -> GpioglpenR {
        GpioglpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - IO port H clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiohlpen(&self) -> GpiohlpenR {
        GpiohlpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - IO port I clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioilpen(&self) -> GpioilpenR {
        GpioilpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 9 - IO port J clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiojlpen(&self) -> GpiojlpenR {
        GpiojlpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - IO port K clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioklpen(&self) -> GpioklpenR {
        GpioklpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - CRC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn crclpen(&self) -> CrclpenR {
        CrclpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - Flash interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn flitflpen(&self) -> FlitflpenR {
        FlitflpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - SRAM 1interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn sram1lpen(&self) -> Sram1lpenR {
        Sram1lpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 17 - SRAM 2 interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn sram2lpen(&self) -> Sram2lpenR {
        Sram2lpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 18 - Backup SRAM interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn bkpsramlpen(&self) -> BkpsramlpenR {
        BkpsramlpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 19 - SRAM 3 interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn sram3lpen(&self) -> Sram3lpenR {
        Sram3lpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 21 - DMA1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dma1lpen(&self) -> Dma1lpenR {
        Dma1lpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 22 - DMA2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dma2lpen(&self) -> Dma2lpenR {
        Dma2lpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 23 - DMA2D clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dma2dlpen(&self) -> Dma2dlpenR {
        Dma2dlpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 25 - Ethernet MAC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn ethmaclpen(&self) -> EthmaclpenR {
        EthmaclpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 26 - Ethernet transmission clock enable during Sleep mode"]
    #[inline(always)]
    pub fn ethmactxlpen(&self) -> EthmactxlpenR {
        EthmactxlpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 27 - Ethernet reception clock enable during Sleep mode"]
    #[inline(always)]
    pub fn ethmacrxlpen(&self) -> EthmacrxlpenR {
        EthmacrxlpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - Ethernet PTP clock enable during Sleep mode"]
    #[inline(always)]
    pub fn ethmacptplpen(&self) -> EthmacptplpenR {
        EthmacptplpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 29 - USB OTG HS clock enable during Sleep mode"]
    #[inline(always)]
    pub fn otghslpen(&self) -> OtghslpenR {
        OtghslpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 30 - USB OTG HS ULPI clock enable during Sleep mode"]
    #[inline(always)]
    pub fn otghsulpilpen(&self) -> OtghsulpilpenR {
        OtghsulpilpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline(always)]
    pub fn reset_value() -> W {
        W { bits: 2120716799 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - IO port A clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioalpen(&mut self) -> _GpioalpenW {
        _GpioalpenW { w: self }
    }
    #[doc = "Bit 1 - IO port B clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioblpen(&mut self) -> _GpioblpenW {
        _GpioblpenW { w: self }
    }
    #[doc = "Bit 2 - IO port C clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioclpen(&mut self) -> _GpioclpenW {
        _GpioclpenW { w: self }
    }
    #[doc = "Bit 3 - IO port D clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiodlpen(&mut self) -> _GpiodlpenW {
        _GpiodlpenW { w: self }
    }
    #[doc = "Bit 4 - IO port E clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioelpen(&mut self) -> _GpioelpenW {
        _GpioelpenW { w: self }
    }
    #[doc = "Bit 5 - IO port F clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioflpen(&mut self) -> _GpioflpenW {
        _GpioflpenW { w: self }
    }
    #[doc = "Bit 6 - IO port G clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioglpen(&mut self) -> _GpioglpenW {
        _GpioglpenW { w: self }
    }
    #[doc = "Bit 7 - IO port H clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiohlpen(&mut self) -> _GpiohlpenW {
        _GpiohlpenW { w: self }
    }
    #[doc = "Bit 8 - IO port I clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioilpen(&mut self) -> _GpioilpenW {
        _GpioilpenW { w: self }
    }
    #[doc = "Bit 9 - IO port J clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiojlpen(&mut self) -> _GpiojlpenW {
        _GpiojlpenW { w: self }
    }
    #[doc = "Bit 10 - IO port K clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioklpen(&mut self) -> _GpioklpenW {
        _GpioklpenW { w: self }
    }
    #[doc = "Bit 12 - CRC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn crclpen(&mut self) -> _CrclpenW {
        _CrclpenW { w: self }
    }
    #[doc = "Bit 15 - Flash interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn flitflpen(&mut self) -> _FlitflpenW {
        _FlitflpenW { w: self }
    }
    #[doc = "Bit 16 - SRAM 1interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn sram1lpen(&mut self) -> _Sram1lpenW {
        _Sram1lpenW { w: self }
    }
    #[doc = "Bit 17 - SRAM 2 interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn sram2lpen(&mut self) -> _Sram2lpenW {
        _Sram2lpenW { w: self }
    }
    #[doc = "Bit 18 - Backup SRAM interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn bkpsramlpen(&mut self) -> _BkpsramlpenW {
        _BkpsramlpenW { w: self }
    }
    #[doc = "Bit 19 - SRAM 3 interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn sram3lpen(&mut self) -> _Sram3lpenW {
        _Sram3lpenW { w: self }
    }
    #[doc = "Bit 21 - DMA1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dma1lpen(&mut self) -> _Dma1lpenW {
        _Dma1lpenW { w: self }
    }
    #[doc = "Bit 22 - DMA2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dma2lpen(&mut self) -> _Dma2lpenW {
        _Dma2lpenW { w: self }
    }
    #[doc = "Bit 23 - DMA2D clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dma2dlpen(&mut self) -> _Dma2dlpenW {
        _Dma2dlpenW { w: self }
    }
    #[doc = "Bit 25 - Ethernet MAC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn ethmaclpen(&mut self) -> _EthmaclpenW {
        _EthmaclpenW { w: self }
    }
    #[doc = "Bit 26 - Ethernet transmission clock enable during Sleep mode"]
    #[inline(always)]
    pub fn ethmactxlpen(&mut self) -> _EthmactxlpenW {
        _EthmactxlpenW { w: self }
    }
    #[doc = "Bit 27 - Ethernet reception clock enable during Sleep mode"]
    #[inline(always)]
    pub fn ethmacrxlpen(&mut self) -> _EthmacrxlpenW {
        _EthmacrxlpenW { w: self }
    }
    #[doc = "Bit 28 - Ethernet PTP clock enable during Sleep mode"]
    #[inline(always)]
    pub fn ethmacptplpen(&mut self) -> _EthmacptplpenW {
        _EthmacptplpenW { w: self }
    }
    #[doc = "Bit 29 - USB OTG HS clock enable during Sleep mode"]
    #[inline(always)]
    pub fn otghslpen(&mut self) -> _OtghslpenW {
        _OtghslpenW { w: self }
    }
    #[doc = "Bit 30 - USB OTG HS ULPI clock enable during Sleep mode"]
    #[inline(always)]
    pub fn otghsulpilpen(&mut self) -> _OtghsulpilpenW {
        _OtghsulpilpenW { w: self }
    }
}
