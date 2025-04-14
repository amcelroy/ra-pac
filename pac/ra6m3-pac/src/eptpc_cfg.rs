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
#[doc = r"EPTPC Configulation"]
unsafe impl ::core::marker::Send for super::EptpcCfg {}
unsafe impl ::core::marker::Sync for super::EptpcCfg {}
impl super::EptpcCfg {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn ptrstr(
        &self,
    ) -> &'static crate::common::Reg<self::Ptrstr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ptrstr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn stcselr(
        &self,
    ) -> &'static crate::common::Reg<self::Stcselr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Stcselr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bypass(
        &self,
    ) -> &'static crate::common::Reg<self::Bypass_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bypass_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptrstr_SPEC;
impl crate::sealed::RegSpec for Ptrstr_SPEC {
    type DataType = u32;
}

pub type Ptrstr = crate::RegValueT<Ptrstr_SPEC>;

impl Ptrstr {
    #[inline(always)]
    pub fn reset(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ptrstr::Reset,
        ptrstr::Reset,
        Ptrstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ptrstr::Reset,
            ptrstr::Reset,
            Ptrstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ptrstr {
    #[inline(always)]
    fn default() -> Ptrstr {
        <crate::RegValueT<Ptrstr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ptrstr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Reset_SPEC;
    pub type Reset = crate::EnumBitfieldStruct<u8, Reset_SPEC>;
    impl Reset {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stcselr_SPEC;
impl crate::sealed::RegSpec for Stcselr_SPEC {
    type DataType = u32;
}

pub type Stcselr = crate::RegValueT<Stcselr_SPEC>;

impl Stcselr {
    #[inline(always)]
    pub fn sclksel(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        stcselr::Sclksel,
        stcselr::Sclksel,
        Stcselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            stcselr::Sclksel,
            stcselr::Sclksel,
            Stcselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sclkdiv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        stcselr::Sclkdiv,
        stcselr::Sclkdiv,
        Stcselr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            stcselr::Sclkdiv,
            stcselr::Sclkdiv,
            Stcselr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Stcselr {
    #[inline(always)]
    fn default() -> Stcselr {
        <crate::RegValueT<Stcselr_SPEC> as RegisterValue<_>>::new(6)
    }
}
pub mod stcselr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sclksel_SPEC;
    pub type Sclksel = crate::EnumBitfieldStruct<u8, Sclksel_SPEC>;
    impl Sclksel {
        pub const _000: Self = Self::new(0);

        pub const _010: Self = Self::new(2);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sclkdiv_SPEC;
    pub type Sclkdiv = crate::EnumBitfieldStruct<u8, Sclkdiv_SPEC>;
    impl Sclkdiv {
        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _101: Self = Self::new(5);

        pub const _110: Self = Self::new(6);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bypass_SPEC;
impl crate::sealed::RegSpec for Bypass_SPEC {
    type DataType = u32;
}

pub type Bypass = crate::RegValueT<Bypass_SPEC>;

impl Bypass {
    #[inline(always)]
    pub fn bypass0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bypass::Bypass0,
        bypass::Bypass0,
        Bypass_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bypass::Bypass0,
            bypass::Bypass0,
            Bypass_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bypass {
    #[inline(always)]
    fn default() -> Bypass {
        <crate::RegValueT<Bypass_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bypass {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bypass0_SPEC;
    pub type Bypass0 = crate::EnumBitfieldStruct<u8, Bypass0_SPEC>;
    impl Bypass0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
