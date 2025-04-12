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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:19:02 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"On-Chip Debug"]
unsafe impl ::core::marker::Send for super::CpuOcd {}
unsafe impl ::core::marker::Sync for super::CpuOcd {}
impl super::CpuOcd {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "MCU Control Register"]
    #[inline(always)]
    pub const fn mcuctrl(
        &self,
    ) -> &'static crate::common::Reg<self::Mcuctrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mcuctrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "JTAG Boot Mode Entry Register"]
    #[inline(always)]
    pub const fn jbmdr(&self) -> &'static crate::common::Reg<self::Jbmdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jbmdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[doc = "JTAG Boot Receive Data Register"]
    #[inline(always)]
    pub const fn jbrdr(&self) -> &'static crate::common::Reg<self::Jbrdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jbrdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(288usize),
            )
        }
    }

    #[doc = "JTAG Boot Transmit Data Register"]
    #[inline(always)]
    pub const fn jbtdr(&self) -> &'static crate::common::Reg<self::Jbtdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jbtdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(304usize),
            )
        }
    }

    #[doc = "JTAG Boot Status register for External Host"]
    #[inline(always)]
    pub const fn jbstr(&self) -> &'static crate::common::Reg<self::Jbstr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jbstr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(320usize),
            )
        }
    }

    #[doc = "JTAG Boot Interrupt Control Register"]
    #[inline(always)]
    pub const fn jbicr(&self) -> &'static crate::common::Reg<self::Jbicr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Jbicr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(336usize),
            )
        }
    }

    #[doc = "First Stage Boot Loader Status Monitor Register"]
    #[inline(always)]
    pub const fn fsblstatm(
        &self,
    ) -> &'static crate::common::Reg<self::Fsblstatm_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fsblstatm_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(768usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcuctrl_SPEC;
impl crate::sealed::RegSpec for Mcuctrl_SPEC {
    type DataType = u32;
}
#[doc = "MCU Control Register"]
pub type Mcuctrl = crate::RegValueT<Mcuctrl_SPEC>;

impl Mcuctrl {
    #[doc = "External Debug Request. Writing 1 to the bit causes CPU Halt or Debug Monitor exception request."]
    #[inline(always)]
    pub fn edbgrq(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, mcuctrl::Edbgrq, Mcuctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,mcuctrl::Edbgrq, Mcuctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Writing 1 to the bit wakes up the CPU from Deep Sleep mode or the MCU from Software Standby Mode or Seep Software Standby mode"]
    #[inline(always)]
    pub fn dbirq(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, mcuctrl::Dbirq, Mcuctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,mcuctrl::Dbirq, Mcuctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CPU WAIT SETTING"]
    #[inline(always)]
    pub fn cpuwait(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        mcuctrl::Cpuwait,
        Mcuctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            mcuctrl::Cpuwait,
            Mcuctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 000000000000000. The write value should be 000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<17, 0x7fff, 1, 0, u16, Mcuctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<17,0x7fff,1,0,u16, Mcuctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mcuctrl {
    #[inline(always)]
    fn default() -> Mcuctrl {
        <crate::RegValueT<Mcuctrl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mcuctrl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Edbgrq_SPEC;
    pub type Edbgrq = crate::EnumBitfieldStruct<u8, Edbgrq_SPEC>;
    impl Edbgrq {
        #[doc = "Not request Debug Event."]
        pub const _0: Self = Self::new(0);
        #[doc = "Request Debug Event."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dbirq_SPEC;
    pub type Dbirq = crate::EnumBitfieldStruct<u8, Dbirq_SPEC>;
    impl Dbirq {
        #[doc = "Debug interrupt is not requested."]
        pub const _0: Self = Self::new(0);
        #[doc = "Debug interrupt requested."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpuwait_SPEC;
    pub type Cpuwait = crate::EnumBitfieldStruct<u8, Cpuwait_SPEC>;
    impl Cpuwait {
        #[doc = "Clear CPUWAIT to Low"]
        pub const _0: Self = Self::new(0);
        #[doc = "Set CPUWAIT to High"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jbmdr_SPEC;
impl crate::sealed::RegSpec for Jbmdr_SPEC {
    type DataType = u32;
}
#[doc = "JTAG Boot Mode Entry Register"]
pub type Jbmdr = crate::RegValueT<Jbmdr_SPEC>;

impl Jbmdr {
    #[doc = "Mode entry key"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Jbmdr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Jbmdr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Jbmdr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Jbmdr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jbmdr {
    #[inline(always)]
    fn default() -> Jbmdr {
        <crate::RegValueT<Jbmdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jbrdr_SPEC;
impl crate::sealed::RegSpec for Jbrdr_SPEC {
    type DataType = u32;
}
#[doc = "JTAG Boot Receive Data Register"]
pub type Jbrdr = crate::RegValueT<Jbrdr_SPEC>;

impl Jbrdr {
    #[doc = "Received data register"]
    #[inline(always)]
    pub fn rdat(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Jbrdr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Jbrdr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jbrdr {
    #[inline(always)]
    fn default() -> Jbrdr {
        <crate::RegValueT<Jbrdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jbtdr_SPEC;
impl crate::sealed::RegSpec for Jbtdr_SPEC {
    type DataType = u32;
}
#[doc = "JTAG Boot Transmit Data Register"]
pub type Jbtdr = crate::RegValueT<Jbtdr_SPEC>;

impl Jbtdr {
    #[doc = "Transmitted data register"]
    #[inline(always)]
    pub fn tdat(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Jbtdr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Jbtdr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jbtdr {
    #[inline(always)]
    fn default() -> Jbtdr {
        <crate::RegValueT<Jbtdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jbstr_SPEC;
impl crate::sealed::RegSpec for Jbstr_SPEC {
    type DataType = u32;
}
#[doc = "JTAG Boot Status register for External Host"]
pub type Jbstr = crate::RegValueT<Jbstr_SPEC>;

impl Jbstr {
    #[doc = "Receive buffer full"]
    #[inline(always)]
    pub fn rdf(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, jbstr::Rdf, Jbstr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,jbstr::Rdf, Jbstr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmit data empty"]
    #[inline(always)]
    pub fn tde(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, jbstr::Tde, Jbstr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,jbstr::Tde, Jbstr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Jbstr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Jbstr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jbstr {
    #[inline(always)]
    fn default() -> Jbstr {
        <crate::RegValueT<Jbstr_SPEC> as RegisterValue<_>>::new(2)
    }
}
pub mod jbstr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdf_SPEC;
    pub type Rdf = crate::EnumBitfieldStruct<u8, Rdf_SPEC>;
    impl Rdf {
        #[doc = "No receiving data"]
        pub const _0: Self = Self::new(0);
        #[doc = "There is receiving data"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tde_SPEC;
    pub type Tde = crate::EnumBitfieldStruct<u8, Tde_SPEC>;
    impl Tde {
        #[doc = "There is data transmission"]
        pub const _0: Self = Self::new(0);
        #[doc = "No data transmission"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jbicr_SPEC;
impl crate::sealed::RegSpec for Jbicr_SPEC {
    type DataType = u32;
}
#[doc = "JTAG Boot Interrupt Control Register"]
pub type Jbicr = crate::RegValueT<Jbicr_SPEC>;

impl Jbicr {
    #[doc = "Receive buffer full interrupt enabled"]
    #[inline(always)]
    pub fn rdfie(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, jbicr::Rdfie, Jbicr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,jbicr::Rdfie, Jbicr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Jbicr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Jbicr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Jbicr {
    #[inline(always)]
    fn default() -> Jbicr {
        <crate::RegValueT<Jbicr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod jbicr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdfie_SPEC;
    pub type Rdfie = crate::EnumBitfieldStruct<u8, Rdfie_SPEC>;
    impl Rdfie {
        #[doc = "Interrupt request disabled by RDF = 1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable interrupt request by RDF = 1"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsblstatm_SPEC;
impl crate::sealed::RegSpec for Fsblstatm_SPEC {
    type DataType = u32;
}
#[doc = "First Stage Boot Loader Status Monitor Register"]
pub type Fsblstatm = crate::RegValueT<Fsblstatm_SPEC>;

impl Fsblstatm {
    #[doc = "FSBL completion status."]
    #[inline(always)]
    pub fn cs(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, fsblstatm::Cs, Fsblstatm_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,fsblstatm::Cs, Fsblstatm_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "FSBL result status."]
    #[inline(always)]
    pub fn rs(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, fsblstatm::Rs, Fsblstatm_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,fsblstatm::Rs, Fsblstatm_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "These bits are read as 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Fsblstatm_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Fsblstatm_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fsblstatm {
    #[inline(always)]
    fn default() -> Fsblstatm {
        <crate::RegValueT<Fsblstatm_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fsblstatm {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cs_SPEC;
    pub type Cs = crate::EnumBitfieldStruct<u8, Cs_SPEC>;
    impl Cs {
        #[doc = "FSBL isn\'t complete."]
        pub const _0: Self = Self::new(0);
        #[doc = "FSBL is complete."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rs_SPEC;
    pub type Rs = crate::EnumBitfieldStruct<u8, Rs_SPEC>;
    impl Rs {
        #[doc = "FSBL failed."]
        pub const _0: Self = Self::new(0);
        #[doc = "FSBL passed."]
        pub const _1: Self = Self::new(1);
    }
}
