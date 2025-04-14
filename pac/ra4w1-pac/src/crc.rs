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
#[doc = r"CRC Calculator"]
unsafe impl ::core::marker::Send for super::Crc {}
unsafe impl ::core::marker::Sync for super::Crc {}
impl super::Crc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn crccr0(
        &self,
    ) -> &'static crate::common::Reg<self::Crccr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Crccr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn crccr1(
        &self,
    ) -> &'static crate::common::Reg<self::Crccr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Crccr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1usize),
            )
        }
    }

    #[inline(always)]
    pub const fn crcdir(
        &self,
    ) -> &'static crate::common::Reg<self::Crcdir_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Crcdir_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn crcdir_by(
        &self,
    ) -> &'static crate::common::Reg<self::CrcdirBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CrcdirBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn crcdor(
        &self,
    ) -> &'static crate::common::Reg<self::Crcdor_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Crcdor_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn crcdor_ha(
        &self,
    ) -> &'static crate::common::Reg<self::CrcdorHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CrcdorHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn crcdor_by(
        &self,
    ) -> &'static crate::common::Reg<self::CrcdorBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CrcdorBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn crcsar(
        &self,
    ) -> &'static crate::common::Reg<self::Crcsar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Crcsar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crccr0_SPEC;
impl crate::sealed::RegSpec for Crccr0_SPEC {
    type DataType = u8;
}

pub type Crccr0 = crate::RegValueT<Crccr0_SPEC>;

impl Crccr0 {
    #[inline(always)]
    pub fn dorcir(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        crccr0::Dorcir,
        crccr0::Dorcir,
        Crccr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            crccr0::Dorcir,
            crccr0::Dorcir,
            Crccr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lms(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        crccr0::Lms,
        crccr0::Lms,
        Crccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            crccr0::Lms,
            crccr0::Lms,
            Crccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, u8, Crccr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x7,1,0,u8,u8,Crccr0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gps(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        crccr0::Gps,
        crccr0::Gps,
        Crccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            crccr0::Gps,
            crccr0::Gps,
            Crccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Crccr0 {
    #[inline(always)]
    fn default() -> Crccr0 {
        <crate::RegValueT<Crccr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod crccr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dorcir_SPEC;
    pub type Dorcir = crate::EnumBitfieldStruct<u8, Dorcir_SPEC>;
    impl Dorcir {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lms_SPEC;
    pub type Lms = crate::EnumBitfieldStruct<u8, Lms_SPEC>;
    impl Lms {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gps_SPEC;
    pub type Gps = crate::EnumBitfieldStruct<u8, Gps_SPEC>;
    impl Gps {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _101: Self = Self::new(5);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crccr1_SPEC;
impl crate::sealed::RegSpec for Crccr1_SPEC {
    type DataType = u8;
}

pub type Crccr1 = crate::RegValueT<Crccr1_SPEC>;

impl Crccr1 {
    #[inline(always)]
    pub fn crcsen(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        crccr1::Crcsen,
        crccr1::Crcsen,
        Crccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            crccr1::Crcsen,
            crccr1::Crcsen,
            Crccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn crcswr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        crccr1::Crcswr,
        crccr1::Crcswr,
        Crccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            crccr1::Crcswr,
            crccr1::Crcswr,
            Crccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Crccr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Crccr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Crccr1 {
    #[inline(always)]
    fn default() -> Crccr1 {
        <crate::RegValueT<Crccr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod crccr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Crcsen_SPEC;
    pub type Crcsen = crate::EnumBitfieldStruct<u8, Crcsen_SPEC>;
    impl Crcsen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Crcswr_SPEC;
    pub type Crcswr = crate::EnumBitfieldStruct<u8, Crcswr_SPEC>;
    impl Crcswr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crcdir_SPEC;
impl crate::sealed::RegSpec for Crcdir_SPEC {
    type DataType = u32;
}

pub type Crcdir = crate::RegValueT<Crcdir_SPEC>;

impl Crcdir {
    #[inline(always)]
    pub fn crcdir(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Crcdir_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Crcdir_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Crcdir {
    #[inline(always)]
    fn default() -> Crcdir {
        <crate::RegValueT<Crcdir_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CrcdirBy_SPEC;
impl crate::sealed::RegSpec for CrcdirBy_SPEC {
    type DataType = u8;
}

pub type CrcdirBy = crate::RegValueT<CrcdirBy_SPEC>;

impl CrcdirBy {
    #[inline(always)]
    pub fn crcdir_by(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, CrcdirBy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,CrcdirBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CrcdirBy {
    #[inline(always)]
    fn default() -> CrcdirBy {
        <crate::RegValueT<CrcdirBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crcdor_SPEC;
impl crate::sealed::RegSpec for Crcdor_SPEC {
    type DataType = u32;
}

pub type Crcdor = crate::RegValueT<Crcdor_SPEC>;

impl Crcdor {
    #[inline(always)]
    pub fn crcdor(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Crcdor_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Crcdor_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Crcdor {
    #[inline(always)]
    fn default() -> Crcdor {
        <crate::RegValueT<Crcdor_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CrcdorHa_SPEC;
impl crate::sealed::RegSpec for CrcdorHa_SPEC {
    type DataType = u16;
}

pub type CrcdorHa = crate::RegValueT<CrcdorHa_SPEC>;

impl CrcdorHa {
    #[inline(always)]
    pub fn crcdor_ha(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, CrcdorHa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,CrcdorHa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CrcdorHa {
    #[inline(always)]
    fn default() -> CrcdorHa {
        <crate::RegValueT<CrcdorHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CrcdorBy_SPEC;
impl crate::sealed::RegSpec for CrcdorBy_SPEC {
    type DataType = u8;
}

pub type CrcdorBy = crate::RegValueT<CrcdorBy_SPEC>;

impl CrcdorBy {
    #[inline(always)]
    pub fn crcdor_by(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, CrcdorBy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,CrcdorBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CrcdorBy {
    #[inline(always)]
    fn default() -> CrcdorBy {
        <crate::RegValueT<CrcdorBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crcsar_SPEC;
impl crate::sealed::RegSpec for Crcsar_SPEC {
    type DataType = u16;
}

pub type Crcsar = crate::RegValueT<Crcsar_SPEC>;

impl Crcsar {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, Crcsar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,Crcsar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn crcsa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3fff,
        1,
        0,
        crcsar::Crcsa,
        crcsar::Crcsa,
        Crcsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3fff,
            1,
            0,
            crcsar::Crcsa,
            crcsar::Crcsa,
            Crcsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Crcsar {
    #[inline(always)]
    fn default() -> Crcsar {
        <crate::RegValueT<Crcsar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod crcsar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Crcsa_SPEC;
    pub type Crcsa = crate::EnumBitfieldStruct<u16, Crcsa_SPEC>;
    impl Crcsa {
        pub const _0_X_0003: Self = Self::new(3);

        pub const _0_X_0005: Self = Self::new(5);

        pub const _0_X_0023: Self = Self::new(35);

        pub const _0_X_0025: Self = Self::new(37);

        pub const _0_X_0043: Self = Self::new(67);

        pub const _0_X_0045: Self = Self::new(69);

        pub const _0_X_0063: Self = Self::new(99);

        pub const _0_X_0065: Self = Self::new(101);

        pub const _0_X_0083: Self = Self::new(131);

        pub const _0_X_0085: Self = Self::new(133);

        pub const _0_X_0123: Self = Self::new(291);

        pub const _0_X_0125: Self = Self::new(293);

        pub const OTHERS: Self = Self::new(0);
    }
}
