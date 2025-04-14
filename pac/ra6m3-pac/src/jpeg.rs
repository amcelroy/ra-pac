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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:20:22 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"JPEG Codec"]
unsafe impl ::core::marker::Send for super::Jpeg {}
unsafe impl ::core::marker::Sync for super::Jpeg {}
impl super::Jpeg {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn jcmod(&self) -> &'static crate::common::Reg<self::Jcmod_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jcmod_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn jccmd(&self) -> &'static crate::common::Reg<self::Jccmd_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Jccmd_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(1usize),
            )
        }
    }

    #[inline(always)]
    pub const fn jcqtn(&self) -> &'static crate::common::Reg<self::Jcqtn_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jcqtn_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3usize),
            )
        }
    }

    #[inline(always)]
    pub const fn jchtn(&self) -> &'static crate::common::Reg<self::Jchtn_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jchtn_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn jcdriu(
        &self,
    ) -> &'static crate::common::Reg<self::Jcdriu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jcdriu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5usize),
            )
        }
    }

    #[inline(always)]
    pub const fn jcdrid(
        &self,
    ) -> &'static crate::common::Reg<self::Jcdrid_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jcdrid_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[inline(always)]
    pub const fn jcvszu(
        &self,
    ) -> &'static crate::common::Reg<self::Jcvszu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jcvszu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(7usize),
            )
        }
    }

    #[inline(always)]
    pub const fn jcvszd(
        &self,
    ) -> &'static crate::common::Reg<self::Jcvszd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jcvszd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn jchszu(
        &self,
    ) -> &'static crate::common::Reg<self::Jchszu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jchszu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(9usize),
            )
        }
    }

    #[inline(always)]
    pub const fn jchszd(
        &self,
    ) -> &'static crate::common::Reg<self::Jchszd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jchszd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[inline(always)]
    pub const fn jcdtcu(&self) -> &'static crate::common::Reg<self::Jcdtcu_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Jcdtcu_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(11usize),
            )
        }
    }

    #[inline(always)]
    pub const fn jcdtcm(&self) -> &'static crate::common::Reg<self::Jcdtcm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Jcdtcm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn jcdtcd(&self) -> &'static crate::common::Reg<self::Jcdtcd_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Jcdtcd_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(13usize),
            )
        }
    }

    #[inline(always)]
    pub const fn jinte0(
        &self,
    ) -> &'static crate::common::Reg<self::Jinte0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jinte0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[inline(always)]
    pub const fn jints0(
        &self,
    ) -> &'static crate::common::Reg<self::Jints0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jints0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(15usize),
            )
        }
    }

    #[inline(always)]
    pub const fn jcderr(
        &self,
    ) -> &'static crate::common::Reg<self::Jcderr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jcderr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn jcrst(&self) -> &'static crate::common::Reg<self::Jcrst_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Jcrst_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(17usize),
            )
        }
    }

    #[inline(always)]
    pub const fn jifecnt(
        &self,
    ) -> &'static crate::common::Reg<self::Jifecnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jifecnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn jifesa(
        &self,
    ) -> &'static crate::common::Reg<self::Jifesa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jifesa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[inline(always)]
    pub const fn jifesofst(
        &self,
    ) -> &'static crate::common::Reg<self::Jifesofst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jifesofst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[inline(always)]
    pub const fn jifeda(
        &self,
    ) -> &'static crate::common::Reg<self::Jifeda_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jifeda_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[inline(always)]
    pub const fn jifeslc(
        &self,
    ) -> &'static crate::common::Reg<self::Jifeslc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jifeslc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[inline(always)]
    pub const fn jifdcnt(
        &self,
    ) -> &'static crate::common::Reg<self::Jifdcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jifdcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[inline(always)]
    pub const fn jifdsa(
        &self,
    ) -> &'static crate::common::Reg<self::Jifdsa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jifdsa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[inline(always)]
    pub const fn jifddofst(
        &self,
    ) -> &'static crate::common::Reg<self::Jifddofst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jifddofst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[inline(always)]
    pub const fn jifdda(
        &self,
    ) -> &'static crate::common::Reg<self::Jifdda_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jifdda_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[inline(always)]
    pub const fn jifdsdc(
        &self,
    ) -> &'static crate::common::Reg<self::Jifdsdc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jifdsdc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[inline(always)]
    pub const fn jifddlc(
        &self,
    ) -> &'static crate::common::Reg<self::Jifddlc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jifddlc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[inline(always)]
    pub const fn jifdadt(
        &self,
    ) -> &'static crate::common::Reg<self::Jifdadt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jifdadt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[inline(always)]
    pub const fn jinte1(
        &self,
    ) -> &'static crate::common::Reg<self::Jinte1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jinte1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
            )
        }
    }

    #[inline(always)]
    pub const fn jints1(
        &self,
    ) -> &'static crate::common::Reg<self::Jints1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jints1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jcmod_SPEC;
impl crate::sealed::RegSpec for Jcmod_SPEC {
    type DataType = u8;
}

pub type Jcmod = crate::RegValueT<Jcmod_SPEC>;

impl Jcmod {
    #[inline(always)]
    pub fn dsp(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        jcmod::Dsp,
        jcmod::Dsp,
        Jcmod_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            jcmod::Dsp,
            jcmod::Dsp,
            Jcmod_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn redu(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        jcmod::Redu,
        jcmod::Redu,
        Jcmod_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            jcmod::Redu,
            jcmod::Redu,
            Jcmod_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Jcmod {
    #[inline(always)]
    fn default() -> Jcmod {
        <crate::RegValueT<Jcmod_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod jcmod {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dsp_SPEC;
    pub type Dsp = crate::EnumBitfieldStruct<u8, Dsp_SPEC>;
    impl Dsp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Redu_SPEC;
    pub type Redu = crate::EnumBitfieldStruct<u8, Redu_SPEC>;
    impl Redu {
        pub const _001: Self = Self::new(1);

        pub const _000: Self = Self::new(0);

        pub const _110: Self = Self::new(6);

        pub const _010: Self = Self::new(2);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jccmd_SPEC;
impl crate::sealed::RegSpec for Jccmd_SPEC {
    type DataType = u8;
}

pub type Jccmd = crate::RegValueT<Jccmd_SPEC>;

impl Jccmd {
    #[inline(always)]
    pub fn brst(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        jccmd::Brst,
        jccmd::Brst,
        Jccmd_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            jccmd::Brst,
            jccmd::Brst,
            Jccmd_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn jend(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        jccmd::Jend,
        jccmd::Jend,
        Jccmd_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            jccmd::Jend,
            jccmd::Jend,
            Jccmd_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn jrst(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        jccmd::Jrst,
        jccmd::Jrst,
        Jccmd_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            jccmd::Jrst,
            jccmd::Jrst,
            Jccmd_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn jsrt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        jccmd::Jsrt,
        jccmd::Jsrt,
        Jccmd_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            jccmd::Jsrt,
            jccmd::Jsrt,
            Jccmd_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Jccmd {
    #[inline(always)]
    fn default() -> Jccmd {
        <crate::RegValueT<Jccmd_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod jccmd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brst_SPEC;
    pub type Brst = crate::EnumBitfieldStruct<u8, Brst_SPEC>;
    impl Brst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Jend_SPEC;
    pub type Jend = crate::EnumBitfieldStruct<u8, Jend_SPEC>;
    impl Jend {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Jrst_SPEC;
    pub type Jrst = crate::EnumBitfieldStruct<u8, Jrst_SPEC>;
    impl Jrst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Jsrt_SPEC;
    pub type Jsrt = crate::EnumBitfieldStruct<u8, Jsrt_SPEC>;
    impl Jsrt {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jcqtn_SPEC;
impl crate::sealed::RegSpec for Jcqtn_SPEC {
    type DataType = u8;
}

pub type Jcqtn = crate::RegValueT<Jcqtn_SPEC>;

impl Jcqtn {
    #[inline(always)]
    pub fn qt3(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        jcqtn::Qt3,
        jcqtn::Qt3,
        Jcqtn_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            jcqtn::Qt3,
            jcqtn::Qt3,
            Jcqtn_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qt2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        jcqtn::Qt2,
        jcqtn::Qt2,
        Jcqtn_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            jcqtn::Qt2,
            jcqtn::Qt2,
            Jcqtn_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qt1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        jcqtn::Qt1,
        jcqtn::Qt1,
        Jcqtn_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            jcqtn::Qt1,
            jcqtn::Qt1,
            Jcqtn_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Jcqtn {
    #[inline(always)]
    fn default() -> Jcqtn {
        <crate::RegValueT<Jcqtn_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod jcqtn {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Qt3_SPEC;
    pub type Qt3 = crate::EnumBitfieldStruct<u8, Qt3_SPEC>;
    impl Qt3 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Qt2_SPEC;
    pub type Qt2 = crate::EnumBitfieldStruct<u8, Qt2_SPEC>;
    impl Qt2 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Qt1_SPEC;
    pub type Qt1 = crate::EnumBitfieldStruct<u8, Qt1_SPEC>;
    impl Qt1 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jchtn_SPEC;
impl crate::sealed::RegSpec for Jchtn_SPEC {
    type DataType = u8;
}

pub type Jchtn = crate::RegValueT<Jchtn_SPEC>;

impl Jchtn {
    #[inline(always)]
    pub fn hta3(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        jchtn::Hta3,
        jchtn::Hta3,
        Jchtn_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            jchtn::Hta3,
            jchtn::Hta3,
            Jchtn_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn htd3(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        jchtn::Htd3,
        jchtn::Htd3,
        Jchtn_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            jchtn::Htd3,
            jchtn::Htd3,
            Jchtn_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn hta2(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        jchtn::Hta2,
        jchtn::Hta2,
        Jchtn_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            jchtn::Hta2,
            jchtn::Hta2,
            Jchtn_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn htd2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        jchtn::Htd2,
        jchtn::Htd2,
        Jchtn_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            jchtn::Htd2,
            jchtn::Htd2,
            Jchtn_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn hta1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        jchtn::Hta1,
        jchtn::Hta1,
        Jchtn_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            jchtn::Hta1,
            jchtn::Hta1,
            Jchtn_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn htd1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        jchtn::Htd1,
        jchtn::Htd1,
        Jchtn_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            jchtn::Htd1,
            jchtn::Htd1,
            Jchtn_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Jchtn {
    #[inline(always)]
    fn default() -> Jchtn {
        <crate::RegValueT<Jchtn_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod jchtn {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hta3_SPEC;
    pub type Hta3 = crate::EnumBitfieldStruct<u8, Hta3_SPEC>;
    impl Hta3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Htd3_SPEC;
    pub type Htd3 = crate::EnumBitfieldStruct<u8, Htd3_SPEC>;
    impl Htd3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hta2_SPEC;
    pub type Hta2 = crate::EnumBitfieldStruct<u8, Hta2_SPEC>;
    impl Hta2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Htd2_SPEC;
    pub type Htd2 = crate::EnumBitfieldStruct<u8, Htd2_SPEC>;
    impl Htd2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hta1_SPEC;
    pub type Hta1 = crate::EnumBitfieldStruct<u8, Hta1_SPEC>;
    impl Hta1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Htd1_SPEC;
    pub type Htd1 = crate::EnumBitfieldStruct<u8, Htd1_SPEC>;
    impl Htd1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jcdriu_SPEC;
impl crate::sealed::RegSpec for Jcdriu_SPEC {
    type DataType = u8;
}

pub type Jcdriu = crate::RegValueT<Jcdriu_SPEC>;

impl Jcdriu {
    #[inline(always)]
    pub fn driu(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Jcdriu_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Jcdriu_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jcdriu {
    #[inline(always)]
    fn default() -> Jcdriu {
        <crate::RegValueT<Jcdriu_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jcdrid_SPEC;
impl crate::sealed::RegSpec for Jcdrid_SPEC {
    type DataType = u8;
}

pub type Jcdrid = crate::RegValueT<Jcdrid_SPEC>;

impl Jcdrid {
    #[inline(always)]
    pub fn drid(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Jcdrid_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Jcdrid_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jcdrid {
    #[inline(always)]
    fn default() -> Jcdrid {
        <crate::RegValueT<Jcdrid_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jcvszu_SPEC;
impl crate::sealed::RegSpec for Jcvszu_SPEC {
    type DataType = u8;
}

pub type Jcvszu = crate::RegValueT<Jcvszu_SPEC>;

impl Jcvszu {
    #[inline(always)]
    pub fn vszu(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Jcvszu_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Jcvszu_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jcvszu {
    #[inline(always)]
    fn default() -> Jcvszu {
        <crate::RegValueT<Jcvszu_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jcvszd_SPEC;
impl crate::sealed::RegSpec for Jcvszd_SPEC {
    type DataType = u8;
}

pub type Jcvszd = crate::RegValueT<Jcvszd_SPEC>;

impl Jcvszd {
    #[inline(always)]
    pub fn vszd(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Jcvszd_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Jcvszd_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jcvszd {
    #[inline(always)]
    fn default() -> Jcvszd {
        <crate::RegValueT<Jcvszd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jchszu_SPEC;
impl crate::sealed::RegSpec for Jchszu_SPEC {
    type DataType = u8;
}

pub type Jchszu = crate::RegValueT<Jchszu_SPEC>;

impl Jchszu {
    #[inline(always)]
    pub fn hszu(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Jchszu_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Jchszu_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jchszu {
    #[inline(always)]
    fn default() -> Jchszu {
        <crate::RegValueT<Jchszu_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jchszd_SPEC;
impl crate::sealed::RegSpec for Jchszd_SPEC {
    type DataType = u8;
}

pub type Jchszd = crate::RegValueT<Jchszd_SPEC>;

impl Jchszd {
    #[inline(always)]
    pub fn hszd(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Jchszd_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Jchszd_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jchszd {
    #[inline(always)]
    fn default() -> Jchszd {
        <crate::RegValueT<Jchszd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jcdtcu_SPEC;
impl crate::sealed::RegSpec for Jcdtcu_SPEC {
    type DataType = u8;
}

pub type Jcdtcu = crate::RegValueT<Jcdtcu_SPEC>;

impl Jcdtcu {
    #[inline(always)]
    pub fn dcu(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Jcdtcu_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Jcdtcu_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Jcdtcu {
    #[inline(always)]
    fn default() -> Jcdtcu {
        <crate::RegValueT<Jcdtcu_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jcdtcm_SPEC;
impl crate::sealed::RegSpec for Jcdtcm_SPEC {
    type DataType = u8;
}

pub type Jcdtcm = crate::RegValueT<Jcdtcm_SPEC>;

impl Jcdtcm {
    #[inline(always)]
    pub fn dcm(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Jcdtcm_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Jcdtcm_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Jcdtcm {
    #[inline(always)]
    fn default() -> Jcdtcm {
        <crate::RegValueT<Jcdtcm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jcdtcd_SPEC;
impl crate::sealed::RegSpec for Jcdtcd_SPEC {
    type DataType = u8;
}

pub type Jcdtcd = crate::RegValueT<Jcdtcd_SPEC>;

impl Jcdtcd {
    #[inline(always)]
    pub fn dcd(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Jcdtcd_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Jcdtcd_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Jcdtcd {
    #[inline(always)]
    fn default() -> Jcdtcd {
        <crate::RegValueT<Jcdtcd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jinte0_SPEC;
impl crate::sealed::RegSpec for Jinte0_SPEC {
    type DataType = u8;
}

pub type Jinte0 = crate::RegValueT<Jinte0_SPEC>;

impl Jinte0 {
    #[inline(always)]
    pub fn int7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        jinte0::Int7,
        jinte0::Int7,
        Jinte0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            jinte0::Int7,
            jinte0::Int7,
            Jinte0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn int6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        jinte0::Int6,
        jinte0::Int6,
        Jinte0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            jinte0::Int6,
            jinte0::Int6,
            Jinte0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn int5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        jinte0::Int5,
        jinte0::Int5,
        Jinte0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            jinte0::Int5,
            jinte0::Int5,
            Jinte0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn int3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        jinte0::Int3,
        jinte0::Int3,
        Jinte0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            jinte0::Int3,
            jinte0::Int3,
            Jinte0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Jinte0 {
    #[inline(always)]
    fn default() -> Jinte0 {
        <crate::RegValueT<Jinte0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod jinte0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Int7_SPEC;
    pub type Int7 = crate::EnumBitfieldStruct<u8, Int7_SPEC>;
    impl Int7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Int6_SPEC;
    pub type Int6 = crate::EnumBitfieldStruct<u8, Int6_SPEC>;
    impl Int6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Int5_SPEC;
    pub type Int5 = crate::EnumBitfieldStruct<u8, Int5_SPEC>;
    impl Int5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Int3_SPEC;
    pub type Int3 = crate::EnumBitfieldStruct<u8, Int3_SPEC>;
    impl Int3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jints0_SPEC;
impl crate::sealed::RegSpec for Jints0_SPEC {
    type DataType = u8;
}

pub type Jints0 = crate::RegValueT<Jints0_SPEC>;

impl Jints0 {
    #[inline(always)]
    pub fn ins6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Jints0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Jints0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn ins5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Jints0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Jints0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn ins3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Jints0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Jints0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Jints0 {
    #[inline(always)]
    fn default() -> Jints0 {
        <crate::RegValueT<Jints0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jcderr_SPEC;
impl crate::sealed::RegSpec for Jcderr_SPEC {
    type DataType = u8;
}

pub type Jcderr = crate::RegValueT<Jcderr_SPEC>;

impl Jcderr {
    #[inline(always)]
    pub fn err(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        jcderr::Err,
        jcderr::Err,
        Jcderr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            jcderr::Err,
            jcderr::Err,
            Jcderr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Jcderr {
    #[inline(always)]
    fn default() -> Jcderr {
        <crate::RegValueT<Jcderr_SPEC> as RegisterValue<_>>::new(10)
    }
}
pub mod jcderr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Err_SPEC;
    pub type Err = crate::EnumBitfieldStruct<u8, Err_SPEC>;
    impl Err {
        pub const _0000: Self = Self::new(0);

        pub const _0001: Self = Self::new(1);

        pub const _0010: Self = Self::new(2);

        pub const _0011: Self = Self::new(3);

        pub const _0100: Self = Self::new(4);

        pub const _0101: Self = Self::new(5);

        pub const _0110: Self = Self::new(6);

        pub const _0111: Self = Self::new(7);

        pub const _1000: Self = Self::new(8);

        pub const _1001: Self = Self::new(9);

        pub const _1010: Self = Self::new(10);

        pub const _1011: Self = Self::new(11);

        pub const _1100: Self = Self::new(12);

        pub const _1101: Self = Self::new(13);

        pub const _1110: Self = Self::new(14);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jcrst_SPEC;
impl crate::sealed::RegSpec for Jcrst_SPEC {
    type DataType = u8;
}

pub type Jcrst = crate::RegValueT<Jcrst_SPEC>;

impl Jcrst {
    #[inline(always)]
    pub fn rst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        jcrst::Rst,
        jcrst::Rst,
        Jcrst_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            jcrst::Rst,
            jcrst::Rst,
            Jcrst_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Jcrst {
    #[inline(always)]
    fn default() -> Jcrst {
        <crate::RegValueT<Jcrst_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod jcrst {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rst_SPEC;
    pub type Rst = crate::EnumBitfieldStruct<u8, Rst_SPEC>;
    impl Rst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jifecnt_SPEC;
impl crate::sealed::RegSpec for Jifecnt_SPEC {
    type DataType = u32;
}

pub type Jifecnt = crate::RegValueT<Jifecnt_SPEC>;

impl Jifecnt {
    #[inline(always)]
    pub fn joutswap(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        jifecnt::Joutswap,
        jifecnt::Joutswap,
        Jifecnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            jifecnt::Joutswap,
            jifecnt::Joutswap,
            Jifecnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dinrini(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        jifecnt::Dinrini,
        jifecnt::Dinrini,
        Jifecnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            jifecnt::Dinrini,
            jifecnt::Dinrini,
            Jifecnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dinrcmd(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Jifecnt_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Jifecnt_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn dinlc(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        jifecnt::Dinlc,
        jifecnt::Dinlc,
        Jifecnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            jifecnt::Dinlc,
            jifecnt::Dinlc,
            Jifecnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dinswap(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        jifecnt::Dinswap,
        jifecnt::Dinswap,
        Jifecnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            jifecnt::Dinswap,
            jifecnt::Dinswap,
            Jifecnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Jifecnt {
    #[inline(always)]
    fn default() -> Jifecnt {
        <crate::RegValueT<Jifecnt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod jifecnt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Joutswap_SPEC;
    pub type Joutswap = crate::EnumBitfieldStruct<u8, Joutswap_SPEC>;
    impl Joutswap {
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
    pub struct Dinrini_SPEC;
    pub type Dinrini = crate::EnumBitfieldStruct<u8, Dinrini_SPEC>;
    impl Dinrini {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dinlc_SPEC;
    pub type Dinlc = crate::EnumBitfieldStruct<u8, Dinlc_SPEC>;
    impl Dinlc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dinswap_SPEC;
    pub type Dinswap = crate::EnumBitfieldStruct<u8, Dinswap_SPEC>;
    impl Dinswap {
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
pub struct Jifesa_SPEC;
impl crate::sealed::RegSpec for Jifesa_SPEC {
    type DataType = u32;
}

pub type Jifesa = crate::RegValueT<Jifesa_SPEC>;

impl Jifesa {
    #[inline(always)]
    pub fn esa(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Jifesa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Jifesa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jifesa {
    #[inline(always)]
    fn default() -> Jifesa {
        <crate::RegValueT<Jifesa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jifesofst_SPEC;
impl crate::sealed::RegSpec for Jifesofst_SPEC {
    type DataType = u32;
}

pub type Jifesofst = crate::RegValueT<Jifesofst_SPEC>;

impl Jifesofst {
    #[inline(always)]
    pub fn esmw(
        self,
    ) -> crate::common::RegisterField<0, 0x7fff, 1, 0, u16, u16, Jifesofst_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7fff,1,0,u16,u16,Jifesofst_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jifesofst {
    #[inline(always)]
    fn default() -> Jifesofst {
        <crate::RegValueT<Jifesofst_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jifeda_SPEC;
impl crate::sealed::RegSpec for Jifeda_SPEC {
    type DataType = u32;
}

pub type Jifeda = crate::RegValueT<Jifeda_SPEC>;

impl Jifeda {
    #[inline(always)]
    pub fn eda(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Jifeda_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Jifeda_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jifeda {
    #[inline(always)]
    fn default() -> Jifeda {
        <crate::RegValueT<Jifeda_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jifeslc_SPEC;
impl crate::sealed::RegSpec for Jifeslc_SPEC {
    type DataType = u32;
}

pub type Jifeslc = crate::RegValueT<Jifeslc_SPEC>;

impl Jifeslc {
    #[inline(always)]
    pub fn lines(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Jifeslc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Jifeslc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jifeslc {
    #[inline(always)]
    fn default() -> Jifeslc {
        <crate::RegValueT<Jifeslc_SPEC> as RegisterValue<_>>::new(4294508536)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jifdcnt_SPEC;
impl crate::sealed::RegSpec for Jifdcnt_SPEC {
    type DataType = u32;
}

pub type Jifdcnt = crate::RegValueT<Jifdcnt_SPEC>;

impl Jifdcnt {
    #[inline(always)]
    pub fn vinter(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x3,
        1,
        0,
        jifdcnt::Vinter,
        jifdcnt::Vinter,
        Jifdcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x3,
            1,
            0,
            jifdcnt::Vinter,
            jifdcnt::Vinter,
            Jifdcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn hinter(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x3,
        1,
        0,
        jifdcnt::Hinter,
        jifdcnt::Hinter,
        Jifdcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x3,
            1,
            0,
            jifdcnt::Hinter,
            jifdcnt::Hinter,
            Jifdcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn opf(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x3,
        1,
        0,
        jifdcnt::Opf,
        jifdcnt::Opf,
        Jifdcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x3,
            1,
            0,
            jifdcnt::Opf,
            jifdcnt::Opf,
            Jifdcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn jinrini(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        jifdcnt::Jinrini,
        jifdcnt::Jinrini,
        Jifdcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            jifdcnt::Jinrini,
            jifdcnt::Jinrini,
            Jifdcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn jinrcmd(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Jifdcnt_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, Jifdcnt_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn jinc(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        jifdcnt::Jinc,
        jifdcnt::Jinc,
        Jifdcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            jifdcnt::Jinc,
            jifdcnt::Jinc,
            Jifdcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn jinswap(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        jifdcnt::Jinswap,
        jifdcnt::Jinswap,
        Jifdcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            jifdcnt::Jinswap,
            jifdcnt::Jinswap,
            Jifdcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn doutrini(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        jifdcnt::Doutrini,
        jifdcnt::Doutrini,
        Jifdcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            jifdcnt::Doutrini,
            jifdcnt::Doutrini,
            Jifdcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn doutrcmd(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Jifdcnt_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Jifdcnt_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn doutlc(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        jifdcnt::Doutlc,
        jifdcnt::Doutlc,
        Jifdcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            jifdcnt::Doutlc,
            jifdcnt::Doutlc,
            Jifdcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn doutswap(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        jifdcnt::Doutswap,
        jifdcnt::Doutswap,
        Jifdcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            jifdcnt::Doutswap,
            jifdcnt::Doutswap,
            Jifdcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Jifdcnt {
    #[inline(always)]
    fn default() -> Jifdcnt {
        <crate::RegValueT<Jifdcnt_SPEC> as RegisterValue<_>>::new(16777216)
    }
}
pub mod jifdcnt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vinter_SPEC;
    pub type Vinter = crate::EnumBitfieldStruct<u8, Vinter_SPEC>;
    impl Vinter {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hinter_SPEC;
    pub type Hinter = crate::EnumBitfieldStruct<u8, Hinter_SPEC>;
    impl Hinter {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Opf_SPEC;
    pub type Opf = crate::EnumBitfieldStruct<u8, Opf_SPEC>;
    impl Opf {
        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Jinrini_SPEC;
    pub type Jinrini = crate::EnumBitfieldStruct<u8, Jinrini_SPEC>;
    impl Jinrini {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Jinc_SPEC;
    pub type Jinc = crate::EnumBitfieldStruct<u8, Jinc_SPEC>;
    impl Jinc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Jinswap_SPEC;
    pub type Jinswap = crate::EnumBitfieldStruct<u8, Jinswap_SPEC>;
    impl Jinswap {
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
    pub struct Doutrini_SPEC;
    pub type Doutrini = crate::EnumBitfieldStruct<u8, Doutrini_SPEC>;
    impl Doutrini {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Doutlc_SPEC;
    pub type Doutlc = crate::EnumBitfieldStruct<u8, Doutlc_SPEC>;
    impl Doutlc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Doutswap_SPEC;
    pub type Doutswap = crate::EnumBitfieldStruct<u8, Doutswap_SPEC>;
    impl Doutswap {
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
pub struct Jifdsa_SPEC;
impl crate::sealed::RegSpec for Jifdsa_SPEC {
    type DataType = u32;
}

pub type Jifdsa = crate::RegValueT<Jifdsa_SPEC>;

impl Jifdsa {
    #[inline(always)]
    pub fn dsa(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Jifdsa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Jifdsa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jifdsa {
    #[inline(always)]
    fn default() -> Jifdsa {
        <crate::RegValueT<Jifdsa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jifddofst_SPEC;
impl crate::sealed::RegSpec for Jifddofst_SPEC {
    type DataType = u32;
}

pub type Jifddofst = crate::RegValueT<Jifddofst_SPEC>;

impl Jifddofst {
    #[inline(always)]
    pub fn ddmw(
        self,
    ) -> crate::common::RegisterField<0, 0x7fff, 1, 0, u16, u16, Jifddofst_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7fff,1,0,u16,u16,Jifddofst_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jifddofst {
    #[inline(always)]
    fn default() -> Jifddofst {
        <crate::RegValueT<Jifddofst_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jifdda_SPEC;
impl crate::sealed::RegSpec for Jifdda_SPEC {
    type DataType = u32;
}

pub type Jifdda = crate::RegValueT<Jifdda_SPEC>;

impl Jifdda {
    #[inline(always)]
    pub fn dda(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Jifdda_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Jifdda_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jifdda {
    #[inline(always)]
    fn default() -> Jifdda {
        <crate::RegValueT<Jifdda_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jifdsdc_SPEC;
impl crate::sealed::RegSpec for Jifdsdc_SPEC {
    type DataType = u32;
}

pub type Jifdsdc = crate::RegValueT<Jifdsdc_SPEC>;

impl Jifdsdc {
    #[inline(always)]
    pub fn jdatas(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Jifdsdc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Jifdsdc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jifdsdc {
    #[inline(always)]
    fn default() -> Jifdsdc {
        <crate::RegValueT<Jifdsdc_SPEC> as RegisterValue<_>>::new(4294508536)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jifddlc_SPEC;
impl crate::sealed::RegSpec for Jifddlc_SPEC {
    type DataType = u32;
}

pub type Jifddlc = crate::RegValueT<Jifddlc_SPEC>;

impl Jifddlc {
    #[inline(always)]
    pub fn lines(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Jifddlc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Jifddlc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jifddlc {
    #[inline(always)]
    fn default() -> Jifddlc {
        <crate::RegValueT<Jifddlc_SPEC> as RegisterValue<_>>::new(4294508536)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jifdadt_SPEC;
impl crate::sealed::RegSpec for Jifdadt_SPEC {
    type DataType = u32;
}

pub type Jifdadt = crate::RegValueT<Jifdadt_SPEC>;

impl Jifdadt {
    #[inline(always)]
    pub fn alpha(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Jifdadt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Jifdadt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jifdadt {
    #[inline(always)]
    fn default() -> Jifdadt {
        <crate::RegValueT<Jifdadt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jinte1_SPEC;
impl crate::sealed::RegSpec for Jinte1_SPEC {
    type DataType = u32;
}

pub type Jinte1 = crate::RegValueT<Jinte1_SPEC>;

impl Jinte1 {
    #[inline(always)]
    pub fn cbten(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        jinte1::Cbten,
        jinte1::Cbten,
        Jinte1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            jinte1::Cbten,
            jinte1::Cbten,
            Jinte1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dinlen(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        jinte1::Dinlen,
        jinte1::Dinlen,
        Jinte1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            jinte1::Dinlen,
            jinte1::Dinlen,
            Jinte1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dbten(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        jinte1::Dbten,
        jinte1::Dbten,
        Jinte1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            jinte1::Dbten,
            jinte1::Dbten,
            Jinte1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn jinen(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        jinte1::Jinen,
        jinte1::Jinen,
        Jinte1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            jinte1::Jinen,
            jinte1::Jinen,
            Jinte1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn doutlen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        jinte1::Doutlen,
        jinte1::Doutlen,
        Jinte1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            jinte1::Doutlen,
            jinte1::Doutlen,
            Jinte1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Jinte1 {
    #[inline(always)]
    fn default() -> Jinte1 {
        <crate::RegValueT<Jinte1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod jinte1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cbten_SPEC;
    pub type Cbten = crate::EnumBitfieldStruct<u8, Cbten_SPEC>;
    impl Cbten {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dinlen_SPEC;
    pub type Dinlen = crate::EnumBitfieldStruct<u8, Dinlen_SPEC>;
    impl Dinlen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dbten_SPEC;
    pub type Dbten = crate::EnumBitfieldStruct<u8, Dbten_SPEC>;
    impl Dbten {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Jinen_SPEC;
    pub type Jinen = crate::EnumBitfieldStruct<u8, Jinen_SPEC>;
    impl Jinen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Doutlen_SPEC;
    pub type Doutlen = crate::EnumBitfieldStruct<u8, Doutlen_SPEC>;
    impl Doutlen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jints1_SPEC;
impl crate::sealed::RegSpec for Jints1_SPEC {
    type DataType = u32;
}

pub type Jints1 = crate::RegValueT<Jints1_SPEC>;

impl Jints1 {
    #[inline(always)]
    pub fn cbtf(self) -> crate::common::RegisterFieldBool<6, 1, 0, Jints1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Jints1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn dinlf(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Jints1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Jints1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn dbtf(self) -> crate::common::RegisterFieldBool<2, 1, 0, Jints1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Jints1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn jinf(self) -> crate::common::RegisterFieldBool<1, 1, 0, Jints1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Jints1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn doutlf(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Jints1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Jints1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Jints1 {
    #[inline(always)]
    fn default() -> Jints1 {
        <crate::RegValueT<Jints1_SPEC> as RegisterValue<_>>::new(0)
    }
}
