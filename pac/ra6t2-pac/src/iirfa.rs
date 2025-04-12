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
// Generated from SVD 1.40.00, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:18:35 +0000

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
    #[doc = "Channel Processing Status Register"]
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

    #[doc = "Channel Processing Completion Flag Register"]
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

    #[doc = "Output Data Preparation Completion Flag Register"]
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

    #[doc = "Operation Error Flag Register"]
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

    #[doc = "Operation Control Register"]
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

    #[doc = "ECC Control Register"]
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

    #[doc = "ECC Interrupt Enable Register"]
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

    #[doc = "ECC Error Flag Register"]
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

    #[doc = "ECC Error Flag Clear Register"]
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

    #[doc = "ECC 1-bit Error Address Register"]
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

    #[doc = "ECC 2-bit Error Address Register"]
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

    #[doc = "Channel %s Input Register"]
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

    #[doc = "Channel %s Output Register"]
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

    #[doc = "Channel %s Control Register"]
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

    #[doc = "Channel %s Interrupt Enable Register"]
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

    #[doc = "Channel %s Status Register"]
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

    #[doc = "Channel %s Flag Clear Register"]
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

    #[doc = "Stage %s Coefficient b0 Register"]
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

    #[doc = "Stage %s Coefficient b1 Register"]
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

    #[doc = "Stage %s Coefficient b2 Register"]
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

    #[doc = "Stage %s Coefficient a1 Register"]
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

    #[doc = "Stage %s Coefficient a2 Register"]
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

    #[doc = "Stage %s Delay Data D0 Register"]
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

    #[doc = "Stage %s Delay Data D1 Register"]
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
#[doc = "Channel Processing Status Register"]
pub type Iircprcs = crate::RegValueT<Iircprcs_SPEC>;

impl Iircprcs {
    #[doc = "Channel processing status bit"]
    #[inline(always)]
    pub fn cprcs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
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
        #[doc = "The channel processing of the corresponding channel is not being performed."]
        pub const _0: Self = Self::new(0);
        #[doc = "The channel processing of the corresponding channel is being performed."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iircprcff_SPEC;
impl crate::sealed::RegSpec for Iircprcff_SPEC {
    type DataType = u32;
}
#[doc = "Channel Processing Completion Flag Register"]
pub type Iircprcff = crate::RegValueT<Iircprcff_SPEC>;

impl Iircprcff {
    #[doc = "Channel processing completion flag"]
    #[inline(always)]
    pub fn cprcff(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
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
        #[doc = "The channel processing of the corresponding channel is not completed."]
        pub const _0: Self = Self::new(0);
        #[doc = "The channel processing of the corresponding channel is completed."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirordyf_SPEC;
impl crate::sealed::RegSpec for Iirordyf_SPEC {
    type DataType = u32;
}
#[doc = "Output Data Preparation Completion Flag Register"]
pub type Iirordyf = crate::RegValueT<Iirordyf_SPEC>;

impl Iirordyf {
    #[doc = "Output data preparation completion flag"]
    #[inline(always)]
    pub fn ordyf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
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
        #[doc = "The output data preparation of the corresponding channel is not completed."]
        pub const _0: Self = Self::new(0);
        #[doc = "The output data preparation of the corresponding channel is completed."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iircerrf_SPEC;
impl crate::sealed::RegSpec for Iircerrf_SPEC {
    type DataType = u32;
}
#[doc = "Operation Error Flag Register"]
pub type Iircerrf = crate::RegValueT<Iircerrf_SPEC>;

impl Iircerrf {
    #[doc = "Operation error flag"]
    #[inline(always)]
    pub fn cerrf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
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
        #[doc = "No operation error has occurred in the corresponding channel."]
        pub const _0: Self = Self::new(0);
        #[doc = "An operation error has occurred in the corresponding channel."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iiropcnt_SPEC;
impl crate::sealed::RegSpec for Iiropcnt_SPEC {
    type DataType = u32;
}
#[doc = "Operation Control Register"]
pub type Iiropcnt = crate::RegValueT<Iiropcnt_SPEC>;

impl Iiropcnt {
    #[doc = "Setting for the rounding mode for addition and multiplication"]
    #[inline(always)]
    pub fn rnd(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, iiropcnt::Rnd, Iiropcnt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,iiropcnt::Rnd, Iiropcnt_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Round to nearest"]
        pub const _000: Self = Self::new(0);
        #[doc = "Round toward zero"]
        pub const _001: Self = Self::new(1);
        #[doc = "Setting prohibited."]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirecccnt_SPEC;
impl crate::sealed::RegSpec for Iirecccnt_SPEC {
    type DataType = u32;
}
#[doc = "ECC Control Register"]
pub type Iirecccnt = crate::RegValueT<Iirecccnt_SPEC>;

impl Iirecccnt {
    #[doc = "ECC setting bit"]
    #[inline(always)]
    pub fn eccmd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
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
            Iirecccnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ECC-corrected data write-back disable bit"]
    #[inline(always)]
    pub fn eccwbdis(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
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
        #[doc = "The ECC error detection/correction function is disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "The ECC error detection/correction function is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eccwbdis_SPEC;
    pub type Eccwbdis = crate::EnumBitfieldStruct<u8, Eccwbdis_SPEC>;
    impl Eccwbdis {
        #[doc = "The error-corrected data write-back is enabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "The error-corrected data write-back is disabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iireccint_SPEC;
impl crate::sealed::RegSpec for Iireccint_SPEC {
    type DataType = u32;
}
#[doc = "ECC Interrupt Enable Register"]
pub type Iireccint = crate::RegValueT<Iireccint_SPEC>;

impl Iireccint {
    #[doc = "ECC 1-bit error interrupt enable bit"]
    #[inline(always)]
    pub fn eseie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
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
            Iireccint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ECC 2-bit error interrupt enable bit"]
    #[inline(always)]
    pub fn edeie(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
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
        #[doc = "The generation of ECC 1-bit error interrupt requests is disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "The generation of ECC 1-bit error interrupt requests is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Edeie_SPEC;
    pub type Edeie = crate::EnumBitfieldStruct<u8, Edeie_SPEC>;
    impl Edeie {
        #[doc = "The generation of ECC 2-bit error interrupt requests is disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "The generation of ECC 2-bit error interrupt requests is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iireccef_SPEC;
impl crate::sealed::RegSpec for Iireccef_SPEC {
    type DataType = u32;
}
#[doc = "ECC Error Flag Register"]
pub type Iireccef = crate::RegValueT<Iireccef_SPEC>;

impl Iireccef {
    #[doc = "ECC 1-bit error flag"]
    #[inline(always)]
    pub fn esef(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, iireccef::Esef, Iireccef_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,iireccef::Esef, Iireccef_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ECC 2-bit error flag"]
    #[inline(always)]
    pub fn edef(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, iireccef::Edef, Iireccef_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,iireccef::Edef, Iireccef_SPEC,crate::common::R>::from_register(self,0)
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
        #[doc = "No 1-bit ECC error is detected."]
        pub const _0: Self = Self::new(0);
        #[doc = "1-bit ECC error is detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Edef_SPEC;
    pub type Edef = crate::EnumBitfieldStruct<u8, Edef_SPEC>;
    impl Edef {
        #[doc = "No 2-bit ECC error is detected."]
        pub const _0: Self = Self::new(0);
        #[doc = "2-bit ECC error is detected."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iireccefclr_SPEC;
impl crate::sealed::RegSpec for Iireccefclr_SPEC {
    type DataType = u32;
}
#[doc = "ECC Error Flag Clear Register"]
pub type Iireccefclr = crate::RegValueT<Iireccefclr_SPEC>;

impl Iireccefclr {
    #[doc = "ECC 1-bit error flag clear bit"]
    #[inline(always)]
    pub fn esefclr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
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
            Iireccefclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "ECC 2-bit error status flag clear bit"]
    #[inline(always)]
    pub fn edefclr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
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
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);
        #[doc = "Clears the ESEF flag of the IIRECCEF register."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Edefclr_SPEC;
    pub type Edefclr = crate::EnumBitfieldStruct<u8, Edefclr_SPEC>;
    impl Edefclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);
        #[doc = "Clears the EDEF flag of the IIRECCEF register."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iireseadr_SPEC;
impl crate::sealed::RegSpec for Iireseadr_SPEC {
    type DataType = u32;
}
#[doc = "ECC 1-bit Error Address Register"]
pub type Iireseadr = crate::RegValueT<Iireseadr_SPEC>;

impl Iireseadr {
    #[doc = "Error address"]
    #[inline(always)]
    pub fn seadr(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, Iireseadr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16, Iireseadr_SPEC,crate::common::R>::from_register(self,0)
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
#[doc = "ECC 2-bit Error Address Register"]
pub type Iiredeadr = crate::RegValueT<Iiredeadr_SPEC>;

impl Iiredeadr {
    #[doc = "Error address"]
    #[inline(always)]
    pub fn deadr(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, Iiredeadr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16, Iiredeadr_SPEC,crate::common::R>::from_register(self,0)
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
#[doc = "Channel %s Input Register"]
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
#[doc = "Channel %s Output Register"]
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
#[doc = "Channel %s Control Register"]
pub type Iirchcnt = crate::RegValueT<Iirchcnt_SPEC>;

impl Iirchcnt {
    #[doc = "Stage selection bit"]
    #[inline(always)]
    pub fn stgsel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
        #[doc = "The corresponding stage is not used for channel n."]
        pub const _0: Self = Self::new(0);
        #[doc = "The corresponding stage is used for channel n."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirchint_SPEC;
impl crate::sealed::RegSpec for Iirchint_SPEC {
    type DataType = u8;
}
#[doc = "Channel %s Interrupt Enable Register"]
pub type Iirchint = crate::RegValueT<Iirchint_SPEC>;

impl Iirchint {
    #[doc = "Channel processing completion interrupt enable bit"]
    #[inline(always)]
    pub fn cprcfie(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
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
            Iirchint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Output data preparation completion interrupt enable bit"]
    #[inline(always)]
    pub fn ordyie(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
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
            Iirchint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Operation error interrupt enable bit"]
    #[inline(always)]
    pub fn cerrie(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
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
        #[doc = "The generation of channel processing completion interrupt requests is disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "The generation of channel processing completion interrupt requests is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ordyie_SPEC;
    pub type Ordyie = crate::EnumBitfieldStruct<u8, Ordyie_SPEC>;
    impl Ordyie {
        #[doc = "The generation of output data preparation completion interrupt requests is disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "The generation of output data preparation completion interrupt requests is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cerrie_SPEC;
    pub type Cerrie = crate::EnumBitfieldStruct<u8, Cerrie_SPEC>;
    impl Cerrie {
        #[doc = "The generation of operation error interrupt requests is disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "The generation of operation error interrupt requests is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirchsts_SPEC;
impl crate::sealed::RegSpec for Iirchsts_SPEC {
    type DataType = u8;
}
#[doc = "Channel %s Status Register"]
pub type Iirchsts = crate::RegValueT<Iirchsts_SPEC>;

impl Iirchsts {
    #[doc = "Channel processing status flag"]
    #[inline(always)]
    pub fn cprcs(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, iirchsts::Cprcs, Iirchsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,iirchsts::Cprcs, Iirchsts_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Channel processing completion flag"]
    #[inline(always)]
    pub fn cprcff(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, iirchsts::Cprcff, Iirchsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            iirchsts::Cprcff,
            Iirchsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Output data preparation completion flag"]
    #[inline(always)]
    pub fn ordyf(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, iirchsts::Ordyf, Iirchsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x1,1,0,iirchsts::Ordyf, Iirchsts_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Operation error flag"]
    #[inline(always)]
    pub fn cerrf(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, iirchsts::Cerrf, Iirchsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<3,0x1,1,0,iirchsts::Cerrf, Iirchsts_SPEC,crate::common::R>::from_register(self,0)
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
        #[doc = "The channel processing is not being performed."]
        pub const _0: Self = Self::new(0);
        #[doc = "The channel processing is being performed."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cprcff_SPEC;
    pub type Cprcff = crate::EnumBitfieldStruct<u8, Cprcff_SPEC>;
    impl Cprcff {
        #[doc = "The channel processing is not completed."]
        pub const _0: Self = Self::new(0);
        #[doc = "The channel processing is completed."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ordyf_SPEC;
    pub type Ordyf = crate::EnumBitfieldStruct<u8, Ordyf_SPEC>;
    impl Ordyf {
        #[doc = "The output data preparation is not completed."]
        pub const _0: Self = Self::new(0);
        #[doc = "The output data preparation is completed."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cerrf_SPEC;
    pub type Cerrf = crate::EnumBitfieldStruct<u8, Cerrf_SPEC>;
    impl Cerrf {
        #[doc = "No operation error has occurred."]
        pub const _0: Self = Self::new(0);
        #[doc = "An operation error has occurred."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirchfclr_SPEC;
impl crate::sealed::RegSpec for Iirchfclr_SPEC {
    type DataType = u8;
}
#[doc = "Channel %s Flag Clear Register"]
pub type Iirchfclr = crate::RegValueT<Iirchfclr_SPEC>;

impl Iirchfclr {
    #[doc = "Channel processing completion flag clear bit"]
    #[inline(always)]
    pub fn cprcffclr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
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
            Iirchfclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Operation error flag clear bit"]
    #[inline(always)]
    pub fn cerrfclr(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
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
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);
        #[doc = "Clears the CPRCFF flag of the IIRCHnSTS register."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cerrfclr_SPEC;
    pub type Cerrfclr = crate::EnumBitfieldStruct<u8, Cerrfclr_SPEC>;
    impl Cerrfclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);
        #[doc = "Clears the CERRF flag of the IIRCHnSTS register."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iirstgb0_SPEC;
impl crate::sealed::RegSpec for Iirstgb0_SPEC {
    type DataType = u32;
}
#[doc = "Stage %s Coefficient b0 Register"]
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
#[doc = "Stage %s Coefficient b1 Register"]
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
#[doc = "Stage %s Coefficient b2 Register"]
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
#[doc = "Stage %s Coefficient a1 Register"]
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
#[doc = "Stage %s Coefficient a2 Register"]
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
#[doc = "Stage %s Delay Data D0 Register"]
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
#[doc = "Stage %s Delay Data D1 Register"]
pub type Iirstgd1 = crate::RegValueT<Iirstgd1_SPEC>;

impl NoBitfieldReg<Iirstgd1_SPEC> for Iirstgd1 {}
impl ::core::default::Default for Iirstgd1 {
    #[inline(always)]
    fn default() -> Iirstgd1 {
        <crate::RegValueT<Iirstgd1_SPEC> as RegisterValue<_>>::new(0)
    }
}
