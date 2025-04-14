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
// Generated from SVD 1.1, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:53 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"OperationalAmplifier"]
unsafe impl ::core::marker::Send for super::Opamp {}
unsafe impl ::core::marker::Sync for super::Opamp {}
impl super::Opamp {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn ampmc(&self) -> &'static crate::common::Reg<self::Ampmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ampmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn amptrm(
        &self,
    ) -> &'static crate::common::Reg<self::Amptrm_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Amptrm_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1usize),
            )
        }
    }

    #[inline(always)]
    pub const fn amptrs(
        &self,
    ) -> &'static crate::common::Reg<self::Amptrs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Amptrs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ampc(&self) -> &'static crate::common::Reg<self::Ampc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ampc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ampmon(&self) -> &'static crate::common::Reg<self::Ampmon_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ampmon_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn amp0os(
        &self,
    ) -> &'static crate::common::Reg<self::Amp0Os_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Amp0Os_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[inline(always)]
    pub const fn amp0ms(
        &self,
    ) -> &'static crate::common::Reg<self::Amp0Ms_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Amp0Ms_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(7usize),
            )
        }
    }

    #[inline(always)]
    pub const fn amp0ps(
        &self,
    ) -> &'static crate::common::Reg<self::Amp0Ps_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Amp0Ps_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn amp1ms(
        &self,
    ) -> &'static crate::common::Reg<self::Amp1Ms_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Amp1Ms_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[inline(always)]
    pub const fn amp1ps(
        &self,
    ) -> &'static crate::common::Reg<self::Amp1Ps_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Amp1Ps_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(11usize),
            )
        }
    }

    #[inline(always)]
    pub const fn amp2ms(
        &self,
    ) -> &'static crate::common::Reg<self::Amp2Ms_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Amp2Ms_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(13usize),
            )
        }
    }

    #[inline(always)]
    pub const fn amp2ps(
        &self,
    ) -> &'static crate::common::Reg<self::Amp2Ps_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Amp2Ps_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ampcpc(
        &self,
    ) -> &'static crate::common::Reg<self::Ampcpc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ampcpc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ampuote(
        &self,
    ) -> &'static crate::common::Reg<self::Ampuote_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ampuote_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(23usize),
            )
        }
    }

    #[inline(always)]
    pub const fn amp0otp(
        &self,
    ) -> &'static crate::common::Reg<self::Amp0Otp_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Amp0Otp_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn amp0otn(
        &self,
    ) -> &'static crate::common::Reg<self::Amp0Otn_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Amp0Otn_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(25usize),
            )
        }
    }

    #[inline(always)]
    pub const fn amp1otp(
        &self,
    ) -> &'static crate::common::Reg<self::Amp1Otp_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Amp1Otp_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(26usize),
            )
        }
    }

    #[inline(always)]
    pub const fn amp1otn(
        &self,
    ) -> &'static crate::common::Reg<self::Amp1Otn_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Amp1Otn_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(27usize),
            )
        }
    }

    #[inline(always)]
    pub const fn amp2otp(
        &self,
    ) -> &'static crate::common::Reg<self::Amp2Otp_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Amp2Otp_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn amp2otn(
        &self,
    ) -> &'static crate::common::Reg<self::Amp2Otn_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Amp2Otn_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(29usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ampmc_SPEC;
impl crate::sealed::RegSpec for Ampmc_SPEC {
    type DataType = u8;
}

pub type Ampmc = crate::RegValueT<Ampmc_SPEC>;

impl Ampmc {
    #[inline(always)]
    pub fn ampsp(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        ampmc::Ampsp,
        ampmc::Ampsp,
        Ampmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            ampmc::Ampsp,
            ampmc::Ampsp,
            Ampmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Ampmc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Ampmc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ampmc {
    #[inline(always)]
    fn default() -> Ampmc {
        <crate::RegValueT<Ampmc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ampmc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampsp_SPEC;
    pub type Ampsp = crate::EnumBitfieldStruct<u8, Ampsp_SPEC>;
    impl Ampsp {
        pub const _00: Self = Self::new(0);

        pub const _10: Self = Self::new(2);

        pub const _01: Self = Self::new(1);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amptrm_SPEC;
impl crate::sealed::RegSpec for Amptrm_SPEC {
    type DataType = u8;
}

pub type Amptrm = crate::RegValueT<Amptrm_SPEC>;

impl Amptrm {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, Amptrm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,Amptrm_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn amptrm21(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        amptrm::Amptrm21,
        amptrm::Amptrm21,
        Amptrm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            amptrm::Amptrm21,
            amptrm::Amptrm21,
            Amptrm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn amptrm20(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        amptrm::Amptrm20,
        amptrm::Amptrm20,
        Amptrm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            amptrm::Amptrm20,
            amptrm::Amptrm20,
            Amptrm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn amptrm11(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        amptrm::Amptrm11,
        amptrm::Amptrm11,
        Amptrm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            amptrm::Amptrm11,
            amptrm::Amptrm11,
            Amptrm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn amptrm10(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        amptrm::Amptrm10,
        amptrm::Amptrm10,
        Amptrm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            amptrm::Amptrm10,
            amptrm::Amptrm10,
            Amptrm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn amptrm01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        amptrm::Amptrm01,
        amptrm::Amptrm01,
        Amptrm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            amptrm::Amptrm01,
            amptrm::Amptrm01,
            Amptrm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn amptrm00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        amptrm::Amptrm00,
        amptrm::Amptrm00,
        Amptrm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            amptrm::Amptrm00,
            amptrm::Amptrm00,
            Amptrm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Amptrm {
    #[inline(always)]
    fn default() -> Amptrm {
        <crate::RegValueT<Amptrm_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod amptrm {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Amptrm21_SPEC;
    pub type Amptrm21 = crate::EnumBitfieldStruct<u8, Amptrm21_SPEC>;
    impl Amptrm21 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Amptrm20_SPEC;
    pub type Amptrm20 = crate::EnumBitfieldStruct<u8, Amptrm20_SPEC>;
    impl Amptrm20 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Amptrm11_SPEC;
    pub type Amptrm11 = crate::EnumBitfieldStruct<u8, Amptrm11_SPEC>;
    impl Amptrm11 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Amptrm10_SPEC;
    pub type Amptrm10 = crate::EnumBitfieldStruct<u8, Amptrm10_SPEC>;
    impl Amptrm10 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Amptrm01_SPEC;
    pub type Amptrm01 = crate::EnumBitfieldStruct<u8, Amptrm01_SPEC>;
    impl Amptrm01 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Amptrm00_SPEC;
    pub type Amptrm00 = crate::EnumBitfieldStruct<u8, Amptrm00_SPEC>;
    impl Amptrm00 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amptrs_SPEC;
impl crate::sealed::RegSpec for Amptrs_SPEC {
    type DataType = u8;
}

pub type Amptrs = crate::RegValueT<Amptrs_SPEC>;

impl Amptrs {
    #[inline(always)]
    pub fn amptrs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        amptrs::Amptrs,
        amptrs::Amptrs,
        Amptrs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            amptrs::Amptrs,
            amptrs::Amptrs,
            Amptrs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Amptrs {
    #[inline(always)]
    fn default() -> Amptrs {
        <crate::RegValueT<Amptrs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod amptrs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Amptrs_SPEC;
    pub type Amptrs = crate::EnumBitfieldStruct<u8, Amptrs_SPEC>;
    impl Amptrs {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ampc_SPEC;
impl crate::sealed::RegSpec for Ampc_SPEC {
    type DataType = u8;
}

pub type Ampc = crate::RegValueT<Ampc_SPEC>;

impl Ampc {
    #[inline(always)]
    pub fn irefe(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ampc::Irefe,
        ampc::Irefe,
        Ampc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ampc::Irefe,
            ampc::Irefe,
            Ampc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0xf, 1, 0, u8, u8, Ampc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0xf,1,0,u8,u8,Ampc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ampe2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ampc::Ampe2,
        ampc::Ampe2,
        Ampc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ampc::Ampe2,
            ampc::Ampe2,
            Ampc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ampe1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ampc::Ampe1,
        ampc::Ampe1,
        Ampc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ampc::Ampe1,
            ampc::Ampe1,
            Ampc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ampe0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ampc::Ampe0,
        ampc::Ampe0,
        Ampc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ampc::Ampe0,
            ampc::Ampe0,
            Ampc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ampc {
    #[inline(always)]
    fn default() -> Ampc {
        <crate::RegValueT<Ampc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ampc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irefe_SPEC;
    pub type Irefe = crate::EnumBitfieldStruct<u8, Irefe_SPEC>;
    impl Irefe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampe2_SPEC;
    pub type Ampe2 = crate::EnumBitfieldStruct<u8, Ampe2_SPEC>;
    impl Ampe2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampe1_SPEC;
    pub type Ampe1 = crate::EnumBitfieldStruct<u8, Ampe1_SPEC>;
    impl Ampe1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampe0_SPEC;
    pub type Ampe0 = crate::EnumBitfieldStruct<u8, Ampe0_SPEC>;
    impl Ampe0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ampmon_SPEC;
impl crate::sealed::RegSpec for Ampmon_SPEC {
    type DataType = u8;
}

pub type Ampmon = crate::RegValueT<Ampmon_SPEC>;

impl Ampmon {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Ampmon_SPEC, crate::common::R> {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Ampmon_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ampmon2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ampmon::Ampmon2,
        ampmon::Ampmon2,
        Ampmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ampmon::Ampmon2,
            ampmon::Ampmon2,
            Ampmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ampmon1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ampmon::Ampmon1,
        ampmon::Ampmon1,
        Ampmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ampmon::Ampmon1,
            ampmon::Ampmon1,
            Ampmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ampmon0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ampmon::Ampmon0,
        ampmon::Ampmon0,
        Ampmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ampmon::Ampmon0,
            ampmon::Ampmon0,
            Ampmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ampmon {
    #[inline(always)]
    fn default() -> Ampmon {
        <crate::RegValueT<Ampmon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ampmon {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampmon2_SPEC;
    pub type Ampmon2 = crate::EnumBitfieldStruct<u8, Ampmon2_SPEC>;
    impl Ampmon2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampmon1_SPEC;
    pub type Ampmon1 = crate::EnumBitfieldStruct<u8, Ampmon1_SPEC>;
    impl Ampmon1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampmon0_SPEC;
    pub type Ampmon0 = crate::EnumBitfieldStruct<u8, Ampmon0_SPEC>;
    impl Ampmon0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amp0Os_SPEC;
impl crate::sealed::RegSpec for Amp0Os_SPEC {
    type DataType = u8;
}

pub type Amp0Os = crate::RegValueT<Amp0Os_SPEC>;

impl Amp0Os {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Amp0Os_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Amp0Os_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ampos3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        amp0os::Ampos3,
        amp0os::Ampos3,
        Amp0Os_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            amp0os::Ampos3,
            amp0os::Ampos3,
            Amp0Os_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ampos2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        amp0os::Ampos2,
        amp0os::Ampos2,
        Amp0Os_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            amp0os::Ampos2,
            amp0os::Ampos2,
            Amp0Os_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ampos1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        amp0os::Ampos1,
        amp0os::Ampos1,
        Amp0Os_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            amp0os::Ampos1,
            amp0os::Ampos1,
            Amp0Os_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ampos0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        amp0os::Ampos0,
        amp0os::Ampos0,
        Amp0Os_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            amp0os::Ampos0,
            amp0os::Ampos0,
            Amp0Os_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Amp0Os {
    #[inline(always)]
    fn default() -> Amp0Os {
        <crate::RegValueT<Amp0Os_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod amp0os {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampos3_SPEC;
    pub type Ampos3 = crate::EnumBitfieldStruct<u8, Ampos3_SPEC>;
    impl Ampos3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampos2_SPEC;
    pub type Ampos2 = crate::EnumBitfieldStruct<u8, Ampos2_SPEC>;
    impl Ampos2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampos1_SPEC;
    pub type Ampos1 = crate::EnumBitfieldStruct<u8, Ampos1_SPEC>;
    impl Ampos1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampos0_SPEC;
    pub type Ampos0 = crate::EnumBitfieldStruct<u8, Ampos0_SPEC>;
    impl Ampos0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amp0Ms_SPEC;
impl crate::sealed::RegSpec for Amp0Ms_SPEC {
    type DataType = u8;
}

pub type Amp0Ms = crate::RegValueT<Amp0Ms_SPEC>;

impl Amp0Ms {
    #[inline(always)]
    pub fn ampms7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        amp0ms::Ampms7,
        amp0ms::Ampms7,
        Amp0Ms_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            amp0ms::Ampms7,
            amp0ms::Ampms7,
            Amp0Ms_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, Amp0Ms_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,Amp0Ms_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ampms4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        amp0ms::Ampms4,
        amp0ms::Ampms4,
        Amp0Ms_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            amp0ms::Ampms4,
            amp0ms::Ampms4,
            Amp0Ms_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ampms3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        amp0ms::Ampms3,
        amp0ms::Ampms3,
        Amp0Ms_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            amp0ms::Ampms3,
            amp0ms::Ampms3,
            Amp0Ms_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ampms2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        amp0ms::Ampms2,
        amp0ms::Ampms2,
        Amp0Ms_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            amp0ms::Ampms2,
            amp0ms::Ampms2,
            Amp0Ms_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ampms1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        amp0ms::Ampms1,
        amp0ms::Ampms1,
        Amp0Ms_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            amp0ms::Ampms1,
            amp0ms::Ampms1,
            Amp0Ms_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ampms0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        amp0ms::Ampms0,
        amp0ms::Ampms0,
        Amp0Ms_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            amp0ms::Ampms0,
            amp0ms::Ampms0,
            Amp0Ms_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Amp0Ms {
    #[inline(always)]
    fn default() -> Amp0Ms {
        <crate::RegValueT<Amp0Ms_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod amp0ms {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampms7_SPEC;
    pub type Ampms7 = crate::EnumBitfieldStruct<u8, Ampms7_SPEC>;
    impl Ampms7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampms4_SPEC;
    pub type Ampms4 = crate::EnumBitfieldStruct<u8, Ampms4_SPEC>;
    impl Ampms4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampms3_SPEC;
    pub type Ampms3 = crate::EnumBitfieldStruct<u8, Ampms3_SPEC>;
    impl Ampms3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampms2_SPEC;
    pub type Ampms2 = crate::EnumBitfieldStruct<u8, Ampms2_SPEC>;
    impl Ampms2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampms1_SPEC;
    pub type Ampms1 = crate::EnumBitfieldStruct<u8, Ampms1_SPEC>;
    impl Ampms1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampms0_SPEC;
    pub type Ampms0 = crate::EnumBitfieldStruct<u8, Ampms0_SPEC>;
    impl Ampms0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amp0Ps_SPEC;
impl crate::sealed::RegSpec for Amp0Ps_SPEC {
    type DataType = u8;
}

pub type Amp0Ps = crate::RegValueT<Amp0Ps_SPEC>;

impl Amp0Ps {
    #[inline(always)]
    pub fn ampms7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        amp0ps::Ampms7,
        amp0ps::Ampms7,
        Amp0Ps_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            amp0ps::Ampms7,
            amp0ps::Ampms7,
            Amp0Ps_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Amp0Ps_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Amp0Ps_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ampps3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        amp0ps::Ampps3,
        amp0ps::Ampps3,
        Amp0Ps_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            amp0ps::Ampps3,
            amp0ps::Ampps3,
            Amp0Ps_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ampps2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        amp0ps::Ampps2,
        amp0ps::Ampps2,
        Amp0Ps_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            amp0ps::Ampps2,
            amp0ps::Ampps2,
            Amp0Ps_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ampps1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        amp0ps::Ampps1,
        amp0ps::Ampps1,
        Amp0Ps_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            amp0ps::Ampps1,
            amp0ps::Ampps1,
            Amp0Ps_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ampps0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        amp0ps::Ampps0,
        amp0ps::Ampps0,
        Amp0Ps_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            amp0ps::Ampps0,
            amp0ps::Ampps0,
            Amp0Ps_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Amp0Ps {
    #[inline(always)]
    fn default() -> Amp0Ps {
        <crate::RegValueT<Amp0Ps_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod amp0ps {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampms7_SPEC;
    pub type Ampms7 = crate::EnumBitfieldStruct<u8, Ampms7_SPEC>;
    impl Ampms7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampps3_SPEC;
    pub type Ampps3 = crate::EnumBitfieldStruct<u8, Ampps3_SPEC>;
    impl Ampps3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampps2_SPEC;
    pub type Ampps2 = crate::EnumBitfieldStruct<u8, Ampps2_SPEC>;
    impl Ampps2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampps1_SPEC;
    pub type Ampps1 = crate::EnumBitfieldStruct<u8, Ampps1_SPEC>;
    impl Ampps1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampps0_SPEC;
    pub type Ampps0 = crate::EnumBitfieldStruct<u8, Ampps0_SPEC>;
    impl Ampps0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amp1Ms_SPEC;
impl crate::sealed::RegSpec for Amp1Ms_SPEC {
    type DataType = u8;
}

pub type Amp1Ms = crate::RegValueT<Amp1Ms_SPEC>;

impl Amp1Ms {
    #[inline(always)]
    pub fn ampms7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        amp1ms::Ampms7,
        amp1ms::Ampms7,
        Amp1Ms_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            amp1ms::Ampms7,
            amp1ms::Ampms7,
            Amp1Ms_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x3f, 1, 0, u8, u8, Amp1Ms_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x3f,1,0,u8,u8,Amp1Ms_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ampms0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        amp1ms::Ampms0,
        amp1ms::Ampms0,
        Amp1Ms_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            amp1ms::Ampms0,
            amp1ms::Ampms0,
            Amp1Ms_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Amp1Ms {
    #[inline(always)]
    fn default() -> Amp1Ms {
        <crate::RegValueT<Amp1Ms_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod amp1ms {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampms7_SPEC;
    pub type Ampms7 = crate::EnumBitfieldStruct<u8, Ampms7_SPEC>;
    impl Ampms7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampms0_SPEC;
    pub type Ampms0 = crate::EnumBitfieldStruct<u8, Ampms0_SPEC>;
    impl Ampms0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amp1Ps_SPEC;
impl crate::sealed::RegSpec for Amp1Ps_SPEC {
    type DataType = u8;
}

pub type Amp1Ps = crate::RegValueT<Amp1Ps_SPEC>;

impl Amp1Ps {
    #[inline(always)]
    pub fn ampms7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        amp1ps::Ampms7,
        amp1ps::Ampms7,
        Amp1Ps_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            amp1ps::Ampms7,
            amp1ps::Ampms7,
            Amp1Ps_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Amp1Ps_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Amp1Ps_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ampps3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        amp1ps::Ampps3,
        amp1ps::Ampps3,
        Amp1Ps_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            amp1ps::Ampps3,
            amp1ps::Ampps3,
            Amp1Ps_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ampps2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        amp1ps::Ampps2,
        amp1ps::Ampps2,
        Amp1Ps_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            amp1ps::Ampps2,
            amp1ps::Ampps2,
            Amp1Ps_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ampps1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        amp1ps::Ampps1,
        amp1ps::Ampps1,
        Amp1Ps_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            amp1ps::Ampps1,
            amp1ps::Ampps1,
            Amp1Ps_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ampps0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        amp1ps::Ampps0,
        amp1ps::Ampps0,
        Amp1Ps_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            amp1ps::Ampps0,
            amp1ps::Ampps0,
            Amp1Ps_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Amp1Ps {
    #[inline(always)]
    fn default() -> Amp1Ps {
        <crate::RegValueT<Amp1Ps_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod amp1ps {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampms7_SPEC;
    pub type Ampms7 = crate::EnumBitfieldStruct<u8, Ampms7_SPEC>;
    impl Ampms7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampps3_SPEC;
    pub type Ampps3 = crate::EnumBitfieldStruct<u8, Ampps3_SPEC>;
    impl Ampps3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampps2_SPEC;
    pub type Ampps2 = crate::EnumBitfieldStruct<u8, Ampps2_SPEC>;
    impl Ampps2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampps1_SPEC;
    pub type Ampps1 = crate::EnumBitfieldStruct<u8, Ampps1_SPEC>;
    impl Ampps1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampps0_SPEC;
    pub type Ampps0 = crate::EnumBitfieldStruct<u8, Ampps0_SPEC>;
    impl Ampps0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amp2Ms_SPEC;
impl crate::sealed::RegSpec for Amp2Ms_SPEC {
    type DataType = u8;
}

pub type Amp2Ms = crate::RegValueT<Amp2Ms_SPEC>;

impl Amp2Ms {
    #[inline(always)]
    pub fn ampms7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        amp2ms::Ampms7,
        amp2ms::Ampms7,
        Amp2Ms_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            amp2ms::Ampms7,
            amp2ms::Ampms7,
            Amp2Ms_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x3f, 1, 0, u8, u8, Amp2Ms_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x3f,1,0,u8,u8,Amp2Ms_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ampms0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        amp2ms::Ampms0,
        amp2ms::Ampms0,
        Amp2Ms_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            amp2ms::Ampms0,
            amp2ms::Ampms0,
            Amp2Ms_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Amp2Ms {
    #[inline(always)]
    fn default() -> Amp2Ms {
        <crate::RegValueT<Amp2Ms_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod amp2ms {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampms7_SPEC;
    pub type Ampms7 = crate::EnumBitfieldStruct<u8, Ampms7_SPEC>;
    impl Ampms7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampms0_SPEC;
    pub type Ampms0 = crate::EnumBitfieldStruct<u8, Ampms0_SPEC>;
    impl Ampms0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amp2Ps_SPEC;
impl crate::sealed::RegSpec for Amp2Ps_SPEC {
    type DataType = u8;
}

pub type Amp2Ps = crate::RegValueT<Amp2Ps_SPEC>;

impl Amp2Ps {
    #[inline(always)]
    pub fn ampps7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        amp2ps::Ampps7,
        amp2ps::Ampps7,
        Amp2Ps_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            amp2ps::Ampps7,
            amp2ps::Ampps7,
            Amp2Ps_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, u8, Amp2Ps_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1f,1,0,u8,u8,Amp2Ps_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ampps1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        amp2ps::Ampps1,
        amp2ps::Ampps1,
        Amp2Ps_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            amp2ps::Ampps1,
            amp2ps::Ampps1,
            Amp2Ps_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ampps0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        amp2ps::Ampps0,
        amp2ps::Ampps0,
        Amp2Ps_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            amp2ps::Ampps0,
            amp2ps::Ampps0,
            Amp2Ps_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Amp2Ps {
    #[inline(always)]
    fn default() -> Amp2Ps {
        <crate::RegValueT<Amp2Ps_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod amp2ps {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampps7_SPEC;
    pub type Ampps7 = crate::EnumBitfieldStruct<u8, Ampps7_SPEC>;
    impl Ampps7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampps1_SPEC;
    pub type Ampps1 = crate::EnumBitfieldStruct<u8, Ampps1_SPEC>;
    impl Ampps1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ampps0_SPEC;
    pub type Ampps0 = crate::EnumBitfieldStruct<u8, Ampps0_SPEC>;
    impl Ampps0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ampcpc_SPEC;
impl crate::sealed::RegSpec for Ampcpc_SPEC {
    type DataType = u8;
}

pub type Ampcpc = crate::RegValueT<Ampcpc_SPEC>;

impl Ampcpc {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Ampcpc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Ampcpc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pump2en(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ampcpc::Pump2En,
        ampcpc::Pump2En,
        Ampcpc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ampcpc::Pump2En,
            ampcpc::Pump2En,
            Ampcpc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pump1en(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ampcpc::Pump1En,
        ampcpc::Pump1En,
        Ampcpc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ampcpc::Pump1En,
            ampcpc::Pump1En,
            Ampcpc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pump0en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ampcpc::Pump0En,
        ampcpc::Pump0En,
        Ampcpc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ampcpc::Pump0En,
            ampcpc::Pump0En,
            Ampcpc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ampcpc {
    #[inline(always)]
    fn default() -> Ampcpc {
        <crate::RegValueT<Ampcpc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ampcpc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pump2En_SPEC;
    pub type Pump2En = crate::EnumBitfieldStruct<u8, Pump2En_SPEC>;
    impl Pump2En {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pump1En_SPEC;
    pub type Pump1En = crate::EnumBitfieldStruct<u8, Pump1En_SPEC>;
    impl Pump1En {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pump0En_SPEC;
    pub type Pump0En = crate::EnumBitfieldStruct<u8, Pump0En_SPEC>;
    impl Pump0En {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ampuote_SPEC;
impl crate::sealed::RegSpec for Ampuote_SPEC {
    type DataType = u8;
}

pub type Ampuote = crate::RegValueT<Ampuote_SPEC>;

impl Ampuote {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Ampuote_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Ampuote_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn amp2te(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ampuote::Amp2Te,
        ampuote::Amp2Te,
        Ampuote_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ampuote::Amp2Te,
            ampuote::Amp2Te,
            Ampuote_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn amp1te(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ampuote::Amp1Te,
        ampuote::Amp1Te,
        Ampuote_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ampuote::Amp1Te,
            ampuote::Amp1Te,
            Ampuote_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn amp0te(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ampuote::Amp0Te,
        ampuote::Amp0Te,
        Ampuote_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ampuote::Amp0Te,
            ampuote::Amp0Te,
            Ampuote_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ampuote {
    #[inline(always)]
    fn default() -> Ampuote {
        <crate::RegValueT<Ampuote_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ampuote {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Amp2Te_SPEC;
    pub type Amp2Te = crate::EnumBitfieldStruct<u8, Amp2Te_SPEC>;
    impl Amp2Te {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Amp1Te_SPEC;
    pub type Amp1Te = crate::EnumBitfieldStruct<u8, Amp1Te_SPEC>;
    impl Amp1Te {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Amp0Te_SPEC;
    pub type Amp0Te = crate::EnumBitfieldStruct<u8, Amp0Te_SPEC>;
    impl Amp0Te {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amp0Otp_SPEC;
impl crate::sealed::RegSpec for Amp0Otp_SPEC {
    type DataType = u8;
}

pub type Amp0Otp = crate::RegValueT<Amp0Otp_SPEC>;

impl Amp0Otp {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, Amp0Otp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,Amp0Otp_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn trmp(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Amp0Otp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Amp0Otp_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Amp0Otp {
    #[inline(always)]
    fn default() -> Amp0Otp {
        <crate::RegValueT<Amp0Otp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amp0Otn_SPEC;
impl crate::sealed::RegSpec for Amp0Otn_SPEC {
    type DataType = u8;
}

pub type Amp0Otn = crate::RegValueT<Amp0Otn_SPEC>;

impl Amp0Otn {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, Amp0Otn_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,Amp0Otn_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn trmn(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Amp0Otn_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Amp0Otn_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Amp0Otn {
    #[inline(always)]
    fn default() -> Amp0Otn {
        <crate::RegValueT<Amp0Otn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amp1Otp_SPEC;
impl crate::sealed::RegSpec for Amp1Otp_SPEC {
    type DataType = u8;
}

pub type Amp1Otp = crate::RegValueT<Amp1Otp_SPEC>;

impl Amp1Otp {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, Amp1Otp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,Amp1Otp_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn trmp(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Amp1Otp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Amp1Otp_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Amp1Otp {
    #[inline(always)]
    fn default() -> Amp1Otp {
        <crate::RegValueT<Amp1Otp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amp1Otn_SPEC;
impl crate::sealed::RegSpec for Amp1Otn_SPEC {
    type DataType = u8;
}

pub type Amp1Otn = crate::RegValueT<Amp1Otn_SPEC>;

impl Amp1Otn {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, Amp1Otn_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,Amp1Otn_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn trmn(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Amp1Otn_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Amp1Otn_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Amp1Otn {
    #[inline(always)]
    fn default() -> Amp1Otn {
        <crate::RegValueT<Amp1Otn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amp2Otp_SPEC;
impl crate::sealed::RegSpec for Amp2Otp_SPEC {
    type DataType = u8;
}

pub type Amp2Otp = crate::RegValueT<Amp2Otp_SPEC>;

impl Amp2Otp {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, Amp2Otp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,Amp2Otp_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn trmp(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Amp2Otp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Amp2Otp_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Amp2Otp {
    #[inline(always)]
    fn default() -> Amp2Otp {
        <crate::RegValueT<Amp2Otp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amp2Otn_SPEC;
impl crate::sealed::RegSpec for Amp2Otn_SPEC {
    type DataType = u8;
}

pub type Amp2Otn = crate::RegValueT<Amp2Otn_SPEC>;

impl Amp2Otn {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, Amp2Otn_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,Amp2Otn_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn trmn(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Amp2Otn_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Amp2Otn_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Amp2Otn {
    #[inline(always)]
    fn default() -> Amp2Otn {
        <crate::RegValueT<Amp2Otn_SPEC> as RegisterValue<_>>::new(0)
    }
}
