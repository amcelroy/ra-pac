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
// Generated from SVD 1.0, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:16:43 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Bus Slave MPU"]
unsafe impl ::core::marker::Send for super::Smpu {}
unsafe impl ::core::marker::Sync for super::Smpu {}
impl super::Smpu {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "Slave MPU Control Register"]
    #[inline(always)]
    pub const fn smpuctl(
        &self,
    ) -> &'static crate::common::Reg<self::Smpuctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Smpuctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Access Control Register for MBIU"]
    #[inline(always)]
    pub const fn smpumbiu(
        &self,
    ) -> &'static crate::common::Reg<self::Smpumbiu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Smpumbiu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Access Control Register for FBIU"]
    #[inline(always)]
    pub const fn smpufbiu(
        &self,
    ) -> &'static crate::common::Reg<self::Smpufbiu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Smpufbiu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Access Control Register for SRAM0"]
    #[inline(always)]
    pub const fn smpusram0(
        &self,
    ) -> &'static crate::common::Reg<self::Smpusram0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Smpusram0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Access Control Register for P%sBIU"]
    #[inline(always)]
    pub const fn smpupbiu(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Smpupbiu_SPEC, crate::common::RW>,
        3,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x20usize))
        }
    }

    #[doc = "Access Control Register for EXBIU"]
    #[inline(always)]
    pub const fn smpuexbiu(
        &self,
    ) -> &'static crate::common::Reg<self::Smpuexbiu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Smpuexbiu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "Access Control Register for EXBIU2"]
    #[inline(always)]
    pub const fn smpuexbiu2(
        &self,
    ) -> &'static crate::common::Reg<self::Smpuexbiu2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Smpuexbiu2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smpuctl_SPEC;
impl crate::sealed::RegSpec for Smpuctl_SPEC {
    type DataType = u16;
}
#[doc = "Slave MPU Control Register"]
pub type Smpuctl = crate::RegValueT<Smpuctl_SPEC>;

impl Smpuctl {
    #[doc = "Key Code This bit is used to enable or disable rewriting of the PROTECT and OAD bit."]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, smpuctl::Key, Smpuctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,smpuctl::Key, Smpuctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, Smpuctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8, Smpuctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Protection of register   Protected register    SMPUMBIU, SMPUFBIU, SMPUSRAM0, SMPUP0BIU, SMPUP2BIU, SMPUP6BIU,SMPUEXBIU, SMPUEXBIU2"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, smpuctl::Protect, Smpuctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            1,
            0x3,
            1,
            0,
            smpuctl::Protect,
            Smpuctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Master Group enable"]
    #[inline(always)]
    pub fn oad(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, smpuctl::Oad, Smpuctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,smpuctl::Oad, Smpuctl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Smpuctl {
    #[inline(always)]
    fn default() -> Smpuctl {
        <crate::RegValueT<Smpuctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod smpuctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Key_SPEC;
    pub type Key = crate::EnumBitfieldStruct<u8, Key_SPEC>;
    impl Key {
        #[doc = "Writing to the PROTECT and OAD bit is valid, when the KEY bits are written 0xA5."]
        pub const _0_X_A_5: Self = Self::new(165);
        #[doc = "Writing to the  PROTECT and OAD bit is invalid."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "All Bus Slave register writing is possible."]
        pub const _0: Self = Self::new(0);
        #[doc = "All Bus Slave register writing is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
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
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smpumbiu_SPEC;
impl crate::sealed::RegSpec for Smpumbiu_SPEC {
    type DataType = u16;
}
#[doc = "Access Control Register for MBIU"]
pub type Smpumbiu = crate::RegValueT<Smpumbiu_SPEC>;

impl Smpumbiu {
    #[doc = "Master Group A Write protection"]
    #[inline(always)]
    pub fn wpgrpa(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        smpumbiu::Wpgrpa,
        Smpumbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            smpumbiu::Wpgrpa,
            Smpumbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Master Group A Read protection"]
    #[inline(always)]
    pub fn rpgrpa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        smpumbiu::Rpgrpa,
        Smpumbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            smpumbiu::Rpgrpa,
            Smpumbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Smpumbiu_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Smpumbiu_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Smpumbiu {
    #[inline(always)]
    fn default() -> Smpumbiu {
        <crate::RegValueT<Smpumbiu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod smpumbiu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpgrpa_SPEC;
    pub type Wpgrpa = crate::EnumBitfieldStruct<u8, Wpgrpa_SPEC>;
    impl Wpgrpa {
        #[doc = "Master group A write of memory protection disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "Master group A write of memory protection enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpgrpa_SPEC;
    pub type Rpgrpa = crate::EnumBitfieldStruct<u8, Rpgrpa_SPEC>;
    impl Rpgrpa {
        #[doc = "Master group A read of memory protection disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "Master group A read of memory protection enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smpufbiu_SPEC;
impl crate::sealed::RegSpec for Smpufbiu_SPEC {
    type DataType = u16;
}
#[doc = "Access Control Register for FBIU"]
pub type Smpufbiu = crate::RegValueT<Smpufbiu_SPEC>;

impl Smpufbiu {
    #[doc = "These bits are read as 000000000000. The write value should be 000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xfff, 1, 0, u16, Smpufbiu_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xfff,1,0,u16, Smpufbiu_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Master Group A Write protection"]
    #[inline(always)]
    pub fn wpgrpa(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        smpufbiu::Wpgrpa,
        Smpufbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            smpufbiu::Wpgrpa,
            Smpufbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Master Group A Read protection"]
    #[inline(always)]
    pub fn rpgrpa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        smpufbiu::Rpgrpa,
        Smpufbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            smpufbiu::Rpgrpa,
            Smpufbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU Write protection"]
    #[inline(always)]
    pub fn wpcpu(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, smpufbiu::Wpcpu, Smpufbiu_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            smpufbiu::Wpcpu,
            Smpufbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU Read protection"]
    #[inline(always)]
    pub fn rpcpu(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, smpufbiu::Rpcpu, Smpufbiu_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            smpufbiu::Rpcpu,
            Smpufbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Smpufbiu {
    #[inline(always)]
    fn default() -> Smpufbiu {
        <crate::RegValueT<Smpufbiu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod smpufbiu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpgrpa_SPEC;
    pub type Wpgrpa = crate::EnumBitfieldStruct<u8, Wpgrpa_SPEC>;
    impl Wpgrpa {
        #[doc = "Master group A write of memory protection disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "Master group A write of memory protection enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpgrpa_SPEC;
    pub type Rpgrpa = crate::EnumBitfieldStruct<u8, Rpgrpa_SPEC>;
    impl Rpgrpa {
        #[doc = "Master group A read of memory protection disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "Master group A read of memory protection enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpcpu_SPEC;
    pub type Wpcpu = crate::EnumBitfieldStruct<u8, Wpcpu_SPEC>;
    impl Wpcpu {
        #[doc = "CPU write of memory protection disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "CPU write of memory protection enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpcpu_SPEC;
    pub type Rpcpu = crate::EnumBitfieldStruct<u8, Rpcpu_SPEC>;
    impl Rpcpu {
        #[doc = "CPU read of memory protection disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "CPU read of memory protection enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smpusram0_SPEC;
impl crate::sealed::RegSpec for Smpusram0_SPEC {
    type DataType = u16;
}
#[doc = "Access Control Register for SRAM0"]
pub type Smpusram0 = crate::RegValueT<Smpusram0_SPEC>;

impl Smpusram0 {
    #[doc = "These bits are read as 000000000000. The write value should be 000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xfff, 1, 0, u16, Smpusram0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xfff,1,0,u16, Smpusram0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Master Group A Write protection"]
    #[inline(always)]
    pub fn wpgrpa(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        smpusram0::Wpgrpa,
        Smpusram0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            smpusram0::Wpgrpa,
            Smpusram0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Master Group A Read protection"]
    #[inline(always)]
    pub fn rpgrpa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        smpusram0::Rpgrpa,
        Smpusram0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            smpusram0::Rpgrpa,
            Smpusram0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU Write protection"]
    #[inline(always)]
    pub fn wpcpu(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        smpusram0::Wpcpu,
        Smpusram0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            smpusram0::Wpcpu,
            Smpusram0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU Read protection"]
    #[inline(always)]
    pub fn rpcpu(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        smpusram0::Rpcpu,
        Smpusram0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            smpusram0::Rpcpu,
            Smpusram0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Smpusram0 {
    #[inline(always)]
    fn default() -> Smpusram0 {
        <crate::RegValueT<Smpusram0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod smpusram0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpgrpa_SPEC;
    pub type Wpgrpa = crate::EnumBitfieldStruct<u8, Wpgrpa_SPEC>;
    impl Wpgrpa {
        #[doc = "Master group A write of memory protection disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "Master group A write of memory protection enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpgrpa_SPEC;
    pub type Rpgrpa = crate::EnumBitfieldStruct<u8, Rpgrpa_SPEC>;
    impl Rpgrpa {
        #[doc = "Master group A read of memory protection disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "Master group A read of memory protection enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpcpu_SPEC;
    pub type Wpcpu = crate::EnumBitfieldStruct<u8, Wpcpu_SPEC>;
    impl Wpcpu {
        #[doc = "CPU write of memory protection disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "CPU write of memory protection enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpcpu_SPEC;
    pub type Rpcpu = crate::EnumBitfieldStruct<u8, Rpcpu_SPEC>;
    impl Rpcpu {
        #[doc = "CPU read of memory protection disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "CPU read of memory protection enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smpupbiu_SPEC;
impl crate::sealed::RegSpec for Smpupbiu_SPEC {
    type DataType = u16;
}
#[doc = "Access Control Register for P%sBIU"]
pub type Smpupbiu = crate::RegValueT<Smpupbiu_SPEC>;

impl Smpupbiu {
    #[doc = "These bits are read as 000000000000. The write value should be 000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xfff, 1, 0, u16, Smpupbiu_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xfff,1,0,u16, Smpupbiu_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Master Group A Write protection"]
    #[inline(always)]
    pub fn wpgrpa(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        smpupbiu::Wpgrpa,
        Smpupbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            smpupbiu::Wpgrpa,
            Smpupbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Master Group A Read protection"]
    #[inline(always)]
    pub fn rpgrpa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        smpupbiu::Rpgrpa,
        Smpupbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            smpupbiu::Rpgrpa,
            Smpupbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU Write protection"]
    #[inline(always)]
    pub fn wpcpu(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, smpupbiu::Wpcpu, Smpupbiu_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            smpupbiu::Wpcpu,
            Smpupbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU Read protection"]
    #[inline(always)]
    pub fn rpcpu(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, smpupbiu::Rpcpu, Smpupbiu_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            smpupbiu::Rpcpu,
            Smpupbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Smpupbiu {
    #[inline(always)]
    fn default() -> Smpupbiu {
        <crate::RegValueT<Smpupbiu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod smpupbiu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpgrpa_SPEC;
    pub type Wpgrpa = crate::EnumBitfieldStruct<u8, Wpgrpa_SPEC>;
    impl Wpgrpa {
        #[doc = "Master group A write of memory protection disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "Master group A write of memory protection enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpgrpa_SPEC;
    pub type Rpgrpa = crate::EnumBitfieldStruct<u8, Rpgrpa_SPEC>;
    impl Rpgrpa {
        #[doc = "Master group A read of memory protection disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "Master group A read of memory protection enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpcpu_SPEC;
    pub type Wpcpu = crate::EnumBitfieldStruct<u8, Wpcpu_SPEC>;
    impl Wpcpu {
        #[doc = "CPU write of memory protection disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "CPU write of memory protection enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpcpu_SPEC;
    pub type Rpcpu = crate::EnumBitfieldStruct<u8, Rpcpu_SPEC>;
    impl Rpcpu {
        #[doc = "CPU read of memory protection disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "CPU read of memory protection enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smpuexbiu_SPEC;
impl crate::sealed::RegSpec for Smpuexbiu_SPEC {
    type DataType = u16;
}
#[doc = "Access Control Register for EXBIU"]
pub type Smpuexbiu = crate::RegValueT<Smpuexbiu_SPEC>;

impl Smpuexbiu {
    #[doc = "These bits are read as 000000000000. The write value should be 000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xfff, 1, 0, u16, Smpuexbiu_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xfff,1,0,u16, Smpuexbiu_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Master Group A Write protection"]
    #[inline(always)]
    pub fn wpgrpa(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        smpuexbiu::Wpgrpa,
        Smpuexbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            smpuexbiu::Wpgrpa,
            Smpuexbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Master Group A Read protection"]
    #[inline(always)]
    pub fn rpgrpa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        smpuexbiu::Rpgrpa,
        Smpuexbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            smpuexbiu::Rpgrpa,
            Smpuexbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU Write protection"]
    #[inline(always)]
    pub fn wpcpu(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        smpuexbiu::Wpcpu,
        Smpuexbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            smpuexbiu::Wpcpu,
            Smpuexbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU Read protection"]
    #[inline(always)]
    pub fn rpcpu(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        smpuexbiu::Rpcpu,
        Smpuexbiu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            smpuexbiu::Rpcpu,
            Smpuexbiu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Smpuexbiu {
    #[inline(always)]
    fn default() -> Smpuexbiu {
        <crate::RegValueT<Smpuexbiu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod smpuexbiu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpgrpa_SPEC;
    pub type Wpgrpa = crate::EnumBitfieldStruct<u8, Wpgrpa_SPEC>;
    impl Wpgrpa {
        #[doc = "Master group A write of memory protection disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "Master group A write of memory protection enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpgrpa_SPEC;
    pub type Rpgrpa = crate::EnumBitfieldStruct<u8, Rpgrpa_SPEC>;
    impl Rpgrpa {
        #[doc = "Master group A read of memory protection disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "Master group A read of memory protection enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpcpu_SPEC;
    pub type Wpcpu = crate::EnumBitfieldStruct<u8, Wpcpu_SPEC>;
    impl Wpcpu {
        #[doc = "CPU write of memory protection disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "CPU write of memory protection enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpcpu_SPEC;
    pub type Rpcpu = crate::EnumBitfieldStruct<u8, Rpcpu_SPEC>;
    impl Rpcpu {
        #[doc = "CPU read of memory protection disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "CPU read of memory protection enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smpuexbiu2_SPEC;
impl crate::sealed::RegSpec for Smpuexbiu2_SPEC {
    type DataType = u16;
}
#[doc = "Access Control Register for EXBIU2"]
pub type Smpuexbiu2 = crate::RegValueT<Smpuexbiu2_SPEC>;

impl Smpuexbiu2 {
    #[doc = "These bits are read as 000000000000. The write value should be 000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xfff, 1, 0, u16, Smpuexbiu2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xfff,1,0,u16, Smpuexbiu2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Master Group A Write protection"]
    #[inline(always)]
    pub fn wpgrpa(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        smpuexbiu2::Wpgrpa,
        Smpuexbiu2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            smpuexbiu2::Wpgrpa,
            Smpuexbiu2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Master Group A Read protection"]
    #[inline(always)]
    pub fn rpgrpa(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        smpuexbiu2::Rpgrpa,
        Smpuexbiu2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            smpuexbiu2::Rpgrpa,
            Smpuexbiu2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU Write protection"]
    #[inline(always)]
    pub fn wpcpu(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        smpuexbiu2::Wpcpu,
        Smpuexbiu2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            smpuexbiu2::Wpcpu,
            Smpuexbiu2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU Read protection"]
    #[inline(always)]
    pub fn rpcpu(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        smpuexbiu2::Rpcpu,
        Smpuexbiu2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            smpuexbiu2::Rpcpu,
            Smpuexbiu2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Smpuexbiu2 {
    #[inline(always)]
    fn default() -> Smpuexbiu2 {
        <crate::RegValueT<Smpuexbiu2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod smpuexbiu2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpgrpa_SPEC;
    pub type Wpgrpa = crate::EnumBitfieldStruct<u8, Wpgrpa_SPEC>;
    impl Wpgrpa {
        #[doc = "Master group A write of memory protection disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "Master group A write of memory protection enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpgrpa_SPEC;
    pub type Rpgrpa = crate::EnumBitfieldStruct<u8, Rpgrpa_SPEC>;
    impl Rpgrpa {
        #[doc = "Master group A read of memory protection disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "Master group A read of memory protection enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wpcpu_SPEC;
    pub type Wpcpu = crate::EnumBitfieldStruct<u8, Wpcpu_SPEC>;
    impl Wpcpu {
        #[doc = "CPU write of memory protection disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "CPU write of memory protection enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpcpu_SPEC;
    pub type Rpcpu = crate::EnumBitfieldStruct<u8, Rpcpu_SPEC>;
    impl Rpcpu {
        #[doc = "CPU read of memory protection disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "CPU read of memory protection enabled."]
        pub const _1: Self = Self::new(1);
    }
}
