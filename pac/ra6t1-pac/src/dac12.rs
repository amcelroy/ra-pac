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
// Generated from SVD 1.0, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:21:11 +0000

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
    pub const fn dadr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Dadr_SPEC, crate::common::RW>,
        2,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x0usize))
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
    pub const fn daampcr(
        &self,
    ) -> &'static crate::common::Reg<self::Daampcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Daampcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn daaswcr(
        &self,
    ) -> &'static crate::common::Reg<self::Daaswcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Daaswcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4124usize),
            )
        }
    }

    #[inline(always)]
    pub const fn daadusr(
        &self,
    ) -> &'static crate::common::Reg<self::Daadusr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Daadusr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4288usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dadr_SPEC;
impl crate::sealed::RegSpec for Dadr_SPEC {
    type DataType = u16;
}

pub type Dadr = crate::RegValueT<Dadr_SPEC>;

impl Dadr {
    #[inline(always)]
    pub fn dadr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dadr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dadr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dadr {
    #[inline(always)]
    fn default() -> Dadr {
        <crate::RegValueT<Dadr_SPEC> as RegisterValue<_>>::new(0)
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
    pub fn daoe1(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dacr::Daoe1,
        dacr::Daoe1,
        Dacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dacr::Daoe1,
            dacr::Daoe1,
            Dacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

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
    pub fn dae(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dacr::Dae,
        dacr::Dae,
        Dacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dacr::Dae,
            dacr::Dae,
            Dacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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
    pub struct Daoe1_SPEC;
    pub type Daoe1 = crate::EnumBitfieldStruct<u8, Daoe1_SPEC>;
    impl Daoe1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Daoe0_SPEC;
    pub type Daoe0 = crate::EnumBitfieldStruct<u8, Daoe0_SPEC>;
    impl Daoe0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dae_SPEC;
    pub type Dae = crate::EnumBitfieldStruct<u8, Dae_SPEC>;
    impl Dae {
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
pub struct Daampcr_SPEC;
impl crate::sealed::RegSpec for Daampcr_SPEC {
    type DataType = u8;
}

pub type Daampcr = crate::RegValueT<Daampcr_SPEC>;

impl Daampcr {
    #[inline(always)]
    pub fn daamp1(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        daampcr::Daamp1,
        daampcr::Daamp1,
        Daampcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            daampcr::Daamp1,
            daampcr::Daamp1,
            Daampcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn daamp0(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        daampcr::Daamp0,
        daampcr::Daamp0,
        Daampcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            daampcr::Daamp0,
            daampcr::Daamp0,
            Daampcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Daampcr {
    #[inline(always)]
    fn default() -> Daampcr {
        <crate::RegValueT<Daampcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod daampcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Daamp1_SPEC;
    pub type Daamp1 = crate::EnumBitfieldStruct<u8, Daamp1_SPEC>;
    impl Daamp1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Daamp0_SPEC;
    pub type Daamp0 = crate::EnumBitfieldStruct<u8, Daamp0_SPEC>;
    impl Daamp0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Daaswcr_SPEC;
impl crate::sealed::RegSpec for Daaswcr_SPEC {
    type DataType = u8;
}

pub type Daaswcr = crate::RegValueT<Daaswcr_SPEC>;

impl Daaswcr {
    #[inline(always)]
    pub fn daasw1(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        daaswcr::Daasw1,
        daaswcr::Daasw1,
        Daaswcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            daaswcr::Daasw1,
            daaswcr::Daasw1,
            Daaswcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn daasw0(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        daaswcr::Daasw0,
        daaswcr::Daasw0,
        Daaswcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            daaswcr::Daasw0,
            daaswcr::Daasw0,
            Daaswcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Daaswcr {
    #[inline(always)]
    fn default() -> Daaswcr {
        <crate::RegValueT<Daaswcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod daaswcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Daasw1_SPEC;
    pub type Daasw1 = crate::EnumBitfieldStruct<u8, Daasw1_SPEC>;
    impl Daasw1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Daasw0_SPEC;
    pub type Daasw0 = crate::EnumBitfieldStruct<u8, Daasw0_SPEC>;
    impl Daasw0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Daadusr_SPEC;
impl crate::sealed::RegSpec for Daadusr_SPEC {
    type DataType = u8;
}

pub type Daadusr = crate::RegValueT<Daadusr_SPEC>;

impl Daadusr {
    #[inline(always)]
    pub fn amadsel1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        daadusr::Amadsel1,
        daadusr::Amadsel1,
        Daadusr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            daadusr::Amadsel1,
            daadusr::Amadsel1,
            Daadusr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Daadusr {
    #[inline(always)]
    fn default() -> Daadusr {
        <crate::RegValueT<Daadusr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod daadusr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Amadsel1_SPEC;
    pub type Amadsel1 = crate::EnumBitfieldStruct<u8, Amadsel1_SPEC>;
    impl Amadsel1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
