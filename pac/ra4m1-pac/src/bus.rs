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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:15:57 +0000

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
    #[doc = "Master Bus Control Register %s"]
    #[inline(always)]
    pub const fn busmcnt(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Busmcnt_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1000usize))
        }
    }

    #[doc = "Slave Bus Control Register FLI"]
    #[inline(always)]
    pub const fn busscntfli(
        &self,
    ) -> &'static crate::common::Reg<self::Busscntfli_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busscntfli_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4352usize),
            )
        }
    }

    #[doc = "Slave Bus Control Register %s"]
    #[inline(always)]
    pub const fn busscnt(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Busscnt_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1114usize))
        }
    }

    #[doc = "Slave Bus Control Register P6B"]
    #[inline(always)]
    pub const fn busscntp6b(
        &self,
    ) -> &'static crate::common::Reg<self::Busscntp6B_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busscntp6B_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4392usize),
            )
        }
    }

    #[doc = "Slave Bus Control Register FBU"]
    #[inline(always)]
    pub const fn busscntfbu(
        &self,
    ) -> &'static crate::common::Reg<self::Busscntfbu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busscntfbu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4400usize),
            )
        }
    }

    #[doc = "Bus Error Address Register %s"]
    #[inline(always)]
    pub const fn buserradd(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Buserradd_SPEC, crate::common::R>,
        4,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1800usize))
        }
    }

    #[doc = "Bus Error Status Register %s"]
    #[inline(always)]
    pub const fn buserrstat(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Buserrstat_SPEC, crate::common::R>,
        4,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1804usize))
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busmcnt_SPEC;
impl crate::sealed::RegSpec for Busmcnt_SPEC {
    type DataType = u16;
}
#[doc = "Master Bus Control Register %s"]
pub type Busmcnt = crate::RegValueT<Busmcnt_SPEC>;

impl Busmcnt {
    #[doc = "Ignore Error Responses"]
    #[inline(always)]
    pub fn ieres(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, busmcnt::Ieres, Busmcnt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,busmcnt::Ieres, Busmcnt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 000000000000000. The write value should be 000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0x7fff, 1, 0, u16, Busmcnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7fff,1,0,u16, Busmcnt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Busmcnt {
    #[inline(always)]
    fn default() -> Busmcnt {
        <crate::RegValueT<Busmcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busmcnt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ieres_SPEC;
    pub type Ieres = crate::EnumBitfieldStruct<u8, Ieres_SPEC>;
    impl Ieres {
        #[doc = "A bus error is reported"]
        pub const _0: Self = Self::new(0);
        #[doc = "A bus error is not reported."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscntfli_SPEC;
impl crate::sealed::RegSpec for Busscntfli_SPEC {
    type DataType = u16;
}
#[doc = "Slave Bus Control Register FLI"]
pub type Busscntfli = crate::RegValueT<Busscntfli_SPEC>;

impl Busscntfli {
    #[doc = "Arbitration Method\nSpecify the priority between groups"]
    #[inline(always)]
    pub fn arbmet(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        busscntfli::Arbmet,
        Busscntfli_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            busscntfli::Arbmet,
            Busscntfli_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Busscntfli_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Busscntfli_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Busscntfli {
    #[inline(always)]
    fn default() -> Busscntfli {
        <crate::RegValueT<Busscntfli_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busscntfli {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbmet_SPEC;
    pub type Arbmet = crate::EnumBitfieldStruct<u8, Arbmet_SPEC>;
    impl Arbmet {
        #[doc = "fixed priority"]
        pub const _00: Self = Self::new(0);
        #[doc = "round-robin"]
        pub const _01: Self = Self::new(1);
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscnt_SPEC;
impl crate::sealed::RegSpec for Busscnt_SPEC {
    type DataType = u16;
}
#[doc = "Slave Bus Control Register %s"]
pub type Busscnt = crate::RegValueT<Busscnt_SPEC>;

impl Busscnt {
    #[doc = "Arbitration Method\nSpecify the priority between groups"]
    #[inline(always)]
    pub fn arbmet(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, busscnt::Arbmet, Busscnt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,busscnt::Arbmet, Busscnt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Busscnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Busscnt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Busscnt {
    #[inline(always)]
    fn default() -> Busscnt {
        <crate::RegValueT<Busscnt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busscnt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbmet_SPEC;
    pub type Arbmet = crate::EnumBitfieldStruct<u8, Arbmet_SPEC>;
    impl Arbmet {
        #[doc = "fixed priority"]
        pub const _00: Self = Self::new(0);
        #[doc = "round-robin"]
        pub const _01: Self = Self::new(1);
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscntp6B_SPEC;
impl crate::sealed::RegSpec for Busscntp6B_SPEC {
    type DataType = u16;
}
#[doc = "Slave Bus Control Register P6B"]
pub type Busscntp6B = crate::RegValueT<Busscntp6B_SPEC>;

impl Busscntp6B {
    #[doc = "Arbitration Method\nSpecify the priority between groups"]
    #[inline(always)]
    pub fn arbmet(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        busscntp6b::Arbmet,
        Busscntp6B_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            busscntp6b::Arbmet,
            Busscntp6B_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Busscntp6B_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Busscntp6B_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Busscntp6B {
    #[inline(always)]
    fn default() -> Busscntp6B {
        <crate::RegValueT<Busscntp6B_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busscntp6b {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbmet_SPEC;
    pub type Arbmet = crate::EnumBitfieldStruct<u8, Arbmet_SPEC>;
    impl Arbmet {
        #[doc = "fixed priority"]
        pub const _00: Self = Self::new(0);
        #[doc = "round-robin"]
        pub const _01: Self = Self::new(1);
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscntfbu_SPEC;
impl crate::sealed::RegSpec for Busscntfbu_SPEC {
    type DataType = u16;
}
#[doc = "Slave Bus Control Register FBU"]
pub type Busscntfbu = crate::RegValueT<Busscntfbu_SPEC>;

impl Busscntfbu {
    #[doc = "Arbitration Method\nSpecify the priority between groups"]
    #[inline(always)]
    pub fn arbmet(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        busscntfbu::Arbmet,
        Busscntfbu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            busscntfbu::Arbmet,
            Busscntfbu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Busscntfbu_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Busscntfbu_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Busscntfbu {
    #[inline(always)]
    fn default() -> Busscntfbu {
        <crate::RegValueT<Busscntfbu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busscntfbu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arbmet_SPEC;
    pub type Arbmet = crate::EnumBitfieldStruct<u8, Arbmet_SPEC>;
    impl Arbmet {
        #[doc = "fixed priority"]
        pub const _00: Self = Self::new(0);
        #[doc = "round-robin"]
        pub const _01: Self = Self::new(1);
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buserradd_SPEC;
impl crate::sealed::RegSpec for Buserradd_SPEC {
    type DataType = u32;
}
#[doc = "Bus Error Address Register %s"]
pub type Buserradd = crate::RegValueT<Buserradd_SPEC>;

impl Buserradd {
    #[doc = "Bus Error Address\nWhen a bus error occurs, It stores an error address."]
    #[inline(always)]
    pub fn berad(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Buserradd_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Buserradd_SPEC,crate::common::R>::from_register(self,0)
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
pub struct Buserrstat_SPEC;
impl crate::sealed::RegSpec for Buserrstat_SPEC {
    type DataType = u8;
}
#[doc = "Bus Error Status Register %s"]
pub type Buserrstat = crate::RegValueT<Buserrstat_SPEC>;

impl Buserrstat {
    #[doc = "Bus Error Status\nWhen bus error assert, error flag occurs."]
    #[inline(always)]
    pub fn errstat(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        buserrstat::Errstat,
        Buserrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            buserrstat::Errstat,
            Buserrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x3f, 1, 0, u8, Buserrstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x3f,1,0,u8, Buserrstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Error Access Status\nThe status at the time of the error"]
    #[inline(always)]
    pub fn accstat(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        buserrstat::Accstat,
        Buserrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            buserrstat::Accstat,
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
    pub struct Errstat_SPEC;
    pub type Errstat = crate::EnumBitfieldStruct<u8, Errstat_SPEC>;
    impl Errstat {
        #[doc = "No bus error occurred"]
        pub const _0: Self = Self::new(0);
        #[doc = "Bus error occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Accstat_SPEC;
    pub type Accstat = crate::EnumBitfieldStruct<u8, Accstat_SPEC>;
    impl Accstat {
        #[doc = "Read access"]
        pub const _0: Self = Self::new(0);
        #[doc = "Write Access"]
        pub const _1: Self = Self::new(1);
    }
}
