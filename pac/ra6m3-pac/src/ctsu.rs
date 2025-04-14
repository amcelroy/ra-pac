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
#[doc = r"Capacitive Touch Sensing Unit"]
unsafe impl ::core::marker::Send for super::Ctsu {}
unsafe impl ::core::marker::Sync for super::Ctsu {}
impl super::Ctsu {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn ctsucr0(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsucr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsucr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsucr1(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsucr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsucr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsusdprs(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsusdprs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsusdprs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsusst(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsusst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsusst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsumch0(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsumch0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsumch0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsumch1(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsumch1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ctsumch1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(5usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuchac0(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchac0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchac0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuchac1(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchac1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchac1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(7usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuchac2(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchac2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchac2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuchtrc0(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchtrc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchtrc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(11usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuchtrc1(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchtrc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchtrc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuchtrc2(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchtrc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchtrc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(13usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsudclkc(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsudclkc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsudclkc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsust(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsust_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsust_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(17usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsussc(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsussc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsussc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuso0(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuso0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuso0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuso1(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuso1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuso1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsusc(&self) -> &'static crate::common::Reg<self::Ctsusc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ctsusc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsurc(&self) -> &'static crate::common::Reg<self::Ctsurc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ctsurc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(26usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuerrs(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuerrs_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ctsuerrs_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsucr0_SPEC;
impl crate::sealed::RegSpec for Ctsucr0_SPEC {
    type DataType = u8;
}

pub type Ctsucr0 = crate::RegValueT<Ctsucr0_SPEC>;

impl Ctsucr0 {
    #[inline(always)]
    pub fn ctsutxvsel(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ctsucr0::Ctsutxvsel,
        ctsucr0::Ctsutxvsel,
        Ctsucr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ctsucr0::Ctsutxvsel,
            ctsucr0::Ctsutxvsel,
            Ctsucr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ctsuinit(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ctsucr0::Ctsuinit,
        ctsucr0::Ctsuinit,
        Ctsucr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ctsucr0::Ctsuinit,
            ctsucr0::Ctsuinit,
            Ctsucr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ctsusnz(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ctsucr0::Ctsusnz,
        ctsucr0::Ctsusnz,
        Ctsucr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ctsucr0::Ctsusnz,
            ctsucr0::Ctsusnz,
            Ctsucr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ctsucap(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ctsucr0::Ctsucap,
        ctsucr0::Ctsucap,
        Ctsucr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ctsucr0::Ctsucap,
            ctsucr0::Ctsucap,
            Ctsucr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ctsustrt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ctsucr0::Ctsustrt,
        ctsucr0::Ctsustrt,
        Ctsucr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ctsucr0::Ctsustrt,
            ctsucr0::Ctsustrt,
            Ctsucr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsucr0 {
    #[inline(always)]
    fn default() -> Ctsucr0 {
        <crate::RegValueT<Ctsucr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctsucr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctsutxvsel_SPEC;
    pub type Ctsutxvsel = crate::EnumBitfieldStruct<u8, Ctsutxvsel_SPEC>;
    impl Ctsutxvsel {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctsuinit_SPEC;
    pub type Ctsuinit = crate::EnumBitfieldStruct<u8, Ctsuinit_SPEC>;
    impl Ctsuinit {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctsusnz_SPEC;
    pub type Ctsusnz = crate::EnumBitfieldStruct<u8, Ctsusnz_SPEC>;
    impl Ctsusnz {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctsucap_SPEC;
    pub type Ctsucap = crate::EnumBitfieldStruct<u8, Ctsucap_SPEC>;
    impl Ctsucap {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctsustrt_SPEC;
    pub type Ctsustrt = crate::EnumBitfieldStruct<u8, Ctsustrt_SPEC>;
    impl Ctsustrt {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsucr1_SPEC;
impl crate::sealed::RegSpec for Ctsucr1_SPEC {
    type DataType = u8;
}

pub type Ctsucr1 = crate::RegValueT<Ctsucr1_SPEC>;

impl Ctsucr1 {
    #[inline(always)]
    pub fn ctsumd(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        ctsucr1::Ctsumd,
        ctsucr1::Ctsumd,
        Ctsucr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            ctsucr1::Ctsumd,
            ctsucr1::Ctsumd,
            Ctsucr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ctsuclk(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        ctsucr1::Ctsuclk,
        ctsucr1::Ctsuclk,
        Ctsucr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            ctsucr1::Ctsuclk,
            ctsucr1::Ctsuclk,
            Ctsucr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ctsuatune1(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ctsucr1::Ctsuatune1,
        ctsucr1::Ctsuatune1,
        Ctsucr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ctsucr1::Ctsuatune1,
            ctsucr1::Ctsuatune1,
            Ctsucr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ctsucsw(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ctsucr1::Ctsucsw,
        ctsucr1::Ctsucsw,
        Ctsucr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ctsucr1::Ctsucsw,
            ctsucr1::Ctsucsw,
            Ctsucr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ctsupon(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ctsucr1::Ctsupon,
        ctsucr1::Ctsupon,
        Ctsucr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ctsucr1::Ctsupon,
            ctsucr1::Ctsupon,
            Ctsucr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsucr1 {
    #[inline(always)]
    fn default() -> Ctsucr1 {
        <crate::RegValueT<Ctsucr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctsucr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctsumd_SPEC;
    pub type Ctsumd = crate::EnumBitfieldStruct<u8, Ctsumd_SPEC>;
    impl Ctsumd {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctsuclk_SPEC;
    pub type Ctsuclk = crate::EnumBitfieldStruct<u8, Ctsuclk_SPEC>;
    impl Ctsuclk {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctsuatune1_SPEC;
    pub type Ctsuatune1 = crate::EnumBitfieldStruct<u8, Ctsuatune1_SPEC>;
    impl Ctsuatune1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctsucsw_SPEC;
    pub type Ctsucsw = crate::EnumBitfieldStruct<u8, Ctsucsw_SPEC>;
    impl Ctsucsw {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctsupon_SPEC;
    pub type Ctsupon = crate::EnumBitfieldStruct<u8, Ctsupon_SPEC>;
    impl Ctsupon {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusdprs_SPEC;
impl crate::sealed::RegSpec for Ctsusdprs_SPEC {
    type DataType = u8;
}

pub type Ctsusdprs = crate::RegValueT<Ctsusdprs_SPEC>;

impl Ctsusdprs {
    #[inline(always)]
    pub fn ctsusoff(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ctsusdprs::Ctsusoff,
        ctsusdprs::Ctsusoff,
        Ctsusdprs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ctsusdprs::Ctsusoff,
            ctsusdprs::Ctsusoff,
            Ctsusdprs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ctsuprmode(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        ctsusdprs::Ctsuprmode,
        ctsusdprs::Ctsuprmode,
        Ctsusdprs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            ctsusdprs::Ctsuprmode,
            ctsusdprs::Ctsuprmode,
            Ctsusdprs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ctsuprratio(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Ctsusdprs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Ctsusdprs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsusdprs {
    #[inline(always)]
    fn default() -> Ctsusdprs {
        <crate::RegValueT<Ctsusdprs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctsusdprs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctsusoff_SPEC;
    pub type Ctsusoff = crate::EnumBitfieldStruct<u8, Ctsusoff_SPEC>;
    impl Ctsusoff {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctsuprmode_SPEC;
    pub type Ctsuprmode = crate::EnumBitfieldStruct<u8, Ctsuprmode_SPEC>;
    impl Ctsuprmode {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusst_SPEC;
impl crate::sealed::RegSpec for Ctsusst_SPEC {
    type DataType = u8;
}

pub type Ctsusst = crate::RegValueT<Ctsusst_SPEC>;

impl Ctsusst {
    #[inline(always)]
    pub fn ctsusst(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ctsusst_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ctsusst_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsusst {
    #[inline(always)]
    fn default() -> Ctsusst {
        <crate::RegValueT<Ctsusst_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumch0_SPEC;
impl crate::sealed::RegSpec for Ctsumch0_SPEC {
    type DataType = u8;
}

pub type Ctsumch0 = crate::RegValueT<Ctsumch0_SPEC>;

impl Ctsumch0 {
    #[inline(always)]
    pub fn ctsumch0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        ctsumch0::Ctsumch0,
        ctsumch0::Ctsumch0,
        Ctsumch0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            ctsumch0::Ctsumch0,
            ctsumch0::Ctsumch0,
            Ctsumch0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsumch0 {
    #[inline(always)]
    fn default() -> Ctsumch0 {
        <crate::RegValueT<Ctsumch0_SPEC> as RegisterValue<_>>::new(31)
    }
}
pub mod ctsumch0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctsumch0_SPEC;
    pub type Ctsumch0 = crate::EnumBitfieldStruct<u8, Ctsumch0_SPEC>;
    impl Ctsumch0 {
        pub const CTSUMCH_0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumch1_SPEC;
impl crate::sealed::RegSpec for Ctsumch1_SPEC {
    type DataType = u8;
}

pub type Ctsumch1 = crate::RegValueT<Ctsumch1_SPEC>;

impl Ctsumch1 {
    #[inline(always)]
    pub fn ctsumch1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        ctsumch1::Ctsumch1,
        ctsumch1::Ctsumch1,
        Ctsumch1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            ctsumch1::Ctsumch1,
            ctsumch1::Ctsumch1,
            Ctsumch1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsumch1 {
    #[inline(always)]
    fn default() -> Ctsumch1 {
        <crate::RegValueT<Ctsumch1_SPEC> as RegisterValue<_>>::new(31)
    }
}
pub mod ctsumch1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctsumch1_SPEC;
    pub type Ctsumch1 = crate::EnumBitfieldStruct<u8, Ctsumch1_SPEC>;
    impl Ctsumch1 {
        pub const CTSUMCH_1: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchac0_SPEC;
impl crate::sealed::RegSpec for Ctsuchac0_SPEC {
    type DataType = u8;
}

pub type Ctsuchac0 = crate::RegValueT<Ctsuchac0_SPEC>;

impl Ctsuchac0 {
    #[inline(always)]
    pub fn ctsuchac0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        ctsuchac0::Ctsuchac0,
        ctsuchac0::Ctsuchac0,
        Ctsuchac0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            ctsuchac0::Ctsuchac0,
            ctsuchac0::Ctsuchac0,
            Ctsuchac0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsuchac0 {
    #[inline(always)]
    fn default() -> Ctsuchac0 {
        <crate::RegValueT<Ctsuchac0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctsuchac0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctsuchac0_SPEC;
    pub type Ctsuchac0 = crate::EnumBitfieldStruct<u8, Ctsuchac0_SPEC>;
    impl Ctsuchac0 {
        pub const CTSUCHAC_0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchac1_SPEC;
impl crate::sealed::RegSpec for Ctsuchac1_SPEC {
    type DataType = u8;
}

pub type Ctsuchac1 = crate::RegValueT<Ctsuchac1_SPEC>;

impl Ctsuchac1 {
    #[inline(always)]
    pub fn ctsuchac1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        ctsuchac1::Ctsuchac1,
        ctsuchac1::Ctsuchac1,
        Ctsuchac1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            ctsuchac1::Ctsuchac1,
            ctsuchac1::Ctsuchac1,
            Ctsuchac1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsuchac1 {
    #[inline(always)]
    fn default() -> Ctsuchac1 {
        <crate::RegValueT<Ctsuchac1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctsuchac1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctsuchac1_SPEC;
    pub type Ctsuchac1 = crate::EnumBitfieldStruct<u8, Ctsuchac1_SPEC>;
    impl Ctsuchac1 {
        pub const CTSUCHAC_1: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchac2_SPEC;
impl crate::sealed::RegSpec for Ctsuchac2_SPEC {
    type DataType = u8;
}

pub type Ctsuchac2 = crate::RegValueT<Ctsuchac2_SPEC>;

impl NoBitfieldReg<Ctsuchac2_SPEC> for Ctsuchac2 {}
impl ::core::default::Default for Ctsuchac2 {
    #[inline(always)]
    fn default() -> Ctsuchac2 {
        <crate::RegValueT<Ctsuchac2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchtrc0_SPEC;
impl crate::sealed::RegSpec for Ctsuchtrc0_SPEC {
    type DataType = u8;
}

pub type Ctsuchtrc0 = crate::RegValueT<Ctsuchtrc0_SPEC>;

impl Ctsuchtrc0 {
    #[inline(always)]
    pub fn ctsuchtrc0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ctsuchtrc0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ctsuchtrc0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuchtrc0 {
    #[inline(always)]
    fn default() -> Ctsuchtrc0 {
        <crate::RegValueT<Ctsuchtrc0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchtrc1_SPEC;
impl crate::sealed::RegSpec for Ctsuchtrc1_SPEC {
    type DataType = u8;
}

pub type Ctsuchtrc1 = crate::RegValueT<Ctsuchtrc1_SPEC>;

impl Ctsuchtrc1 {
    #[inline(always)]
    pub fn ctsuchtrc1(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ctsuchtrc1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ctsuchtrc1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuchtrc1 {
    #[inline(always)]
    fn default() -> Ctsuchtrc1 {
        <crate::RegValueT<Ctsuchtrc1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchtrc2_SPEC;
impl crate::sealed::RegSpec for Ctsuchtrc2_SPEC {
    type DataType = u8;
}

pub type Ctsuchtrc2 = crate::RegValueT<Ctsuchtrc2_SPEC>;

impl NoBitfieldReg<Ctsuchtrc2_SPEC> for Ctsuchtrc2 {}
impl ::core::default::Default for Ctsuchtrc2 {
    #[inline(always)]
    fn default() -> Ctsuchtrc2 {
        <crate::RegValueT<Ctsuchtrc2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsudclkc_SPEC;
impl crate::sealed::RegSpec for Ctsudclkc_SPEC {
    type DataType = u8;
}

pub type Ctsudclkc = crate::RegValueT<Ctsudclkc_SPEC>;

impl Ctsudclkc {
    #[inline(always)]
    pub fn ctsusscnt(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Ctsudclkc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Ctsudclkc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ctsussmod(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Ctsudclkc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,Ctsudclkc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsudclkc {
    #[inline(always)]
    fn default() -> Ctsudclkc {
        <crate::RegValueT<Ctsudclkc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsust_SPEC;
impl crate::sealed::RegSpec for Ctsust_SPEC {
    type DataType = u8;
}

pub type Ctsust = crate::RegValueT<Ctsust_SPEC>;

impl Ctsust {
    #[inline(always)]
    pub fn ctsups(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ctsust::Ctsups,
        ctsust::Ctsups,
        Ctsust_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ctsust::Ctsups,
            ctsust::Ctsups,
            Ctsust_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ctsurovf(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ctsust::Ctsurovf,
        ctsust::Ctsurovf,
        Ctsust_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ctsust::Ctsurovf,
            ctsust::Ctsurovf,
            Ctsust_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ctsusovf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ctsust::Ctsusovf,
        ctsust::Ctsusovf,
        Ctsust_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ctsust::Ctsusovf,
            ctsust::Ctsusovf,
            Ctsust_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ctsudtsr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ctsust::Ctsudtsr,
        ctsust::Ctsudtsr,
        Ctsust_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ctsust::Ctsudtsr,
            ctsust::Ctsudtsr,
            Ctsust_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ctsustc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        ctsust::Ctsustc,
        ctsust::Ctsustc,
        Ctsust_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            ctsust::Ctsustc,
            ctsust::Ctsustc,
            Ctsust_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsust {
    #[inline(always)]
    fn default() -> Ctsust {
        <crate::RegValueT<Ctsust_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctsust {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctsups_SPEC;
    pub type Ctsups = crate::EnumBitfieldStruct<u8, Ctsups_SPEC>;
    impl Ctsups {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctsurovf_SPEC;
    pub type Ctsurovf = crate::EnumBitfieldStruct<u8, Ctsurovf_SPEC>;
    impl Ctsurovf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctsusovf_SPEC;
    pub type Ctsusovf = crate::EnumBitfieldStruct<u8, Ctsusovf_SPEC>;
    impl Ctsusovf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctsudtsr_SPEC;
    pub type Ctsudtsr = crate::EnumBitfieldStruct<u8, Ctsudtsr_SPEC>;
    impl Ctsudtsr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctsustc_SPEC;
    pub type Ctsustc = crate::EnumBitfieldStruct<u8, Ctsustc_SPEC>;
    impl Ctsustc {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _101: Self = Self::new(5);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsussc_SPEC;
impl crate::sealed::RegSpec for Ctsussc_SPEC {
    type DataType = u16;
}

pub type Ctsussc = crate::RegValueT<Ctsussc_SPEC>;

impl Ctsussc {
    #[inline(always)]
    pub fn ctsussdiv(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        ctsussc::Ctsussdiv,
        ctsussc::Ctsussdiv,
        Ctsussc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            ctsussc::Ctsussdiv,
            ctsussc::Ctsussdiv,
            Ctsussc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsussc {
    #[inline(always)]
    fn default() -> Ctsussc {
        <crate::RegValueT<Ctsussc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctsussc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctsussdiv_SPEC;
    pub type Ctsussdiv = crate::EnumBitfieldStruct<u8, Ctsussdiv_SPEC>;
    impl Ctsussdiv {
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

        pub const _1111: Self = Self::new(15);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuso0_SPEC;
impl crate::sealed::RegSpec for Ctsuso0_SPEC {
    type DataType = u16;
}

pub type Ctsuso0 = crate::RegValueT<Ctsuso0_SPEC>;

impl Ctsuso0 {
    #[inline(always)]
    pub fn ctsusnum(
        self,
    ) -> crate::common::RegisterField<10, 0x3f, 1, 0, u8, u8, Ctsuso0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3f,1,0,u8,u8,Ctsuso0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ctsuso(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Ctsuso0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Ctsuso0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuso0 {
    #[inline(always)]
    fn default() -> Ctsuso0 {
        <crate::RegValueT<Ctsuso0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuso1_SPEC;
impl crate::sealed::RegSpec for Ctsuso1_SPEC {
    type DataType = u16;
}

pub type Ctsuso1 = crate::RegValueT<Ctsuso1_SPEC>;

impl Ctsuso1 {
    #[inline(always)]
    pub fn ctsuicog(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x3,
        1,
        0,
        ctsuso1::Ctsuicog,
        ctsuso1::Ctsuicog,
        Ctsuso1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x3,
            1,
            0,
            ctsuso1::Ctsuicog,
            ctsuso1::Ctsuicog,
            Ctsuso1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ctsusdpa(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, Ctsuso1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,Ctsuso1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ctsuricoa(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ctsuso1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ctsuso1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuso1 {
    #[inline(always)]
    fn default() -> Ctsuso1 {
        <crate::RegValueT<Ctsuso1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctsuso1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctsuicog_SPEC;
    pub type Ctsuicog = crate::EnumBitfieldStruct<u8, Ctsuicog_SPEC>;
    impl Ctsuicog {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusc_SPEC;
impl crate::sealed::RegSpec for Ctsusc_SPEC {
    type DataType = u16;
}

pub type Ctsusc = crate::RegValueT<Ctsusc_SPEC>;

impl Ctsusc {
    #[inline(always)]
    pub fn ctsusc(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsusc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Ctsusc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsusc {
    #[inline(always)]
    fn default() -> Ctsusc {
        <crate::RegValueT<Ctsusc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsurc_SPEC;
impl crate::sealed::RegSpec for Ctsurc_SPEC {
    type DataType = u16;
}

pub type Ctsurc = crate::RegValueT<Ctsurc_SPEC>;

impl Ctsurc {
    #[inline(always)]
    pub fn ctsurc(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsurc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Ctsurc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsurc {
    #[inline(always)]
    fn default() -> Ctsurc {
        <crate::RegValueT<Ctsurc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuerrs_SPEC;
impl crate::sealed::RegSpec for Ctsuerrs_SPEC {
    type DataType = u16;
}

pub type Ctsuerrs = crate::RegValueT<Ctsuerrs_SPEC>;

impl Ctsuerrs {
    #[inline(always)]
    pub fn ctsuicomp(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        ctsuerrs::Ctsuicomp,
        ctsuerrs::Ctsuicomp,
        Ctsuerrs_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            ctsuerrs::Ctsuicomp,
            ctsuerrs::Ctsuicomp,
            Ctsuerrs_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsuerrs {
    #[inline(always)]
    fn default() -> Ctsuerrs {
        <crate::RegValueT<Ctsuerrs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctsuerrs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctsuicomp_SPEC;
    pub type Ctsuicomp = crate::EnumBitfieldStruct<u8, Ctsuicomp_SPEC>;
    impl Ctsuicomp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
