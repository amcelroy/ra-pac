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
// Generated from SVD 1.30.00, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:20:54 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"CACHE"]
unsafe impl ::core::marker::Send for super::Cache {}
unsafe impl ::core::marker::Sync for super::Cache {}
impl super::Cache {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn ccactl(
        &self,
    ) -> &'static crate::common::Reg<self::Ccactl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccactl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ccafct(
        &self,
    ) -> &'static crate::common::Reg<self::Ccafct_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccafct_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ccalcf(
        &self,
    ) -> &'static crate::common::Reg<self::Ccalcf_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccalcf_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn scactl(
        &self,
    ) -> &'static crate::common::Reg<self::Scactl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Scactl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn scafct(
        &self,
    ) -> &'static crate::common::Reg<self::Scafct_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Scafct_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[inline(always)]
    pub const fn scalcf(
        &self,
    ) -> &'static crate::common::Reg<self::Scalcf_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Scalcf_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[inline(always)]
    pub const fn capoad(
        &self,
    ) -> &'static crate::common::Reg<self::Capoad_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Capoad_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(512usize),
            )
        }
    }

    #[inline(always)]
    pub const fn caprcr(
        &self,
    ) -> &'static crate::common::Reg<self::Caprcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Caprcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(516usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccactl_SPEC;
impl crate::sealed::RegSpec for Ccactl_SPEC {
    type DataType = u32;
}

pub type Ccactl = crate::RegValueT<Ccactl_SPEC>;

impl Ccactl {
    #[inline(always)]
    pub fn enc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ccactl::Enc,
        ccactl::Enc,
        Ccactl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ccactl::Enc,
            ccactl::Enc,
            Ccactl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ccactl {
    #[inline(always)]
    fn default() -> Ccactl {
        <crate::RegValueT<Ccactl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ccactl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enc_SPEC;
    pub type Enc = crate::EnumBitfieldStruct<u8, Enc_SPEC>;
    impl Enc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccafct_SPEC;
impl crate::sealed::RegSpec for Ccafct_SPEC {
    type DataType = u32;
}

pub type Ccafct = crate::RegValueT<Ccafct_SPEC>;

impl Ccafct {
    #[inline(always)]
    pub fn fc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ccafct::Fc,
        ccafct::Fc,
        Ccafct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ccafct::Fc,
            ccafct::Fc,
            Ccafct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ccafct {
    #[inline(always)]
    fn default() -> Ccafct {
        <crate::RegValueT<Ccafct_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ccafct {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fc_SPEC;
    pub type Fc = crate::EnumBitfieldStruct<u8, Fc_SPEC>;
    impl Fc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccalcf_SPEC;
impl crate::sealed::RegSpec for Ccalcf_SPEC {
    type DataType = u32;
}

pub type Ccalcf = crate::RegValueT<Ccalcf_SPEC>;

impl Ccalcf {
    #[inline(always)]
    pub fn cc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        ccalcf::Cc,
        ccalcf::Cc,
        Ccalcf_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            ccalcf::Cc,
            ccalcf::Cc,
            Ccalcf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ccalcf {
    #[inline(always)]
    fn default() -> Ccalcf {
        <crate::RegValueT<Ccalcf_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod ccalcf {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cc_SPEC;
    pub type Cc = crate::EnumBitfieldStruct<u8, Cc_SPEC>;
    impl Cc {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scactl_SPEC;
impl crate::sealed::RegSpec for Scactl_SPEC {
    type DataType = u32;
}

pub type Scactl = crate::RegValueT<Scactl_SPEC>;

impl Scactl {
    #[inline(always)]
    pub fn ens(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        scactl::Ens,
        scactl::Ens,
        Scactl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            scactl::Ens,
            scactl::Ens,
            Scactl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Scactl {
    #[inline(always)]
    fn default() -> Scactl {
        <crate::RegValueT<Scactl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod scactl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ens_SPEC;
    pub type Ens = crate::EnumBitfieldStruct<u8, Ens_SPEC>;
    impl Ens {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scafct_SPEC;
impl crate::sealed::RegSpec for Scafct_SPEC {
    type DataType = u32;
}

pub type Scafct = crate::RegValueT<Scafct_SPEC>;

impl Scafct {
    #[inline(always)]
    pub fn fs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        scafct::Fs,
        scafct::Fs,
        Scafct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            scafct::Fs,
            scafct::Fs,
            Scafct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Scafct {
    #[inline(always)]
    fn default() -> Scafct {
        <crate::RegValueT<Scafct_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod scafct {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fs_SPEC;
    pub type Fs = crate::EnumBitfieldStruct<u8, Fs_SPEC>;
    impl Fs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scalcf_SPEC;
impl crate::sealed::RegSpec for Scalcf_SPEC {
    type DataType = u32;
}

pub type Scalcf = crate::RegValueT<Scalcf_SPEC>;

impl Scalcf {
    #[inline(always)]
    pub fn cs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        scalcf::Cs,
        scalcf::Cs,
        Scalcf_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            scalcf::Cs,
            scalcf::Cs,
            Scalcf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Scalcf {
    #[inline(always)]
    fn default() -> Scalcf {
        <crate::RegValueT<Scalcf_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod scalcf {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cs_SPEC;
    pub type Cs = crate::EnumBitfieldStruct<u8, Cs_SPEC>;
    impl Cs {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capoad_SPEC;
impl crate::sealed::RegSpec for Capoad_SPEC {
    type DataType = u32;
}

pub type Capoad = crate::RegValueT<Capoad_SPEC>;

impl Capoad {
    #[inline(always)]
    pub fn oad(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        capoad::Oad,
        capoad::Oad,
        Capoad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            capoad::Oad,
            capoad::Oad,
            Capoad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Capoad {
    #[inline(always)]
    fn default() -> Capoad {
        <crate::RegValueT<Capoad_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod capoad {

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
pub struct Caprcr_SPEC;
impl crate::sealed::RegSpec for Caprcr_SPEC {
    type DataType = u32;
}

pub type Caprcr = crate::RegValueT<Caprcr_SPEC>;

impl Caprcr {
    #[inline(always)]
    pub fn prcr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        caprcr::Prcr,
        caprcr::Prcr,
        Caprcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            caprcr::Prcr,
            caprcr::Prcr,
            Caprcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn kw(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Caprcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Caprcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Caprcr {
    #[inline(always)]
    fn default() -> Caprcr {
        <crate::RegValueT<Caprcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod caprcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prcr_SPEC;
    pub type Prcr = crate::EnumBitfieldStruct<u8, Prcr_SPEC>;
    impl Prcr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
