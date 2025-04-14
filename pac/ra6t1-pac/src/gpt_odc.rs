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
// Generated from SVD 1.0, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:21:11 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"PWM Delay Generation Circuit"]
unsafe impl ::core::marker::Send for super::GptOdc {}
unsafe impl ::core::marker::Sync for super::GptOdc {}
impl super::GptOdc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn gtdlycr(
        &self,
    ) -> &'static crate::common::Reg<self::Gtdlycr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtdlycr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtdlycr2(
        &self,
    ) -> &'static crate::common::Reg<self::Gtdlycr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtdlycr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtdlyra(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gtdlyra_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x18usize))
        }
    }

    #[inline(always)]
    pub const fn gtdlyrb(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gtdlyrb_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1ausize))
        }
    }

    #[inline(always)]
    pub const fn gtdlyfa(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gtdlyfa_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x28usize))
        }
    }

    #[inline(always)]
    pub const fn gtdlyfb(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gtdlyfb_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2ausize))
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtdlycr_SPEC;
impl crate::sealed::RegSpec for Gtdlycr_SPEC {
    type DataType = u16;
}

pub type Gtdlycr = crate::RegValueT<Gtdlycr_SPEC>;

impl Gtdlycr {
    #[inline(always)]
    pub fn dlyrst(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gtdlycr::Dlyrst,
        gtdlycr::Dlyrst,
        Gtdlycr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gtdlycr::Dlyrst,
            gtdlycr::Dlyrst,
            Gtdlycr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dllen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtdlycr::Dllen,
        gtdlycr::Dllen,
        Gtdlycr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtdlycr::Dllen,
            gtdlycr::Dllen,
            Gtdlycr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtdlycr {
    #[inline(always)]
    fn default() -> Gtdlycr {
        <crate::RegValueT<Gtdlycr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtdlycr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlyrst_SPEC;
    pub type Dlyrst = crate::EnumBitfieldStruct<u8, Dlyrst_SPEC>;
    impl Dlyrst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dllen_SPEC;
    pub type Dllen = crate::EnumBitfieldStruct<u8, Dllen_SPEC>;
    impl Dllen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtdlycr2_SPEC;
impl crate::sealed::RegSpec for Gtdlycr2_SPEC {
    type DataType = u16;
}

pub type Gtdlycr2 = crate::RegValueT<Gtdlycr2_SPEC>;

impl Gtdlycr2 {
    #[inline(always)]
    pub fn dlyen3(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        gtdlycr2::Dlyen3,
        gtdlycr2::Dlyen3,
        Gtdlycr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            gtdlycr2::Dlyen3,
            gtdlycr2::Dlyen3,
            Gtdlycr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dlyen2(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        gtdlycr2::Dlyen2,
        gtdlycr2::Dlyen2,
        Gtdlycr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            gtdlycr2::Dlyen2,
            gtdlycr2::Dlyen2,
            Gtdlycr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dlyen1(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        gtdlycr2::Dlyen1,
        gtdlycr2::Dlyen1,
        Gtdlycr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            gtdlycr2::Dlyen1,
            gtdlycr2::Dlyen1,
            Gtdlycr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dlyen0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gtdlycr2::Dlyen0,
        gtdlycr2::Dlyen0,
        Gtdlycr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gtdlycr2::Dlyen0,
            gtdlycr2::Dlyen0,
            Gtdlycr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dlybs3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gtdlycr2::Dlybs3,
        gtdlycr2::Dlybs3,
        Gtdlycr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gtdlycr2::Dlybs3,
            gtdlycr2::Dlybs3,
            Gtdlycr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dlybs2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gtdlycr2::Dlybs2,
        gtdlycr2::Dlybs2,
        Gtdlycr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gtdlycr2::Dlybs2,
            gtdlycr2::Dlybs2,
            Gtdlycr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dlybs1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gtdlycr2::Dlybs1,
        gtdlycr2::Dlybs1,
        Gtdlycr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gtdlycr2::Dlybs1,
            gtdlycr2::Dlybs1,
            Gtdlycr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dlybs0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtdlycr2::Dlybs0,
        gtdlycr2::Dlybs0,
        Gtdlycr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtdlycr2::Dlybs0,
            gtdlycr2::Dlybs0,
            Gtdlycr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtdlycr2 {
    #[inline(always)]
    fn default() -> Gtdlycr2 {
        <crate::RegValueT<Gtdlycr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtdlycr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlyen3_SPEC;
    pub type Dlyen3 = crate::EnumBitfieldStruct<u8, Dlyen3_SPEC>;
    impl Dlyen3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlyen2_SPEC;
    pub type Dlyen2 = crate::EnumBitfieldStruct<u8, Dlyen2_SPEC>;
    impl Dlyen2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlyen1_SPEC;
    pub type Dlyen1 = crate::EnumBitfieldStruct<u8, Dlyen1_SPEC>;
    impl Dlyen1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlyen0_SPEC;
    pub type Dlyen0 = crate::EnumBitfieldStruct<u8, Dlyen0_SPEC>;
    impl Dlyen0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlybs3_SPEC;
    pub type Dlybs3 = crate::EnumBitfieldStruct<u8, Dlybs3_SPEC>;
    impl Dlybs3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlybs2_SPEC;
    pub type Dlybs2 = crate::EnumBitfieldStruct<u8, Dlybs2_SPEC>;
    impl Dlybs2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlybs1_SPEC;
    pub type Dlybs1 = crate::EnumBitfieldStruct<u8, Dlybs1_SPEC>;
    impl Dlybs1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlybs0_SPEC;
    pub type Dlybs0 = crate::EnumBitfieldStruct<u8, Dlybs0_SPEC>;
    impl Dlybs0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtdlyra_SPEC;
impl crate::sealed::RegSpec for Gtdlyra_SPEC {
    type DataType = u16;
}

pub type Gtdlyra = crate::RegValueT<Gtdlyra_SPEC>;

impl Gtdlyra {
    #[inline(always)]
    pub fn dly(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        gtdlyra::Dly,
        gtdlyra::Dly,
        Gtdlyra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            gtdlyra::Dly,
            gtdlyra::Dly,
            Gtdlyra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtdlyra {
    #[inline(always)]
    fn default() -> Gtdlyra {
        <crate::RegValueT<Gtdlyra_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtdlyra {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dly_SPEC;
    pub type Dly = crate::EnumBitfieldStruct<u8, Dly_SPEC>;
    impl Dly {
        pub const _00000: Self = Self::new(0);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtdlyrb_SPEC;
impl crate::sealed::RegSpec for Gtdlyrb_SPEC {
    type DataType = u16;
}

pub type Gtdlyrb = crate::RegValueT<Gtdlyrb_SPEC>;

impl Gtdlyrb {
    #[inline(always)]
    pub fn dly(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        gtdlyrb::Dly,
        gtdlyrb::Dly,
        Gtdlyrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            gtdlyrb::Dly,
            gtdlyrb::Dly,
            Gtdlyrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtdlyrb {
    #[inline(always)]
    fn default() -> Gtdlyrb {
        <crate::RegValueT<Gtdlyrb_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtdlyrb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dly_SPEC;
    pub type Dly = crate::EnumBitfieldStruct<u8, Dly_SPEC>;
    impl Dly {
        pub const _00000: Self = Self::new(0);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtdlyfa_SPEC;
impl crate::sealed::RegSpec for Gtdlyfa_SPEC {
    type DataType = u16;
}

pub type Gtdlyfa = crate::RegValueT<Gtdlyfa_SPEC>;

impl Gtdlyfa {
    #[inline(always)]
    pub fn dly(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        gtdlyfa::Dly,
        gtdlyfa::Dly,
        Gtdlyfa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            gtdlyfa::Dly,
            gtdlyfa::Dly,
            Gtdlyfa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtdlyfa {
    #[inline(always)]
    fn default() -> Gtdlyfa {
        <crate::RegValueT<Gtdlyfa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtdlyfa {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dly_SPEC;
    pub type Dly = crate::EnumBitfieldStruct<u8, Dly_SPEC>;
    impl Dly {
        pub const _00000: Self = Self::new(0);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtdlyfb_SPEC;
impl crate::sealed::RegSpec for Gtdlyfb_SPEC {
    type DataType = u16;
}

pub type Gtdlyfb = crate::RegValueT<Gtdlyfb_SPEC>;

impl Gtdlyfb {
    #[inline(always)]
    pub fn dly(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        gtdlyfb::Dly,
        gtdlyfb::Dly,
        Gtdlyfb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            gtdlyfb::Dly,
            gtdlyfb::Dly,
            Gtdlyfb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtdlyfb {
    #[inline(always)]
    fn default() -> Gtdlyfb {
        <crate::RegValueT<Gtdlyfb_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtdlyfb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dly_SPEC;
    pub type Dly = crate::EnumBitfieldStruct<u8, Dly_SPEC>;
    impl Dly {
        pub const _00000: Self = Self::new(0);

        pub const OTHERS: Self = Self::new(0);
    }
}
