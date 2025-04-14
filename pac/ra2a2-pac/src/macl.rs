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
// Generated from SVD 1.20.02, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:17:03 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"32-bit Multiply-Accumulator"]
unsafe impl ::core::marker::Send for super::Macl {}
unsafe impl ::core::marker::Sync for super::Macl {}
impl super::Macl {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn mulb(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mulb_SPEC, crate::common::RW>,
        24,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x0usize))
        }
    }

    #[inline(always)]
    pub const fn mulrl(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mulrl_SPEC, crate::common::RW>,
        24,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xc0usize))
        }
    }

    #[inline(always)]
    pub const fn mulrh(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mulrh_SPEC, crate::common::RW>,
        24,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xc4usize))
        }
    }

    #[inline(always)]
    pub const fn mul32u(
        &self,
    ) -> &'static crate::common::Reg<self::Mul32U_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mul32U_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(384usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mul32s(
        &self,
    ) -> &'static crate::common::Reg<self::Mul32S_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mul32S_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(388usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mac32u(
        &self,
    ) -> &'static crate::common::Reg<self::Mac32U_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mac32U_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(392usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mac32s(
        &self,
    ) -> &'static crate::common::Reg<self::Mac32S_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mac32S_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(396usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mulc(&self) -> &'static crate::common::Reg<self::Mulc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mulc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(400usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mulrclr(
        &self,
    ) -> &'static crate::common::Reg<self::Mulrclr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mulrclr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(404usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mulb_SPEC;
impl crate::sealed::RegSpec for Mulb_SPEC {
    type DataType = u32;
}

pub type Mulb = crate::RegValueT<Mulb_SPEC>;

impl NoBitfieldReg<Mulb_SPEC> for Mulb {}
impl ::core::default::Default for Mulb {
    #[inline(always)]
    fn default() -> Mulb {
        <crate::RegValueT<Mulb_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mulrl_SPEC;
impl crate::sealed::RegSpec for Mulrl_SPEC {
    type DataType = u32;
}

pub type Mulrl = crate::RegValueT<Mulrl_SPEC>;

impl NoBitfieldReg<Mulrl_SPEC> for Mulrl {}
impl ::core::default::Default for Mulrl {
    #[inline(always)]
    fn default() -> Mulrl {
        <crate::RegValueT<Mulrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mulrh_SPEC;
impl crate::sealed::RegSpec for Mulrh_SPEC {
    type DataType = u32;
}

pub type Mulrh = crate::RegValueT<Mulrh_SPEC>;

impl NoBitfieldReg<Mulrh_SPEC> for Mulrh {}
impl ::core::default::Default for Mulrh {
    #[inline(always)]
    fn default() -> Mulrh {
        <crate::RegValueT<Mulrh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mul32U_SPEC;
impl crate::sealed::RegSpec for Mul32U_SPEC {
    type DataType = u32;
}

pub type Mul32U = crate::RegValueT<Mul32U_SPEC>;

impl NoBitfieldReg<Mul32U_SPEC> for Mul32U {}
impl ::core::default::Default for Mul32U {
    #[inline(always)]
    fn default() -> Mul32U {
        <crate::RegValueT<Mul32U_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mul32S_SPEC;
impl crate::sealed::RegSpec for Mul32S_SPEC {
    type DataType = u32;
}

pub type Mul32S = crate::RegValueT<Mul32S_SPEC>;

impl NoBitfieldReg<Mul32S_SPEC> for Mul32S {}
impl ::core::default::Default for Mul32S {
    #[inline(always)]
    fn default() -> Mul32S {
        <crate::RegValueT<Mul32S_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mac32U_SPEC;
impl crate::sealed::RegSpec for Mac32U_SPEC {
    type DataType = u32;
}

pub type Mac32U = crate::RegValueT<Mac32U_SPEC>;

impl NoBitfieldReg<Mac32U_SPEC> for Mac32U {}
impl ::core::default::Default for Mac32U {
    #[inline(always)]
    fn default() -> Mac32U {
        <crate::RegValueT<Mac32U_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mac32S_SPEC;
impl crate::sealed::RegSpec for Mac32S_SPEC {
    type DataType = u32;
}

pub type Mac32S = crate::RegValueT<Mac32S_SPEC>;

impl NoBitfieldReg<Mac32S_SPEC> for Mac32S {}
impl ::core::default::Default for Mac32S {
    #[inline(always)]
    fn default() -> Mac32S {
        <crate::RegValueT<Mac32S_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mulc_SPEC;
impl crate::sealed::RegSpec for Mulc_SPEC {
    type DataType = u8;
}

pub type Mulc = crate::RegValueT<Mulc_SPEC>;

impl Mulc {
    #[inline(always)]
    pub fn mulst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mulc::Mulst,
        mulc::Mulst,
        Mulc_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mulc::Mulst,
            mulc::Mulst,
            Mulc_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn macsf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mulc::Macsf,
        mulc::Macsf,
        Mulc_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mulc::Macsf,
            mulc::Macsf,
            Mulc_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn macof(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mulc::Macof,
        mulc::Macof,
        Mulc_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mulc::Macof,
            mulc::Macof,
            Mulc_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mulfrac(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        mulc::Mulfrac,
        mulc::Mulfrac,
        Mulc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            mulc::Mulfrac,
            mulc::Mulfrac,
            Mulc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mulsm(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        mulc::Mulsm,
        mulc::Mulsm,
        Mulc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            mulc::Mulsm,
            mulc::Mulsm,
            Mulc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn macmode(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        mulc::Macmode,
        mulc::Macmode,
        Mulc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            mulc::Macmode,
            mulc::Macmode,
            Mulc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mulc {
    #[inline(always)]
    fn default() -> Mulc {
        <crate::RegValueT<Mulc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mulc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mulst_SPEC;
    pub type Mulst = crate::EnumBitfieldStruct<u8, Mulst_SPEC>;
    impl Mulst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Macsf_SPEC;
    pub type Macsf = crate::EnumBitfieldStruct<u8, Macsf_SPEC>;
    impl Macsf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Macof_SPEC;
    pub type Macof = crate::EnumBitfieldStruct<u8, Macof_SPEC>;
    impl Macof {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mulfrac_SPEC;
    pub type Mulfrac = crate::EnumBitfieldStruct<u8, Mulfrac_SPEC>;
    impl Mulfrac {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mulsm_SPEC;
    pub type Mulsm = crate::EnumBitfieldStruct<u8, Mulsm_SPEC>;
    impl Mulsm {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Macmode_SPEC;
    pub type Macmode = crate::EnumBitfieldStruct<u8, Macmode_SPEC>;
    impl Macmode {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mulrclr_SPEC;
impl crate::sealed::RegSpec for Mulrclr_SPEC {
    type DataType = u32;
}

pub type Mulrclr = crate::RegValueT<Mulrclr_SPEC>;

impl NoBitfieldReg<Mulrclr_SPEC> for Mulrclr {}
impl ::core::default::Default for Mulrclr {
    #[inline(always)]
    fn default() -> Mulrclr {
        <crate::RegValueT<Mulrclr_SPEC> as RegisterValue<_>>::new(0)
    }
}
