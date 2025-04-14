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
#[doc = r"Low Power Asynchronous General Purpose Timer 0"]
unsafe impl ::core::marker::Send for super::Agtw0 {}
unsafe impl ::core::marker::Sync for super::Agtw0 {}
impl super::Agtw0 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn agt(&self) -> &'static crate::common::Reg<self::Agt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Agt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn agtcma(
        &self,
    ) -> &'static crate::common::Reg<self::Agtcma_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Agtcma_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn agtcmb(
        &self,
    ) -> &'static crate::common::Reg<self::Agtcmb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Agtcmb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn agtcr(&self) -> &'static crate::common::Reg<self::Agtcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Agtcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn agtmr1(
        &self,
    ) -> &'static crate::common::Reg<self::Agtmr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Agtmr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(13usize),
            )
        }
    }

    #[inline(always)]
    pub const fn agtmr2(
        &self,
    ) -> &'static crate::common::Reg<self::Agtmr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Agtmr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[inline(always)]
    pub const fn agtiosel(
        &self,
    ) -> &'static crate::common::Reg<self::Agtiosel_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Agtiosel_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(15usize),
            )
        }
    }

    #[inline(always)]
    pub const fn agtioc(
        &self,
    ) -> &'static crate::common::Reg<self::Agtioc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Agtioc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn agtisr(
        &self,
    ) -> &'static crate::common::Reg<self::Agtisr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Agtisr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(17usize),
            )
        }
    }

    #[inline(always)]
    pub const fn agtcmsr(
        &self,
    ) -> &'static crate::common::Reg<self::Agtcmsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Agtcmsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Agt_SPEC;
impl crate::sealed::RegSpec for Agt_SPEC {
    type DataType = u32;
}

pub type Agt = crate::RegValueT<Agt_SPEC>;

impl NoBitfieldReg<Agt_SPEC> for Agt {}
impl ::core::default::Default for Agt {
    #[inline(always)]
    fn default() -> Agt {
        <crate::RegValueT<Agt_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Agtcma_SPEC;
impl crate::sealed::RegSpec for Agtcma_SPEC {
    type DataType = u32;
}

pub type Agtcma = crate::RegValueT<Agtcma_SPEC>;

impl NoBitfieldReg<Agtcma_SPEC> for Agtcma {}
impl ::core::default::Default for Agtcma {
    #[inline(always)]
    fn default() -> Agtcma {
        <crate::RegValueT<Agtcma_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Agtcmb_SPEC;
impl crate::sealed::RegSpec for Agtcmb_SPEC {
    type DataType = u32;
}

pub type Agtcmb = crate::RegValueT<Agtcmb_SPEC>;

impl NoBitfieldReg<Agtcmb_SPEC> for Agtcmb {}
impl ::core::default::Default for Agtcmb {
    #[inline(always)]
    fn default() -> Agtcmb {
        <crate::RegValueT<Agtcmb_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Agtcr_SPEC;
impl crate::sealed::RegSpec for Agtcr_SPEC {
    type DataType = u8;
}

pub type Agtcr = crate::RegValueT<Agtcr_SPEC>;

impl Agtcr {
    #[inline(always)]
    pub fn tstart(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        agtcr::Tstart,
        agtcr::Tstart,
        Agtcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            agtcr::Tstart,
            agtcr::Tstart,
            Agtcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tcstf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        agtcr::Tcstf,
        agtcr::Tcstf,
        Agtcr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            agtcr::Tcstf,
            agtcr::Tcstf,
            Agtcr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tstop(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        agtcr::Tstop,
        agtcr::Tstop,
        Agtcr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            agtcr::Tstop,
            agtcr::Tstop,
            Agtcr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tedgf(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        agtcr::Tedgf,
        agtcr::Tedgf,
        Agtcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            agtcr::Tedgf,
            agtcr::Tedgf,
            Agtcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tundf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        agtcr::Tundf,
        agtcr::Tundf,
        Agtcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            agtcr::Tundf,
            agtcr::Tundf,
            Agtcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tcmaf(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        agtcr::Tcmaf,
        agtcr::Tcmaf,
        Agtcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            agtcr::Tcmaf,
            agtcr::Tcmaf,
            Agtcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tcmbf(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        agtcr::Tcmbf,
        agtcr::Tcmbf,
        Agtcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            agtcr::Tcmbf,
            agtcr::Tcmbf,
            Agtcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Agtcr {
    #[inline(always)]
    fn default() -> Agtcr {
        <crate::RegValueT<Agtcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod agtcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tstart_SPEC;
    pub type Tstart = crate::EnumBitfieldStruct<u8, Tstart_SPEC>;
    impl Tstart {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcstf_SPEC;
    pub type Tcstf = crate::EnumBitfieldStruct<u8, Tcstf_SPEC>;
    impl Tcstf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tstop_SPEC;
    pub type Tstop = crate::EnumBitfieldStruct<u8, Tstop_SPEC>;
    impl Tstop {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tedgf_SPEC;
    pub type Tedgf = crate::EnumBitfieldStruct<u8, Tedgf_SPEC>;
    impl Tedgf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tundf_SPEC;
    pub type Tundf = crate::EnumBitfieldStruct<u8, Tundf_SPEC>;
    impl Tundf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcmaf_SPEC;
    pub type Tcmaf = crate::EnumBitfieldStruct<u8, Tcmaf_SPEC>;
    impl Tcmaf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcmbf_SPEC;
    pub type Tcmbf = crate::EnumBitfieldStruct<u8, Tcmbf_SPEC>;
    impl Tcmbf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Agtmr1_SPEC;
impl crate::sealed::RegSpec for Agtmr1_SPEC {
    type DataType = u8;
}

pub type Agtmr1 = crate::RegValueT<Agtmr1_SPEC>;

impl Agtmr1 {
    #[inline(always)]
    pub fn tmod(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        agtmr1::Tmod,
        agtmr1::Tmod,
        Agtmr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            agtmr1::Tmod,
            agtmr1::Tmod,
            Agtmr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tedgpl(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        agtmr1::Tedgpl,
        agtmr1::Tedgpl,
        Agtmr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            agtmr1::Tedgpl,
            agtmr1::Tedgpl,
            Agtmr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tck(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x7,
        1,
        0,
        agtmr1::Tck,
        agtmr1::Tck,
        Agtmr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            agtmr1::Tck,
            agtmr1::Tck,
            Agtmr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Agtmr1 {
    #[inline(always)]
    fn default() -> Agtmr1 {
        <crate::RegValueT<Agtmr1_SPEC> as RegisterValue<_>>::new(64)
    }
}
pub mod agtmr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmod_SPEC;
    pub type Tmod = crate::EnumBitfieldStruct<u8, Tmod_SPEC>;
    impl Tmod {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _101: Self = Self::new(5);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tedgpl_SPEC;
    pub type Tedgpl = crate::EnumBitfieldStruct<u8, Tedgpl_SPEC>;
    impl Tedgpl {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tck_SPEC;
    pub type Tck = crate::EnumBitfieldStruct<u8, Tck_SPEC>;
    impl Tck {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _101: Self = Self::new(5);

        pub const _110: Self = Self::new(6);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Agtmr2_SPEC;
impl crate::sealed::RegSpec for Agtmr2_SPEC {
    type DataType = u8;
}

pub type Agtmr2 = crate::RegValueT<Agtmr2_SPEC>;

impl Agtmr2 {
    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        agtmr2::Cks,
        agtmr2::Cks,
        Agtmr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            agtmr2::Cks,
            agtmr2::Cks,
            Agtmr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lpm(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        agtmr2::Lpm,
        agtmr2::Lpm,
        Agtmr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            agtmr2::Lpm,
            agtmr2::Lpm,
            Agtmr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Agtmr2 {
    #[inline(always)]
    fn default() -> Agtmr2 {
        <crate::RegValueT<Agtmr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod agtmr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
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
    pub struct Lpm_SPEC;
    pub type Lpm = crate::EnumBitfieldStruct<u8, Lpm_SPEC>;
    impl Lpm {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Agtiosel_SPEC;
impl crate::sealed::RegSpec for Agtiosel_SPEC {
    type DataType = u8;
}

pub type Agtiosel = crate::RegValueT<Agtiosel_SPEC>;

impl Agtiosel {
    #[inline(always)]
    pub fn ties(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        agtiosel::Ties,
        agtiosel::Ties,
        Agtiosel_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            agtiosel::Ties,
            agtiosel::Ties,
            Agtiosel_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Agtiosel {
    #[inline(always)]
    fn default() -> Agtiosel {
        <crate::RegValueT<Agtiosel_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod agtiosel {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ties_SPEC;
    pub type Ties = crate::EnumBitfieldStruct<u8, Ties_SPEC>;
    impl Ties {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Agtioc_SPEC;
impl crate::sealed::RegSpec for Agtioc_SPEC {
    type DataType = u8;
}

pub type Agtioc = crate::RegValueT<Agtioc_SPEC>;

impl Agtioc {
    #[inline(always)]
    pub fn tedgsel(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Agtioc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Agtioc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn toe(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        agtioc::Toe,
        agtioc::Toe,
        Agtioc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            agtioc::Toe,
            agtioc::Toe,
            Agtioc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tipf(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        agtioc::Tipf,
        agtioc::Tipf,
        Agtioc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            agtioc::Tipf,
            agtioc::Tipf,
            Agtioc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tiogt(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        agtioc::Tiogt,
        agtioc::Tiogt,
        Agtioc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            agtioc::Tiogt,
            agtioc::Tiogt,
            Agtioc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Agtioc {
    #[inline(always)]
    fn default() -> Agtioc {
        <crate::RegValueT<Agtioc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod agtioc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Toe_SPEC;
    pub type Toe = crate::EnumBitfieldStruct<u8, Toe_SPEC>;
    impl Toe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tipf_SPEC;
    pub type Tipf = crate::EnumBitfieldStruct<u8, Tipf_SPEC>;
    impl Tipf {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tiogt_SPEC;
    pub type Tiogt = crate::EnumBitfieldStruct<u8, Tiogt_SPEC>;
    impl Tiogt {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Agtisr_SPEC;
impl crate::sealed::RegSpec for Agtisr_SPEC {
    type DataType = u8;
}

pub type Agtisr = crate::RegValueT<Agtisr_SPEC>;

impl Agtisr {
    #[inline(always)]
    pub fn eeps(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        agtisr::Eeps,
        agtisr::Eeps,
        Agtisr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            agtisr::Eeps,
            agtisr::Eeps,
            Agtisr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Agtisr {
    #[inline(always)]
    fn default() -> Agtisr {
        <crate::RegValueT<Agtisr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod agtisr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eeps_SPEC;
    pub type Eeps = crate::EnumBitfieldStruct<u8, Eeps_SPEC>;
    impl Eeps {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Agtcmsr_SPEC;
impl crate::sealed::RegSpec for Agtcmsr_SPEC {
    type DataType = u8;
}

pub type Agtcmsr = crate::RegValueT<Agtcmsr_SPEC>;

impl Agtcmsr {
    #[inline(always)]
    pub fn tcmea(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        agtcmsr::Tcmea,
        agtcmsr::Tcmea,
        Agtcmsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            agtcmsr::Tcmea,
            agtcmsr::Tcmea,
            Agtcmsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn toea(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        agtcmsr::Toea,
        agtcmsr::Toea,
        Agtcmsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            agtcmsr::Toea,
            agtcmsr::Toea,
            Agtcmsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn topola(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        agtcmsr::Topola,
        agtcmsr::Topola,
        Agtcmsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            agtcmsr::Topola,
            agtcmsr::Topola,
            Agtcmsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tcmeb(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        agtcmsr::Tcmeb,
        agtcmsr::Tcmeb,
        Agtcmsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            agtcmsr::Tcmeb,
            agtcmsr::Tcmeb,
            Agtcmsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn toeb(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        agtcmsr::Toeb,
        agtcmsr::Toeb,
        Agtcmsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            agtcmsr::Toeb,
            agtcmsr::Toeb,
            Agtcmsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn topolb(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        agtcmsr::Topolb,
        agtcmsr::Topolb,
        Agtcmsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            agtcmsr::Topolb,
            agtcmsr::Topolb,
            Agtcmsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Agtcmsr {
    #[inline(always)]
    fn default() -> Agtcmsr {
        <crate::RegValueT<Agtcmsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod agtcmsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcmea_SPEC;
    pub type Tcmea = crate::EnumBitfieldStruct<u8, Tcmea_SPEC>;
    impl Tcmea {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Toea_SPEC;
    pub type Toea = crate::EnumBitfieldStruct<u8, Toea_SPEC>;
    impl Toea {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Topola_SPEC;
    pub type Topola = crate::EnumBitfieldStruct<u8, Topola_SPEC>;
    impl Topola {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcmeb_SPEC;
    pub type Tcmeb = crate::EnumBitfieldStruct<u8, Tcmeb_SPEC>;
    impl Tcmeb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Toeb_SPEC;
    pub type Toeb = crate::EnumBitfieldStruct<u8, Toeb_SPEC>;
    impl Toeb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Topolb_SPEC;
    pub type Topolb = crate::EnumBitfieldStruct<u8, Topolb_SPEC>;
    impl Topolb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
