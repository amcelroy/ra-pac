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
#[doc = r"SRAM Control"]
unsafe impl ::core::marker::Send for super::Sram {}
unsafe impl ::core::marker::Sync for super::Sram {}
impl super::Sram {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn sramprcr_s(
        &self,
    ) -> &'static crate::common::Reg<self::SramprcrS_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SramprcrS_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sramprcr_ns(
        &self,
    ) -> &'static crate::common::Reg<self::SramprcrNs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SramprcrNs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sramwtsc(
        &self,
    ) -> &'static crate::common::Reg<self::Sramwtsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sramwtsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sramcr0(
        &self,
    ) -> &'static crate::common::Reg<self::Sramcr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sramcr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sramcr1(
        &self,
    ) -> &'static crate::common::Reg<self::Sramcr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sramcr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn srameccrgn0(
        &self,
    ) -> &'static crate::common::Reg<self::Srameccrgn0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Srameccrgn0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sramesr(
        &self,
    ) -> &'static crate::common::Reg<self::Sramesr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sramesr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sramesclr(
        &self,
    ) -> &'static crate::common::Reg<self::Sramesclr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sramesclr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sramear(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Sramear_SPEC, crate::common::R>,
        3,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x50usize))
        }
    }

    #[inline(always)]
    pub const fn stbramcr(
        &self,
    ) -> &'static crate::common::Reg<self::Stbramcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Stbramcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(272usize),
            )
        }
    }

    #[inline(always)]
    pub const fn stbramear(
        &self,
    ) -> &'static crate::common::Reg<self::Stbramear_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Stbramear_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(336usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SramprcrS_SPEC;
impl crate::sealed::RegSpec for SramprcrS_SPEC {
    type DataType = u16;
}

pub type SramprcrS = crate::RegValueT<SramprcrS_SPEC>;

impl SramprcrS {
    #[inline(always)]
    pub fn pr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sramprcr_s::Pr,
        sramprcr_s::Pr,
        SramprcrS_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sramprcr_s::Pr,
            sramprcr_s::Pr,
            SramprcrS_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, SramprcrS_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,SramprcrS_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kw(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, SramprcrS_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,SramprcrS_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SramprcrS {
    #[inline(always)]
    fn default() -> SramprcrS {
        <crate::RegValueT<SramprcrS_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sramprcr_s {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pr_SPEC;
    pub type Pr = crate::EnumBitfieldStruct<u8, Pr_SPEC>;
    impl Pr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SramprcrNs_SPEC;
impl crate::sealed::RegSpec for SramprcrNs_SPEC {
    type DataType = u16;
}

pub type SramprcrNs = crate::RegValueT<SramprcrNs_SPEC>;

impl SramprcrNs {
    #[inline(always)]
    pub fn pr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sramprcr_ns::Pr,
        sramprcr_ns::Pr,
        SramprcrNs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sramprcr_ns::Pr,
            sramprcr_ns::Pr,
            SramprcrNs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, SramprcrNs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,SramprcrNs_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kw(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, SramprcrNs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,SramprcrNs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SramprcrNs {
    #[inline(always)]
    fn default() -> SramprcrNs {
        <crate::RegValueT<SramprcrNs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sramprcr_ns {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pr_SPEC;
    pub type Pr = crate::EnumBitfieldStruct<u8, Pr_SPEC>;
    impl Pr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramwtsc_SPEC;
impl crate::sealed::RegSpec for Sramwtsc_SPEC {
    type DataType = u8;
}

pub type Sramwtsc = crate::RegValueT<Sramwtsc_SPEC>;

impl Sramwtsc {
    #[inline(always)]
    pub fn wten(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sramwtsc::Wten,
        sramwtsc::Wten,
        Sramwtsc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sramwtsc::Wten,
            sramwtsc::Wten,
            Sramwtsc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Sramwtsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Sramwtsc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sramwtsc {
    #[inline(always)]
    fn default() -> Sramwtsc {
        <crate::RegValueT<Sramwtsc_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod sramwtsc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wten_SPEC;
    pub type Wten = crate::EnumBitfieldStruct<u8, Wten_SPEC>;
    impl Wten {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramcr0_SPEC;
impl crate::sealed::RegSpec for Sramcr0_SPEC {
    type DataType = u8;
}

pub type Sramcr0 = crate::RegValueT<Sramcr0_SPEC>;

impl Sramcr0 {
    #[inline(always)]
    pub fn oad(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sramcr0::Oad,
        sramcr0::Oad,
        Sramcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sramcr0::Oad,
            sramcr0::Oad,
            Sramcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn eccmod(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        sramcr0::Eccmod,
        sramcr0::Eccmod,
        Sramcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            sramcr0::Eccmod,
            sramcr0::Eccmod,
            Sramcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn e1stsen(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sramcr0::E1Stsen,
        sramcr0::E1Stsen,
        Sramcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sramcr0::E1Stsen,
            sramcr0::E1Stsen,
            Sramcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, Sramcr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,Sramcr0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tstbyp(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        sramcr0::Tstbyp,
        sramcr0::Tstbyp,
        Sramcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            sramcr0::Tstbyp,
            sramcr0::Tstbyp,
            Sramcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sramcr0 {
    #[inline(always)]
    fn default() -> Sramcr0 {
        <crate::RegValueT<Sramcr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sramcr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oad_SPEC;
    pub type Oad = crate::EnumBitfieldStruct<u8, Oad_SPEC>;
    impl Oad {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eccmod_SPEC;
    pub type Eccmod = crate::EnumBitfieldStruct<u8, Eccmod_SPEC>;
    impl Eccmod {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct E1Stsen_SPEC;
    pub type E1Stsen = crate::EnumBitfieldStruct<u8, E1Stsen_SPEC>;
    impl E1Stsen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tstbyp_SPEC;
    pub type Tstbyp = crate::EnumBitfieldStruct<u8, Tstbyp_SPEC>;
    impl Tstbyp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramcr1_SPEC;
impl crate::sealed::RegSpec for Sramcr1_SPEC {
    type DataType = u8;
}

pub type Sramcr1 = crate::RegValueT<Sramcr1_SPEC>;

impl Sramcr1 {
    #[inline(always)]
    pub fn oad(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sramcr1::Oad,
        sramcr1::Oad,
        Sramcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sramcr1::Oad,
            sramcr1::Oad,
            Sramcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Sramcr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Sramcr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sramcr1 {
    #[inline(always)]
    fn default() -> Sramcr1 {
        <crate::RegValueT<Sramcr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sramcr1 {

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
pub struct Srameccrgn0_SPEC;
impl crate::sealed::RegSpec for Srameccrgn0_SPEC {
    type DataType = u8;
}

pub type Srameccrgn0 = crate::RegValueT<Srameccrgn0_SPEC>;

impl Srameccrgn0 {
    #[inline(always)]
    pub fn eccrgn(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        srameccrgn0::Eccrgn,
        srameccrgn0::Eccrgn,
        Srameccrgn0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            srameccrgn0::Eccrgn,
            srameccrgn0::Eccrgn,
            Srameccrgn0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, u8, Srameccrgn0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3f,1,0,u8,u8,Srameccrgn0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Srameccrgn0 {
    #[inline(always)]
    fn default() -> Srameccrgn0 {
        <crate::RegValueT<Srameccrgn0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod srameccrgn0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eccrgn_SPEC;
    pub type Eccrgn = crate::EnumBitfieldStruct<u8, Eccrgn_SPEC>;
    impl Eccrgn {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramesr_SPEC;
impl crate::sealed::RegSpec for Sramesr_SPEC {
    type DataType = u16;
}

pub type Sramesr = crate::RegValueT<Sramesr_SPEC>;

impl Sramesr {
    #[inline(always)]
    pub fn err00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sramesr::Err00,
        sramesr::Err00,
        Sramesr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sramesr::Err00,
            sramesr::Err00,
            Sramesr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn err01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        sramesr::Err01,
        sramesr::Err01,
        Sramesr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sramesr::Err01,
            sramesr::Err01,
            Sramesr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn err1(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        sramesr::Err1,
        sramesr::Err1,
        Sramesr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            sramesr::Err1,
            sramesr::Err1,
            Sramesr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn errs(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        sramesr::Errs,
        sramesr::Errs,
        Sramesr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            sramesr::Errs,
            sramesr::Errs,
            Sramesr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Sramesr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Sramesr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Sramesr {
    #[inline(always)]
    fn default() -> Sramesr {
        <crate::RegValueT<Sramesr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sramesr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Err00_SPEC;
    pub type Err00 = crate::EnumBitfieldStruct<u8, Err00_SPEC>;
    impl Err00 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Err01_SPEC;
    pub type Err01 = crate::EnumBitfieldStruct<u8, Err01_SPEC>;
    impl Err01 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Err1_SPEC;
    pub type Err1 = crate::EnumBitfieldStruct<u8, Err1_SPEC>;
    impl Err1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Errs_SPEC;
    pub type Errs = crate::EnumBitfieldStruct<u8, Errs_SPEC>;
    impl Errs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramesclr_SPEC;
impl crate::sealed::RegSpec for Sramesclr_SPEC {
    type DataType = u16;
}

pub type Sramesclr = crate::RegValueT<Sramesclr_SPEC>;

impl Sramesclr {
    #[inline(always)]
    pub fn clr00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sramesclr::Clr00,
        sramesclr::Clr00,
        Sramesclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sramesclr::Clr00,
            sramesclr::Clr00,
            Sramesclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn clr01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        sramesclr::Clr01,
        sramesclr::Clr01,
        Sramesclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sramesclr::Clr01,
            sramesclr::Clr01,
            Sramesclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn clr1(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        sramesclr::Clr1,
        sramesclr::Clr1,
        Sramesclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            sramesclr::Clr1,
            sramesclr::Clr1,
            Sramesclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn clrs(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        sramesclr::Clrs,
        sramesclr::Clrs,
        Sramesclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            sramesclr::Clrs,
            sramesclr::Clrs,
            Sramesclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Sramesclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Sramesclr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sramesclr {
    #[inline(always)]
    fn default() -> Sramesclr {
        <crate::RegValueT<Sramesclr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sramesclr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr00_SPEC;
    pub type Clr00 = crate::EnumBitfieldStruct<u8, Clr00_SPEC>;
    impl Clr00 {
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr01_SPEC;
    pub type Clr01 = crate::EnumBitfieldStruct<u8, Clr01_SPEC>;
    impl Clr01 {
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr1_SPEC;
    pub type Clr1 = crate::EnumBitfieldStruct<u8, Clr1_SPEC>;
    impl Clr1 {
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clrs_SPEC;
    pub type Clrs = crate::EnumBitfieldStruct<u8, Clrs_SPEC>;
    impl Clrs {
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramear_SPEC;
impl crate::sealed::RegSpec for Sramear_SPEC {
    type DataType = u32;
}

pub type Sramear = crate::RegValueT<Sramear_SPEC>;

impl Sramear {
    #[inline(always)]
    pub fn ea(
        self,
    ) -> crate::common::RegisterField<3, 0x1ffff, 1, 0, u32, u32, Sramear_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<3,0x1ffff,1,0,u32,u32,Sramear_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<20, 0xfff, 1, 0, u16, u16, Sramear_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<20,0xfff,1,0,u16,u16,Sramear_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Sramear {
    #[inline(always)]
    fn default() -> Sramear {
        <crate::RegValueT<Sramear_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stbramcr_SPEC;
impl crate::sealed::RegSpec for Stbramcr_SPEC {
    type DataType = u8;
}

pub type Stbramcr = crate::RegValueT<Stbramcr_SPEC>;

impl Stbramcr {
    #[inline(always)]
    pub fn oad(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        stbramcr::Oad,
        stbramcr::Oad,
        Stbramcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            stbramcr::Oad,
            stbramcr::Oad,
            Stbramcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Stbramcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Stbramcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Stbramcr {
    #[inline(always)]
    fn default() -> Stbramcr {
        <crate::RegValueT<Stbramcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod stbramcr {

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
pub struct Stbramear_SPEC;
impl crate::sealed::RegSpec for Stbramear_SPEC {
    type DataType = u32;
}

pub type Stbramear = crate::RegValueT<Stbramear_SPEC>;

impl Stbramear {
    #[inline(always)]
    pub fn ea(
        self,
    ) -> crate::common::RegisterField<2, 0xff, 1, 0, u8, u8, Stbramear_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0xff,1,0,u8,u8,Stbramear_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<10, 0x3fffff, 1, 0, u32, u32, Stbramear_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<10,0x3fffff,1,0,u32,u32,Stbramear_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Stbramear {
    #[inline(always)]
    fn default() -> Stbramear {
        <crate::RegValueT<Stbramear_SPEC> as RegisterValue<_>>::new(0)
    }
}
