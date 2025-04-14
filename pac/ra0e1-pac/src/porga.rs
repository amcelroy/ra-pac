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
#[doc = r"Product Organize"]
unsafe impl ::core::marker::Send for super::Porga {}
unsafe impl ::core::marker::Sync for super::Porga {}
impl super::Porga {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn snfen(&self) -> &'static crate::common::Reg<self::Snfen_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Snfen_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tnfen(&self) -> &'static crate::common::Reg<self::Tnfen_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tnfen_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1usize),
            )
        }
    }

    #[inline(always)]
    pub const fn isc(&self) -> &'static crate::common::Reg<self::Isc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Isc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tis0(&self) -> &'static crate::common::Reg<self::Tis0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tis0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tis1(&self) -> &'static crate::common::Reg<self::Tis1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tis1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ulbs(&self) -> &'static crate::common::Reg<self::Ulbs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ulbs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(9usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Snfen_SPEC;
impl crate::sealed::RegSpec for Snfen_SPEC {
    type DataType = u8;
}

pub type Snfen = crate::RegValueT<Snfen_SPEC>;

impl Snfen {
    #[inline(always)]
    pub fn snfen00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        snfen::Snfen00,
        snfen::Snfen00,
        Snfen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            snfen::Snfen00,
            snfen::Snfen00,
            Snfen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn snfen10(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        snfen::Snfen10,
        snfen::Snfen10,
        Snfen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            snfen::Snfen10,
            snfen::Snfen10,
            Snfen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn snfen20(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        snfen::Snfen20,
        snfen::Snfen20,
        Snfen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            snfen::Snfen20,
            snfen::Snfen20,
            Snfen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Snfen {
    #[inline(always)]
    fn default() -> Snfen {
        <crate::RegValueT<Snfen_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod snfen {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snfen00_SPEC;
    pub type Snfen00 = crate::EnumBitfieldStruct<u8, Snfen00_SPEC>;
    impl Snfen00 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snfen10_SPEC;
    pub type Snfen10 = crate::EnumBitfieldStruct<u8, Snfen10_SPEC>;
    impl Snfen10 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snfen20_SPEC;
    pub type Snfen20 = crate::EnumBitfieldStruct<u8, Snfen20_SPEC>;
    impl Snfen20 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tnfen_SPEC;
impl crate::sealed::RegSpec for Tnfen_SPEC {
    type DataType = u8;
}

pub type Tnfen = crate::RegValueT<Tnfen_SPEC>;

impl Tnfen {
    #[inline(always)]
    pub fn tnfen00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        tnfen::Tnfen00,
        tnfen::Tnfen00,
        Tnfen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            tnfen::Tnfen00,
            tnfen::Tnfen00,
            Tnfen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tnfen01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        tnfen::Tnfen01,
        tnfen::Tnfen01,
        Tnfen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            tnfen::Tnfen01,
            tnfen::Tnfen01,
            Tnfen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tnfen02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        tnfen::Tnfen02,
        tnfen::Tnfen02,
        Tnfen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            tnfen::Tnfen02,
            tnfen::Tnfen02,
            Tnfen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tnfen03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        tnfen::Tnfen03,
        tnfen::Tnfen03,
        Tnfen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            tnfen::Tnfen03,
            tnfen::Tnfen03,
            Tnfen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tnfen04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        tnfen::Tnfen04,
        tnfen::Tnfen04,
        Tnfen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            tnfen::Tnfen04,
            tnfen::Tnfen04,
            Tnfen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tnfen05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        tnfen::Tnfen05,
        tnfen::Tnfen05,
        Tnfen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            tnfen::Tnfen05,
            tnfen::Tnfen05,
            Tnfen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tnfen06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        tnfen::Tnfen06,
        tnfen::Tnfen06,
        Tnfen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            tnfen::Tnfen06,
            tnfen::Tnfen06,
            Tnfen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tnfen07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        tnfen::Tnfen07,
        tnfen::Tnfen07,
        Tnfen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            tnfen::Tnfen07,
            tnfen::Tnfen07,
            Tnfen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tnfen {
    #[inline(always)]
    fn default() -> Tnfen {
        <crate::RegValueT<Tnfen_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tnfen {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tnfen00_SPEC;
    pub type Tnfen00 = crate::EnumBitfieldStruct<u8, Tnfen00_SPEC>;
    impl Tnfen00 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tnfen01_SPEC;
    pub type Tnfen01 = crate::EnumBitfieldStruct<u8, Tnfen01_SPEC>;
    impl Tnfen01 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tnfen02_SPEC;
    pub type Tnfen02 = crate::EnumBitfieldStruct<u8, Tnfen02_SPEC>;
    impl Tnfen02 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tnfen03_SPEC;
    pub type Tnfen03 = crate::EnumBitfieldStruct<u8, Tnfen03_SPEC>;
    impl Tnfen03 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tnfen04_SPEC;
    pub type Tnfen04 = crate::EnumBitfieldStruct<u8, Tnfen04_SPEC>;
    impl Tnfen04 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tnfen05_SPEC;
    pub type Tnfen05 = crate::EnumBitfieldStruct<u8, Tnfen05_SPEC>;
    impl Tnfen05 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tnfen06_SPEC;
    pub type Tnfen06 = crate::EnumBitfieldStruct<u8, Tnfen06_SPEC>;
    impl Tnfen06 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tnfen07_SPEC;
    pub type Tnfen07 = crate::EnumBitfieldStruct<u8, Tnfen07_SPEC>;
    impl Tnfen07 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isc_SPEC;
impl crate::sealed::RegSpec for Isc_SPEC {
    type DataType = u8;
}

pub type Isc = crate::RegValueT<Isc_SPEC>;

impl Isc {
    #[inline(always)]
    pub fn isc0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, isc::Isc0, isc::Isc0, Isc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            isc::Isc0,
            isc::Isc0,
            Isc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn isc1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, isc::Isc1, isc::Isc1, Isc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            isc::Isc1,
            isc::Isc1,
            Isc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ssie00(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        isc::Ssie00,
        isc::Ssie00,
        Isc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            isc::Ssie00,
            isc::Ssie00,
            Isc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Isc {
    #[inline(always)]
    fn default() -> Isc {
        <crate::RegValueT<Isc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod isc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isc0_SPEC;
    pub type Isc0 = crate::EnumBitfieldStruct<u8, Isc0_SPEC>;
    impl Isc0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isc1_SPEC;
    pub type Isc1 = crate::EnumBitfieldStruct<u8, Isc1_SPEC>;
    impl Isc1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssie00_SPEC;
    pub type Ssie00 = crate::EnumBitfieldStruct<u8, Ssie00_SPEC>;
    impl Ssie00 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tis0_SPEC;
impl crate::sealed::RegSpec for Tis0_SPEC {
    type DataType = u8;
}

pub type Tis0 = crate::RegValueT<Tis0_SPEC>;

impl Tis0 {
    #[inline(always)]
    pub fn tis(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        tis0::Tis,
        tis0::Tis,
        Tis0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            tis0::Tis,
            tis0::Tis,
            Tis0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tis0 {
    #[inline(always)]
    fn default() -> Tis0 {
        <crate::RegValueT<Tis0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tis0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tis_SPEC;
    pub type Tis = crate::EnumBitfieldStruct<u8, Tis_SPEC>;
    impl Tis {
        pub const _000: Self = Self::new(0);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _101: Self = Self::new(5);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tis1_SPEC;
impl crate::sealed::RegSpec for Tis1_SPEC {
    type DataType = u8;
}

pub type Tis1 = crate::RegValueT<Tis1_SPEC>;

impl Tis1 {
    #[inline(always)]
    pub fn tis0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        tis1::Tis0,
        tis1::Tis0,
        Tis1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            tis1::Tis0,
            tis1::Tis0,
            Tis1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tis1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        tis1::Tis1,
        tis1::Tis1,
        Tis1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            tis1::Tis1,
            tis1::Tis1,
            Tis1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tis1 {
    #[inline(always)]
    fn default() -> Tis1 {
        <crate::RegValueT<Tis1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tis1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tis0_SPEC;
    pub type Tis0 = crate::EnumBitfieldStruct<u8, Tis0_SPEC>;
    impl Tis0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tis1_SPEC;
    pub type Tis1 = crate::EnumBitfieldStruct<u8, Tis1_SPEC>;
    impl Tis1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ulbs_SPEC;
impl crate::sealed::RegSpec for Ulbs_SPEC {
    type DataType = u8;
}

pub type Ulbs = crate::RegValueT<Ulbs_SPEC>;

impl Ulbs {
    #[inline(always)]
    pub fn ulbs0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ulbs::Ulbs0,
        ulbs::Ulbs0,
        Ulbs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ulbs::Ulbs0,
            ulbs::Ulbs0,
            Ulbs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ulbs1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ulbs::Ulbs1,
        ulbs::Ulbs1,
        Ulbs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ulbs::Ulbs1,
            ulbs::Ulbs1,
            Ulbs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ulbs2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ulbs::Ulbs2,
        ulbs::Ulbs2,
        Ulbs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ulbs::Ulbs2,
            ulbs::Ulbs2,
            Ulbs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ulbs4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ulbs::Ulbs4,
        ulbs::Ulbs4,
        Ulbs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ulbs::Ulbs4,
            ulbs::Ulbs4,
            Ulbs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ulbs {
    #[inline(always)]
    fn default() -> Ulbs {
        <crate::RegValueT<Ulbs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ulbs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ulbs0_SPEC;
    pub type Ulbs0 = crate::EnumBitfieldStruct<u8, Ulbs0_SPEC>;
    impl Ulbs0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ulbs1_SPEC;
    pub type Ulbs1 = crate::EnumBitfieldStruct<u8, Ulbs1_SPEC>;
    impl Ulbs1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ulbs2_SPEC;
    pub type Ulbs2 = crate::EnumBitfieldStruct<u8, Ulbs2_SPEC>;
    impl Ulbs2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ulbs4_SPEC;
    pub type Ulbs4 = crate::EnumBitfieldStruct<u8, Ulbs4_SPEC>;
    impl Ulbs4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
