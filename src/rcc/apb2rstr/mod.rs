#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::Apb2rstr {
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
pub struct Tim1rstR {
    bits: u8,
}
impl Tim1rstR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Tim8rstR {
    bits: u8,
}
impl Tim8rstR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Usart1rstR {
    bits: u8,
}
impl Usart1rstR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Usart6rstR {
    bits: u8,
}
impl Usart6rstR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AdcrstR {
    bits: u8,
}
impl AdcrstR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SdiorstR {
    bits: u8,
}
impl SdiorstR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Spi1rstR {
    bits: u8,
}
impl Spi1rstR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Spi4rstR {
    bits: u8,
}
impl Spi4rstR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SyscfgrstR {
    bits: u8,
}
impl SyscfgrstR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Tim9rstR {
    bits: u8,
}
impl Tim9rstR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Tim10rstR {
    bits: u8,
}
impl Tim10rstR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Tim11rstR {
    bits: u8,
}
impl Tim11rstR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Spi5rstR {
    bits: u8,
}
impl Spi5rstR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Spi6rstR {
    bits: u8,
}
impl Spi6rstR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Sai1rstR {
    bits: u8,
}
impl Sai1rstR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LtdcrstR {
    bits: u8,
}
impl LtdcrstR {
    #[doc = r" Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _Tim1rstW<'a> {
    w: &'a mut W,
}
impl<'a> _Tim1rstW<'a> {
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
pub struct _Tim8rstW<'a> {
    w: &'a mut W,
}
impl<'a> _Tim8rstW<'a> {
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
pub struct _Usart1rstW<'a> {
    w: &'a mut W,
}
impl<'a> _Usart1rstW<'a> {
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
pub struct _Usart6rstW<'a> {
    w: &'a mut W,
}
impl<'a> _Usart6rstW<'a> {
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
pub struct _AdcrstW<'a> {
    w: &'a mut W,
}
impl<'a> _AdcrstW<'a> {
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
pub struct _SdiorstW<'a> {
    w: &'a mut W,
}
impl<'a> _SdiorstW<'a> {
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
pub struct _Spi1rstW<'a> {
    w: &'a mut W,
}
impl<'a> _Spi1rstW<'a> {
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
pub struct _Spi4rstW<'a> {
    w: &'a mut W,
}
impl<'a> _Spi4rstW<'a> {
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
pub struct _SyscfgrstW<'a> {
    w: &'a mut W,
}
impl<'a> _SyscfgrstW<'a> {
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
pub struct _Tim9rstW<'a> {
    w: &'a mut W,
}
impl<'a> _Tim9rstW<'a> {
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
pub struct _Tim10rstW<'a> {
    w: &'a mut W,
}
impl<'a> _Tim10rstW<'a> {
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
pub struct _Tim11rstW<'a> {
    w: &'a mut W,
}
impl<'a> _Tim11rstW<'a> {
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
pub struct _Spi5rstW<'a> {
    w: &'a mut W,
}
impl<'a> _Spi5rstW<'a> {
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
pub struct _Spi6rstW<'a> {
    w: &'a mut W,
}
impl<'a> _Spi6rstW<'a> {
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
pub struct _Sai1rstW<'a> {
    w: &'a mut W,
}
impl<'a> _Sai1rstW<'a> {
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
pub struct _LtdcrstW<'a> {
    w: &'a mut W,
}
impl<'a> _LtdcrstW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - TIM1 reset"]
    #[inline(always)]
    pub fn tim1rst(&self) -> Tim1rstR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Tim1rstR { bits }
    }
    #[doc = "Bit 1 - TIM8 reset"]
    #[inline(always)]
    pub fn tim8rst(&self) -> Tim8rstR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Tim8rstR { bits }
    }
    #[doc = "Bit 4 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&self) -> Usart1rstR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Usart1rstR { bits }
    }
    #[doc = "Bit 5 - USART6 reset"]
    #[inline(always)]
    pub fn usart6rst(&self) -> Usart6rstR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Usart6rstR { bits }
    }
    #[doc = "Bit 8 - ADC interface reset (common to all ADCs)"]
    #[inline(always)]
    pub fn adcrst(&self) -> AdcrstR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AdcrstR { bits }
    }
    #[doc = "Bit 11 - SDIO reset"]
    #[inline(always)]
    pub fn sdiorst(&self) -> SdiorstR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SdiorstR { bits }
    }
    #[doc = "Bit 12 - SPI 1 reset"]
    #[inline(always)]
    pub fn spi1rst(&self) -> Spi1rstR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Spi1rstR { bits }
    }
    #[doc = "Bit 13 - SPI4 reset"]
    #[inline(always)]
    pub fn spi4rst(&self) -> Spi4rstR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Spi4rstR { bits }
    }
    #[doc = "Bit 14 - System configuration controller reset"]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SyscfgrstR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SyscfgrstR { bits }
    }
    #[doc = "Bit 16 - TIM9 reset"]
    #[inline(always)]
    pub fn tim9rst(&self) -> Tim9rstR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Tim9rstR { bits }
    }
    #[doc = "Bit 17 - TIM10 reset"]
    #[inline(always)]
    pub fn tim10rst(&self) -> Tim10rstR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Tim10rstR { bits }
    }
    #[doc = "Bit 18 - TIM11 reset"]
    #[inline(always)]
    pub fn tim11rst(&self) -> Tim11rstR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Tim11rstR { bits }
    }
    #[doc = "Bit 20 - SPI5 reset"]
    #[inline(always)]
    pub fn spi5rst(&self) -> Spi5rstR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Spi5rstR { bits }
    }
    #[doc = "Bit 21 - SPI6 reset"]
    #[inline(always)]
    pub fn spi6rst(&self) -> Spi6rstR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Spi6rstR { bits }
    }
    #[doc = "Bit 22 - SAI1 reset"]
    #[inline(always)]
    pub fn sai1rst(&self) -> Sai1rstR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Sai1rstR { bits }
    }
    #[doc = "Bit 26 - LTDC reset"]
    #[inline(always)]
    pub fn ltdcrst(&self) -> LtdcrstR {
        let bits = {
            const MASK: u8 = 1;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LtdcrstR { bits }
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
    #[doc = "Bit 0 - TIM1 reset"]
    #[inline(always)]
    pub fn tim1rst(&mut self) -> _Tim1rstW {
        _Tim1rstW { w: self }
    }
    #[doc = "Bit 1 - TIM8 reset"]
    #[inline(always)]
    pub fn tim8rst(&mut self) -> _Tim8rstW {
        _Tim8rstW { w: self }
    }
    #[doc = "Bit 4 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&mut self) -> _Usart1rstW {
        _Usart1rstW { w: self }
    }
    #[doc = "Bit 5 - USART6 reset"]
    #[inline(always)]
    pub fn usart6rst(&mut self) -> _Usart6rstW {
        _Usart6rstW { w: self }
    }
    #[doc = "Bit 8 - ADC interface reset (common to all ADCs)"]
    #[inline(always)]
    pub fn adcrst(&mut self) -> _AdcrstW {
        _AdcrstW { w: self }
    }
    #[doc = "Bit 11 - SDIO reset"]
    #[inline(always)]
    pub fn sdiorst(&mut self) -> _SdiorstW {
        _SdiorstW { w: self }
    }
    #[doc = "Bit 12 - SPI 1 reset"]
    #[inline(always)]
    pub fn spi1rst(&mut self) -> _Spi1rstW {
        _Spi1rstW { w: self }
    }
    #[doc = "Bit 13 - SPI4 reset"]
    #[inline(always)]
    pub fn spi4rst(&mut self) -> _Spi4rstW {
        _Spi4rstW { w: self }
    }
    #[doc = "Bit 14 - System configuration controller reset"]
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> _SyscfgrstW {
        _SyscfgrstW { w: self }
    }
    #[doc = "Bit 16 - TIM9 reset"]
    #[inline(always)]
    pub fn tim9rst(&mut self) -> _Tim9rstW {
        _Tim9rstW { w: self }
    }
    #[doc = "Bit 17 - TIM10 reset"]
    #[inline(always)]
    pub fn tim10rst(&mut self) -> _Tim10rstW {
        _Tim10rstW { w: self }
    }
    #[doc = "Bit 18 - TIM11 reset"]
    #[inline(always)]
    pub fn tim11rst(&mut self) -> _Tim11rstW {
        _Tim11rstW { w: self }
    }
    #[doc = "Bit 20 - SPI5 reset"]
    #[inline(always)]
    pub fn spi5rst(&mut self) -> _Spi5rstW {
        _Spi5rstW { w: self }
    }
    #[doc = "Bit 21 - SPI6 reset"]
    #[inline(always)]
    pub fn spi6rst(&mut self) -> _Spi6rstW {
        _Spi6rstW { w: self }
    }
    #[doc = "Bit 22 - SAI1 reset"]
    #[inline(always)]
    pub fn sai1rst(&mut self) -> _Sai1rstW {
        _Sai1rstW { w: self }
    }
    #[doc = "Bit 26 - LTDC reset"]
    #[inline(always)]
    pub fn ltdcrst(&mut self) -> _LtdcrstW {
        _LtdcrstW { w: self }
    }
}
