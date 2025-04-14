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
// Generated from SVD 1.20.00, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:19:27 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Renesas Memory Protection Unit"]
unsafe impl ::core::marker::Send for super::Rmpu {}
unsafe impl ::core::marker::Sync for super::Rmpu {}
impl super::Rmpu {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn mmpuoad(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuoad_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuoad_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mmpuoadpt(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuoadpt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuoadpt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mmpuendmac(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuendmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuendmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mmpuenptdmac(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuenptdmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuenptdmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(260usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mmpurptdmac(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpurptdmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpurptdmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(264usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mmpurptdmac_sec(
        &self,
    ) -> &'static crate::common::Reg<self::MmpurptdmacSec_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::MmpurptdmacSec_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(268usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mmpuacdmac(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuacdmac_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x200usize))
        }
    }

    #[inline(always)]
    pub const fn mmpusdmac(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpusdmac_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x204usize))
        }
    }

    #[inline(always)]
    pub const fn mmpuedmac(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuedmac_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x208usize))
        }
    }

    #[inline(always)]
    pub const fn mmpuenedmac(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuenedmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuenedmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1280usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mmpuenptedmac(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuenptedmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuenptedmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1284usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mmpurptedmac(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpurptedmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpurptedmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1288usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mmpuacedmac(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuacedmac_SPEC, crate::common::RW>,
        4,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x600usize))
        }
    }

    #[inline(always)]
    pub const fn mmpusedmac(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpusedmac_SPEC, crate::common::RW>,
        4,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x604usize))
        }
    }

    #[inline(always)]
    pub const fn mmpueedmac(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpueedmac_SPEC, crate::common::RW>,
        4,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x608usize))
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuoad_SPEC;
impl crate::sealed::RegSpec for Mmpuoad_SPEC {
    type DataType = u16;
}

pub type Mmpuoad = crate::RegValueT<Mmpuoad_SPEC>;

impl Mmpuoad {
    #[inline(always)]
    pub fn oad(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuoad::Oad,
        mmpuoad::Oad,
        Mmpuoad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuoad::Oad,
            mmpuoad::Oad,
            Mmpuoad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpuoad_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpuoad_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuoad {
    #[inline(always)]
    fn default() -> Mmpuoad {
        <crate::RegValueT<Mmpuoad_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuoad {

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
pub struct Mmpuoadpt_SPEC;
impl crate::sealed::RegSpec for Mmpuoadpt_SPEC {
    type DataType = u16;
}

pub type Mmpuoadpt = crate::RegValueT<Mmpuoadpt_SPEC>;

impl Mmpuoadpt {
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuoadpt::Protect,
        mmpuoadpt::Protect,
        Mmpuoadpt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuoadpt::Protect,
            mmpuoadpt::Protect,
            Mmpuoadpt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpuoadpt_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpuoadpt_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuoadpt {
    #[inline(always)]
    fn default() -> Mmpuoadpt {
        <crate::RegValueT<Mmpuoadpt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuoadpt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuendmac_SPEC;
impl crate::sealed::RegSpec for Mmpuendmac_SPEC {
    type DataType = u16;
}

pub type Mmpuendmac = crate::RegValueT<Mmpuendmac_SPEC>;

impl Mmpuendmac {
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuendmac::Enable,
        mmpuendmac::Enable,
        Mmpuendmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuendmac::Enable,
            mmpuendmac::Enable,
            Mmpuendmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpuendmac_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpuendmac_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuendmac {
    #[inline(always)]
    fn default() -> Mmpuendmac {
        <crate::RegValueT<Mmpuendmac_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuendmac {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuenptdmac_SPEC;
impl crate::sealed::RegSpec for Mmpuenptdmac_SPEC {
    type DataType = u16;
}

pub type Mmpuenptdmac = crate::RegValueT<Mmpuenptdmac_SPEC>;

impl Mmpuenptdmac {
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuenptdmac::Protect,
        mmpuenptdmac::Protect,
        Mmpuenptdmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuenptdmac::Protect,
            mmpuenptdmac::Protect,
            Mmpuenptdmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpuenptdmac_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpuenptdmac_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuenptdmac {
    #[inline(always)]
    fn default() -> Mmpuenptdmac {
        <crate::RegValueT<Mmpuenptdmac_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuenptdmac {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpurptdmac_SPEC;
impl crate::sealed::RegSpec for Mmpurptdmac_SPEC {
    type DataType = u16;
}

pub type Mmpurptdmac = crate::RegValueT<Mmpurptdmac_SPEC>;

impl Mmpurptdmac {
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpurptdmac::Protect,
        mmpurptdmac::Protect,
        Mmpurptdmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpurptdmac::Protect,
            mmpurptdmac::Protect,
            Mmpurptdmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpurptdmac_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpurptdmac_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpurptdmac {
    #[inline(always)]
    fn default() -> Mmpurptdmac {
        <crate::RegValueT<Mmpurptdmac_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpurptdmac {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MmpurptdmacSec_SPEC;
impl crate::sealed::RegSpec for MmpurptdmacSec_SPEC {
    type DataType = u16;
}

pub type MmpurptdmacSec = crate::RegValueT<MmpurptdmacSec_SPEC>;

impl MmpurptdmacSec {
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpurptdmac_sec::Protect,
        mmpurptdmac_sec::Protect,
        MmpurptdmacSec_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpurptdmac_sec::Protect,
            mmpurptdmac_sec::Protect,
            MmpurptdmacSec_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, MmpurptdmacSec_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,MmpurptdmacSec_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for MmpurptdmacSec {
    #[inline(always)]
    fn default() -> MmpurptdmacSec {
        <crate::RegValueT<MmpurptdmacSec_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpurptdmac_sec {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuacdmac_SPEC;
impl crate::sealed::RegSpec for Mmpuacdmac_SPEC {
    type DataType = u16;
}

pub type Mmpuacdmac = crate::RegValueT<Mmpuacdmac_SPEC>;

impl Mmpuacdmac {
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuacdmac::Enable,
        mmpuacdmac::Enable,
        Mmpuacdmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuacdmac::Enable,
            mmpuacdmac::Enable,
            Mmpuacdmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rp(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mmpuacdmac::Rp,
        mmpuacdmac::Rp,
        Mmpuacdmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mmpuacdmac::Rp,
            mmpuacdmac::Rp,
            Mmpuacdmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wp(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mmpuacdmac::Wp,
        mmpuacdmac::Wp,
        Mmpuacdmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mmpuacdmac::Wp,
            mmpuacdmac::Wp,
            Mmpuacdmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mmpuacdmac {
    #[inline(always)]
    fn default() -> Mmpuacdmac {
        <crate::RegValueT<Mmpuacdmac_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuacdmac {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rp_SPEC;
    pub type Rp = crate::EnumBitfieldStruct<u8, Rp_SPEC>;
    impl Rp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wp_SPEC;
    pub type Wp = crate::EnumBitfieldStruct<u8, Wp_SPEC>;
    impl Wp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpusdmac_SPEC;
impl crate::sealed::RegSpec for Mmpusdmac_SPEC {
    type DataType = u32;
}

pub type Mmpusdmac = crate::RegValueT<Mmpusdmac_SPEC>;

impl Mmpusdmac {
    #[inline(always)]
    pub fn mmpus(
        self,
    ) -> crate::common::RegisterField<5, 0x7ffffff, 1, 0, u32, u32, Mmpusdmac_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            5,
            0x7ffffff,
            1,
            0,
            u32,
            u32,
            Mmpusdmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mmpusdmac {
    #[inline(always)]
    fn default() -> Mmpusdmac {
        <crate::RegValueT<Mmpusdmac_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuedmac_SPEC;
impl crate::sealed::RegSpec for Mmpuedmac_SPEC {
    type DataType = u32;
}

pub type Mmpuedmac = crate::RegValueT<Mmpuedmac_SPEC>;

impl Mmpuedmac {
    #[inline(always)]
    pub fn mmpue(
        self,
    ) -> crate::common::RegisterField<5, 0x7ffffff, 1, 0, u32, u32, Mmpuedmac_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            5,
            0x7ffffff,
            1,
            0,
            u32,
            u32,
            Mmpuedmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mmpuedmac {
    #[inline(always)]
    fn default() -> Mmpuedmac {
        <crate::RegValueT<Mmpuedmac_SPEC> as RegisterValue<_>>::new(31)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuenedmac_SPEC;
impl crate::sealed::RegSpec for Mmpuenedmac_SPEC {
    type DataType = u16;
}

pub type Mmpuenedmac = crate::RegValueT<Mmpuenedmac_SPEC>;

impl Mmpuenedmac {
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuenedmac::Enable,
        mmpuenedmac::Enable,
        Mmpuenedmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuenedmac::Enable,
            mmpuenedmac::Enable,
            Mmpuenedmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpuenedmac_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpuenedmac_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuenedmac {
    #[inline(always)]
    fn default() -> Mmpuenedmac {
        <crate::RegValueT<Mmpuenedmac_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuenedmac {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuenptedmac_SPEC;
impl crate::sealed::RegSpec for Mmpuenptedmac_SPEC {
    type DataType = u16;
}

pub type Mmpuenptedmac = crate::RegValueT<Mmpuenptedmac_SPEC>;

impl Mmpuenptedmac {
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuenptedmac::Protect,
        mmpuenptedmac::Protect,
        Mmpuenptedmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuenptedmac::Protect,
            mmpuenptedmac::Protect,
            Mmpuenptedmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpuenptedmac_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpuenptedmac_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuenptedmac {
    #[inline(always)]
    fn default() -> Mmpuenptedmac {
        <crate::RegValueT<Mmpuenptedmac_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuenptedmac {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpurptedmac_SPEC;
impl crate::sealed::RegSpec for Mmpurptedmac_SPEC {
    type DataType = u16;
}

pub type Mmpurptedmac = crate::RegValueT<Mmpurptedmac_SPEC>;

impl Mmpurptedmac {
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpurptedmac::Protect,
        mmpurptedmac::Protect,
        Mmpurptedmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpurptedmac::Protect,
            mmpurptedmac::Protect,
            Mmpurptedmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpurptedmac_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpurptedmac_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpurptedmac {
    #[inline(always)]
    fn default() -> Mmpurptedmac {
        <crate::RegValueT<Mmpurptedmac_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpurptedmac {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuacedmac_SPEC;
impl crate::sealed::RegSpec for Mmpuacedmac_SPEC {
    type DataType = u16;
}

pub type Mmpuacedmac = crate::RegValueT<Mmpuacedmac_SPEC>;

impl Mmpuacedmac {
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuacedmac::Enable,
        mmpuacedmac::Enable,
        Mmpuacedmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuacedmac::Enable,
            mmpuacedmac::Enable,
            Mmpuacedmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rp(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mmpuacedmac::Rp,
        mmpuacedmac::Rp,
        Mmpuacedmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mmpuacedmac::Rp,
            mmpuacedmac::Rp,
            Mmpuacedmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wp(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mmpuacedmac::Wp,
        mmpuacedmac::Wp,
        Mmpuacedmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mmpuacedmac::Wp,
            mmpuacedmac::Wp,
            Mmpuacedmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mmpuacedmac {
    #[inline(always)]
    fn default() -> Mmpuacedmac {
        <crate::RegValueT<Mmpuacedmac_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuacedmac {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rp_SPEC;
    pub type Rp = crate::EnumBitfieldStruct<u8, Rp_SPEC>;
    impl Rp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wp_SPEC;
    pub type Wp = crate::EnumBitfieldStruct<u8, Wp_SPEC>;
    impl Wp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpusedmac_SPEC;
impl crate::sealed::RegSpec for Mmpusedmac_SPEC {
    type DataType = u32;
}

pub type Mmpusedmac = crate::RegValueT<Mmpusedmac_SPEC>;

impl Mmpusedmac {
    #[inline(always)]
    pub fn mmpus(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x7ffffff,
        1,
        0,
        u32,
        u32,
        Mmpusedmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x7ffffff,
            1,
            0,
            u32,
            u32,
            Mmpusedmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mmpusedmac {
    #[inline(always)]
    fn default() -> Mmpusedmac {
        <crate::RegValueT<Mmpusedmac_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpueedmac_SPEC;
impl crate::sealed::RegSpec for Mmpueedmac_SPEC {
    type DataType = u32;
}

pub type Mmpueedmac = crate::RegValueT<Mmpueedmac_SPEC>;

impl Mmpueedmac {
    #[inline(always)]
    pub fn mmpue(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x7ffffff,
        1,
        0,
        u32,
        u32,
        Mmpueedmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x7ffffff,
            1,
            0,
            u32,
            u32,
            Mmpueedmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mmpueedmac {
    #[inline(always)]
    fn default() -> Mmpueedmac {
        <crate::RegValueT<Mmpueedmac_SPEC> as RegisterValue<_>>::new(31)
    }
}
