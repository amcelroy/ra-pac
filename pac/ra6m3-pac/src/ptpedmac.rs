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
#[doc = r"DMA Controller for EPTPC"]
unsafe impl ::core::marker::Send for super::Ptpedmac {}
unsafe impl ::core::marker::Sync for super::Ptpedmac {}
impl super::Ptpedmac {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn edmr(&self) -> &'static crate::common::Reg<self::Edmr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Edmr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn edtrr(&self) -> &'static crate::common::Reg<self::Edtrr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Edtrr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn edrrr(&self) -> &'static crate::common::Reg<self::Edrrr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Edrrr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tdlar(&self) -> &'static crate::common::Reg<self::Tdlar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tdlar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rdlar(&self) -> &'static crate::common::Reg<self::Rdlar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rdlar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn eesr(&self) -> &'static crate::common::Reg<self::Eesr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eesr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn eesipr(
        &self,
    ) -> &'static crate::common::Reg<self::Eesipr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eesipr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rmfcr(&self) -> &'static crate::common::Reg<self::Rmfcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rmfcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tftr(&self) -> &'static crate::common::Reg<self::Tftr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tftr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fdr(&self) -> &'static crate::common::Reg<self::Fdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rmcr(&self) -> &'static crate::common::Reg<self::Rmcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rmcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tfucr(&self) -> &'static crate::common::Reg<self::Tfucr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tfucr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rfocr(&self) -> &'static crate::common::Reg<self::Rfocr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rfocr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[inline(always)]
    pub const fn iosr(&self) -> &'static crate::common::Reg<self::Iosr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iosr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fcftr(&self) -> &'static crate::common::Reg<self::Fcftr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fcftr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rpadir(
        &self,
    ) -> &'static crate::common::Reg<self::Rpadir_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rpadir_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[inline(always)]
    pub const fn trimd(&self) -> &'static crate::common::Reg<self::Trimd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Trimd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rbwar(&self) -> &'static crate::common::Reg<self::Rbwar_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rbwar_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(200usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rdfar(&self) -> &'static crate::common::Reg<self::Rdfar_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rdfar_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(204usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tbrar(&self) -> &'static crate::common::Reg<self::Tbrar_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Tbrar_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(212usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tdfar(&self) -> &'static crate::common::Reg<self::Tdfar_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Tdfar_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(216usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Edmr_SPEC;
impl crate::sealed::RegSpec for Edmr_SPEC {
    type DataType = u32;
}

pub type Edmr = crate::RegValueT<Edmr_SPEC>;

impl Edmr {
    #[inline(always)]
    pub fn de(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, edmr::De, edmr::De, Edmr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,edmr::De,edmr::De,Edmr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dl(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, edmr::Dl, edmr::Dl, Edmr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,edmr::Dl,edmr::Dl,Edmr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn swr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        edmr::Swr,
        edmr::Swr,
        Edmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            edmr::Swr,
            edmr::Swr,
            Edmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Edmr {
    #[inline(always)]
    fn default() -> Edmr {
        <crate::RegValueT<Edmr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod edmr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct De_SPEC;
    pub type De = crate::EnumBitfieldStruct<u8, De_SPEC>;
    impl De {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dl_SPEC;
    pub type Dl = crate::EnumBitfieldStruct<u8, Dl_SPEC>;
    impl Dl {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Swr_SPEC;
    pub type Swr = crate::EnumBitfieldStruct<u8, Swr_SPEC>;
    impl Swr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Edtrr_SPEC;
impl crate::sealed::RegSpec for Edtrr_SPEC {
    type DataType = u32;
}

pub type Edtrr = crate::RegValueT<Edtrr_SPEC>;

impl Edtrr {
    #[inline(always)]
    pub fn tr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        edtrr::Tr,
        edtrr::Tr,
        Edtrr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            edtrr::Tr,
            edtrr::Tr,
            Edtrr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Edtrr {
    #[inline(always)]
    fn default() -> Edtrr {
        <crate::RegValueT<Edtrr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod edtrr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tr_SPEC;
    pub type Tr = crate::EnumBitfieldStruct<u8, Tr_SPEC>;
    impl Tr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Edrrr_SPEC;
impl crate::sealed::RegSpec for Edrrr_SPEC {
    type DataType = u32;
}

pub type Edrrr = crate::RegValueT<Edrrr_SPEC>;

impl Edrrr {
    #[inline(always)]
    pub fn rr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        edrrr::Rr,
        edrrr::Rr,
        Edrrr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            edrrr::Rr,
            edrrr::Rr,
            Edrrr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Edrrr {
    #[inline(always)]
    fn default() -> Edrrr {
        <crate::RegValueT<Edrrr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod edrrr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rr_SPEC;
    pub type Rr = crate::EnumBitfieldStruct<u8, Rr_SPEC>;
    impl Rr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdlar_SPEC;
impl crate::sealed::RegSpec for Tdlar_SPEC {
    type DataType = u32;
}

pub type Tdlar = crate::RegValueT<Tdlar_SPEC>;

impl Tdlar {
    #[inline(always)]
    pub fn tdlar(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Tdlar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Tdlar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tdlar {
    #[inline(always)]
    fn default() -> Tdlar {
        <crate::RegValueT<Tdlar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdlar_SPEC;
impl crate::sealed::RegSpec for Rdlar_SPEC {
    type DataType = u32;
}

pub type Rdlar = crate::RegValueT<Rdlar_SPEC>;

impl Rdlar {
    #[inline(always)]
    pub fn rdlar(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Rdlar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Rdlar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rdlar {
    #[inline(always)]
    fn default() -> Rdlar {
        <crate::RegValueT<Rdlar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eesr_SPEC;
impl crate::sealed::RegSpec for Eesr_SPEC {
    type DataType = u32;
}

pub type Eesr = crate::RegValueT<Eesr_SPEC>;

impl Eesr {
    #[inline(always)]
    pub fn twb(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        eesr::Twb,
        eesr::Twb,
        Eesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            eesr::Twb,
            eesr::Twb,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tabt(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        eesr::Tabt,
        eesr::Tabt,
        Eesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            eesr::Tabt,
            eesr::Tabt,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfcof(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        eesr::Rfcof,
        eesr::Rfcof,
        Eesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            eesr::Rfcof,
            eesr::Rfcof,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ade(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        eesr::Ade,
        eesr::Ade,
        Eesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            eesr::Ade,
            eesr::Ade,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tc(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, eesr::Tc, eesr::Tc, Eesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            eesr::Tc,
            eesr::Tc,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tde(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        eesr::Tde,
        eesr::Tde,
        Eesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            eesr::Tde,
            eesr::Tde,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tfuf(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        eesr::Tfuf,
        eesr::Tfuf,
        Eesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            eesr::Tfuf,
            eesr::Tfuf,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fr(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, eesr::Fr, eesr::Fr, Eesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            eesr::Fr,
            eesr::Fr,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rde(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        eesr::Rde,
        eesr::Rde,
        Eesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            eesr::Rde,
            eesr::Rde,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfof(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        eesr::Rfof,
        eesr::Rfof,
        Eesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            eesr::Rfof,
            eesr::Rfof,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mace(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        eesr::Mace,
        eesr::Mace,
        Eesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            eesr::Mace,
            eesr::Mace,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pver(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        eesr::Pver,
        eesr::Pver,
        Eesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            eesr::Pver,
            eesr::Pver,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn r#type(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        eesr::Type,
        eesr::Type,
        Eesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            eesr::Type,
            eesr::Type,
            Eesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eesr {
    #[inline(always)]
    fn default() -> Eesr {
        <crate::RegValueT<Eesr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eesr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Twb_SPEC;
    pub type Twb = crate::EnumBitfieldStruct<u8, Twb_SPEC>;
    impl Twb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tabt_SPEC;
    pub type Tabt = crate::EnumBitfieldStruct<u8, Tabt_SPEC>;
    impl Tabt {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfcof_SPEC;
    pub type Rfcof = crate::EnumBitfieldStruct<u8, Rfcof_SPEC>;
    impl Rfcof {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ade_SPEC;
    pub type Ade = crate::EnumBitfieldStruct<u8, Ade_SPEC>;
    impl Ade {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tc_SPEC;
    pub type Tc = crate::EnumBitfieldStruct<u8, Tc_SPEC>;
    impl Tc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tde_SPEC;
    pub type Tde = crate::EnumBitfieldStruct<u8, Tde_SPEC>;
    impl Tde {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tfuf_SPEC;
    pub type Tfuf = crate::EnumBitfieldStruct<u8, Tfuf_SPEC>;
    impl Tfuf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fr_SPEC;
    pub type Fr = crate::EnumBitfieldStruct<u8, Fr_SPEC>;
    impl Fr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rde_SPEC;
    pub type Rde = crate::EnumBitfieldStruct<u8, Rde_SPEC>;
    impl Rde {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfof_SPEC;
    pub type Rfof = crate::EnumBitfieldStruct<u8, Rfof_SPEC>;
    impl Rfof {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mace_SPEC;
    pub type Mace = crate::EnumBitfieldStruct<u8, Mace_SPEC>;
    impl Mace {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pver_SPEC;
    pub type Pver = crate::EnumBitfieldStruct<u8, Pver_SPEC>;
    impl Pver {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Type_SPEC;
    pub type Type = crate::EnumBitfieldStruct<u8, Type_SPEC>;
    impl Type {
        pub const _0000: Self = Self::new(0);

        pub const _0001: Self = Self::new(1);

        pub const _0010: Self = Self::new(2);

        pub const _0011: Self = Self::new(3);

        pub const _1000: Self = Self::new(8);

        pub const _1001: Self = Self::new(9);

        pub const _1010: Self = Self::new(10);

        pub const _1011: Self = Self::new(11);

        pub const _1100: Self = Self::new(12);

        pub const _1101: Self = Self::new(13);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eesipr_SPEC;
impl crate::sealed::RegSpec for Eesipr_SPEC {
    type DataType = u32;
}

pub type Eesipr = crate::RegValueT<Eesipr_SPEC>;

impl Eesipr {
    #[inline(always)]
    pub fn twbip(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        eesipr::Twbip,
        eesipr::Twbip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            eesipr::Twbip,
            eesipr::Twbip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tabtip(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        eesipr::Tabtip,
        eesipr::Tabtip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            eesipr::Tabtip,
            eesipr::Tabtip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfcofip(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        eesipr::Rfcofip,
        eesipr::Rfcofip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            eesipr::Rfcofip,
            eesipr::Rfcofip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adeip(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        eesipr::Adeip,
        eesipr::Adeip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            eesipr::Adeip,
            eesipr::Adeip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tcip(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        eesipr::Tcip,
        eesipr::Tcip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            eesipr::Tcip,
            eesipr::Tcip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tdeip(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        eesipr::Tdeip,
        eesipr::Tdeip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            eesipr::Tdeip,
            eesipr::Tdeip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tfufip(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        eesipr::Tfufip,
        eesipr::Tfufip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            eesipr::Tfufip,
            eesipr::Tfufip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn frip(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        eesipr::Frip,
        eesipr::Frip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            eesipr::Frip,
            eesipr::Frip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rdeip(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        eesipr::Rdeip,
        eesipr::Rdeip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            eesipr::Rdeip,
            eesipr::Rdeip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfofip(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        eesipr::Rfofip,
        eesipr::Rfofip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            eesipr::Rfofip,
            eesipr::Rfofip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn maceip(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        eesipr::Maceip,
        eesipr::Maceip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            eesipr::Maceip,
            eesipr::Maceip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pverip(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        eesipr::Pverip,
        eesipr::Pverip,
        Eesipr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            eesipr::Pverip,
            eesipr::Pverip,
            Eesipr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eesipr {
    #[inline(always)]
    fn default() -> Eesipr {
        <crate::RegValueT<Eesipr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eesipr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Twbip_SPEC;
    pub type Twbip = crate::EnumBitfieldStruct<u8, Twbip_SPEC>;
    impl Twbip {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tabtip_SPEC;
    pub type Tabtip = crate::EnumBitfieldStruct<u8, Tabtip_SPEC>;
    impl Tabtip {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfcofip_SPEC;
    pub type Rfcofip = crate::EnumBitfieldStruct<u8, Rfcofip_SPEC>;
    impl Rfcofip {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adeip_SPEC;
    pub type Adeip = crate::EnumBitfieldStruct<u8, Adeip_SPEC>;
    impl Adeip {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcip_SPEC;
    pub type Tcip = crate::EnumBitfieldStruct<u8, Tcip_SPEC>;
    impl Tcip {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdeip_SPEC;
    pub type Tdeip = crate::EnumBitfieldStruct<u8, Tdeip_SPEC>;
    impl Tdeip {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tfufip_SPEC;
    pub type Tfufip = crate::EnumBitfieldStruct<u8, Tfufip_SPEC>;
    impl Tfufip {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Frip_SPEC;
    pub type Frip = crate::EnumBitfieldStruct<u8, Frip_SPEC>;
    impl Frip {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdeip_SPEC;
    pub type Rdeip = crate::EnumBitfieldStruct<u8, Rdeip_SPEC>;
    impl Rdeip {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfofip_SPEC;
    pub type Rfofip = crate::EnumBitfieldStruct<u8, Rfofip_SPEC>;
    impl Rfofip {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Maceip_SPEC;
    pub type Maceip = crate::EnumBitfieldStruct<u8, Maceip_SPEC>;
    impl Maceip {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pverip_SPEC;
    pub type Pverip = crate::EnumBitfieldStruct<u8, Pverip_SPEC>;
    impl Pverip {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rmfcr_SPEC;
impl crate::sealed::RegSpec for Rmfcr_SPEC {
    type DataType = u32;
}

pub type Rmfcr = crate::RegValueT<Rmfcr_SPEC>;

impl Rmfcr {
    #[inline(always)]
    pub fn mfc(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Rmfcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Rmfcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rmfcr {
    #[inline(always)]
    fn default() -> Rmfcr {
        <crate::RegValueT<Rmfcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tftr_SPEC;
impl crate::sealed::RegSpec for Tftr_SPEC {
    type DataType = u32;
}

pub type Tftr = crate::RegValueT<Tftr_SPEC>;

impl Tftr {
    #[inline(always)]
    pub fn tft(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        tftr::Tft,
        tftr::Tft,
        Tftr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            tftr::Tft,
            tftr::Tft,
            Tftr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tftr {
    #[inline(always)]
    fn default() -> Tftr {
        <crate::RegValueT<Tftr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tftr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tft_SPEC;
    pub type Tft = crate::EnumBitfieldStruct<u8, Tft_SPEC>;
    impl Tft {
        pub const _0_X_000: Self = Self::new(0);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdr_SPEC;
impl crate::sealed::RegSpec for Fdr_SPEC {
    type DataType = u32;
}

pub type Fdr = crate::RegValueT<Fdr_SPEC>;

impl Fdr {
    #[inline(always)]
    pub fn tfd(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, fdr::Tfd, fdr::Tfd, Fdr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1f,1,0,fdr::Tfd,fdr::Tfd,Fdr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfd(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, fdr::Rfd, fdr::Rfd, Fdr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,fdr::Rfd,fdr::Rfd,Fdr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fdr {
    #[inline(always)]
    fn default() -> Fdr {
        <crate::RegValueT<Fdr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fdr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tfd_SPEC;
    pub type Tfd = crate::EnumBitfieldStruct<u8, Tfd_SPEC>;
    impl Tfd {
        pub const _01111: Self = Self::new(15);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfd_SPEC;
    pub type Rfd = crate::EnumBitfieldStruct<u8, Rfd_SPEC>;
    impl Rfd {
        pub const _00111: Self = Self::new(7);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rmcr_SPEC;
impl crate::sealed::RegSpec for Rmcr_SPEC {
    type DataType = u32;
}

pub type Rmcr = crate::RegValueT<Rmcr_SPEC>;

impl Rmcr {
    #[inline(always)]
    pub fn rnr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        rmcr::Rnr,
        rmcr::Rnr,
        Rmcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            rmcr::Rnr,
            rmcr::Rnr,
            Rmcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rmcr {
    #[inline(always)]
    fn default() -> Rmcr {
        <crate::RegValueT<Rmcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rmcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rnr_SPEC;
    pub type Rnr = crate::EnumBitfieldStruct<u8, Rnr_SPEC>;
    impl Rnr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tfucr_SPEC;
impl crate::sealed::RegSpec for Tfucr_SPEC {
    type DataType = u32;
}

pub type Tfucr = crate::RegValueT<Tfucr_SPEC>;

impl Tfucr {
    #[inline(always)]
    pub fn under(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Tfucr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Tfucr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tfucr {
    #[inline(always)]
    fn default() -> Tfucr {
        <crate::RegValueT<Tfucr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rfocr_SPEC;
impl crate::sealed::RegSpec for Rfocr_SPEC {
    type DataType = u32;
}

pub type Rfocr = crate::RegValueT<Rfocr_SPEC>;

impl Rfocr {
    #[inline(always)]
    pub fn over(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Rfocr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Rfocr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rfocr {
    #[inline(always)]
    fn default() -> Rfocr {
        <crate::RegValueT<Rfocr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iosr_SPEC;
impl crate::sealed::RegSpec for Iosr_SPEC {
    type DataType = u32;
}

pub type Iosr = crate::RegValueT<Iosr_SPEC>;

impl Iosr {
    #[inline(always)]
    pub fn elb(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        iosr::Elb,
        iosr::Elb,
        Iosr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            iosr::Elb,
            iosr::Elb,
            Iosr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iosr {
    #[inline(always)]
    fn default() -> Iosr {
        <crate::RegValueT<Iosr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iosr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elb_SPEC;
    pub type Elb = crate::EnumBitfieldStruct<u8, Elb_SPEC>;
    impl Elb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcftr_SPEC;
impl crate::sealed::RegSpec for Fcftr_SPEC {
    type DataType = u32;
}

pub type Fcftr = crate::RegValueT<Fcftr_SPEC>;

impl Fcftr {
    #[inline(always)]
    pub fn rffo(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        fcftr::Rffo,
        fcftr::Rffo,
        Fcftr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            fcftr::Rffo,
            fcftr::Rffo,
            Fcftr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdo(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        fcftr::Rfdo,
        fcftr::Rfdo,
        Fcftr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            fcftr::Rfdo,
            fcftr::Rfdo,
            Fcftr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fcftr {
    #[inline(always)]
    fn default() -> Fcftr {
        <crate::RegValueT<Fcftr_SPEC> as RegisterValue<_>>::new(458759)
    }
}
pub mod fcftr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rffo_SPEC;
    pub type Rffo = crate::EnumBitfieldStruct<u8, Rffo_SPEC>;
    impl Rffo {
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
    pub struct Rfdo_SPEC;
    pub type Rfdo = crate::EnumBitfieldStruct<u8, Rfdo_SPEC>;
    impl Rfdo {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _101: Self = Self::new(5);

        pub const _110: Self = Self::new(6);

        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rpadir_SPEC;
impl crate::sealed::RegSpec for Rpadir_SPEC {
    type DataType = u32;
}

pub type Rpadir = crate::RegValueT<Rpadir_SPEC>;

impl Rpadir {
    #[inline(always)]
    pub fn pads(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        rpadir::Pads,
        rpadir::Pads,
        Rpadir_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            rpadir::Pads,
            rpadir::Pads,
            Rpadir_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn padr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        rpadir::Padr,
        rpadir::Padr,
        Rpadir_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            rpadir::Padr,
            rpadir::Padr,
            Rpadir_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rpadir {
    #[inline(always)]
    fn default() -> Rpadir {
        <crate::RegValueT<Rpadir_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rpadir {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pads_SPEC;
    pub type Pads = crate::EnumBitfieldStruct<u8, Pads_SPEC>;
    impl Pads {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padr_SPEC;
    pub type Padr = crate::EnumBitfieldStruct<u8, Padr_SPEC>;
    impl Padr {
        pub const _00_H: Self = Self::new(0);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trimd_SPEC;
impl crate::sealed::RegSpec for Trimd_SPEC {
    type DataType = u32;
}

pub type Trimd = crate::RegValueT<Trimd_SPEC>;

impl Trimd {
    #[inline(always)]
    pub fn tim(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        trimd::Tim,
        trimd::Tim,
        Trimd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            trimd::Tim,
            trimd::Tim,
            Trimd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tis(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        trimd::Tis,
        trimd::Tis,
        Trimd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            trimd::Tis,
            trimd::Tis,
            Trimd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Trimd {
    #[inline(always)]
    fn default() -> Trimd {
        <crate::RegValueT<Trimd_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod trimd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tim_SPEC;
    pub type Tim = crate::EnumBitfieldStruct<u8, Tim_SPEC>;
    impl Tim {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tis_SPEC;
    pub type Tis = crate::EnumBitfieldStruct<u8, Tis_SPEC>;
    impl Tis {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rbwar_SPEC;
impl crate::sealed::RegSpec for Rbwar_SPEC {
    type DataType = u32;
}

pub type Rbwar = crate::RegValueT<Rbwar_SPEC>;

impl Rbwar {
    #[inline(always)]
    pub fn rbwar(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Rbwar_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Rbwar_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Rbwar {
    #[inline(always)]
    fn default() -> Rbwar {
        <crate::RegValueT<Rbwar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdfar_SPEC;
impl crate::sealed::RegSpec for Rdfar_SPEC {
    type DataType = u32;
}

pub type Rdfar = crate::RegValueT<Rdfar_SPEC>;

impl Rdfar {
    #[inline(always)]
    pub fn rdfar(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Rdfar_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Rdfar_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Rdfar {
    #[inline(always)]
    fn default() -> Rdfar {
        <crate::RegValueT<Rdfar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tbrar_SPEC;
impl crate::sealed::RegSpec for Tbrar_SPEC {
    type DataType = u32;
}

pub type Tbrar = crate::RegValueT<Tbrar_SPEC>;

impl Tbrar {
    #[inline(always)]
    pub fn tbrar(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Tbrar_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Tbrar_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Tbrar {
    #[inline(always)]
    fn default() -> Tbrar {
        <crate::RegValueT<Tbrar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdfar_SPEC;
impl crate::sealed::RegSpec for Tdfar_SPEC {
    type DataType = u32;
}

pub type Tdfar = crate::RegValueT<Tdfar_SPEC>;

impl Tdfar {
    #[inline(always)]
    pub fn tdfar(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Tdfar_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Tdfar_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Tdfar {
    #[inline(always)]
    fn default() -> Tdfar {
        <crate::RegValueT<Tdfar_SPEC> as RegisterValue<_>>::new(0)
    }
}
