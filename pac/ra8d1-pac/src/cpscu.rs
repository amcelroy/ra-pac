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
#[doc = r"CPU System Security Control Unit"]
unsafe impl ::core::marker::Send for super::Cpscu {}
unsafe impl ::core::marker::Sync for super::Cpscu {}
impl super::Cpscu {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "SRAM Security Attribution Register"]
    #[inline(always)]
    pub const fn sramsar(
        &self,
    ) -> &'static crate::common::Reg<self::Sramsar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sramsar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "DTC Security Attribution Register"]
    #[inline(always)]
    pub const fn dtcsar(
        &self,
    ) -> &'static crate::common::Reg<self::Dtcsar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dtcsar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "DMAC Security Attribution Register"]
    #[inline(always)]
    pub const fn dmacsar(
        &self,
    ) -> &'static crate::common::Reg<self::Dmacsar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmacsar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "ICU Security Attribution Register A"]
    #[inline(always)]
    pub const fn icusara(
        &self,
    ) -> &'static crate::common::Reg<self::Icusara_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icusara_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "ICU Security Attribution Register B"]
    #[inline(always)]
    pub const fn icusarb(
        &self,
    ) -> &'static crate::common::Reg<self::Icusarb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icusarb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = "ICU Security Attribution Register E"]
    #[inline(always)]
    pub const fn icusare(
        &self,
    ) -> &'static crate::common::Reg<self::Icusare_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icusare_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "ICU Security Attribution Register F"]
    #[inline(always)]
    pub const fn icusarf(
        &self,
    ) -> &'static crate::common::Reg<self::Icusarf_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icusarf_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[doc = "ICU Security Attribution Register G"]
    #[inline(always)]
    pub const fn icusarg(
        &self,
    ) -> &'static crate::common::Reg<self::Icusarg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icusarg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[doc = "ICU Security Attribution Register H"]
    #[inline(always)]
    pub const fn icusarh(
        &self,
    ) -> &'static crate::common::Reg<self::Icusarh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icusarh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[doc = "ICU Security Attribution Register I"]
    #[inline(always)]
    pub const fn icusari(
        &self,
    ) -> &'static crate::common::Reg<self::Icusari_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icusari_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[doc = "BUS Security Attribution Register A"]
    #[inline(always)]
    pub const fn bussara(
        &self,
    ) -> &'static crate::common::Reg<self::Bussara_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bussara_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[doc = "BUS Security Attribution Register B"]
    #[inline(always)]
    pub const fn bussarb(
        &self,
    ) -> &'static crate::common::Reg<self::Bussarb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bussarb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(260usize),
            )
        }
    }

    #[doc = "BUS Security Attribution Register C (External BUS)"]
    #[inline(always)]
    pub const fn bussarc(
        &self,
    ) -> &'static crate::common::Reg<self::Bussarc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bussarc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(272usize),
            )
        }
    }

    #[doc = "BUS Privileged Attribution Register C (External BUS)"]
    #[inline(always)]
    pub const fn busparc(
        &self,
    ) -> &'static crate::common::Reg<self::Busparc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Busparc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(276usize),
            )
        }
    }

    #[doc = "Master MPU Security Attribution Register A"]
    #[inline(always)]
    pub const fn mmpusara(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusara_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusara_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(304usize),
            )
        }
    }

    #[doc = "Master MPU Security Attribution Register B"]
    #[inline(always)]
    pub const fn mmpusarb(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusarb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusarb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(308usize),
            )
        }
    }

    #[doc = "Debug Security Attribution Register"]
    #[inline(always)]
    pub const fn debugsar(
        &self,
    ) -> &'static crate::common::Reg<self::Debugsar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Debugsar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(384usize),
            )
        }
    }

    #[doc = "DMAC channel Security Attribution Register"]
    #[inline(always)]
    pub const fn dmacchsar(
        &self,
    ) -> &'static crate::common::Reg<self::Dmacchsar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmacchsar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(416usize),
            )
        }
    }

    #[doc = "DMAC channel Privileged Attribution Register"]
    #[inline(always)]
    pub const fn dmacchpar(
        &self,
    ) -> &'static crate::common::Reg<self::Dmacchpar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmacchpar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(496usize),
            )
        }
    }

    #[doc = "SRAM Security Attribute Boundary Address Register %s"]
    #[inline(always)]
    pub const fn sramsabar(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Sramsabar_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x400usize))
        }
    }

    #[doc = "Standby SRAM Security Attribute Boundary Address Register"]
    #[inline(always)]
    pub const fn stbramsabar(
        &self,
    ) -> &'static crate::common::Reg<self::Stbramsabar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Stbramsabar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1056usize),
            )
        }
    }

    #[doc = "Standby SRAM Privileged Attribute Boundary Address Register for Non-Secure region"]
    #[inline(always)]
    pub const fn stbrampabar_ns(
        &self,
    ) -> &'static crate::common::Reg<self::StbrampabarNs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::StbrampabarNs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1168usize),
            )
        }
    }

    #[doc = "Standby SRAM Privileged Attribute Boundary Address Register for Secure region"]
    #[inline(always)]
    pub const fn stbrampabar_s(
        &self,
    ) -> &'static crate::common::Reg<self::StbrampabarS_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::StbrampabarS_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1172usize),
            )
        }
    }

    #[doc = "Trusted EVenT Route Control Register"]
    #[inline(always)]
    pub const fn tevtrcr(
        &self,
    ) -> &'static crate::common::Reg<self::Tevtrcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tevtrcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1536usize),
            )
        }
    }

    #[doc = "BUS Memory Error Status Register"]
    #[inline(always)]
    pub const fn busmemerrsts(
        &self,
    ) -> &'static crate::common::Reg<self::Busmemerrsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Busmemerrsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(3328usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramsar_SPEC;
impl crate::sealed::RegSpec for Sramsar_SPEC {
    type DataType = u32;
}
#[doc = "SRAM Security Attribution Register"]
pub type Sramsar = crate::RegValueT<Sramsar_SPEC>;

impl Sramsar {
    #[doc = "Security attributes of registers for SRAM0"]
    #[inline(always)]
    pub fn sramsa0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, sramsar::Sramsa0, Sramsar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sramsar::Sramsa0,
            Sramsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for SRAM1"]
    #[inline(always)]
    pub fn sramsa1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, sramsar::Sramsa1, Sramsar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sramsar::Sramsa1,
            Sramsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for StandbySRAM"]
    #[inline(always)]
    pub fn stbramsa(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        sramsar::Stbramsa,
        Sramsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            sramsar::Stbramsa,
            Sramsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for SRAMWTSC"]
    #[inline(always)]
    pub fn sramwtsa(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        sramsar::Sramwtsa,
        Sramsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            sramsar::Sramwtsa,
            Sramsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Sramsar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Sramsar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sramsar {
    #[inline(always)]
    fn default() -> Sramsar {
        <crate::RegValueT<Sramsar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sramsar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sramsa0_SPEC;
    pub type Sramsa0 = crate::EnumBitfieldStruct<u8, Sramsa0_SPEC>;
    impl Sramsa0 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sramsa1_SPEC;
    pub type Sramsa1 = crate::EnumBitfieldStruct<u8, Sramsa1_SPEC>;
    impl Sramsa1 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stbramsa_SPEC;
    pub type Stbramsa = crate::EnumBitfieldStruct<u8, Stbramsa_SPEC>;
    impl Stbramsa {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sramwtsa_SPEC;
    pub type Sramwtsa = crate::EnumBitfieldStruct<u8, Sramwtsa_SPEC>;
    impl Sramwtsa {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtcsar_SPEC;
impl crate::sealed::RegSpec for Dtcsar_SPEC {
    type DataType = u32;
}
#[doc = "DTC Security Attribution Register"]
pub type Dtcsar = crate::RegValueT<Dtcsar_SPEC>;

impl Dtcsar {
    #[doc = "DTC Security Attribution"]
    #[inline(always)]
    pub fn dtcstsa(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, dtcsar::Dtcstsa, Dtcsar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,dtcsar::Dtcstsa, Dtcsar_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Dtcsar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Dtcsar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dtcsar {
    #[inline(always)]
    fn default() -> Dtcsar {
        <crate::RegValueT<Dtcsar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dtcsar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtcstsa_SPEC;
    pub type Dtcstsa = crate::EnumBitfieldStruct<u8, Dtcstsa_SPEC>;
    impl Dtcstsa {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmacsar_SPEC;
impl crate::sealed::RegSpec for Dmacsar_SPEC {
    type DataType = u32;
}
#[doc = "DMAC Security Attribution Register"]
pub type Dmacsar = crate::RegValueT<Dmacsar_SPEC>;

impl Dmacsar {
    #[doc = "DMAST Security Attribution"]
    #[inline(always)]
    pub fn dmastsa(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, dmacsar::Dmastsa, Dmacsar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dmacsar::Dmastsa,
            Dmacsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Dmacsar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Dmacsar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dmacsar {
    #[inline(always)]
    fn default() -> Dmacsar {
        <crate::RegValueT<Dmacsar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dmacsar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dmastsa_SPEC;
    pub type Dmastsa = crate::EnumBitfieldStruct<u8, Dmastsa_SPEC>;
    impl Dmastsa {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icusara_SPEC;
impl crate::sealed::RegSpec for Icusara_SPEC {
    type DataType = u32;
}
#[doc = "ICU Security Attribution Register A"]
pub type Icusara = crate::RegValueT<Icusara_SPEC>;

impl Icusara {
    #[doc = "Security attributes of registers for the IRQCR0 register"]
    #[inline(always)]
    pub fn sairqcr0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        icusara::Sairqcr0,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            icusara::Sairqcr0,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for the IRQCR1 register"]
    #[inline(always)]
    pub fn sairqcr1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        icusara::Sairqcr1,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            icusara::Sairqcr1,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for the IRQCR2 register"]
    #[inline(always)]
    pub fn sairqcr2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        icusara::Sairqcr2,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            icusara::Sairqcr2,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for the IRQCR3 register"]
    #[inline(always)]
    pub fn sairqcr3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        icusara::Sairqcr3,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            icusara::Sairqcr3,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for the IRQCR4 register"]
    #[inline(always)]
    pub fn sairqcr4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        icusara::Sairqcr4,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            icusara::Sairqcr4,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for the IRQCR5 register"]
    #[inline(always)]
    pub fn sairqcr5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        icusara::Sairqcr5,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            icusara::Sairqcr5,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for the IRQCR6 register"]
    #[inline(always)]
    pub fn sairqcr6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        icusara::Sairqcr6,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            icusara::Sairqcr6,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for the IRQCR7 register"]
    #[inline(always)]
    pub fn sairqcr7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        icusara::Sairqcr7,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            icusara::Sairqcr7,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for the IRQCR8 register"]
    #[inline(always)]
    pub fn sairqcr8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        icusara::Sairqcr8,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            icusara::Sairqcr8,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for the IRQCR9 register"]
    #[inline(always)]
    pub fn sairqcr9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        icusara::Sairqcr9,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            icusara::Sairqcr9,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for the IRQCR10 register"]
    #[inline(always)]
    pub fn sairqcr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        icusara::Sairqcr10,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            icusara::Sairqcr10,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for the IRQCR11 register"]
    #[inline(always)]
    pub fn sairqcr11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        icusara::Sairqcr11,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            icusara::Sairqcr11,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for the IRQCR12 register"]
    #[inline(always)]
    pub fn sairqcr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        icusara::Sairqcr12,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            icusara::Sairqcr12,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for the IRQCR13 register"]
    #[inline(always)]
    pub fn sairqcr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        icusara::Sairqcr13,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            icusara::Sairqcr13,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for the IRQCR14 register"]
    #[inline(always)]
    pub fn sairqcr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        icusara::Sairqcr14,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            icusara::Sairqcr14,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for the IRQCR15 register"]
    #[inline(always)]
    pub fn sairqcr15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        icusara::Sairqcr15,
        Icusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            icusara::Sairqcr15,
            Icusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Icusara_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Icusara_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Icusara {
    #[inline(always)]
    fn default() -> Icusara {
        <crate::RegValueT<Icusara_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icusara {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr0_SPEC;
    pub type Sairqcr0 = crate::EnumBitfieldStruct<u8, Sairqcr0_SPEC>;
    impl Sairqcr0 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr1_SPEC;
    pub type Sairqcr1 = crate::EnumBitfieldStruct<u8, Sairqcr1_SPEC>;
    impl Sairqcr1 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr2_SPEC;
    pub type Sairqcr2 = crate::EnumBitfieldStruct<u8, Sairqcr2_SPEC>;
    impl Sairqcr2 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr3_SPEC;
    pub type Sairqcr3 = crate::EnumBitfieldStruct<u8, Sairqcr3_SPEC>;
    impl Sairqcr3 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr4_SPEC;
    pub type Sairqcr4 = crate::EnumBitfieldStruct<u8, Sairqcr4_SPEC>;
    impl Sairqcr4 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr5_SPEC;
    pub type Sairqcr5 = crate::EnumBitfieldStruct<u8, Sairqcr5_SPEC>;
    impl Sairqcr5 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr6_SPEC;
    pub type Sairqcr6 = crate::EnumBitfieldStruct<u8, Sairqcr6_SPEC>;
    impl Sairqcr6 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr7_SPEC;
    pub type Sairqcr7 = crate::EnumBitfieldStruct<u8, Sairqcr7_SPEC>;
    impl Sairqcr7 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr8_SPEC;
    pub type Sairqcr8 = crate::EnumBitfieldStruct<u8, Sairqcr8_SPEC>;
    impl Sairqcr8 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr9_SPEC;
    pub type Sairqcr9 = crate::EnumBitfieldStruct<u8, Sairqcr9_SPEC>;
    impl Sairqcr9 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr10_SPEC;
    pub type Sairqcr10 = crate::EnumBitfieldStruct<u8, Sairqcr10_SPEC>;
    impl Sairqcr10 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr11_SPEC;
    pub type Sairqcr11 = crate::EnumBitfieldStruct<u8, Sairqcr11_SPEC>;
    impl Sairqcr11 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr12_SPEC;
    pub type Sairqcr12 = crate::EnumBitfieldStruct<u8, Sairqcr12_SPEC>;
    impl Sairqcr12 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr13_SPEC;
    pub type Sairqcr13 = crate::EnumBitfieldStruct<u8, Sairqcr13_SPEC>;
    impl Sairqcr13 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr14_SPEC;
    pub type Sairqcr14 = crate::EnumBitfieldStruct<u8, Sairqcr14_SPEC>;
    impl Sairqcr14 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sairqcr15_SPEC;
    pub type Sairqcr15 = crate::EnumBitfieldStruct<u8, Sairqcr15_SPEC>;
    impl Sairqcr15 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icusarb_SPEC;
impl crate::sealed::RegSpec for Icusarb_SPEC {
    type DataType = u32;
}
#[doc = "ICU Security Attribution Register B"]
pub type Icusarb = crate::RegValueT<Icusarb_SPEC>;

impl Icusarb {
    #[doc = "Security attributes of registers for non-maskable interrupt"]
    #[inline(always)]
    pub fn sanmi(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, icusarb::Sanmi, Icusarb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,icusarb::Sanmi, Icusarb_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Icusarb_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Icusarb_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Icusarb {
    #[inline(always)]
    fn default() -> Icusarb {
        <crate::RegValueT<Icusarb_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icusarb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sanmi_SPEC;
    pub type Sanmi = crate::EnumBitfieldStruct<u8, Sanmi_SPEC>;
    impl Sanmi {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icusare_SPEC;
impl crate::sealed::RegSpec for Icusare_SPEC {
    type DataType = u32;
}
#[doc = "ICU Security Attribution Register E"]
pub type Icusare = crate::RegValueT<Icusare_SPEC>;

impl Icusare {
    #[doc = "Security attributes of registers for WUPEN0.b16"]
    #[inline(always)]
    pub fn saiwdtwup(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        icusare::Saiwdtwup,
        Icusare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            icusare::Saiwdtwup,
            Icusare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for WUPEN0.b18"]
    #[inline(always)]
    pub fn sapvd1wup(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        icusare::Sapvd1Wup,
        Icusare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            icusare::Sapvd1Wup,
            Icusare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for WUPEN0.b19"]
    #[inline(always)]
    pub fn sapvd2wup(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        icusare::Sapvd2Wup,
        Icusare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            icusare::Sapvd2Wup,
            Icusare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for WUPEN0.b20"]
    #[inline(always)]
    pub fn savbattwup(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        icusare::Savbattwup,
        Icusare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            icusare::Savbattwup,
            Icusare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<21, 0x7, 1, 0, u8, Icusare_SPEC, crate::common::RW> {
        crate::common::RegisterField::<21,0x7,1,0,u8, Icusare_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Security attributes of registers for WUPEN0.b24"]
    #[inline(always)]
    pub fn sartcalmwup(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        icusare::Sartcalmwup,
        Icusare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            icusare::Sartcalmwup,
            Icusare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for WUPEN0.b25"]
    #[inline(always)]
    pub fn sartcprdwup(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        icusare::Sartcprdwup,
        Icusare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            icusare::Sartcprdwup,
            Icusare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for WUPEN0.b26"]
    #[inline(always)]
    pub fn sausbhs0wup(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        icusare::Sausbhs0Wup,
        Icusare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            icusare::Sausbhs0Wup,
            Icusare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for WUPEN0.b27"]
    #[inline(always)]
    pub fn sausbfs0wup(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        icusare::Sausbfs0Wup,
        Icusare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            icusare::Sausbfs0Wup,
            Icusare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for WUPEN0.b28"]
    #[inline(always)]
    pub fn saagt1udwup(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        icusare::Saagt1Udwup,
        Icusare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            icusare::Saagt1Udwup,
            Icusare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for WUPEN0.b29"]
    #[inline(always)]
    pub fn saagt1cawup(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        icusare::Saagt1Cawup,
        Icusare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            icusare::Saagt1Cawup,
            Icusare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for WUPEN0.b30"]
    #[inline(always)]
    pub fn saagt1cbwup(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        icusare::Saagt1Cbwup,
        Icusare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            icusare::Saagt1Cbwup,
            Icusare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for WUPEN0.b31"]
    #[inline(always)]
    pub fn saiic0wup(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        icusare::Saiic0Wup,
        Icusare_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            icusare::Saiic0Wup,
            Icusare_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icusare {
    #[inline(always)]
    fn default() -> Icusare {
        <crate::RegValueT<Icusare_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icusare {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saiwdtwup_SPEC;
    pub type Saiwdtwup = crate::EnumBitfieldStruct<u8, Saiwdtwup_SPEC>;
    impl Saiwdtwup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sapvd1Wup_SPEC;
    pub type Sapvd1Wup = crate::EnumBitfieldStruct<u8, Sapvd1Wup_SPEC>;
    impl Sapvd1Wup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sapvd2Wup_SPEC;
    pub type Sapvd2Wup = crate::EnumBitfieldStruct<u8, Sapvd2Wup_SPEC>;
    impl Sapvd2Wup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Savbattwup_SPEC;
    pub type Savbattwup = crate::EnumBitfieldStruct<u8, Savbattwup_SPEC>;
    impl Savbattwup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sartcalmwup_SPEC;
    pub type Sartcalmwup = crate::EnumBitfieldStruct<u8, Sartcalmwup_SPEC>;
    impl Sartcalmwup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sartcprdwup_SPEC;
    pub type Sartcprdwup = crate::EnumBitfieldStruct<u8, Sartcprdwup_SPEC>;
    impl Sartcprdwup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sausbhs0Wup_SPEC;
    pub type Sausbhs0Wup = crate::EnumBitfieldStruct<u8, Sausbhs0Wup_SPEC>;
    impl Sausbhs0Wup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sausbfs0Wup_SPEC;
    pub type Sausbfs0Wup = crate::EnumBitfieldStruct<u8, Sausbfs0Wup_SPEC>;
    impl Sausbfs0Wup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saagt1Udwup_SPEC;
    pub type Saagt1Udwup = crate::EnumBitfieldStruct<u8, Saagt1Udwup_SPEC>;
    impl Saagt1Udwup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saagt1Cawup_SPEC;
    pub type Saagt1Cawup = crate::EnumBitfieldStruct<u8, Saagt1Cawup_SPEC>;
    impl Saagt1Cawup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saagt1Cbwup_SPEC;
    pub type Saagt1Cbwup = crate::EnumBitfieldStruct<u8, Saagt1Cbwup_SPEC>;
    impl Saagt1Cbwup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saiic0Wup_SPEC;
    pub type Saiic0Wup = crate::EnumBitfieldStruct<u8, Saiic0Wup_SPEC>;
    impl Saiic0Wup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icusarf_SPEC;
impl crate::sealed::RegSpec for Icusarf_SPEC {
    type DataType = u32;
}
#[doc = "ICU Security Attribution Register F"]
pub type Icusarf = crate::RegValueT<Icusarf_SPEC>;

impl Icusarf {
    #[doc = "Security attributes of registers for WUPEN1.b3"]
    #[inline(always)]
    pub fn sacomphs0wup(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        icusarf::Sacomphs0Wup,
        Icusarf_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            icusarf::Sacomphs0Wup,
            Icusarf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for WUPEN1.b8"]
    #[inline(always)]
    pub fn saulp0uwup(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        icusarf::Saulp0Uwup,
        Icusarf_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            icusarf::Saulp0Uwup,
            Icusarf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for WUPEN1.b9"]
    #[inline(always)]
    pub fn saulp0awup(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        icusarf::Saulp0Awup,
        Icusarf_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            icusarf::Saulp0Awup,
            Icusarf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for WUPEN1.b10"]
    #[inline(always)]
    pub fn saulp0bwup(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        icusarf::Saulp0Bwup,
        Icusarf_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            icusarf::Saulp0Bwup,
            Icusarf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for WUPEN1.b11"]
    #[inline(always)]
    pub fn sai3cwup(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        icusarf::Sai3Cwup,
        Icusarf_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            icusarf::Sai3Cwup,
            Icusarf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for WUPEN1.b12"]
    #[inline(always)]
    pub fn saulp1uwup(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        icusarf::Saulp1Uwup,
        Icusarf_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            icusarf::Saulp1Uwup,
            Icusarf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for WUPEN1.b13"]
    #[inline(always)]
    pub fn saulp1awup(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        icusarf::Saulp1Awup,
        Icusarf_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            icusarf::Saulp1Awup,
            Icusarf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for WUPEN1.b14"]
    #[inline(always)]
    pub fn saulp1bwup(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        icusarf::Saulp1Bwup,
        Icusarf_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            icusarf::Saulp1Bwup,
            Icusarf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Icusarf_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Icusarf_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Icusarf {
    #[inline(always)]
    fn default() -> Icusarf {
        <crate::RegValueT<Icusarf_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icusarf {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sacomphs0Wup_SPEC;
    pub type Sacomphs0Wup = crate::EnumBitfieldStruct<u8, Sacomphs0Wup_SPEC>;
    impl Sacomphs0Wup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saulp0Uwup_SPEC;
    pub type Saulp0Uwup = crate::EnumBitfieldStruct<u8, Saulp0Uwup_SPEC>;
    impl Saulp0Uwup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saulp0Awup_SPEC;
    pub type Saulp0Awup = crate::EnumBitfieldStruct<u8, Saulp0Awup_SPEC>;
    impl Saulp0Awup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saulp0Bwup_SPEC;
    pub type Saulp0Bwup = crate::EnumBitfieldStruct<u8, Saulp0Bwup_SPEC>;
    impl Saulp0Bwup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sai3Cwup_SPEC;
    pub type Sai3Cwup = crate::EnumBitfieldStruct<u8, Sai3Cwup_SPEC>;
    impl Sai3Cwup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saulp1Uwup_SPEC;
    pub type Saulp1Uwup = crate::EnumBitfieldStruct<u8, Saulp1Uwup_SPEC>;
    impl Saulp1Uwup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saulp1Awup_SPEC;
    pub type Saulp1Awup = crate::EnumBitfieldStruct<u8, Saulp1Awup_SPEC>;
    impl Saulp1Awup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saulp1Bwup_SPEC;
    pub type Saulp1Bwup = crate::EnumBitfieldStruct<u8, Saulp1Bwup_SPEC>;
    impl Saulp1Bwup {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icusarg_SPEC;
impl crate::sealed::RegSpec for Icusarg_SPEC {
    type DataType = u32;
}
#[doc = "ICU Security Attribution Register G"]
pub type Icusarg = crate::RegValueT<Icusarg_SPEC>;

impl Icusarg {
    #[doc = "Security attributes of registers for IELSR0"]
    #[inline(always)]
    pub fn saielsr0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        icusarg::Saielsr0,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            icusarg::Saielsr0,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR1"]
    #[inline(always)]
    pub fn saielsr1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        icusarg::Saielsr1,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            icusarg::Saielsr1,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR2"]
    #[inline(always)]
    pub fn saielsr2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        icusarg::Saielsr2,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            icusarg::Saielsr2,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR3"]
    #[inline(always)]
    pub fn saielsr3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        icusarg::Saielsr3,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            icusarg::Saielsr3,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR4"]
    #[inline(always)]
    pub fn saielsr4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        icusarg::Saielsr4,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            icusarg::Saielsr4,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR5"]
    #[inline(always)]
    pub fn saielsr5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        icusarg::Saielsr5,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            icusarg::Saielsr5,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR6"]
    #[inline(always)]
    pub fn saielsr6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        icusarg::Saielsr6,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            icusarg::Saielsr6,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR7"]
    #[inline(always)]
    pub fn saielsr7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        icusarg::Saielsr7,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            icusarg::Saielsr7,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR8"]
    #[inline(always)]
    pub fn saielsr8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        icusarg::Saielsr8,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            icusarg::Saielsr8,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR9"]
    #[inline(always)]
    pub fn saielsr9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        icusarg::Saielsr9,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            icusarg::Saielsr9,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR10"]
    #[inline(always)]
    pub fn saielsr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        icusarg::Saielsr10,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            icusarg::Saielsr10,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR11"]
    #[inline(always)]
    pub fn saielsr11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        icusarg::Saielsr11,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            icusarg::Saielsr11,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR12"]
    #[inline(always)]
    pub fn saielsr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        icusarg::Saielsr12,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            icusarg::Saielsr12,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR13"]
    #[inline(always)]
    pub fn saielsr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        icusarg::Saielsr13,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            icusarg::Saielsr13,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR14"]
    #[inline(always)]
    pub fn saielsr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        icusarg::Saielsr14,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            icusarg::Saielsr14,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR15"]
    #[inline(always)]
    pub fn saielsr15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        icusarg::Saielsr15,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            icusarg::Saielsr15,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR16"]
    #[inline(always)]
    pub fn saielsr16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        icusarg::Saielsr16,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            icusarg::Saielsr16,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR17"]
    #[inline(always)]
    pub fn saielsr17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        icusarg::Saielsr17,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            icusarg::Saielsr17,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR18"]
    #[inline(always)]
    pub fn saielsr18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        icusarg::Saielsr18,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            icusarg::Saielsr18,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR19"]
    #[inline(always)]
    pub fn saielsr19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        icusarg::Saielsr19,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            icusarg::Saielsr19,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR20"]
    #[inline(always)]
    pub fn saielsr20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        icusarg::Saielsr20,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            icusarg::Saielsr20,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR21"]
    #[inline(always)]
    pub fn saielsr21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        icusarg::Saielsr21,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            icusarg::Saielsr21,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR22"]
    #[inline(always)]
    pub fn saielsr22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        icusarg::Saielsr22,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            icusarg::Saielsr22,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR23"]
    #[inline(always)]
    pub fn saielsr23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        icusarg::Saielsr23,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            icusarg::Saielsr23,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR24"]
    #[inline(always)]
    pub fn saielsr24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        icusarg::Saielsr24,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            icusarg::Saielsr24,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR25"]
    #[inline(always)]
    pub fn saielsr25(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        icusarg::Saielsr25,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            icusarg::Saielsr25,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR26"]
    #[inline(always)]
    pub fn saielsr26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        icusarg::Saielsr26,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            icusarg::Saielsr26,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR27"]
    #[inline(always)]
    pub fn saielsr27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        icusarg::Saielsr27,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            icusarg::Saielsr27,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR28"]
    #[inline(always)]
    pub fn saielsr28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        icusarg::Saielsr28,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            icusarg::Saielsr28,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR29"]
    #[inline(always)]
    pub fn saielsr29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        icusarg::Saielsr29,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            icusarg::Saielsr29,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR30"]
    #[inline(always)]
    pub fn saielsr30(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        icusarg::Saielsr30,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            icusarg::Saielsr30,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR31"]
    #[inline(always)]
    pub fn saielsr31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        icusarg::Saielsr31,
        Icusarg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            icusarg::Saielsr31,
            Icusarg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icusarg {
    #[inline(always)]
    fn default() -> Icusarg {
        <crate::RegValueT<Icusarg_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icusarg {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr0_SPEC;
    pub type Saielsr0 = crate::EnumBitfieldStruct<u8, Saielsr0_SPEC>;
    impl Saielsr0 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr1_SPEC;
    pub type Saielsr1 = crate::EnumBitfieldStruct<u8, Saielsr1_SPEC>;
    impl Saielsr1 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr2_SPEC;
    pub type Saielsr2 = crate::EnumBitfieldStruct<u8, Saielsr2_SPEC>;
    impl Saielsr2 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr3_SPEC;
    pub type Saielsr3 = crate::EnumBitfieldStruct<u8, Saielsr3_SPEC>;
    impl Saielsr3 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr4_SPEC;
    pub type Saielsr4 = crate::EnumBitfieldStruct<u8, Saielsr4_SPEC>;
    impl Saielsr4 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr5_SPEC;
    pub type Saielsr5 = crate::EnumBitfieldStruct<u8, Saielsr5_SPEC>;
    impl Saielsr5 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr6_SPEC;
    pub type Saielsr6 = crate::EnumBitfieldStruct<u8, Saielsr6_SPEC>;
    impl Saielsr6 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr7_SPEC;
    pub type Saielsr7 = crate::EnumBitfieldStruct<u8, Saielsr7_SPEC>;
    impl Saielsr7 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr8_SPEC;
    pub type Saielsr8 = crate::EnumBitfieldStruct<u8, Saielsr8_SPEC>;
    impl Saielsr8 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr9_SPEC;
    pub type Saielsr9 = crate::EnumBitfieldStruct<u8, Saielsr9_SPEC>;
    impl Saielsr9 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr10_SPEC;
    pub type Saielsr10 = crate::EnumBitfieldStruct<u8, Saielsr10_SPEC>;
    impl Saielsr10 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr11_SPEC;
    pub type Saielsr11 = crate::EnumBitfieldStruct<u8, Saielsr11_SPEC>;
    impl Saielsr11 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr12_SPEC;
    pub type Saielsr12 = crate::EnumBitfieldStruct<u8, Saielsr12_SPEC>;
    impl Saielsr12 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr13_SPEC;
    pub type Saielsr13 = crate::EnumBitfieldStruct<u8, Saielsr13_SPEC>;
    impl Saielsr13 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr14_SPEC;
    pub type Saielsr14 = crate::EnumBitfieldStruct<u8, Saielsr14_SPEC>;
    impl Saielsr14 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr15_SPEC;
    pub type Saielsr15 = crate::EnumBitfieldStruct<u8, Saielsr15_SPEC>;
    impl Saielsr15 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr16_SPEC;
    pub type Saielsr16 = crate::EnumBitfieldStruct<u8, Saielsr16_SPEC>;
    impl Saielsr16 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr17_SPEC;
    pub type Saielsr17 = crate::EnumBitfieldStruct<u8, Saielsr17_SPEC>;
    impl Saielsr17 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr18_SPEC;
    pub type Saielsr18 = crate::EnumBitfieldStruct<u8, Saielsr18_SPEC>;
    impl Saielsr18 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr19_SPEC;
    pub type Saielsr19 = crate::EnumBitfieldStruct<u8, Saielsr19_SPEC>;
    impl Saielsr19 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr20_SPEC;
    pub type Saielsr20 = crate::EnumBitfieldStruct<u8, Saielsr20_SPEC>;
    impl Saielsr20 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr21_SPEC;
    pub type Saielsr21 = crate::EnumBitfieldStruct<u8, Saielsr21_SPEC>;
    impl Saielsr21 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr22_SPEC;
    pub type Saielsr22 = crate::EnumBitfieldStruct<u8, Saielsr22_SPEC>;
    impl Saielsr22 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr23_SPEC;
    pub type Saielsr23 = crate::EnumBitfieldStruct<u8, Saielsr23_SPEC>;
    impl Saielsr23 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr24_SPEC;
    pub type Saielsr24 = crate::EnumBitfieldStruct<u8, Saielsr24_SPEC>;
    impl Saielsr24 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr25_SPEC;
    pub type Saielsr25 = crate::EnumBitfieldStruct<u8, Saielsr25_SPEC>;
    impl Saielsr25 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr26_SPEC;
    pub type Saielsr26 = crate::EnumBitfieldStruct<u8, Saielsr26_SPEC>;
    impl Saielsr26 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr27_SPEC;
    pub type Saielsr27 = crate::EnumBitfieldStruct<u8, Saielsr27_SPEC>;
    impl Saielsr27 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr28_SPEC;
    pub type Saielsr28 = crate::EnumBitfieldStruct<u8, Saielsr28_SPEC>;
    impl Saielsr28 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr29_SPEC;
    pub type Saielsr29 = crate::EnumBitfieldStruct<u8, Saielsr29_SPEC>;
    impl Saielsr29 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr30_SPEC;
    pub type Saielsr30 = crate::EnumBitfieldStruct<u8, Saielsr30_SPEC>;
    impl Saielsr30 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr31_SPEC;
    pub type Saielsr31 = crate::EnumBitfieldStruct<u8, Saielsr31_SPEC>;
    impl Saielsr31 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icusarh_SPEC;
impl crate::sealed::RegSpec for Icusarh_SPEC {
    type DataType = u32;
}
#[doc = "ICU Security Attribution Register H"]
pub type Icusarh = crate::RegValueT<Icusarh_SPEC>;

impl Icusarh {
    #[doc = "Security attributes of registers for IELSR32"]
    #[inline(always)]
    pub fn saielsr32(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        icusarh::Saielsr32,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            icusarh::Saielsr32,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR33"]
    #[inline(always)]
    pub fn saielsr33(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        icusarh::Saielsr33,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            icusarh::Saielsr33,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR34"]
    #[inline(always)]
    pub fn saielsr34(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        icusarh::Saielsr34,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            icusarh::Saielsr34,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR35"]
    #[inline(always)]
    pub fn saielsr35(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        icusarh::Saielsr35,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            icusarh::Saielsr35,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR36"]
    #[inline(always)]
    pub fn saielsr36(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        icusarh::Saielsr36,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            icusarh::Saielsr36,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR37"]
    #[inline(always)]
    pub fn saielsr37(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        icusarh::Saielsr37,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            icusarh::Saielsr37,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR38"]
    #[inline(always)]
    pub fn saielsr38(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        icusarh::Saielsr38,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            icusarh::Saielsr38,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR39"]
    #[inline(always)]
    pub fn saielsr39(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        icusarh::Saielsr39,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            icusarh::Saielsr39,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR40"]
    #[inline(always)]
    pub fn saielsr40(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        icusarh::Saielsr40,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            icusarh::Saielsr40,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR41"]
    #[inline(always)]
    pub fn saielsr41(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        icusarh::Saielsr41,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            icusarh::Saielsr41,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR42"]
    #[inline(always)]
    pub fn saielsr42(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        icusarh::Saielsr42,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            icusarh::Saielsr42,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR43"]
    #[inline(always)]
    pub fn saielsr43(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        icusarh::Saielsr43,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            icusarh::Saielsr43,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR44"]
    #[inline(always)]
    pub fn saielsr44(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        icusarh::Saielsr44,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            icusarh::Saielsr44,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR45"]
    #[inline(always)]
    pub fn saielsr45(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        icusarh::Saielsr45,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            icusarh::Saielsr45,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR46"]
    #[inline(always)]
    pub fn saielsr46(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        icusarh::Saielsr46,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            icusarh::Saielsr46,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR47"]
    #[inline(always)]
    pub fn saielsr47(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        icusarh::Saielsr47,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            icusarh::Saielsr47,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR48"]
    #[inline(always)]
    pub fn saielsr48(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        icusarh::Saielsr48,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            icusarh::Saielsr48,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR49"]
    #[inline(always)]
    pub fn saielsr49(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        icusarh::Saielsr49,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            icusarh::Saielsr49,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR50"]
    #[inline(always)]
    pub fn saielsr50(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        icusarh::Saielsr50,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            icusarh::Saielsr50,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR51"]
    #[inline(always)]
    pub fn saielsr51(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        icusarh::Saielsr51,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            icusarh::Saielsr51,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR52"]
    #[inline(always)]
    pub fn saielsr52(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        icusarh::Saielsr52,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            icusarh::Saielsr52,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR53"]
    #[inline(always)]
    pub fn saielsr53(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        icusarh::Saielsr53,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            icusarh::Saielsr53,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR54"]
    #[inline(always)]
    pub fn saielsr54(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        icusarh::Saielsr54,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            icusarh::Saielsr54,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR55"]
    #[inline(always)]
    pub fn saielsr55(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        icusarh::Saielsr55,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            icusarh::Saielsr55,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR56"]
    #[inline(always)]
    pub fn saielsr56(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        icusarh::Saielsr56,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            icusarh::Saielsr56,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR57"]
    #[inline(always)]
    pub fn saielsr57(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        icusarh::Saielsr57,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            icusarh::Saielsr57,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR58"]
    #[inline(always)]
    pub fn saielsr58(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        icusarh::Saielsr58,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            icusarh::Saielsr58,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR59"]
    #[inline(always)]
    pub fn saielsr59(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        icusarh::Saielsr59,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            icusarh::Saielsr59,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR60"]
    #[inline(always)]
    pub fn saielsr60(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        icusarh::Saielsr60,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            icusarh::Saielsr60,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR61"]
    #[inline(always)]
    pub fn saielsr61(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        icusarh::Saielsr61,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            icusarh::Saielsr61,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR62"]
    #[inline(always)]
    pub fn saielsr62(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        icusarh::Saielsr62,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            icusarh::Saielsr62,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR63"]
    #[inline(always)]
    pub fn saielsr63(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        icusarh::Saielsr63,
        Icusarh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            icusarh::Saielsr63,
            Icusarh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icusarh {
    #[inline(always)]
    fn default() -> Icusarh {
        <crate::RegValueT<Icusarh_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icusarh {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr32_SPEC;
    pub type Saielsr32 = crate::EnumBitfieldStruct<u8, Saielsr32_SPEC>;
    impl Saielsr32 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr33_SPEC;
    pub type Saielsr33 = crate::EnumBitfieldStruct<u8, Saielsr33_SPEC>;
    impl Saielsr33 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr34_SPEC;
    pub type Saielsr34 = crate::EnumBitfieldStruct<u8, Saielsr34_SPEC>;
    impl Saielsr34 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr35_SPEC;
    pub type Saielsr35 = crate::EnumBitfieldStruct<u8, Saielsr35_SPEC>;
    impl Saielsr35 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr36_SPEC;
    pub type Saielsr36 = crate::EnumBitfieldStruct<u8, Saielsr36_SPEC>;
    impl Saielsr36 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr37_SPEC;
    pub type Saielsr37 = crate::EnumBitfieldStruct<u8, Saielsr37_SPEC>;
    impl Saielsr37 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr38_SPEC;
    pub type Saielsr38 = crate::EnumBitfieldStruct<u8, Saielsr38_SPEC>;
    impl Saielsr38 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr39_SPEC;
    pub type Saielsr39 = crate::EnumBitfieldStruct<u8, Saielsr39_SPEC>;
    impl Saielsr39 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr40_SPEC;
    pub type Saielsr40 = crate::EnumBitfieldStruct<u8, Saielsr40_SPEC>;
    impl Saielsr40 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr41_SPEC;
    pub type Saielsr41 = crate::EnumBitfieldStruct<u8, Saielsr41_SPEC>;
    impl Saielsr41 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr42_SPEC;
    pub type Saielsr42 = crate::EnumBitfieldStruct<u8, Saielsr42_SPEC>;
    impl Saielsr42 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr43_SPEC;
    pub type Saielsr43 = crate::EnumBitfieldStruct<u8, Saielsr43_SPEC>;
    impl Saielsr43 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr44_SPEC;
    pub type Saielsr44 = crate::EnumBitfieldStruct<u8, Saielsr44_SPEC>;
    impl Saielsr44 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr45_SPEC;
    pub type Saielsr45 = crate::EnumBitfieldStruct<u8, Saielsr45_SPEC>;
    impl Saielsr45 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr46_SPEC;
    pub type Saielsr46 = crate::EnumBitfieldStruct<u8, Saielsr46_SPEC>;
    impl Saielsr46 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr47_SPEC;
    pub type Saielsr47 = crate::EnumBitfieldStruct<u8, Saielsr47_SPEC>;
    impl Saielsr47 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr48_SPEC;
    pub type Saielsr48 = crate::EnumBitfieldStruct<u8, Saielsr48_SPEC>;
    impl Saielsr48 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr49_SPEC;
    pub type Saielsr49 = crate::EnumBitfieldStruct<u8, Saielsr49_SPEC>;
    impl Saielsr49 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr50_SPEC;
    pub type Saielsr50 = crate::EnumBitfieldStruct<u8, Saielsr50_SPEC>;
    impl Saielsr50 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr51_SPEC;
    pub type Saielsr51 = crate::EnumBitfieldStruct<u8, Saielsr51_SPEC>;
    impl Saielsr51 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr52_SPEC;
    pub type Saielsr52 = crate::EnumBitfieldStruct<u8, Saielsr52_SPEC>;
    impl Saielsr52 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr53_SPEC;
    pub type Saielsr53 = crate::EnumBitfieldStruct<u8, Saielsr53_SPEC>;
    impl Saielsr53 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr54_SPEC;
    pub type Saielsr54 = crate::EnumBitfieldStruct<u8, Saielsr54_SPEC>;
    impl Saielsr54 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr55_SPEC;
    pub type Saielsr55 = crate::EnumBitfieldStruct<u8, Saielsr55_SPEC>;
    impl Saielsr55 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr56_SPEC;
    pub type Saielsr56 = crate::EnumBitfieldStruct<u8, Saielsr56_SPEC>;
    impl Saielsr56 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr57_SPEC;
    pub type Saielsr57 = crate::EnumBitfieldStruct<u8, Saielsr57_SPEC>;
    impl Saielsr57 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr58_SPEC;
    pub type Saielsr58 = crate::EnumBitfieldStruct<u8, Saielsr58_SPEC>;
    impl Saielsr58 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr59_SPEC;
    pub type Saielsr59 = crate::EnumBitfieldStruct<u8, Saielsr59_SPEC>;
    impl Saielsr59 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr60_SPEC;
    pub type Saielsr60 = crate::EnumBitfieldStruct<u8, Saielsr60_SPEC>;
    impl Saielsr60 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr61_SPEC;
    pub type Saielsr61 = crate::EnumBitfieldStruct<u8, Saielsr61_SPEC>;
    impl Saielsr61 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr62_SPEC;
    pub type Saielsr62 = crate::EnumBitfieldStruct<u8, Saielsr62_SPEC>;
    impl Saielsr62 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr63_SPEC;
    pub type Saielsr63 = crate::EnumBitfieldStruct<u8, Saielsr63_SPEC>;
    impl Saielsr63 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icusari_SPEC;
impl crate::sealed::RegSpec for Icusari_SPEC {
    type DataType = u32;
}
#[doc = "ICU Security Attribution Register I"]
pub type Icusari = crate::RegValueT<Icusari_SPEC>;

impl Icusari {
    #[doc = "Security attributes of registers for IELSR64"]
    #[inline(always)]
    pub fn saielsr64(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        icusari::Saielsr64,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            icusari::Saielsr64,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR65"]
    #[inline(always)]
    pub fn saielsr65(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        icusari::Saielsr65,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            icusari::Saielsr65,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR66"]
    #[inline(always)]
    pub fn saielsr66(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        icusari::Saielsr66,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            icusari::Saielsr66,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR67"]
    #[inline(always)]
    pub fn saielsr67(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        icusari::Saielsr67,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            icusari::Saielsr67,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR68"]
    #[inline(always)]
    pub fn saielsr68(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        icusari::Saielsr68,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            icusari::Saielsr68,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR69"]
    #[inline(always)]
    pub fn saielsr69(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        icusari::Saielsr69,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            icusari::Saielsr69,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR70"]
    #[inline(always)]
    pub fn saielsr70(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        icusari::Saielsr70,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            icusari::Saielsr70,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR71"]
    #[inline(always)]
    pub fn saielsr71(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        icusari::Saielsr71,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            icusari::Saielsr71,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR72"]
    #[inline(always)]
    pub fn saielsr72(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        icusari::Saielsr72,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            icusari::Saielsr72,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR73"]
    #[inline(always)]
    pub fn saielsr73(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        icusari::Saielsr73,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            icusari::Saielsr73,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR74"]
    #[inline(always)]
    pub fn saielsr74(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        icusari::Saielsr74,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            icusari::Saielsr74,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR75"]
    #[inline(always)]
    pub fn saielsr75(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        icusari::Saielsr75,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            icusari::Saielsr75,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR76"]
    #[inline(always)]
    pub fn saielsr76(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        icusari::Saielsr76,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            icusari::Saielsr76,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR77"]
    #[inline(always)]
    pub fn saielsr77(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        icusari::Saielsr77,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            icusari::Saielsr77,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR78"]
    #[inline(always)]
    pub fn saielsr78(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        icusari::Saielsr78,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            icusari::Saielsr78,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR79"]
    #[inline(always)]
    pub fn saielsr79(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        icusari::Saielsr79,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            icusari::Saielsr79,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR80"]
    #[inline(always)]
    pub fn saielsr80(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        icusari::Saielsr80,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            icusari::Saielsr80,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR81"]
    #[inline(always)]
    pub fn saielsr81(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        icusari::Saielsr81,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            icusari::Saielsr81,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR82"]
    #[inline(always)]
    pub fn saielsr82(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        icusari::Saielsr82,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            icusari::Saielsr82,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR83"]
    #[inline(always)]
    pub fn saielsr83(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        icusari::Saielsr83,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            icusari::Saielsr83,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR84"]
    #[inline(always)]
    pub fn saielsr84(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        icusari::Saielsr84,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            icusari::Saielsr84,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR85"]
    #[inline(always)]
    pub fn saielsr85(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        icusari::Saielsr85,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            icusari::Saielsr85,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR86"]
    #[inline(always)]
    pub fn saielsr86(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        icusari::Saielsr86,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            icusari::Saielsr86,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR87"]
    #[inline(always)]
    pub fn saielsr87(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        icusari::Saielsr87,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            icusari::Saielsr87,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR88"]
    #[inline(always)]
    pub fn saielsr88(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        icusari::Saielsr88,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            icusari::Saielsr88,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR89"]
    #[inline(always)]
    pub fn saielsr89(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        icusari::Saielsr89,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            icusari::Saielsr89,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR90"]
    #[inline(always)]
    pub fn saielsr90(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        icusari::Saielsr90,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            icusari::Saielsr90,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR91"]
    #[inline(always)]
    pub fn saielsr91(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        icusari::Saielsr91,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            icusari::Saielsr91,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR92"]
    #[inline(always)]
    pub fn saielsr92(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        icusari::Saielsr92,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            icusari::Saielsr92,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR93"]
    #[inline(always)]
    pub fn saielsr93(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        icusari::Saielsr93,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            icusari::Saielsr93,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR94"]
    #[inline(always)]
    pub fn saielsr94(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        icusari::Saielsr94,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            icusari::Saielsr94,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for IELSR95"]
    #[inline(always)]
    pub fn saielsr95(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        icusari::Saielsr95,
        Icusari_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            icusari::Saielsr95,
            Icusari_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icusari {
    #[inline(always)]
    fn default() -> Icusari {
        <crate::RegValueT<Icusari_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icusari {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr64_SPEC;
    pub type Saielsr64 = crate::EnumBitfieldStruct<u8, Saielsr64_SPEC>;
    impl Saielsr64 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr65_SPEC;
    pub type Saielsr65 = crate::EnumBitfieldStruct<u8, Saielsr65_SPEC>;
    impl Saielsr65 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr66_SPEC;
    pub type Saielsr66 = crate::EnumBitfieldStruct<u8, Saielsr66_SPEC>;
    impl Saielsr66 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr67_SPEC;
    pub type Saielsr67 = crate::EnumBitfieldStruct<u8, Saielsr67_SPEC>;
    impl Saielsr67 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr68_SPEC;
    pub type Saielsr68 = crate::EnumBitfieldStruct<u8, Saielsr68_SPEC>;
    impl Saielsr68 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr69_SPEC;
    pub type Saielsr69 = crate::EnumBitfieldStruct<u8, Saielsr69_SPEC>;
    impl Saielsr69 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr70_SPEC;
    pub type Saielsr70 = crate::EnumBitfieldStruct<u8, Saielsr70_SPEC>;
    impl Saielsr70 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr71_SPEC;
    pub type Saielsr71 = crate::EnumBitfieldStruct<u8, Saielsr71_SPEC>;
    impl Saielsr71 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr72_SPEC;
    pub type Saielsr72 = crate::EnumBitfieldStruct<u8, Saielsr72_SPEC>;
    impl Saielsr72 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr73_SPEC;
    pub type Saielsr73 = crate::EnumBitfieldStruct<u8, Saielsr73_SPEC>;
    impl Saielsr73 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr74_SPEC;
    pub type Saielsr74 = crate::EnumBitfieldStruct<u8, Saielsr74_SPEC>;
    impl Saielsr74 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr75_SPEC;
    pub type Saielsr75 = crate::EnumBitfieldStruct<u8, Saielsr75_SPEC>;
    impl Saielsr75 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr76_SPEC;
    pub type Saielsr76 = crate::EnumBitfieldStruct<u8, Saielsr76_SPEC>;
    impl Saielsr76 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr77_SPEC;
    pub type Saielsr77 = crate::EnumBitfieldStruct<u8, Saielsr77_SPEC>;
    impl Saielsr77 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr78_SPEC;
    pub type Saielsr78 = crate::EnumBitfieldStruct<u8, Saielsr78_SPEC>;
    impl Saielsr78 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr79_SPEC;
    pub type Saielsr79 = crate::EnumBitfieldStruct<u8, Saielsr79_SPEC>;
    impl Saielsr79 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr80_SPEC;
    pub type Saielsr80 = crate::EnumBitfieldStruct<u8, Saielsr80_SPEC>;
    impl Saielsr80 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr81_SPEC;
    pub type Saielsr81 = crate::EnumBitfieldStruct<u8, Saielsr81_SPEC>;
    impl Saielsr81 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr82_SPEC;
    pub type Saielsr82 = crate::EnumBitfieldStruct<u8, Saielsr82_SPEC>;
    impl Saielsr82 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr83_SPEC;
    pub type Saielsr83 = crate::EnumBitfieldStruct<u8, Saielsr83_SPEC>;
    impl Saielsr83 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr84_SPEC;
    pub type Saielsr84 = crate::EnumBitfieldStruct<u8, Saielsr84_SPEC>;
    impl Saielsr84 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr85_SPEC;
    pub type Saielsr85 = crate::EnumBitfieldStruct<u8, Saielsr85_SPEC>;
    impl Saielsr85 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr86_SPEC;
    pub type Saielsr86 = crate::EnumBitfieldStruct<u8, Saielsr86_SPEC>;
    impl Saielsr86 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr87_SPEC;
    pub type Saielsr87 = crate::EnumBitfieldStruct<u8, Saielsr87_SPEC>;
    impl Saielsr87 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr88_SPEC;
    pub type Saielsr88 = crate::EnumBitfieldStruct<u8, Saielsr88_SPEC>;
    impl Saielsr88 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr89_SPEC;
    pub type Saielsr89 = crate::EnumBitfieldStruct<u8, Saielsr89_SPEC>;
    impl Saielsr89 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr90_SPEC;
    pub type Saielsr90 = crate::EnumBitfieldStruct<u8, Saielsr90_SPEC>;
    impl Saielsr90 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr91_SPEC;
    pub type Saielsr91 = crate::EnumBitfieldStruct<u8, Saielsr91_SPEC>;
    impl Saielsr91 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr92_SPEC;
    pub type Saielsr92 = crate::EnumBitfieldStruct<u8, Saielsr92_SPEC>;
    impl Saielsr92 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr93_SPEC;
    pub type Saielsr93 = crate::EnumBitfieldStruct<u8, Saielsr93_SPEC>;
    impl Saielsr93 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr94_SPEC;
    pub type Saielsr94 = crate::EnumBitfieldStruct<u8, Saielsr94_SPEC>;
    impl Saielsr94 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Saielsr95_SPEC;
    pub type Saielsr95 = crate::EnumBitfieldStruct<u8, Saielsr95_SPEC>;
    impl Saielsr95 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bussara_SPEC;
impl crate::sealed::RegSpec for Bussara_SPEC {
    type DataType = u32;
}
#[doc = "BUS Security Attribution Register A"]
pub type Bussara = crate::RegValueT<Bussara_SPEC>;

impl Bussara {
    #[doc = "BUS Security attributes A0."]
    #[inline(always)]
    pub fn bussa0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, bussara::Bussa0, Bussara_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,bussara::Bussa0, Bussara_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Security attributes of registers for QOS register"]
    #[inline(always)]
    pub fn qossa(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, bussara::Qossa, Bussara_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,bussara::Qossa, Bussara_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Bussara_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Bussara_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bussara {
    #[inline(always)]
    fn default() -> Bussara {
        <crate::RegValueT<Bussara_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bussara {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bussa0_SPEC;
    pub type Bussa0 = crate::EnumBitfieldStruct<u8, Bussa0_SPEC>;
    impl Bussa0 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Qossa_SPEC;
    pub type Qossa = crate::EnumBitfieldStruct<u8, Qossa_SPEC>;
    impl Qossa {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bussarb_SPEC;
impl crate::sealed::RegSpec for Bussarb_SPEC {
    type DataType = u32;
}
#[doc = "BUS Security Attribution Register B"]
pub type Bussarb = crate::RegValueT<Bussarb_SPEC>;

impl Bussarb {
    #[doc = "BUS Security attributes B0."]
    #[inline(always)]
    pub fn bussb0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, bussarb::Bussb0, Bussarb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,bussarb::Bussb0, Bussarb_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Bussarb_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Bussarb_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bussarb {
    #[inline(always)]
    fn default() -> Bussarb {
        <crate::RegValueT<Bussarb_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bussarb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bussb0_SPEC;
    pub type Bussb0 = crate::EnumBitfieldStruct<u8, Bussb0_SPEC>;
    impl Bussb0 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bussarc_SPEC;
impl crate::sealed::RegSpec for Bussarc_SPEC {
    type DataType = u32;
}
#[doc = "BUS Security Attribution Register C (External BUS)"]
pub type Bussarc = crate::RegValueT<Bussarc_SPEC>;

impl Bussarc {
    #[doc = "External BUS Security attributes."]
    #[inline(always)]
    pub fn bussc0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, bussarc::Bussc0, Bussarc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,bussarc::Bussc0, Bussarc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Bussarc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Bussarc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bussarc {
    #[inline(always)]
    fn default() -> Bussarc {
        <crate::RegValueT<Bussarc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bussarc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bussc0_SPEC;
    pub type Bussc0 = crate::EnumBitfieldStruct<u8, Bussc0_SPEC>;
    impl Bussc0 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busparc_SPEC;
impl crate::sealed::RegSpec for Busparc_SPEC {
    type DataType = u32;
}
#[doc = "BUS Privileged Attribution Register C (External BUS)"]
pub type Busparc = crate::RegValueT<Busparc_SPEC>;

impl Busparc {
    #[doc = "External BUS Privileged attributes."]
    #[inline(always)]
    pub fn buspa0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, busparc::Buspa0, Busparc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,busparc::Buspa0, Busparc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 1111111111111111. The write value should be 1111111111111111."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Busparc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Busparc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Busparc {
    #[inline(always)]
    fn default() -> Busparc {
        <crate::RegValueT<Busparc_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod busparc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Buspa0_SPEC;
    pub type Buspa0 = crate::EnumBitfieldStruct<u8, Buspa0_SPEC>;
    impl Buspa0 {
        #[doc = "privileged."]
        pub const _0: Self = Self::new(0);
        #[doc = "unprivileged."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpusara_SPEC;
impl crate::sealed::RegSpec for Mmpusara_SPEC {
    type DataType = u32;
}
#[doc = "Master MPU Security Attribution Register A"]
pub type Mmpusara = crate::RegValueT<Mmpusara_SPEC>;

impl Mmpusara {
    #[doc = "MMPUA Security attributes0"]
    #[inline(always)]
    pub fn mmpusa0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpusara::Mmpusa0,
        Mmpusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpusara::Mmpusa0,
            Mmpusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "MMPUA Security attributes1"]
    #[inline(always)]
    pub fn mmpusa1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mmpusara::Mmpusa1,
        Mmpusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mmpusara::Mmpusa1,
            Mmpusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "MMPUA Security attributes2"]
    #[inline(always)]
    pub fn mmpusa2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mmpusara::Mmpusa2,
        Mmpusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mmpusara::Mmpusa2,
            Mmpusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "MMPUA Security attributes3"]
    #[inline(always)]
    pub fn mmpusa3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        mmpusara::Mmpusa3,
        Mmpusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            mmpusara::Mmpusa3,
            Mmpusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "MMPUA Security attributes4"]
    #[inline(always)]
    pub fn mmpusa4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        mmpusara::Mmpusa4,
        Mmpusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            mmpusara::Mmpusa4,
            Mmpusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "MMPUA Security attributes5"]
    #[inline(always)]
    pub fn mmpusa5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        mmpusara::Mmpusa5,
        Mmpusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            mmpusara::Mmpusa5,
            Mmpusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "MMPUA Security attributes6"]
    #[inline(always)]
    pub fn mmpusa6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        mmpusara::Mmpusa6,
        Mmpusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            mmpusara::Mmpusa6,
            Mmpusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "MMPUA Security attributes7"]
    #[inline(always)]
    pub fn mmpusa7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        mmpusara::Mmpusa7,
        Mmpusara_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            mmpusara::Mmpusa7,
            Mmpusara_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Mmpusara_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Mmpusara_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpusara {
    #[inline(always)]
    fn default() -> Mmpusara {
        <crate::RegValueT<Mmpusara_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpusara {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmpusa0_SPEC;
    pub type Mmpusa0 = crate::EnumBitfieldStruct<u8, Mmpusa0_SPEC>;
    impl Mmpusa0 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmpusa1_SPEC;
    pub type Mmpusa1 = crate::EnumBitfieldStruct<u8, Mmpusa1_SPEC>;
    impl Mmpusa1 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmpusa2_SPEC;
    pub type Mmpusa2 = crate::EnumBitfieldStruct<u8, Mmpusa2_SPEC>;
    impl Mmpusa2 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmpusa3_SPEC;
    pub type Mmpusa3 = crate::EnumBitfieldStruct<u8, Mmpusa3_SPEC>;
    impl Mmpusa3 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmpusa4_SPEC;
    pub type Mmpusa4 = crate::EnumBitfieldStruct<u8, Mmpusa4_SPEC>;
    impl Mmpusa4 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmpusa5_SPEC;
    pub type Mmpusa5 = crate::EnumBitfieldStruct<u8, Mmpusa5_SPEC>;
    impl Mmpusa5 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmpusa6_SPEC;
    pub type Mmpusa6 = crate::EnumBitfieldStruct<u8, Mmpusa6_SPEC>;
    impl Mmpusa6 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmpusa7_SPEC;
    pub type Mmpusa7 = crate::EnumBitfieldStruct<u8, Mmpusa7_SPEC>;
    impl Mmpusa7 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpusarb_SPEC;
impl crate::sealed::RegSpec for Mmpusarb_SPEC {
    type DataType = u32;
}
#[doc = "Master MPU Security Attribution Register B"]
pub type Mmpusarb = crate::RegValueT<Mmpusarb_SPEC>;

impl Mmpusarb {
    #[doc = "MMPUB0 Security attributes."]
    #[inline(always)]
    pub fn mmpubsa0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpusarb::Mmpubsa0,
        Mmpusarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpusarb::Mmpubsa0,
            Mmpusarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "MMPUB8 Security attributes."]
    #[inline(always)]
    pub fn mmpubsa8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        mmpusarb::Mmpubsa8,
        Mmpusarb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            mmpusarb::Mmpubsa8,
            Mmpusarb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Mmpusarb_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Mmpusarb_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpusarb {
    #[inline(always)]
    fn default() -> Mmpusarb {
        <crate::RegValueT<Mmpusarb_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpusarb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmpubsa0_SPEC;
    pub type Mmpubsa0 = crate::EnumBitfieldStruct<u8, Mmpubsa0_SPEC>;
    impl Mmpubsa0 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mmpubsa8_SPEC;
    pub type Mmpubsa8 = crate::EnumBitfieldStruct<u8, Mmpubsa8_SPEC>;
    impl Mmpubsa8 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debugsar_SPEC;
impl crate::sealed::RegSpec for Debugsar_SPEC {
    type DataType = u32;
}
#[doc = "Debug Security Attribution Register"]
pub type Debugsar = crate::RegValueT<Debugsar_SPEC>;

impl Debugsar {
    #[doc = "Security attributes of registers for the DEBUG register"]
    #[inline(always)]
    pub fn dbgsa0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        debugsar::Dbgsa0,
        Debugsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            debugsar::Dbgsa0,
            Debugsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Debugsar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Debugsar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Debugsar {
    #[inline(always)]
    fn default() -> Debugsar {
        <crate::RegValueT<Debugsar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod debugsar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dbgsa0_SPEC;
    pub type Dbgsa0 = crate::EnumBitfieldStruct<u8, Dbgsa0_SPEC>;
    impl Dbgsa0 {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmacchsar_SPEC;
impl crate::sealed::RegSpec for Dmacchsar_SPEC {
    type DataType = u32;
}
#[doc = "DMAC channel Security Attribution Register"]
pub type Dmacchsar = crate::RegValueT<Dmacchsar_SPEC>;

impl Dmacchsar {
    #[doc = "Security attributes of registers for DMAC channel 0"]
    #[inline(always)]
    pub fn sadmac0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dmacchsar::Sadmac0,
        Dmacchsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dmacchsar::Sadmac0,
            Dmacchsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for DMAC channel 1"]
    #[inline(always)]
    pub fn sadmac1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dmacchsar::Sadmac1,
        Dmacchsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dmacchsar::Sadmac1,
            Dmacchsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for DMAC channel 2"]
    #[inline(always)]
    pub fn sadmac2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dmacchsar::Sadmac2,
        Dmacchsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dmacchsar::Sadmac2,
            Dmacchsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for DMAC channel 3"]
    #[inline(always)]
    pub fn sadmac3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dmacchsar::Sadmac3,
        Dmacchsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dmacchsar::Sadmac3,
            Dmacchsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for DMAC channel 4"]
    #[inline(always)]
    pub fn sadmac4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dmacchsar::Sadmac4,
        Dmacchsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dmacchsar::Sadmac4,
            Dmacchsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for DMAC channel 5"]
    #[inline(always)]
    pub fn sadmac5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dmacchsar::Sadmac5,
        Dmacchsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dmacchsar::Sadmac5,
            Dmacchsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for DMAC channel 6"]
    #[inline(always)]
    pub fn sadmac6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        dmacchsar::Sadmac6,
        Dmacchsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            dmacchsar::Sadmac6,
            Dmacchsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Security attributes of registers for DMAC channel 7"]
    #[inline(always)]
    pub fn sadmac7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dmacchsar::Sadmac7,
        Dmacchsar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dmacchsar::Sadmac7,
            Dmacchsar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Dmacchsar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Dmacchsar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dmacchsar {
    #[inline(always)]
    fn default() -> Dmacchsar {
        <crate::RegValueT<Dmacchsar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dmacchsar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadmac0_SPEC;
    pub type Sadmac0 = crate::EnumBitfieldStruct<u8, Sadmac0_SPEC>;
    impl Sadmac0 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadmac1_SPEC;
    pub type Sadmac1 = crate::EnumBitfieldStruct<u8, Sadmac1_SPEC>;
    impl Sadmac1 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadmac2_SPEC;
    pub type Sadmac2 = crate::EnumBitfieldStruct<u8, Sadmac2_SPEC>;
    impl Sadmac2 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadmac3_SPEC;
    pub type Sadmac3 = crate::EnumBitfieldStruct<u8, Sadmac3_SPEC>;
    impl Sadmac3 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadmac4_SPEC;
    pub type Sadmac4 = crate::EnumBitfieldStruct<u8, Sadmac4_SPEC>;
    impl Sadmac4 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadmac5_SPEC;
    pub type Sadmac5 = crate::EnumBitfieldStruct<u8, Sadmac5_SPEC>;
    impl Sadmac5 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadmac6_SPEC;
    pub type Sadmac6 = crate::EnumBitfieldStruct<u8, Sadmac6_SPEC>;
    impl Sadmac6 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadmac7_SPEC;
    pub type Sadmac7 = crate::EnumBitfieldStruct<u8, Sadmac7_SPEC>;
    impl Sadmac7 {
        #[doc = "Secure."]
        pub const _0: Self = Self::new(0);
        #[doc = "Non-secure."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmacchpar_SPEC;
impl crate::sealed::RegSpec for Dmacchpar_SPEC {
    type DataType = u32;
}
#[doc = "DMAC channel Privileged Attribution Register"]
pub type Dmacchpar = crate::RegValueT<Dmacchpar_SPEC>;

impl Dmacchpar {
    #[doc = "Privileged attributes of registers for DMAC channel 0"]
    #[inline(always)]
    pub fn padmac0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dmacchpar::Padmac0,
        Dmacchpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dmacchpar::Padmac0,
            Dmacchpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Privileged attributes of registers for DMAC channel 1"]
    #[inline(always)]
    pub fn padmac1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dmacchpar::Padmac1,
        Dmacchpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dmacchpar::Padmac1,
            Dmacchpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Privileged attributes of registers for DMAC channel 2"]
    #[inline(always)]
    pub fn padmac2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dmacchpar::Padmac2,
        Dmacchpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dmacchpar::Padmac2,
            Dmacchpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Privileged attributes of registers for DMAC channel 3"]
    #[inline(always)]
    pub fn padmac3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dmacchpar::Padmac3,
        Dmacchpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dmacchpar::Padmac3,
            Dmacchpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Privileged attributes of registers for DMAC channel 4"]
    #[inline(always)]
    pub fn padmac4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dmacchpar::Padmac4,
        Dmacchpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dmacchpar::Padmac4,
            Dmacchpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Privileged attributes of registers for DMAC channel 5"]
    #[inline(always)]
    pub fn padmac5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dmacchpar::Padmac5,
        Dmacchpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dmacchpar::Padmac5,
            Dmacchpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Privileged attributes of registers for DMAC channel 6"]
    #[inline(always)]
    pub fn padmac6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        dmacchpar::Padmac6,
        Dmacchpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            dmacchpar::Padmac6,
            Dmacchpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Privileged attributes of registers for DMAC channel 7"]
    #[inline(always)]
    pub fn padmac7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dmacchpar::Padmac7,
        Dmacchpar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dmacchpar::Padmac7,
            Dmacchpar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 1111111111111111. The write value should be 1111111111111111."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Dmacchpar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Dmacchpar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dmacchpar {
    #[inline(always)]
    fn default() -> Dmacchpar {
        <crate::RegValueT<Dmacchpar_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod dmacchpar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padmac0_SPEC;
    pub type Padmac0 = crate::EnumBitfieldStruct<u8, Padmac0_SPEC>;
    impl Padmac0 {
        #[doc = "privileged"]
        pub const _0: Self = Self::new(0);
        #[doc = "unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padmac1_SPEC;
    pub type Padmac1 = crate::EnumBitfieldStruct<u8, Padmac1_SPEC>;
    impl Padmac1 {
        #[doc = "privileged"]
        pub const _0: Self = Self::new(0);
        #[doc = "unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padmac2_SPEC;
    pub type Padmac2 = crate::EnumBitfieldStruct<u8, Padmac2_SPEC>;
    impl Padmac2 {
        #[doc = "privileged"]
        pub const _0: Self = Self::new(0);
        #[doc = "unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padmac3_SPEC;
    pub type Padmac3 = crate::EnumBitfieldStruct<u8, Padmac3_SPEC>;
    impl Padmac3 {
        #[doc = "privileged"]
        pub const _0: Self = Self::new(0);
        #[doc = "unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padmac4_SPEC;
    pub type Padmac4 = crate::EnumBitfieldStruct<u8, Padmac4_SPEC>;
    impl Padmac4 {
        #[doc = "privileged"]
        pub const _0: Self = Self::new(0);
        #[doc = "unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padmac5_SPEC;
    pub type Padmac5 = crate::EnumBitfieldStruct<u8, Padmac5_SPEC>;
    impl Padmac5 {
        #[doc = "privileged"]
        pub const _0: Self = Self::new(0);
        #[doc = "unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padmac6_SPEC;
    pub type Padmac6 = crate::EnumBitfieldStruct<u8, Padmac6_SPEC>;
    impl Padmac6 {
        #[doc = "privileged"]
        pub const _0: Self = Self::new(0);
        #[doc = "unprivileged"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padmac7_SPEC;
    pub type Padmac7 = crate::EnumBitfieldStruct<u8, Padmac7_SPEC>;
    impl Padmac7 {
        #[doc = "privileged"]
        pub const _0: Self = Self::new(0);
        #[doc = "unprivileged"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramsabar_SPEC;
impl crate::sealed::RegSpec for Sramsabar_SPEC {
    type DataType = u32;
}
#[doc = "SRAM Security Attribute Boundary Address Register %s"]
pub type Sramsabar = crate::RegValueT<Sramsabar_SPEC>;

impl Sramsabar {
    #[doc = "These bits are read as 00000000000111111110000000000000. The write value should be 00000000000111111110000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Sramsabar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Sramsabar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sramsabar {
    #[inline(always)]
    fn default() -> Sramsabar {
        <crate::RegValueT<Sramsabar_SPEC> as RegisterValue<_>>::new(2088960)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stbramsabar_SPEC;
impl crate::sealed::RegSpec for Stbramsabar_SPEC {
    type DataType = u32;
}
#[doc = "Standby SRAM Security Attribute Boundary Address Register"]
pub type Stbramsabar = crate::RegValueT<Stbramsabar_SPEC>;

impl Stbramsabar {
    #[doc = "These bits are read as 00000000000000000111111110000000. The write value should be 00000000000000000111111110000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Stbramsabar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Stbramsabar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Stbramsabar {
    #[inline(always)]
    fn default() -> Stbramsabar {
        <crate::RegValueT<Stbramsabar_SPEC> as RegisterValue<_>>::new(32640)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StbrampabarNs_SPEC;
impl crate::sealed::RegSpec for StbrampabarNs_SPEC {
    type DataType = u32;
}
#[doc = "Standby SRAM Privileged Attribute Boundary Address Register for Non-Secure region"]
pub type StbrampabarNs = crate::RegValueT<StbrampabarNs_SPEC>;

impl StbrampabarNs {
    #[doc = "These bits are read as 00000000000000000000000000000000. The write value should be 00000000000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, StbrampabarNs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            StbrampabarNs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for StbrampabarNs {
    #[inline(always)]
    fn default() -> StbrampabarNs {
        <crate::RegValueT<StbrampabarNs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StbrampabarS_SPEC;
impl crate::sealed::RegSpec for StbrampabarS_SPEC {
    type DataType = u32;
}
#[doc = "Standby SRAM Privileged Attribute Boundary Address Register for Secure region"]
pub type StbrampabarS = crate::RegValueT<StbrampabarS_SPEC>;

impl StbrampabarS {
    #[doc = "These bits are read as 00000000000000000000000000000000. The write value should be 00000000000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, StbrampabarS_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, StbrampabarS_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for StbrampabarS {
    #[inline(always)]
    fn default() -> StbrampabarS {
        <crate::RegValueT<StbrampabarS_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tevtrcr_SPEC;
impl crate::sealed::RegSpec for Tevtrcr_SPEC {
    type DataType = u32;
}
#[doc = "Trusted EVenT Route Control Register"]
pub type Tevtrcr = crate::RegValueT<Tevtrcr_SPEC>;

impl Tevtrcr {
    #[doc = "Trusted Event Route Control"]
    #[inline(always)]
    pub fn tevte(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, tevtrcr::Tevte, Tevtrcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,tevtrcr::Tevte, Tevtrcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Tevtrcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Tevtrcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tevtrcr {
    #[inline(always)]
    fn default() -> Tevtrcr {
        <crate::RegValueT<Tevtrcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tevtrcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tevte_SPEC;
    pub type Tevte = crate::EnumBitfieldStruct<u8, Tevte_SPEC>;
    impl Tevte {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busmemerrsts_SPEC;
impl crate::sealed::RegSpec for Busmemerrsts_SPEC {
    type DataType = u8;
}
#[doc = "BUS Memory Error Status Register"]
pub type Busmemerrsts = crate::RegValueT<Busmemerrsts_SPEC>;

impl Busmemerrsts {
    #[doc = "BUS Error STatuS"]
    #[inline(always)]
    pub fn busests(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        busmemerrsts::Busests,
        Busmemerrsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            busmemerrsts::Busests,
            Busmemerrsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Local_MEMory0 Error STatuS"]
    #[inline(always)]
    pub fn lmem0ests(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        busmemerrsts::Lmem0Ests,
        Busmemerrsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            busmemerrsts::Lmem0Ests,
            Busmemerrsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Local_MEMory1 Error STatuS"]
    #[inline(always)]
    pub fn lmem1ests(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        busmemerrsts::Lmem1Ests,
        Busmemerrsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            busmemerrsts::Lmem1Ests,
            Busmemerrsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "SRAM Error STatuS"]
    #[inline(always)]
    pub fn sramests(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        busmemerrsts::Sramests,
        Busmemerrsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            busmemerrsts::Sramests,
            Busmemerrsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Busmemerrsts {
    #[inline(always)]
    fn default() -> Busmemerrsts {
        <crate::RegValueT<Busmemerrsts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busmemerrsts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Busests_SPEC;
    pub type Busests = crate::EnumBitfieldStruct<u8, Busests_SPEC>;
    impl Busests {
        #[doc = "No error occurred"]
        pub const _0: Self = Self::new(0);
        #[doc = "Error occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lmem0Ests_SPEC;
    pub type Lmem0Ests = crate::EnumBitfieldStruct<u8, Lmem0Ests_SPEC>;
    impl Lmem0Ests {
        #[doc = "No error occurred"]
        pub const _0: Self = Self::new(0);
        #[doc = "Error occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lmem1Ests_SPEC;
    pub type Lmem1Ests = crate::EnumBitfieldStruct<u8, Lmem1Ests_SPEC>;
    impl Lmem1Ests {
        #[doc = "No error occurred"]
        pub const _0: Self = Self::new(0);
        #[doc = "Error occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sramests_SPEC;
    pub type Sramests = crate::EnumBitfieldStruct<u8, Sramests_SPEC>;
    impl Sramests {
        #[doc = "No error occurred"]
        pub const _0: Self = Self::new(0);
        #[doc = "Error occurred."]
        pub const _1: Self = Self::new(1);
    }
}
