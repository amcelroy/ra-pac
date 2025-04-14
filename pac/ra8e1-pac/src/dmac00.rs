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
// Generated from SVD 1.00.01, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:22:22 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Direct memory access controller 0"]
unsafe impl ::core::marker::Send for super::Dmac00 {}
unsafe impl ::core::marker::Sync for super::Dmac00 {}
impl super::Dmac00 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn dmsar(&self) -> &'static crate::common::Reg<self::Dmsar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmsar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dmdar(&self) -> &'static crate::common::Reg<self::Dmdar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmdar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dmcra(&self) -> &'static crate::common::Reg<self::Dmcra_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmcra_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dmcrb(&self) -> &'static crate::common::Reg<self::Dmcrb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmcrb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dmtmd(&self) -> &'static crate::common::Reg<self::Dmtmd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmtmd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dmint(&self) -> &'static crate::common::Reg<self::Dmint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(19usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dmamd(&self) -> &'static crate::common::Reg<self::Dmamd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmamd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dmofr(&self) -> &'static crate::common::Reg<self::Dmofr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmofr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dmcnt(&self) -> &'static crate::common::Reg<self::Dmcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dmreq(&self) -> &'static crate::common::Reg<self::Dmreq_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmreq_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(29usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dmsts(&self) -> &'static crate::common::Reg<self::Dmsts_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmsts_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(30usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dmsrr(&self) -> &'static crate::common::Reg<self::Dmsrr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmsrr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dmdrr(&self) -> &'static crate::common::Reg<self::Dmdrr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmdrr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dmsbs(&self) -> &'static crate::common::Reg<self::Dmsbs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmsbs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dmdbs(&self) -> &'static crate::common::Reg<self::Dmdbs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmdbs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dmbwr(&self) -> &'static crate::common::Reg<self::Dmbwr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmbwr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmsar_SPEC;
impl crate::sealed::RegSpec for Dmsar_SPEC {
    type DataType = u32;
}

pub type Dmsar = crate::RegValueT<Dmsar_SPEC>;

impl NoBitfieldReg<Dmsar_SPEC> for Dmsar {}
impl ::core::default::Default for Dmsar {
    #[inline(always)]
    fn default() -> Dmsar {
        <crate::RegValueT<Dmsar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmdar_SPEC;
impl crate::sealed::RegSpec for Dmdar_SPEC {
    type DataType = u32;
}

pub type Dmdar = crate::RegValueT<Dmdar_SPEC>;

impl NoBitfieldReg<Dmdar_SPEC> for Dmdar {}
impl ::core::default::Default for Dmdar {
    #[inline(always)]
    fn default() -> Dmdar {
        <crate::RegValueT<Dmdar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmcra_SPEC;
impl crate::sealed::RegSpec for Dmcra_SPEC {
    type DataType = u32;
}

pub type Dmcra = crate::RegValueT<Dmcra_SPEC>;

impl Dmcra {
    #[inline(always)]
    pub fn dmcral(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dmcra_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dmcra_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dmcrah(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, u16, Dmcra_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3ff,1,0,u16,u16,Dmcra_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dmcra {
    #[inline(always)]
    fn default() -> Dmcra {
        <crate::RegValueT<Dmcra_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmcrb_SPEC;
impl crate::sealed::RegSpec for Dmcrb_SPEC {
    type DataType = u32;
}

pub type Dmcrb = crate::RegValueT<Dmcrb_SPEC>;

impl Dmcrb {
    #[inline(always)]
    pub fn dmcrbl(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dmcrb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dmcrb_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dmcrbh(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Dmcrb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Dmcrb_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dmcrb {
    #[inline(always)]
    fn default() -> Dmcrb {
        <crate::RegValueT<Dmcrb_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmtmd_SPEC;
impl crate::sealed::RegSpec for Dmtmd_SPEC {
    type DataType = u16;
}

pub type Dmtmd = crate::RegValueT<Dmtmd_SPEC>;

impl Dmtmd {
    #[inline(always)]
    pub fn dctg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        dmtmd::Dctg,
        dmtmd::Dctg,
        Dmtmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            dmtmd::Dctg,
            dmtmd::Dctg,
            Dmtmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sz(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        dmtmd::Sz,
        dmtmd::Sz,
        Dmtmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            dmtmd::Sz,
            dmtmd::Sz,
            Dmtmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tkp(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        dmtmd::Tkp,
        dmtmd::Tkp,
        Dmtmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            dmtmd::Tkp,
            dmtmd::Tkp,
            Dmtmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dts(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        dmtmd::Dts,
        dmtmd::Dts,
        Dmtmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            dmtmd::Dts,
            dmtmd::Dts,
            Dmtmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn md(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        dmtmd::Md,
        dmtmd::Md,
        Dmtmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            dmtmd::Md,
            dmtmd::Md,
            Dmtmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dmtmd {
    #[inline(always)]
    fn default() -> Dmtmd {
        <crate::RegValueT<Dmtmd_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dmtmd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dctg_SPEC;
    pub type Dctg = crate::EnumBitfieldStruct<u8, Dctg_SPEC>;
    impl Dctg {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sz_SPEC;
    pub type Sz = crate::EnumBitfieldStruct<u8, Sz_SPEC>;
    impl Sz {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tkp_SPEC;
    pub type Tkp = crate::EnumBitfieldStruct<u8, Tkp_SPEC>;
    impl Tkp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dts_SPEC;
    pub type Dts = crate::EnumBitfieldStruct<u8, Dts_SPEC>;
    impl Dts {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Md_SPEC;
    pub type Md = crate::EnumBitfieldStruct<u8, Md_SPEC>;
    impl Md {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmint_SPEC;
impl crate::sealed::RegSpec for Dmint_SPEC {
    type DataType = u8;
}

pub type Dmint = crate::RegValueT<Dmint_SPEC>;

impl Dmint {
    #[inline(always)]
    pub fn darie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dmint::Darie,
        dmint::Darie,
        Dmint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dmint::Darie,
            dmint::Darie,
            Dmint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sarie(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dmint::Sarie,
        dmint::Sarie,
        Dmint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dmint::Sarie,
            dmint::Sarie,
            Dmint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rptie(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dmint::Rptie,
        dmint::Rptie,
        Dmint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dmint::Rptie,
            dmint::Rptie,
            Dmint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn esie(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dmint::Esie,
        dmint::Esie,
        Dmint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dmint::Esie,
            dmint::Esie,
            Dmint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dtie(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dmint::Dtie,
        dmint::Dtie,
        Dmint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dmint::Dtie,
            dmint::Dtie,
            Dmint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dmint {
    #[inline(always)]
    fn default() -> Dmint {
        <crate::RegValueT<Dmint_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dmint {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Darie_SPEC;
    pub type Darie = crate::EnumBitfieldStruct<u8, Darie_SPEC>;
    impl Darie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sarie_SPEC;
    pub type Sarie = crate::EnumBitfieldStruct<u8, Sarie_SPEC>;
    impl Sarie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rptie_SPEC;
    pub type Rptie = crate::EnumBitfieldStruct<u8, Rptie_SPEC>;
    impl Rptie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Esie_SPEC;
    pub type Esie = crate::EnumBitfieldStruct<u8, Esie_SPEC>;
    impl Esie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtie_SPEC;
    pub type Dtie = crate::EnumBitfieldStruct<u8, Dtie_SPEC>;
    impl Dtie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmamd_SPEC;
impl crate::sealed::RegSpec for Dmamd_SPEC {
    type DataType = u16;
}

pub type Dmamd = crate::RegValueT<Dmamd_SPEC>;

impl Dmamd {
    #[inline(always)]
    pub fn dara(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Dmamd_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Dmamd_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dadr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dmamd::Dadr,
        dmamd::Dadr,
        Dmamd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dmamd::Dadr,
            dmamd::Dadr,
            Dmamd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dm(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        dmamd::Dm,
        dmamd::Dm,
        Dmamd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            dmamd::Dm,
            dmamd::Dm,
            Dmamd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sara(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, Dmamd_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,Dmamd_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sadr(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        dmamd::Sadr,
        dmamd::Sadr,
        Dmamd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            dmamd::Sadr,
            dmamd::Sadr,
            Dmamd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sm(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        dmamd::Sm,
        dmamd::Sm,
        Dmamd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            dmamd::Sm,
            dmamd::Sm,
            Dmamd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dmamd {
    #[inline(always)]
    fn default() -> Dmamd {
        <crate::RegValueT<Dmamd_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dmamd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dadr_SPEC;
    pub type Dadr = crate::EnumBitfieldStruct<u8, Dadr_SPEC>;
    impl Dadr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dm_SPEC;
    pub type Dm = crate::EnumBitfieldStruct<u8, Dm_SPEC>;
    impl Dm {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadr_SPEC;
    pub type Sadr = crate::EnumBitfieldStruct<u8, Sadr_SPEC>;
    impl Sadr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sm_SPEC;
    pub type Sm = crate::EnumBitfieldStruct<u8, Sm_SPEC>;
    impl Sm {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmofr_SPEC;
impl crate::sealed::RegSpec for Dmofr_SPEC {
    type DataType = u32;
}

pub type Dmofr = crate::RegValueT<Dmofr_SPEC>;

impl NoBitfieldReg<Dmofr_SPEC> for Dmofr {}
impl ::core::default::Default for Dmofr {
    #[inline(always)]
    fn default() -> Dmofr {
        <crate::RegValueT<Dmofr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmcnt_SPEC;
impl crate::sealed::RegSpec for Dmcnt_SPEC {
    type DataType = u8;
}

pub type Dmcnt = crate::RegValueT<Dmcnt_SPEC>;

impl Dmcnt {
    #[inline(always)]
    pub fn dte(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dmcnt::Dte,
        dmcnt::Dte,
        Dmcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dmcnt::Dte,
            dmcnt::Dte,
            Dmcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dmcnt {
    #[inline(always)]
    fn default() -> Dmcnt {
        <crate::RegValueT<Dmcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dmcnt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dte_SPEC;
    pub type Dte = crate::EnumBitfieldStruct<u8, Dte_SPEC>;
    impl Dte {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmreq_SPEC;
impl crate::sealed::RegSpec for Dmreq_SPEC {
    type DataType = u8;
}

pub type Dmreq = crate::RegValueT<Dmreq_SPEC>;

impl Dmreq {
    #[inline(always)]
    pub fn swreq(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dmreq::Swreq,
        dmreq::Swreq,
        Dmreq_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dmreq::Swreq,
            dmreq::Swreq,
            Dmreq_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn clrs(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dmreq::Clrs,
        dmreq::Clrs,
        Dmreq_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dmreq::Clrs,
            dmreq::Clrs,
            Dmreq_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dmreq {
    #[inline(always)]
    fn default() -> Dmreq {
        <crate::RegValueT<Dmreq_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dmreq {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Swreq_SPEC;
    pub type Swreq = crate::EnumBitfieldStruct<u8, Swreq_SPEC>;
    impl Swreq {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clrs_SPEC;
    pub type Clrs = crate::EnumBitfieldStruct<u8, Clrs_SPEC>;
    impl Clrs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmsts_SPEC;
impl crate::sealed::RegSpec for Dmsts_SPEC {
    type DataType = u8;
}

pub type Dmsts = crate::RegValueT<Dmsts_SPEC>;

impl Dmsts {
    #[inline(always)]
    pub fn esif(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dmsts::Esif,
        dmsts::Esif,
        Dmsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dmsts::Esif,
            dmsts::Esif,
            Dmsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dtif(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dmsts::Dtif,
        dmsts::Dtif,
        Dmsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dmsts::Dtif,
            dmsts::Dtif,
            Dmsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn act(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dmsts::Act,
        dmsts::Act,
        Dmsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dmsts::Act,
            dmsts::Act,
            Dmsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dmsts {
    #[inline(always)]
    fn default() -> Dmsts {
        <crate::RegValueT<Dmsts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dmsts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Esif_SPEC;
    pub type Esif = crate::EnumBitfieldStruct<u8, Esif_SPEC>;
    impl Esif {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtif_SPEC;
    pub type Dtif = crate::EnumBitfieldStruct<u8, Dtif_SPEC>;
    impl Dtif {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Act_SPEC;
    pub type Act = crate::EnumBitfieldStruct<u8, Act_SPEC>;
    impl Act {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmsrr_SPEC;
impl crate::sealed::RegSpec for Dmsrr_SPEC {
    type DataType = u32;
}

pub type Dmsrr = crate::RegValueT<Dmsrr_SPEC>;

impl NoBitfieldReg<Dmsrr_SPEC> for Dmsrr {}
impl ::core::default::Default for Dmsrr {
    #[inline(always)]
    fn default() -> Dmsrr {
        <crate::RegValueT<Dmsrr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmdrr_SPEC;
impl crate::sealed::RegSpec for Dmdrr_SPEC {
    type DataType = u32;
}

pub type Dmdrr = crate::RegValueT<Dmdrr_SPEC>;

impl NoBitfieldReg<Dmdrr_SPEC> for Dmdrr {}
impl ::core::default::Default for Dmdrr {
    #[inline(always)]
    fn default() -> Dmdrr {
        <crate::RegValueT<Dmdrr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmsbs_SPEC;
impl crate::sealed::RegSpec for Dmsbs_SPEC {
    type DataType = u32;
}

pub type Dmsbs = crate::RegValueT<Dmsbs_SPEC>;

impl Dmsbs {
    #[inline(always)]
    pub fn dmsbsl(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dmsbs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dmsbs_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dmsbsh(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Dmsbs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Dmsbs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dmsbs {
    #[inline(always)]
    fn default() -> Dmsbs {
        <crate::RegValueT<Dmsbs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmdbs_SPEC;
impl crate::sealed::RegSpec for Dmdbs_SPEC {
    type DataType = u32;
}

pub type Dmdbs = crate::RegValueT<Dmdbs_SPEC>;

impl Dmdbs {
    #[inline(always)]
    pub fn dmdbsl(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dmdbs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dmdbs_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dmdbsh(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Dmdbs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Dmdbs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dmdbs {
    #[inline(always)]
    fn default() -> Dmdbs {
        <crate::RegValueT<Dmdbs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmbwr_SPEC;
impl crate::sealed::RegSpec for Dmbwr_SPEC {
    type DataType = u8;
}

pub type Dmbwr = crate::RegValueT<Dmbwr_SPEC>;

impl Dmbwr {
    #[inline(always)]
    pub fn bwe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dmbwr::Bwe,
        dmbwr::Bwe,
        Dmbwr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dmbwr::Bwe,
            dmbwr::Bwe,
            Dmbwr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dmbwr {
    #[inline(always)]
    fn default() -> Dmbwr {
        <crate::RegValueT<Dmbwr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dmbwr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bwe_SPEC;
    pub type Bwe = crate::EnumBitfieldStruct<u8, Bwe_SPEC>;
    impl Bwe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
