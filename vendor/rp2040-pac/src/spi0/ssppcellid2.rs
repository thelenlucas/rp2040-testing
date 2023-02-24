#[doc = "Register `SSPPCELLID2` reader"]
pub struct R(crate::R<SSPPCELLID2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSPPCELLID2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSPPCELLID2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSPPCELLID2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SSPPCELLID2` reader - These bits read back as 0x05"]
pub struct SSPPCELLID2_R(crate::FieldReader<u8, u8>);
impl SSPPCELLID2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SSPPCELLID2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSPPCELLID2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0x05"]
    #[inline(always)]
    pub fn ssppcellid2(&self) -> SSPPCELLID2_R {
        SSPPCELLID2_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ssppcellid2](index.html) module"]
pub struct SSPPCELLID2_SPEC;
impl crate::RegisterSpec for SSPPCELLID2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssppcellid2::R](R) reader structure"]
impl crate::Readable for SSPPCELLID2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SSPPCELLID2 to value 0x05"]
impl crate::Resettable for SSPPCELLID2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x05
    }
}
