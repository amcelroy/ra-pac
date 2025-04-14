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
#[doc = r"Memory Mirror Function"]
unsafe impl ::core::marker::Send for super::Mmf {}
unsafe impl ::core::marker::Sync for super::Mmf {}
impl super::Mmf {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn mmsfr(&self) -> &'static crate::common::Reg<self::Mmsfr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmsfr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mmen(&self) -> &'static crate::common::Reg<self::Mmen_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmen_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmsfr_SPEC;
impl crate::sealed::RegSpec for Mmsfr_SPEC {
    type DataType = u32;
}

pub type Mmsfr = crate::RegValueT<Mmsfr_SPEC>;

impl Mmsfr {
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        mmsfr::Key,
        mmsfr::Key,
        Mmsfr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            mmsfr::Key,
            mmsfr::Key,
            Mmsfr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn memmiraddr(
        self,
    ) -> crate::common::RegisterField<7, 0xffff, 1, 0, u16, u16, Mmsfr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0xffff,1,0,u16,u16,Mmsfr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, Mmsfr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,Mmsfr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmsfr {
    #[inline(always)]
    fn default() -> Mmsfr {
        <crate::RegValueT<Mmsfr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmsfr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Key_SPEC;
    pub type Key = crate::EnumBitfieldStruct<u8, Key_SPEC>;
    impl Key {
        pub const _0_X_DB: Self = Self::new(219);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmen_SPEC;
impl crate::sealed::RegSpec for Mmen_SPEC {
    type DataType = u32;
}

pub type Mmen = crate::RegValueT<Mmen_SPEC>;

impl Mmen {
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        mmen::Key,
        mmen::Key,
        Mmen_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            mmen::Key,
            mmen::Key,
            Mmen_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7fffff, 1, 0, u32, u32, Mmen_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7fffff,1,0,u32,u32,Mmen_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, mmen::En, mmen::En, Mmen_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,mmen::En,mmen::En,Mmen_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmen {
    #[inline(always)]
    fn default() -> Mmen {
        <crate::RegValueT<Mmen_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmen {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Key_SPEC;
    pub type Key = crate::EnumBitfieldStruct<u8, Key_SPEC>;
    impl Key {
        pub const _0_X_DB: Self = Self::new(219);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        pub const _1: Self = Self::new(1);

        pub const _0: Self = Self::new(0);
    }
}
