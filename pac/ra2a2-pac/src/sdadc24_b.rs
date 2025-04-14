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
// Generated from SVD 1.20.02, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:17:03 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"24-Bit Sigma-Delta A/D Converter B"]
unsafe impl ::core::marker::Send for super::Sdadc24B {}
unsafe impl ::core::marker::Sync for super::Sdadc24B {}
impl super::Sdadc24B {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn sdadccr(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdadccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sdadmr(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadmr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdadmr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sdadrr(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadrr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdadrr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sdadgcr(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadgcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdadgcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sdadhpfcr(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadhpfcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdadhpfcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sdadicr(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadicr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdadicr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sdadiclr(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadiclr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdadiclr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sdadisr(
        &self,
    ) -> &'static crate::common::Reg<self::Sdadisr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdadisr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sdadphcr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Sdadphcr_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x40usize))
        }
    }

    #[inline(always)]
    pub const fn sdadcr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Sdadcr_SPEC, crate::common::R>,
        7,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x80usize))
        }
    }

    #[inline(always)]
    pub const fn sdadcrt2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Sdadcrt2_SPEC, crate::common::R>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xa0usize))
        }
    }

    #[inline(always)]
    pub const fn sdadcrlpf(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Sdadcrlpf_SPEC, crate::common::R>,
        7,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xc0usize))
        }
    }

    #[inline(always)]
    pub const fn sdadcrlpft2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Sdadcrlpft2_SPEC, crate::common::R>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xe0usize))
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdadccr_SPEC;
impl crate::sealed::RegSpec for Sdadccr_SPEC {
    type DataType = u8;
}

pub type Sdadccr = crate::RegValueT<Sdadccr_SPEC>;

impl Sdadccr {
    #[inline(always)]
    pub fn ck(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        sdadccr::Ck,
        sdadccr::Ck,
        Sdadccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            sdadccr::Ck,
            sdadccr::Ck,
            Sdadccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sdadccr {
    #[inline(always)]
    fn default() -> Sdadccr {
        <crate::RegValueT<Sdadccr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdadccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ck_SPEC;
    pub type Ck = crate::EnumBitfieldStruct<u8, Ck_SPEC>;
    impl Ck {
        pub const _00: Self = Self::new(0);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdadmr_SPEC;
impl crate::sealed::RegSpec for Sdadmr_SPEC {
    type DataType = u32;
}

pub type Sdadmr = crate::RegValueT<Sdadmr_SPEC>;

impl Sdadmr {
    #[inline(always)]
    pub fn ce0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sdadmr::Ce0,
        sdadmr::Ce0,
        Sdadmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sdadmr::Ce0,
            sdadmr::Ce0,
            Sdadmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ce1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        sdadmr::Ce1,
        sdadmr::Ce1,
        Sdadmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sdadmr::Ce1,
            sdadmr::Ce1,
            Sdadmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ce2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        sdadmr::Ce2,
        sdadmr::Ce2,
        Sdadmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            sdadmr::Ce2,
            sdadmr::Ce2,
            Sdadmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ce3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        sdadmr::Ce3,
        sdadmr::Ce3,
        Sdadmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            sdadmr::Ce3,
            sdadmr::Ce3,
            Sdadmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ce4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sdadmr::Ce4,
        sdadmr::Ce4,
        Sdadmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sdadmr::Ce4,
            sdadmr::Ce4,
            Sdadmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ce5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        sdadmr::Ce5,
        sdadmr::Ce5,
        Sdadmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            sdadmr::Ce5,
            sdadmr::Ce5,
            Sdadmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ce6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        sdadmr::Ce6,
        sdadmr::Ce6,
        Sdadmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            sdadmr::Ce6,
            sdadmr::Ce6,
            Sdadmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pon0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        sdadmr::Pon0,
        sdadmr::Pon0,
        Sdadmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            sdadmr::Pon0,
            sdadmr::Pon0,
            Sdadmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pon1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        sdadmr::Pon1,
        sdadmr::Pon1,
        Sdadmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            sdadmr::Pon1,
            sdadmr::Pon1,
            Sdadmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pon2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        sdadmr::Pon2,
        sdadmr::Pon2,
        Sdadmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            sdadmr::Pon2,
            sdadmr::Pon2,
            Sdadmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pon3(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        sdadmr::Pon3,
        sdadmr::Pon3,
        Sdadmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            sdadmr::Pon3,
            sdadmr::Pon3,
            Sdadmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pon4(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        sdadmr::Pon4,
        sdadmr::Pon4,
        Sdadmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            sdadmr::Pon4,
            sdadmr::Pon4,
            Sdadmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pon5(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        sdadmr::Pon5,
        sdadmr::Pon5,
        Sdadmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            sdadmr::Pon5,
            sdadmr::Pon5,
            Sdadmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pon6(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        sdadmr::Pon6,
        sdadmr::Pon6,
        Sdadmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            sdadmr::Pon6,
            sdadmr::Pon6,
            Sdadmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fr(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x3,
        1,
        0,
        sdadmr::Fr,
        sdadmr::Fr,
        Sdadmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x3,
            1,
            0,
            sdadmr::Fr,
            sdadmr::Fr,
            Sdadmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn typ(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        sdadmr::Typ,
        sdadmr::Typ,
        Sdadmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            sdadmr::Typ,
            sdadmr::Typ,
            Sdadmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sdadmr {
    #[inline(always)]
    fn default() -> Sdadmr {
        <crate::RegValueT<Sdadmr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdadmr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ce0_SPEC;
    pub type Ce0 = crate::EnumBitfieldStruct<u8, Ce0_SPEC>;
    impl Ce0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ce1_SPEC;
    pub type Ce1 = crate::EnumBitfieldStruct<u8, Ce1_SPEC>;
    impl Ce1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ce2_SPEC;
    pub type Ce2 = crate::EnumBitfieldStruct<u8, Ce2_SPEC>;
    impl Ce2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ce3_SPEC;
    pub type Ce3 = crate::EnumBitfieldStruct<u8, Ce3_SPEC>;
    impl Ce3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ce4_SPEC;
    pub type Ce4 = crate::EnumBitfieldStruct<u8, Ce4_SPEC>;
    impl Ce4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ce5_SPEC;
    pub type Ce5 = crate::EnumBitfieldStruct<u8, Ce5_SPEC>;
    impl Ce5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ce6_SPEC;
    pub type Ce6 = crate::EnumBitfieldStruct<u8, Ce6_SPEC>;
    impl Ce6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pon0_SPEC;
    pub type Pon0 = crate::EnumBitfieldStruct<u8, Pon0_SPEC>;
    impl Pon0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pon1_SPEC;
    pub type Pon1 = crate::EnumBitfieldStruct<u8, Pon1_SPEC>;
    impl Pon1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pon2_SPEC;
    pub type Pon2 = crate::EnumBitfieldStruct<u8, Pon2_SPEC>;
    impl Pon2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pon3_SPEC;
    pub type Pon3 = crate::EnumBitfieldStruct<u8, Pon3_SPEC>;
    impl Pon3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pon4_SPEC;
    pub type Pon4 = crate::EnumBitfieldStruct<u8, Pon4_SPEC>;
    impl Pon4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pon5_SPEC;
    pub type Pon5 = crate::EnumBitfieldStruct<u8, Pon5_SPEC>;
    impl Pon5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pon6_SPEC;
    pub type Pon6 = crate::EnumBitfieldStruct<u8, Pon6_SPEC>;
    impl Pon6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fr_SPEC;
    pub type Fr = crate::EnumBitfieldStruct<u8, Fr_SPEC>;
    impl Fr {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Typ_SPEC;
    pub type Typ = crate::EnumBitfieldStruct<u8, Typ_SPEC>;
    impl Typ {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdadrr_SPEC;
impl crate::sealed::RegSpec for Sdadrr_SPEC {
    type DataType = u8;
}

pub type Sdadrr = crate::RegValueT<Sdadrr_SPEC>;

impl Sdadrr {
    #[inline(always)]
    pub fn res(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sdadrr::Res,
        sdadrr::Res,
        Sdadrr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sdadrr::Res,
            sdadrr::Res,
            Sdadrr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sdadrr {
    #[inline(always)]
    fn default() -> Sdadrr {
        <crate::RegValueT<Sdadrr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdadrr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Res_SPEC;
    pub type Res = crate::EnumBitfieldStruct<u8, Res_SPEC>;
    impl Res {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdadgcr_SPEC;
impl crate::sealed::RegSpec for Sdadgcr_SPEC {
    type DataType = u32;
}

pub type Sdadgcr = crate::RegValueT<Sdadgcr_SPEC>;

impl Sdadgcr {
    #[inline(always)]
    pub fn gain0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        sdadgcr::Gain0,
        sdadgcr::Gain0,
        Sdadgcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            sdadgcr::Gain0,
            sdadgcr::Gain0,
            Sdadgcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gain1(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x7,
        1,
        0,
        sdadgcr::Gain1,
        sdadgcr::Gain1,
        Sdadgcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            sdadgcr::Gain1,
            sdadgcr::Gain1,
            Sdadgcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gain2(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        sdadgcr::Gain2,
        sdadgcr::Gain2,
        Sdadgcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            sdadgcr::Gain2,
            sdadgcr::Gain2,
            Sdadgcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gain3(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x7,
        1,
        0,
        sdadgcr::Gain3,
        sdadgcr::Gain3,
        Sdadgcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x7,
            1,
            0,
            sdadgcr::Gain3,
            sdadgcr::Gain3,
            Sdadgcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gain4(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        sdadgcr::Gain4,
        sdadgcr::Gain4,
        Sdadgcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            sdadgcr::Gain4,
            sdadgcr::Gain4,
            Sdadgcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gain5(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x7,
        1,
        0,
        sdadgcr::Gain5,
        sdadgcr::Gain5,
        Sdadgcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x7,
            1,
            0,
            sdadgcr::Gain5,
            sdadgcr::Gain5,
            Sdadgcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gain6(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        sdadgcr::Gain6,
        sdadgcr::Gain6,
        Sdadgcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            sdadgcr::Gain6,
            sdadgcr::Gain6,
            Sdadgcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sdadgcr {
    #[inline(always)]
    fn default() -> Sdadgcr {
        <crate::RegValueT<Sdadgcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdadgcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain0_SPEC;
    pub type Gain0 = crate::EnumBitfieldStruct<u8, Gain0_SPEC>;
    impl Gain0 {
        pub const _0_X_0: Self = Self::new(0);

        pub const _0_X_1: Self = Self::new(1);

        pub const _0_X_2: Self = Self::new(2);

        pub const _0_X_3: Self = Self::new(3);

        pub const _0_X_4: Self = Self::new(4);

        pub const _0_X_5: Self = Self::new(5);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain1_SPEC;
    pub type Gain1 = crate::EnumBitfieldStruct<u8, Gain1_SPEC>;
    impl Gain1 {
        pub const _0_X_0: Self = Self::new(0);

        pub const _0_X_1: Self = Self::new(1);

        pub const _0_X_2: Self = Self::new(2);

        pub const _0_X_3: Self = Self::new(3);

        pub const _0_X_4: Self = Self::new(4);

        pub const _0_X_5: Self = Self::new(5);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain2_SPEC;
    pub type Gain2 = crate::EnumBitfieldStruct<u8, Gain2_SPEC>;
    impl Gain2 {
        pub const _0_X_0: Self = Self::new(0);

        pub const _0_X_1: Self = Self::new(1);

        pub const _0_X_2: Self = Self::new(2);

        pub const _0_X_3: Self = Self::new(3);

        pub const _0_X_4: Self = Self::new(4);

        pub const _0_X_5: Self = Self::new(5);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain3_SPEC;
    pub type Gain3 = crate::EnumBitfieldStruct<u8, Gain3_SPEC>;
    impl Gain3 {
        pub const _0_X_0: Self = Self::new(0);

        pub const _0_X_1: Self = Self::new(1);

        pub const _0_X_2: Self = Self::new(2);

        pub const _0_X_3: Self = Self::new(3);

        pub const _0_X_4: Self = Self::new(4);

        pub const _0_X_5: Self = Self::new(5);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain4_SPEC;
    pub type Gain4 = crate::EnumBitfieldStruct<u8, Gain4_SPEC>;
    impl Gain4 {
        pub const _0_X_0: Self = Self::new(0);

        pub const _0_X_1: Self = Self::new(1);

        pub const _0_X_2: Self = Self::new(2);

        pub const _0_X_3: Self = Self::new(3);

        pub const _0_X_4: Self = Self::new(4);

        pub const _0_X_5: Self = Self::new(5);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain5_SPEC;
    pub type Gain5 = crate::EnumBitfieldStruct<u8, Gain5_SPEC>;
    impl Gain5 {
        pub const _0_X_0: Self = Self::new(0);

        pub const _0_X_1: Self = Self::new(1);

        pub const _0_X_2: Self = Self::new(2);

        pub const _0_X_3: Self = Self::new(3);

        pub const _0_X_4: Self = Self::new(4);

        pub const _0_X_5: Self = Self::new(5);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain6_SPEC;
    pub type Gain6 = crate::EnumBitfieldStruct<u8, Gain6_SPEC>;
    impl Gain6 {
        pub const _0_X_0: Self = Self::new(0);

        pub const _0_X_1: Self = Self::new(1);

        pub const _0_X_2: Self = Self::new(2);

        pub const _0_X_3: Self = Self::new(3);

        pub const _0_X_4: Self = Self::new(4);

        pub const _0_X_5: Self = Self::new(5);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdadhpfcr_SPEC;
impl crate::sealed::RegSpec for Sdadhpfcr_SPEC {
    type DataType = u32;
}

pub type Sdadhpfcr = crate::RegValueT<Sdadhpfcr_SPEC>;

impl Sdadhpfcr {
    #[inline(always)]
    pub fn dis0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sdadhpfcr::Dis0,
        sdadhpfcr::Dis0,
        Sdadhpfcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sdadhpfcr::Dis0,
            sdadhpfcr::Dis0,
            Sdadhpfcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dis1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        sdadhpfcr::Dis1,
        sdadhpfcr::Dis1,
        Sdadhpfcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sdadhpfcr::Dis1,
            sdadhpfcr::Dis1,
            Sdadhpfcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dis2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        sdadhpfcr::Dis2,
        sdadhpfcr::Dis2,
        Sdadhpfcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            sdadhpfcr::Dis2,
            sdadhpfcr::Dis2,
            Sdadhpfcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dis3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        sdadhpfcr::Dis3,
        sdadhpfcr::Dis3,
        Sdadhpfcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            sdadhpfcr::Dis3,
            sdadhpfcr::Dis3,
            Sdadhpfcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dis4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sdadhpfcr::Dis4,
        sdadhpfcr::Dis4,
        Sdadhpfcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sdadhpfcr::Dis4,
            sdadhpfcr::Dis4,
            Sdadhpfcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dis5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        sdadhpfcr::Dis5,
        sdadhpfcr::Dis5,
        Sdadhpfcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            sdadhpfcr::Dis5,
            sdadhpfcr::Dis5,
            Sdadhpfcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dis6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        sdadhpfcr::Dis6,
        sdadhpfcr::Dis6,
        Sdadhpfcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            sdadhpfcr::Dis6,
            sdadhpfcr::Dis6,
            Sdadhpfcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dis3t2(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        sdadhpfcr::Dis3T2,
        sdadhpfcr::Dis3T2,
        Sdadhpfcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            sdadhpfcr::Dis3T2,
            sdadhpfcr::Dis3T2,
            Sdadhpfcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dis0t2(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        sdadhpfcr::Dis0T2,
        sdadhpfcr::Dis0T2,
        Sdadhpfcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            sdadhpfcr::Dis0T2,
            sdadhpfcr::Dis0T2,
            Sdadhpfcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cof(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, u8, Sdadhpfcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x3,1,0,u8,u8,Sdadhpfcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sdadhpfcr {
    #[inline(always)]
    fn default() -> Sdadhpfcr {
        <crate::RegValueT<Sdadhpfcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdadhpfcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dis0_SPEC;
    pub type Dis0 = crate::EnumBitfieldStruct<u8, Dis0_SPEC>;
    impl Dis0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dis1_SPEC;
    pub type Dis1 = crate::EnumBitfieldStruct<u8, Dis1_SPEC>;
    impl Dis1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dis2_SPEC;
    pub type Dis2 = crate::EnumBitfieldStruct<u8, Dis2_SPEC>;
    impl Dis2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dis3_SPEC;
    pub type Dis3 = crate::EnumBitfieldStruct<u8, Dis3_SPEC>;
    impl Dis3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dis4_SPEC;
    pub type Dis4 = crate::EnumBitfieldStruct<u8, Dis4_SPEC>;
    impl Dis4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dis5_SPEC;
    pub type Dis5 = crate::EnumBitfieldStruct<u8, Dis5_SPEC>;
    impl Dis5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dis6_SPEC;
    pub type Dis6 = crate::EnumBitfieldStruct<u8, Dis6_SPEC>;
    impl Dis6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dis3T2_SPEC;
    pub type Dis3T2 = crate::EnumBitfieldStruct<u8, Dis3T2_SPEC>;
    impl Dis3T2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dis0T2_SPEC;
    pub type Dis0T2 = crate::EnumBitfieldStruct<u8, Dis0T2_SPEC>;
    impl Dis0T2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdadicr_SPEC;
impl crate::sealed::RegSpec for Sdadicr_SPEC {
    type DataType = u8;
}

pub type Sdadicr = crate::RegValueT<Sdadicr_SPEC>;

impl Sdadicr {
    #[inline(always)]
    pub fn zcctl0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sdadicr::Zcctl0,
        sdadicr::Zcctl0,
        Sdadicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sdadicr::Zcctl0,
            sdadicr::Zcctl0,
            Sdadicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn zcmd0(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        sdadicr::Zcmd0,
        sdadicr::Zcmd0,
        Sdadicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sdadicr::Zcmd0,
            sdadicr::Zcmd0,
            Sdadicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn zcegn0(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        sdadicr::Zcegn0,
        sdadicr::Zcegn0,
        Sdadicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            sdadicr::Zcegn0,
            sdadicr::Zcegn0,
            Sdadicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn zcegp0(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        sdadicr::Zcegp0,
        sdadicr::Zcegp0,
        Sdadicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            sdadicr::Zcegp0,
            sdadicr::Zcegp0,
            Sdadicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn zcctl1(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sdadicr::Zcctl1,
        sdadicr::Zcctl1,
        Sdadicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sdadicr::Zcctl1,
            sdadicr::Zcctl1,
            Sdadicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn zcmd1(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        sdadicr::Zcmd1,
        sdadicr::Zcmd1,
        Sdadicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            sdadicr::Zcmd1,
            sdadicr::Zcmd1,
            Sdadicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn zcegn1(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        sdadicr::Zcegn1,
        sdadicr::Zcegn1,
        Sdadicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            sdadicr::Zcegn1,
            sdadicr::Zcegn1,
            Sdadicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn zcegp1(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        sdadicr::Zcegp1,
        sdadicr::Zcegp1,
        Sdadicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            sdadicr::Zcegp1,
            sdadicr::Zcegp1,
            Sdadicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sdadicr {
    #[inline(always)]
    fn default() -> Sdadicr {
        <crate::RegValueT<Sdadicr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdadicr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Zcctl0_SPEC;
    pub type Zcctl0 = crate::EnumBitfieldStruct<u8, Zcctl0_SPEC>;
    impl Zcctl0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Zcmd0_SPEC;
    pub type Zcmd0 = crate::EnumBitfieldStruct<u8, Zcmd0_SPEC>;
    impl Zcmd0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Zcegn0_SPEC;
    pub type Zcegn0 = crate::EnumBitfieldStruct<u8, Zcegn0_SPEC>;
    impl Zcegn0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Zcegp0_SPEC;
    pub type Zcegp0 = crate::EnumBitfieldStruct<u8, Zcegp0_SPEC>;
    impl Zcegp0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Zcctl1_SPEC;
    pub type Zcctl1 = crate::EnumBitfieldStruct<u8, Zcctl1_SPEC>;
    impl Zcctl1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Zcmd1_SPEC;
    pub type Zcmd1 = crate::EnumBitfieldStruct<u8, Zcmd1_SPEC>;
    impl Zcmd1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Zcegn1_SPEC;
    pub type Zcegn1 = crate::EnumBitfieldStruct<u8, Zcegn1_SPEC>;
    impl Zcegn1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Zcegp1_SPEC;
    pub type Zcegp1 = crate::EnumBitfieldStruct<u8, Zcegp1_SPEC>;
    impl Zcegp1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdadiclr_SPEC;
impl crate::sealed::RegSpec for Sdadiclr_SPEC {
    type DataType = u8;
}

pub type Sdadiclr = crate::RegValueT<Sdadiclr_SPEC>;

impl Sdadiclr {
    #[inline(always)]
    pub fn icl0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sdadiclr::Icl0,
        sdadiclr::Icl0,
        Sdadiclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sdadiclr::Icl0,
            sdadiclr::Icl0,
            Sdadiclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn icl1(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sdadiclr::Icl1,
        sdadiclr::Icl1,
        Sdadiclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sdadiclr::Icl1,
            sdadiclr::Icl1,
            Sdadiclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sdadiclr {
    #[inline(always)]
    fn default() -> Sdadiclr {
        <crate::RegValueT<Sdadiclr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdadiclr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icl0_SPEC;
    pub type Icl0 = crate::EnumBitfieldStruct<u8, Icl0_SPEC>;
    impl Icl0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icl1_SPEC;
    pub type Icl1 = crate::EnumBitfieldStruct<u8, Icl1_SPEC>;
    impl Icl1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdadisr_SPEC;
impl crate::sealed::RegSpec for Sdadisr_SPEC {
    type DataType = u8;
}

pub type Sdadisr = crate::RegValueT<Sdadisr_SPEC>;

impl Sdadisr {
    #[inline(always)]
    pub fn zci0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sdadisr::Zci0,
        sdadisr::Zci0,
        Sdadisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sdadisr::Zci0,
            sdadisr::Zci0,
            Sdadisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn zc0(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        sdadisr::Zc0,
        sdadisr::Zc0,
        Sdadisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sdadisr::Zc0,
            sdadisr::Zc0,
            Sdadisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn zci1(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sdadisr::Zci1,
        sdadisr::Zci1,
        Sdadisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sdadisr::Zci1,
            sdadisr::Zci1,
            Sdadisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn zc1(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        sdadisr::Zc1,
        sdadisr::Zc1,
        Sdadisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            sdadisr::Zc1,
            sdadisr::Zc1,
            Sdadisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sdadisr {
    #[inline(always)]
    fn default() -> Sdadisr {
        <crate::RegValueT<Sdadisr_SPEC> as RegisterValue<_>>::new(34)
    }
}
pub mod sdadisr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Zci0_SPEC;
    pub type Zci0 = crate::EnumBitfieldStruct<u8, Zci0_SPEC>;
    impl Zci0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Zc0_SPEC;
    pub type Zc0 = crate::EnumBitfieldStruct<u8, Zc0_SPEC>;
    impl Zc0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Zci1_SPEC;
    pub type Zci1 = crate::EnumBitfieldStruct<u8, Zci1_SPEC>;
    impl Zci1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Zc1_SPEC;
    pub type Zc1 = crate::EnumBitfieldStruct<u8, Zc1_SPEC>;
    impl Zc1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdadphcr_SPEC;
impl crate::sealed::RegSpec for Sdadphcr_SPEC {
    type DataType = u16;
}

pub type Sdadphcr = crate::RegValueT<Sdadphcr_SPEC>;

impl Sdadphcr {
    #[inline(always)]
    pub fn ph(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, Sdadphcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,Sdadphcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sdadphcr {
    #[inline(always)]
    fn default() -> Sdadphcr {
        <crate::RegValueT<Sdadphcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdadcr_SPEC;
impl crate::sealed::RegSpec for Sdadcr_SPEC {
    type DataType = u32;
}

pub type Sdadcr = crate::RegValueT<Sdadcr_SPEC>;

impl Sdadcr {
    #[inline(always)]
    pub fn sdadcr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Sdadcr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Sdadcr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Sdadcr {
    #[inline(always)]
    fn default() -> Sdadcr {
        <crate::RegValueT<Sdadcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdadcrt2_SPEC;
impl crate::sealed::RegSpec for Sdadcrt2_SPEC {
    type DataType = u32;
}

pub type Sdadcrt2 = crate::RegValueT<Sdadcrt2_SPEC>;

impl Sdadcrt2 {
    #[inline(always)]
    pub fn sdadcr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Sdadcrt2_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Sdadcrt2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Sdadcrt2 {
    #[inline(always)]
    fn default() -> Sdadcrt2 {
        <crate::RegValueT<Sdadcrt2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdadcrlpf_SPEC;
impl crate::sealed::RegSpec for Sdadcrlpf_SPEC {
    type DataType = u32;
}

pub type Sdadcrlpf = crate::RegValueT<Sdadcrlpf_SPEC>;

impl Sdadcrlpf {
    #[inline(always)]
    pub fn sdadcr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Sdadcrlpf_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Sdadcrlpf_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sdadcrlpf {
    #[inline(always)]
    fn default() -> Sdadcrlpf {
        <crate::RegValueT<Sdadcrlpf_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdadcrlpft2_SPEC;
impl crate::sealed::RegSpec for Sdadcrlpft2_SPEC {
    type DataType = u32;
}

pub type Sdadcrlpft2 = crate::RegValueT<Sdadcrlpft2_SPEC>;

impl Sdadcrlpft2 {
    #[inline(always)]
    pub fn sdadcr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Sdadcrlpft2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Sdadcrlpft2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sdadcrlpft2 {
    #[inline(always)]
    fn default() -> Sdadcrlpft2 {
        <crate::RegValueT<Sdadcrlpft2_SPEC> as RegisterValue<_>>::new(0)
    }
}
