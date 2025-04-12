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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:20:32 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Event Link Controller"]
unsafe impl ::core::marker::Send for super::Elc {}
unsafe impl ::core::marker::Sync for super::Elc {}
impl super::Elc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "Event Link Controller Register"]
    #[inline(always)]
    pub const fn elcr(&self) -> &'static crate::common::Reg<self::Elcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Event Link Software Event Generation Register %s"]
    #[inline(always)]
    pub const fn elsegr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Elsegr_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x4usize))
        }
    }

    #[doc = "Event Link Setting Register %s"]
    #[inline(always)]
    pub const fn elsr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Elsr_SPEC, crate::common::RW>,
        18,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x20usize))
        }
    }

    #[doc = "Event Link Setting Register 30"]
    #[inline(always)]
    pub const fn elsr30(
        &self,
    ) -> &'static crate::common::Reg<self::Elsr30_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elsr30_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
            )
        }
    }

    #[doc = "Event Link Controller Security Attribution Register A"]
    #[inline(always)]
    pub const fn elcsara(
        &self,
    ) -> &'static crate::common::Reg<self::Elcsara_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elcsara_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(224usize),
            )
        }
    }

    #[doc = "Event Link Controller Security Attribution Register B"]
    #[inline(always)]
    pub const fn elcsarb(
        &self,
    ) -> &'static crate::common::Reg<self::Elcsarb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elcsarb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(228usize),
            )
        }
    }

    #[doc = "Event Link Controller Priviledge Attribution Register A"]
    #[inline(always)]
    pub const fn elcpara(
        &self,
    ) -> &'static crate::common::Reg<self::Elcpara_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elcpara_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(240usize),
            )
        }
    }

    #[doc = "Event Link Controller Priviledge Attribution Register B"]
    #[inline(always)]
    pub const fn elcparb(
        &self,
    ) -> &'static crate::common::Reg<self::Elcparb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Elcparb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(244usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elcr_SPEC;
impl crate::sealed::RegSpec for Elcr_SPEC {
    type DataType = u8;
}
#[doc = "Event Link Controller Register"]
pub type Elcr = crate::RegValueT<Elcr_SPEC>;

impl Elcr {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, Elcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8, Elcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "All Event Link Enable"]
    #[inline(always)]
    pub fn elcon(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, elcr::Elcon, Elcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,elcr::Elcon, Elcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Elcr {
    #[inline(always)]
    fn default() -> Elcr {
        <crate::RegValueT<Elcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod elcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elcon_SPEC;
    pub type Elcon = crate::EnumBitfieldStruct<u8, Elcon_SPEC>;
    impl Elcon {
        #[doc = "Disable ELC function"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable ELC function."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elsegr_SPEC;
impl crate::sealed::RegSpec for Elsegr_SPEC {
    type DataType = u8;
}
#[doc = "Event Link Software Event Generation Register %s"]
pub type Elsegr = crate::RegValueT<Elsegr_SPEC>;

impl Elsegr {
    #[doc = "Software Event Generation"]
    #[inline(always)]
    pub fn seg(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, elsegr::Seg, Elsegr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0x1,1,0,elsegr::Seg, Elsegr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x1f, 1, 0, u8, Elsegr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1f,1,0,u8, Elsegr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SEG Bit Write Enable"]
    #[inline(always)]
    pub fn we(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, elsegr::We, Elsegr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,elsegr::We, Elsegr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELSEGR Register Write Disable"]
    #[inline(always)]
    pub fn wi(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, elsegr::Wi, Elsegr_SPEC, crate::common::W> {
        crate::common::RegisterField::<7,0x1,1,0,elsegr::Wi, Elsegr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Elsegr {
    #[inline(always)]
    fn default() -> Elsegr {
        <crate::RegValueT<Elsegr_SPEC> as RegisterValue<_>>::new(128)
    }
}
pub mod elsegr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Seg_SPEC;
    pub type Seg = crate::EnumBitfieldStruct<u8, Seg_SPEC>;
    impl Seg {
        #[doc = "Normal operation"]
        pub const _0: Self = Self::new(0);
        #[doc = "Generate a software event"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct We_SPEC;
    pub type We = crate::EnumBitfieldStruct<u8, We_SPEC>;
    impl We {
        #[doc = "Disable writes to SEG bit"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable writes to SEG bit"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wi_SPEC;
    pub type Wi = crate::EnumBitfieldStruct<u8, Wi_SPEC>;
    impl Wi {
        #[doc = "Enable writes to ELSEGR register"]
        pub const _0: Self = Self::new(0);
        #[doc = "Disable writes to ELSEGR register."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elsr_SPEC;
impl crate::sealed::RegSpec for Elsr_SPEC {
    type DataType = u16;
}
#[doc = "Event Link Setting Register %s"]
pub type Elsr = crate::RegValueT<Elsr_SPEC>;

impl Elsr {
    #[doc = "Event Link Select"]
    #[inline(always)]
    pub fn els(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, elsr::Els, Elsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,elsr::Els, Elsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, Elsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x7f,1,0,u8, Elsr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Elsr {
    #[inline(always)]
    fn default() -> Elsr {
        <crate::RegValueT<Elsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod elsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Els_SPEC;
    pub type Els = crate::EnumBitfieldStruct<u8, Els_SPEC>;
    impl Els {
        #[doc = "Event output to the corresponding peripheral module is disabled."]
        pub const _0_X_000: Self = Self::new(0);
        #[doc = "Set the number for the event signal to be linked."]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elsr30_SPEC;
impl crate::sealed::RegSpec for Elsr30_SPEC {
    type DataType = u16;
}
#[doc = "Event Link Setting Register 30"]
pub type Elsr30 = crate::RegValueT<Elsr30_SPEC>;

impl Elsr30 {
    #[doc = "Event Link Select"]
    #[inline(always)]
    pub fn els(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, elsr30::Els, Elsr30_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,elsr30::Els, Elsr30_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, Elsr30_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x7f,1,0,u8, Elsr30_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Elsr30 {
    #[inline(always)]
    fn default() -> Elsr30 {
        <crate::RegValueT<Elsr30_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod elsr30 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Els_SPEC;
    pub type Els = crate::EnumBitfieldStruct<u8, Els_SPEC>;
    impl Els {
        #[doc = "Event output to the corresponding peripheral module is disabled."]
        pub const _0_X_000: Self = Self::new(0);
        #[doc = "Set the number for the event signal to be linked."]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elcsara_SPEC;
impl crate::sealed::RegSpec for Elcsara_SPEC {
    type DataType = u32;
}
#[doc = "Event Link Controller Security Attribution Register A"]
pub type Elcsara = crate::RegValueT<Elcsara_SPEC>;

impl Elcsara {
    #[doc = "Event Link Software Event Generation Register 1Security Attribution"]
    #[inline(always)]
    pub fn elsegr1(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, elcsara::Elsegr1, Elcsara_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            elcsara::Elsegr1,
            Elcsara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Elcsara_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Elcsara_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Elcsara {
    #[inline(always)]
    fn default() -> Elcsara {
        <crate::RegValueT<Elcsara_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod elcsara {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsegr1_SPEC;
    pub type Elsegr1 = crate::EnumBitfieldStruct<u8, Elsegr1_SPEC>;
    impl Elsegr1 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);
        #[doc = "NonSecure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elcsarb_SPEC;
impl crate::sealed::RegSpec for Elcsarb_SPEC {
    type DataType = u32;
}
#[doc = "Event Link Controller Security Attribution Register B"]
pub type Elcsarb = crate::RegValueT<Elcsarb_SPEC>;

impl Elcsarb {
    #[doc = "Event Link Setting Register 15Security Attribution"]
    #[inline(always)]
    pub fn elsr15(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, elcsarb::Elsr15, Elcsarb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            elcsarb::Elsr15,
            Elcsarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Event Link Setting Register 17Security Attribution"]
    #[inline(always)]
    pub fn elsr17(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, elcsarb::Elsr17, Elcsarb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            elcsarb::Elsr17,
            Elcsarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Event Link Setting Register 30Security Attribution"]
    #[inline(always)]
    pub fn elsr30(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, elcsarb::Elsr30, Elcsarb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            elcsarb::Elsr30,
            Elcsarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Elcsarb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Elcsarb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Elcsarb {
    #[inline(always)]
    fn default() -> Elcsarb {
        <crate::RegValueT<Elcsarb_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod elcsarb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr15_SPEC;
    pub type Elsr15 = crate::EnumBitfieldStruct<u8, Elsr15_SPEC>;
    impl Elsr15 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);
        #[doc = "NonSecure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr17_SPEC;
    pub type Elsr17 = crate::EnumBitfieldStruct<u8, Elsr17_SPEC>;
    impl Elsr17 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);
        #[doc = "NonSecure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr30_SPEC;
    pub type Elsr30 = crate::EnumBitfieldStruct<u8, Elsr30_SPEC>;
    impl Elsr30 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);
        #[doc = "NonSecure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elcpara_SPEC;
impl crate::sealed::RegSpec for Elcpara_SPEC {
    type DataType = u32;
}
#[doc = "Event Link Controller Priviledge Attribution Register A"]
pub type Elcpara = crate::RegValueT<Elcpara_SPEC>;

impl Elcpara {
    #[doc = "Event Link Software Event Generation Register 1Priviledge Attribution"]
    #[inline(always)]
    pub fn elsegr1(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, elcpara::Elsegr1, Elcpara_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            elcpara::Elsegr1,
            Elcpara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 1111111111111111. The write value should be 1111111111111111."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Elcpara_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Elcpara_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Elcpara {
    #[inline(always)]
    fn default() -> Elcpara {
        <crate::RegValueT<Elcpara_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod elcpara {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsegr1_SPEC;
    pub type Elsegr1 = crate::EnumBitfieldStruct<u8, Elsegr1_SPEC>;
    impl Elsegr1 {
        #[doc = "Priviledge"]
        pub const _0: Self = Self::new(0);
        #[doc = "AnyPriviledge"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elcparb_SPEC;
impl crate::sealed::RegSpec for Elcparb_SPEC {
    type DataType = u32;
}
#[doc = "Event Link Controller Priviledge Attribution Register B"]
pub type Elcparb = crate::RegValueT<Elcparb_SPEC>;

impl Elcparb {
    #[doc = "Event Link Setting Register 15Priviledge Attribution"]
    #[inline(always)]
    pub fn elsr15(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, elcparb::Elsr15, Elcparb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            elcparb::Elsr15,
            Elcparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Event Link Setting Register 17Priviledge Attribution"]
    #[inline(always)]
    pub fn elsr17(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, elcparb::Elsr17, Elcparb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            elcparb::Elsr17,
            Elcparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Event Link Setting Register 30Priviledge Attribution"]
    #[inline(always)]
    pub fn elsr30(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, elcparb::Elsr30, Elcparb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            elcparb::Elsr30,
            Elcparb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "This bit is read as 1. The write value should be 1."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Elcparb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Elcparb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Elcparb {
    #[inline(always)]
    fn default() -> Elcparb {
        <crate::RegValueT<Elcparb_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod elcparb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr15_SPEC;
    pub type Elsr15 = crate::EnumBitfieldStruct<u8, Elsr15_SPEC>;
    impl Elsr15 {
        #[doc = "Priviledge"]
        pub const _0: Self = Self::new(0);
        #[doc = "AnyPriviledge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr17_SPEC;
    pub type Elsr17 = crate::EnumBitfieldStruct<u8, Elsr17_SPEC>;
    impl Elsr17 {
        #[doc = "Priviledge"]
        pub const _0: Self = Self::new(0);
        #[doc = "AnyPriviledge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Elsr30_SPEC;
    pub type Elsr30 = crate::EnumBitfieldStruct<u8, Elsr30_SPEC>;
    impl Elsr30 {
        #[doc = "Priviledge"]
        pub const _0: Self = Self::new(0);
        #[doc = "AnyPriviledge"]
        pub const _1: Self = Self::new(1);
    }
}
