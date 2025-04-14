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
// Generated from SVD 1.00.01, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:22:22 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Serial Peripheral Interface 0"]
unsafe impl ::core::marker::Send for super::Spi0BNs {}
unsafe impl ::core::marker::Sync for super::Spi0BNs {}
impl super::Spi0BNs {
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
    pub const fn spcmd(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Spcmd_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x14usize))
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
    pub const fn spsrc(&self) -> &'static crate::common::Reg<self::Spsrc_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Spsrc_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[inline(always)]
    pub const fn spfcr(&self) -> &'static crate::common::Reg<self::Spfcr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Spfcr_SPEC, crate::common::W>::from_ptr(
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

        pub const OTHERS: Self = Self::new(0);
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
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmsttg_SPEC;
    pub type Rmsttg = crate::EnumBitfieldStruct<u8, Rmsttg_SPEC>;
    impl Rmsttg {
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
    pub fn spbr(
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
pub struct Spcmd_SPEC;
impl crate::sealed::RegSpec for Spcmd_SPEC {
    type DataType = u32;
}

pub type Spcmd = crate::RegValueT<Spcmd_SPEC>;

impl Spcmd {
    #[inline(always)]
    pub fn cpha(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        spcmd::Cpha,
        spcmd::Cpha,
        Spcmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            spcmd::Cpha,
            spcmd::Cpha,
            Spcmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cpol(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        spcmd::Cpol,
        spcmd::Cpol,
        Spcmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            spcmd::Cpol,
            spcmd::Cpol,
            Spcmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn brdv(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        spcmd::Brdv,
        spcmd::Brdv,
        Spcmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            spcmd::Brdv,
            spcmd::Brdv,
            Spcmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sslkp(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        spcmd::Sslkp,
        spcmd::Sslkp,
        Spcmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            spcmd::Sslkp,
            spcmd::Sslkp,
            Spcmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lsbf(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        spcmd::Lsbf,
        spcmd::Lsbf,
        Spcmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            spcmd::Lsbf,
            spcmd::Lsbf,
            Spcmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spnden(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        spcmd::Spnden,
        spcmd::Spnden,
        Spcmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            spcmd::Spnden,
            spcmd::Spnden,
            Spcmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn slnden(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        spcmd::Slnden,
        spcmd::Slnden,
        Spcmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            spcmd::Slnden,
            spcmd::Slnden,
            Spcmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sckden(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        spcmd::Sckden,
        spcmd::Sckden,
        Spcmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            spcmd::Sckden,
            spcmd::Sckden,
            Spcmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spb(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Spcmd_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Spcmd_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ssla(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        spcmd::Ssla,
        spcmd::Ssla,
        Spcmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            spcmd::Ssla,
            spcmd::Ssla,
            Spcmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Spcmd {
    #[inline(always)]
    fn default() -> Spcmd {
        <crate::RegValueT<Spcmd_SPEC> as RegisterValue<_>>::new(458752)
    }
}
pub mod spcmd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpha_SPEC;
    pub type Cpha = crate::EnumBitfieldStruct<u8, Cpha_SPEC>;
    impl Cpha {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpol_SPEC;
    pub type Cpol = crate::EnumBitfieldStruct<u8, Cpol_SPEC>;
    impl Cpol {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brdv_SPEC;
    pub type Brdv = crate::EnumBitfieldStruct<u8, Brdv_SPEC>;
    impl Brdv {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sslkp_SPEC;
    pub type Sslkp = crate::EnumBitfieldStruct<u8, Sslkp_SPEC>;
    impl Sslkp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lsbf_SPEC;
    pub type Lsbf = crate::EnumBitfieldStruct<u8, Lsbf_SPEC>;
    impl Lsbf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spnden_SPEC;
    pub type Spnden = crate::EnumBitfieldStruct<u8, Spnden_SPEC>;
    impl Spnden {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slnden_SPEC;
    pub type Slnden = crate::EnumBitfieldStruct<u8, Slnden_SPEC>;
    impl Slnden {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sckden_SPEC;
    pub type Sckden = crate::EnumBitfieldStruct<u8, Sckden_SPEC>;
    impl Sckden {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssla_SPEC;
    pub type Ssla = crate::EnumBitfieldStruct<u8, Ssla_SPEC>;
    impl Ssla {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const OTHERS: Self = Self::new(0);
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
    pub struct Spfc_SPEC;
    pub type Spfc = crate::EnumBitfieldStruct<u8, Spfc_SPEC>;
    impl Spfc {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
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

        pub const _1: Self = Self::new(1);
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
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Sptfsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Sptfsr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Sptfsr {
    #[inline(always)]
    fn default() -> Sptfsr {
        <crate::RegValueT<Sptfsr_SPEC> as RegisterValue<_>>::new(0)
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
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Sprfsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Sprfsr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Sprfsr {
    #[inline(always)]
    fn default() -> Sprfsr {
        <crate::RegValueT<Sprfsr_SPEC> as RegisterValue<_>>::new(4)
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
    pub fn speps(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sppsr::Speps,
        sppsr::Speps,
        Sppsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sppsr::Speps,
            sppsr::Speps,
            Sppsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sppsr {
    #[inline(always)]
    fn default() -> Sppsr {
        <crate::RegValueT<Sppsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sppsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Speps_SPEC;
    pub type Speps = crate::EnumBitfieldStruct<u8, Speps_SPEC>;
    impl Speps {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
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
}
impl ::core::default::Default for Spfcr {
    #[inline(always)]
    fn default() -> Spfcr {
        <crate::RegValueT<Spfcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
