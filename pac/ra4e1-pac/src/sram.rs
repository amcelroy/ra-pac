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
// Generated from SVD 1.20.00, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:17:48 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"SRAM Control"]
unsafe impl ::core::marker::Send for super::Sram {}
unsafe impl ::core::marker::Sync for super::Sram {}
impl super::Sram {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn parioad(
        &self,
    ) -> &'static crate::common::Reg<self::Parioad_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Parioad_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sramprcr(
        &self,
    ) -> &'static crate::common::Reg<self::Sramprcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sramprcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sramwtsc(
        &self,
    ) -> &'static crate::common::Reg<self::Sramwtsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sramwtsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sramprcr2(
        &self,
    ) -> &'static crate::common::Reg<self::Sramprcr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sramprcr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Parioad_SPEC;
impl crate::sealed::RegSpec for Parioad_SPEC {
    type DataType = u8;
}

pub type Parioad = crate::RegValueT<Parioad_SPEC>;

impl Parioad {
    #[inline(always)]
    pub fn oad(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        parioad::Oad,
        parioad::Oad,
        Parioad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            parioad::Oad,
            parioad::Oad,
            Parioad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Parioad {
    #[inline(always)]
    fn default() -> Parioad {
        <crate::RegValueT<Parioad_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod parioad {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oad_SPEC;
    pub type Oad = crate::EnumBitfieldStruct<u8, Oad_SPEC>;
    impl Oad {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramprcr_SPEC;
impl crate::sealed::RegSpec for Sramprcr_SPEC {
    type DataType = u8;
}

pub type Sramprcr = crate::RegValueT<Sramprcr_SPEC>;

impl Sramprcr {
    #[inline(always)]
    pub fn sramprcr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sramprcr::Sramprcr,
        sramprcr::Sramprcr,
        Sramprcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sramprcr::Sramprcr,
            sramprcr::Sramprcr,
            Sramprcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn kw(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Sramprcr_SPEC, crate::common::W> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Sramprcr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Sramprcr {
    #[inline(always)]
    fn default() -> Sramprcr {
        <crate::RegValueT<Sramprcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sramprcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sramprcr_SPEC;
    pub type Sramprcr = crate::EnumBitfieldStruct<u8, Sramprcr_SPEC>;
    impl Sramprcr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramwtsc_SPEC;
impl crate::sealed::RegSpec for Sramwtsc_SPEC {
    type DataType = u8;
}

pub type Sramwtsc = crate::RegValueT<Sramwtsc_SPEC>;

impl Sramwtsc {
    #[inline(always)]
    pub fn sram0wten(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sramwtsc::Sram0Wten,
        sramwtsc::Sram0Wten,
        Sramwtsc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sramwtsc::Sram0Wten,
            sramwtsc::Sram0Wten,
            Sramwtsc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sramwtsc {
    #[inline(always)]
    fn default() -> Sramwtsc {
        <crate::RegValueT<Sramwtsc_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod sramwtsc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sram0Wten_SPEC;
    pub type Sram0Wten = crate::EnumBitfieldStruct<u8, Sram0Wten_SPEC>;
    impl Sram0Wten {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramprcr2_SPEC;
impl crate::sealed::RegSpec for Sramprcr2_SPEC {
    type DataType = u8;
}

pub type Sramprcr2 = crate::RegValueT<Sramprcr2_SPEC>;

impl Sramprcr2 {
    #[inline(always)]
    pub fn sramprcr2(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sramprcr2::Sramprcr2,
        sramprcr2::Sramprcr2,
        Sramprcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sramprcr2::Sramprcr2,
            sramprcr2::Sramprcr2,
            Sramprcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn kw(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Sramprcr2_SPEC, crate::common::W> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Sramprcr2_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Sramprcr2 {
    #[inline(always)]
    fn default() -> Sramprcr2 {
        <crate::RegValueT<Sramprcr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sramprcr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sramprcr2_SPEC;
    pub type Sramprcr2 = crate::EnumBitfieldStruct<u8, Sramprcr2_SPEC>;
    impl Sramprcr2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
