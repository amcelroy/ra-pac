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
// Generated from SVD 1.00.01, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:22:53 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Peripheral Security Control Unit"]
unsafe impl ::core::marker::Send for super::PscuNs {}
unsafe impl ::core::marker::Sync for super::PscuNs {}
impl super::PscuNs {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn psarb(&self) -> &'static crate::common::Reg<self::Psarb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Psarb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn psarc(&self) -> &'static crate::common::Reg<self::Psarc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Psarc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn psard(&self) -> &'static crate::common::Reg<self::Psard_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Psard_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn psare(&self) -> &'static crate::common::Reg<self::Psare_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Psare_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mssar(&self) -> &'static crate::common::Reg<self::Mssar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mssar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pparb(&self) -> &'static crate::common::Reg<self::Pparb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pparb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pparc(&self) -> &'static crate::common::Reg<self::Pparc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pparc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ppard(&self) -> &'static crate::common::Reg<self::Ppard_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ppard_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ppare(&self) -> &'static crate::common::Reg<self::Ppare_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ppare_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mspar(&self) -> &'static crate::common::Reg<self::Mspar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mspar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfsamona(
        &self,
    ) -> &'static crate::common::Reg<self::Cfsamona_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cfsamona_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dfsamon(
        &self,
    ) -> &'static crate::common::Reg<self::Dfsamon_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Dfsamon_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dlmmon(&self) -> &'static crate::common::Reg<self::Dlmmon_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Dlmmon_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psarb_SPEC;
impl crate::sealed::RegSpec for Psarb_SPEC {
    type DataType = u32;
}

pub type Psarb = crate::RegValueT<Psarb_SPEC>;

impl Psarb {
    #[inline(always)]
    pub fn psarb8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        psarb::Psarb8,
        psarb::Psarb8,
        Psarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            psarb::Psarb8,
            psarb::Psarb8,
            Psarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psarb9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        psarb::Psarb9,
        psarb::Psarb9,
        Psarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            psarb::Psarb9,
            psarb::Psarb9,
            Psarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psarb11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        psarb::Psarb11,
        psarb::Psarb11,
        Psarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            psarb::Psarb11,
            psarb::Psarb11,
            Psarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psarb16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        psarb::Psarb16,
        psarb::Psarb16,
        Psarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            psarb::Psarb16,
            psarb::Psarb16,
            Psarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psarb18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        psarb::Psarb18,
        psarb::Psarb18,
        Psarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            psarb::Psarb18,
            psarb::Psarb18,
            Psarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psarb19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        psarb::Psarb19,
        psarb::Psarb19,
        Psarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            psarb::Psarb19,
            psarb::Psarb19,
            Psarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psarb22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        psarb::Psarb22,
        psarb::Psarb22,
        Psarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            psarb::Psarb22,
            psarb::Psarb22,
            Psarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psarb27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        psarb::Psarb27,
        psarb::Psarb27,
        Psarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            psarb::Psarb27,
            psarb::Psarb27,
            Psarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psarb28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        psarb::Psarb28,
        psarb::Psarb28,
        Psarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            psarb::Psarb28,
            psarb::Psarb28,
            Psarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psarb29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        psarb::Psarb29,
        psarb::Psarb29,
        Psarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            psarb::Psarb29,
            psarb::Psarb29,
            Psarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psarb30(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        psarb::Psarb30,
        psarb::Psarb30,
        Psarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            psarb::Psarb30,
            psarb::Psarb30,
            Psarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psarb31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        psarb::Psarb31,
        psarb::Psarb31,
        Psarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            psarb::Psarb31,
            psarb::Psarb31,
            Psarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Psarb {
    #[inline(always)]
    fn default() -> Psarb {
        <crate::RegValueT<Psarb_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod psarb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarb8_SPEC;
    pub type Psarb8 = crate::EnumBitfieldStruct<u8, Psarb8_SPEC>;
    impl Psarb8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarb9_SPEC;
    pub type Psarb9 = crate::EnumBitfieldStruct<u8, Psarb9_SPEC>;
    impl Psarb9 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarb11_SPEC;
    pub type Psarb11 = crate::EnumBitfieldStruct<u8, Psarb11_SPEC>;
    impl Psarb11 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarb16_SPEC;
    pub type Psarb16 = crate::EnumBitfieldStruct<u8, Psarb16_SPEC>;
    impl Psarb16 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarb18_SPEC;
    pub type Psarb18 = crate::EnumBitfieldStruct<u8, Psarb18_SPEC>;
    impl Psarb18 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarb19_SPEC;
    pub type Psarb19 = crate::EnumBitfieldStruct<u8, Psarb19_SPEC>;
    impl Psarb19 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarb22_SPEC;
    pub type Psarb22 = crate::EnumBitfieldStruct<u8, Psarb22_SPEC>;
    impl Psarb22 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarb27_SPEC;
    pub type Psarb27 = crate::EnumBitfieldStruct<u8, Psarb27_SPEC>;
    impl Psarb27 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarb28_SPEC;
    pub type Psarb28 = crate::EnumBitfieldStruct<u8, Psarb28_SPEC>;
    impl Psarb28 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarb29_SPEC;
    pub type Psarb29 = crate::EnumBitfieldStruct<u8, Psarb29_SPEC>;
    impl Psarb29 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarb30_SPEC;
    pub type Psarb30 = crate::EnumBitfieldStruct<u8, Psarb30_SPEC>;
    impl Psarb30 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarb31_SPEC;
    pub type Psarb31 = crate::EnumBitfieldStruct<u8, Psarb31_SPEC>;
    impl Psarb31 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psarc_SPEC;
impl crate::sealed::RegSpec for Psarc_SPEC {
    type DataType = u32;
}

pub type Psarc = crate::RegValueT<Psarc_SPEC>;

impl Psarc {
    #[inline(always)]
    pub fn psarc0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        psarc::Psarc0,
        psarc::Psarc0,
        Psarc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            psarc::Psarc0,
            psarc::Psarc0,
            Psarc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psarc1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        psarc::Psarc1,
        psarc::Psarc1,
        Psarc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            psarc::Psarc1,
            psarc::Psarc1,
            Psarc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psarc7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        psarc::Psarc7,
        psarc::Psarc7,
        Psarc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            psarc::Psarc7,
            psarc::Psarc7,
            Psarc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psarc8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        psarc::Psarc8,
        psarc::Psarc8,
        Psarc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            psarc::Psarc8,
            psarc::Psarc8,
            Psarc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psarc13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        psarc::Psarc13,
        psarc::Psarc13,
        Psarc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            psarc::Psarc13,
            psarc::Psarc13,
            Psarc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psarc15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        psarc::Psarc15,
        psarc::Psarc15,
        Psarc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            psarc::Psarc15,
            psarc::Psarc15,
            Psarc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psarc26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        psarc::Psarc26,
        psarc::Psarc26,
        Psarc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            psarc::Psarc26,
            psarc::Psarc26,
            Psarc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psarc27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        psarc::Psarc27,
        psarc::Psarc27,
        Psarc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            psarc::Psarc27,
            psarc::Psarc27,
            Psarc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psarc31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        psarc::Psarc31,
        psarc::Psarc31,
        Psarc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            psarc::Psarc31,
            psarc::Psarc31,
            Psarc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Psarc {
    #[inline(always)]
    fn default() -> Psarc {
        <crate::RegValueT<Psarc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod psarc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarc0_SPEC;
    pub type Psarc0 = crate::EnumBitfieldStruct<u8, Psarc0_SPEC>;
    impl Psarc0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarc1_SPEC;
    pub type Psarc1 = crate::EnumBitfieldStruct<u8, Psarc1_SPEC>;
    impl Psarc1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarc7_SPEC;
    pub type Psarc7 = crate::EnumBitfieldStruct<u8, Psarc7_SPEC>;
    impl Psarc7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarc8_SPEC;
    pub type Psarc8 = crate::EnumBitfieldStruct<u8, Psarc8_SPEC>;
    impl Psarc8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarc13_SPEC;
    pub type Psarc13 = crate::EnumBitfieldStruct<u8, Psarc13_SPEC>;
    impl Psarc13 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarc15_SPEC;
    pub type Psarc15 = crate::EnumBitfieldStruct<u8, Psarc15_SPEC>;
    impl Psarc15 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarc26_SPEC;
    pub type Psarc26 = crate::EnumBitfieldStruct<u8, Psarc26_SPEC>;
    impl Psarc26 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarc27_SPEC;
    pub type Psarc27 = crate::EnumBitfieldStruct<u8, Psarc27_SPEC>;
    impl Psarc27 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psarc31_SPEC;
    pub type Psarc31 = crate::EnumBitfieldStruct<u8, Psarc31_SPEC>;
    impl Psarc31 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psard_SPEC;
impl crate::sealed::RegSpec for Psard_SPEC {
    type DataType = u32;
}

pub type Psard = crate::RegValueT<Psard_SPEC>;

impl Psard {
    #[inline(always)]
    pub fn psard4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        psard::Psard4,
        psard::Psard4,
        Psard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            psard::Psard4,
            psard::Psard4,
            Psard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psard5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        psard::Psard5,
        psard::Psard5,
        Psard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            psard::Psard5,
            psard::Psard5,
            Psard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psard11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        psard::Psard11,
        psard::Psard11,
        Psard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            psard::Psard11,
            psard::Psard11,
            Psard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psard12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        psard::Psard12,
        psard::Psard12,
        Psard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            psard::Psard12,
            psard::Psard12,
            Psard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psard13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        psard::Psard13,
        psard::Psard13,
        Psard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            psard::Psard13,
            psard::Psard13,
            Psard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psard14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        psard::Psard14,
        psard::Psard14,
        Psard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            psard::Psard14,
            psard::Psard14,
            Psard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psard15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        psard::Psard15,
        psard::Psard15,
        Psard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            psard::Psard15,
            psard::Psard15,
            Psard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psard16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        psard::Psard16,
        psard::Psard16,
        Psard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            psard::Psard16,
            psard::Psard16,
            Psard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psard20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        psard::Psard20,
        psard::Psard20,
        Psard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            psard::Psard20,
            psard::Psard20,
            Psard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psard22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        psard::Psard22,
        psard::Psard22,
        Psard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            psard::Psard22,
            psard::Psard22,
            Psard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psard27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        psard::Psard27,
        psard::Psard27,
        Psard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            psard::Psard27,
            psard::Psard27,
            Psard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psard28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        psard::Psard28,
        psard::Psard28,
        Psard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            psard::Psard28,
            psard::Psard28,
            Psard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Psard {
    #[inline(always)]
    fn default() -> Psard {
        <crate::RegValueT<Psard_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod psard {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psard4_SPEC;
    pub type Psard4 = crate::EnumBitfieldStruct<u8, Psard4_SPEC>;
    impl Psard4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psard5_SPEC;
    pub type Psard5 = crate::EnumBitfieldStruct<u8, Psard5_SPEC>;
    impl Psard5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psard11_SPEC;
    pub type Psard11 = crate::EnumBitfieldStruct<u8, Psard11_SPEC>;
    impl Psard11 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psard12_SPEC;
    pub type Psard12 = crate::EnumBitfieldStruct<u8, Psard12_SPEC>;
    impl Psard12 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psard13_SPEC;
    pub type Psard13 = crate::EnumBitfieldStruct<u8, Psard13_SPEC>;
    impl Psard13 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psard14_SPEC;
    pub type Psard14 = crate::EnumBitfieldStruct<u8, Psard14_SPEC>;
    impl Psard14 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psard15_SPEC;
    pub type Psard15 = crate::EnumBitfieldStruct<u8, Psard15_SPEC>;
    impl Psard15 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psard16_SPEC;
    pub type Psard16 = crate::EnumBitfieldStruct<u8, Psard16_SPEC>;
    impl Psard16 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psard20_SPEC;
    pub type Psard20 = crate::EnumBitfieldStruct<u8, Psard20_SPEC>;
    impl Psard20 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psard22_SPEC;
    pub type Psard22 = crate::EnumBitfieldStruct<u8, Psard22_SPEC>;
    impl Psard22 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psard27_SPEC;
    pub type Psard27 = crate::EnumBitfieldStruct<u8, Psard27_SPEC>;
    impl Psard27 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psard28_SPEC;
    pub type Psard28 = crate::EnumBitfieldStruct<u8, Psard28_SPEC>;
    impl Psard28 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psare_SPEC;
impl crate::sealed::RegSpec for Psare_SPEC {
    type DataType = u32;
}

pub type Psare = crate::RegValueT<Psare_SPEC>;

impl Psare {
    #[inline(always)]
    pub fn psare1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        psare::Psare1,
        psare::Psare1,
        Psare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            psare::Psare1,
            psare::Psare1,
            Psare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psare2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        psare::Psare2,
        psare::Psare2,
        Psare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            psare::Psare2,
            psare::Psare2,
            Psare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psare3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        psare::Psare3,
        psare::Psare3,
        Psare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            psare::Psare3,
            psare::Psare3,
            Psare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psare8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        psare::Psare8,
        psare::Psare8,
        Psare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            psare::Psare8,
            psare::Psare8,
            Psare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psare9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        psare::Psare9,
        psare::Psare9,
        Psare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            psare::Psare9,
            psare::Psare9,
            Psare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psare18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        psare::Psare18,
        psare::Psare18,
        Psare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            psare::Psare18,
            psare::Psare18,
            Psare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psare19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        psare::Psare19,
        psare::Psare19,
        Psare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            psare::Psare19,
            psare::Psare19,
            Psare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psare20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        psare::Psare20,
        psare::Psare20,
        Psare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            psare::Psare20,
            psare::Psare20,
            Psare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psare21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        psare::Psare21,
        psare::Psare21,
        Psare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            psare::Psare21,
            psare::Psare21,
            Psare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psare26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        psare::Psare26,
        psare::Psare26,
        Psare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            psare::Psare26,
            psare::Psare26,
            Psare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psare27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        psare::Psare27,
        psare::Psare27,
        Psare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            psare::Psare27,
            psare::Psare27,
            Psare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psare28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        psare::Psare28,
        psare::Psare28,
        Psare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            psare::Psare28,
            psare::Psare28,
            Psare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psare29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        psare::Psare29,
        psare::Psare29,
        Psare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            psare::Psare29,
            psare::Psare29,
            Psare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psare30(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        psare::Psare30,
        psare::Psare30,
        Psare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            psare::Psare30,
            psare::Psare30,
            Psare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psare31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        psare::Psare31,
        psare::Psare31,
        Psare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            psare::Psare31,
            psare::Psare31,
            Psare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Psare {
    #[inline(always)]
    fn default() -> Psare {
        <crate::RegValueT<Psare_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod psare {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psare1_SPEC;
    pub type Psare1 = crate::EnumBitfieldStruct<u8, Psare1_SPEC>;
    impl Psare1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psare2_SPEC;
    pub type Psare2 = crate::EnumBitfieldStruct<u8, Psare2_SPEC>;
    impl Psare2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psare3_SPEC;
    pub type Psare3 = crate::EnumBitfieldStruct<u8, Psare3_SPEC>;
    impl Psare3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psare8_SPEC;
    pub type Psare8 = crate::EnumBitfieldStruct<u8, Psare8_SPEC>;
    impl Psare8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psare9_SPEC;
    pub type Psare9 = crate::EnumBitfieldStruct<u8, Psare9_SPEC>;
    impl Psare9 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psare18_SPEC;
    pub type Psare18 = crate::EnumBitfieldStruct<u8, Psare18_SPEC>;
    impl Psare18 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psare19_SPEC;
    pub type Psare19 = crate::EnumBitfieldStruct<u8, Psare19_SPEC>;
    impl Psare19 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psare20_SPEC;
    pub type Psare20 = crate::EnumBitfieldStruct<u8, Psare20_SPEC>;
    impl Psare20 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psare21_SPEC;
    pub type Psare21 = crate::EnumBitfieldStruct<u8, Psare21_SPEC>;
    impl Psare21 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psare26_SPEC;
    pub type Psare26 = crate::EnumBitfieldStruct<u8, Psare26_SPEC>;
    impl Psare26 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psare27_SPEC;
    pub type Psare27 = crate::EnumBitfieldStruct<u8, Psare27_SPEC>;
    impl Psare27 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psare28_SPEC;
    pub type Psare28 = crate::EnumBitfieldStruct<u8, Psare28_SPEC>;
    impl Psare28 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psare29_SPEC;
    pub type Psare29 = crate::EnumBitfieldStruct<u8, Psare29_SPEC>;
    impl Psare29 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psare30_SPEC;
    pub type Psare30 = crate::EnumBitfieldStruct<u8, Psare30_SPEC>;
    impl Psare30 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psare31_SPEC;
    pub type Psare31 = crate::EnumBitfieldStruct<u8, Psare31_SPEC>;
    impl Psare31 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mssar_SPEC;
impl crate::sealed::RegSpec for Mssar_SPEC {
    type DataType = u32;
}

pub type Mssar = crate::RegValueT<Mssar_SPEC>;

impl Mssar {
    #[inline(always)]
    pub fn mssar0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mssar::Mssar0,
        mssar::Mssar0,
        Mssar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mssar::Mssar0,
            mssar::Mssar0,
            Mssar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mssar1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mssar::Mssar1,
        mssar::Mssar1,
        Mssar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mssar::Mssar1,
            mssar::Mssar1,
            Mssar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mssar15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        mssar::Mssar15,
        mssar::Mssar15,
        Mssar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            mssar::Mssar15,
            mssar::Mssar15,
            Mssar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mssar22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        mssar::Mssar22,
        mssar::Mssar22,
        Mssar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            mssar::Mssar22,
            mssar::Mssar22,
            Mssar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mssar31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        mssar::Mssar31,
        mssar::Mssar31,
        Mssar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            mssar::Mssar31,
            mssar::Mssar31,
            Mssar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mssar {
    #[inline(always)]
    fn default() -> Mssar {
        <crate::RegValueT<Mssar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mssar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mssar0_SPEC;
    pub type Mssar0 = crate::EnumBitfieldStruct<u8, Mssar0_SPEC>;
    impl Mssar0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mssar1_SPEC;
    pub type Mssar1 = crate::EnumBitfieldStruct<u8, Mssar1_SPEC>;
    impl Mssar1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mssar15_SPEC;
    pub type Mssar15 = crate::EnumBitfieldStruct<u8, Mssar15_SPEC>;
    impl Mssar15 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mssar22_SPEC;
    pub type Mssar22 = crate::EnumBitfieldStruct<u8, Mssar22_SPEC>;
    impl Mssar22 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mssar31_SPEC;
    pub type Mssar31 = crate::EnumBitfieldStruct<u8, Mssar31_SPEC>;
    impl Mssar31 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pparb_SPEC;
impl crate::sealed::RegSpec for Pparb_SPEC {
    type DataType = u32;
}

pub type Pparb = crate::RegValueT<Pparb_SPEC>;

impl Pparb {
    #[inline(always)]
    pub fn pparb8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        pparb::Pparb8,
        pparb::Pparb8,
        Pparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            pparb::Pparb8,
            pparb::Pparb8,
            Pparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pparb9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        pparb::Pparb9,
        pparb::Pparb9,
        Pparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            pparb::Pparb9,
            pparb::Pparb9,
            Pparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pparb11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        pparb::Pparb11,
        pparb::Pparb11,
        Pparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            pparb::Pparb11,
            pparb::Pparb11,
            Pparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pparb16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        pparb::Pparb16,
        pparb::Pparb16,
        Pparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            pparb::Pparb16,
            pparb::Pparb16,
            Pparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pparb18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        pparb::Pparb18,
        pparb::Pparb18,
        Pparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            pparb::Pparb18,
            pparb::Pparb18,
            Pparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pparb19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        pparb::Pparb19,
        pparb::Pparb19,
        Pparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            pparb::Pparb19,
            pparb::Pparb19,
            Pparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pparb22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        pparb::Pparb22,
        pparb::Pparb22,
        Pparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            pparb::Pparb22,
            pparb::Pparb22,
            Pparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pparb27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        pparb::Pparb27,
        pparb::Pparb27,
        Pparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            pparb::Pparb27,
            pparb::Pparb27,
            Pparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pparb28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        pparb::Pparb28,
        pparb::Pparb28,
        Pparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            pparb::Pparb28,
            pparb::Pparb28,
            Pparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pparb29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        pparb::Pparb29,
        pparb::Pparb29,
        Pparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            pparb::Pparb29,
            pparb::Pparb29,
            Pparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pparb30(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        pparb::Pparb30,
        pparb::Pparb30,
        Pparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            pparb::Pparb30,
            pparb::Pparb30,
            Pparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pparb31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        pparb::Pparb31,
        pparb::Pparb31,
        Pparb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            pparb::Pparb31,
            pparb::Pparb31,
            Pparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pparb {
    #[inline(always)]
    fn default() -> Pparb {
        <crate::RegValueT<Pparb_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod pparb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pparb8_SPEC;
    pub type Pparb8 = crate::EnumBitfieldStruct<u8, Pparb8_SPEC>;
    impl Pparb8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pparb9_SPEC;
    pub type Pparb9 = crate::EnumBitfieldStruct<u8, Pparb9_SPEC>;
    impl Pparb9 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pparb11_SPEC;
    pub type Pparb11 = crate::EnumBitfieldStruct<u8, Pparb11_SPEC>;
    impl Pparb11 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pparb16_SPEC;
    pub type Pparb16 = crate::EnumBitfieldStruct<u8, Pparb16_SPEC>;
    impl Pparb16 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pparb18_SPEC;
    pub type Pparb18 = crate::EnumBitfieldStruct<u8, Pparb18_SPEC>;
    impl Pparb18 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pparb19_SPEC;
    pub type Pparb19 = crate::EnumBitfieldStruct<u8, Pparb19_SPEC>;
    impl Pparb19 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pparb22_SPEC;
    pub type Pparb22 = crate::EnumBitfieldStruct<u8, Pparb22_SPEC>;
    impl Pparb22 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pparb27_SPEC;
    pub type Pparb27 = crate::EnumBitfieldStruct<u8, Pparb27_SPEC>;
    impl Pparb27 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pparb28_SPEC;
    pub type Pparb28 = crate::EnumBitfieldStruct<u8, Pparb28_SPEC>;
    impl Pparb28 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pparb29_SPEC;
    pub type Pparb29 = crate::EnumBitfieldStruct<u8, Pparb29_SPEC>;
    impl Pparb29 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pparb30_SPEC;
    pub type Pparb30 = crate::EnumBitfieldStruct<u8, Pparb30_SPEC>;
    impl Pparb30 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pparb31_SPEC;
    pub type Pparb31 = crate::EnumBitfieldStruct<u8, Pparb31_SPEC>;
    impl Pparb31 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pparc_SPEC;
impl crate::sealed::RegSpec for Pparc_SPEC {
    type DataType = u32;
}

pub type Pparc = crate::RegValueT<Pparc_SPEC>;

impl Pparc {
    #[inline(always)]
    pub fn pparc0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pparc::Pparc0,
        pparc::Pparc0,
        Pparc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pparc::Pparc0,
            pparc::Pparc0,
            Pparc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pparc1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pparc::Pparc1,
        pparc::Pparc1,
        Pparc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pparc::Pparc1,
            pparc::Pparc1,
            Pparc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pparc7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        pparc::Pparc7,
        pparc::Pparc7,
        Pparc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pparc::Pparc7,
            pparc::Pparc7,
            Pparc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pparc8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        pparc::Pparc8,
        pparc::Pparc8,
        Pparc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            pparc::Pparc8,
            pparc::Pparc8,
            Pparc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pparc13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        pparc::Pparc13,
        pparc::Pparc13,
        Pparc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            pparc::Pparc13,
            pparc::Pparc13,
            Pparc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pparc15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        pparc::Pparc15,
        pparc::Pparc15,
        Pparc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pparc::Pparc15,
            pparc::Pparc15,
            Pparc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pparc26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        pparc::Pparc26,
        pparc::Pparc26,
        Pparc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            pparc::Pparc26,
            pparc::Pparc26,
            Pparc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pparc27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        pparc::Pparc27,
        pparc::Pparc27,
        Pparc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            pparc::Pparc27,
            pparc::Pparc27,
            Pparc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pparc31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        pparc::Pparc31,
        pparc::Pparc31,
        Pparc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            pparc::Pparc31,
            pparc::Pparc31,
            Pparc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pparc {
    #[inline(always)]
    fn default() -> Pparc {
        <crate::RegValueT<Pparc_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod pparc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pparc0_SPEC;
    pub type Pparc0 = crate::EnumBitfieldStruct<u8, Pparc0_SPEC>;
    impl Pparc0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pparc1_SPEC;
    pub type Pparc1 = crate::EnumBitfieldStruct<u8, Pparc1_SPEC>;
    impl Pparc1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pparc7_SPEC;
    pub type Pparc7 = crate::EnumBitfieldStruct<u8, Pparc7_SPEC>;
    impl Pparc7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pparc8_SPEC;
    pub type Pparc8 = crate::EnumBitfieldStruct<u8, Pparc8_SPEC>;
    impl Pparc8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pparc13_SPEC;
    pub type Pparc13 = crate::EnumBitfieldStruct<u8, Pparc13_SPEC>;
    impl Pparc13 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pparc15_SPEC;
    pub type Pparc15 = crate::EnumBitfieldStruct<u8, Pparc15_SPEC>;
    impl Pparc15 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pparc26_SPEC;
    pub type Pparc26 = crate::EnumBitfieldStruct<u8, Pparc26_SPEC>;
    impl Pparc26 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pparc27_SPEC;
    pub type Pparc27 = crate::EnumBitfieldStruct<u8, Pparc27_SPEC>;
    impl Pparc27 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pparc31_SPEC;
    pub type Pparc31 = crate::EnumBitfieldStruct<u8, Pparc31_SPEC>;
    impl Pparc31 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ppard_SPEC;
impl crate::sealed::RegSpec for Ppard_SPEC {
    type DataType = u32;
}

pub type Ppard = crate::RegValueT<Ppard_SPEC>;

impl Ppard {
    #[inline(always)]
    pub fn ppard4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ppard::Ppard4,
        ppard::Ppard4,
        Ppard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ppard::Ppard4,
            ppard::Ppard4,
            Ppard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ppard5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ppard::Ppard5,
        ppard::Ppard5,
        Ppard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ppard::Ppard5,
            ppard::Ppard5,
            Ppard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ppard11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        ppard::Ppard11,
        ppard::Ppard11,
        Ppard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            ppard::Ppard11,
            ppard::Ppard11,
            Ppard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ppard12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        ppard::Ppard12,
        ppard::Ppard12,
        Ppard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            ppard::Ppard12,
            ppard::Ppard12,
            Ppard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ppard13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        ppard::Ppard13,
        ppard::Ppard13,
        Ppard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            ppard::Ppard13,
            ppard::Ppard13,
            Ppard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ppard14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        ppard::Ppard14,
        ppard::Ppard14,
        Ppard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            ppard::Ppard14,
            ppard::Ppard14,
            Ppard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ppard15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        ppard::Ppard15,
        ppard::Ppard15,
        Ppard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            ppard::Ppard15,
            ppard::Ppard15,
            Ppard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ppard16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ppard::Ppard16,
        ppard::Ppard16,
        Ppard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ppard::Ppard16,
            ppard::Ppard16,
            Ppard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ppard20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        ppard::Ppard20,
        ppard::Ppard20,
        Ppard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            ppard::Ppard20,
            ppard::Ppard20,
            Ppard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ppard22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        ppard::Ppard22,
        ppard::Ppard22,
        Ppard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            ppard::Ppard22,
            ppard::Ppard22,
            Ppard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ppard27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        ppard::Ppard27,
        ppard::Ppard27,
        Ppard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            ppard::Ppard27,
            ppard::Ppard27,
            Ppard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ppard28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        ppard::Ppard28,
        ppard::Ppard28,
        Ppard_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            ppard::Ppard28,
            ppard::Ppard28,
            Ppard_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ppard {
    #[inline(always)]
    fn default() -> Ppard {
        <crate::RegValueT<Ppard_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod ppard {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ppard4_SPEC;
    pub type Ppard4 = crate::EnumBitfieldStruct<u8, Ppard4_SPEC>;
    impl Ppard4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ppard5_SPEC;
    pub type Ppard5 = crate::EnumBitfieldStruct<u8, Ppard5_SPEC>;
    impl Ppard5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ppard11_SPEC;
    pub type Ppard11 = crate::EnumBitfieldStruct<u8, Ppard11_SPEC>;
    impl Ppard11 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ppard12_SPEC;
    pub type Ppard12 = crate::EnumBitfieldStruct<u8, Ppard12_SPEC>;
    impl Ppard12 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ppard13_SPEC;
    pub type Ppard13 = crate::EnumBitfieldStruct<u8, Ppard13_SPEC>;
    impl Ppard13 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ppard14_SPEC;
    pub type Ppard14 = crate::EnumBitfieldStruct<u8, Ppard14_SPEC>;
    impl Ppard14 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ppard15_SPEC;
    pub type Ppard15 = crate::EnumBitfieldStruct<u8, Ppard15_SPEC>;
    impl Ppard15 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ppard16_SPEC;
    pub type Ppard16 = crate::EnumBitfieldStruct<u8, Ppard16_SPEC>;
    impl Ppard16 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ppard20_SPEC;
    pub type Ppard20 = crate::EnumBitfieldStruct<u8, Ppard20_SPEC>;
    impl Ppard20 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ppard22_SPEC;
    pub type Ppard22 = crate::EnumBitfieldStruct<u8, Ppard22_SPEC>;
    impl Ppard22 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ppard27_SPEC;
    pub type Ppard27 = crate::EnumBitfieldStruct<u8, Ppard27_SPEC>;
    impl Ppard27 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ppard28_SPEC;
    pub type Ppard28 = crate::EnumBitfieldStruct<u8, Ppard28_SPEC>;
    impl Ppard28 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ppare_SPEC;
impl crate::sealed::RegSpec for Ppare_SPEC {
    type DataType = u32;
}

pub type Ppare = crate::RegValueT<Ppare_SPEC>;

impl Ppare {
    #[inline(always)]
    pub fn ppare1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ppare::Ppare1,
        ppare::Ppare1,
        Ppare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ppare::Ppare1,
            ppare::Ppare1,
            Ppare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ppare2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ppare::Ppare2,
        ppare::Ppare2,
        Ppare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ppare::Ppare2,
            ppare::Ppare2,
            Ppare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ppare3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ppare::Ppare3,
        ppare::Ppare3,
        Ppare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ppare::Ppare3,
            ppare::Ppare3,
            Ppare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ppare8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        ppare::Ppare8,
        ppare::Ppare8,
        Ppare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            ppare::Ppare8,
            ppare::Ppare8,
            Ppare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ppare9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ppare::Ppare9,
        ppare::Ppare9,
        Ppare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ppare::Ppare9,
            ppare::Ppare9,
            Ppare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ppare18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        ppare::Ppare18,
        ppare::Ppare18,
        Ppare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            ppare::Ppare18,
            ppare::Ppare18,
            Ppare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ppare19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        ppare::Ppare19,
        ppare::Ppare19,
        Ppare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            ppare::Ppare19,
            ppare::Ppare19,
            Ppare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ppare20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        ppare::Ppare20,
        ppare::Ppare20,
        Ppare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            ppare::Ppare20,
            ppare::Ppare20,
            Ppare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ppare21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        ppare::Ppare21,
        ppare::Ppare21,
        Ppare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            ppare::Ppare21,
            ppare::Ppare21,
            Ppare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ppare26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        ppare::Ppare26,
        ppare::Ppare26,
        Ppare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            ppare::Ppare26,
            ppare::Ppare26,
            Ppare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ppare27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        ppare::Ppare27,
        ppare::Ppare27,
        Ppare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            ppare::Ppare27,
            ppare::Ppare27,
            Ppare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ppare28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        ppare::Ppare28,
        ppare::Ppare28,
        Ppare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            ppare::Ppare28,
            ppare::Ppare28,
            Ppare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ppare29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        ppare::Ppare29,
        ppare::Ppare29,
        Ppare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            ppare::Ppare29,
            ppare::Ppare29,
            Ppare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ppare30(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        ppare::Ppare30,
        ppare::Ppare30,
        Ppare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            ppare::Ppare30,
            ppare::Ppare30,
            Ppare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ppare31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        ppare::Ppare31,
        ppare::Ppare31,
        Ppare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            ppare::Ppare31,
            ppare::Ppare31,
            Ppare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ppare {
    #[inline(always)]
    fn default() -> Ppare {
        <crate::RegValueT<Ppare_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod ppare {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ppare1_SPEC;
    pub type Ppare1 = crate::EnumBitfieldStruct<u8, Ppare1_SPEC>;
    impl Ppare1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ppare2_SPEC;
    pub type Ppare2 = crate::EnumBitfieldStruct<u8, Ppare2_SPEC>;
    impl Ppare2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ppare3_SPEC;
    pub type Ppare3 = crate::EnumBitfieldStruct<u8, Ppare3_SPEC>;
    impl Ppare3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ppare8_SPEC;
    pub type Ppare8 = crate::EnumBitfieldStruct<u8, Ppare8_SPEC>;
    impl Ppare8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ppare9_SPEC;
    pub type Ppare9 = crate::EnumBitfieldStruct<u8, Ppare9_SPEC>;
    impl Ppare9 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ppare18_SPEC;
    pub type Ppare18 = crate::EnumBitfieldStruct<u8, Ppare18_SPEC>;
    impl Ppare18 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ppare19_SPEC;
    pub type Ppare19 = crate::EnumBitfieldStruct<u8, Ppare19_SPEC>;
    impl Ppare19 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ppare20_SPEC;
    pub type Ppare20 = crate::EnumBitfieldStruct<u8, Ppare20_SPEC>;
    impl Ppare20 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ppare21_SPEC;
    pub type Ppare21 = crate::EnumBitfieldStruct<u8, Ppare21_SPEC>;
    impl Ppare21 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ppare26_SPEC;
    pub type Ppare26 = crate::EnumBitfieldStruct<u8, Ppare26_SPEC>;
    impl Ppare26 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ppare27_SPEC;
    pub type Ppare27 = crate::EnumBitfieldStruct<u8, Ppare27_SPEC>;
    impl Ppare27 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ppare28_SPEC;
    pub type Ppare28 = crate::EnumBitfieldStruct<u8, Ppare28_SPEC>;
    impl Ppare28 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ppare29_SPEC;
    pub type Ppare29 = crate::EnumBitfieldStruct<u8, Ppare29_SPEC>;
    impl Ppare29 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ppare30_SPEC;
    pub type Ppare30 = crate::EnumBitfieldStruct<u8, Ppare30_SPEC>;
    impl Ppare30 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ppare31_SPEC;
    pub type Ppare31 = crate::EnumBitfieldStruct<u8, Ppare31_SPEC>;
    impl Ppare31 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mspar_SPEC;
impl crate::sealed::RegSpec for Mspar_SPEC {
    type DataType = u32;
}

pub type Mspar = crate::RegValueT<Mspar_SPEC>;

impl Mspar {
    #[inline(always)]
    pub fn mspar31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        mspar::Mspar31,
        mspar::Mspar31,
        Mspar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            mspar::Mspar31,
            mspar::Mspar31,
            Mspar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mspar {
    #[inline(always)]
    fn default() -> Mspar {
        <crate::RegValueT<Mspar_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod mspar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mspar31_SPEC;
    pub type Mspar31 = crate::EnumBitfieldStruct<u8, Mspar31_SPEC>;
    impl Mspar31 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfsamona_SPEC;
impl crate::sealed::RegSpec for Cfsamona_SPEC {
    type DataType = u32;
}

pub type Cfsamona = crate::RegValueT<Cfsamona_SPEC>;

impl Cfsamona {
    #[inline(always)]
    pub fn cfs2(
        self,
    ) -> crate::common::RegisterField<15, 0x1ff, 1, 0, u16, u16, Cfsamona_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<15,0x1ff,1,0,u16,u16,Cfsamona_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfsamona {
    #[inline(always)]
    fn default() -> Cfsamona {
        <crate::RegValueT<Cfsamona_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfsamon_SPEC;
impl crate::sealed::RegSpec for Dfsamon_SPEC {
    type DataType = u32;
}

pub type Dfsamon = crate::RegValueT<Dfsamon_SPEC>;

impl Dfsamon {
    #[inline(always)]
    pub fn dfs(
        self,
    ) -> crate::common::RegisterField<10, 0x3f, 1, 0, u8, u8, Dfsamon_SPEC, crate::common::R> {
        crate::common::RegisterField::<10,0x3f,1,0,u8,u8,Dfsamon_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Dfsamon {
    #[inline(always)]
    fn default() -> Dfsamon {
        <crate::RegValueT<Dfsamon_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dlmmon_SPEC;
impl crate::sealed::RegSpec for Dlmmon_SPEC {
    type DataType = u32;
}

pub type Dlmmon = crate::RegValueT<Dlmmon_SPEC>;

impl Dlmmon {
    #[inline(always)]
    pub fn dlmmon(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        dlmmon::Dlmmon,
        dlmmon::Dlmmon,
        Dlmmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            dlmmon::Dlmmon,
            dlmmon::Dlmmon,
            Dlmmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dlmmon {
    #[inline(always)]
    fn default() -> Dlmmon {
        <crate::RegValueT<Dlmmon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dlmmon {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlmmon_SPEC;
    pub type Dlmmon = crate::EnumBitfieldStruct<u8, Dlmmon_SPEC>;
    impl Dlmmon {
        pub const _0_X_0: Self = Self::new(0);

        pub const _0_X_1: Self = Self::new(1);

        pub const _0_X_2: Self = Self::new(2);

        pub const _0_X_3: Self = Self::new(3);

        pub const _0_X_4: Self = Self::new(4);

        pub const _0_X_5: Self = Self::new(5);

        pub const _0_X_6: Self = Self::new(6);

        pub const _0_X_7: Self = Self::new(7);

        pub const _0_X_8: Self = Self::new(8);

        pub const _0_X_9: Self = Self::new(9);

        pub const OTHERS: Self = Self::new(0);
    }
}
