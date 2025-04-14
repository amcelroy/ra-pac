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
// Generated from SVD 1.30.00, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:17:59 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Consumer Electronics Control"]
unsafe impl ::core::marker::Send for super::Cec {}
unsafe impl ::core::marker::Sync for super::Cec {}
impl super::Cec {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn cadr(&self) -> &'static crate::common::Reg<self::Cadr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cadr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cecctl1(
        &self,
    ) -> &'static crate::common::Reg<self::Cecctl1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cecctl1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[inline(always)]
    pub const fn statb(&self) -> &'static crate::common::Reg<self::Statb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Statb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn statl(&self) -> &'static crate::common::Reg<self::Statl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Statl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lgc0l(&self) -> &'static crate::common::Reg<self::Lgc0L_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lgc0L_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lgc1l(&self) -> &'static crate::common::Reg<self::Lgc1L_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lgc1L_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[inline(always)]
    pub const fn datb(&self) -> &'static crate::common::Reg<self::Datb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Datb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn nomt(&self) -> &'static crate::common::Reg<self::Nomt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Nomt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[inline(always)]
    pub const fn statll(
        &self,
    ) -> &'static crate::common::Reg<self::Statll_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Statll_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn statlh(
        &self,
    ) -> &'static crate::common::Reg<self::Statlh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Statlh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[inline(always)]
    pub const fn statbl(
        &self,
    ) -> &'static crate::common::Reg<self::Statbl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Statbl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn statbh(
        &self,
    ) -> &'static crate::common::Reg<self::Statbh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Statbh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lgc0ll(
        &self,
    ) -> &'static crate::common::Reg<self::Lgc0Ll_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lgc0Ll_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lgc0lh(
        &self,
    ) -> &'static crate::common::Reg<self::Lgc0Lh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lgc0Lh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(26usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lgc1ll(
        &self,
    ) -> &'static crate::common::Reg<self::Lgc1Ll_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lgc1Ll_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lgc1lh(
        &self,
    ) -> &'static crate::common::Reg<self::Lgc1Lh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lgc1Lh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(30usize),
            )
        }
    }

    #[inline(always)]
    pub const fn datbl(&self) -> &'static crate::common::Reg<self::Datbl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Datbl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn datbh(&self) -> &'static crate::common::Reg<self::Datbh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Datbh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(34usize),
            )
        }
    }

    #[inline(always)]
    pub const fn nomp(&self) -> &'static crate::common::Reg<self::Nomp_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Nomp_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cecexmd(
        &self,
    ) -> &'static crate::common::Reg<self::Cecexmd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cecexmd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cecexmon(
        &self,
    ) -> &'static crate::common::Reg<self::Cecexmon_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cecexmon_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(42usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctxd(&self) -> &'static crate::common::Reg<self::Ctxd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctxd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn crxd(&self) -> &'static crate::common::Reg<self::Crxd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Crxd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ceces(&self) -> &'static crate::common::Reg<self::Ceces_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ceces_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cecs(&self) -> &'static crate::common::Reg<self::Cecs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cecs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(67usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cecfc(&self) -> &'static crate::common::Reg<self::Cecfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cecfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cecctl0(
        &self,
    ) -> &'static crate::common::Reg<self::Cecctl0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cecctl0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(69usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cadr_SPEC;
impl crate::sealed::RegSpec for Cadr_SPEC {
    type DataType = u16;
}

pub type Cadr = crate::RegValueT<Cadr_SPEC>;

impl Cadr {
    #[inline(always)]
    pub fn adr00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cadr::Adr00,
        cadr::Adr00,
        Cadr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cadr::Adr00,
            cadr::Adr00,
            Cadr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adr01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cadr::Adr01,
        cadr::Adr01,
        Cadr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cadr::Adr01,
            cadr::Adr01,
            Cadr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adr02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cadr::Adr02,
        cadr::Adr02,
        Cadr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cadr::Adr02,
            cadr::Adr02,
            Cadr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adr03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        cadr::Adr03,
        cadr::Adr03,
        Cadr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            cadr::Adr03,
            cadr::Adr03,
            Cadr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adr04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        cadr::Adr04,
        cadr::Adr04,
        Cadr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            cadr::Adr04,
            cadr::Adr04,
            Cadr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adr05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        cadr::Adr05,
        cadr::Adr05,
        Cadr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            cadr::Adr05,
            cadr::Adr05,
            Cadr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adr06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        cadr::Adr06,
        cadr::Adr06,
        Cadr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            cadr::Adr06,
            cadr::Adr06,
            Cadr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adr07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        cadr::Adr07,
        cadr::Adr07,
        Cadr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            cadr::Adr07,
            cadr::Adr07,
            Cadr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adr08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cadr::Adr08,
        cadr::Adr08,
        Cadr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cadr::Adr08,
            cadr::Adr08,
            Cadr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adr09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        cadr::Adr09,
        cadr::Adr09,
        Cadr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            cadr::Adr09,
            cadr::Adr09,
            Cadr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        cadr::Adr10,
        cadr::Adr10,
        Cadr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            cadr::Adr10,
            cadr::Adr10,
            Cadr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adr11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        cadr::Adr11,
        cadr::Adr11,
        Cadr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            cadr::Adr11,
            cadr::Adr11,
            Cadr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        cadr::Adr12,
        cadr::Adr12,
        Cadr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            cadr::Adr12,
            cadr::Adr12,
            Cadr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        cadr::Adr13,
        cadr::Adr13,
        Cadr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            cadr::Adr13,
            cadr::Adr13,
            Cadr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        cadr::Adr14,
        cadr::Adr14,
        Cadr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            cadr::Adr14,
            cadr::Adr14,
            Cadr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cadr {
    #[inline(always)]
    fn default() -> Cadr {
        <crate::RegValueT<Cadr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cadr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adr00_SPEC;
    pub type Adr00 = crate::EnumBitfieldStruct<u8, Adr00_SPEC>;
    impl Adr00 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adr01_SPEC;
    pub type Adr01 = crate::EnumBitfieldStruct<u8, Adr01_SPEC>;
    impl Adr01 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adr02_SPEC;
    pub type Adr02 = crate::EnumBitfieldStruct<u8, Adr02_SPEC>;
    impl Adr02 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adr03_SPEC;
    pub type Adr03 = crate::EnumBitfieldStruct<u8, Adr03_SPEC>;
    impl Adr03 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adr04_SPEC;
    pub type Adr04 = crate::EnumBitfieldStruct<u8, Adr04_SPEC>;
    impl Adr04 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adr05_SPEC;
    pub type Adr05 = crate::EnumBitfieldStruct<u8, Adr05_SPEC>;
    impl Adr05 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adr06_SPEC;
    pub type Adr06 = crate::EnumBitfieldStruct<u8, Adr06_SPEC>;
    impl Adr06 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adr07_SPEC;
    pub type Adr07 = crate::EnumBitfieldStruct<u8, Adr07_SPEC>;
    impl Adr07 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adr08_SPEC;
    pub type Adr08 = crate::EnumBitfieldStruct<u8, Adr08_SPEC>;
    impl Adr08 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adr09_SPEC;
    pub type Adr09 = crate::EnumBitfieldStruct<u8, Adr09_SPEC>;
    impl Adr09 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adr10_SPEC;
    pub type Adr10 = crate::EnumBitfieldStruct<u8, Adr10_SPEC>;
    impl Adr10 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adr11_SPEC;
    pub type Adr11 = crate::EnumBitfieldStruct<u8, Adr11_SPEC>;
    impl Adr11 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adr12_SPEC;
    pub type Adr12 = crate::EnumBitfieldStruct<u8, Adr12_SPEC>;
    impl Adr12 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adr13_SPEC;
    pub type Adr13 = crate::EnumBitfieldStruct<u8, Adr13_SPEC>;
    impl Adr13 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adr14_SPEC;
    pub type Adr14 = crate::EnumBitfieldStruct<u8, Adr14_SPEC>;
    impl Adr14 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cecctl1_SPEC;
impl crate::sealed::RegSpec for Cecctl1_SPEC {
    type DataType = u8;
}

pub type Cecctl1 = crate::RegValueT<Cecctl1_SPEC>;

impl Cecctl1 {
    #[inline(always)]
    pub fn sft(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        cecctl1::Sft,
        cecctl1::Sft,
        Cecctl1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            cecctl1::Sft,
            cecctl1::Sft,
            Cecctl1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cesel(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        cecctl1::Cesel,
        cecctl1::Cesel,
        Cecctl1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            cecctl1::Cesel,
            cecctl1::Cesel,
            Cecctl1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sterrd(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        cecctl1::Sterrd,
        cecctl1::Sterrd,
        Cecctl1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            cecctl1::Sterrd,
            cecctl1::Sterrd,
            Cecctl1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn blerrd(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        cecctl1::Blerrd,
        cecctl1::Blerrd,
        Cecctl1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            cecctl1::Blerrd,
            cecctl1::Blerrd,
            Cecctl1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cintmk(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        cecctl1::Cintmk,
        cecctl1::Cintmk,
        Cecctl1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            cecctl1::Cintmk,
            cecctl1::Cintmk,
            Cecctl1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cdfc(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        cecctl1::Cdfc,
        cecctl1::Cdfc,
        Cecctl1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            cecctl1::Cdfc,
            cecctl1::Cdfc,
            Cecctl1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cecctl1 {
    #[inline(always)]
    fn default() -> Cecctl1 {
        <crate::RegValueT<Cecctl1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cecctl1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sft_SPEC;
    pub type Sft = crate::EnumBitfieldStruct<u8, Sft_SPEC>;
    impl Sft {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cesel_SPEC;
    pub type Cesel = crate::EnumBitfieldStruct<u8, Cesel_SPEC>;
    impl Cesel {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sterrd_SPEC;
    pub type Sterrd = crate::EnumBitfieldStruct<u8, Sterrd_SPEC>;
    impl Sterrd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Blerrd_SPEC;
    pub type Blerrd = crate::EnumBitfieldStruct<u8, Blerrd_SPEC>;
    impl Blerrd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cintmk_SPEC;
    pub type Cintmk = crate::EnumBitfieldStruct<u8, Cintmk_SPEC>;
    impl Cintmk {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cdfc_SPEC;
    pub type Cdfc = crate::EnumBitfieldStruct<u8, Cdfc_SPEC>;
    impl Cdfc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Statb_SPEC;
impl crate::sealed::RegSpec for Statb_SPEC {
    type DataType = u16;
}

pub type Statb = crate::RegValueT<Statb_SPEC>;

impl Statb {
    #[inline(always)]
    pub fn statb(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Statb_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Statb_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Statb {
    #[inline(always)]
    fn default() -> Statb {
        <crate::RegValueT<Statb_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Statl_SPEC;
impl crate::sealed::RegSpec for Statl_SPEC {
    type DataType = u16;
}

pub type Statl = crate::RegValueT<Statl_SPEC>;

impl Statl {
    #[inline(always)]
    pub fn statl(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Statl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Statl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Statl {
    #[inline(always)]
    fn default() -> Statl {
        <crate::RegValueT<Statl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lgc0L_SPEC;
impl crate::sealed::RegSpec for Lgc0L_SPEC {
    type DataType = u16;
}

pub type Lgc0L = crate::RegValueT<Lgc0L_SPEC>;

impl Lgc0L {
    #[inline(always)]
    pub fn lgc0l(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Lgc0L_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Lgc0L_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Lgc0L {
    #[inline(always)]
    fn default() -> Lgc0L {
        <crate::RegValueT<Lgc0L_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lgc1L_SPEC;
impl crate::sealed::RegSpec for Lgc1L_SPEC {
    type DataType = u16;
}

pub type Lgc1L = crate::RegValueT<Lgc1L_SPEC>;

impl Lgc1L {
    #[inline(always)]
    pub fn lgc1l(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Lgc1L_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Lgc1L_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Lgc1L {
    #[inline(always)]
    fn default() -> Lgc1L {
        <crate::RegValueT<Lgc1L_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Datb_SPEC;
impl crate::sealed::RegSpec for Datb_SPEC {
    type DataType = u16;
}

pub type Datb = crate::RegValueT<Datb_SPEC>;

impl Datb {
    #[inline(always)]
    pub fn datb(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Datb_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Datb_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Datb {
    #[inline(always)]
    fn default() -> Datb {
        <crate::RegValueT<Datb_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nomt_SPEC;
impl crate::sealed::RegSpec for Nomt_SPEC {
    type DataType = u16;
}

pub type Nomt = crate::RegValueT<Nomt_SPEC>;

impl Nomt {
    #[inline(always)]
    pub fn nomt(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Nomt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Nomt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Nomt {
    #[inline(always)]
    fn default() -> Nomt {
        <crate::RegValueT<Nomt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Statll_SPEC;
impl crate::sealed::RegSpec for Statll_SPEC {
    type DataType = u16;
}

pub type Statll = crate::RegValueT<Statll_SPEC>;

impl Statll {
    #[inline(always)]
    pub fn statll(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Statll_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Statll_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Statll {
    #[inline(always)]
    fn default() -> Statll {
        <crate::RegValueT<Statll_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Statlh_SPEC;
impl crate::sealed::RegSpec for Statlh_SPEC {
    type DataType = u16;
}

pub type Statlh = crate::RegValueT<Statlh_SPEC>;

impl Statlh {
    #[inline(always)]
    pub fn statlh(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Statlh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Statlh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Statlh {
    #[inline(always)]
    fn default() -> Statlh {
        <crate::RegValueT<Statlh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Statbl_SPEC;
impl crate::sealed::RegSpec for Statbl_SPEC {
    type DataType = u16;
}

pub type Statbl = crate::RegValueT<Statbl_SPEC>;

impl Statbl {
    #[inline(always)]
    pub fn statbl(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Statbl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Statbl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Statbl {
    #[inline(always)]
    fn default() -> Statbl {
        <crate::RegValueT<Statbl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Statbh_SPEC;
impl crate::sealed::RegSpec for Statbh_SPEC {
    type DataType = u16;
}

pub type Statbh = crate::RegValueT<Statbh_SPEC>;

impl Statbh {
    #[inline(always)]
    pub fn statbh(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Statbh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Statbh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Statbh {
    #[inline(always)]
    fn default() -> Statbh {
        <crate::RegValueT<Statbh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lgc0Ll_SPEC;
impl crate::sealed::RegSpec for Lgc0Ll_SPEC {
    type DataType = u16;
}

pub type Lgc0Ll = crate::RegValueT<Lgc0Ll_SPEC>;

impl Lgc0Ll {
    #[inline(always)]
    pub fn lgc0ll(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Lgc0Ll_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Lgc0Ll_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Lgc0Ll {
    #[inline(always)]
    fn default() -> Lgc0Ll {
        <crate::RegValueT<Lgc0Ll_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lgc0Lh_SPEC;
impl crate::sealed::RegSpec for Lgc0Lh_SPEC {
    type DataType = u16;
}

pub type Lgc0Lh = crate::RegValueT<Lgc0Lh_SPEC>;

impl Lgc0Lh {
    #[inline(always)]
    pub fn lgc0lh(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Lgc0Lh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Lgc0Lh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Lgc0Lh {
    #[inline(always)]
    fn default() -> Lgc0Lh {
        <crate::RegValueT<Lgc0Lh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lgc1Ll_SPEC;
impl crate::sealed::RegSpec for Lgc1Ll_SPEC {
    type DataType = u16;
}

pub type Lgc1Ll = crate::RegValueT<Lgc1Ll_SPEC>;

impl Lgc1Ll {
    #[inline(always)]
    pub fn lgc1ll(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Lgc1Ll_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Lgc1Ll_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Lgc1Ll {
    #[inline(always)]
    fn default() -> Lgc1Ll {
        <crate::RegValueT<Lgc1Ll_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lgc1Lh_SPEC;
impl crate::sealed::RegSpec for Lgc1Lh_SPEC {
    type DataType = u16;
}

pub type Lgc1Lh = crate::RegValueT<Lgc1Lh_SPEC>;

impl Lgc1Lh {
    #[inline(always)]
    pub fn lgc1lh(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Lgc1Lh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Lgc1Lh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Lgc1Lh {
    #[inline(always)]
    fn default() -> Lgc1Lh {
        <crate::RegValueT<Lgc1Lh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Datbl_SPEC;
impl crate::sealed::RegSpec for Datbl_SPEC {
    type DataType = u16;
}

pub type Datbl = crate::RegValueT<Datbl_SPEC>;

impl Datbl {
    #[inline(always)]
    pub fn datbl(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Datbl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Datbl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Datbl {
    #[inline(always)]
    fn default() -> Datbl {
        <crate::RegValueT<Datbl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Datbh_SPEC;
impl crate::sealed::RegSpec for Datbh_SPEC {
    type DataType = u16;
}

pub type Datbh = crate::RegValueT<Datbh_SPEC>;

impl Datbh {
    #[inline(always)]
    pub fn datbh(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Datbh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Datbh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Datbh {
    #[inline(always)]
    fn default() -> Datbh {
        <crate::RegValueT<Datbh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nomp_SPEC;
impl crate::sealed::RegSpec for Nomp_SPEC {
    type DataType = u16;
}

pub type Nomp = crate::RegValueT<Nomp_SPEC>;

impl Nomp {
    #[inline(always)]
    pub fn nomp(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Nomp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Nomp_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Nomp {
    #[inline(always)]
    fn default() -> Nomp {
        <crate::RegValueT<Nomp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cecexmd_SPEC;
impl crate::sealed::RegSpec for Cecexmd_SPEC {
    type DataType = u8;
}

pub type Cecexmd = crate::RegValueT<Cecexmd_SPEC>;

impl Cecexmd {
    #[inline(always)]
    pub fn lerplen(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        cecexmd::Lerplen,
        cecexmd::Lerplen,
        Cecexmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            cecexmd::Lerplen,
            cecexmd::Lerplen,
            Cecexmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rercven(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        cecexmd::Rercven,
        cecexmd::Rercven,
        Cecexmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            cecexmd::Rercven,
            cecexmd::Rercven,
            Cecexmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rcvintdsel(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        cecexmd::Rcvintdsel,
        cecexmd::Rcvintdsel,
        Cecexmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            cecexmd::Rcvintdsel,
            cecexmd::Rcvintdsel,
            Cecexmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cecexmd {
    #[inline(always)]
    fn default() -> Cecexmd {
        <crate::RegValueT<Cecexmd_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cecexmd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lerplen_SPEC;
    pub type Lerplen = crate::EnumBitfieldStruct<u8, Lerplen_SPEC>;
    impl Lerplen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rercven_SPEC;
    pub type Rercven = crate::EnumBitfieldStruct<u8, Rercven_SPEC>;
    impl Rercven {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcvintdsel_SPEC;
    pub type Rcvintdsel = crate::EnumBitfieldStruct<u8, Rcvintdsel_SPEC>;
    impl Rcvintdsel {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cecexmon_SPEC;
impl crate::sealed::RegSpec for Cecexmon_SPEC {
    type DataType = u8;
}

pub type Cecexmon = crate::RegValueT<Cecexmon_SPEC>;

impl Cecexmon {
    #[inline(always)]
    pub fn ceclnmon(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cecexmon::Ceclnmon,
        cecexmon::Ceclnmon,
        Cecexmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cecexmon::Ceclnmon,
            cecexmon::Ceclnmon,
            Cecexmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ackf(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Cecexmon_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cecexmon_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Cecexmon {
    #[inline(always)]
    fn default() -> Cecexmon {
        <crate::RegValueT<Cecexmon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cecexmon {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ceclnmon_SPEC;
    pub type Ceclnmon = crate::EnumBitfieldStruct<u8, Ceclnmon_SPEC>;
    impl Ceclnmon {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctxd_SPEC;
impl crate::sealed::RegSpec for Ctxd_SPEC {
    type DataType = u8;
}

pub type Ctxd = crate::RegValueT<Ctxd_SPEC>;

impl NoBitfieldReg<Ctxd_SPEC> for Ctxd {}
impl ::core::default::Default for Ctxd {
    #[inline(always)]
    fn default() -> Ctxd {
        <crate::RegValueT<Ctxd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crxd_SPEC;
impl crate::sealed::RegSpec for Crxd_SPEC {
    type DataType = u8;
}

pub type Crxd = crate::RegValueT<Crxd_SPEC>;

impl NoBitfieldReg<Crxd_SPEC> for Crxd {}
impl ::core::default::Default for Crxd {
    #[inline(always)]
    fn default() -> Crxd {
        <crate::RegValueT<Crxd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ceces_SPEC;
impl crate::sealed::RegSpec for Ceces_SPEC {
    type DataType = u8;
}

pub type Ceces = crate::RegValueT<Ceces_SPEC>;

impl Ceces {
    #[inline(always)]
    pub fn oerr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ceces::Oerr,
        ceces::Oerr,
        Ceces_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ceces::Oerr,
            ceces::Oerr,
            Ceces_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn uerr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ceces::Uerr,
        ceces::Uerr,
        Ceces_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ceces::Uerr,
            ceces::Uerr,
            Ceces_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ackerr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ceces::Ackerr,
        ceces::Ackerr,
        Ceces_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ceces::Ackerr,
            ceces::Ackerr,
            Ceces_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn terr(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ceces::Terr,
        ceces::Terr,
        Ceces_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ceces::Terr,
            ceces::Terr,
            Ceces_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txerr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ceces::Txerr,
        ceces::Txerr,
        Ceces_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ceces::Txerr,
            ceces::Txerr,
            Ceces_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn aerr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ceces::Aerr,
        ceces::Aerr,
        Ceces_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ceces::Aerr,
            ceces::Aerr,
            Ceces_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn blerr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ceces::Blerr,
        ceces::Blerr,
        Ceces_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ceces::Blerr,
            ceces::Blerr,
            Ceces_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ceces {
    #[inline(always)]
    fn default() -> Ceces {
        <crate::RegValueT<Ceces_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ceces {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oerr_SPEC;
    pub type Oerr = crate::EnumBitfieldStruct<u8, Oerr_SPEC>;
    impl Oerr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uerr_SPEC;
    pub type Uerr = crate::EnumBitfieldStruct<u8, Uerr_SPEC>;
    impl Uerr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ackerr_SPEC;
    pub type Ackerr = crate::EnumBitfieldStruct<u8, Ackerr_SPEC>;
    impl Ackerr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Terr_SPEC;
    pub type Terr = crate::EnumBitfieldStruct<u8, Terr_SPEC>;
    impl Terr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txerr_SPEC;
    pub type Txerr = crate::EnumBitfieldStruct<u8, Txerr_SPEC>;
    impl Txerr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aerr_SPEC;
    pub type Aerr = crate::EnumBitfieldStruct<u8, Aerr_SPEC>;
    impl Aerr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Blerr_SPEC;
    pub type Blerr = crate::EnumBitfieldStruct<u8, Blerr_SPEC>;
    impl Blerr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cecs_SPEC;
impl crate::sealed::RegSpec for Cecs_SPEC {
    type DataType = u8;
}

pub type Cecs = crate::RegValueT<Cecs_SPEC>;

impl Cecs {
    #[inline(always)]
    pub fn adrf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cecs::Adrf,
        cecs::Adrf,
        Cecs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cecs::Adrf,
            cecs::Adrf,
            Cecs_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn busst(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cecs::Busst,
        cecs::Busst,
        Cecs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cecs::Busst,
            cecs::Busst,
            Cecs_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txst(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cecs::Txst,
        cecs::Txst,
        Cecs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cecs::Txst,
            cecs::Txst,
            Cecs_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn eomf(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        cecs::Eomf,
        cecs::Eomf,
        Cecs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            cecs::Eomf,
            cecs::Eomf,
            Cecs_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn itcef(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        cecs::Itcef,
        cecs::Itcef,
        Cecs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            cecs::Itcef,
            cecs::Itcef,
            Cecs_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sftst(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        cecs::Sftst,
        cecs::Sftst,
        Cecs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            cecs::Sftst,
            cecs::Sftst,
            Cecs_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cecs {
    #[inline(always)]
    fn default() -> Cecs {
        <crate::RegValueT<Cecs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cecs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adrf_SPEC;
    pub type Adrf = crate::EnumBitfieldStruct<u8, Adrf_SPEC>;
    impl Adrf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Busst_SPEC;
    pub type Busst = crate::EnumBitfieldStruct<u8, Busst_SPEC>;
    impl Busst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txst_SPEC;
    pub type Txst = crate::EnumBitfieldStruct<u8, Txst_SPEC>;
    impl Txst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eomf_SPEC;
    pub type Eomf = crate::EnumBitfieldStruct<u8, Eomf_SPEC>;
    impl Eomf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Itcef_SPEC;
    pub type Itcef = crate::EnumBitfieldStruct<u8, Itcef_SPEC>;
    impl Itcef {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sftst_SPEC;
    pub type Sftst = crate::EnumBitfieldStruct<u8, Sftst_SPEC>;
    impl Sftst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cecfc_SPEC;
impl crate::sealed::RegSpec for Cecfc_SPEC {
    type DataType = u8;
}

pub type Cecfc = crate::RegValueT<Cecfc_SPEC>;

impl Cecfc {
    #[inline(always)]
    pub fn octrg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cecfc::Octrg,
        cecfc::Octrg,
        Cecfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cecfc::Octrg,
            cecfc::Octrg,
            Cecfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn uctrg(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cecfc::Uctrg,
        cecfc::Uctrg,
        Cecfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cecfc::Uctrg,
            cecfc::Uctrg,
            Cecfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ackctrg(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cecfc::Ackctrg,
        cecfc::Ackctrg,
        Cecfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cecfc::Ackctrg,
            cecfc::Ackctrg,
            Cecfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tctrg(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        cecfc::Tctrg,
        cecfc::Tctrg,
        Cecfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            cecfc::Tctrg,
            cecfc::Tctrg,
            Cecfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txctrg(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        cecfc::Txctrg,
        cecfc::Txctrg,
        Cecfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            cecfc::Txctrg,
            cecfc::Txctrg,
            Cecfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn actrg(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        cecfc::Actrg,
        cecfc::Actrg,
        Cecfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            cecfc::Actrg,
            cecfc::Actrg,
            Cecfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn blctrg(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        cecfc::Blctrg,
        cecfc::Blctrg,
        Cecfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            cecfc::Blctrg,
            cecfc::Blctrg,
            Cecfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cecfc {
    #[inline(always)]
    fn default() -> Cecfc {
        <crate::RegValueT<Cecfc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cecfc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Octrg_SPEC;
    pub type Octrg = crate::EnumBitfieldStruct<u8, Octrg_SPEC>;
    impl Octrg {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uctrg_SPEC;
    pub type Uctrg = crate::EnumBitfieldStruct<u8, Uctrg_SPEC>;
    impl Uctrg {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ackctrg_SPEC;
    pub type Ackctrg = crate::EnumBitfieldStruct<u8, Ackctrg_SPEC>;
    impl Ackctrg {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tctrg_SPEC;
    pub type Tctrg = crate::EnumBitfieldStruct<u8, Tctrg_SPEC>;
    impl Tctrg {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txctrg_SPEC;
    pub type Txctrg = crate::EnumBitfieldStruct<u8, Txctrg_SPEC>;
    impl Txctrg {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Actrg_SPEC;
    pub type Actrg = crate::EnumBitfieldStruct<u8, Actrg_SPEC>;
    impl Actrg {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Blctrg_SPEC;
    pub type Blctrg = crate::EnumBitfieldStruct<u8, Blctrg_SPEC>;
    impl Blctrg {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cecctl0_SPEC;
impl crate::sealed::RegSpec for Cecctl0_SPEC {
    type DataType = u8;
}

pub type Cecctl0 = crate::RegValueT<Cecctl0_SPEC>;

impl Cecctl0 {
    #[inline(always)]
    pub fn eom(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cecctl0::Eom,
        cecctl0::Eom,
        Cecctl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cecctl0::Eom,
            cecctl0::Eom,
            Cecctl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cecrxen(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cecctl0::Cecrxen,
        cecctl0::Cecrxen,
        Cecctl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cecctl0::Cecrxen,
            cecctl0::Cecrxen,
            Cecctl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txtrg(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cecctl0::Txtrg,
        cecctl0::Txtrg,
        Cecctl0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cecctl0::Txtrg,
            cecctl0::Txtrg,
            Cecctl0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ccl(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x7,
        1,
        0,
        cecctl0::Ccl,
        cecctl0::Ccl,
        Cecctl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x7,
            1,
            0,
            cecctl0::Ccl,
            cecctl0::Ccl,
            Cecctl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ackten(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        cecctl0::Ackten,
        cecctl0::Ackten,
        Cecctl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            cecctl0::Ackten,
            cecctl0::Ackten,
            Cecctl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cece(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        cecctl0::Cece,
        cecctl0::Cece,
        Cecctl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            cecctl0::Cece,
            cecctl0::Cece,
            Cecctl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cecctl0 {
    #[inline(always)]
    fn default() -> Cecctl0 {
        <crate::RegValueT<Cecctl0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cecctl0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eom_SPEC;
    pub type Eom = crate::EnumBitfieldStruct<u8, Eom_SPEC>;
    impl Eom {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cecrxen_SPEC;
    pub type Cecrxen = crate::EnumBitfieldStruct<u8, Cecrxen_SPEC>;
    impl Cecrxen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txtrg_SPEC;
    pub type Txtrg = crate::EnumBitfieldStruct<u8, Txtrg_SPEC>;
    impl Txtrg {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccl_SPEC;
    pub type Ccl = crate::EnumBitfieldStruct<u8, Ccl_SPEC>;
    impl Ccl {
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
    pub struct Ackten_SPEC;
    pub type Ackten = crate::EnumBitfieldStruct<u8, Ackten_SPEC>;
    impl Ackten {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cece_SPEC;
    pub type Cece = crate::EnumBitfieldStruct<u8, Cece_SPEC>;
    impl Cece {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
