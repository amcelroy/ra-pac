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
#[doc = r"Data Operation Circuit-4"]
unsafe impl ::core::marker::Send for super::Doc {}
unsafe impl ::core::marker::Sync for super::Doc {}
impl super::Doc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn docr(&self) -> &'static crate::common::Reg<self::Docr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Docr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dosr(&self) -> &'static crate::common::Reg<self::Dosr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Dosr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn doscr(&self) -> &'static crate::common::Reg<self::Doscr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Doscr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dodir(&self) -> &'static crate::common::Reg<self::Dodir_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dodir_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dodir_ha(
        &self,
    ) -> &'static crate::common::Reg<self::DodirHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DodirHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dodsr0(
        &self,
    ) -> &'static crate::common::Reg<self::Dodsr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dodsr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dodsr0_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Dodsr0Ha_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dodsr0Ha_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dodsr1(
        &self,
    ) -> &'static crate::common::Reg<self::Dodsr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dodsr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dodsr1_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Dodsr1Ha_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dodsr1Ha_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Docr_SPEC;
impl crate::sealed::RegSpec for Docr_SPEC {
    type DataType = u8;
}

pub type Docr = crate::RegValueT<Docr_SPEC>;

impl Docr {
    #[inline(always)]
    pub fn oms(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        docr::Oms,
        docr::Oms,
        Docr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            docr::Oms,
            docr::Oms,
            Docr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Docr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Docr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn dobw(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        docr::Dobw,
        docr::Dobw,
        Docr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            docr::Dobw,
            docr::Dobw,
            Docr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dcsel(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x7,
        1,
        0,
        docr::Dcsel,
        docr::Dcsel,
        Docr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            docr::Dcsel,
            docr::Dcsel,
            Docr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dopcie(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        docr::Dopcie,
        docr::Dopcie,
        Docr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            docr::Dopcie,
            docr::Dopcie,
            Docr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Docr {
    #[inline(always)]
    fn default() -> Docr {
        <crate::RegValueT<Docr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod docr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oms_SPEC;
    pub type Oms = crate::EnumBitfieldStruct<u8, Oms_SPEC>;
    impl Oms {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dobw_SPEC;
    pub type Dobw = crate::EnumBitfieldStruct<u8, Dobw_SPEC>;
    impl Dobw {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcsel_SPEC;
    pub type Dcsel = crate::EnumBitfieldStruct<u8, Dcsel_SPEC>;
    impl Dcsel {
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
    pub struct Dopcie_SPEC;
    pub type Dopcie = crate::EnumBitfieldStruct<u8, Dopcie_SPEC>;
    impl Dopcie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dosr_SPEC;
impl crate::sealed::RegSpec for Dosr_SPEC {
    type DataType = u8;
}

pub type Dosr = crate::RegValueT<Dosr_SPEC>;

impl Dosr {
    #[inline(always)]
    pub fn dopcf(self) -> crate::common::RegisterFieldBool<0, 1, 0, Dosr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Dosr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Dosr_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Dosr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Dosr {
    #[inline(always)]
    fn default() -> Dosr {
        <crate::RegValueT<Dosr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doscr_SPEC;
impl crate::sealed::RegSpec for Doscr_SPEC {
    type DataType = u8;
}

pub type Doscr = crate::RegValueT<Doscr_SPEC>;

impl Doscr {
    #[inline(always)]
    pub fn dopcfcl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        doscr::Dopcfcl,
        doscr::Dopcfcl,
        Doscr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            doscr::Dopcfcl,
            doscr::Dopcfcl,
            Doscr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Doscr_SPEC, crate::common::W> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Doscr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Doscr {
    #[inline(always)]
    fn default() -> Doscr {
        <crate::RegValueT<Doscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod doscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dopcfcl_SPEC;
    pub type Dopcfcl = crate::EnumBitfieldStruct<u8, Dopcfcl_SPEC>;
    impl Dopcfcl {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dodir_SPEC;
impl crate::sealed::RegSpec for Dodir_SPEC {
    type DataType = u32;
}

pub type Dodir = crate::RegValueT<Dodir_SPEC>;

impl Dodir {
    #[inline(always)]
    pub fn dodir(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Dodir_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Dodir_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dodir {
    #[inline(always)]
    fn default() -> Dodir {
        <crate::RegValueT<Dodir_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DodirHa_SPEC;
impl crate::sealed::RegSpec for DodirHa_SPEC {
    type DataType = u16;
}

pub type DodirHa = crate::RegValueT<DodirHa_SPEC>;

impl DodirHa {
    #[inline(always)]
    pub fn dodir_ha(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, DodirHa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,DodirHa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DodirHa {
    #[inline(always)]
    fn default() -> DodirHa {
        <crate::RegValueT<DodirHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dodsr0_SPEC;
impl crate::sealed::RegSpec for Dodsr0_SPEC {
    type DataType = u32;
}

pub type Dodsr0 = crate::RegValueT<Dodsr0_SPEC>;

impl Dodsr0 {
    #[inline(always)]
    pub fn dodsr0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Dodsr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Dodsr0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dodsr0 {
    #[inline(always)]
    fn default() -> Dodsr0 {
        <crate::RegValueT<Dodsr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dodsr0Ha_SPEC;
impl crate::sealed::RegSpec for Dodsr0Ha_SPEC {
    type DataType = u16;
}

pub type Dodsr0Ha = crate::RegValueT<Dodsr0Ha_SPEC>;

impl Dodsr0Ha {
    #[inline(always)]
    pub fn dodsr0_ha(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dodsr0Ha_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dodsr0Ha_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dodsr0Ha {
    #[inline(always)]
    fn default() -> Dodsr0Ha {
        <crate::RegValueT<Dodsr0Ha_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dodsr1_SPEC;
impl crate::sealed::RegSpec for Dodsr1_SPEC {
    type DataType = u32;
}

pub type Dodsr1 = crate::RegValueT<Dodsr1_SPEC>;

impl Dodsr1 {
    #[inline(always)]
    pub fn dodsr1(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Dodsr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Dodsr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dodsr1 {
    #[inline(always)]
    fn default() -> Dodsr1 {
        <crate::RegValueT<Dodsr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dodsr1Ha_SPEC;
impl crate::sealed::RegSpec for Dodsr1Ha_SPEC {
    type DataType = u16;
}

pub type Dodsr1Ha = crate::RegValueT<Dodsr1Ha_SPEC>;

impl Dodsr1Ha {
    #[inline(always)]
    pub fn dodsr1_ha(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dodsr1Ha_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dodsr1Ha_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dodsr1Ha {
    #[inline(always)]
    fn default() -> Dodsr1Ha {
        <crate::RegValueT<Dodsr1Ha_SPEC> as RegisterValue<_>>::new(0)
    }
}
