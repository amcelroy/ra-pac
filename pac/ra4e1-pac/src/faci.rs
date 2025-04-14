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
// Generated from SVD 1.20.00, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:17:48 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Flash/CPU Interface"]
unsafe impl ::core::marker::Send for super::Faci {}
unsafe impl ::core::marker::Sync for super::Faci {}
impl super::Faci {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn fastat(
        &self,
    ) -> &'static crate::common::Reg<self::Fastat_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fastat_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn faeint(
        &self,
    ) -> &'static crate::common::Reg<self::Faeint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Faeint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn frdyie(
        &self,
    ) -> &'static crate::common::Reg<self::Frdyie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Frdyie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fsaddr(
        &self,
    ) -> &'static crate::common::Reg<self::Fsaddr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fsaddr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn feaddr(
        &self,
    ) -> &'static crate::common::Reg<self::Feaddr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Feaddr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fmeprot(
        &self,
    ) -> &'static crate::common::Reg<self::Fmeprot_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fmeprot_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fbprot0(
        &self,
    ) -> &'static crate::common::Reg<self::Fbprot0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fbprot0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fbprot1(
        &self,
    ) -> &'static crate::common::Reg<self::Fbprot1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fbprot1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fstatr(
        &self,
    ) -> &'static crate::common::Reg<self::Fstatr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fstatr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fentryr(
        &self,
    ) -> &'static crate::common::Reg<self::Fentryr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fentryr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fsuinitr(
        &self,
    ) -> &'static crate::common::Reg<self::Fsuinitr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fsuinitr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fcmdr(&self) -> &'static crate::common::Reg<self::Fcmdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fcmdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fbccnt(
        &self,
    ) -> &'static crate::common::Reg<self::Fbccnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fbccnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(208usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fbcstat(
        &self,
    ) -> &'static crate::common::Reg<self::Fbcstat_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fbcstat_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(212usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fpsaddr(
        &self,
    ) -> &'static crate::common::Reg<self::Fpsaddr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fpsaddr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(216usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fsuasmon(
        &self,
    ) -> &'static crate::common::Reg<self::Fsuasmon_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fsuasmon_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(220usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fcpsr(&self) -> &'static crate::common::Reg<self::Fcpsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fcpsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(224usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fpckar(
        &self,
    ) -> &'static crate::common::Reg<self::Fpckar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fpckar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(228usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fsuacr(
        &self,
    ) -> &'static crate::common::Reg<self::Fsuacr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fsuacr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(232usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fastat_SPEC;
impl crate::sealed::RegSpec for Fastat_SPEC {
    type DataType = u8;
}

pub type Fastat = crate::RegValueT<Fastat_SPEC>;

impl Fastat {
    #[inline(always)]
    pub fn dfae(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        fastat::Dfae,
        fastat::Dfae,
        Fastat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            fastat::Dfae,
            fastat::Dfae,
            Fastat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmdlk(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        fastat::Cmdlk,
        fastat::Cmdlk,
        Fastat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            fastat::Cmdlk,
            fastat::Cmdlk,
            Fastat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfae(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        fastat::Cfae,
        fastat::Cfae,
        Fastat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            fastat::Cfae,
            fastat::Cfae,
            Fastat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fastat {
    #[inline(always)]
    fn default() -> Fastat {
        <crate::RegValueT<Fastat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fastat {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dfae_SPEC;
    pub type Dfae = crate::EnumBitfieldStruct<u8, Dfae_SPEC>;
    impl Dfae {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmdlk_SPEC;
    pub type Cmdlk = crate::EnumBitfieldStruct<u8, Cmdlk_SPEC>;
    impl Cmdlk {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfae_SPEC;
    pub type Cfae = crate::EnumBitfieldStruct<u8, Cfae_SPEC>;
    impl Cfae {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Faeint_SPEC;
impl crate::sealed::RegSpec for Faeint_SPEC {
    type DataType = u8;
}

pub type Faeint = crate::RegValueT<Faeint_SPEC>;

impl Faeint {
    #[inline(always)]
    pub fn dfaeie(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        faeint::Dfaeie,
        faeint::Dfaeie,
        Faeint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            faeint::Dfaeie,
            faeint::Dfaeie,
            Faeint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmdlkie(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        faeint::Cmdlkie,
        faeint::Cmdlkie,
        Faeint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            faeint::Cmdlkie,
            faeint::Cmdlkie,
            Faeint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfaeie(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        faeint::Cfaeie,
        faeint::Cfaeie,
        Faeint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            faeint::Cfaeie,
            faeint::Cfaeie,
            Faeint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Faeint {
    #[inline(always)]
    fn default() -> Faeint {
        <crate::RegValueT<Faeint_SPEC> as RegisterValue<_>>::new(152)
    }
}
pub mod faeint {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dfaeie_SPEC;
    pub type Dfaeie = crate::EnumBitfieldStruct<u8, Dfaeie_SPEC>;
    impl Dfaeie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmdlkie_SPEC;
    pub type Cmdlkie = crate::EnumBitfieldStruct<u8, Cmdlkie_SPEC>;
    impl Cmdlkie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfaeie_SPEC;
    pub type Cfaeie = crate::EnumBitfieldStruct<u8, Cfaeie_SPEC>;
    impl Cfaeie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frdyie_SPEC;
impl crate::sealed::RegSpec for Frdyie_SPEC {
    type DataType = u8;
}

pub type Frdyie = crate::RegValueT<Frdyie_SPEC>;

impl Frdyie {
    #[inline(always)]
    pub fn frdyie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        frdyie::Frdyie,
        frdyie::Frdyie,
        Frdyie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            frdyie::Frdyie,
            frdyie::Frdyie,
            Frdyie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Frdyie {
    #[inline(always)]
    fn default() -> Frdyie {
        <crate::RegValueT<Frdyie_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod frdyie {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Frdyie_SPEC;
    pub type Frdyie = crate::EnumBitfieldStruct<u8, Frdyie_SPEC>;
    impl Frdyie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsaddr_SPEC;
impl crate::sealed::RegSpec for Fsaddr_SPEC {
    type DataType = u32;
}

pub type Fsaddr = crate::RegValueT<Fsaddr_SPEC>;

impl Fsaddr {
    #[inline(always)]
    pub fn fsaddr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fsaddr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fsaddr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fsaddr {
    #[inline(always)]
    fn default() -> Fsaddr {
        <crate::RegValueT<Fsaddr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Feaddr_SPEC;
impl crate::sealed::RegSpec for Feaddr_SPEC {
    type DataType = u32;
}

pub type Feaddr = crate::RegValueT<Feaddr_SPEC>;

impl Feaddr {
    #[inline(always)]
    pub fn feaddr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Feaddr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Feaddr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Feaddr {
    #[inline(always)]
    fn default() -> Feaddr {
        <crate::RegValueT<Feaddr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fmeprot_SPEC;
impl crate::sealed::RegSpec for Fmeprot_SPEC {
    type DataType = u16;
}

pub type Fmeprot = crate::RegValueT<Fmeprot_SPEC>;

impl Fmeprot {
    #[inline(always)]
    pub fn ceprot(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fmeprot::Ceprot,
        fmeprot::Ceprot,
        Fmeprot_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fmeprot::Ceprot,
            fmeprot::Ceprot,
            Fmeprot_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Fmeprot_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Fmeprot_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Fmeprot {
    #[inline(always)]
    fn default() -> Fmeprot {
        <crate::RegValueT<Fmeprot_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod fmeprot {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ceprot_SPEC;
    pub type Ceprot = crate::EnumBitfieldStruct<u8, Ceprot_SPEC>;
    impl Ceprot {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fbprot0_SPEC;
impl crate::sealed::RegSpec for Fbprot0_SPEC {
    type DataType = u16;
}

pub type Fbprot0 = crate::RegValueT<Fbprot0_SPEC>;

impl Fbprot0 {
    #[inline(always)]
    pub fn bpcn0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fbprot0::Bpcn0,
        fbprot0::Bpcn0,
        Fbprot0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fbprot0::Bpcn0,
            fbprot0::Bpcn0,
            Fbprot0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Fbprot0_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Fbprot0_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Fbprot0 {
    #[inline(always)]
    fn default() -> Fbprot0 {
        <crate::RegValueT<Fbprot0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fbprot0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bpcn0_SPEC;
    pub type Bpcn0 = crate::EnumBitfieldStruct<u8, Bpcn0_SPEC>;
    impl Bpcn0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fbprot1_SPEC;
impl crate::sealed::RegSpec for Fbprot1_SPEC {
    type DataType = u16;
}

pub type Fbprot1 = crate::RegValueT<Fbprot1_SPEC>;

impl Fbprot1 {
    #[inline(always)]
    pub fn bpcn1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fbprot1::Bpcn1,
        fbprot1::Bpcn1,
        Fbprot1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fbprot1::Bpcn1,
            fbprot1::Bpcn1,
            Fbprot1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Fbprot1_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Fbprot1_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Fbprot1 {
    #[inline(always)]
    fn default() -> Fbprot1 {
        <crate::RegValueT<Fbprot1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fbprot1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bpcn1_SPEC;
    pub type Bpcn1 = crate::EnumBitfieldStruct<u8, Bpcn1_SPEC>;
    impl Bpcn1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fstatr_SPEC;
impl crate::sealed::RegSpec for Fstatr_SPEC {
    type DataType = u32;
}

pub type Fstatr = crate::RegValueT<Fstatr_SPEC>;

impl Fstatr {
    #[inline(always)]
    pub fn flweerr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        fstatr::Flweerr,
        fstatr::Flweerr,
        Fstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            fstatr::Flweerr,
            fstatr::Flweerr,
            Fstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn prgspd(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        fstatr::Prgspd,
        fstatr::Prgspd,
        Fstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            fstatr::Prgspd,
            fstatr::Prgspd,
            Fstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ersspd(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        fstatr::Ersspd,
        fstatr::Ersspd,
        Fstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            fstatr::Ersspd,
            fstatr::Ersspd,
            Fstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dbfull(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        fstatr::Dbfull,
        fstatr::Dbfull,
        Fstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            fstatr::Dbfull,
            fstatr::Dbfull,
            Fstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn susrdy(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        fstatr::Susrdy,
        fstatr::Susrdy,
        Fstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            fstatr::Susrdy,
            fstatr::Susrdy,
            Fstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn prgerr(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        fstatr::Prgerr,
        fstatr::Prgerr,
        Fstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            fstatr::Prgerr,
            fstatr::Prgerr,
            Fstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn erserr(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        fstatr::Erserr,
        fstatr::Erserr,
        Fstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            fstatr::Erserr,
            fstatr::Erserr,
            Fstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ilglerr(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        fstatr::Ilglerr,
        fstatr::Ilglerr,
        Fstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            fstatr::Ilglerr,
            fstatr::Ilglerr,
            Fstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn frdy(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        fstatr::Frdy,
        fstatr::Frdy,
        Fstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            fstatr::Frdy,
            fstatr::Frdy,
            Fstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn oterr(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        fstatr::Oterr,
        fstatr::Oterr,
        Fstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            fstatr::Oterr,
            fstatr::Oterr,
            Fstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn secerr(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        fstatr::Secerr,
        fstatr::Secerr,
        Fstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            fstatr::Secerr,
            fstatr::Secerr,
            Fstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn feseterr(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        fstatr::Feseterr,
        fstatr::Feseterr,
        Fstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            fstatr::Feseterr,
            fstatr::Feseterr,
            Fstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ilgcomerr(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        fstatr::Ilgcomerr,
        fstatr::Ilgcomerr,
        Fstatr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            fstatr::Ilgcomerr,
            fstatr::Ilgcomerr,
            Fstatr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fstatr {
    #[inline(always)]
    fn default() -> Fstatr {
        <crate::RegValueT<Fstatr_SPEC> as RegisterValue<_>>::new(32768)
    }
}
pub mod fstatr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flweerr_SPEC;
    pub type Flweerr = crate::EnumBitfieldStruct<u8, Flweerr_SPEC>;
    impl Flweerr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prgspd_SPEC;
    pub type Prgspd = crate::EnumBitfieldStruct<u8, Prgspd_SPEC>;
    impl Prgspd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ersspd_SPEC;
    pub type Ersspd = crate::EnumBitfieldStruct<u8, Ersspd_SPEC>;
    impl Ersspd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dbfull_SPEC;
    pub type Dbfull = crate::EnumBitfieldStruct<u8, Dbfull_SPEC>;
    impl Dbfull {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Susrdy_SPEC;
    pub type Susrdy = crate::EnumBitfieldStruct<u8, Susrdy_SPEC>;
    impl Susrdy {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prgerr_SPEC;
    pub type Prgerr = crate::EnumBitfieldStruct<u8, Prgerr_SPEC>;
    impl Prgerr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Erserr_SPEC;
    pub type Erserr = crate::EnumBitfieldStruct<u8, Erserr_SPEC>;
    impl Erserr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ilglerr_SPEC;
    pub type Ilglerr = crate::EnumBitfieldStruct<u8, Ilglerr_SPEC>;
    impl Ilglerr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Frdy_SPEC;
    pub type Frdy = crate::EnumBitfieldStruct<u8, Frdy_SPEC>;
    impl Frdy {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oterr_SPEC;
    pub type Oterr = crate::EnumBitfieldStruct<u8, Oterr_SPEC>;
    impl Oterr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Secerr_SPEC;
    pub type Secerr = crate::EnumBitfieldStruct<u8, Secerr_SPEC>;
    impl Secerr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Feseterr_SPEC;
    pub type Feseterr = crate::EnumBitfieldStruct<u8, Feseterr_SPEC>;
    impl Feseterr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ilgcomerr_SPEC;
    pub type Ilgcomerr = crate::EnumBitfieldStruct<u8, Ilgcomerr_SPEC>;
    impl Ilgcomerr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fentryr_SPEC;
impl crate::sealed::RegSpec for Fentryr_SPEC {
    type DataType = u16;
}

pub type Fentryr = crate::RegValueT<Fentryr_SPEC>;

impl Fentryr {
    #[inline(always)]
    pub fn fentryc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fentryr::Fentryc,
        fentryr::Fentryc,
        Fentryr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fentryr::Fentryc,
            fentryr::Fentryc,
            Fentryr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fentryd(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        fentryr::Fentryd,
        fentryr::Fentryd,
        Fentryr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            fentryr::Fentryd,
            fentryr::Fentryd,
            Fentryr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Fentryr_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Fentryr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Fentryr {
    #[inline(always)]
    fn default() -> Fentryr {
        <crate::RegValueT<Fentryr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fentryr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fentryc_SPEC;
    pub type Fentryc = crate::EnumBitfieldStruct<u8, Fentryc_SPEC>;
    impl Fentryc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fentryd_SPEC;
    pub type Fentryd = crate::EnumBitfieldStruct<u8, Fentryd_SPEC>;
    impl Fentryd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsuinitr_SPEC;
impl crate::sealed::RegSpec for Fsuinitr_SPEC {
    type DataType = u16;
}

pub type Fsuinitr = crate::RegValueT<Fsuinitr_SPEC>;

impl Fsuinitr {
    #[inline(always)]
    pub fn suinit(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fsuinitr::Suinit,
        fsuinitr::Suinit,
        Fsuinitr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fsuinitr::Suinit,
            fsuinitr::Suinit,
            Fsuinitr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Fsuinitr_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Fsuinitr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Fsuinitr {
    #[inline(always)]
    fn default() -> Fsuinitr {
        <crate::RegValueT<Fsuinitr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fsuinitr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Suinit_SPEC;
    pub type Suinit = crate::EnumBitfieldStruct<u8, Suinit_SPEC>;
    impl Suinit {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcmdr_SPEC;
impl crate::sealed::RegSpec for Fcmdr_SPEC {
    type DataType = u16;
}

pub type Fcmdr = crate::RegValueT<Fcmdr_SPEC>;

impl Fcmdr {
    #[inline(always)]
    pub fn pcmdr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Fcmdr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Fcmdr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmdr(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Fcmdr_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Fcmdr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fcmdr {
    #[inline(always)]
    fn default() -> Fcmdr {
        <crate::RegValueT<Fcmdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fbccnt_SPEC;
impl crate::sealed::RegSpec for Fbccnt_SPEC {
    type DataType = u8;
}

pub type Fbccnt = crate::RegValueT<Fbccnt_SPEC>;

impl Fbccnt {
    #[inline(always)]
    pub fn bcdir(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fbccnt::Bcdir,
        fbccnt::Bcdir,
        Fbccnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fbccnt::Bcdir,
            fbccnt::Bcdir,
            Fbccnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fbccnt {
    #[inline(always)]
    fn default() -> Fbccnt {
        <crate::RegValueT<Fbccnt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fbccnt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bcdir_SPEC;
    pub type Bcdir = crate::EnumBitfieldStruct<u8, Bcdir_SPEC>;
    impl Bcdir {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fbcstat_SPEC;
impl crate::sealed::RegSpec for Fbcstat_SPEC {
    type DataType = u8;
}

pub type Fbcstat = crate::RegValueT<Fbcstat_SPEC>;

impl Fbcstat {
    #[inline(always)]
    pub fn bcst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fbcstat::Bcst,
        fbcstat::Bcst,
        Fbcstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fbcstat::Bcst,
            fbcstat::Bcst,
            Fbcstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fbcstat {
    #[inline(always)]
    fn default() -> Fbcstat {
        <crate::RegValueT<Fbcstat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fbcstat {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bcst_SPEC;
    pub type Bcst = crate::EnumBitfieldStruct<u8, Bcst_SPEC>;
    impl Bcst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fpsaddr_SPEC;
impl crate::sealed::RegSpec for Fpsaddr_SPEC {
    type DataType = u32;
}

pub type Fpsaddr = crate::RegValueT<Fpsaddr_SPEC>;

impl Fpsaddr {
    #[inline(always)]
    pub fn psadr(
        self,
    ) -> crate::common::RegisterField<0, 0x1ffff, 1, 0, u32, u32, Fpsaddr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1ffff,1,0,u32,u32,Fpsaddr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fpsaddr {
    #[inline(always)]
    fn default() -> Fpsaddr {
        <crate::RegValueT<Fpsaddr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsuasmon_SPEC;
impl crate::sealed::RegSpec for Fsuasmon_SPEC {
    type DataType = u32;
}

pub type Fsuasmon = crate::RegValueT<Fsuasmon_SPEC>;

impl Fsuasmon {
    #[inline(always)]
    pub fn fspr(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        fsuasmon::Fspr,
        fsuasmon::Fspr,
        Fsuasmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            fsuasmon::Fspr,
            fsuasmon::Fspr,
            Fsuasmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn btflg(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        fsuasmon::Btflg,
        fsuasmon::Btflg,
        Fsuasmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            fsuasmon::Btflg,
            fsuasmon::Btflg,
            Fsuasmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fsuasmon {
    #[inline(always)]
    fn default() -> Fsuasmon {
        <crate::RegValueT<Fsuasmon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fsuasmon {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fspr_SPEC;
    pub type Fspr = crate::EnumBitfieldStruct<u8, Fspr_SPEC>;
    impl Fspr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Btflg_SPEC;
    pub type Btflg = crate::EnumBitfieldStruct<u8, Btflg_SPEC>;
    impl Btflg {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcpsr_SPEC;
impl crate::sealed::RegSpec for Fcpsr_SPEC {
    type DataType = u16;
}

pub type Fcpsr = crate::RegValueT<Fcpsr_SPEC>;

impl Fcpsr {
    #[inline(always)]
    pub fn esuspmd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fcpsr::Esuspmd,
        fcpsr::Esuspmd,
        Fcpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fcpsr::Esuspmd,
            fcpsr::Esuspmd,
            Fcpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fcpsr {
    #[inline(always)]
    fn default() -> Fcpsr {
        <crate::RegValueT<Fcpsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fcpsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Esuspmd_SPEC;
    pub type Esuspmd = crate::EnumBitfieldStruct<u8, Esuspmd_SPEC>;
    impl Esuspmd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fpckar_SPEC;
impl crate::sealed::RegSpec for Fpckar_SPEC {
    type DataType = u16;
}

pub type Fpckar = crate::RegValueT<Fpckar_SPEC>;

impl Fpckar {
    #[inline(always)]
    pub fn pcka(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Fpckar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Fpckar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Fpckar_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Fpckar_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Fpckar {
    #[inline(always)]
    fn default() -> Fpckar {
        <crate::RegValueT<Fpckar_SPEC> as RegisterValue<_>>::new(50)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsuacr_SPEC;
impl crate::sealed::RegSpec for Fsuacr_SPEC {
    type DataType = u16;
}

pub type Fsuacr = crate::RegValueT<Fsuacr_SPEC>;

impl Fsuacr {
    #[inline(always)]
    pub fn sas(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        fsuacr::Sas,
        fsuacr::Sas,
        Fsuacr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            fsuacr::Sas,
            fsuacr::Sas,
            Fsuacr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Fsuacr_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Fsuacr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Fsuacr {
    #[inline(always)]
    fn default() -> Fsuacr {
        <crate::RegValueT<Fsuacr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fsuacr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sas_SPEC;
    pub type Sas = crate::EnumBitfieldStruct<u8, Sas_SPEC>;
    impl Sas {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
