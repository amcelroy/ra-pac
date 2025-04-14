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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:20:22 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Infrared Data Association"]
unsafe impl ::core::marker::Send for super::Irda {}
unsafe impl ::core::marker::Sync for super::Irda {}
impl super::Irda {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn ircr(&self) -> &'static crate::common::Reg<self::Ircr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ircr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ircr_SPEC;
impl crate::sealed::RegSpec for Ircr_SPEC {
    type DataType = u8;
}

pub type Ircr = crate::RegValueT<Ircr_SPEC>;

impl Ircr {
    #[inline(always)]
    pub fn ire(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ircr::Ire,
        ircr::Ire,
        Ircr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ircr::Ire,
            ircr::Ire,
            Ircr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn irtxinv(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ircr::Irtxinv,
        ircr::Irtxinv,
        Ircr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ircr::Irtxinv,
            ircr::Irtxinv,
            Ircr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn irrxinv(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ircr::Irrxinv,
        ircr::Irrxinv,
        Ircr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ircr::Irrxinv,
            ircr::Irrxinv,
            Ircr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ircr {
    #[inline(always)]
    fn default() -> Ircr {
        <crate::RegValueT<Ircr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ircr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ire_SPEC;
    pub type Ire = crate::EnumBitfieldStruct<u8, Ire_SPEC>;
    impl Ire {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irtxinv_SPEC;
    pub type Irtxinv = crate::EnumBitfieldStruct<u8, Irtxinv_SPEC>;
    impl Irtxinv {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irrxinv_SPEC;
    pub type Irrxinv = crate::EnumBitfieldStruct<u8, Irrxinv_SPEC>;
    impl Irrxinv {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
