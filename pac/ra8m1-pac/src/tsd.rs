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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:23:25 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Temperature Sensor Data"]
unsafe impl ::core::marker::Send for super::Tsd {}
unsafe impl ::core::marker::Sync for super::Tsd {}
impl super::Tsd {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn tscdr(&self) -> &'static crate::common::Reg<self::Tscdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Tscdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tscdr_SPEC;
impl crate::sealed::RegSpec for Tscdr_SPEC {
    type DataType = u32;
}

pub type Tscdr = crate::RegValueT<Tscdr_SPEC>;

impl Tscdr {
    #[inline(always)]
    pub fn tscd(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Tscdr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Tscdr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<12, 0xfffff, 1, 0, u32, u32, Tscdr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<12,0xfffff,1,0,u32,u32,Tscdr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Tscdr {
    #[inline(always)]
    fn default() -> Tscdr {
        <crate::RegValueT<Tscdr_SPEC> as RegisterValue<_>>::new(0)
    }
}
