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
// Generated from SVD 0.90.02, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:18:12 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"UARTA"]
unsafe impl ::core::marker::Send for super::Uarta {}
unsafe impl ::core::marker::Sync for super::Uarta {}
impl super::Uarta {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn txba(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Txba_SPEC, crate::common::RW>,
        2,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x0usize))
        }
    }

    #[inline(always)]
    pub const fn rxba(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Rxba_SPEC, crate::common::R>,
        2,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1usize))
        }
    }

    #[inline(always)]
    pub const fn asima0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Asima0_SPEC, crate::common::RW>,
        2,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2usize))
        }
    }

    #[inline(always)]
    pub const fn asima1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Asima1_SPEC, crate::common::RW>,
        2,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x3usize))
        }
    }

    #[inline(always)]
    pub const fn brgca(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Brgca_SPEC, crate::common::RW>,
        2,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x4usize))
        }
    }

    #[inline(always)]
    pub const fn asisa(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Asisa_SPEC, crate::common::R>,
        2,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x5usize))
        }
    }

    #[inline(always)]
    pub const fn ascta(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Ascta_SPEC, crate::common::RW>,
        2,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6usize))
        }
    }

    #[inline(always)]
    pub const fn uta0ck(
        &self,
    ) -> &'static crate::common::Reg<self::Uta0Ck_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uta0Ck_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uta1ck(
        &self,
    ) -> &'static crate::common::Reg<self::Uta1Ck_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uta1Ck_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(17usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txba_SPEC;
impl crate::sealed::RegSpec for Txba_SPEC {
    type DataType = u8;
}

pub type Txba = crate::RegValueT<Txba_SPEC>;

impl NoBitfieldReg<Txba_SPEC> for Txba {}
impl ::core::default::Default for Txba {
    #[inline(always)]
    fn default() -> Txba {
        <crate::RegValueT<Txba_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxba_SPEC;
impl crate::sealed::RegSpec for Rxba_SPEC {
    type DataType = u8;
}

pub type Rxba = crate::RegValueT<Rxba_SPEC>;

impl NoBitfieldReg<Rxba_SPEC> for Rxba {}
impl ::core::default::Default for Rxba {
    #[inline(always)]
    fn default() -> Rxba {
        <crate::RegValueT<Rxba_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asima0_SPEC;
impl crate::sealed::RegSpec for Asima0_SPEC {
    type DataType = u8;
}

pub type Asima0 = crate::RegValueT<Asima0_SPEC>;

impl Asima0 {
    #[inline(always)]
    pub fn isrma(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        asima0::Isrma,
        asima0::Isrma,
        Asima0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            asima0::Isrma,
            asima0::Isrma,
            Asima0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn issma(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        asima0::Issma,
        asima0::Issma,
        Asima0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            asima0::Issma,
            asima0::Issma,
            Asima0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rxea(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        asima0::Rxea,
        asima0::Rxea,
        Asima0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            asima0::Rxea,
            asima0::Rxea,
            Asima0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txea(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        asima0::Txea,
        asima0::Txea,
        Asima0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            asima0::Txea,
            asima0::Txea,
            Asima0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        asima0::En,
        asima0::En,
        Asima0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            asima0::En,
            asima0::En,
            Asima0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Asima0 {
    #[inline(always)]
    fn default() -> Asima0 {
        <crate::RegValueT<Asima0_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod asima0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isrma_SPEC;
    pub type Isrma = crate::EnumBitfieldStruct<u8, Isrma_SPEC>;
    impl Isrma {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Issma_SPEC;
    pub type Issma = crate::EnumBitfieldStruct<u8, Issma_SPEC>;
    impl Issma {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxea_SPEC;
    pub type Rxea = crate::EnumBitfieldStruct<u8, Rxea_SPEC>;
    impl Rxea {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txea_SPEC;
    pub type Txea = crate::EnumBitfieldStruct<u8, Txea_SPEC>;
    impl Txea {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asima1_SPEC;
impl crate::sealed::RegSpec for Asima1_SPEC {
    type DataType = u8;
}

pub type Asima1 = crate::RegValueT<Asima1_SPEC>;

impl Asima1 {
    #[inline(always)]
    pub fn alv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        asima1::Alv,
        asima1::Alv,
        Asima1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            asima1::Alv,
            asima1::Alv,
            Asima1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dir(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        asima1::Dir,
        asima1::Dir,
        Asima1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            asima1::Dir,
            asima1::Dir,
            Asima1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sl(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        asima1::Sl,
        asima1::Sl,
        Asima1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            asima1::Sl,
            asima1::Sl,
            Asima1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cl(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x3,
        1,
        0,
        asima1::Cl,
        asima1::Cl,
        Asima1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x3,
            1,
            0,
            asima1::Cl,
            asima1::Cl,
            Asima1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ps(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x3,
        1,
        0,
        asima1::Ps,
        asima1::Ps,
        Asima1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x3,
            1,
            0,
            asima1::Ps,
            asima1::Ps,
            Asima1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Asima1 {
    #[inline(always)]
    fn default() -> Asima1 {
        <crate::RegValueT<Asima1_SPEC> as RegisterValue<_>>::new(26)
    }
}
pub mod asima1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Alv_SPEC;
    pub type Alv = crate::EnumBitfieldStruct<u8, Alv_SPEC>;
    impl Alv {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dir_SPEC;
    pub type Dir = crate::EnumBitfieldStruct<u8, Dir_SPEC>;
    impl Dir {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sl_SPEC;
    pub type Sl = crate::EnumBitfieldStruct<u8, Sl_SPEC>;
    impl Sl {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cl_SPEC;
    pub type Cl = crate::EnumBitfieldStruct<u8, Cl_SPEC>;
    impl Cl {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ps_SPEC;
    pub type Ps = crate::EnumBitfieldStruct<u8, Ps_SPEC>;
    impl Ps {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Brgca_SPEC;
impl crate::sealed::RegSpec for Brgca_SPEC {
    type DataType = u8;
}

pub type Brgca = crate::RegValueT<Brgca_SPEC>;

impl NoBitfieldReg<Brgca_SPEC> for Brgca {}
impl ::core::default::Default for Brgca {
    #[inline(always)]
    fn default() -> Brgca {
        <crate::RegValueT<Brgca_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asisa_SPEC;
impl crate::sealed::RegSpec for Asisa_SPEC {
    type DataType = u8;
}

pub type Asisa = crate::RegValueT<Asisa_SPEC>;

impl Asisa {
    #[inline(always)]
    pub fn ovea(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        asisa::Ovea,
        asisa::Ovea,
        Asisa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            asisa::Ovea,
            asisa::Ovea,
            Asisa_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fea(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        asisa::Fea,
        asisa::Fea,
        Asisa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            asisa::Fea,
            asisa::Fea,
            Asisa_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pea(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        asisa::Pea,
        asisa::Pea,
        Asisa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            asisa::Pea,
            asisa::Pea,
            Asisa_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txsfa(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        asisa::Txsfa,
        asisa::Txsfa,
        Asisa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            asisa::Txsfa,
            asisa::Txsfa,
            Asisa_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txbfa(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        asisa::Txbfa,
        asisa::Txbfa,
        Asisa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            asisa::Txbfa,
            asisa::Txbfa,
            Asisa_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Asisa {
    #[inline(always)]
    fn default() -> Asisa {
        <crate::RegValueT<Asisa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod asisa {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovea_SPEC;
    pub type Ovea = crate::EnumBitfieldStruct<u8, Ovea_SPEC>;
    impl Ovea {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fea_SPEC;
    pub type Fea = crate::EnumBitfieldStruct<u8, Fea_SPEC>;
    impl Fea {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pea_SPEC;
    pub type Pea = crate::EnumBitfieldStruct<u8, Pea_SPEC>;
    impl Pea {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txsfa_SPEC;
    pub type Txsfa = crate::EnumBitfieldStruct<u8, Txsfa_SPEC>;
    impl Txsfa {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txbfa_SPEC;
    pub type Txbfa = crate::EnumBitfieldStruct<u8, Txbfa_SPEC>;
    impl Txbfa {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ascta_SPEC;
impl crate::sealed::RegSpec for Ascta_SPEC {
    type DataType = u8;
}

pub type Ascta = crate::RegValueT<Ascta_SPEC>;

impl Ascta {
    #[inline(always)]
    pub fn ovecta(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ascta::Ovecta,
        ascta::Ovecta,
        Ascta_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ascta::Ovecta,
            ascta::Ovecta,
            Ascta_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fecta(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ascta::Fecta,
        ascta::Fecta,
        Ascta_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ascta::Fecta,
            ascta::Fecta,
            Ascta_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pecta(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ascta::Pecta,
        ascta::Pecta,
        Ascta_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ascta::Pecta,
            ascta::Pecta,
            Ascta_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ascta {
    #[inline(always)]
    fn default() -> Ascta {
        <crate::RegValueT<Ascta_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ascta {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovecta_SPEC;
    pub type Ovecta = crate::EnumBitfieldStruct<u8, Ovecta_SPEC>;
    impl Ovecta {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fecta_SPEC;
    pub type Fecta = crate::EnumBitfieldStruct<u8, Fecta_SPEC>;
    impl Fecta {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pecta_SPEC;
    pub type Pecta = crate::EnumBitfieldStruct<u8, Pecta_SPEC>;
    impl Pecta {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uta0Ck_SPEC;
impl crate::sealed::RegSpec for Uta0Ck_SPEC {
    type DataType = u8;
}

pub type Uta0Ck = crate::RegValueT<Uta0Ck_SPEC>;

impl Uta0Ck {
    #[inline(always)]
    pub fn ck(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        uta0ck::Ck,
        uta0ck::Ck,
        Uta0Ck_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            uta0ck::Ck,
            uta0ck::Ck,
            Uta0Ck_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sel(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        uta0ck::Sel,
        uta0ck::Sel,
        Uta0Ck_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            uta0ck::Sel,
            uta0ck::Sel,
            Uta0Ck_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        uta0ck::En,
        uta0ck::En,
        Uta0Ck_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            uta0ck::En,
            uta0ck::En,
            Uta0Ck_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Uta0Ck {
    #[inline(always)]
    fn default() -> Uta0Ck {
        <crate::RegValueT<Uta0Ck_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod uta0ck {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ck_SPEC;
    pub type Ck = crate::EnumBitfieldStruct<u8, Ck_SPEC>;
    impl Ck {
        pub const _0_X_0: Self = Self::new(0);

        pub const _0_X_1: Self = Self::new(1);

        pub const _0_X_2: Self = Self::new(2);

        pub const _0_X_3: Self = Self::new(3);

        pub const _0_X_4: Self = Self::new(4);

        pub const _0_X_5: Self = Self::new(5);

        pub const _0_X_6: Self = Self::new(6);

        pub const _0_X_8: Self = Self::new(8);

        pub const _0_X_9: Self = Self::new(9);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sel_SPEC;
    pub type Sel = crate::EnumBitfieldStruct<u8, Sel_SPEC>;
    impl Sel {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uta1Ck_SPEC;
impl crate::sealed::RegSpec for Uta1Ck_SPEC {
    type DataType = u8;
}

pub type Uta1Ck = crate::RegValueT<Uta1Ck_SPEC>;

impl Uta1Ck {
    #[inline(always)]
    pub fn ck(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        uta1ck::Ck,
        uta1ck::Ck,
        Uta1Ck_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            uta1ck::Ck,
            uta1ck::Ck,
            Uta1Ck_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        uta1ck::En,
        uta1ck::En,
        Uta1Ck_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            uta1ck::En,
            uta1ck::En,
            Uta1Ck_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Uta1Ck {
    #[inline(always)]
    fn default() -> Uta1Ck {
        <crate::RegValueT<Uta1Ck_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod uta1ck {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ck_SPEC;
    pub type Ck = crate::EnumBitfieldStruct<u8, Ck_SPEC>;
    impl Ck {
        pub const _0_X_0: Self = Self::new(0);

        pub const _0_X_1: Self = Self::new(1);

        pub const _0_X_2: Self = Self::new(2);

        pub const _0_X_3: Self = Self::new(3);

        pub const _0_X_4: Self = Self::new(4);

        pub const _0_X_5: Self = Self::new(5);

        pub const _0_X_6: Self = Self::new(6);

        pub const _0_X_8: Self = Self::new(8);

        pub const _0_X_9: Self = Self::new(9);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
