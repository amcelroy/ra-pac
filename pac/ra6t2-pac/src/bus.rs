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
// Generated from SVD 1.40.00, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:21:24 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"BUS Control"]
unsafe impl ::core::marker::Send for super::Bus {}
unsafe impl ::core::marker::Sync for super::Bus {}
impl super::Bus {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn busscntfhbiu(
        &self,
    ) -> &'static crate::common::Reg<self::Busscntfhbiu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busscntfhbiu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4352usize),
            )
        }
    }

    #[inline(always)]
    pub const fn busscntflbiu(
        &self,
    ) -> &'static crate::common::Reg<self::Busscntflbiu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busscntflbiu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4356usize),
            )
        }
    }

    #[inline(always)]
    pub const fn busscnts0biu(
        &self,
    ) -> &'static crate::common::Reg<self::Busscnts0Biu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busscnts0Biu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4368usize),
            )
        }
    }

    #[inline(always)]
    pub const fn busscntpsbiu(
        &self,
    ) -> &'static crate::common::Reg<self::Busscntpsbiu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busscntpsbiu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4384usize),
            )
        }
    }

    #[inline(always)]
    pub const fn busscntplbiu(
        &self,
    ) -> &'static crate::common::Reg<self::Busscntplbiu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busscntplbiu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4400usize),
            )
        }
    }

    #[inline(always)]
    pub const fn busscntphbiu(
        &self,
    ) -> &'static crate::common::Reg<self::Busscntphbiu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busscntphbiu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4404usize),
            )
        }
    }

    #[inline(always)]
    pub const fn buserradd(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Buserradd_SPEC, crate::common::R>,
        3,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1800usize))
        }
    }

    #[inline(always)]
    pub const fn buserrrw(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Buserrrw_SPEC, crate::common::RW>,
        3,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1804usize))
        }
    }

    #[inline(always)]
    pub const fn btzferradd(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Btzferradd_SPEC, crate::common::R>,
        3,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1900usize))
        }
    }

    #[inline(always)]
    pub const fn btzferrrw(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Btzferrrw_SPEC, crate::common::RW>,
        3,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1904usize))
        }
    }

    #[inline(always)]
    pub const fn buserrstat(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Buserrstat_SPEC, crate::common::R>,
        3,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1a00usize))
        }
    }

    #[inline(always)]
    pub const fn buserrclr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Buserrclr_SPEC, crate::common::RW>,
        3,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1a08usize))
        }
    }

    #[inline(always)]
    pub const fn dmacdtcerrstat(
        &self,
    ) -> &'static crate::common::Reg<self::Dmacdtcerrstat_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Dmacdtcerrstat_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(6692usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dmacdtcerrclr(
        &self,
    ) -> &'static crate::common::Reg<self::Dmacdtcerrclr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmacdtcerrclr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6700usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscntfhbiu_SPEC;
impl crate::sealed::RegSpec for Busscntfhbiu_SPEC {
    type DataType = u16;
}

pub type Busscntfhbiu = crate::RegValueT<Busscntfhbiu_SPEC>;

impl Busscntfhbiu {
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        busscntfhbiu::Arbs,
        busscntfhbiu::Arbs,
        Busscntfhbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            busscntfhbiu::Arbs,
            busscntfhbiu::Arbs,
            Busscntfhbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busscntfhbiu {
    #[inline(always)]
    fn default() -> Busscntfhbiu {
        <crate::RegValueT<Busscntfhbiu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busscntfhbiu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbs_SPEC;
    pub type Arbs = crate::EnumBitfieldStruct<u8, Arbs_SPEC>;
    impl Arbs {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscntflbiu_SPEC;
impl crate::sealed::RegSpec for Busscntflbiu_SPEC {
    type DataType = u16;
}

pub type Busscntflbiu = crate::RegValueT<Busscntflbiu_SPEC>;

impl Busscntflbiu {
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        busscntflbiu::Arbs,
        busscntflbiu::Arbs,
        Busscntflbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            busscntflbiu::Arbs,
            busscntflbiu::Arbs,
            Busscntflbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busscntflbiu {
    #[inline(always)]
    fn default() -> Busscntflbiu {
        <crate::RegValueT<Busscntflbiu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busscntflbiu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbs_SPEC;
    pub type Arbs = crate::EnumBitfieldStruct<u8, Arbs_SPEC>;
    impl Arbs {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscnts0Biu_SPEC;
impl crate::sealed::RegSpec for Busscnts0Biu_SPEC {
    type DataType = u16;
}

pub type Busscnts0Biu = crate::RegValueT<Busscnts0Biu_SPEC>;

impl Busscnts0Biu {
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        busscnts0biu::Arbs,
        busscnts0biu::Arbs,
        Busscnts0Biu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            busscnts0biu::Arbs,
            busscnts0biu::Arbs,
            Busscnts0Biu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busscnts0Biu {
    #[inline(always)]
    fn default() -> Busscnts0Biu {
        <crate::RegValueT<Busscnts0Biu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busscnts0biu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbs_SPEC;
    pub type Arbs = crate::EnumBitfieldStruct<u8, Arbs_SPEC>;
    impl Arbs {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscntpsbiu_SPEC;
impl crate::sealed::RegSpec for Busscntpsbiu_SPEC {
    type DataType = u16;
}

pub type Busscntpsbiu = crate::RegValueT<Busscntpsbiu_SPEC>;

impl Busscntpsbiu {
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        busscntpsbiu::Arbs,
        busscntpsbiu::Arbs,
        Busscntpsbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            busscntpsbiu::Arbs,
            busscntpsbiu::Arbs,
            Busscntpsbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busscntpsbiu {
    #[inline(always)]
    fn default() -> Busscntpsbiu {
        <crate::RegValueT<Busscntpsbiu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busscntpsbiu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbs_SPEC;
    pub type Arbs = crate::EnumBitfieldStruct<u8, Arbs_SPEC>;
    impl Arbs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscntplbiu_SPEC;
impl crate::sealed::RegSpec for Busscntplbiu_SPEC {
    type DataType = u16;
}

pub type Busscntplbiu = crate::RegValueT<Busscntplbiu_SPEC>;

impl Busscntplbiu {
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        busscntplbiu::Arbs,
        busscntplbiu::Arbs,
        Busscntplbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            busscntplbiu::Arbs,
            busscntplbiu::Arbs,
            Busscntplbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busscntplbiu {
    #[inline(always)]
    fn default() -> Busscntplbiu {
        <crate::RegValueT<Busscntplbiu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busscntplbiu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbs_SPEC;
    pub type Arbs = crate::EnumBitfieldStruct<u8, Arbs_SPEC>;
    impl Arbs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscntphbiu_SPEC;
impl crate::sealed::RegSpec for Busscntphbiu_SPEC {
    type DataType = u16;
}

pub type Busscntphbiu = crate::RegValueT<Busscntphbiu_SPEC>;

impl Busscntphbiu {
    #[inline(always)]
    pub fn arbs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        busscntphbiu::Arbs,
        busscntphbiu::Arbs,
        Busscntphbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            busscntphbiu::Arbs,
            busscntphbiu::Arbs,
            Busscntphbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busscntphbiu {
    #[inline(always)]
    fn default() -> Busscntphbiu {
        <crate::RegValueT<Busscntphbiu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busscntphbiu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbs_SPEC;
    pub type Arbs = crate::EnumBitfieldStruct<u8, Arbs_SPEC>;
    impl Arbs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buserradd_SPEC;
impl crate::sealed::RegSpec for Buserradd_SPEC {
    type DataType = u32;
}

pub type Buserradd = crate::RegValueT<Buserradd_SPEC>;

impl Buserradd {
    #[inline(always)]
    pub fn berad(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Buserradd_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Buserradd_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Buserradd {
    #[inline(always)]
    fn default() -> Buserradd {
        <crate::RegValueT<Buserradd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buserrrw_SPEC;
impl crate::sealed::RegSpec for Buserrrw_SPEC {
    type DataType = u8;
}

pub type Buserrrw = crate::RegValueT<Buserrrw_SPEC>;

impl Buserrrw {
    #[inline(always)]
    pub fn rwstat(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        buserrrw::Rwstat,
        buserrrw::Rwstat,
        Buserrrw_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            buserrrw::Rwstat,
            buserrrw::Rwstat,
            Buserrrw_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Buserrrw {
    #[inline(always)]
    fn default() -> Buserrrw {
        <crate::RegValueT<Buserrrw_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod buserrrw {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rwstat_SPEC;
    pub type Rwstat = crate::EnumBitfieldStruct<u8, Rwstat_SPEC>;
    impl Rwstat {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Btzferradd_SPEC;
impl crate::sealed::RegSpec for Btzferradd_SPEC {
    type DataType = u32;
}

pub type Btzferradd = crate::RegValueT<Btzferradd_SPEC>;

impl Btzferradd {
    #[inline(always)]
    pub fn btzferad(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Btzferradd_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Btzferradd_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Btzferradd {
    #[inline(always)]
    fn default() -> Btzferradd {
        <crate::RegValueT<Btzferradd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Btzferrrw_SPEC;
impl crate::sealed::RegSpec for Btzferrrw_SPEC {
    type DataType = u8;
}

pub type Btzferrrw = crate::RegValueT<Btzferrrw_SPEC>;

impl Btzferrrw {
    #[inline(always)]
    pub fn trwstat(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        btzferrrw::Trwstat,
        btzferrrw::Trwstat,
        Btzferrrw_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            btzferrrw::Trwstat,
            btzferrrw::Trwstat,
            Btzferrrw_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Btzferrrw {
    #[inline(always)]
    fn default() -> Btzferrrw {
        <crate::RegValueT<Btzferrrw_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod btzferrrw {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trwstat_SPEC;
    pub type Trwstat = crate::EnumBitfieldStruct<u8, Trwstat_SPEC>;
    impl Trwstat {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buserrstat_SPEC;
impl crate::sealed::RegSpec for Buserrstat_SPEC {
    type DataType = u8;
}

pub type Buserrstat = crate::RegValueT<Buserrstat_SPEC>;

impl Buserrstat {
    #[inline(always)]
    pub fn slerrstat(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        buserrstat::Slerrstat,
        buserrstat::Slerrstat,
        Buserrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            buserrstat::Slerrstat,
            buserrstat::Slerrstat,
            Buserrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sterrstat(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        buserrstat::Sterrstat,
        buserrstat::Sterrstat,
        Buserrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            buserrstat::Sterrstat,
            buserrstat::Sterrstat,
            Buserrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mmerrstat(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        buserrstat::Mmerrstat,
        buserrstat::Mmerrstat,
        Buserrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            buserrstat::Mmerrstat,
            buserrstat::Mmerrstat,
            Buserrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ilerrstat(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        buserrstat::Ilerrstat,
        buserrstat::Ilerrstat,
        Buserrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            buserrstat::Ilerrstat,
            buserrstat::Ilerrstat,
            Buserrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Buserrstat {
    #[inline(always)]
    fn default() -> Buserrstat {
        <crate::RegValueT<Buserrstat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod buserrstat {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slerrstat_SPEC;
    pub type Slerrstat = crate::EnumBitfieldStruct<u8, Slerrstat_SPEC>;
    impl Slerrstat {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sterrstat_SPEC;
    pub type Sterrstat = crate::EnumBitfieldStruct<u8, Sterrstat_SPEC>;
    impl Sterrstat {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmerrstat_SPEC;
    pub type Mmerrstat = crate::EnumBitfieldStruct<u8, Mmerrstat_SPEC>;
    impl Mmerrstat {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ilerrstat_SPEC;
    pub type Ilerrstat = crate::EnumBitfieldStruct<u8, Ilerrstat_SPEC>;
    impl Ilerrstat {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buserrclr_SPEC;
impl crate::sealed::RegSpec for Buserrclr_SPEC {
    type DataType = u8;
}

pub type Buserrclr = crate::RegValueT<Buserrclr_SPEC>;

impl Buserrclr {
    #[inline(always)]
    pub fn slerrclr(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Buserrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Buserrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sterrclr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Buserrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Buserrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mmerrclr(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Buserrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Buserrclr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ilerrclr(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Buserrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Buserrclr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Buserrclr {
    #[inline(always)]
    fn default() -> Buserrclr {
        <crate::RegValueT<Buserrclr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmacdtcerrstat_SPEC;
impl crate::sealed::RegSpec for Dmacdtcerrstat_SPEC {
    type DataType = u8;
}

pub type Dmacdtcerrstat = crate::RegValueT<Dmacdtcerrstat_SPEC>;

impl Dmacdtcerrstat {
    #[inline(always)]
    pub fn mterrstat(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dmacdtcerrstat::Mterrstat,
        dmacdtcerrstat::Mterrstat,
        Dmacdtcerrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dmacdtcerrstat::Mterrstat,
            dmacdtcerrstat::Mterrstat,
            Dmacdtcerrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dmacdtcerrstat {
    #[inline(always)]
    fn default() -> Dmacdtcerrstat {
        <crate::RegValueT<Dmacdtcerrstat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dmacdtcerrstat {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mterrstat_SPEC;
    pub type Mterrstat = crate::EnumBitfieldStruct<u8, Mterrstat_SPEC>;
    impl Mterrstat {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmacdtcerrclr_SPEC;
impl crate::sealed::RegSpec for Dmacdtcerrclr_SPEC {
    type DataType = u8;
}

pub type Dmacdtcerrclr = crate::RegValueT<Dmacdtcerrclr_SPEC>;

impl Dmacdtcerrclr {
    #[inline(always)]
    pub fn mterrclr(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dmacdtcerrclr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Dmacdtcerrclr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dmacdtcerrclr {
    #[inline(always)]
    fn default() -> Dmacdtcerrclr {
        <crate::RegValueT<Dmacdtcerrclr_SPEC> as RegisterValue<_>>::new(0)
    }
}
