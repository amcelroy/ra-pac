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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:21:54 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Independent Watchdog Timer"]
unsafe impl ::core::marker::Send for super::Iwdt {}
unsafe impl ::core::marker::Sync for super::Iwdt {}
impl super::Iwdt {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn iwdtrr(
        &self,
    ) -> &'static crate::common::Reg<self::Iwdtrr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iwdtrr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn iwdtcr(
        &self,
    ) -> &'static crate::common::Reg<self::Iwdtcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iwdtcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[inline(always)]
    pub const fn iwdtrcr(
        &self,
    ) -> &'static crate::common::Reg<self::Iwdtrcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iwdtrcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[inline(always)]
    pub const fn iwdtcstpr(
        &self,
    ) -> &'static crate::common::Reg<self::Iwdtcstpr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iwdtcstpr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iwdtrr_SPEC;
impl crate::sealed::RegSpec for Iwdtrr_SPEC {
    type DataType = u8;
}

pub type Iwdtrr = crate::RegValueT<Iwdtrr_SPEC>;

impl Iwdtrr {
    #[inline(always)]
    pub fn iwdtrr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Iwdtrr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Iwdtrr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Iwdtrr {
    #[inline(always)]
    fn default() -> Iwdtrr {
        <crate::RegValueT<Iwdtrr_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iwdtcr_SPEC;
impl crate::sealed::RegSpec for Iwdtcr_SPEC {
    type DataType = u16;
}

pub type Iwdtcr = crate::RegValueT<Iwdtcr_SPEC>;

impl Iwdtcr {
    #[inline(always)]
    pub fn tops(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        iwdtcr::Tops,
        iwdtcr::Tops,
        Iwdtcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            iwdtcr::Tops,
            iwdtcr::Tops,
            Iwdtcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xf,
        1,
        0,
        iwdtcr::Cks,
        iwdtcr::Cks,
        Iwdtcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            iwdtcr::Cks,
            iwdtcr::Cks,
            Iwdtcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rpes(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        iwdtcr::Rpes,
        iwdtcr::Rpes,
        Iwdtcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            iwdtcr::Rpes,
            iwdtcr::Rpes,
            Iwdtcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rpss(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        iwdtcr::Rpss,
        iwdtcr::Rpss,
        Iwdtcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            iwdtcr::Rpss,
            iwdtcr::Rpss,
            Iwdtcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, Iwdtcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,Iwdtcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Iwdtcr {
    #[inline(always)]
    fn default() -> Iwdtcr {
        <crate::RegValueT<Iwdtcr_SPEC> as RegisterValue<_>>::new(13299)
    }
}
pub mod iwdtcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tops_SPEC;
    pub type Tops = crate::EnumBitfieldStruct<u8, Tops_SPEC>;
    impl Tops {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);

        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
        pub const _0000: Self = Self::new(0);

        pub const _0010: Self = Self::new(2);

        pub const _0011: Self = Self::new(3);

        pub const _0100: Self = Self::new(4);

        pub const _0111: Self = Self::new(7);

        pub const _0101: Self = Self::new(5);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpes_SPEC;
    pub type Rpes = crate::EnumBitfieldStruct<u8, Rpes_SPEC>;
    impl Rpes {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpss_SPEC;
    pub type Rpss = crate::EnumBitfieldStruct<u8, Rpss_SPEC>;
    impl Rpss {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iwdtrcr_SPEC;
impl crate::sealed::RegSpec for Iwdtrcr_SPEC {
    type DataType = u8;
}

pub type Iwdtrcr = crate::RegValueT<Iwdtrcr_SPEC>;

impl Iwdtrcr {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, Iwdtrcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,Iwdtrcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rstirqs(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        iwdtrcr::Rstirqs,
        iwdtrcr::Rstirqs,
        Iwdtrcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            iwdtrcr::Rstirqs,
            iwdtrcr::Rstirqs,
            Iwdtrcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iwdtrcr {
    #[inline(always)]
    fn default() -> Iwdtrcr {
        <crate::RegValueT<Iwdtrcr_SPEC> as RegisterValue<_>>::new(128)
    }
}
pub mod iwdtrcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rstirqs_SPEC;
    pub type Rstirqs = crate::EnumBitfieldStruct<u8, Rstirqs_SPEC>;
    impl Rstirqs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iwdtcstpr_SPEC;
impl crate::sealed::RegSpec for Iwdtcstpr_SPEC {
    type DataType = u8;
}

pub type Iwdtcstpr = crate::RegValueT<Iwdtcstpr_SPEC>;

impl Iwdtcstpr {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Iwdtcstpr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Iwdtcstpr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cpu0sel(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        iwdtcstpr::Cpu0Sel,
        iwdtcstpr::Cpu0Sel,
        Iwdtcstpr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            iwdtcstpr::Cpu0Sel,
            iwdtcstpr::Cpu0Sel,
            Iwdtcstpr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn slcstp(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        iwdtcstpr::Slcstp,
        iwdtcstpr::Slcstp,
        Iwdtcstpr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            iwdtcstpr::Slcstp,
            iwdtcstpr::Slcstp,
            Iwdtcstpr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iwdtcstpr {
    #[inline(always)]
    fn default() -> Iwdtcstpr {
        <crate::RegValueT<Iwdtcstpr_SPEC> as RegisterValue<_>>::new(192)
    }
}
pub mod iwdtcstpr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpu0Sel_SPEC;
    pub type Cpu0Sel = crate::EnumBitfieldStruct<u8, Cpu0Sel_SPEC>;
    impl Cpu0Sel {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slcstp_SPEC;
    pub type Slcstp = crate::EnumBitfieldStruct<u8, Slcstp_SPEC>;
    impl Slcstp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
