#[doc = "Reader of register OUTTGL%s"]
pub type R = crate::R<u32, super::OUTTGL>;
#[doc = "Writer for register OUTTGL%s"]
pub type W = crate::W<u32, super::OUTTGL>;
#[doc = "Register OUTTGL%s `reset()`'s with value 0"]
impl crate::ResetValue for super::OUTTGL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OUTTGL`"]
pub type OUTTGL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `OUTTGL`"]
pub struct OUTTGL_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTGL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Port Data Output Value Toggle"]
    #[inline(always)]
    pub fn outtgl(&self) -> OUTTGL_R {
        OUTTGL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Output Value Toggle"]
    #[inline(always)]
    pub fn outtgl(&mut self) -> OUTTGL_W {
        OUTTGL_W { w: self }
    }
}
