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
// Generated from SVD 1.0, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:19:15 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"12-bit D/A converter"]
unsafe impl ::core::marker::Send for super::Dac12 {}
unsafe impl ::core::marker::Sync for super::Dac12 {}
impl super::Dac12 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn dadr0(&self) -> &'static crate::common::Reg<self::Dadr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dadr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dacr(&self) -> &'static crate::common::Reg<self::Dacr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dacr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dadpr(&self) -> &'static crate::common::Reg<self::Dadpr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dadpr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5usize),
            )
        }
    }

    #[inline(always)]
    pub const fn daadscr(
        &self,
    ) -> &'static crate::common::Reg<self::Daadscr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Daadscr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[inline(always)]
    pub const fn davrefcr(
        &self,
    ) -> &'static crate::common::Reg<self::Davrefcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Davrefcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(7usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dadr0_SPEC;
impl crate::sealed::RegSpec for Dadr0_SPEC {
    type DataType = u16;
}

pub type Dadr0 = crate::RegValueT<Dadr0_SPEC>;

impl Dadr0 {
    #[inline(always)]
    pub fn dadr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dadr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dadr0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dadr0 {
    #[inline(always)]
    fn default() -> Dadr0 {
        <crate::RegValueT<Dadr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dacr_SPEC;
impl crate::sealed::RegSpec for Dacr_SPEC {
    type DataType = u8;
}

pub type Dacr = crate::RegValueT<Dacr_SPEC>;

impl Dacr {
    #[inline(always)]
    pub fn daoe0(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        dacr::Daoe0,
        dacr::Daoe0,
        Dacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            dacr::Daoe0,
            dacr::Daoe0,
            Dacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Dacr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Dacr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dacr {
    #[inline(always)]
    fn default() -> Dacr {
        <crate::RegValueT<Dacr_SPEC> as RegisterValue<_>>::new(31)
    }
}
pub mod dacr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Daoe0_SPEC;
    pub type Daoe0 = crate::EnumBitfieldStruct<u8, Daoe0_SPEC>;
    impl Daoe0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dadpr_SPEC;
impl crate::sealed::RegSpec for Dadpr_SPEC {
    type DataType = u8;
}

pub type Dadpr = crate::RegValueT<Dadpr_SPEC>;

impl Dadpr {
    #[inline(always)]
    pub fn dpsel(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dadpr::Dpsel,
        dadpr::Dpsel,
        Dadpr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dadpr::Dpsel,
            dadpr::Dpsel,
            Dadpr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, Dadpr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,Dadpr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dadpr {
    #[inline(always)]
    fn default() -> Dadpr {
        <crate::RegValueT<Dadpr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dadpr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpsel_SPEC;
    pub type Dpsel = crate::EnumBitfieldStruct<u8, Dpsel_SPEC>;
    impl Dpsel {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Daadscr_SPEC;
impl crate::sealed::RegSpec for Daadscr_SPEC {
    type DataType = u8;
}

pub type Daadscr = crate::RegValueT<Daadscr_SPEC>;

impl Daadscr {
    #[inline(always)]
    pub fn daadst(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        daadscr::Daadst,
        daadscr::Daadst,
        Daadscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            daadscr::Daadst,
            daadscr::Daadst,
            Daadscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Daadscr {
    #[inline(always)]
    fn default() -> Daadscr {
        <crate::RegValueT<Daadscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod daadscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Daadst_SPEC;
    pub type Daadst = crate::EnumBitfieldStruct<u8, Daadst_SPEC>;
    impl Daadst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Davrefcr_SPEC;
impl crate::sealed::RegSpec for Davrefcr_SPEC {
    type DataType = u8;
}

pub type Davrefcr = crate::RegValueT<Davrefcr_SPEC>;

impl Davrefcr {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Davrefcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Davrefcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r#ref(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        davrefcr::Ref,
        davrefcr::Ref,
        Davrefcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            davrefcr::Ref,
            davrefcr::Ref,
            Davrefcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Davrefcr {
    #[inline(always)]
    fn default() -> Davrefcr {
        <crate::RegValueT<Davrefcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod davrefcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ref_SPEC;
    pub type Ref = crate::EnumBitfieldStruct<u8, Ref_SPEC>;
    impl Ref {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _011: Self = Self::new(3);

        pub const _110: Self = Self::new(6);

        pub const OTHERS: Self = Self::new(0);
    }
}
