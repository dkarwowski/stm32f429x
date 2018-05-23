#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::Apb1lpenr {
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
#[doc = "Possible values of the field `TIM2LPEN`"]
pub type Tim2lpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `TIM3LPEN`"]
pub type Tim3lpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `TIM4LPEN`"]
pub type Tim4lpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `TIM5LPEN`"]
pub type Tim5lpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `TIM6LPEN`"]
pub type Tim6lpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `TIM7LPEN`"]
pub type Tim7lpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `TIM12LPEN`"]
pub type Tim12lpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `TIM13LPEN`"]
pub type Tim13lpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `TIM14LPEN`"]
pub type Tim14lpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `WWDGLPEN`"]
pub type WwdglpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `SPI2LPEN`"]
pub type Spi2lpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `SPI3LPEN`"]
pub type Spi3lpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `USART2LPEN`"]
pub type Usart2lpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `USART3LPEN`"]
pub type Usart3lpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `UART4LPEN`"]
pub type Uart4lpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `UART5LPEN`"]
pub type Uart5lpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `I2C1LPEN`"]
pub type I2c1lpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `I2C2LPEN`"]
pub type I2c2lpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `I2C3LPEN`"]
pub type I2c3lpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `CAN1LPEN`"]
pub type Can1lpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `CAN2LPEN`"]
pub type Can2lpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `PWRLPEN`"]
pub type PwrlpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `DACLPEN`"]
pub type DaclpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `UART7LPEN`"]
pub type Uart7lpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Possible values of the field `UART8LPEN`"]
pub type Uart8lpenR = super::ahb1enr::OtghsulpienR;
#[doc = "Values that can be written to the field `TIM2LPEN`"]
pub type Tim2lpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Tim2lpenW<'a> {
    w: &'a mut W,
}
impl<'a> _Tim2lpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Tim2lpenW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `TIM3LPEN`"]
pub type Tim3lpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Tim3lpenW<'a> {
    w: &'a mut W,
}
impl<'a> _Tim3lpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Tim3lpenW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `TIM4LPEN`"]
pub type Tim4lpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Tim4lpenW<'a> {
    w: &'a mut W,
}
impl<'a> _Tim4lpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Tim4lpenW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `TIM5LPEN`"]
pub type Tim5lpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Tim5lpenW<'a> {
    w: &'a mut W,
}
impl<'a> _Tim5lpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Tim5lpenW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `TIM6LPEN`"]
pub type Tim6lpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Tim6lpenW<'a> {
    w: &'a mut W,
}
impl<'a> _Tim6lpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Tim6lpenW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `TIM7LPEN`"]
pub type Tim7lpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Tim7lpenW<'a> {
    w: &'a mut W,
}
impl<'a> _Tim7lpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Tim7lpenW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `TIM12LPEN`"]
pub type Tim12lpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Tim12lpenW<'a> {
    w: &'a mut W,
}
impl<'a> _Tim12lpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Tim12lpenW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `TIM13LPEN`"]
pub type Tim13lpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Tim13lpenW<'a> {
    w: &'a mut W,
}
impl<'a> _Tim13lpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Tim13lpenW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `TIM14LPEN`"]
pub type Tim14lpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Tim14lpenW<'a> {
    w: &'a mut W,
}
impl<'a> _Tim14lpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Tim14lpenW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `WWDGLPEN`"]
pub type WwdglpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _WwdglpenW<'a> {
    w: &'a mut W,
}
impl<'a> _WwdglpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WwdglpenW) -> &'a mut W {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPI2LPEN`"]
pub type Spi2lpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Spi2lpenW<'a> {
    w: &'a mut W,
}
impl<'a> _Spi2lpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Spi2lpenW) -> &'a mut W {
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPI3LPEN`"]
pub type Spi3lpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Spi3lpenW<'a> {
    w: &'a mut W,
}
impl<'a> _Spi3lpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Spi3lpenW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `USART2LPEN`"]
pub type Usart2lpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Usart2lpenW<'a> {
    w: &'a mut W,
}
impl<'a> _Usart2lpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Usart2lpenW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `USART3LPEN`"]
pub type Usart3lpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Usart3lpenW<'a> {
    w: &'a mut W,
}
impl<'a> _Usart3lpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Usart3lpenW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UART4LPEN`"]
pub type Uart4lpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Uart4lpenW<'a> {
    w: &'a mut W,
}
impl<'a> _Uart4lpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Uart4lpenW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UART5LPEN`"]
pub type Uart5lpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Uart5lpenW<'a> {
    w: &'a mut W,
}
impl<'a> _Uart5lpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Uart5lpenW) -> &'a mut W {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((bits & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `I2C1LPEN`"]
pub type I2c1lpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _I2c1lpenW<'a> {
    w: &'a mut W,
}
impl<'a> _I2c1lpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2c1lpenW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `I2C2LPEN`"]
pub type I2c2lpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _I2c2lpenW<'a> {
    w: &'a mut W,
}
impl<'a> _I2c2lpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2c2lpenW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `I2C3LPEN`"]
pub type I2c3lpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _I2c3lpenW<'a> {
    w: &'a mut W,
}
impl<'a> _I2c3lpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2c3lpenW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `CAN1LPEN`"]
pub type Can1lpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Can1lpenW<'a> {
    w: &'a mut W,
}
impl<'a> _Can1lpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Can1lpenW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `CAN2LPEN`"]
pub type Can2lpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Can2lpenW<'a> {
    w: &'a mut W,
}
impl<'a> _Can2lpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Can2lpenW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `PWRLPEN`"]
pub type PwrlpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _PwrlpenW<'a> {
    w: &'a mut W,
}
impl<'a> _PwrlpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PwrlpenW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `DACLPEN`"]
pub type DaclpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _DaclpenW<'a> {
    w: &'a mut W,
}
impl<'a> _DaclpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DaclpenW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UART7LPEN`"]
pub type Uart7lpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Uart7lpenW<'a> {
    w: &'a mut W,
}
impl<'a> _Uart7lpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Uart7lpenW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UART8LPEN`"]
pub type Uart8lpenW = super::ahb1enr::OtghsulpienW;
#[doc = r" Proxy"]
pub struct _Uart8lpenW<'a> {
    w: &'a mut W,
}
impl<'a> _Uart8lpenW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Uart8lpenW) -> &'a mut W {
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
    #[doc = "Bit 0 - TIM2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim2lpen(&self) -> Tim2lpenR {
        Tim2lpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 1 - TIM3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim3lpen(&self) -> Tim3lpenR {
        Tim3lpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - TIM4 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim4lpen(&self) -> Tim4lpenR {
        Tim4lpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - TIM5 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim5lpen(&self) -> Tim5lpenR {
        Tim5lpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - TIM6 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim6lpen(&self) -> Tim6lpenR {
        Tim6lpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - TIM7 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim7lpen(&self) -> Tim7lpenR {
        Tim7lpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - TIM12 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim12lpen(&self) -> Tim12lpenR {
        Tim12lpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - TIM13 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim13lpen(&self) -> Tim13lpenR {
        Tim13lpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - TIM14 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim14lpen(&self) -> Tim14lpenR {
        Tim14lpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - Window watchdog clock enable during Sleep mode"]
    #[inline(always)]
    pub fn wwdglpen(&self) -> WwdglpenR {
        WwdglpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 14 - SPI2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi2lpen(&self) -> Spi2lpenR {
        Spi2lpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - SPI3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi3lpen(&self) -> Spi3lpenR {
        Spi3lpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 17 - USART2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart2lpen(&self) -> Usart2lpenR {
        Usart2lpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 18 - USART3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart3lpen(&self) -> Usart3lpenR {
        Usart3lpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 19 - UART4 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn uart4lpen(&self) -> Uart4lpenR {
        Uart4lpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - UART5 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn uart5lpen(&self) -> Uart5lpenR {
        Uart5lpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 21 - I2C1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c1lpen(&self) -> I2c1lpenR {
        I2c1lpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 22 - I2C2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c2lpen(&self) -> I2c2lpenR {
        I2c2lpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 23 - I2C3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c3lpen(&self) -> I2c3lpenR {
        I2c3lpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 25 - CAN 1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn can1lpen(&self) -> Can1lpenR {
        Can1lpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 26 - CAN 2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn can2lpen(&self) -> Can2lpenR {
        Can2lpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn pwrlpen(&self) -> PwrlpenR {
        PwrlpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 29 - DAC interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn daclpen(&self) -> DaclpenR {
        DaclpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 30 - UART7 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn uart7lpen(&self) -> Uart7lpenR {
        Uart7lpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 31 - UART8 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn uart8lpen(&self) -> Uart8lpenR {
        Uart8lpenR::_from({
            const MASK: u8 = 1;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline(always)]
    pub fn reset_value() -> W {
        W { bits: 922667519 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - TIM2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim2lpen(&mut self) -> _Tim2lpenW {
        _Tim2lpenW { w: self }
    }
    #[doc = "Bit 1 - TIM3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim3lpen(&mut self) -> _Tim3lpenW {
        _Tim3lpenW { w: self }
    }
    #[doc = "Bit 2 - TIM4 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim4lpen(&mut self) -> _Tim4lpenW {
        _Tim4lpenW { w: self }
    }
    #[doc = "Bit 3 - TIM5 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim5lpen(&mut self) -> _Tim5lpenW {
        _Tim5lpenW { w: self }
    }
    #[doc = "Bit 4 - TIM6 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim6lpen(&mut self) -> _Tim6lpenW {
        _Tim6lpenW { w: self }
    }
    #[doc = "Bit 5 - TIM7 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim7lpen(&mut self) -> _Tim7lpenW {
        _Tim7lpenW { w: self }
    }
    #[doc = "Bit 6 - TIM12 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim12lpen(&mut self) -> _Tim12lpenW {
        _Tim12lpenW { w: self }
    }
    #[doc = "Bit 7 - TIM13 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim13lpen(&mut self) -> _Tim13lpenW {
        _Tim13lpenW { w: self }
    }
    #[doc = "Bit 8 - TIM14 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim14lpen(&mut self) -> _Tim14lpenW {
        _Tim14lpenW { w: self }
    }
    #[doc = "Bit 11 - Window watchdog clock enable during Sleep mode"]
    #[inline(always)]
    pub fn wwdglpen(&mut self) -> _WwdglpenW {
        _WwdglpenW { w: self }
    }
    #[doc = "Bit 14 - SPI2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi2lpen(&mut self) -> _Spi2lpenW {
        _Spi2lpenW { w: self }
    }
    #[doc = "Bit 15 - SPI3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi3lpen(&mut self) -> _Spi3lpenW {
        _Spi3lpenW { w: self }
    }
    #[doc = "Bit 17 - USART2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart2lpen(&mut self) -> _Usart2lpenW {
        _Usart2lpenW { w: self }
    }
    #[doc = "Bit 18 - USART3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart3lpen(&mut self) -> _Usart3lpenW {
        _Usart3lpenW { w: self }
    }
    #[doc = "Bit 19 - UART4 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn uart4lpen(&mut self) -> _Uart4lpenW {
        _Uart4lpenW { w: self }
    }
    #[doc = "Bit 20 - UART5 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn uart5lpen(&mut self) -> _Uart5lpenW {
        _Uart5lpenW { w: self }
    }
    #[doc = "Bit 21 - I2C1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c1lpen(&mut self) -> _I2c1lpenW {
        _I2c1lpenW { w: self }
    }
    #[doc = "Bit 22 - I2C2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c2lpen(&mut self) -> _I2c2lpenW {
        _I2c2lpenW { w: self }
    }
    #[doc = "Bit 23 - I2C3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c3lpen(&mut self) -> _I2c3lpenW {
        _I2c3lpenW { w: self }
    }
    #[doc = "Bit 25 - CAN 1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn can1lpen(&mut self) -> _Can1lpenW {
        _Can1lpenW { w: self }
    }
    #[doc = "Bit 26 - CAN 2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn can2lpen(&mut self) -> _Can2lpenW {
        _Can2lpenW { w: self }
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn pwrlpen(&mut self) -> _PwrlpenW {
        _PwrlpenW { w: self }
    }
    #[doc = "Bit 29 - DAC interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn daclpen(&mut self) -> _DaclpenW {
        _DaclpenW { w: self }
    }
    #[doc = "Bit 30 - UART7 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn uart7lpen(&mut self) -> _Uart7lpenW {
        _Uart7lpenW { w: self }
    }
    #[doc = "Bit 31 - UART8 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn uart8lpen(&mut self) -> _Uart8lpenW {
        _Uart8lpenW { w: self }
    }
}
