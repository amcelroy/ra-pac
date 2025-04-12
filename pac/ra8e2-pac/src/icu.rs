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
// Generated from SVD 1.00.01, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:19:56 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Interrupt Controller"]
unsafe impl ::core::marker::Send for super::Icu {}
unsafe impl ::core::marker::Sync for super::Icu {}
impl super::Icu {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "Non-Maskable Interrupt Enable Register"]
    #[inline(always)]
    pub const fn nmier(&self) -> &'static crate::common::Reg<self::Nmier_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Nmier_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[doc = "Non-Maskable Interrupt Status Clear Register"]
    #[inline(always)]
    pub const fn nmiclr(
        &self,
    ) -> &'static crate::common::Reg<self::Nmiclr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Nmiclr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(272usize),
            )
        }
    }

    #[doc = "Non-Maskable Interrupt Status Register"]
    #[inline(always)]
    pub const fn nmisr(&self) -> &'static crate::common::Reg<self::Nmisr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Nmisr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(288usize),
            )
        }
    }

    #[doc = "Wake Up Interrupt Enable Register 0"]
    #[inline(always)]
    pub const fn wupen0(
        &self,
    ) -> &'static crate::common::Reg<self::Wupen0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Wupen0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(416usize),
            )
        }
    }

    #[doc = "Wake Up interrupt enable register 1"]
    #[inline(always)]
    pub const fn wupen1(
        &self,
    ) -> &'static crate::common::Reg<self::Wupen1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Wupen1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(420usize),
            )
        }
    }

    #[doc = "ICU Event Link Setting Register %s"]
    #[inline(always)]
    pub const fn ielsr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Ielsr_SPEC, crate::common::RW>,
        96,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x300usize))
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nmier_SPEC;
impl crate::sealed::RegSpec for Nmier_SPEC {
    type DataType = u16;
}
#[doc = "Non-Maskable Interrupt Enable Register"]
pub type Nmier = crate::RegValueT<Nmier_SPEC>;

impl Nmier {
    #[doc = "IWDT Underflow/Refresh Error Interrupt Enable"]
    #[inline(always)]
    pub fn iwdten(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, nmier::Iwdten, Nmier_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,nmier::Iwdten, Nmier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "WDT Underflow/Refresh Error Interrupt Enable"]
    #[inline(always)]
    pub fn wdten(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, nmier::Wdten, Nmier_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,nmier::Wdten, Nmier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Voltage monitor 1 Interrupt Enable"]
    #[inline(always)]
    pub fn pvd1en(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, nmier::Pvd1En, Nmier_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,nmier::Pvd1En, Nmier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Voltage monitor 2 Interrupt Enable"]
    #[inline(always)]
    pub fn pvd2en(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, nmier::Pvd2En, Nmier_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,nmier::Pvd2En, Nmier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Main Clock Oscillation Stop Detection Interrupt Enable"]
    #[inline(always)]
    pub fn osten(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, nmier::Osten, Nmier_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,nmier::Osten, Nmier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "NMI Pin Interrupt Enable"]
    #[inline(always)]
    pub fn nmien(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, nmier::Nmien, Nmier_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,nmier::Nmien, Nmier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn busen(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, nmier::Busen, Nmier_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,nmier::Busen, Nmier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn cmen(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, nmier::Cmen, Nmier_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,nmier::Cmen, Nmier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn luen(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, nmier::Luen, Nmier_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,nmier::Luen, Nmier_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Nmier {
    #[inline(always)]
    fn default() -> Nmier {
        <crate::RegValueT<Nmier_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod nmier {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iwdten_SPEC;
    pub type Iwdten = crate::EnumBitfieldStruct<u8, Iwdten_SPEC>;
    impl Iwdten {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wdten_SPEC;
    pub type Wdten = crate::EnumBitfieldStruct<u8, Wdten_SPEC>;
    impl Wdten {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pvd1En_SPEC;
    pub type Pvd1En = crate::EnumBitfieldStruct<u8, Pvd1En_SPEC>;
    impl Pvd1En {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pvd2En_SPEC;
    pub type Pvd2En = crate::EnumBitfieldStruct<u8, Pvd2En_SPEC>;
    impl Pvd2En {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Osten_SPEC;
    pub type Osten = crate::EnumBitfieldStruct<u8, Osten_SPEC>;
    impl Osten {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nmien_SPEC;
    pub type Nmien = crate::EnumBitfieldStruct<u8, Nmien_SPEC>;
    impl Nmien {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Busen_SPEC;
    pub type Busen = crate::EnumBitfieldStruct<u8, Busen_SPEC>;
    impl Busen {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmen_SPEC;
    pub type Cmen = crate::EnumBitfieldStruct<u8, Cmen_SPEC>;
    impl Cmen {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Luen_SPEC;
    pub type Luen = crate::EnumBitfieldStruct<u8, Luen_SPEC>;
    impl Luen {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nmiclr_SPEC;
impl crate::sealed::RegSpec for Nmiclr_SPEC {
    type DataType = u16;
}
#[doc = "Non-Maskable Interrupt Status Clear Register"]
pub type Nmiclr = crate::RegValueT<Nmiclr_SPEC>;

impl Nmiclr {
    #[doc = "IWDT Underflow/Refresh Error Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn iwdtclr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, nmiclr::Iwdtclr, Nmiclr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,nmiclr::Iwdtclr, Nmiclr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "WDT Underflow/Refresh Error Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn wdtclr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, nmiclr::Wdtclr, Nmiclr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,nmiclr::Wdtclr, Nmiclr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Voltage Monitor 1 Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn pvd1clr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, nmiclr::Pvd1Clr, Nmiclr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,nmiclr::Pvd1Clr, Nmiclr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Voltage Monitor 2 Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn pvd2clr(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, nmiclr::Pvd2Clr, Nmiclr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,nmiclr::Pvd2Clr, Nmiclr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Oscillation Stop Detection Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn ostclr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, nmiclr::Ostclr, Nmiclr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,nmiclr::Ostclr, Nmiclr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "NMI Pin Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn nmiclr(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, nmiclr::Nmiclr, Nmiclr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,nmiclr::Nmiclr, Nmiclr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn busclr(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, nmiclr::Busclr, Nmiclr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,nmiclr::Busclr, Nmiclr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn cmclr(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, nmiclr::Cmclr, Nmiclr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,nmiclr::Cmclr, Nmiclr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn luclr(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, nmiclr::Luclr, Nmiclr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,nmiclr::Luclr, Nmiclr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Nmiclr {
    #[inline(always)]
    fn default() -> Nmiclr {
        <crate::RegValueT<Nmiclr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod nmiclr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iwdtclr_SPEC;
    pub type Iwdtclr = crate::EnumBitfieldStruct<u8, Iwdtclr_SPEC>;
    impl Iwdtclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);
        #[doc = "Clear the NMISR.IWDTST flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wdtclr_SPEC;
    pub type Wdtclr = crate::EnumBitfieldStruct<u8, Wdtclr_SPEC>;
    impl Wdtclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);
        #[doc = "Clear the NMISR.WDTST flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pvd1Clr_SPEC;
    pub type Pvd1Clr = crate::EnumBitfieldStruct<u8, Pvd1Clr_SPEC>;
    impl Pvd1Clr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);
        #[doc = "Clear the NMISR.PVD1ST flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pvd2Clr_SPEC;
    pub type Pvd2Clr = crate::EnumBitfieldStruct<u8, Pvd2Clr_SPEC>;
    impl Pvd2Clr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);
        #[doc = "Clear the NMISR.PVD2ST flag."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostclr_SPEC;
    pub type Ostclr = crate::EnumBitfieldStruct<u8, Ostclr_SPEC>;
    impl Ostclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);
        #[doc = "Clear the NMISR.OSTST flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nmiclr_SPEC;
    pub type Nmiclr = crate::EnumBitfieldStruct<u8, Nmiclr_SPEC>;
    impl Nmiclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);
        #[doc = "Clear the NMISR.NMIST flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Busclr_SPEC;
    pub type Busclr = crate::EnumBitfieldStruct<u8, Busclr_SPEC>;
    impl Busclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);
        #[doc = "Clear the NMISR.BUSST flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmclr_SPEC;
    pub type Cmclr = crate::EnumBitfieldStruct<u8, Cmclr_SPEC>;
    impl Cmclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);
        #[doc = "Clear the NMISR.CMST flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Luclr_SPEC;
    pub type Luclr = crate::EnumBitfieldStruct<u8, Luclr_SPEC>;
    impl Luclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);
        #[doc = "Clear the NMISR.LUST flag"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nmisr_SPEC;
impl crate::sealed::RegSpec for Nmisr_SPEC {
    type DataType = u16;
}
#[doc = "Non-Maskable Interrupt Status Register"]
pub type Nmisr = crate::RegValueT<Nmisr_SPEC>;

impl Nmisr {
    #[doc = "IWDT Underflow/Refresh Error Interrupt Status Flag"]
    #[inline(always)]
    pub fn iwdtst(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, nmisr::Iwdtst, Nmisr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,nmisr::Iwdtst, Nmisr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "WDT Underflow/Refresh Error Interrupt Status Flag"]
    #[inline(always)]
    pub fn wdtst(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, nmisr::Wdtst, Nmisr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,nmisr::Wdtst, Nmisr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Voltage Monitor 1 Interrupt Status Flag"]
    #[inline(always)]
    pub fn pvd1st(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, nmisr::Pvd1St, Nmisr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x1,1,0,nmisr::Pvd1St, Nmisr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Voltage Monitor 2 Interrupt Status Flag"]
    #[inline(always)]
    pub fn pvd2st(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, nmisr::Pvd2St, Nmisr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<3,0x1,1,0,nmisr::Pvd2St, Nmisr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Main Clock Oscillation Stop Detection Interrupt Status Flag"]
    #[inline(always)]
    pub fn ostst(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, nmisr::Ostst, Nmisr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<6,0x1,1,0,nmisr::Ostst, Nmisr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "NMI Pin Interrupt Status Flag"]
    #[inline(always)]
    pub fn nmist(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, nmisr::Nmist, Nmisr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<7,0x1,1,0,nmisr::Nmist, Nmisr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Bus Error Interrupt Status Flag"]
    #[inline(always)]
    pub fn busst(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, nmisr::Busst, Nmisr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<12,0x1,1,0,nmisr::Busst, Nmisr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn cmst(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, nmisr::Cmst, Nmisr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<13,0x1,1,0,nmisr::Cmst, Nmisr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn lust(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, nmisr::Lust, Nmisr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<15,0x1,1,0,nmisr::Lust, Nmisr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Nmisr {
    #[inline(always)]
    fn default() -> Nmisr {
        <crate::RegValueT<Nmisr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod nmisr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iwdtst_SPEC;
    pub type Iwdtst = crate::EnumBitfieldStruct<u8, Iwdtst_SPEC>;
    impl Iwdtst {
        #[doc = "Interrupt not requested"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt requested"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wdtst_SPEC;
    pub type Wdtst = crate::EnumBitfieldStruct<u8, Wdtst_SPEC>;
    impl Wdtst {
        #[doc = "Interrupt not requested"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt requested"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pvd1St_SPEC;
    pub type Pvd1St = crate::EnumBitfieldStruct<u8, Pvd1St_SPEC>;
    impl Pvd1St {
        #[doc = "Interrupt not requested"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt requested"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pvd2St_SPEC;
    pub type Pvd2St = crate::EnumBitfieldStruct<u8, Pvd2St_SPEC>;
    impl Pvd2St {
        #[doc = "Interrupt not requested"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt requested"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostst_SPEC;
    pub type Ostst = crate::EnumBitfieldStruct<u8, Ostst_SPEC>;
    impl Ostst {
        #[doc = "Interrupt not requested for main clock oscillation stop"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt requested for main clock oscillation stop"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nmist_SPEC;
    pub type Nmist = crate::EnumBitfieldStruct<u8, Nmist_SPEC>;
    impl Nmist {
        #[doc = "Interrupt not requested"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt requested"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Busst_SPEC;
    pub type Busst = crate::EnumBitfieldStruct<u8, Busst_SPEC>;
    impl Busst {
        #[doc = "Interrupt not requested"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt requested"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmst_SPEC;
    pub type Cmst = crate::EnumBitfieldStruct<u8, Cmst_SPEC>;
    impl Cmst {
        #[doc = "Interrupt not requested"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt requested"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lust_SPEC;
    pub type Lust = crate::EnumBitfieldStruct<u8, Lust_SPEC>;
    impl Lust {
        #[doc = "Interrupt not requested"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt requested"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wupen0_SPEC;
impl crate::sealed::RegSpec for Wupen0_SPEC {
    type DataType = u32;
}
#[doc = "Wake Up Interrupt Enable Register 0"]
pub type Wupen0 = crate::RegValueT<Wupen0_SPEC>;

impl Wupen0 {
    #[doc = ""]
    #[inline(always)]
    pub fn irqwupen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        wupen0::Irqwupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            wupen0::Irqwupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "IWDT Interrupt Deep Sleep/Software Standby Mode Returns Enable"]
    #[inline(always)]
    pub fn iwdtwupen(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        wupen0::Iwdtwupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            wupen0::Iwdtwupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "PVD1 Interrupt Deep Sleep/Software Standby Mode Returns Enable"]
    #[inline(always)]
    pub fn pvd1wupen(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        wupen0::Pvd1Wupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            wupen0::Pvd1Wupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "PVD2 Interrupt Deep Sleep/Software Standby Mode Returns Enable"]
    #[inline(always)]
    pub fn pvd2wupen(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        wupen0::Pvd2Wupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            wupen0::Pvd2Wupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "VBATT Monitor Interrupt Deep Sleep/Software Standby Mode Returns Enable"]
    #[inline(always)]
    pub fn vbattwupen(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        wupen0::Vbattwupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            wupen0::Vbattwupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "RTC Alarm Interrupt Deep Sleep/Software Standby Mode Returns Enable"]
    #[inline(always)]
    pub fn rtcalmwupen(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        wupen0::Rtcalmwupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            wupen0::Rtcalmwupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "RTC Period Interrupt Deep Sleep/Software Standby Mode Returns Enable"]
    #[inline(always)]
    pub fn rtcprdwupen(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        wupen0::Rtcprdwupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            wupen0::Rtcprdwupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "USBFS Interrupt Deep Sleep/Software Standby Mode Returns Enable"]
    #[inline(always)]
    pub fn usbfs0wupen(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        wupen0::Usbfs0Wupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            wupen0::Usbfs0Wupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "AGT1 Underflow Interrupt Deep Sleep/Software Standby Mode Returns Enable"]
    #[inline(always)]
    pub fn agt1udwupen(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        wupen0::Agt1Udwupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            wupen0::Agt1Udwupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "AGT1 Compare Match A Interrupt Deep Sleep/Software Standby Mode Returns Enable"]
    #[inline(always)]
    pub fn agt1cawupen(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        wupen0::Agt1Cawupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            wupen0::Agt1Cawupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "AGT1 Compare Match B Interrupt Deep Sleep/Software Standby Mode Returns Enable"]
    #[inline(always)]
    pub fn agt1cbwupen(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        wupen0::Agt1Cbwupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            wupen0::Agt1Cbwupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "IIC0 Address Match Interrupt Deep Sleep/Software Standby Mode Returns Enable"]
    #[inline(always)]
    pub fn iic0wupen(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        wupen0::Iic0Wupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            wupen0::Iic0Wupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Wupen0 {
    #[inline(always)]
    fn default() -> Wupen0 {
        <crate::RegValueT<Wupen0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod wupen0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen_SPEC;
    pub type Irqwupen = crate::EnumBitfieldStruct<u8, Irqwupen_SPEC>;
    impl Irqwupen {
        #[doc = "Deep Sleep/Software Standby Mode returns by IRQn interrupt is disabled ."]
        pub const _0: Self = Self::new(0);
        #[doc = "Deep Sleep/Software Standby Mode returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iwdtwupen_SPEC;
    pub type Iwdtwupen = crate::EnumBitfieldStruct<u8, Iwdtwupen_SPEC>;
    impl Iwdtwupen {
        #[doc = "Deep Sleep/Software Standby Mode returns by IWDT interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Deep Sleep/Software Standby Mode returns by IWDT interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pvd1Wupen_SPEC;
    pub type Pvd1Wupen = crate::EnumBitfieldStruct<u8, Pvd1Wupen_SPEC>;
    impl Pvd1Wupen {
        #[doc = "Deep Sleep/Software Standby Mode returns by PVD1 interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Deep Sleep/Software Standby Mode returns by PVD1 interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pvd2Wupen_SPEC;
    pub type Pvd2Wupen = crate::EnumBitfieldStruct<u8, Pvd2Wupen_SPEC>;
    impl Pvd2Wupen {
        #[doc = "Deep Sleep/Software Standby Mode returns by PVD2 interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Deep Sleep/Software Standby Mode returns by PVD2 interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbattwupen_SPEC;
    pub type Vbattwupen = crate::EnumBitfieldStruct<u8, Vbattwupen_SPEC>;
    impl Vbattwupen {
        #[doc = "Deep Sleep/Software Standby Mode returns by VBATT monitor interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Deep Sleep/Software Standby Mode returns by VBATT monitor interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtcalmwupen_SPEC;
    pub type Rtcalmwupen = crate::EnumBitfieldStruct<u8, Rtcalmwupen_SPEC>;
    impl Rtcalmwupen {
        #[doc = "Deep Sleep/Software Standby Mode returns by RTC alarm interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Deep Sleep/Software Standby Mode returns by RTC alarm interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtcprdwupen_SPEC;
    pub type Rtcprdwupen = crate::EnumBitfieldStruct<u8, Rtcprdwupen_SPEC>;
    impl Rtcprdwupen {
        #[doc = "Deep Sleep/Software Standby Mode returns by RTC period interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Deep Sleep/Software Standby Mode returns by RTC period interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usbfs0Wupen_SPEC;
    pub type Usbfs0Wupen = crate::EnumBitfieldStruct<u8, Usbfs0Wupen_SPEC>;
    impl Usbfs0Wupen {
        #[doc = "Deep Sleep/Software Standby Mode returns by USBFS interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Deep Sleep/Software Standby Mode returns by USBFS interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Agt1Udwupen_SPEC;
    pub type Agt1Udwupen = crate::EnumBitfieldStruct<u8, Agt1Udwupen_SPEC>;
    impl Agt1Udwupen {
        #[doc = "Deep Sleep/Software Standby Mode returns by AGT1 underflow interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Deep Sleep/Software Standby Mode returns by AGT1 underflow interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Agt1Cawupen_SPEC;
    pub type Agt1Cawupen = crate::EnumBitfieldStruct<u8, Agt1Cawupen_SPEC>;
    impl Agt1Cawupen {
        #[doc = "Deep Sleep/Software Standby Mode returns by AGT1 compare match A interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Deep Sleep/Software Standby Mode returns by AGT1 compare match A interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Agt1Cbwupen_SPEC;
    pub type Agt1Cbwupen = crate::EnumBitfieldStruct<u8, Agt1Cbwupen_SPEC>;
    impl Agt1Cbwupen {
        #[doc = "Deep Sleep/Software Standby Mode returns by AGT1 compare match B interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Deep Sleep/Software Standby Mode returns by AGT1 compare match B interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iic0Wupen_SPEC;
    pub type Iic0Wupen = crate::EnumBitfieldStruct<u8, Iic0Wupen_SPEC>;
    impl Iic0Wupen {
        #[doc = "Deep Sleep/Software Standby Mode returns by IIC0 address match A interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Deep Sleep/Software Standby Mode returns by IIC0 address match A interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wupen1_SPEC;
impl crate::sealed::RegSpec for Wupen1_SPEC {
    type DataType = u32;
}
#[doc = "Wake Up interrupt enable register 1"]
pub type Wupen1 = crate::RegValueT<Wupen1_SPEC>;

impl Wupen1 {
    #[doc = "Comparator-HS0 Interrupt Deep Sleep/Software Standby Mode returns Enable bit"]
    #[inline(always)]
    pub fn comphs0wupen(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        wupen1::Comphs0Wupen,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            wupen1::Comphs0Wupen,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ULPT0 Underflow Interrupt Deep Sleep/Software Standby Mode returns Enable bit"]
    #[inline(always)]
    pub fn ulp0uwupen(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        wupen1::Ulp0Uwupen,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            wupen1::Ulp0Uwupen,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ULPT0 Compare Match A Interrupt Deep Sleep/Software Standby Mode returns Enable bit"]
    #[inline(always)]
    pub fn ulp0awupen(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        wupen1::Ulp0Awupen,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            wupen1::Ulp0Awupen,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ULPT0 Compare Match B Interrupt Deep Sleep/Software Standby Mode returns Enable bit"]
    #[inline(always)]
    pub fn ulp0bwupen(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        wupen1::Ulp0Bwupen,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            wupen1::Ulp0Bwupen,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ULPT1 Underflow Interrupt Deep Sleep/Software Standby Mode returns Enable bit"]
    #[inline(always)]
    pub fn ulp1uwupen(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        wupen1::Ulp1Uwupen,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            wupen1::Ulp1Uwupen,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ULPT1 Compare Match A Interrupt Deep Sleep/Software Standby Mode returns Enable bit"]
    #[inline(always)]
    pub fn ulp1awupen(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        wupen1::Ulp1Awupen,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            wupen1::Ulp1Awupen,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ULPT1 Compare Match B Interrupt Deep Sleep/Software Standby Mode returns Enable bit"]
    #[inline(always)]
    pub fn ulp1bwupen(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        wupen1::Ulp1Bwupen,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            wupen1::Ulp1Bwupen,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Wupen1 {
    #[inline(always)]
    fn default() -> Wupen1 {
        <crate::RegValueT<Wupen1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod wupen1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Comphs0Wupen_SPEC;
    pub type Comphs0Wupen = crate::EnumBitfieldStruct<u8, Comphs0Wupen_SPEC>;
    impl Comphs0Wupen {
        #[doc = "Deep Sleep/Software Standby returns by Comparator-HS0 interrupt is disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "Deep Sleep/Software Standby returns by ULPT0 Underflow interrupt is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ulp0Uwupen_SPEC;
    pub type Ulp0Uwupen = crate::EnumBitfieldStruct<u8, Ulp0Uwupen_SPEC>;
    impl Ulp0Uwupen {
        #[doc = "Deep Sleep/Software Standby returns by ULPT0 interrupt is disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "Deep Sleep/Software Standby returns by ULPT0 Underflow interrupt is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ulp0Awupen_SPEC;
    pub type Ulp0Awupen = crate::EnumBitfieldStruct<u8, Ulp0Awupen_SPEC>;
    impl Ulp0Awupen {
        #[doc = "Deep Sleep/Software Standby returns by ULPT0 Compare match A interrupt is disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "Deep Sleep/Software Standby returns by ULPT0 Compare match A interrupt is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ulp0Bwupen_SPEC;
    pub type Ulp0Bwupen = crate::EnumBitfieldStruct<u8, Ulp0Bwupen_SPEC>;
    impl Ulp0Bwupen {
        #[doc = "Deep Sleep/Software Standby returns by ULPT0 Compare match B interrupt is disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "Deep Sleep/Software Standby returns by ULPT0 Compare match B interrupt is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ulp1Uwupen_SPEC;
    pub type Ulp1Uwupen = crate::EnumBitfieldStruct<u8, Ulp1Uwupen_SPEC>;
    impl Ulp1Uwupen {
        #[doc = "Deep Sleep/Software Standby returns by ULPT1 Underflow interrupt is disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "Deep Sleep/Software Standby returns by ULPT1 Underflow interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ulp1Awupen_SPEC;
    pub type Ulp1Awupen = crate::EnumBitfieldStruct<u8, Ulp1Awupen_SPEC>;
    impl Ulp1Awupen {
        #[doc = "Deep Sleep/Software Standby returns by ULPT1 Compare match A interrupt is disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "Deep Sleep/Software Standby returns by ULPT1 Compare match A interrupt is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ulp1Bwupen_SPEC;
    pub type Ulp1Bwupen = crate::EnumBitfieldStruct<u8, Ulp1Bwupen_SPEC>;
    impl Ulp1Bwupen {
        #[doc = "Deep Sleep/Software Standby returns by ULPT1 Compare match B interrupt is disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "Deep Sleep/Software Standby returns by ULPT1 Compare match B interrupt is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ielsr_SPEC;
impl crate::sealed::RegSpec for Ielsr_SPEC {
    type DataType = u32;
}
#[doc = "ICU Event Link Setting Register %s"]
pub type Ielsr = crate::RegValueT<Ielsr_SPEC>;

impl NoBitfieldReg<Ielsr_SPEC> for Ielsr {}
impl ::core::default::Default for Ielsr {
    #[inline(always)]
    fn default() -> Ielsr {
        <crate::RegValueT<Ielsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
