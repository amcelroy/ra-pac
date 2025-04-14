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
// Generated from SVD 1.40.00, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:21:24 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"IIR filter accelerator"]
unsafe impl ::core::marker::Send for super::Iirfa {}
unsafe impl ::core::marker::Sync for super::Iirfa {}
impl super::Iirfa {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn iircprcs(
        &self,
    ) -> &'static crate::common::Reg<self::Iircprcs_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iircprcs_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn iircprcff(
        &self,
    ) -> &'static crate::common::Reg<self::Iircprcff_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iircprcff_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn iirordyf(
        &self,
    ) -> &'static crate::common::Reg<self::Iirordyf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iirordyf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn iircerrf(
        &self,
    ) -> &'static crate::common::Reg<self::Iircerrf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iircerrf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn iiropcnt(
        &self,
    ) -> &'static crate::common::Reg<self::Iiropcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iiropcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn iirecccnt(
        &self,
    ) -> &'static crate::common::Reg<self::Iirecccnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iirecccnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn iireccint(
        &self,
    ) -> &'static crate::common::Reg<self::Iireccint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iireccint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn iireccef(
        &self,
    ) -> &'static crate::common::Reg<self::Iireccef_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iireccef_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn iireccefclr(
        &self,
    ) -> &'static crate::common::Reg<self::Iireccefclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Iireccefclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn iireseadr(
        &self,
    ) -> &'static crate::common::Reg<self::Iireseadr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iireseadr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[inline(always)]
    pub const fn iiredeadr(
        &self,
    ) -> &'static crate::common::Reg<self::Iiredeadr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Iiredeadr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[inline(always)]
    pub const fn iirchinp(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Iirchinp_SPEC, crate::common::W>,
        16,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x100usize))
        }
    }

    #[inline(always)]
    pub const fn iirchout(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Iirchout_SPEC, crate::common::R>,
        16,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x104usize))
        }
    }

    #[inline(always)]
    pub const fn iirchcnt(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Iirchcnt_SPEC, crate::common::RW>,
        16,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x108usize))
        }
    }

    #[inline(always)]
    pub const fn iirchint(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Iirchint_SPEC, crate::common::RW>,
        16,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x10cusize))
        }
    }

    #[inline(always)]
    pub const fn iirchsts(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Iirchsts_SPEC, crate::common::R>,
        16,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x10dusize))
        }
    }

    #[inline(always)]
    pub const fn iirchfclr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Iirchfclr_SPEC, crate::common::W>,
        16,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x10eusize))
        }
    }

    #[inline(always)]
    pub const fn iirstgb0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Iirstgb0_SPEC, crate::common::RW>,
        32,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x400usize))
        }
    }

    #[inline(always)]
    pub const fn iirstgb1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Iirstgb1_SPEC, crate::common::RW>,
        32,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x404usize))
        }
    }

    #[inline(always)]
    pub const fn iirstgb2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Iirstgb2_SPEC, crate::common::RW>,
        32,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x408usize))
        }
    }

    #[inline(always)]
    pub const fn iirstga1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Iirstga1_SPEC, crate::common::RW>,
        32,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x40cusize))
        }
    }

    #[inline(always)]
    pub const fn iirstga2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Iirstga2_SPEC, crate::common::RW>,
        32,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x410usize))
        }
    }

    #[inline(always)]
    pub const fn iirstgd0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Iirstgd0_SPEC, crate::common::RW>,
        32,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x414usize))
        }
    }

    #[inline(always)]
    pub const fn iirstgd1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Iirstgd1_SPEC, crate::common::RW>,
        32,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x418usize))
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iircprcs_SPEC;
impl crate::sealed::RegSpec for Iircprcs_SPEC {
    type DataType = u32;
}

pub type Iircprcs = crate::RegValueT<Iircprcs_SPEC>;

impl Iircprcs {
    #[inline(always)]
    pub fn cprcs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        iircprcs::Cprcs,
        iircprcs::Cprcs,
        Iircprcs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            iircprcs::Cprcs,
            iircprcs::Cprcs,
            Iircprcs_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iircprcs {
    #[inline(always)]
    fn default() -> Iircprcs {
        <crate::RegValueT<Iircprcs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iircprcs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cprcs_SPEC;
    pub type Cprcs = crate::EnumBitfieldStruct<u8, Cprcs_SPEC>;
    impl Cprcs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iircprcff_SPEC;
impl crate::sealed::RegSpec for Iircprcff_SPEC {
    type DataType = u32;
}

pub type Iircprcff = crate::RegValueT<Iircprcff_SPEC>;

impl Iircprcff {
    #[inline(always)]
    pub fn cprcff(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        iircprcff::Cprcff,
        iircprcff::Cprcff,
        Iircprcff_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            iircprcff::Cprcff,
            iircprcff::Cprcff,
            Iircprcff_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iircprcff {
    #[inline(always)]
    fn default() -> Iircprcff {
        <crate::RegValueT<Iircprcff_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iircprcff {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cprcff_SPEC;
    pub type Cprcff = crate::EnumBitfieldStruct<u8, Cprcff_SPEC>;
    impl Cprcff {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirordyf_SPEC;
impl crate::sealed::RegSpec for Iirordyf_SPEC {
    type DataType = u32;
}

pub type Iirordyf = crate::RegValueT<Iirordyf_SPEC>;

impl Iirordyf {
    #[inline(always)]
    pub fn ordyf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        iirordyf::Ordyf,
        iirordyf::Ordyf,
        Iirordyf_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            iirordyf::Ordyf,
            iirordyf::Ordyf,
            Iirordyf_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iirordyf {
    #[inline(always)]
    fn default() -> Iirordyf {
        <crate::RegValueT<Iirordyf_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iirordyf {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ordyf_SPEC;
    pub type Ordyf = crate::EnumBitfieldStruct<u8, Ordyf_SPEC>;
    impl Ordyf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iircerrf_SPEC;
impl crate::sealed::RegSpec for Iircerrf_SPEC {
    type DataType = u32;
}

pub type Iircerrf = crate::RegValueT<Iircerrf_SPEC>;

impl Iircerrf {
    #[inline(always)]
    pub fn cerrf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        iircerrf::Cerrf,
        iircerrf::Cerrf,
        Iircerrf_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            iircerrf::Cerrf,
            iircerrf::Cerrf,
            Iircerrf_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iircerrf {
    #[inline(always)]
    fn default() -> Iircerrf {
        <crate::RegValueT<Iircerrf_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iircerrf {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cerrf_SPEC;
    pub type Cerrf = crate::EnumBitfieldStruct<u8, Cerrf_SPEC>;
    impl Cerrf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iiropcnt_SPEC;
impl crate::sealed::RegSpec for Iiropcnt_SPEC {
    type DataType = u32;
}

pub type Iiropcnt = crate::RegValueT<Iiropcnt_SPEC>;

impl Iiropcnt {
    #[inline(always)]
    pub fn rnd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        iiropcnt::Rnd,
        iiropcnt::Rnd,
        Iiropcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            iiropcnt::Rnd,
            iiropcnt::Rnd,
            Iiropcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iiropcnt {
    #[inline(always)]
    fn default() -> Iiropcnt {
        <crate::RegValueT<Iiropcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iiropcnt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rnd_SPEC;
    pub type Rnd = crate::EnumBitfieldStruct<u8, Rnd_SPEC>;
    impl Rnd {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirecccnt_SPEC;
impl crate::sealed::RegSpec for Iirecccnt_SPEC {
    type DataType = u32;
}

pub type Iirecccnt = crate::RegValueT<Iirecccnt_SPEC>;

impl Iirecccnt {
    #[inline(always)]
    pub fn eccmd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        iirecccnt::Eccmd,
        iirecccnt::Eccmd,
        Iirecccnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            iirecccnt::Eccmd,
            iirecccnt::Eccmd,
            Iirecccnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn eccwbdis(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        iirecccnt::Eccwbdis,
        iirecccnt::Eccwbdis,
        Iirecccnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            iirecccnt::Eccwbdis,
            iirecccnt::Eccwbdis,
            Iirecccnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iirecccnt {
    #[inline(always)]
    fn default() -> Iirecccnt {
        <crate::RegValueT<Iirecccnt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iirecccnt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eccmd_SPEC;
    pub type Eccmd = crate::EnumBitfieldStruct<u8, Eccmd_SPEC>;
    impl Eccmd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eccwbdis_SPEC;
    pub type Eccwbdis = crate::EnumBitfieldStruct<u8, Eccwbdis_SPEC>;
    impl Eccwbdis {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iireccint_SPEC;
impl crate::sealed::RegSpec for Iireccint_SPEC {
    type DataType = u32;
}

pub type Iireccint = crate::RegValueT<Iireccint_SPEC>;

impl Iireccint {
    #[inline(always)]
    pub fn eseie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        iireccint::Eseie,
        iireccint::Eseie,
        Iireccint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            iireccint::Eseie,
            iireccint::Eseie,
            Iireccint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn edeie(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        iireccint::Edeie,
        iireccint::Edeie,
        Iireccint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            iireccint::Edeie,
            iireccint::Edeie,
            Iireccint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iireccint {
    #[inline(always)]
    fn default() -> Iireccint {
        <crate::RegValueT<Iireccint_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iireccint {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eseie_SPEC;
    pub type Eseie = crate::EnumBitfieldStruct<u8, Eseie_SPEC>;
    impl Eseie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Edeie_SPEC;
    pub type Edeie = crate::EnumBitfieldStruct<u8, Edeie_SPEC>;
    impl Edeie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iireccef_SPEC;
impl crate::sealed::RegSpec for Iireccef_SPEC {
    type DataType = u32;
}

pub type Iireccef = crate::RegValueT<Iireccef_SPEC>;

impl Iireccef {
    #[inline(always)]
    pub fn esef(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        iireccef::Esef,
        iireccef::Esef,
        Iireccef_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            iireccef::Esef,
            iireccef::Esef,
            Iireccef_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn edef(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        iireccef::Edef,
        iireccef::Edef,
        Iireccef_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            iireccef::Edef,
            iireccef::Edef,
            Iireccef_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iireccef {
    #[inline(always)]
    fn default() -> Iireccef {
        <crate::RegValueT<Iireccef_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iireccef {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Esef_SPEC;
    pub type Esef = crate::EnumBitfieldStruct<u8, Esef_SPEC>;
    impl Esef {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Edef_SPEC;
    pub type Edef = crate::EnumBitfieldStruct<u8, Edef_SPEC>;
    impl Edef {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iireccefclr_SPEC;
impl crate::sealed::RegSpec for Iireccefclr_SPEC {
    type DataType = u32;
}

pub type Iireccefclr = crate::RegValueT<Iireccefclr_SPEC>;

impl Iireccefclr {
    #[inline(always)]
    pub fn esefclr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        iireccefclr::Esefclr,
        iireccefclr::Esefclr,
        Iireccefclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            iireccefclr::Esefclr,
            iireccefclr::Esefclr,
            Iireccefclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn edefclr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        iireccefclr::Edefclr,
        iireccefclr::Edefclr,
        Iireccefclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            iireccefclr::Edefclr,
            iireccefclr::Edefclr,
            Iireccefclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iireccefclr {
    #[inline(always)]
    fn default() -> Iireccefclr {
        <crate::RegValueT<Iireccefclr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iireccefclr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Esefclr_SPEC;
    pub type Esefclr = crate::EnumBitfieldStruct<u8, Esefclr_SPEC>;
    impl Esefclr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Edefclr_SPEC;
    pub type Edefclr = crate::EnumBitfieldStruct<u8, Edefclr_SPEC>;
    impl Edefclr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iireseadr_SPEC;
impl crate::sealed::RegSpec for Iireseadr_SPEC {
    type DataType = u32;
}

pub type Iireseadr = crate::RegValueT<Iireseadr_SPEC>;

impl Iireseadr {
    #[inline(always)]
    pub fn seadr(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, Iireseadr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,Iireseadr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Iireseadr {
    #[inline(always)]
    fn default() -> Iireseadr {
        <crate::RegValueT<Iireseadr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iiredeadr_SPEC;
impl crate::sealed::RegSpec for Iiredeadr_SPEC {
    type DataType = u32;
}

pub type Iiredeadr = crate::RegValueT<Iiredeadr_SPEC>;

impl Iiredeadr {
    #[inline(always)]
    pub fn deadr(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, Iiredeadr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,Iiredeadr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Iiredeadr {
    #[inline(always)]
    fn default() -> Iiredeadr {
        <crate::RegValueT<Iiredeadr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirchinp_SPEC;
impl crate::sealed::RegSpec for Iirchinp_SPEC {
    type DataType = u32;
}

pub type Iirchinp = crate::RegValueT<Iirchinp_SPEC>;

impl NoBitfieldReg<Iirchinp_SPEC> for Iirchinp {}
impl ::core::default::Default for Iirchinp {
    #[inline(always)]
    fn default() -> Iirchinp {
        <crate::RegValueT<Iirchinp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirchout_SPEC;
impl crate::sealed::RegSpec for Iirchout_SPEC {
    type DataType = u32;
}

pub type Iirchout = crate::RegValueT<Iirchout_SPEC>;

impl NoBitfieldReg<Iirchout_SPEC> for Iirchout {}
impl ::core::default::Default for Iirchout {
    #[inline(always)]
    fn default() -> Iirchout {
        <crate::RegValueT<Iirchout_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirchcnt_SPEC;
impl crate::sealed::RegSpec for Iirchcnt_SPEC {
    type DataType = u32;
}

pub type Iirchcnt = crate::RegValueT<Iirchcnt_SPEC>;

impl Iirchcnt {
    #[inline(always)]
    pub fn stgsel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        iirchcnt::Stgsel,
        iirchcnt::Stgsel,
        Iirchcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            iirchcnt::Stgsel,
            iirchcnt::Stgsel,
            Iirchcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iirchcnt {
    #[inline(always)]
    fn default() -> Iirchcnt {
        <crate::RegValueT<Iirchcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iirchcnt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stgsel_SPEC;
    pub type Stgsel = crate::EnumBitfieldStruct<u8, Stgsel_SPEC>;
    impl Stgsel {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirchint_SPEC;
impl crate::sealed::RegSpec for Iirchint_SPEC {
    type DataType = u8;
}

pub type Iirchint = crate::RegValueT<Iirchint_SPEC>;

impl Iirchint {
    #[inline(always)]
    pub fn cprcfie(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        iirchint::Cprcfie,
        iirchint::Cprcfie,
        Iirchint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            iirchint::Cprcfie,
            iirchint::Cprcfie,
            Iirchint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ordyie(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        iirchint::Ordyie,
        iirchint::Ordyie,
        Iirchint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            iirchint::Ordyie,
            iirchint::Ordyie,
            Iirchint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cerrie(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        iirchint::Cerrie,
        iirchint::Cerrie,
        Iirchint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            iirchint::Cerrie,
            iirchint::Cerrie,
            Iirchint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iirchint {
    #[inline(always)]
    fn default() -> Iirchint {
        <crate::RegValueT<Iirchint_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iirchint {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cprcfie_SPEC;
    pub type Cprcfie = crate::EnumBitfieldStruct<u8, Cprcfie_SPEC>;
    impl Cprcfie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ordyie_SPEC;
    pub type Ordyie = crate::EnumBitfieldStruct<u8, Ordyie_SPEC>;
    impl Ordyie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cerrie_SPEC;
    pub type Cerrie = crate::EnumBitfieldStruct<u8, Cerrie_SPEC>;
    impl Cerrie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirchsts_SPEC;
impl crate::sealed::RegSpec for Iirchsts_SPEC {
    type DataType = u8;
}

pub type Iirchsts = crate::RegValueT<Iirchsts_SPEC>;

impl Iirchsts {
    #[inline(always)]
    pub fn cprcs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        iirchsts::Cprcs,
        iirchsts::Cprcs,
        Iirchsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            iirchsts::Cprcs,
            iirchsts::Cprcs,
            Iirchsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cprcff(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        iirchsts::Cprcff,
        iirchsts::Cprcff,
        Iirchsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            iirchsts::Cprcff,
            iirchsts::Cprcff,
            Iirchsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ordyf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        iirchsts::Ordyf,
        iirchsts::Ordyf,
        Iirchsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            iirchsts::Ordyf,
            iirchsts::Ordyf,
            Iirchsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cerrf(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        iirchsts::Cerrf,
        iirchsts::Cerrf,
        Iirchsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            iirchsts::Cerrf,
            iirchsts::Cerrf,
            Iirchsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iirchsts {
    #[inline(always)]
    fn default() -> Iirchsts {
        <crate::RegValueT<Iirchsts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iirchsts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cprcs_SPEC;
    pub type Cprcs = crate::EnumBitfieldStruct<u8, Cprcs_SPEC>;
    impl Cprcs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cprcff_SPEC;
    pub type Cprcff = crate::EnumBitfieldStruct<u8, Cprcff_SPEC>;
    impl Cprcff {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ordyf_SPEC;
    pub type Ordyf = crate::EnumBitfieldStruct<u8, Ordyf_SPEC>;
    impl Ordyf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cerrf_SPEC;
    pub type Cerrf = crate::EnumBitfieldStruct<u8, Cerrf_SPEC>;
    impl Cerrf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirchfclr_SPEC;
impl crate::sealed::RegSpec for Iirchfclr_SPEC {
    type DataType = u8;
}

pub type Iirchfclr = crate::RegValueT<Iirchfclr_SPEC>;

impl Iirchfclr {
    #[inline(always)]
    pub fn cprcffclr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        iirchfclr::Cprcffclr,
        iirchfclr::Cprcffclr,
        Iirchfclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            iirchfclr::Cprcffclr,
            iirchfclr::Cprcffclr,
            Iirchfclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cerrfclr(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        iirchfclr::Cerrfclr,
        iirchfclr::Cerrfclr,
        Iirchfclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            iirchfclr::Cerrfclr,
            iirchfclr::Cerrfclr,
            Iirchfclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iirchfclr {
    #[inline(always)]
    fn default() -> Iirchfclr {
        <crate::RegValueT<Iirchfclr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iirchfclr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cprcffclr_SPEC;
    pub type Cprcffclr = crate::EnumBitfieldStruct<u8, Cprcffclr_SPEC>;
    impl Cprcffclr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cerrfclr_SPEC;
    pub type Cerrfclr = crate::EnumBitfieldStruct<u8, Cerrfclr_SPEC>;
    impl Cerrfclr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirstgb0_SPEC;
impl crate::sealed::RegSpec for Iirstgb0_SPEC {
    type DataType = u32;
}

pub type Iirstgb0 = crate::RegValueT<Iirstgb0_SPEC>;

impl NoBitfieldReg<Iirstgb0_SPEC> for Iirstgb0 {}
impl ::core::default::Default for Iirstgb0 {
    #[inline(always)]
    fn default() -> Iirstgb0 {
        <crate::RegValueT<Iirstgb0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirstgb1_SPEC;
impl crate::sealed::RegSpec for Iirstgb1_SPEC {
    type DataType = u32;
}

pub type Iirstgb1 = crate::RegValueT<Iirstgb1_SPEC>;

impl NoBitfieldReg<Iirstgb1_SPEC> for Iirstgb1 {}
impl ::core::default::Default for Iirstgb1 {
    #[inline(always)]
    fn default() -> Iirstgb1 {
        <crate::RegValueT<Iirstgb1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirstgb2_SPEC;
impl crate::sealed::RegSpec for Iirstgb2_SPEC {
    type DataType = u32;
}

pub type Iirstgb2 = crate::RegValueT<Iirstgb2_SPEC>;

impl NoBitfieldReg<Iirstgb2_SPEC> for Iirstgb2 {}
impl ::core::default::Default for Iirstgb2 {
    #[inline(always)]
    fn default() -> Iirstgb2 {
        <crate::RegValueT<Iirstgb2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirstga1_SPEC;
impl crate::sealed::RegSpec for Iirstga1_SPEC {
    type DataType = u32;
}

pub type Iirstga1 = crate::RegValueT<Iirstga1_SPEC>;

impl NoBitfieldReg<Iirstga1_SPEC> for Iirstga1 {}
impl ::core::default::Default for Iirstga1 {
    #[inline(always)]
    fn default() -> Iirstga1 {
        <crate::RegValueT<Iirstga1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirstga2_SPEC;
impl crate::sealed::RegSpec for Iirstga2_SPEC {
    type DataType = u32;
}

pub type Iirstga2 = crate::RegValueT<Iirstga2_SPEC>;

impl NoBitfieldReg<Iirstga2_SPEC> for Iirstga2 {}
impl ::core::default::Default for Iirstga2 {
    #[inline(always)]
    fn default() -> Iirstga2 {
        <crate::RegValueT<Iirstga2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirstgd0_SPEC;
impl crate::sealed::RegSpec for Iirstgd0_SPEC {
    type DataType = u32;
}

pub type Iirstgd0 = crate::RegValueT<Iirstgd0_SPEC>;

impl NoBitfieldReg<Iirstgd0_SPEC> for Iirstgd0 {}
impl ::core::default::Default for Iirstgd0 {
    #[inline(always)]
    fn default() -> Iirstgd0 {
        <crate::RegValueT<Iirstgd0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirstgd1_SPEC;
impl crate::sealed::RegSpec for Iirstgd1_SPEC {
    type DataType = u32;
}

pub type Iirstgd1 = crate::RegValueT<Iirstgd1_SPEC>;

impl NoBitfieldReg<Iirstgd1_SPEC> for Iirstgd1 {}
impl ::core::default::Default for Iirstgd1 {
    #[inline(always)]
    fn default() -> Iirstgd1 {
        <crate::RegValueT<Iirstgd1_SPEC> as RegisterValue<_>>::new(0)
    }
}
