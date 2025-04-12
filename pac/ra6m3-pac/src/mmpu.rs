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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:17:41 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Bus Master MPU"]
unsafe impl ::core::marker::Send for super::Mmpu {}
unsafe impl ::core::marker::Sync for super::Mmpu {}
impl super::Mmpu {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "Bus Master MPU Control Register"]
    #[inline(always)]
    pub const fn mmpuctl(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuctl_SPEC, crate::common::RW>,
        3,
        0x400,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x0usize))
        }
    }

    #[doc = "Group A Region %s Access Control Register"]
    #[inline(always)]
    pub const fn mmpuaca(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW>,
        32,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x200usize))
        }
    }

    #[doc = "Group B Region %s Access Control Register"]
    #[inline(always)]
    pub const fn mmpuacb(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuacb_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x600usize))
        }
    }

    #[doc = "Group C Region %s Access Control Register"]
    #[inline(always)]
    pub const fn mmpuacc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuacc_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xa00usize))
        }
    }

    #[doc = "Group A Region %s Start Address Register"]
    #[inline(always)]
    pub const fn mmpusa(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW>,
        32,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x204usize))
        }
    }

    #[doc = "Group B Region %s Start Address Register"]
    #[inline(always)]
    pub const fn mmpusb(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpusb_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x604usize))
        }
    }

    #[doc = "Group C Region %s Start Address Register"]
    #[inline(always)]
    pub const fn mmpusc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpusc_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xa04usize))
        }
    }

    #[doc = "Group A Region %s End Address Register"]
    #[inline(always)]
    pub const fn mmpuea(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW>,
        32,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x208usize))
        }
    }

    #[doc = "Group B Region %s End Address Register"]
    #[inline(always)]
    pub const fn mmpueb(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpueb_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x608usize))
        }
    }

    #[doc = "Group C Region %s End Address Register"]
    #[inline(always)]
    pub const fn mmpuec(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuec_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xa08usize))
        }
    }

    #[doc = "Group A Protection of Register"]
    #[inline(always)]
    pub const fn mmpupta(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpupta_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpupta_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(258usize),
            )
        }
    }

    #[doc = "Group B Protection of Register"]
    #[inline(always)]
    pub const fn mmpuptb(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuptb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuptb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1282usize),
            )
        }
    }

    #[doc = "Group C protection of register"]
    #[inline(always)]
    pub const fn mmpuptc(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuptc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuptc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2306usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuctl_SPEC;
impl crate::sealed::RegSpec for Mmpuctl_SPEC {
    type DataType = u16;
}
#[doc = "Bus Master MPU Control Register"]
pub type Mmpuctl = crate::RegValueT<Mmpuctl_SPEC>;

impl Mmpuctl {
    #[doc = "Write Keyword The data written to these bits are not stored."]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, mmpuctl::Key, Mmpuctl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,mmpuctl::Key, Mmpuctl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Operation after detection"]
    #[inline(always)]
    pub fn oad(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, mmpuctl::Oad, Mmpuctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,mmpuctl::Oad, Mmpuctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Master Group enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, mmpuctl::Enable, Mmpuctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,mmpuctl::Enable, Mmpuctl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuctl {
    #[inline(always)]
    fn default() -> Mmpuctl {
        <crate::RegValueT<Mmpuctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Key_SPEC;
    pub type Key = crate::EnumBitfieldStruct<u8, Key_SPEC>;
    impl Key {
        #[doc = "Writing to the OAD and ENABLE bit is valid, when the KEY bits are written 0xA5."]
        pub const _0_X_A_5: Self = Self::new(165);
        #[doc = "Writing to the OAD and ENABLE bit is invalid."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oad_SPEC;
    pub type Oad = crate::EnumBitfieldStruct<u8, Oad_SPEC>;
    impl Oad {
        #[doc = "Non-maskable interrupt."]
        pub const _0: Self = Self::new(0);
        #[doc = "Internal reset."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "Master Group is disabled. Permission of all regions."]
        pub const _0: Self = Self::new(0);
        #[doc = "Master Group is enabled. Protection of all regions."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuaca_SPEC;
impl crate::sealed::RegSpec for Mmpuaca_SPEC {
    type DataType = u16;
}
#[doc = "Group A Region %s Access Control Register"]
pub type Mmpuaca = crate::RegValueT<Mmpuaca_SPEC>;

impl Mmpuaca {
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn wp(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, mmpuaca::Wp, Mmpuaca_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,mmpuaca::Wp, Mmpuaca_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Read protection"]
    #[inline(always)]
    pub fn rp(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, mmpuaca::Rp, Mmpuaca_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,mmpuaca::Rp, Mmpuaca_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Region enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, mmpuaca::Enable, Mmpuaca_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,mmpuaca::Enable, Mmpuaca_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuaca {
    #[inline(always)]
    fn default() -> Mmpuaca {
        <crate::RegValueT<Mmpuaca_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuaca {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wp_SPEC;
    pub type Wp = crate::EnumBitfieldStruct<u8, Wp_SPEC>;
    impl Wp {
        #[doc = "Write permission"]
        pub const _0: Self = Self::new(0);
        #[doc = "Write protection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rp_SPEC;
    pub type Rp = crate::EnumBitfieldStruct<u8, Rp_SPEC>;
    impl Rp {
        #[doc = "Read permission"]
        pub const _0: Self = Self::new(0);
        #[doc = "Read protection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "Group m Region n unit is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Group m Region n unit is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuacb_SPEC;
impl crate::sealed::RegSpec for Mmpuacb_SPEC {
    type DataType = u16;
}
#[doc = "Group B Region %s Access Control Register"]
pub type Mmpuacb = crate::RegValueT<Mmpuacb_SPEC>;

impl Mmpuacb {
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn wp(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, mmpuacb::Wp, Mmpuacb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,mmpuacb::Wp, Mmpuacb_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Read protection"]
    #[inline(always)]
    pub fn rp(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, mmpuacb::Rp, Mmpuacb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,mmpuacb::Rp, Mmpuacb_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Region enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, mmpuacb::Enable, Mmpuacb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,mmpuacb::Enable, Mmpuacb_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuacb {
    #[inline(always)]
    fn default() -> Mmpuacb {
        <crate::RegValueT<Mmpuacb_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuacb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wp_SPEC;
    pub type Wp = crate::EnumBitfieldStruct<u8, Wp_SPEC>;
    impl Wp {
        #[doc = "Write permission"]
        pub const _0: Self = Self::new(0);
        #[doc = "Write protection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rp_SPEC;
    pub type Rp = crate::EnumBitfieldStruct<u8, Rp_SPEC>;
    impl Rp {
        #[doc = "Read permission"]
        pub const _0: Self = Self::new(0);
        #[doc = "Read protection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "Group m Region n unit is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Group m Region n unit is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuacc_SPEC;
impl crate::sealed::RegSpec for Mmpuacc_SPEC {
    type DataType = u16;
}
#[doc = "Group C Region %s Access Control Register"]
pub type Mmpuacc = crate::RegValueT<Mmpuacc_SPEC>;

impl Mmpuacc {
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn wp(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, mmpuacc::Wp, Mmpuacc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,mmpuacc::Wp, Mmpuacc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Read protection"]
    #[inline(always)]
    pub fn rp(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, mmpuacc::Rp, Mmpuacc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,mmpuacc::Rp, Mmpuacc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Region enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, mmpuacc::Enable, Mmpuacc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,mmpuacc::Enable, Mmpuacc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuacc {
    #[inline(always)]
    fn default() -> Mmpuacc {
        <crate::RegValueT<Mmpuacc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuacc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wp_SPEC;
    pub type Wp = crate::EnumBitfieldStruct<u8, Wp_SPEC>;
    impl Wp {
        #[doc = "Write permission"]
        pub const _0: Self = Self::new(0);
        #[doc = "Write protection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rp_SPEC;
    pub type Rp = crate::EnumBitfieldStruct<u8, Rp_SPEC>;
    impl Rp {
        #[doc = "Read permission"]
        pub const _0: Self = Self::new(0);
        #[doc = "Read protection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "Group m Region n unit is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Group m Region n unit is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpusa_SPEC;
impl crate::sealed::RegSpec for Mmpusa_SPEC {
    type DataType = u32;
}
#[doc = "Group A Region %s Start Address Register"]
pub type Mmpusa = crate::RegValueT<Mmpusa_SPEC>;

impl Mmpusa {
    #[doc = "Address where the region starts, for use in region determination.NOTE: The low-order 2 bits are fixed to 0."]
    #[inline(always)]
    pub fn mmpusa(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Mmpusa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Mmpusa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpusa {
    #[inline(always)]
    fn default() -> Mmpusa {
        <crate::RegValueT<Mmpusa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpusb_SPEC;
impl crate::sealed::RegSpec for Mmpusb_SPEC {
    type DataType = u32;
}
#[doc = "Group B Region %s Start Address Register"]
pub type Mmpusb = crate::RegValueT<Mmpusb_SPEC>;

impl Mmpusb {
    #[doc = "Address where the region starts, for use in region determination.NOTE: The low-order 2 bits are fixed to 0."]
    #[inline(always)]
    pub fn mmpusb(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Mmpusb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Mmpusb_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpusb {
    #[inline(always)]
    fn default() -> Mmpusb {
        <crate::RegValueT<Mmpusb_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpusc_SPEC;
impl crate::sealed::RegSpec for Mmpusc_SPEC {
    type DataType = u32;
}
#[doc = "Group C Region %s Start Address Register"]
pub type Mmpusc = crate::RegValueT<Mmpusc_SPEC>;

impl Mmpusc {
    #[doc = "Address where the region starts, for use in region determination.NOTE: The low-order 2 bits are fixed to 0."]
    #[inline(always)]
    pub fn mmpusc(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Mmpusc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Mmpusc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpusc {
    #[inline(always)]
    fn default() -> Mmpusc {
        <crate::RegValueT<Mmpusc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuea_SPEC;
impl crate::sealed::RegSpec for Mmpuea_SPEC {
    type DataType = u32;
}
#[doc = "Group A Region %s End Address Register"]
pub type Mmpuea = crate::RegValueT<Mmpuea_SPEC>;

impl Mmpuea {
    #[doc = "Region end address registerAddress where the region end, for use in region determination.NOTE: The low-order 2 bits are fixed to 1."]
    #[inline(always)]
    pub fn mmpuea(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Mmpuea_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Mmpuea_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuea {
    #[inline(always)]
    fn default() -> Mmpuea {
        <crate::RegValueT<Mmpuea_SPEC> as RegisterValue<_>>::new(3)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpueb_SPEC;
impl crate::sealed::RegSpec for Mmpueb_SPEC {
    type DataType = u32;
}
#[doc = "Group B Region %s End Address Register"]
pub type Mmpueb = crate::RegValueT<Mmpueb_SPEC>;

impl Mmpueb {
    #[doc = "Region end address registerAddress where the region end, for use in region determination.NOTE: The low-order 2 bits are fixed to 1."]
    #[inline(always)]
    pub fn mmpueb(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Mmpueb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Mmpueb_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpueb {
    #[inline(always)]
    fn default() -> Mmpueb {
        <crate::RegValueT<Mmpueb_SPEC> as RegisterValue<_>>::new(3)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuec_SPEC;
impl crate::sealed::RegSpec for Mmpuec_SPEC {
    type DataType = u32;
}
#[doc = "Group C Region %s End Address Register"]
pub type Mmpuec = crate::RegValueT<Mmpuec_SPEC>;

impl Mmpuec {
    #[doc = "Region end address registerAddress where the region end, for use in region determination.NOTE: The low-order 2 bits are fixed to 1."]
    #[inline(always)]
    pub fn mmpuec(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Mmpuec_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Mmpuec_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuec {
    #[inline(always)]
    fn default() -> Mmpuec {
        <crate::RegValueT<Mmpuec_SPEC> as RegisterValue<_>>::new(3)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpupta_SPEC;
impl crate::sealed::RegSpec for Mmpupta_SPEC {
    type DataType = u16;
}
#[doc = "Group A Protection of Register"]
pub type Mmpupta = crate::RegValueT<Mmpupta_SPEC>;

impl Mmpupta {
    #[doc = "Write Keyword The data written to these bits are not stored."]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, mmpupta::Key, Mmpupta_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,mmpupta::Key, Mmpupta_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Protection of register(MMPUSAn, MMPUEAn and MMPUACAn)"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, mmpupta::Protect, Mmpupta_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpupta::Protect,
            Mmpupta_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mmpupta {
    #[inline(always)]
    fn default() -> Mmpupta {
        <crate::RegValueT<Mmpupta_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpupta {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Key_SPEC;
    pub type Key = crate::EnumBitfieldStruct<u8, Key_SPEC>;
    impl Key {
        #[doc = "Writing to the PROTECT bit is valid, when the KEY bits are written 0xA5."]
        pub const _0_X_A_5: Self = Self::new(165);
        #[doc = "Writing to the PROTECT bit is invalid."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "All Bus Master MPU Group A register writing is possible."]
        pub const _0: Self = Self::new(0);
        #[doc = "All Bus Master MPU Group A register writing is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuptb_SPEC;
impl crate::sealed::RegSpec for Mmpuptb_SPEC {
    type DataType = u16;
}
#[doc = "Group B Protection of Register"]
pub type Mmpuptb = crate::RegValueT<Mmpuptb_SPEC>;

impl Mmpuptb {
    #[doc = "Write Keyword The data written to these bits are not stored."]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, mmpuptb::Key, Mmpuptb_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,mmpuptb::Key, Mmpuptb_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Protection of register(MMPUSBn, MMPUEBn and MMPUACBn)"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, mmpuptb::Protect, Mmpuptb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuptb::Protect,
            Mmpuptb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mmpuptb {
    #[inline(always)]
    fn default() -> Mmpuptb {
        <crate::RegValueT<Mmpuptb_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuptb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Key_SPEC;
    pub type Key = crate::EnumBitfieldStruct<u8, Key_SPEC>;
    impl Key {
        #[doc = "Writing to the PROTECT bit is valid, when the KEY bits are written 0xA5."]
        pub const _0_X_A_5: Self = Self::new(165);
        #[doc = "Writing to the PROTECT bit is invalid."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "All Bus Master MPU Group B register writing is possible."]
        pub const _0: Self = Self::new(0);
        #[doc = "All Bus Master MPU Group B register writing is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuptc_SPEC;
impl crate::sealed::RegSpec for Mmpuptc_SPEC {
    type DataType = u16;
}
#[doc = "Group C protection of register"]
pub type Mmpuptc = crate::RegValueT<Mmpuptc_SPEC>;

impl Mmpuptc {
    #[doc = "Write Keyword The data written to these bits are not stored."]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, mmpuptc::Key, Mmpuptc_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,mmpuptc::Key, Mmpuptc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Protection of register (MMPUSCn, MMPUECn and MMPUACCn)"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, mmpuptc::Protect, Mmpuptc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuptc::Protect,
            Mmpuptc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mmpuptc {
    #[inline(always)]
    fn default() -> Mmpuptc {
        <crate::RegValueT<Mmpuptc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuptc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Key_SPEC;
    pub type Key = crate::EnumBitfieldStruct<u8, Key_SPEC>;
    impl Key {
        #[doc = "Writing to the PROTECT bit is valid, when the KEY bits are written 0xA5."]
        pub const _0_X_A_5: Self = Self::new(165);
        #[doc = "Writing to the PROTECT bit is invalid."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "All Bus Master MPU Group C register writing is possible."]
        pub const _0: Self = Self::new(0);
        #[doc = "All Bus Master MPU Group C register writing is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
