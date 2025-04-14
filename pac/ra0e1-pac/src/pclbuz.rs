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
// Generated from SVD 1.10.00, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:47 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Clock Output/Buzzer Output Controller"]
unsafe impl ::core::marker::Send for super::Pclbuz {}
unsafe impl ::core::marker::Sync for super::Pclbuz {}
impl super::Pclbuz {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn cks0(&self) -> &'static crate::common::Reg<self::Cks0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cks0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cks0_SPEC;
impl crate::sealed::RegSpec for Cks0_SPEC {
    type DataType = u8;
}

pub type Cks0 = crate::RegValueT<Cks0_SPEC>;

impl Cks0 {
    #[inline(always)]
    pub fn ccs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        cks0::Ccs,
        cks0::Ccs,
        Cks0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            cks0::Ccs,
            cks0::Ccs,
            Cks0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn csel(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        cks0::Csel,
        cks0::Csel,
        Cks0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            cks0::Csel,
            cks0::Csel,
            Cks0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pcloe(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        cks0::Pcloe,
        cks0::Pcloe,
        Cks0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            cks0::Pcloe,
            cks0::Pcloe,
            Cks0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cks0 {
    #[inline(always)]
    fn default() -> Cks0 {
        <crate::RegValueT<Cks0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cks0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccs_SPEC;
    pub type Ccs = crate::EnumBitfieldStruct<u8, Ccs_SPEC>;
    impl Ccs {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _101: Self = Self::new(5);

        pub const _110: Self = Self::new(6);

        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csel_SPEC;
    pub type Csel = crate::EnumBitfieldStruct<u8, Csel_SPEC>;
    impl Csel {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcloe_SPEC;
    pub type Pcloe = crate::EnumBitfieldStruct<u8, Pcloe_SPEC>;
    impl Pcloe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
