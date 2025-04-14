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
#[doc = r"Flash I/O Registers"]
unsafe impl ::core::marker::Send for super::Flcn {}
unsafe impl ::core::marker::Sync for super::Flcn {}
impl super::Flcn {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn dflctl(
        &self,
    ) -> &'static crate::common::Reg<self::Dflctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dflctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tscdr(&self) -> &'static crate::common::Reg<self::Tscdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Tscdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(552usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fldwaitr(
        &self,
    ) -> &'static crate::common::Reg<self::Fldwaitr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fldwaitr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16324usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pfber(&self) -> &'static crate::common::Reg<self::Pfber_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pfber_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16328usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dflctl_SPEC;
impl crate::sealed::RegSpec for Dflctl_SPEC {
    type DataType = u8;
}

pub type Dflctl = crate::RegValueT<Dflctl_SPEC>;

impl Dflctl {
    #[inline(always)]
    pub fn dflen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dflctl::Dflen,
        dflctl::Dflen,
        Dflctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dflctl::Dflen,
            dflctl::Dflen,
            Dflctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dflctl {
    #[inline(always)]
    fn default() -> Dflctl {
        <crate::RegValueT<Dflctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dflctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dflen_SPEC;
    pub type Dflen = crate::EnumBitfieldStruct<u8, Dflen_SPEC>;
    impl Dflen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tscdr_SPEC;
impl crate::sealed::RegSpec for Tscdr_SPEC {
    type DataType = u16;
}

pub type Tscdr = crate::RegValueT<Tscdr_SPEC>;

impl Tscdr {
    #[inline(always)]
    pub fn tscdr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Tscdr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Tscdr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Tscdr {
    #[inline(always)]
    fn default() -> Tscdr {
        <crate::RegValueT<Tscdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fldwaitr_SPEC;
impl crate::sealed::RegSpec for Fldwaitr_SPEC {
    type DataType = u8;
}

pub type Fldwaitr = crate::RegValueT<Fldwaitr_SPEC>;

impl Fldwaitr {
    #[inline(always)]
    pub fn fldwait1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fldwaitr::Fldwait1,
        fldwaitr::Fldwait1,
        Fldwaitr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fldwaitr::Fldwait1,
            fldwaitr::Fldwait1,
            Fldwaitr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fldwaitr {
    #[inline(always)]
    fn default() -> Fldwaitr {
        <crate::RegValueT<Fldwaitr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fldwaitr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fldwait1_SPEC;
    pub type Fldwait1 = crate::EnumBitfieldStruct<u8, Fldwait1_SPEC>;
    impl Fldwait1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfber_SPEC;
impl crate::sealed::RegSpec for Pfber_SPEC {
    type DataType = u8;
}

pub type Pfber = crate::RegValueT<Pfber_SPEC>;

impl Pfber {
    #[inline(always)]
    pub fn pfbe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pfber::Pfbe,
        pfber::Pfbe,
        Pfber_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pfber::Pfbe,
            pfber::Pfbe,
            Pfber_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pfber {
    #[inline(always)]
    fn default() -> Pfber {
        <crate::RegValueT<Pfber_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pfber {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfbe_SPEC;
    pub type Pfbe = crate::EnumBitfieldStruct<u8, Pfbe_SPEC>;
    impl Pfbe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
