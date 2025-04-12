/*
DISCLAIMER
This software is supplied by Renesas Electronics Corporation and is only intended for use with Renesas products.
No other uses are authorized. This software is owned by Renesas Electronics Corporation and is protected under all
applicable laws, including copyright laws.
THIS SOFTWARE IS PROVIDED "AS IS" AND RENESAS MAKES NO WARRANTIES REGARDING THIS SOFTWARE, WHETHER EXPRESS, IMPLIED
OR STATUTORY, INCLUDING BUT NOT LIMITED TO WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NON-INFRINGEMENT.  ALL SUCH WARRANTIES ARE EXPRESSLY DISCLAIMED.TO THE MAXIMUM EXTENT PERMITTED NOT PROHIBITED BY
LAW, NEITHER RENESAS ELECTRONICS CORPORATION NOR ANY OF ITS AFFILIATED COMPANIES SHALL BE LIABLE FOR ANY DIRECT,
INDIRECT, SPECIAL, INCIDENTAL OR CONSEQUENTIAL DAMAGES FOR ANY REASON RELATED TO THIS SOFTWARE, EVEN IF RENESAS OR
ITS AFFILIATES HAVE BEEN ADVISED OF THE POSSIBILITY OF SUCH DAMAGES.
Renesas reserves the right, without notice, to make changes to this software and to discontinue the availability
of this software. By using this software, you agree to the additional terms and conditions found by accessing the
following link:
http://www.renesas.com/disclaimer

*/
// Generated from SVD 1.0, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:18:24 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Sampling Rate Converter RAM"]
unsafe impl ::core::marker::Send for super::Srcram {}
unsafe impl ::core::marker::Sync for super::Srcram {}
impl super::Srcram {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "Filter Coefficient Table \\[%s\\]"]
    #[inline(always)]
    pub const fn srcfctr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Srcfctr_SPEC, crate::common::RW>,
        5552,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x0usize))
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srcfctr_SPEC;
impl crate::sealed::RegSpec for Srcfctr_SPEC {
    type DataType = u32;
}
#[doc = "Filter Coefficient Table \\[%s\\]"]
pub type Srcfctr = crate::RegValueT<Srcfctr_SPEC>;

impl Srcfctr {
    #[doc = "Stores a filter coefficient value."]
    #[inline(always)]
    pub fn srcfcoe(
        self,
    ) -> crate::common::RegisterField<0, 0x3fffff, 1, 0, u32, Srcfctr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3fffff,1,0,u32, Srcfctr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Srcfctr {
    #[inline(always)]
    fn default() -> Srcfctr {
        <crate::RegValueT<Srcfctr_SPEC> as RegisterValue<_>>::new(0)
    }
}
