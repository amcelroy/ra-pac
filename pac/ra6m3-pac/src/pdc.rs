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
#[doc = r"Parallel Data Capture Unit"]
unsafe impl ::core::marker::Send for super::Pdc {}
unsafe impl ::core::marker::Sync for super::Pdc {}
impl super::Pdc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn pccr0(&self) -> &'static crate::common::Reg<self::Pccr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pccr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pccr1(&self) -> &'static crate::common::Reg<self::Pccr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pccr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pcsr(&self) -> &'static crate::common::Reg<self::Pcsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pcsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pcmonr(&self) -> &'static crate::common::Reg<self::Pcmonr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Pcmonr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pcdr(&self) -> &'static crate::common::Reg<self::Pcdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Pcdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn vcr(&self) -> &'static crate::common::Reg<self::Vcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn hcr(&self) -> &'static crate::common::Reg<self::Hcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Hcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pccr0_SPEC;
impl crate::sealed::RegSpec for Pccr0_SPEC {
    type DataType = u32;
}

pub type Pccr0 = crate::RegValueT<Pccr0_SPEC>;

impl Pccr0 {
    #[inline(always)]
    pub fn eds(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        pccr0::Eds,
        pccr0::Eds,
        Pccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pccr0::Eds,
            pccr0::Eds,
            Pccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pckdiv(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x7,
        1,
        0,
        pccr0::Pckdiv,
        pccr0::Pckdiv,
        Pccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x7,
            1,
            0,
            pccr0::Pckdiv,
            pccr0::Pckdiv,
            Pccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pckoe(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        pccr0::Pckoe,
        pccr0::Pckoe,
        Pccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            pccr0::Pckoe,
            pccr0::Pckoe,
            Pccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn herie(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        pccr0::Herie,
        pccr0::Herie,
        Pccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            pccr0::Herie,
            pccr0::Herie,
            Pccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn verie(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        pccr0::Verie,
        pccr0::Verie,
        Pccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            pccr0::Verie,
            pccr0::Verie,
            Pccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn udrie(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        pccr0::Udrie,
        pccr0::Udrie,
        Pccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pccr0::Udrie,
            pccr0::Udrie,
            Pccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovie(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        pccr0::Ovie,
        pccr0::Ovie,
        Pccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pccr0::Ovie,
            pccr0::Ovie,
            Pccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn feie(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        pccr0::Feie,
        pccr0::Feie,
        Pccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            pccr0::Feie,
            pccr0::Feie,
            Pccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dfie(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        pccr0::Dfie,
        pccr0::Dfie,
        Pccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pccr0::Dfie,
            pccr0::Dfie,
            Pccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn prst(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        pccr0::Prst,
        pccr0::Prst,
        Pccr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            pccr0::Prst,
            pccr0::Prst,
            Pccr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn hps(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pccr0::Hps,
        pccr0::Hps,
        Pccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pccr0::Hps,
            pccr0::Hps,
            Pccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn vps(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pccr0::Vps,
        pccr0::Vps,
        Pccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pccr0::Vps,
            pccr0::Vps,
            Pccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pcke(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pccr0::Pcke,
        pccr0::Pcke,
        Pccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pccr0::Pcke,
            pccr0::Pcke,
            Pccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pccr0 {
    #[inline(always)]
    fn default() -> Pccr0 {
        <crate::RegValueT<Pccr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pccr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eds_SPEC;
    pub type Eds = crate::EnumBitfieldStruct<u8, Eds_SPEC>;
    impl Eds {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pckdiv_SPEC;
    pub type Pckdiv = crate::EnumBitfieldStruct<u8, Pckdiv_SPEC>;
    impl Pckdiv {
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
    pub struct Pckoe_SPEC;
    pub type Pckoe = crate::EnumBitfieldStruct<u8, Pckoe_SPEC>;
    impl Pckoe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Herie_SPEC;
    pub type Herie = crate::EnumBitfieldStruct<u8, Herie_SPEC>;
    impl Herie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Verie_SPEC;
    pub type Verie = crate::EnumBitfieldStruct<u8, Verie_SPEC>;
    impl Verie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Udrie_SPEC;
    pub type Udrie = crate::EnumBitfieldStruct<u8, Udrie_SPEC>;
    impl Udrie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovie_SPEC;
    pub type Ovie = crate::EnumBitfieldStruct<u8, Ovie_SPEC>;
    impl Ovie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Feie_SPEC;
    pub type Feie = crate::EnumBitfieldStruct<u8, Feie_SPEC>;
    impl Feie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dfie_SPEC;
    pub type Dfie = crate::EnumBitfieldStruct<u8, Dfie_SPEC>;
    impl Dfie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prst_SPEC;
    pub type Prst = crate::EnumBitfieldStruct<u8, Prst_SPEC>;
    impl Prst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hps_SPEC;
    pub type Hps = crate::EnumBitfieldStruct<u8, Hps_SPEC>;
    impl Hps {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vps_SPEC;
    pub type Vps = crate::EnumBitfieldStruct<u8, Vps_SPEC>;
    impl Vps {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcke_SPEC;
    pub type Pcke = crate::EnumBitfieldStruct<u8, Pcke_SPEC>;
    impl Pcke {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pccr1_SPEC;
impl crate::sealed::RegSpec for Pccr1_SPEC {
    type DataType = u32;
}

pub type Pccr1 = crate::RegValueT<Pccr1_SPEC>;

impl Pccr1 {
    #[inline(always)]
    pub fn pce(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pccr1::Pce,
        pccr1::Pce,
        Pccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pccr1::Pce,
            pccr1::Pce,
            Pccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pccr1 {
    #[inline(always)]
    fn default() -> Pccr1 {
        <crate::RegValueT<Pccr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pccr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pce_SPEC;
    pub type Pce = crate::EnumBitfieldStruct<u8, Pce_SPEC>;
    impl Pce {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcsr_SPEC;
impl crate::sealed::RegSpec for Pcsr_SPEC {
    type DataType = u32;
}

pub type Pcsr = crate::RegValueT<Pcsr_SPEC>;

impl Pcsr {
    #[inline(always)]
    pub fn herf(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        pcsr::Herf,
        pcsr::Herf,
        Pcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pcsr::Herf,
            pcsr::Herf,
            Pcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn verf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        pcsr::Verf,
        pcsr::Verf,
        Pcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            pcsr::Verf,
            pcsr::Verf,
            Pcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn udrf(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        pcsr::Udrf,
        pcsr::Udrf,
        Pcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pcsr::Udrf,
            pcsr::Udrf,
            Pcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovrf(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        pcsr::Ovrf,
        pcsr::Ovrf,
        Pcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            pcsr::Ovrf,
            pcsr::Ovrf,
            Pcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fef(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pcsr::Fef,
        pcsr::Fef,
        Pcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pcsr::Fef,
            pcsr::Fef,
            Pcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fempf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pcsr::Fempf,
        pcsr::Fempf,
        Pcsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pcsr::Fempf,
            pcsr::Fempf,
            Pcsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fbsy(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pcsr::Fbsy,
        pcsr::Fbsy,
        Pcsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pcsr::Fbsy,
            pcsr::Fbsy,
            Pcsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pcsr {
    #[inline(always)]
    fn default() -> Pcsr {
        <crate::RegValueT<Pcsr_SPEC> as RegisterValue<_>>::new(2)
    }
}
pub mod pcsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Herf_SPEC;
    pub type Herf = crate::EnumBitfieldStruct<u8, Herf_SPEC>;
    impl Herf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Verf_SPEC;
    pub type Verf = crate::EnumBitfieldStruct<u8, Verf_SPEC>;
    impl Verf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Udrf_SPEC;
    pub type Udrf = crate::EnumBitfieldStruct<u8, Udrf_SPEC>;
    impl Udrf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovrf_SPEC;
    pub type Ovrf = crate::EnumBitfieldStruct<u8, Ovrf_SPEC>;
    impl Ovrf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fef_SPEC;
    pub type Fef = crate::EnumBitfieldStruct<u8, Fef_SPEC>;
    impl Fef {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fempf_SPEC;
    pub type Fempf = crate::EnumBitfieldStruct<u8, Fempf_SPEC>;
    impl Fempf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fbsy_SPEC;
    pub type Fbsy = crate::EnumBitfieldStruct<u8, Fbsy_SPEC>;
    impl Fbsy {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcmonr_SPEC;
impl crate::sealed::RegSpec for Pcmonr_SPEC {
    type DataType = u32;
}

pub type Pcmonr = crate::RegValueT<Pcmonr_SPEC>;

impl Pcmonr {
    #[inline(always)]
    pub fn hsync(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pcmonr::Hsync,
        pcmonr::Hsync,
        Pcmonr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pcmonr::Hsync,
            pcmonr::Hsync,
            Pcmonr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn vsync(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pcmonr::Vsync,
        pcmonr::Vsync,
        Pcmonr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pcmonr::Vsync,
            pcmonr::Vsync,
            Pcmonr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pcmonr {
    #[inline(always)]
    fn default() -> Pcmonr {
        <crate::RegValueT<Pcmonr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pcmonr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hsync_SPEC;
    pub type Hsync = crate::EnumBitfieldStruct<u8, Hsync_SPEC>;
    impl Hsync {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vsync_SPEC;
    pub type Vsync = crate::EnumBitfieldStruct<u8, Vsync_SPEC>;
    impl Vsync {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcdr_SPEC;
impl crate::sealed::RegSpec for Pcdr_SPEC {
    type DataType = u32;
}

pub type Pcdr = crate::RegValueT<Pcdr_SPEC>;

impl Pcdr {
    #[inline(always)]
    pub fn pcdr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Pcdr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Pcdr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Pcdr {
    #[inline(always)]
    fn default() -> Pcdr {
        <crate::RegValueT<Pcdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vcr_SPEC;
impl crate::sealed::RegSpec for Vcr_SPEC {
    type DataType = u32;
}

pub type Vcr = crate::RegValueT<Vcr_SPEC>;

impl Vcr {
    #[inline(always)]
    pub fn vsz(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, u16, Vcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xfff,1,0,u16,u16,Vcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vst(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Vcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Vcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Vcr {
    #[inline(always)]
    fn default() -> Vcr {
        <crate::RegValueT<Vcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hcr_SPEC;
impl crate::sealed::RegSpec for Hcr_SPEC {
    type DataType = u32;
}

pub type Hcr = crate::RegValueT<Hcr_SPEC>;

impl Hcr {
    #[inline(always)]
    pub fn hsz(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, u16, Hcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xfff,1,0,u16,u16,Hcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn hst(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Hcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Hcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Hcr {
    #[inline(always)]
    fn default() -> Hcr {
        <crate::RegValueT<Hcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
