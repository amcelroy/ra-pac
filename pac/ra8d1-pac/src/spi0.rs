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
#[doc = r"Serial Peripheral Interface 0"]
unsafe impl ::core::marker::Send for super::Spi0 {}
unsafe impl ::core::marker::Sync for super::Spi0 {}
impl super::Spi0 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn spdr(&self) -> &'static crate::common::Reg<self::Spdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn spdecr(
        &self,
    ) -> &'static crate::common::Reg<self::Spdecr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spdecr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn spcr(&self) -> &'static crate::common::Reg<self::Spcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn spcr2(&self) -> &'static crate::common::Reg<self::Spcr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spcr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn spcr3(&self) -> &'static crate::common::Reg<self::Spcr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spcr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn spcmd0(
        &self,
    ) -> &'static crate::common::Reg<self::Spcmd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spcmd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn spcmd1(
        &self,
    ) -> &'static crate::common::Reg<self::Spcmd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spcmd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn spcmd2(
        &self,
    ) -> &'static crate::common::Reg<self::Spcmd2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spcmd2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn spcmd3(
        &self,
    ) -> &'static crate::common::Reg<self::Spcmd3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spcmd3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn spcmd4(
        &self,
    ) -> &'static crate::common::Reg<self::Spcmd4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spcmd4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn spcmd5(
        &self,
    ) -> &'static crate::common::Reg<self::Spcmd5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spcmd5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn spcmd6(
        &self,
    ) -> &'static crate::common::Reg<self::Spcmd6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spcmd6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn spcmd7(
        &self,
    ) -> &'static crate::common::Reg<self::Spcmd7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spcmd7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn spdcr(&self) -> &'static crate::common::Reg<self::Spdcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spdcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn spdcr2(
        &self,
    ) -> &'static crate::common::Reg<self::Spdcr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spdcr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[inline(always)]
    pub const fn spsr(&self) -> &'static crate::common::Reg<self::Spsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Spsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sptfsr(&self) -> &'static crate::common::Reg<self::Sptfsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sptfsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sprfsr(&self) -> &'static crate::common::Reg<self::Sprfsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sprfsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sppsr(&self) -> &'static crate::common::Reg<self::Sppsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sppsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[inline(always)]
    pub const fn spsrc(&self) -> &'static crate::common::Reg<self::Spsrc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spsrc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[inline(always)]
    pub const fn spfcr(&self) -> &'static crate::common::Reg<self::Spfcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spfcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spdr_SPEC;
impl crate::sealed::RegSpec for Spdr_SPEC {
    type DataType = u32;
}

pub type Spdr = crate::RegValueT<Spdr_SPEC>;

impl Spdr {
    #[inline(always)]
    pub fn spd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Spdr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Spdr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Spdr {
    #[inline(always)]
    fn default() -> Spdr {
        <crate::RegValueT<Spdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spdecr_SPEC;
impl crate::sealed::RegSpec for Spdecr_SPEC {
    type DataType = u32;
}

pub type Spdecr = crate::RegValueT<Spdecr_SPEC>;

impl Spdecr {
    #[inline(always)]
    pub fn sckdl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        spdecr::Sckdl,
        spdecr::Sckdl,
        Spdecr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            spdecr::Sckdl,
            spdecr::Sckdl,
            Spdecr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn slndl(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        spdecr::Slndl,
        spdecr::Slndl,
        Spdecr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            spdecr::Slndl,
            spdecr::Slndl,
            Spdecr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spndl(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        spdecr::Spndl,
        spdecr::Spndl,
        Spdecr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            spdecr::Spndl,
            spdecr::Spndl,
            Spdecr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn arst(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        spdecr::Arst,
        spdecr::Arst,
        Spdecr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            spdecr::Arst,
            spdecr::Arst,
            Spdecr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0xf, 1, 0, u8, u8, Spdecr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0xf,1,0,u8,u8,Spdecr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn aslpen(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        spdecr::Aslpen,
        spdecr::Aslpen,
        Spdecr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            spdecr::Aslpen,
            spdecr::Aslpen,
            Spdecr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Spdecr {
    #[inline(always)]
    fn default() -> Spdecr {
        <crate::RegValueT<Spdecr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod spdecr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sckdl_SPEC;
    pub type Sckdl = crate::EnumBitfieldStruct<u8, Sckdl_SPEC>;
    impl Sckdl {
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
    pub struct Slndl_SPEC;
    pub type Slndl = crate::EnumBitfieldStruct<u8, Slndl_SPEC>;
    impl Slndl {
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
    pub struct Spndl_SPEC;
    pub type Spndl = crate::EnumBitfieldStruct<u8, Spndl_SPEC>;
    impl Spndl {
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
    pub struct Arst_SPEC;
    pub type Arst = crate::EnumBitfieldStruct<u8, Arst_SPEC>;
    impl Arst {
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
    pub struct Aslpen_SPEC;
    pub type Aslpen = crate::EnumBitfieldStruct<u8, Aslpen_SPEC>;
    impl Aslpen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spcr_SPEC;
impl crate::sealed::RegSpec for Spcr_SPEC {
    type DataType = u32;
}

pub type Spcr = crate::RegValueT<Spcr_SPEC>;

impl Spcr {
    #[inline(always)]
    pub fn spe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        spcr::Spe,
        spcr::Spe,
        Spcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            spcr::Spe,
            spcr::Spe,
            Spcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sptick(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        spcr::Sptick,
        spcr::Sptick,
        Spcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            spcr::Sptick,
            spcr::Sptick,
            Spcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spscksel(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        spcr::Spscksel,
        spcr::Spscksel,
        Spcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            spcr::Spscksel,
            spcr::Spscksel,
            Spcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sppe(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        spcr::Sppe,
        spcr::Sppe,
        Spcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            spcr::Sppe,
            spcr::Sppe,
            Spcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spoe(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        spcr::Spoe,
        spcr::Spoe,
        Spcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            spcr::Spoe,
            spcr::Spoe,
            Spcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pte(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        spcr::Pte,
        spcr::Pte,
        Spcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            spcr::Pte,
            spcr::Pte,
            Spcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sckase(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        spcr::Sckase,
        spcr::Sckase,
        Spcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            spcr::Sckase,
            spcr::Sckase,
            Spcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bfds(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        spcr::Bfds,
        spcr::Bfds,
        Spcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            spcr::Bfds,
            spcr::Bfds,
            Spcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn modfen(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        spcr::Modfen,
        spcr::Modfen,
        Spcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            spcr::Modfen,
            spcr::Modfen,
            Spcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn speie(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        spcr::Speie,
        spcr::Speie,
        Spcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            spcr::Speie,
            spcr::Speie,
            Spcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sprie(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        spcr::Sprie,
        spcr::Sprie,
        Spcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            spcr::Sprie,
            spcr::Sprie,
            Spcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spiie(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        spcr::Spiie,
        spcr::Spiie,
        Spcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            spcr::Spiie,
            spcr::Spiie,
            Spcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spdres(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        spcr::Spdres,
        spcr::Spdres,
        Spcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            spcr::Spdres,
            spcr::Spdres,
            Spcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sptie(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        spcr::Sptie,
        spcr::Sptie,
        Spcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            spcr::Sptie,
            spcr::Sptie,
            Spcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cendie(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        spcr::Cendie,
        spcr::Cendie,
        Spcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            spcr::Cendie,
            spcr::Cendie,
            Spcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spms(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        spcr::Spms,
        spcr::Spms,
        Spcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            spcr::Spms,
            spcr::Spms,
            Spcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spfrf(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        spcr::Spfrf,
        spcr::Spfrf,
        Spcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            spcr::Spfrf,
            spcr::Spfrf,
            Spcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, u8, Spcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x3,1,0,u8,u8,Spcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn txmd(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x3,
        1,
        0,
        spcr::Txmd,
        spcr::Txmd,
        Spcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x3,
            1,
            0,
            spcr::Txmd,
            spcr::Txmd,
            Spcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mstr(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        spcr::Mstr,
        spcr::Mstr,
        Spcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            spcr::Mstr,
            spcr::Mstr,
            Spcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bpen(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        spcr::Bpen,
        spcr::Bpen,
        Spcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            spcr::Bpen,
            spcr::Bpen,
            Spcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Spcr {
    #[inline(always)]
    fn default() -> Spcr {
        <crate::RegValueT<Spcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod spcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spe_SPEC;
    pub type Spe = crate::EnumBitfieldStruct<u8, Spe_SPEC>;
    impl Spe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sptick_SPEC;
    pub type Sptick = crate::EnumBitfieldStruct<u8, Sptick_SPEC>;
    impl Sptick {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spscksel_SPEC;
    pub type Spscksel = crate::EnumBitfieldStruct<u8, Spscksel_SPEC>;
    impl Spscksel {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sppe_SPEC;
    pub type Sppe = crate::EnumBitfieldStruct<u8, Sppe_SPEC>;
    impl Sppe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spoe_SPEC;
    pub type Spoe = crate::EnumBitfieldStruct<u8, Spoe_SPEC>;
    impl Spoe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pte_SPEC;
    pub type Pte = crate::EnumBitfieldStruct<u8, Pte_SPEC>;
    impl Pte {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sckase_SPEC;
    pub type Sckase = crate::EnumBitfieldStruct<u8, Sckase_SPEC>;
    impl Sckase {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bfds_SPEC;
    pub type Bfds = crate::EnumBitfieldStruct<u8, Bfds_SPEC>;
    impl Bfds {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Modfen_SPEC;
    pub type Modfen = crate::EnumBitfieldStruct<u8, Modfen_SPEC>;
    impl Modfen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Speie_SPEC;
    pub type Speie = crate::EnumBitfieldStruct<u8, Speie_SPEC>;
    impl Speie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sprie_SPEC;
    pub type Sprie = crate::EnumBitfieldStruct<u8, Sprie_SPEC>;
    impl Sprie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spiie_SPEC;
    pub type Spiie = crate::EnumBitfieldStruct<u8, Spiie_SPEC>;
    impl Spiie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spdres_SPEC;
    pub type Spdres = crate::EnumBitfieldStruct<u8, Spdres_SPEC>;
    impl Spdres {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sptie_SPEC;
    pub type Sptie = crate::EnumBitfieldStruct<u8, Sptie_SPEC>;
    impl Sptie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cendie_SPEC;
    pub type Cendie = crate::EnumBitfieldStruct<u8, Cendie_SPEC>;
    impl Cendie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spms_SPEC;
    pub type Spms = crate::EnumBitfieldStruct<u8, Spms_SPEC>;
    impl Spms {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spfrf_SPEC;
    pub type Spfrf = crate::EnumBitfieldStruct<u8, Spfrf_SPEC>;
    impl Spfrf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txmd_SPEC;
    pub type Txmd = crate::EnumBitfieldStruct<u8, Txmd_SPEC>;
    impl Txmd {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstr_SPEC;
    pub type Mstr = crate::EnumBitfieldStruct<u8, Mstr_SPEC>;
    impl Mstr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bpen_SPEC;
    pub type Bpen = crate::EnumBitfieldStruct<u8, Bpen_SPEC>;
    impl Bpen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spcr2_SPEC;
impl crate::sealed::RegSpec for Spcr2_SPEC {
    type DataType = u32;
}

pub type Spcr2 = crate::RegValueT<Spcr2_SPEC>;

impl Spcr2 {
    #[inline(always)]
    pub fn rmfm(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Spcr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Spcr2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmedtg(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        spcr2::Rmedtg,
        spcr2::Rmedtg,
        Spcr2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            spcr2::Rmedtg,
            spcr2::Rmedtg,
            Spcr2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmsttg(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        spcr2::Rmsttg,
        spcr2::Rmsttg,
        Spcr2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            spcr2::Rmsttg,
            spcr2::Rmsttg,
            Spcr2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spdrc(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Spcr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Spcr2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn splp(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        spcr2::Splp,
        spcr2::Splp,
        Spcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            spcr2::Splp,
            spcr2::Splp,
            Spcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn splp2(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        spcr2::Splp2,
        spcr2::Splp2,
        Spcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            spcr2::Splp2,
            spcr2::Splp2,
            Spcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spom(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        spcr2::Spom,
        spcr2::Spom,
        Spcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            spcr2::Spom,
            spcr2::Spom,
            Spcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn moifv(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        spcr2::Moifv,
        spcr2::Moifv,
        Spcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            spcr2::Moifv,
            spcr2::Moifv,
            Spcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn moife(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        spcr2::Moife,
        spcr2::Moife,
        Spcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            spcr2::Moife,
            spcr2::Moife,
            Spcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spsckdl(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        spcr2::Spsckdl,
        spcr2::Spsckdl,
        Spcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            spcr2::Spsckdl,
            spcr2::Spsckdl,
            Spcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sptddl(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x7,
        1,
        0,
        spcr2::Sptddl,
        spcr2::Sptddl,
        Spcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x7,
            1,
            0,
            spcr2::Sptddl,
            spcr2::Sptddl,
            Spcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Spcr2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Spcr2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Spcr2 {
    #[inline(always)]
    fn default() -> Spcr2 {
        <crate::RegValueT<Spcr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod spcr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmedtg_SPEC;
    pub type Rmedtg = crate::EnumBitfieldStruct<u8, Rmedtg_SPEC>;
    impl Rmedtg {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmsttg_SPEC;
    pub type Rmsttg = crate::EnumBitfieldStruct<u8, Rmsttg_SPEC>;
    impl Rmsttg {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Splp_SPEC;
    pub type Splp = crate::EnumBitfieldStruct<u8, Splp_SPEC>;
    impl Splp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Splp2_SPEC;
    pub type Splp2 = crate::EnumBitfieldStruct<u8, Splp2_SPEC>;
    impl Splp2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spom_SPEC;
    pub type Spom = crate::EnumBitfieldStruct<u8, Spom_SPEC>;
    impl Spom {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Moifv_SPEC;
    pub type Moifv = crate::EnumBitfieldStruct<u8, Moifv_SPEC>;
    impl Moifv {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Moife_SPEC;
    pub type Moife = crate::EnumBitfieldStruct<u8, Moife_SPEC>;
    impl Moife {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spsckdl_SPEC;
    pub type Spsckdl = crate::EnumBitfieldStruct<u8, Spsckdl_SPEC>;
    impl Spsckdl {
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
    pub struct Sptddl_SPEC;
    pub type Sptddl = crate::EnumBitfieldStruct<u8, Sptddl_SPEC>;
    impl Sptddl {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _101: Self = Self::new(5);

        pub const _110: Self = Self::new(6);

        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spcr3_SPEC;
impl crate::sealed::RegSpec for Spcr3_SPEC {
    type DataType = u32;
}

pub type Spcr3 = crate::RegValueT<Spcr3_SPEC>;

impl Spcr3 {
    #[inline(always)]
    pub fn ssl0p(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        spcr3::Ssl0P,
        spcr3::Ssl0P,
        Spcr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            spcr3::Ssl0P,
            spcr3::Ssl0P,
            Spcr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ssl1p(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        spcr3::Ssl1P,
        spcr3::Ssl1P,
        Spcr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            spcr3::Ssl1P,
            spcr3::Ssl1P,
            Spcr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ssl2p(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        spcr3::Ssl2P,
        spcr3::Ssl2P,
        Spcr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            spcr3::Ssl2P,
            spcr3::Ssl2P,
            Spcr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ssl3p(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        spcr3::Ssl3P,
        spcr3::Ssl3P,
        Spcr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            spcr3::Ssl3P,
            spcr3::Ssl3P,
            Spcr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ssl4p(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        spcr3::Ssl4P,
        spcr3::Ssl4P,
        Spcr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            spcr3::Ssl4P,
            spcr3::Ssl4P,
            Spcr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ssl5p(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        spcr3::Ssl5P,
        spcr3::Ssl5P,
        Spcr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            spcr3::Ssl5P,
            spcr3::Ssl5P,
            Spcr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ssl6p(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        spcr3::Ssl6P,
        spcr3::Ssl6P,
        Spcr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            spcr3::Ssl6P,
            spcr3::Ssl6P,
            Spcr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ssl7p(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        spcr3::Ssl7P,
        spcr3::Ssl7P,
        Spcr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            spcr3::Ssl7P,
            spcr3::Ssl7P,
            Spcr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spr(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Spcr3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Spcr3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spsln(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        spcr3::Spsln,
        spcr3::Spsln,
        Spcr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            spcr3::Spsln,
            spcr3::Spsln,
            Spcr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, Spcr3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,Spcr3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Spcr3 {
    #[inline(always)]
    fn default() -> Spcr3 {
        <crate::RegValueT<Spcr3_SPEC> as RegisterValue<_>>::new(65280)
    }
}
pub mod spcr3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssl0P_SPEC;
    pub type Ssl0P = crate::EnumBitfieldStruct<u8, Ssl0P_SPEC>;
    impl Ssl0P {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssl1P_SPEC;
    pub type Ssl1P = crate::EnumBitfieldStruct<u8, Ssl1P_SPEC>;
    impl Ssl1P {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssl2P_SPEC;
    pub type Ssl2P = crate::EnumBitfieldStruct<u8, Ssl2P_SPEC>;
    impl Ssl2P {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssl3P_SPEC;
    pub type Ssl3P = crate::EnumBitfieldStruct<u8, Ssl3P_SPEC>;
    impl Ssl3P {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssl4P_SPEC;
    pub type Ssl4P = crate::EnumBitfieldStruct<u8, Ssl4P_SPEC>;
    impl Ssl4P {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssl5P_SPEC;
    pub type Ssl5P = crate::EnumBitfieldStruct<u8, Ssl5P_SPEC>;
    impl Ssl5P {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssl6P_SPEC;
    pub type Ssl6P = crate::EnumBitfieldStruct<u8, Ssl6P_SPEC>;
    impl Ssl6P {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssl7P_SPEC;
    pub type Ssl7P = crate::EnumBitfieldStruct<u8, Ssl7P_SPEC>;
    impl Ssl7P {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spsln_SPEC;
    pub type Spsln = crate::EnumBitfieldStruct<u8, Spsln_SPEC>;
    impl Spsln {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _101: Self = Self::new(5);

        pub const _110: Self = Self::new(6);

        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spcmd0_SPEC;
impl crate::sealed::RegSpec for Spcmd0_SPEC {
    type DataType = u32;
}

pub type Spcmd0 = crate::RegValueT<Spcmd0_SPEC>;

impl Spcmd0 {
    #[inline(always)]
    pub fn cpha0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        spcmd0::Cpha0,
        spcmd0::Cpha0,
        Spcmd0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            spcmd0::Cpha0,
            spcmd0::Cpha0,
            Spcmd0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cpol0(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        spcmd0::Cpol0,
        spcmd0::Cpol0,
        Spcmd0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            spcmd0::Cpol0,
            spcmd0::Cpol0,
            Spcmd0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn brdv0(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        spcmd0::Brdv0,
        spcmd0::Brdv0,
        Spcmd0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            spcmd0::Brdv0,
            spcmd0::Brdv0,
            Spcmd0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sslkp0(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        spcmd0::Sslkp0,
        spcmd0::Sslkp0,
        Spcmd0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            spcmd0::Sslkp0,
            spcmd0::Sslkp0,
            Spcmd0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lsbf0(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        spcmd0::Lsbf0,
        spcmd0::Lsbf0,
        Spcmd0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            spcmd0::Lsbf0,
            spcmd0::Lsbf0,
            Spcmd0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spnden0(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        spcmd0::Spnden0,
        spcmd0::Spnden0,
        Spcmd0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            spcmd0::Spnden0,
            spcmd0::Spnden0,
            Spcmd0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn slnden0(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        spcmd0::Slnden0,
        spcmd0::Slnden0,
        Spcmd0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            spcmd0::Slnden0,
            spcmd0::Slnden0,
            Spcmd0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sckden0(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        spcmd0::Sckden0,
        spcmd0::Sckden0,
        Spcmd0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            spcmd0::Sckden0,
            spcmd0::Sckden0,
            Spcmd0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spb0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1f,
        1,
        0,
        spcmd0::Spb0,
        spcmd0::Spb0,
        Spcmd0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1f,
            1,
            0,
            spcmd0::Spb0,
            spcmd0::Spb0,
            Spcmd0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ssla0(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        spcmd0::Ssla0,
        spcmd0::Ssla0,
        Spcmd0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            spcmd0::Ssla0,
            spcmd0::Ssla0,
            Spcmd0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, Spcmd0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,Spcmd0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Spcmd0 {
    #[inline(always)]
    fn default() -> Spcmd0 {
        <crate::RegValueT<Spcmd0_SPEC> as RegisterValue<_>>::new(458752)
    }
}
pub mod spcmd0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpha0_SPEC;
    pub type Cpha0 = crate::EnumBitfieldStruct<u8, Cpha0_SPEC>;
    impl Cpha0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpol0_SPEC;
    pub type Cpol0 = crate::EnumBitfieldStruct<u8, Cpol0_SPEC>;
    impl Cpol0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brdv0_SPEC;
    pub type Brdv0 = crate::EnumBitfieldStruct<u8, Brdv0_SPEC>;
    impl Brdv0 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sslkp0_SPEC;
    pub type Sslkp0 = crate::EnumBitfieldStruct<u8, Sslkp0_SPEC>;
    impl Sslkp0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lsbf0_SPEC;
    pub type Lsbf0 = crate::EnumBitfieldStruct<u8, Lsbf0_SPEC>;
    impl Lsbf0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spnden0_SPEC;
    pub type Spnden0 = crate::EnumBitfieldStruct<u8, Spnden0_SPEC>;
    impl Spnden0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slnden0_SPEC;
    pub type Slnden0 = crate::EnumBitfieldStruct<u8, Slnden0_SPEC>;
    impl Slnden0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sckden0_SPEC;
    pub type Sckden0 = crate::EnumBitfieldStruct<u8, Sckden0_SPEC>;
    impl Sckden0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spb0_SPEC;
    pub type Spb0 = crate::EnumBitfieldStruct<u8, Spb0_SPEC>;
    impl Spb0 {
        pub const _00000: Self = Self::new(0);

        pub const _00011: Self = Self::new(3);

        pub const _00100: Self = Self::new(4);

        pub const _00101: Self = Self::new(5);

        pub const _11110: Self = Self::new(30);

        pub const _11111: Self = Self::new(31);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssla0_SPEC;
    pub type Ssla0 = crate::EnumBitfieldStruct<u8, Ssla0_SPEC>;
    impl Ssla0 {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _101: Self = Self::new(5);

        pub const _110: Self = Self::new(6);

        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spcmd1_SPEC;
impl crate::sealed::RegSpec for Spcmd1_SPEC {
    type DataType = u32;
}

pub type Spcmd1 = crate::RegValueT<Spcmd1_SPEC>;

impl Spcmd1 {
    #[inline(always)]
    pub fn cpha1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        spcmd1::Cpha1,
        spcmd1::Cpha1,
        Spcmd1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            spcmd1::Cpha1,
            spcmd1::Cpha1,
            Spcmd1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cpol1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        spcmd1::Cpol1,
        spcmd1::Cpol1,
        Spcmd1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            spcmd1::Cpol1,
            spcmd1::Cpol1,
            Spcmd1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn brdv1(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        spcmd1::Brdv1,
        spcmd1::Brdv1,
        Spcmd1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            spcmd1::Brdv1,
            spcmd1::Brdv1,
            Spcmd1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sslkp1(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        spcmd1::Sslkp1,
        spcmd1::Sslkp1,
        Spcmd1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            spcmd1::Sslkp1,
            spcmd1::Sslkp1,
            Spcmd1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lsbf1(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        spcmd1::Lsbf1,
        spcmd1::Lsbf1,
        Spcmd1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            spcmd1::Lsbf1,
            spcmd1::Lsbf1,
            Spcmd1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spnden1(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        spcmd1::Spnden1,
        spcmd1::Spnden1,
        Spcmd1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            spcmd1::Spnden1,
            spcmd1::Spnden1,
            Spcmd1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn slnden1(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        spcmd1::Slnden1,
        spcmd1::Slnden1,
        Spcmd1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            spcmd1::Slnden1,
            spcmd1::Slnden1,
            Spcmd1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sckden1(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        spcmd1::Sckden1,
        spcmd1::Sckden1,
        Spcmd1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            spcmd1::Sckden1,
            spcmd1::Sckden1,
            Spcmd1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spb1(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1f,
        1,
        0,
        spcmd1::Spb1,
        spcmd1::Spb1,
        Spcmd1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1f,
            1,
            0,
            spcmd1::Spb1,
            spcmd1::Spb1,
            Spcmd1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ssla1(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        spcmd1::Ssla1,
        spcmd1::Ssla1,
        Spcmd1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            spcmd1::Ssla1,
            spcmd1::Ssla1,
            Spcmd1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, Spcmd1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,Spcmd1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Spcmd1 {
    #[inline(always)]
    fn default() -> Spcmd1 {
        <crate::RegValueT<Spcmd1_SPEC> as RegisterValue<_>>::new(458752)
    }
}
pub mod spcmd1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpha1_SPEC;
    pub type Cpha1 = crate::EnumBitfieldStruct<u8, Cpha1_SPEC>;
    impl Cpha1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpol1_SPEC;
    pub type Cpol1 = crate::EnumBitfieldStruct<u8, Cpol1_SPEC>;
    impl Cpol1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brdv1_SPEC;
    pub type Brdv1 = crate::EnumBitfieldStruct<u8, Brdv1_SPEC>;
    impl Brdv1 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sslkp1_SPEC;
    pub type Sslkp1 = crate::EnumBitfieldStruct<u8, Sslkp1_SPEC>;
    impl Sslkp1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lsbf1_SPEC;
    pub type Lsbf1 = crate::EnumBitfieldStruct<u8, Lsbf1_SPEC>;
    impl Lsbf1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spnden1_SPEC;
    pub type Spnden1 = crate::EnumBitfieldStruct<u8, Spnden1_SPEC>;
    impl Spnden1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slnden1_SPEC;
    pub type Slnden1 = crate::EnumBitfieldStruct<u8, Slnden1_SPEC>;
    impl Slnden1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sckden1_SPEC;
    pub type Sckden1 = crate::EnumBitfieldStruct<u8, Sckden1_SPEC>;
    impl Sckden1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spb1_SPEC;
    pub type Spb1 = crate::EnumBitfieldStruct<u8, Spb1_SPEC>;
    impl Spb1 {
        pub const _00000: Self = Self::new(0);

        pub const _00011: Self = Self::new(3);

        pub const _00100: Self = Self::new(4);

        pub const _00101: Self = Self::new(5);

        pub const _11110: Self = Self::new(30);

        pub const _11111: Self = Self::new(31);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssla1_SPEC;
    pub type Ssla1 = crate::EnumBitfieldStruct<u8, Ssla1_SPEC>;
    impl Ssla1 {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _101: Self = Self::new(5);

        pub const _110: Self = Self::new(6);

        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spcmd2_SPEC;
impl crate::sealed::RegSpec for Spcmd2_SPEC {
    type DataType = u32;
}

pub type Spcmd2 = crate::RegValueT<Spcmd2_SPEC>;

impl Spcmd2 {
    #[inline(always)]
    pub fn cpha2(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        spcmd2::Cpha2,
        spcmd2::Cpha2,
        Spcmd2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            spcmd2::Cpha2,
            spcmd2::Cpha2,
            Spcmd2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cpol2(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        spcmd2::Cpol2,
        spcmd2::Cpol2,
        Spcmd2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            spcmd2::Cpol2,
            spcmd2::Cpol2,
            Spcmd2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn brdv2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        spcmd2::Brdv2,
        spcmd2::Brdv2,
        Spcmd2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            spcmd2::Brdv2,
            spcmd2::Brdv2,
            Spcmd2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sslkp2(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        spcmd2::Sslkp2,
        spcmd2::Sslkp2,
        Spcmd2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            spcmd2::Sslkp2,
            spcmd2::Sslkp2,
            Spcmd2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lsbf2(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        spcmd2::Lsbf2,
        spcmd2::Lsbf2,
        Spcmd2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            spcmd2::Lsbf2,
            spcmd2::Lsbf2,
            Spcmd2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spnden2(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        spcmd2::Spnden2,
        spcmd2::Spnden2,
        Spcmd2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            spcmd2::Spnden2,
            spcmd2::Spnden2,
            Spcmd2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn slnden2(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        spcmd2::Slnden2,
        spcmd2::Slnden2,
        Spcmd2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            spcmd2::Slnden2,
            spcmd2::Slnden2,
            Spcmd2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sckden2(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        spcmd2::Sckden2,
        spcmd2::Sckden2,
        Spcmd2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            spcmd2::Sckden2,
            spcmd2::Sckden2,
            Spcmd2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spb2(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1f,
        1,
        0,
        spcmd2::Spb2,
        spcmd2::Spb2,
        Spcmd2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1f,
            1,
            0,
            spcmd2::Spb2,
            spcmd2::Spb2,
            Spcmd2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ssla2(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        spcmd2::Ssla2,
        spcmd2::Ssla2,
        Spcmd2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            spcmd2::Ssla2,
            spcmd2::Ssla2,
            Spcmd2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, Spcmd2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,Spcmd2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Spcmd2 {
    #[inline(always)]
    fn default() -> Spcmd2 {
        <crate::RegValueT<Spcmd2_SPEC> as RegisterValue<_>>::new(458752)
    }
}
pub mod spcmd2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpha2_SPEC;
    pub type Cpha2 = crate::EnumBitfieldStruct<u8, Cpha2_SPEC>;
    impl Cpha2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpol2_SPEC;
    pub type Cpol2 = crate::EnumBitfieldStruct<u8, Cpol2_SPEC>;
    impl Cpol2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brdv2_SPEC;
    pub type Brdv2 = crate::EnumBitfieldStruct<u8, Brdv2_SPEC>;
    impl Brdv2 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sslkp2_SPEC;
    pub type Sslkp2 = crate::EnumBitfieldStruct<u8, Sslkp2_SPEC>;
    impl Sslkp2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lsbf2_SPEC;
    pub type Lsbf2 = crate::EnumBitfieldStruct<u8, Lsbf2_SPEC>;
    impl Lsbf2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spnden2_SPEC;
    pub type Spnden2 = crate::EnumBitfieldStruct<u8, Spnden2_SPEC>;
    impl Spnden2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slnden2_SPEC;
    pub type Slnden2 = crate::EnumBitfieldStruct<u8, Slnden2_SPEC>;
    impl Slnden2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sckden2_SPEC;
    pub type Sckden2 = crate::EnumBitfieldStruct<u8, Sckden2_SPEC>;
    impl Sckden2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spb2_SPEC;
    pub type Spb2 = crate::EnumBitfieldStruct<u8, Spb2_SPEC>;
    impl Spb2 {
        pub const _00000: Self = Self::new(0);

        pub const _00011: Self = Self::new(3);

        pub const _00100: Self = Self::new(4);

        pub const _00101: Self = Self::new(5);

        pub const _11110: Self = Self::new(30);

        pub const _11111: Self = Self::new(31);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssla2_SPEC;
    pub type Ssla2 = crate::EnumBitfieldStruct<u8, Ssla2_SPEC>;
    impl Ssla2 {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _101: Self = Self::new(5);

        pub const _110: Self = Self::new(6);

        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spcmd3_SPEC;
impl crate::sealed::RegSpec for Spcmd3_SPEC {
    type DataType = u32;
}

pub type Spcmd3 = crate::RegValueT<Spcmd3_SPEC>;

impl Spcmd3 {
    #[inline(always)]
    pub fn cpha3(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        spcmd3::Cpha3,
        spcmd3::Cpha3,
        Spcmd3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            spcmd3::Cpha3,
            spcmd3::Cpha3,
            Spcmd3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cpol3(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        spcmd3::Cpol3,
        spcmd3::Cpol3,
        Spcmd3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            spcmd3::Cpol3,
            spcmd3::Cpol3,
            Spcmd3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn brdv3(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        spcmd3::Brdv3,
        spcmd3::Brdv3,
        Spcmd3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            spcmd3::Brdv3,
            spcmd3::Brdv3,
            Spcmd3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sslkp3(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        spcmd3::Sslkp3,
        spcmd3::Sslkp3,
        Spcmd3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            spcmd3::Sslkp3,
            spcmd3::Sslkp3,
            Spcmd3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lsbf3(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        spcmd3::Lsbf3,
        spcmd3::Lsbf3,
        Spcmd3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            spcmd3::Lsbf3,
            spcmd3::Lsbf3,
            Spcmd3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spnden3(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        spcmd3::Spnden3,
        spcmd3::Spnden3,
        Spcmd3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            spcmd3::Spnden3,
            spcmd3::Spnden3,
            Spcmd3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn slnden3(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        spcmd3::Slnden3,
        spcmd3::Slnden3,
        Spcmd3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            spcmd3::Slnden3,
            spcmd3::Slnden3,
            Spcmd3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sckden3(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        spcmd3::Sckden3,
        spcmd3::Sckden3,
        Spcmd3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            spcmd3::Sckden3,
            spcmd3::Sckden3,
            Spcmd3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spb3(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1f,
        1,
        0,
        spcmd3::Spb3,
        spcmd3::Spb3,
        Spcmd3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1f,
            1,
            0,
            spcmd3::Spb3,
            spcmd3::Spb3,
            Spcmd3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ssla3(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        spcmd3::Ssla3,
        spcmd3::Ssla3,
        Spcmd3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            spcmd3::Ssla3,
            spcmd3::Ssla3,
            Spcmd3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, Spcmd3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,Spcmd3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Spcmd3 {
    #[inline(always)]
    fn default() -> Spcmd3 {
        <crate::RegValueT<Spcmd3_SPEC> as RegisterValue<_>>::new(458752)
    }
}
pub mod spcmd3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpha3_SPEC;
    pub type Cpha3 = crate::EnumBitfieldStruct<u8, Cpha3_SPEC>;
    impl Cpha3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpol3_SPEC;
    pub type Cpol3 = crate::EnumBitfieldStruct<u8, Cpol3_SPEC>;
    impl Cpol3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brdv3_SPEC;
    pub type Brdv3 = crate::EnumBitfieldStruct<u8, Brdv3_SPEC>;
    impl Brdv3 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sslkp3_SPEC;
    pub type Sslkp3 = crate::EnumBitfieldStruct<u8, Sslkp3_SPEC>;
    impl Sslkp3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lsbf3_SPEC;
    pub type Lsbf3 = crate::EnumBitfieldStruct<u8, Lsbf3_SPEC>;
    impl Lsbf3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spnden3_SPEC;
    pub type Spnden3 = crate::EnumBitfieldStruct<u8, Spnden3_SPEC>;
    impl Spnden3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slnden3_SPEC;
    pub type Slnden3 = crate::EnumBitfieldStruct<u8, Slnden3_SPEC>;
    impl Slnden3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sckden3_SPEC;
    pub type Sckden3 = crate::EnumBitfieldStruct<u8, Sckden3_SPEC>;
    impl Sckden3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spb3_SPEC;
    pub type Spb3 = crate::EnumBitfieldStruct<u8, Spb3_SPEC>;
    impl Spb3 {
        pub const _00000: Self = Self::new(0);

        pub const _00011: Self = Self::new(3);

        pub const _00100: Self = Self::new(4);

        pub const _00101: Self = Self::new(5);

        pub const _11110: Self = Self::new(30);

        pub const _11111: Self = Self::new(31);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssla3_SPEC;
    pub type Ssla3 = crate::EnumBitfieldStruct<u8, Ssla3_SPEC>;
    impl Ssla3 {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _101: Self = Self::new(5);

        pub const _110: Self = Self::new(6);

        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spcmd4_SPEC;
impl crate::sealed::RegSpec for Spcmd4_SPEC {
    type DataType = u32;
}

pub type Spcmd4 = crate::RegValueT<Spcmd4_SPEC>;

impl Spcmd4 {
    #[inline(always)]
    pub fn cpha4(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        spcmd4::Cpha4,
        spcmd4::Cpha4,
        Spcmd4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            spcmd4::Cpha4,
            spcmd4::Cpha4,
            Spcmd4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cpol4(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        spcmd4::Cpol4,
        spcmd4::Cpol4,
        Spcmd4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            spcmd4::Cpol4,
            spcmd4::Cpol4,
            Spcmd4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn brdv4(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        spcmd4::Brdv4,
        spcmd4::Brdv4,
        Spcmd4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            spcmd4::Brdv4,
            spcmd4::Brdv4,
            Spcmd4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sslkp4(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        spcmd4::Sslkp4,
        spcmd4::Sslkp4,
        Spcmd4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            spcmd4::Sslkp4,
            spcmd4::Sslkp4,
            Spcmd4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lsbf4(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        spcmd4::Lsbf4,
        spcmd4::Lsbf4,
        Spcmd4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            spcmd4::Lsbf4,
            spcmd4::Lsbf4,
            Spcmd4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spnden4(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        spcmd4::Spnden4,
        spcmd4::Spnden4,
        Spcmd4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            spcmd4::Spnden4,
            spcmd4::Spnden4,
            Spcmd4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn slnden4(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        spcmd4::Slnden4,
        spcmd4::Slnden4,
        Spcmd4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            spcmd4::Slnden4,
            spcmd4::Slnden4,
            Spcmd4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sckden4(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        spcmd4::Sckden4,
        spcmd4::Sckden4,
        Spcmd4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            spcmd4::Sckden4,
            spcmd4::Sckden4,
            Spcmd4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spb4(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1f,
        1,
        0,
        spcmd4::Spb4,
        spcmd4::Spb4,
        Spcmd4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1f,
            1,
            0,
            spcmd4::Spb4,
            spcmd4::Spb4,
            Spcmd4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ssla4(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        spcmd4::Ssla4,
        spcmd4::Ssla4,
        Spcmd4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            spcmd4::Ssla4,
            spcmd4::Ssla4,
            Spcmd4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, Spcmd4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,Spcmd4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Spcmd4 {
    #[inline(always)]
    fn default() -> Spcmd4 {
        <crate::RegValueT<Spcmd4_SPEC> as RegisterValue<_>>::new(458752)
    }
}
pub mod spcmd4 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpha4_SPEC;
    pub type Cpha4 = crate::EnumBitfieldStruct<u8, Cpha4_SPEC>;
    impl Cpha4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpol4_SPEC;
    pub type Cpol4 = crate::EnumBitfieldStruct<u8, Cpol4_SPEC>;
    impl Cpol4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brdv4_SPEC;
    pub type Brdv4 = crate::EnumBitfieldStruct<u8, Brdv4_SPEC>;
    impl Brdv4 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sslkp4_SPEC;
    pub type Sslkp4 = crate::EnumBitfieldStruct<u8, Sslkp4_SPEC>;
    impl Sslkp4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lsbf4_SPEC;
    pub type Lsbf4 = crate::EnumBitfieldStruct<u8, Lsbf4_SPEC>;
    impl Lsbf4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spnden4_SPEC;
    pub type Spnden4 = crate::EnumBitfieldStruct<u8, Spnden4_SPEC>;
    impl Spnden4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slnden4_SPEC;
    pub type Slnden4 = crate::EnumBitfieldStruct<u8, Slnden4_SPEC>;
    impl Slnden4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sckden4_SPEC;
    pub type Sckden4 = crate::EnumBitfieldStruct<u8, Sckden4_SPEC>;
    impl Sckden4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spb4_SPEC;
    pub type Spb4 = crate::EnumBitfieldStruct<u8, Spb4_SPEC>;
    impl Spb4 {
        pub const _00000: Self = Self::new(0);

        pub const _00011: Self = Self::new(3);

        pub const _00100: Self = Self::new(4);

        pub const _00101: Self = Self::new(5);

        pub const _11110: Self = Self::new(30);

        pub const _11111: Self = Self::new(31);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssla4_SPEC;
    pub type Ssla4 = crate::EnumBitfieldStruct<u8, Ssla4_SPEC>;
    impl Ssla4 {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _101: Self = Self::new(5);

        pub const _110: Self = Self::new(6);

        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spcmd5_SPEC;
impl crate::sealed::RegSpec for Spcmd5_SPEC {
    type DataType = u32;
}

pub type Spcmd5 = crate::RegValueT<Spcmd5_SPEC>;

impl Spcmd5 {
    #[inline(always)]
    pub fn cpha5(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        spcmd5::Cpha5,
        spcmd5::Cpha5,
        Spcmd5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            spcmd5::Cpha5,
            spcmd5::Cpha5,
            Spcmd5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cpol5(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        spcmd5::Cpol5,
        spcmd5::Cpol5,
        Spcmd5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            spcmd5::Cpol5,
            spcmd5::Cpol5,
            Spcmd5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn brdv5(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        spcmd5::Brdv5,
        spcmd5::Brdv5,
        Spcmd5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            spcmd5::Brdv5,
            spcmd5::Brdv5,
            Spcmd5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sslkp5(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        spcmd5::Sslkp5,
        spcmd5::Sslkp5,
        Spcmd5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            spcmd5::Sslkp5,
            spcmd5::Sslkp5,
            Spcmd5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lsbf5(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        spcmd5::Lsbf5,
        spcmd5::Lsbf5,
        Spcmd5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            spcmd5::Lsbf5,
            spcmd5::Lsbf5,
            Spcmd5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spnden5(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        spcmd5::Spnden5,
        spcmd5::Spnden5,
        Spcmd5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            spcmd5::Spnden5,
            spcmd5::Spnden5,
            Spcmd5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn slnden5(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        spcmd5::Slnden5,
        spcmd5::Slnden5,
        Spcmd5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            spcmd5::Slnden5,
            spcmd5::Slnden5,
            Spcmd5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sckden5(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        spcmd5::Sckden5,
        spcmd5::Sckden5,
        Spcmd5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            spcmd5::Sckden5,
            spcmd5::Sckden5,
            Spcmd5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spb5(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1f,
        1,
        0,
        spcmd5::Spb5,
        spcmd5::Spb5,
        Spcmd5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1f,
            1,
            0,
            spcmd5::Spb5,
            spcmd5::Spb5,
            Spcmd5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ssla5(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        spcmd5::Ssla5,
        spcmd5::Ssla5,
        Spcmd5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            spcmd5::Ssla5,
            spcmd5::Ssla5,
            Spcmd5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, Spcmd5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,Spcmd5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Spcmd5 {
    #[inline(always)]
    fn default() -> Spcmd5 {
        <crate::RegValueT<Spcmd5_SPEC> as RegisterValue<_>>::new(458752)
    }
}
pub mod spcmd5 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpha5_SPEC;
    pub type Cpha5 = crate::EnumBitfieldStruct<u8, Cpha5_SPEC>;
    impl Cpha5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpol5_SPEC;
    pub type Cpol5 = crate::EnumBitfieldStruct<u8, Cpol5_SPEC>;
    impl Cpol5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brdv5_SPEC;
    pub type Brdv5 = crate::EnumBitfieldStruct<u8, Brdv5_SPEC>;
    impl Brdv5 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sslkp5_SPEC;
    pub type Sslkp5 = crate::EnumBitfieldStruct<u8, Sslkp5_SPEC>;
    impl Sslkp5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lsbf5_SPEC;
    pub type Lsbf5 = crate::EnumBitfieldStruct<u8, Lsbf5_SPEC>;
    impl Lsbf5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spnden5_SPEC;
    pub type Spnden5 = crate::EnumBitfieldStruct<u8, Spnden5_SPEC>;
    impl Spnden5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slnden5_SPEC;
    pub type Slnden5 = crate::EnumBitfieldStruct<u8, Slnden5_SPEC>;
    impl Slnden5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sckden5_SPEC;
    pub type Sckden5 = crate::EnumBitfieldStruct<u8, Sckden5_SPEC>;
    impl Sckden5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spb5_SPEC;
    pub type Spb5 = crate::EnumBitfieldStruct<u8, Spb5_SPEC>;
    impl Spb5 {
        pub const _00000: Self = Self::new(0);

        pub const _00011: Self = Self::new(3);

        pub const _00100: Self = Self::new(4);

        pub const _00101: Self = Self::new(5);

        pub const _11110: Self = Self::new(30);

        pub const _11111: Self = Self::new(31);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssla5_SPEC;
    pub type Ssla5 = crate::EnumBitfieldStruct<u8, Ssla5_SPEC>;
    impl Ssla5 {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _101: Self = Self::new(5);

        pub const _110: Self = Self::new(6);

        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spcmd6_SPEC;
impl crate::sealed::RegSpec for Spcmd6_SPEC {
    type DataType = u32;
}

pub type Spcmd6 = crate::RegValueT<Spcmd6_SPEC>;

impl Spcmd6 {
    #[inline(always)]
    pub fn cpha6(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        spcmd6::Cpha6,
        spcmd6::Cpha6,
        Spcmd6_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            spcmd6::Cpha6,
            spcmd6::Cpha6,
            Spcmd6_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cpol6(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        spcmd6::Cpol6,
        spcmd6::Cpol6,
        Spcmd6_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            spcmd6::Cpol6,
            spcmd6::Cpol6,
            Spcmd6_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn brdv6(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        spcmd6::Brdv6,
        spcmd6::Brdv6,
        Spcmd6_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            spcmd6::Brdv6,
            spcmd6::Brdv6,
            Spcmd6_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sslkp6(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        spcmd6::Sslkp6,
        spcmd6::Sslkp6,
        Spcmd6_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            spcmd6::Sslkp6,
            spcmd6::Sslkp6,
            Spcmd6_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lsbf6(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        spcmd6::Lsbf6,
        spcmd6::Lsbf6,
        Spcmd6_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            spcmd6::Lsbf6,
            spcmd6::Lsbf6,
            Spcmd6_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spnden6(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        spcmd6::Spnden6,
        spcmd6::Spnden6,
        Spcmd6_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            spcmd6::Spnden6,
            spcmd6::Spnden6,
            Spcmd6_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn slnden6(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        spcmd6::Slnden6,
        spcmd6::Slnden6,
        Spcmd6_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            spcmd6::Slnden6,
            spcmd6::Slnden6,
            Spcmd6_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sckden6(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        spcmd6::Sckden6,
        spcmd6::Sckden6,
        Spcmd6_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            spcmd6::Sckden6,
            spcmd6::Sckden6,
            Spcmd6_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spb6(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1f,
        1,
        0,
        spcmd6::Spb6,
        spcmd6::Spb6,
        Spcmd6_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1f,
            1,
            0,
            spcmd6::Spb6,
            spcmd6::Spb6,
            Spcmd6_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ssla6(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        spcmd6::Ssla6,
        spcmd6::Ssla6,
        Spcmd6_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            spcmd6::Ssla6,
            spcmd6::Ssla6,
            Spcmd6_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, Spcmd6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,Spcmd6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Spcmd6 {
    #[inline(always)]
    fn default() -> Spcmd6 {
        <crate::RegValueT<Spcmd6_SPEC> as RegisterValue<_>>::new(458752)
    }
}
pub mod spcmd6 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpha6_SPEC;
    pub type Cpha6 = crate::EnumBitfieldStruct<u8, Cpha6_SPEC>;
    impl Cpha6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpol6_SPEC;
    pub type Cpol6 = crate::EnumBitfieldStruct<u8, Cpol6_SPEC>;
    impl Cpol6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brdv6_SPEC;
    pub type Brdv6 = crate::EnumBitfieldStruct<u8, Brdv6_SPEC>;
    impl Brdv6 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sslkp6_SPEC;
    pub type Sslkp6 = crate::EnumBitfieldStruct<u8, Sslkp6_SPEC>;
    impl Sslkp6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lsbf6_SPEC;
    pub type Lsbf6 = crate::EnumBitfieldStruct<u8, Lsbf6_SPEC>;
    impl Lsbf6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spnden6_SPEC;
    pub type Spnden6 = crate::EnumBitfieldStruct<u8, Spnden6_SPEC>;
    impl Spnden6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slnden6_SPEC;
    pub type Slnden6 = crate::EnumBitfieldStruct<u8, Slnden6_SPEC>;
    impl Slnden6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sckden6_SPEC;
    pub type Sckden6 = crate::EnumBitfieldStruct<u8, Sckden6_SPEC>;
    impl Sckden6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spb6_SPEC;
    pub type Spb6 = crate::EnumBitfieldStruct<u8, Spb6_SPEC>;
    impl Spb6 {
        pub const _00000: Self = Self::new(0);

        pub const _00011: Self = Self::new(3);

        pub const _00100: Self = Self::new(4);

        pub const _00101: Self = Self::new(5);

        pub const _11110: Self = Self::new(30);

        pub const _11111: Self = Self::new(31);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssla6_SPEC;
    pub type Ssla6 = crate::EnumBitfieldStruct<u8, Ssla6_SPEC>;
    impl Ssla6 {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _101: Self = Self::new(5);

        pub const _110: Self = Self::new(6);

        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spcmd7_SPEC;
impl crate::sealed::RegSpec for Spcmd7_SPEC {
    type DataType = u32;
}

pub type Spcmd7 = crate::RegValueT<Spcmd7_SPEC>;

impl Spcmd7 {
    #[inline(always)]
    pub fn cpha7(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        spcmd7::Cpha7,
        spcmd7::Cpha7,
        Spcmd7_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            spcmd7::Cpha7,
            spcmd7::Cpha7,
            Spcmd7_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cpol7(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        spcmd7::Cpol7,
        spcmd7::Cpol7,
        Spcmd7_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            spcmd7::Cpol7,
            spcmd7::Cpol7,
            Spcmd7_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn brdv7(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        spcmd7::Brdv7,
        spcmd7::Brdv7,
        Spcmd7_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            spcmd7::Brdv7,
            spcmd7::Brdv7,
            Spcmd7_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sslkp7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        spcmd7::Sslkp7,
        spcmd7::Sslkp7,
        Spcmd7_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            spcmd7::Sslkp7,
            spcmd7::Sslkp7,
            Spcmd7_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lsbf7(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        spcmd7::Lsbf7,
        spcmd7::Lsbf7,
        Spcmd7_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            spcmd7::Lsbf7,
            spcmd7::Lsbf7,
            Spcmd7_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spnden7(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        spcmd7::Spnden7,
        spcmd7::Spnden7,
        Spcmd7_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            spcmd7::Spnden7,
            spcmd7::Spnden7,
            Spcmd7_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn slnden7(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        spcmd7::Slnden7,
        spcmd7::Slnden7,
        Spcmd7_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            spcmd7::Slnden7,
            spcmd7::Slnden7,
            Spcmd7_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sckden7(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        spcmd7::Sckden7,
        spcmd7::Sckden7,
        Spcmd7_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            spcmd7::Sckden7,
            spcmd7::Sckden7,
            Spcmd7_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spb7(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1f,
        1,
        0,
        spcmd7::Spb7,
        spcmd7::Spb7,
        Spcmd7_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1f,
            1,
            0,
            spcmd7::Spb7,
            spcmd7::Spb7,
            Spcmd7_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ssla7(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        spcmd7::Ssla7,
        spcmd7::Ssla7,
        Spcmd7_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            spcmd7::Ssla7,
            spcmd7::Ssla7,
            Spcmd7_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, Spcmd7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,Spcmd7_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Spcmd7 {
    #[inline(always)]
    fn default() -> Spcmd7 {
        <crate::RegValueT<Spcmd7_SPEC> as RegisterValue<_>>::new(458752)
    }
}
pub mod spcmd7 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpha7_SPEC;
    pub type Cpha7 = crate::EnumBitfieldStruct<u8, Cpha7_SPEC>;
    impl Cpha7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpol7_SPEC;
    pub type Cpol7 = crate::EnumBitfieldStruct<u8, Cpol7_SPEC>;
    impl Cpol7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brdv7_SPEC;
    pub type Brdv7 = crate::EnumBitfieldStruct<u8, Brdv7_SPEC>;
    impl Brdv7 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sslkp7_SPEC;
    pub type Sslkp7 = crate::EnumBitfieldStruct<u8, Sslkp7_SPEC>;
    impl Sslkp7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lsbf7_SPEC;
    pub type Lsbf7 = crate::EnumBitfieldStruct<u8, Lsbf7_SPEC>;
    impl Lsbf7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spnden7_SPEC;
    pub type Spnden7 = crate::EnumBitfieldStruct<u8, Spnden7_SPEC>;
    impl Spnden7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slnden7_SPEC;
    pub type Slnden7 = crate::EnumBitfieldStruct<u8, Slnden7_SPEC>;
    impl Slnden7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sckden7_SPEC;
    pub type Sckden7 = crate::EnumBitfieldStruct<u8, Sckden7_SPEC>;
    impl Sckden7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spb7_SPEC;
    pub type Spb7 = crate::EnumBitfieldStruct<u8, Spb7_SPEC>;
    impl Spb7 {
        pub const _00000: Self = Self::new(0);

        pub const _00011: Self = Self::new(3);

        pub const _00100: Self = Self::new(4);

        pub const _00101: Self = Self::new(5);

        pub const _11110: Self = Self::new(30);

        pub const _11111: Self = Self::new(31);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssla7_SPEC;
    pub type Ssla7 = crate::EnumBitfieldStruct<u8, Ssla7_SPEC>;
    impl Ssla7 {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _101: Self = Self::new(5);

        pub const _110: Self = Self::new(6);

        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spdcr_SPEC;
impl crate::sealed::RegSpec for Spdcr_SPEC {
    type DataType = u32;
}

pub type Spdcr = crate::RegValueT<Spdcr_SPEC>;

impl Spdcr {
    #[inline(always)]
    pub fn bysw(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        spdcr::Bysw,
        spdcr::Bysw,
        Spdcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            spdcr::Bysw,
            spdcr::Bysw,
            Spdcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn slsel(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x3,
        1,
        0,
        spdcr::Slsel,
        spdcr::Slsel,
        Spdcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x3,
            1,
            0,
            spdcr::Slsel,
            spdcr::Slsel,
            Spdcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sprdtd(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        spdcr::Sprdtd,
        spdcr::Sprdtd,
        Spdcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            spdcr::Sprdtd,
            spdcr::Sprdtd,
            Spdcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sinv(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        spdcr::Sinv,
        spdcr::Sinv,
        Spdcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            spdcr::Sinv,
            spdcr::Sinv,
            Spdcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spwral(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        spdcr::Spwral,
        spdcr::Spwral,
        Spdcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            spdcr::Spwral,
            spdcr::Spwral,
            Spdcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spfc(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        spdcr::Spfc,
        spdcr::Spfc,
        Spdcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            spdcr::Spfc,
            spdcr::Spfc,
            Spdcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Spdcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Spdcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Spdcr {
    #[inline(always)]
    fn default() -> Spdcr {
        <crate::RegValueT<Spdcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod spdcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bysw_SPEC;
    pub type Bysw = crate::EnumBitfieldStruct<u8, Bysw_SPEC>;
    impl Bysw {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slsel_SPEC;
    pub type Slsel = crate::EnumBitfieldStruct<u8, Slsel_SPEC>;
    impl Slsel {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sprdtd_SPEC;
    pub type Sprdtd = crate::EnumBitfieldStruct<u8, Sprdtd_SPEC>;
    impl Sprdtd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sinv_SPEC;
    pub type Sinv = crate::EnumBitfieldStruct<u8, Sinv_SPEC>;
    impl Sinv {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spwral_SPEC;
    pub type Spwral = crate::EnumBitfieldStruct<u8, Spwral_SPEC>;
    impl Spwral {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spfc_SPEC;
    pub type Spfc = crate::EnumBitfieldStruct<u8, Spfc_SPEC>;
    impl Spfc {
        pub const _0000: Self = Self::new(0);

        pub const _0001: Self = Self::new(1);

        pub const _1110: Self = Self::new(2);

        pub const _1111: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spdcr2_SPEC;
impl crate::sealed::RegSpec for Spdcr2_SPEC {
    type DataType = u32;
}

pub type Spdcr2 = crate::RegValueT<Spdcr2_SPEC>;

impl Spdcr2 {
    #[inline(always)]
    pub fn rtrg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        spdcr2::Rtrg,
        spdcr2::Rtrg,
        Spdcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            spdcr2::Rtrg,
            spdcr2::Rtrg,
            Spdcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ttrg(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        spdcr2::Ttrg,
        spdcr2::Ttrg,
        Spdcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            spdcr2::Ttrg,
            spdcr2::Ttrg,
            Spdcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Spdcr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Spdcr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Spdcr2 {
    #[inline(always)]
    fn default() -> Spdcr2 {
        <crate::RegValueT<Spdcr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod spdcr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtrg_SPEC;
    pub type Rtrg = crate::EnumBitfieldStruct<u8, Rtrg_SPEC>;
    impl Rtrg {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ttrg_SPEC;
    pub type Ttrg = crate::EnumBitfieldStruct<u8, Ttrg_SPEC>;
    impl Ttrg {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spsr_SPEC;
impl crate::sealed::RegSpec for Spsr_SPEC {
    type DataType = u32;
}

pub type Spsr = crate::RegValueT<Spsr_SPEC>;

impl Spsr {
    #[inline(always)]
    pub fn spcp(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        spsr::Spcp,
        spsr::Spcp,
        Spsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            spsr::Spcp,
            spsr::Spcp,
            Spsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn specm(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x7,
        1,
        0,
        spsr::Specm,
        spsr::Specm,
        Spsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x7,
            1,
            0,
            spsr::Specm,
            spsr::Specm,
            Spsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, u8, Spsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7f,1,0,u8,u8,Spsr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spdrf(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        spsr::Spdrf,
        spsr::Spdrf,
        Spsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            spsr::Spdrf,
            spsr::Spdrf,
            Spsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovrf(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        spsr::Ovrf,
        spsr::Ovrf,
        Spsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            spsr::Ovrf,
            spsr::Ovrf,
            Spsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn idlnf(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        spsr::Idlnf,
        spsr::Idlnf,
        Spsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            spsr::Idlnf,
            spsr::Idlnf,
            Spsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn modf(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        spsr::Modf,
        spsr::Modf,
        Spsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            spsr::Modf,
            spsr::Modf,
            Spsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn perf(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        spsr::Perf,
        spsr::Perf,
        Spsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            spsr::Perf,
            spsr::Perf,
            Spsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn udrf(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        spsr::Udrf,
        spsr::Udrf,
        Spsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            spsr::Udrf,
            spsr::Udrf,
            Spsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sptef(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        spsr::Sptef,
        spsr::Sptef,
        Spsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            spsr::Sptef,
            spsr::Sptef,
            Spsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cendf(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        spsr::Cendf,
        spsr::Cendf,
        Spsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            spsr::Cendf,
            spsr::Cendf,
            Spsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sprf(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        spsr::Sprf,
        spsr::Sprf,
        Spsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            spsr::Sprf,
            spsr::Sprf,
            Spsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Spsr {
    #[inline(always)]
    fn default() -> Spsr {
        <crate::RegValueT<Spsr_SPEC> as RegisterValue<_>>::new(536870912)
    }
}
pub mod spsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spcp_SPEC;
    pub type Spcp = crate::EnumBitfieldStruct<u8, Spcp_SPEC>;
    impl Spcp {
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
    pub struct Specm_SPEC;
    pub type Specm = crate::EnumBitfieldStruct<u8, Specm_SPEC>;
    impl Specm {
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
    pub struct Spdrf_SPEC;
    pub type Spdrf = crate::EnumBitfieldStruct<u8, Spdrf_SPEC>;
    impl Spdrf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovrf_SPEC;
    pub type Ovrf = crate::EnumBitfieldStruct<u8, Ovrf_SPEC>;
    impl Ovrf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idlnf_SPEC;
    pub type Idlnf = crate::EnumBitfieldStruct<u8, Idlnf_SPEC>;
    impl Idlnf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Modf_SPEC;
    pub type Modf = crate::EnumBitfieldStruct<u8, Modf_SPEC>;
    impl Modf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Perf_SPEC;
    pub type Perf = crate::EnumBitfieldStruct<u8, Perf_SPEC>;
    impl Perf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Udrf_SPEC;
    pub type Udrf = crate::EnumBitfieldStruct<u8, Udrf_SPEC>;
    impl Udrf {
        pub const _0: Self = Self::new(0);

        pub const _10: Self = Self::new(0);

        pub const _11: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sptef_SPEC;
    pub type Sptef = crate::EnumBitfieldStruct<u8, Sptef_SPEC>;
    impl Sptef {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cendf_SPEC;
    pub type Cendf = crate::EnumBitfieldStruct<u8, Cendf_SPEC>;
    impl Cendf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sprf_SPEC;
    pub type Sprf = crate::EnumBitfieldStruct<u8, Sprf_SPEC>;
    impl Sprf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sptfsr_SPEC;
impl crate::sealed::RegSpec for Sptfsr_SPEC {
    type DataType = u32;
}

pub type Sptfsr = crate::RegValueT<Sptfsr_SPEC>;

impl Sptfsr {
    #[inline(always)]
    pub fn tfdn(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        sptfsr::Tfdn,
        sptfsr::Tfdn,
        Sptfsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            sptfsr::Tfdn,
            sptfsr::Tfdn,
            Sptfsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Sptfsr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Sptfsr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Sptfsr {
    #[inline(always)]
    fn default() -> Sptfsr {
        <crate::RegValueT<Sptfsr_SPEC> as RegisterValue<_>>::new(4)
    }
}
pub mod sptfsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tfdn_SPEC;
    pub type Tfdn = crate::EnumBitfieldStruct<u8, Tfdn_SPEC>;
    impl Tfdn {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sprfsr_SPEC;
impl crate::sealed::RegSpec for Sprfsr_SPEC {
    type DataType = u32;
}

pub type Sprfsr = crate::RegValueT<Sprfsr_SPEC>;

impl Sprfsr {
    #[inline(always)]
    pub fn rfdn(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        sprfsr::Rfdn,
        sprfsr::Rfdn,
        Sprfsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            sprfsr::Rfdn,
            sprfsr::Rfdn,
            Sprfsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Sprfsr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Sprfsr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Sprfsr {
    #[inline(always)]
    fn default() -> Sprfsr {
        <crate::RegValueT<Sprfsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sprfsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdn_SPEC;
    pub type Rfdn = crate::EnumBitfieldStruct<u8, Rfdn_SPEC>;
    impl Rfdn {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sppsr_SPEC;
impl crate::sealed::RegSpec for Sppsr_SPEC {
    type DataType = u32;
}

pub type Sppsr = crate::RegValueT<Sppsr_SPEC>;

impl Sppsr {
    #[inline(always)]
    pub fn speps(self) -> crate::common::RegisterFieldBool<0, 1, 0, Sppsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Sppsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Sppsr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Sppsr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Sppsr {
    #[inline(always)]
    fn default() -> Sppsr {
        <crate::RegValueT<Sppsr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spsrc_SPEC;
impl crate::sealed::RegSpec for Spsrc_SPEC {
    type DataType = u32;
}

pub type Spsrc = crate::RegValueT<Spsrc_SPEC>;

impl Spsrc {
    #[inline(always)]
    pub fn spdrfc(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Spsrc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, Spsrc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn ovrfc(self) -> crate::common::RegisterFieldBool<24, 1, 0, Spsrc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, Spsrc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Spsrc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Spsrc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn modfc(self) -> crate::common::RegisterFieldBool<26, 1, 0, Spsrc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, Spsrc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn perfc(self) -> crate::common::RegisterFieldBool<27, 1, 0, Spsrc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<27, 1, 0, Spsrc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn udrfc(self) -> crate::common::RegisterFieldBool<28, 1, 0, Spsrc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, Spsrc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn sptefc(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Spsrc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<29, 1, 0, Spsrc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn cendfc(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Spsrc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Spsrc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn sprfc(self) -> crate::common::RegisterFieldBool<31, 1, 0, Spsrc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Spsrc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Spsrc {
    #[inline(always)]
    fn default() -> Spsrc {
        <crate::RegValueT<Spsrc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spfcr_SPEC;
impl crate::sealed::RegSpec for Spfcr_SPEC {
    type DataType = u32;
}

pub type Spfcr = crate::RegValueT<Spfcr_SPEC>;

impl Spfcr {
    #[inline(always)]
    pub fn spfrst(self) -> crate::common::RegisterFieldBool<0, 1, 0, Spfcr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Spfcr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Spfcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Spfcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Spfcr {
    #[inline(always)]
    fn default() -> Spfcr {
        <crate::RegValueT<Spfcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
