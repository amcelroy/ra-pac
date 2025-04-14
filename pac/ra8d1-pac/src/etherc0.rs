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
#[doc = r"Ethernet Controller Channel 0"]
unsafe impl ::core::marker::Send for super::Etherc0 {}
unsafe impl ::core::marker::Sync for super::Etherc0 {}
impl super::Etherc0 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn ecmr(&self) -> &'static crate::common::Reg<self::Ecmr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ecmr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rflr(&self) -> &'static crate::common::Reg<self::Rflr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rflr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ecsr(&self) -> &'static crate::common::Reg<self::Ecsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ecsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ecsipr(
        &self,
    ) -> &'static crate::common::Reg<self::Ecsipr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ecsipr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pir(&self) -> &'static crate::common::Reg<self::Pir_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pir_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn psr(&self) -> &'static crate::common::Reg<self::Psr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Psr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rdmlr(&self) -> &'static crate::common::Reg<self::Rdmlr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rdmlr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ipgr(&self) -> &'static crate::common::Reg<self::Ipgr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ipgr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[inline(always)]
    pub const fn apr(&self) -> &'static crate::common::Reg<self::Apr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Apr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mpr(&self) -> &'static crate::common::Reg<self::Mpr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mpr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rfcf(&self) -> &'static crate::common::Reg<self::Rfcf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rfcf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tpauser(
        &self,
    ) -> &'static crate::common::Reg<self::Tpauser_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tpauser_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tpausecr(
        &self,
    ) -> &'static crate::common::Reg<self::Tpausecr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Tpausecr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bcfrr(&self) -> &'static crate::common::Reg<self::Bcfrr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcfrr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mahr(&self) -> &'static crate::common::Reg<self::Mahr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mahr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }

    #[inline(always)]
    pub const fn malr(&self) -> &'static crate::common::Reg<self::Malr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Malr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(200usize),
            )
        }
    }

    #[inline(always)]
    pub const fn trocr(&self) -> &'static crate::common::Reg<self::Trocr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Trocr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(208usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cdcr(&self) -> &'static crate::common::Reg<self::Cdcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(212usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lccr(&self) -> &'static crate::common::Reg<self::Lccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(216usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cndcr(&self) -> &'static crate::common::Reg<self::Cndcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cndcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(220usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cefcr(&self) -> &'static crate::common::Reg<self::Cefcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cefcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(228usize),
            )
        }
    }

    #[inline(always)]
    pub const fn frecr(&self) -> &'static crate::common::Reg<self::Frecr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Frecr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(232usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tsfrcr(
        &self,
    ) -> &'static crate::common::Reg<self::Tsfrcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tsfrcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(236usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tlfrcr(
        &self,
    ) -> &'static crate::common::Reg<self::Tlfrcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tlfrcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(240usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rfcr(&self) -> &'static crate::common::Reg<self::Rfcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rfcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(244usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mafcr(&self) -> &'static crate::common::Reg<self::Mafcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mafcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(248usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecmr_SPEC;
impl crate::sealed::RegSpec for Ecmr_SPEC {
    type DataType = u32;
}

pub type Ecmr = crate::RegValueT<Ecmr_SPEC>;

impl Ecmr {
    #[inline(always)]
    pub fn prm(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ecmr::Prm,
        ecmr::Prm,
        Ecmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ecmr::Prm,
            ecmr::Prm,
            Ecmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dm(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ecmr::Dm, ecmr::Dm, Ecmr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,ecmr::Dm,ecmr::Dm,Ecmr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rtm(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ecmr::Rtm,
        ecmr::Rtm,
        Ecmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ecmr::Rtm,
            ecmr::Rtm,
            Ecmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ilb(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ecmr::Ilb,
        ecmr::Ilb,
        Ecmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ecmr::Ilb,
            ecmr::Ilb,
            Ecmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn te(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, ecmr::Te, ecmr::Te, Ecmr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,ecmr::Te,ecmr::Te,Ecmr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn re(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, ecmr::Re, ecmr::Re, Ecmr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,ecmr::Re,ecmr::Re,Ecmr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mpde(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ecmr::Mpde,
        ecmr::Mpde,
        Ecmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ecmr::Mpde,
            ecmr::Mpde,
            Ecmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn prcef(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        ecmr::Prcef,
        ecmr::Prcef,
        Ecmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            ecmr::Prcef,
            ecmr::Prcef,
            Ecmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txf(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ecmr::Txf,
        ecmr::Txf,
        Ecmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ecmr::Txf,
            ecmr::Txf,
            Ecmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rxf(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        ecmr::Rxf,
        ecmr::Rxf,
        Ecmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            ecmr::Rxf,
            ecmr::Rxf,
            Ecmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pfr(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        ecmr::Pfr,
        ecmr::Pfr,
        Ecmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            ecmr::Pfr,
            ecmr::Pfr,
            Ecmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn zpf(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        ecmr::Zpf,
        ecmr::Zpf,
        Ecmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            ecmr::Zpf,
            ecmr::Zpf,
            Ecmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tpc(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        ecmr::Tpc,
        ecmr::Tpc,
        Ecmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            ecmr::Tpc,
            ecmr::Tpc,
            Ecmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<21, 0x7ff, 1, 0, u16, u16, Ecmr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<21,0x7ff,1,0,u16,u16,Ecmr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ecmr {
    #[inline(always)]
    fn default() -> Ecmr {
        <crate::RegValueT<Ecmr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ecmr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prm_SPEC;
    pub type Prm = crate::EnumBitfieldStruct<u8, Prm_SPEC>;
    impl Prm {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dm_SPEC;
    pub type Dm = crate::EnumBitfieldStruct<u8, Dm_SPEC>;
    impl Dm {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtm_SPEC;
    pub type Rtm = crate::EnumBitfieldStruct<u8, Rtm_SPEC>;
    impl Rtm {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ilb_SPEC;
    pub type Ilb = crate::EnumBitfieldStruct<u8, Ilb_SPEC>;
    impl Ilb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Te_SPEC;
    pub type Te = crate::EnumBitfieldStruct<u8, Te_SPEC>;
    impl Te {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Re_SPEC;
    pub type Re = crate::EnumBitfieldStruct<u8, Re_SPEC>;
    impl Re {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpde_SPEC;
    pub type Mpde = crate::EnumBitfieldStruct<u8, Mpde_SPEC>;
    impl Mpde {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prcef_SPEC;
    pub type Prcef = crate::EnumBitfieldStruct<u8, Prcef_SPEC>;
    impl Prcef {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txf_SPEC;
    pub type Txf = crate::EnumBitfieldStruct<u8, Txf_SPEC>;
    impl Txf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxf_SPEC;
    pub type Rxf = crate::EnumBitfieldStruct<u8, Rxf_SPEC>;
    impl Rxf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfr_SPEC;
    pub type Pfr = crate::EnumBitfieldStruct<u8, Pfr_SPEC>;
    impl Pfr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Zpf_SPEC;
    pub type Zpf = crate::EnumBitfieldStruct<u8, Zpf_SPEC>;
    impl Zpf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tpc_SPEC;
    pub type Tpc = crate::EnumBitfieldStruct<u8, Tpc_SPEC>;
    impl Tpc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rflr_SPEC;
impl crate::sealed::RegSpec for Rflr_SPEC {
    type DataType = u32;
}

pub type Rflr = crate::RegValueT<Rflr_SPEC>;

impl Rflr {
    #[inline(always)]
    pub fn rfl(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Rflr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Rflr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Rflr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Rflr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rflr {
    #[inline(always)]
    fn default() -> Rflr {
        <crate::RegValueT<Rflr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecsr_SPEC;
impl crate::sealed::RegSpec for Ecsr_SPEC {
    type DataType = u32;
}

pub type Ecsr = crate::RegValueT<Ecsr_SPEC>;

impl Ecsr {
    #[inline(always)]
    pub fn icd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ecsr::Icd,
        ecsr::Icd,
        Ecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ecsr::Icd,
            ecsr::Icd,
            Ecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mpd(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ecsr::Mpd,
        ecsr::Mpd,
        Ecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ecsr::Mpd,
            ecsr::Mpd,
            Ecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lchng(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ecsr::Lchng,
        ecsr::Lchng,
        Ecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ecsr::Lchng,
            ecsr::Lchng,
            Ecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psrto(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ecsr::Psrto,
        ecsr::Psrto,
        Ecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ecsr::Psrto,
            ecsr::Psrto,
            Ecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bfr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ecsr::Bfr,
        ecsr::Bfr,
        Ecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ecsr::Bfr,
            ecsr::Bfr,
            Ecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Ecsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Ecsr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ecsr {
    #[inline(always)]
    fn default() -> Ecsr {
        <crate::RegValueT<Ecsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ecsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icd_SPEC;
    pub type Icd = crate::EnumBitfieldStruct<u8, Icd_SPEC>;
    impl Icd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpd_SPEC;
    pub type Mpd = crate::EnumBitfieldStruct<u8, Mpd_SPEC>;
    impl Mpd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lchng_SPEC;
    pub type Lchng = crate::EnumBitfieldStruct<u8, Lchng_SPEC>;
    impl Lchng {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psrto_SPEC;
    pub type Psrto = crate::EnumBitfieldStruct<u8, Psrto_SPEC>;
    impl Psrto {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bfr_SPEC;
    pub type Bfr = crate::EnumBitfieldStruct<u8, Bfr_SPEC>;
    impl Bfr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecsipr_SPEC;
impl crate::sealed::RegSpec for Ecsipr_SPEC {
    type DataType = u32;
}

pub type Ecsipr = crate::RegValueT<Ecsipr_SPEC>;

impl Ecsipr {
    #[inline(always)]
    pub fn icdip(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ecsipr::Icdip,
        ecsipr::Icdip,
        Ecsipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ecsipr::Icdip,
            ecsipr::Icdip,
            Ecsipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mpdip(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ecsipr::Mpdip,
        ecsipr::Mpdip,
        Ecsipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ecsipr::Mpdip,
            ecsipr::Mpdip,
            Ecsipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lchngip(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ecsipr::Lchngip,
        ecsipr::Lchngip,
        Ecsipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ecsipr::Lchngip,
            ecsipr::Lchngip,
            Ecsipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psrtoip(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ecsipr::Psrtoip,
        ecsipr::Psrtoip,
        Ecsipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ecsipr::Psrtoip,
            ecsipr::Psrtoip,
            Ecsipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bfsipr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ecsipr::Bfsipr,
        ecsipr::Bfsipr,
        Ecsipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ecsipr::Bfsipr,
            ecsipr::Bfsipr,
            Ecsipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Ecsipr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Ecsipr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ecsipr {
    #[inline(always)]
    fn default() -> Ecsipr {
        <crate::RegValueT<Ecsipr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ecsipr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icdip_SPEC;
    pub type Icdip = crate::EnumBitfieldStruct<u8, Icdip_SPEC>;
    impl Icdip {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpdip_SPEC;
    pub type Mpdip = crate::EnumBitfieldStruct<u8, Mpdip_SPEC>;
    impl Mpdip {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lchngip_SPEC;
    pub type Lchngip = crate::EnumBitfieldStruct<u8, Lchngip_SPEC>;
    impl Lchngip {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psrtoip_SPEC;
    pub type Psrtoip = crate::EnumBitfieldStruct<u8, Psrtoip_SPEC>;
    impl Psrtoip {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bfsipr_SPEC;
    pub type Bfsipr = crate::EnumBitfieldStruct<u8, Bfsipr_SPEC>;
    impl Bfsipr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pir_SPEC;
impl crate::sealed::RegSpec for Pir_SPEC {
    type DataType = u32;
}

pub type Pir = crate::RegValueT<Pir_SPEC>;

impl Pir {
    #[inline(always)]
    pub fn mdc(self) -> crate::common::RegisterFieldBool<0, 1, 0, Pir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Pir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn mmd(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, pir::Mmd, pir::Mmd, Pir_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,pir::Mmd,pir::Mmd,Pir_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mdo(self) -> crate::common::RegisterFieldBool<2, 1, 0, Pir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Pir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn mdi(self) -> crate::common::RegisterFieldBool<3, 1, 0, Pir_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Pir_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Pir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Pir_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pir {
    #[inline(always)]
    fn default() -> Pir {
        <crate::RegValueT<Pir_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pir {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmd_SPEC;
    pub type Mmd = crate::EnumBitfieldStruct<u8, Mmd_SPEC>;
    impl Mmd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psr_SPEC;
impl crate::sealed::RegSpec for Psr_SPEC {
    type DataType = u32;
}

pub type Psr = crate::RegValueT<Psr_SPEC>;

impl Psr {
    #[inline(always)]
    pub fn lmon(self) -> crate::common::RegisterFieldBool<0, 1, 0, Psr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Psr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7fffffff, 1, 0, u32, u32, Psr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x7fffffff,1,0,u32,u32,Psr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Psr {
    #[inline(always)]
    fn default() -> Psr {
        <crate::RegValueT<Psr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdmlr_SPEC;
impl crate::sealed::RegSpec for Rdmlr_SPEC {
    type DataType = u32;
}

pub type Rdmlr = crate::RegValueT<Rdmlr_SPEC>;

impl Rdmlr {
    #[inline(always)]
    pub fn rmd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xfffff,
        1,
        0,
        rdmlr::Rmd,
        rdmlr::Rmd,
        Rdmlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xfffff,
            1,
            0,
            rdmlr::Rmd,
            rdmlr::Rmd,
            Rdmlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<20, 0xfff, 1, 0, u16, u16, Rdmlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0xfff,1,0,u16,u16,Rdmlr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rdmlr {
    #[inline(always)]
    fn default() -> Rdmlr {
        <crate::RegValueT<Rdmlr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rdmlr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmd_SPEC;
    pub type Rmd = crate::EnumBitfieldStruct<u8, Rmd_SPEC>;
    impl Rmd {
        pub const _00000_H: Self = Self::new(0);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipgr_SPEC;
impl crate::sealed::RegSpec for Ipgr_SPEC {
    type DataType = u32;
}

pub type Ipgr = crate::RegValueT<Ipgr_SPEC>;

impl Ipgr {
    #[inline(always)]
    pub fn ipg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        ipgr::Ipg,
        ipgr::Ipg,
        Ipgr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            ipgr::Ipg,
            ipgr::Ipg,
            Ipgr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7ffffff, 1, 0, u32, u32, Ipgr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x7ffffff,1,0,u32,u32,Ipgr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ipgr {
    #[inline(always)]
    fn default() -> Ipgr {
        <crate::RegValueT<Ipgr_SPEC> as RegisterValue<_>>::new(20)
    }
}
pub mod ipgr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ipg_SPEC;
    pub type Ipg = crate::EnumBitfieldStruct<u8, Ipg_SPEC>;
    impl Ipg {
        pub const _14_H: Self = Self::new(20);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apr_SPEC;
impl crate::sealed::RegSpec for Apr_SPEC {
    type DataType = u32;
}

pub type Apr = crate::RegValueT<Apr_SPEC>;

impl Apr {
    #[inline(always)]
    pub fn ap(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Apr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Apr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Apr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Apr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Apr {
    #[inline(always)]
    fn default() -> Apr {
        <crate::RegValueT<Apr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mpr_SPEC;
impl crate::sealed::RegSpec for Mpr_SPEC {
    type DataType = u32;
}

pub type Mpr = crate::RegValueT<Mpr_SPEC>;

impl Mpr {
    #[inline(always)]
    pub fn mp(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Mpr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Mpr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Mpr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Mpr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mpr {
    #[inline(always)]
    fn default() -> Mpr {
        <crate::RegValueT<Mpr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rfcf_SPEC;
impl crate::sealed::RegSpec for Rfcf_SPEC {
    type DataType = u32;
}

pub type Rfcf = crate::RegValueT<Rfcf_SPEC>;

impl Rfcf {
    #[inline(always)]
    pub fn rpause(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Rfcf_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Rfcf_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<8, 0xffffff, 1, 0, u32, u32, Rfcf_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xffffff,1,0,u32,u32,Rfcf_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Rfcf {
    #[inline(always)]
    fn default() -> Rfcf {
        <crate::RegValueT<Rfcf_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tpauser_SPEC;
impl crate::sealed::RegSpec for Tpauser_SPEC {
    type DataType = u32;
}

pub type Tpauser = crate::RegValueT<Tpauser_SPEC>;

impl Tpauser {
    #[inline(always)]
    pub fn tpause(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        tpauser::Tpause,
        tpauser::Tpause,
        Tpauser_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            tpauser::Tpause,
            tpauser::Tpause,
            Tpauser_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Tpauser_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Tpauser_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tpauser {
    #[inline(always)]
    fn default() -> Tpauser {
        <crate::RegValueT<Tpauser_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tpauser {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tpause_SPEC;
    pub type Tpause = crate::EnumBitfieldStruct<u8, Tpause_SPEC>;
    impl Tpause {
        pub const _0_X_0000: Self = Self::new(0);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tpausecr_SPEC;
impl crate::sealed::RegSpec for Tpausecr_SPEC {
    type DataType = u32;
}

pub type Tpausecr = crate::RegValueT<Tpausecr_SPEC>;

impl Tpausecr {
    #[inline(always)]
    pub fn txp(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Tpausecr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Tpausecr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<8, 0xffffff, 1, 0, u32, u32, Tpausecr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xffffff,1,0,u32,u32,Tpausecr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Tpausecr {
    #[inline(always)]
    fn default() -> Tpausecr {
        <crate::RegValueT<Tpausecr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcfrr_SPEC;
impl crate::sealed::RegSpec for Bcfrr_SPEC {
    type DataType = u32;
}

pub type Bcfrr = crate::RegValueT<Bcfrr_SPEC>;

impl Bcfrr {
    #[inline(always)]
    pub fn bcf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        bcfrr::Bcf,
        bcfrr::Bcf,
        Bcfrr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            bcfrr::Bcf,
            bcfrr::Bcf,
            Bcfrr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Bcfrr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Bcfrr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bcfrr {
    #[inline(always)]
    fn default() -> Bcfrr {
        <crate::RegValueT<Bcfrr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bcfrr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bcf_SPEC;
    pub type Bcf = crate::EnumBitfieldStruct<u8, Bcf_SPEC>;
    impl Bcf {
        pub const _0000_H: Self = Self::new(0);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mahr_SPEC;
impl crate::sealed::RegSpec for Mahr_SPEC {
    type DataType = u32;
}

pub type Mahr = crate::RegValueT<Mahr_SPEC>;

impl Mahr {
    #[inline(always)]
    pub fn mahr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mahr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mahr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mahr {
    #[inline(always)]
    fn default() -> Mahr {
        <crate::RegValueT<Mahr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Malr_SPEC;
impl crate::sealed::RegSpec for Malr_SPEC {
    type DataType = u32;
}

pub type Malr = crate::RegValueT<Malr_SPEC>;

impl Malr {
    #[inline(always)]
    pub fn malr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Malr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Malr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Malr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Malr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Malr {
    #[inline(always)]
    fn default() -> Malr {
        <crate::RegValueT<Malr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trocr_SPEC;
impl crate::sealed::RegSpec for Trocr_SPEC {
    type DataType = u32;
}

pub type Trocr = crate::RegValueT<Trocr_SPEC>;

impl Trocr {
    #[inline(always)]
    pub fn trocr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Trocr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Trocr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Trocr {
    #[inline(always)]
    fn default() -> Trocr {
        <crate::RegValueT<Trocr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdcr_SPEC;
impl crate::sealed::RegSpec for Cdcr_SPEC {
    type DataType = u32;
}

pub type Cdcr = crate::RegValueT<Cdcr_SPEC>;

impl Cdcr {
    #[inline(always)]
    pub fn cdcr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cdcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cdcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cdcr {
    #[inline(always)]
    fn default() -> Cdcr {
        <crate::RegValueT<Cdcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lccr_SPEC;
impl crate::sealed::RegSpec for Lccr_SPEC {
    type DataType = u32;
}

pub type Lccr = crate::RegValueT<Lccr_SPEC>;

impl Lccr {
    #[inline(always)]
    pub fn lccr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Lccr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Lccr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Lccr {
    #[inline(always)]
    fn default() -> Lccr {
        <crate::RegValueT<Lccr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cndcr_SPEC;
impl crate::sealed::RegSpec for Cndcr_SPEC {
    type DataType = u32;
}

pub type Cndcr = crate::RegValueT<Cndcr_SPEC>;

impl Cndcr {
    #[inline(always)]
    pub fn cndcr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cndcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cndcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cndcr {
    #[inline(always)]
    fn default() -> Cndcr {
        <crate::RegValueT<Cndcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cefcr_SPEC;
impl crate::sealed::RegSpec for Cefcr_SPEC {
    type DataType = u32;
}

pub type Cefcr = crate::RegValueT<Cefcr_SPEC>;

impl Cefcr {
    #[inline(always)]
    pub fn cefcr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cefcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cefcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cefcr {
    #[inline(always)]
    fn default() -> Cefcr {
        <crate::RegValueT<Cefcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frecr_SPEC;
impl crate::sealed::RegSpec for Frecr_SPEC {
    type DataType = u32;
}

pub type Frecr = crate::RegValueT<Frecr_SPEC>;

impl Frecr {
    #[inline(always)]
    pub fn frecr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Frecr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Frecr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Frecr {
    #[inline(always)]
    fn default() -> Frecr {
        <crate::RegValueT<Frecr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsfrcr_SPEC;
impl crate::sealed::RegSpec for Tsfrcr_SPEC {
    type DataType = u32;
}

pub type Tsfrcr = crate::RegValueT<Tsfrcr_SPEC>;

impl Tsfrcr {
    #[inline(always)]
    pub fn tsfrcr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Tsfrcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Tsfrcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tsfrcr {
    #[inline(always)]
    fn default() -> Tsfrcr {
        <crate::RegValueT<Tsfrcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tlfrcr_SPEC;
impl crate::sealed::RegSpec for Tlfrcr_SPEC {
    type DataType = u32;
}

pub type Tlfrcr = crate::RegValueT<Tlfrcr_SPEC>;

impl Tlfrcr {
    #[inline(always)]
    pub fn tlfrcr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Tlfrcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Tlfrcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tlfrcr {
    #[inline(always)]
    fn default() -> Tlfrcr {
        <crate::RegValueT<Tlfrcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rfcr_SPEC;
impl crate::sealed::RegSpec for Rfcr_SPEC {
    type DataType = u32;
}

pub type Rfcr = crate::RegValueT<Rfcr_SPEC>;

impl Rfcr {
    #[inline(always)]
    pub fn rfcr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Rfcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Rfcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rfcr {
    #[inline(always)]
    fn default() -> Rfcr {
        <crate::RegValueT<Rfcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mafcr_SPEC;
impl crate::sealed::RegSpec for Mafcr_SPEC {
    type DataType = u32;
}

pub type Mafcr = crate::RegValueT<Mafcr_SPEC>;

impl Mafcr {
    #[inline(always)]
    pub fn mafcr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mafcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mafcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mafcr {
    #[inline(always)]
    fn default() -> Mafcr {
        <crate::RegValueT<Mafcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
