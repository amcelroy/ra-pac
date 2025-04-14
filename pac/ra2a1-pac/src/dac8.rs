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
// Generated from SVD 1.1, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:53 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"8-bit D/A converter"]
unsafe impl ::core::marker::Send for super::Dac8 {}
unsafe impl ::core::marker::Sync for super::Dac8 {}
impl super::Dac8 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn dacs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Dacs_SPEC, crate::common::RW>,
        2,
        0x1,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x0usize))
        }
    }

    #[inline(always)]
    pub const fn dam(&self) -> &'static crate::common::Reg<self::Dam_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dam_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dacadscr(
        &self,
    ) -> &'static crate::common::Reg<self::Dacadscr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dacadscr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dacpc(&self) -> &'static crate::common::Reg<self::Dacpc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dacpc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(7usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dacs_SPEC;
impl crate::sealed::RegSpec for Dacs_SPEC {
    type DataType = u8;
}

pub type Dacs = crate::RegValueT<Dacs_SPEC>;

impl Dacs {
    #[inline(always)]
    pub fn dacs(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dacs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dacs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dacs {
    #[inline(always)]
    fn default() -> Dacs {
        <crate::RegValueT<Dacs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dam_SPEC;
impl crate::sealed::RegSpec for Dam_SPEC {
    type DataType = u8;
}

pub type Dam = crate::RegValueT<Dam_SPEC>;

impl Dam {
    #[inline(always)]
    pub fn dace1(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dam::Dace1,
        dam::Dace1,
        Dam_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dam::Dace1,
            dam::Dace1,
            Dam_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dace0(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dam::Dace0,
        dam::Dace0,
        Dam_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dam::Dace0,
            dam::Dace0,
            Dam_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Dam_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Dam_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn damd1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dam::Damd1,
        dam::Damd1,
        Dam_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dam::Damd1,
            dam::Damd1,
            Dam_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn damd0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dam::Damd0,
        dam::Damd0,
        Dam_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dam::Damd0,
            dam::Damd0,
            Dam_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dam {
    #[inline(always)]
    fn default() -> Dam {
        <crate::RegValueT<Dam_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dam {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dace1_SPEC;
    pub type Dace1 = crate::EnumBitfieldStruct<u8, Dace1_SPEC>;
    impl Dace1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dace0_SPEC;
    pub type Dace0 = crate::EnumBitfieldStruct<u8, Dace0_SPEC>;
    impl Dace0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Damd1_SPEC;
    pub type Damd1 = crate::EnumBitfieldStruct<u8, Damd1_SPEC>;
    impl Damd1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Damd0_SPEC;
    pub type Damd0 = crate::EnumBitfieldStruct<u8, Damd0_SPEC>;
    impl Damd0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dacadscr_SPEC;
impl crate::sealed::RegSpec for Dacadscr_SPEC {
    type DataType = u8;
}

pub type Dacadscr = crate::RegValueT<Dacadscr_SPEC>;

impl Dacadscr {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Dacadscr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Dacadscr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dacadst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dacadscr::Dacadst,
        dacadscr::Dacadst,
        Dacadscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dacadscr::Dacadst,
            dacadscr::Dacadst,
            Dacadscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dacadscr {
    #[inline(always)]
    fn default() -> Dacadscr {
        <crate::RegValueT<Dacadscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dacadscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dacadst_SPEC;
    pub type Dacadst = crate::EnumBitfieldStruct<u8, Dacadst_SPEC>;
    impl Dacadst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dacpc_SPEC;
impl crate::sealed::RegSpec for Dacpc_SPEC {
    type DataType = u8;
}

pub type Dacpc = crate::RegValueT<Dacpc_SPEC>;

impl Dacpc {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Dacpc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Dacpc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pumpen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dacpc::Pumpen,
        dacpc::Pumpen,
        Dacpc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dacpc::Pumpen,
            dacpc::Pumpen,
            Dacpc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dacpc {
    #[inline(always)]
    fn default() -> Dacpc {
        <crate::RegValueT<Dacpc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dacpc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pumpen_SPEC;
    pub type Pumpen = crate::EnumBitfieldStruct<u8, Pumpen_SPEC>;
    impl Pumpen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
