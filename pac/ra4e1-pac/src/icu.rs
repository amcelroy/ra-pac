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
// Generated from SVD 1.20.00, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:15:25 +0000

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
    #[doc = "IRQ Control Register"]
    #[inline(always)]
    pub const fn irqcr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Irqcr_SPEC, crate::common::RW>,
        10,
        0x1,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x0usize))
        }
    }

    #[doc = "IRQ Control Register 13"]
    #[inline(always)]
    pub const fn irqcr13(
        &self,
    ) -> &'static crate::common::Reg<self::Irqcr13_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irqcr13_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(13usize),
            )
        }
    }

    #[doc = "NMI Pin Interrupt Control Register"]
    #[inline(always)]
    pub const fn nmicr(&self) -> &'static crate::common::Reg<self::Nmicr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Nmicr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[doc = "Non-Maskable Interrupt Enable Register"]
    #[inline(always)]
    pub const fn nmier(&self) -> &'static crate::common::Reg<self::Nmier_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Nmier_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(288usize),
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
                self._svd2pac_as_ptr().add(304usize),
            )
        }
    }

    #[doc = "Non-Maskable Interrupt Status Register"]
    #[inline(always)]
    pub const fn nmisr(&self) -> &'static crate::common::Reg<self::Nmisr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Nmisr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(320usize),
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

    #[doc = "SYS Event Link Setting Register"]
    #[inline(always)]
    pub const fn selsr0(
        &self,
    ) -> &'static crate::common::Reg<self::Selsr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Selsr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(512usize),
            )
        }
    }

    #[doc = "DMAC Event Link Setting Register %s"]
    #[inline(always)]
    pub const fn delsr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Delsr_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x280usize))
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
pub struct Irqcr_SPEC;
impl crate::sealed::RegSpec for Irqcr_SPEC {
    type DataType = u8;
}
#[doc = "IRQ Control Register"]
pub type Irqcr = crate::RegValueT<Irqcr_SPEC>;

impl Irqcr {
    #[doc = "IRQi Detection Sense Select"]
    #[inline(always)]
    pub fn irqmd(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, irqcr::Irqmd, Irqcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,irqcr::Irqmd, Irqcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQi Digital Filter Sampling Clock Select"]
    #[inline(always)]
    pub fn fclksel(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, irqcr::Fclksel, Irqcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,irqcr::Fclksel, Irqcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQi Digital Filter Enable"]
    #[inline(always)]
    pub fn flten(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, irqcr::Flten, Irqcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,irqcr::Flten, Irqcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Irqcr {
    #[inline(always)]
    fn default() -> Irqcr {
        <crate::RegValueT<Irqcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod irqcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqmd_SPEC;
    pub type Irqmd = crate::EnumBitfieldStruct<u8, Irqmd_SPEC>;
    impl Irqmd {
        #[doc = "Falling edge"]
        pub const _00: Self = Self::new(0);
        #[doc = "Rising edge"]
        pub const _01: Self = Self::new(1);
        #[doc = "Rising and falling edges"]
        pub const _10: Self = Self::new(2);
        #[doc = "Low level"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fclksel_SPEC;
    pub type Fclksel = crate::EnumBitfieldStruct<u8, Fclksel_SPEC>;
    impl Fclksel {
        #[doc = "PCLKB"]
        pub const _00: Self = Self::new(0);
        #[doc = "PCLKB/8"]
        pub const _01: Self = Self::new(1);
        #[doc = "PCLKB/32"]
        pub const _10: Self = Self::new(2);
        #[doc = "PCLKB/64"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flten_SPEC;
    pub type Flten = crate::EnumBitfieldStruct<u8, Flten_SPEC>;
    impl Flten {
        #[doc = "Digital filter is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Digital filter is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irqcr13_SPEC;
impl crate::sealed::RegSpec for Irqcr13_SPEC {
    type DataType = u8;
}
#[doc = "IRQ Control Register 13"]
pub type Irqcr13 = crate::RegValueT<Irqcr13_SPEC>;

impl Irqcr13 {
    #[doc = "IRQi Detection Sense Select"]
    #[inline(always)]
    pub fn irqmd(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, irqcr13::Irqmd, Irqcr13_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,irqcr13::Irqmd, Irqcr13_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQi Digital Filter Sampling Clock Select"]
    #[inline(always)]
    pub fn fclksel(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, irqcr13::Fclksel, Irqcr13_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            irqcr13::Fclksel,
            Irqcr13_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "IRQi Digital Filter Enable"]
    #[inline(always)]
    pub fn flten(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, irqcr13::Flten, Irqcr13_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,irqcr13::Flten, Irqcr13_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Irqcr13 {
    #[inline(always)]
    fn default() -> Irqcr13 {
        <crate::RegValueT<Irqcr13_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod irqcr13 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqmd_SPEC;
    pub type Irqmd = crate::EnumBitfieldStruct<u8, Irqmd_SPEC>;
    impl Irqmd {
        #[doc = "Falling edge"]
        pub const _00: Self = Self::new(0);
        #[doc = "Rising edge"]
        pub const _01: Self = Self::new(1);
        #[doc = "Rising and falling edges"]
        pub const _10: Self = Self::new(2);
        #[doc = "Low level"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fclksel_SPEC;
    pub type Fclksel = crate::EnumBitfieldStruct<u8, Fclksel_SPEC>;
    impl Fclksel {
        #[doc = "PCLKB"]
        pub const _00: Self = Self::new(0);
        #[doc = "PCLKB/8"]
        pub const _01: Self = Self::new(1);
        #[doc = "PCLKB/32"]
        pub const _10: Self = Self::new(2);
        #[doc = "PCLKB/64"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flten_SPEC;
    pub type Flten = crate::EnumBitfieldStruct<u8, Flten_SPEC>;
    impl Flten {
        #[doc = "Digital filter is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Digital filter is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nmicr_SPEC;
impl crate::sealed::RegSpec for Nmicr_SPEC {
    type DataType = u8;
}
#[doc = "NMI Pin Interrupt Control Register"]
pub type Nmicr = crate::RegValueT<Nmicr_SPEC>;

impl Nmicr {
    #[doc = "NMI Detection Set"]
    #[inline(always)]
    pub fn nmimd(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, nmicr::Nmimd, Nmicr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,nmicr::Nmimd, Nmicr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "NMI Digital Filter Sampling Clock Select"]
    #[inline(always)]
    pub fn nfclksel(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, nmicr::Nfclksel, Nmicr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,nmicr::Nfclksel, Nmicr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "NMI Digital Filter Enable"]
    #[inline(always)]
    pub fn nflten(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, nmicr::Nflten, Nmicr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,nmicr::Nflten, Nmicr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Nmicr {
    #[inline(always)]
    fn default() -> Nmicr {
        <crate::RegValueT<Nmicr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod nmicr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nmimd_SPEC;
    pub type Nmimd = crate::EnumBitfieldStruct<u8, Nmimd_SPEC>;
    impl Nmimd {
        #[doc = "Falling edge"]
        pub const _0: Self = Self::new(0);
        #[doc = "Rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfclksel_SPEC;
    pub type Nfclksel = crate::EnumBitfieldStruct<u8, Nfclksel_SPEC>;
    impl Nfclksel {
        #[doc = "PCLKB"]
        pub const _00: Self = Self::new(0);
        #[doc = "PCLKB/8"]
        pub const _01: Self = Self::new(1);
        #[doc = "PCLKB/32"]
        pub const _10: Self = Self::new(2);
        #[doc = "PCLKB/64"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nflten_SPEC;
    pub type Nflten = crate::EnumBitfieldStruct<u8, Nflten_SPEC>;
    impl Nflten {
        #[doc = "Disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enabled."]
        pub const _1: Self = Self::new(1);
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
    pub fn lvd1en(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, nmier::Lvd1En, Nmier_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,nmier::Lvd1En, Nmier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Voltage monitor 2 Interrupt Enable"]
    #[inline(always)]
    pub fn lvd2en(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, nmier::Lvd2En, Nmier_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,nmier::Lvd2En, Nmier_SPEC,crate::common::RW>::from_register(self,0)
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
    #[doc = "SRAM Parity Error Interrupt Enable"]
    #[inline(always)]
    pub fn rpeen(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, nmier::Rpeen, Nmier_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,nmier::Rpeen, Nmier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bus Master MPU Error Interrupt Enable"]
    #[inline(always)]
    pub fn busmen(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, nmier::Busmen, Nmier_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,nmier::Busmen, Nmier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn tzfen(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, nmier::Tzfen, Nmier_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,nmier::Tzfen, Nmier_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Enabled."]
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
    pub struct Lvd1En_SPEC;
    pub type Lvd1En = crate::EnumBitfieldStruct<u8, Lvd1En_SPEC>;
    impl Lvd1En {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd2En_SPEC;
    pub type Lvd2En = crate::EnumBitfieldStruct<u8, Lvd2En_SPEC>;
    impl Lvd2En {
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
    pub struct Rpeen_SPEC;
    pub type Rpeen = crate::EnumBitfieldStruct<u8, Rpeen_SPEC>;
    impl Rpeen {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Busmen_SPEC;
    pub type Busmen = crate::EnumBitfieldStruct<u8, Busmen_SPEC>;
    impl Busmen {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tzfen_SPEC;
    pub type Tzfen = crate::EnumBitfieldStruct<u8, Tzfen_SPEC>;
    impl Tzfen {
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
    pub fn lvd1clr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, nmiclr::Lvd1Clr, Nmiclr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,nmiclr::Lvd1Clr, Nmiclr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Voltage Monitor 2 Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn lvd2clr(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, nmiclr::Lvd2Clr, Nmiclr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,nmiclr::Lvd2Clr, Nmiclr_SPEC,crate::common::RW>::from_register(self,0)
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
    #[doc = "SRAM Parity Error Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn rpeclr(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, nmiclr::Rpeclr, Nmiclr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,nmiclr::Rpeclr, Nmiclr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bus Master MPU Error Interrupt Status Flag Clear"]
    #[inline(always)]
    pub fn busmclr(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, nmiclr::Busmclr, Nmiclr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,nmiclr::Busmclr, Nmiclr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn tzfclr(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, nmiclr::Tzfclr, Nmiclr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,nmiclr::Tzfclr, Nmiclr_SPEC,crate::common::RW>::from_register(self,0)
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
    pub struct Lvd1Clr_SPEC;
    pub type Lvd1Clr = crate::EnumBitfieldStruct<u8, Lvd1Clr_SPEC>;
    impl Lvd1Clr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);
        #[doc = "Clear the NMISR.LVD1ST flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd2Clr_SPEC;
    pub type Lvd2Clr = crate::EnumBitfieldStruct<u8, Lvd2Clr_SPEC>;
    impl Lvd2Clr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);
        #[doc = "Clear the NMISR.LVD2ST flag."]
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
    pub struct Rpeclr_SPEC;
    pub type Rpeclr = crate::EnumBitfieldStruct<u8, Rpeclr_SPEC>;
    impl Rpeclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);
        #[doc = "Clear the NMISR.RPEST flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Busmclr_SPEC;
    pub type Busmclr = crate::EnumBitfieldStruct<u8, Busmclr_SPEC>;
    impl Busmclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);
        #[doc = "Clear the NMISR.BUSMST flag"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tzfclr_SPEC;
    pub type Tzfclr = crate::EnumBitfieldStruct<u8, Tzfclr_SPEC>;
    impl Tzfclr {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);
        #[doc = "Clear the NMISR.TZFCLR flag"]
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
    pub fn lvd1st(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, nmisr::Lvd1St, Nmisr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x1,1,0,nmisr::Lvd1St, Nmisr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Voltage Monitor 2 Interrupt Status Flag"]
    #[inline(always)]
    pub fn lvd2st(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, nmisr::Lvd2St, Nmisr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<3,0x1,1,0,nmisr::Lvd2St, Nmisr_SPEC,crate::common::R>::from_register(self,0)
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
    #[doc = "SRAM Parity Error Interrupt Status Flag"]
    #[inline(always)]
    pub fn rpest(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, nmisr::Rpest, Nmisr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x1,1,0,nmisr::Rpest, Nmisr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Bus Master MPU Error Interrupt Status Flag"]
    #[inline(always)]
    pub fn busmst(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, nmisr::Busmst, Nmisr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<11,0x1,1,0,nmisr::Busmst, Nmisr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn tzfst(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, nmisr::Tzfst, Nmisr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<13,0x1,1,0,nmisr::Tzfst, Nmisr_SPEC,crate::common::R>::from_register(self,0)
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
    pub struct Lvd1St_SPEC;
    pub type Lvd1St = crate::EnumBitfieldStruct<u8, Lvd1St_SPEC>;
    impl Lvd1St {
        #[doc = "Interrupt not requested"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt requested"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd2St_SPEC;
    pub type Lvd2St = crate::EnumBitfieldStruct<u8, Lvd2St_SPEC>;
    impl Lvd2St {
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
    pub struct Rpest_SPEC;
    pub type Rpest = crate::EnumBitfieldStruct<u8, Rpest_SPEC>;
    impl Rpest {
        #[doc = "Interrupt not requested"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt requested"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Busmst_SPEC;
    pub type Busmst = crate::EnumBitfieldStruct<u8, Busmst_SPEC>;
    impl Busmst {
        #[doc = "Interrupt not requested"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt requested"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tzfst_SPEC;
    pub type Tzfst = crate::EnumBitfieldStruct<u8, Tzfst_SPEC>;
    impl Tzfst {
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
    #[doc = "IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
    #[inline(always)]
    pub fn irqwupen0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, wupen0::Irqwupen0, Wupen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            wupen0::Irqwupen0,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
    #[inline(always)]
    pub fn irqwupen1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, wupen0::Irqwupen1, Wupen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            wupen0::Irqwupen1,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
    #[inline(always)]
    pub fn irqwupen2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, wupen0::Irqwupen2, Wupen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            wupen0::Irqwupen2,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
    #[inline(always)]
    pub fn irqwupen3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, wupen0::Irqwupen3, Wupen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            wupen0::Irqwupen3,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
    #[inline(always)]
    pub fn irqwupen4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, wupen0::Irqwupen4, Wupen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            wupen0::Irqwupen4,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
    #[inline(always)]
    pub fn irqwupen5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, wupen0::Irqwupen5, Wupen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            wupen0::Irqwupen5,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
    #[inline(always)]
    pub fn irqwupen6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, wupen0::Irqwupen6, Wupen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            wupen0::Irqwupen6,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
    #[inline(always)]
    pub fn irqwupen7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, wupen0::Irqwupen7, Wupen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            wupen0::Irqwupen7,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
    #[inline(always)]
    pub fn irqwupen8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, wupen0::Irqwupen8, Wupen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            wupen0::Irqwupen8,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 9)"]
    #[inline(always)]
    pub fn irqwupen9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, wupen0::Irqwupen9, Wupen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            wupen0::Irqwupen9,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "IRQ13 Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    pub fn irqwupen13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        wupen0::Irqwupen13,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            wupen0::Irqwupen13,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "IWDT Interrupt Software Standby/Snooze Mode Returns Enable bit"]
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
    #[doc = "LVD1 Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    pub fn lvd1wupen(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        wupen0::Lvd1Wupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            wupen0::Lvd1Wupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "LVD2 Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    pub fn lvd2wupen(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        wupen0::Lvd2Wupen,
        Wupen0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            wupen0::Lvd2Wupen,
            Wupen0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "RTC Alarm Interrupt Software Standby/Snooze Mode Returns Enable bit"]
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
    #[doc = "RTC Period Interrupt Software Standby/Snooze Mode Returns Enable bit"]
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
    #[doc = "USBFS0 Interrupt Software Standby/Snooze Mode Returns Enable bit"]
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
    #[doc = "AGT1 Underflow Interrupt Software Standby/Snooze Mode Returns Enable bit"]
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
    #[doc = "AGT1 Compare Match A Interrupt Software Standby/Snooze Mode Returns Enable bit"]
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
    #[doc = "AGT1 Compare Match B Interrupt Software Standby/Snooze Mode Returns Enable bit"]
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
    #[doc = "IIC0 Address Match Interrupt Software Standby/Snooze Mode Returns Enable bit"]
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
    pub struct Irqwupen0_SPEC;
    pub type Irqwupen0 = crate::EnumBitfieldStruct<u8, Irqwupen0_SPEC>;
    impl Irqwupen0 {
        #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen1_SPEC;
    pub type Irqwupen1 = crate::EnumBitfieldStruct<u8, Irqwupen1_SPEC>;
    impl Irqwupen1 {
        #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen2_SPEC;
    pub type Irqwupen2 = crate::EnumBitfieldStruct<u8, Irqwupen2_SPEC>;
    impl Irqwupen2 {
        #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen3_SPEC;
    pub type Irqwupen3 = crate::EnumBitfieldStruct<u8, Irqwupen3_SPEC>;
    impl Irqwupen3 {
        #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen4_SPEC;
    pub type Irqwupen4 = crate::EnumBitfieldStruct<u8, Irqwupen4_SPEC>;
    impl Irqwupen4 {
        #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen5_SPEC;
    pub type Irqwupen5 = crate::EnumBitfieldStruct<u8, Irqwupen5_SPEC>;
    impl Irqwupen5 {
        #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen6_SPEC;
    pub type Irqwupen6 = crate::EnumBitfieldStruct<u8, Irqwupen6_SPEC>;
    impl Irqwupen6 {
        #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen7_SPEC;
    pub type Irqwupen7 = crate::EnumBitfieldStruct<u8, Irqwupen7_SPEC>;
    impl Irqwupen7 {
        #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen8_SPEC;
    pub type Irqwupen8 = crate::EnumBitfieldStruct<u8, Irqwupen8_SPEC>;
    impl Irqwupen8 {
        #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen9_SPEC;
    pub type Irqwupen9 = crate::EnumBitfieldStruct<u8, Irqwupen9_SPEC>;
    impl Irqwupen9 {
        #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqwupen13_SPEC;
    pub type Irqwupen13 = crate::EnumBitfieldStruct<u8, Irqwupen13_SPEC>;
    impl Irqwupen13 {
        #[doc = "Software Standby/Snooze Mode returns by IRQ13 interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Software Standby/Snooze Mode returns by IRQ13 interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iwdtwupen_SPEC;
    pub type Iwdtwupen = crate::EnumBitfieldStruct<u8, Iwdtwupen_SPEC>;
    impl Iwdtwupen {
        #[doc = "Software Standby/Snooze Mode returns by IWDT interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Software Standby/Snooze Mode returns by IWDT interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd1Wupen_SPEC;
    pub type Lvd1Wupen = crate::EnumBitfieldStruct<u8, Lvd1Wupen_SPEC>;
    impl Lvd1Wupen {
        #[doc = "Software Standby/Snooze Mode returns by LVD1 interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Software Standby/Snooze Mode returns by LVD1 interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd2Wupen_SPEC;
    pub type Lvd2Wupen = crate::EnumBitfieldStruct<u8, Lvd2Wupen_SPEC>;
    impl Lvd2Wupen {
        #[doc = "Software Standby/Snooze Mode returns by LVD2 interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Software Standby/Snooze Mode returns by LVD2 interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtcalmwupen_SPEC;
    pub type Rtcalmwupen = crate::EnumBitfieldStruct<u8, Rtcalmwupen_SPEC>;
    impl Rtcalmwupen {
        #[doc = "Software Standby/Snooze Mode returns by RTC alarm interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Software Standby/Snooze Mode returns by RTC alarm interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtcprdwupen_SPEC;
    pub type Rtcprdwupen = crate::EnumBitfieldStruct<u8, Rtcprdwupen_SPEC>;
    impl Rtcprdwupen {
        #[doc = "Software Standby/Snooze Mode returns by RTC period interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Software Standby/Snooze Mode returns by RTC period interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usbfs0Wupen_SPEC;
    pub type Usbfs0Wupen = crate::EnumBitfieldStruct<u8, Usbfs0Wupen_SPEC>;
    impl Usbfs0Wupen {
        #[doc = "Software Standby/Snooze Mode returns by USBFS0 interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Software Standby/Snooze Mode returns by USBFS0 interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Agt1Udwupen_SPEC;
    pub type Agt1Udwupen = crate::EnumBitfieldStruct<u8, Agt1Udwupen_SPEC>;
    impl Agt1Udwupen {
        #[doc = "Software Standby/Snooze Mode returns by AGT1 underflow interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Software Standby/Snooze Mode returns by AGT1 underflow interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Agt1Cawupen_SPEC;
    pub type Agt1Cawupen = crate::EnumBitfieldStruct<u8, Agt1Cawupen_SPEC>;
    impl Agt1Cawupen {
        #[doc = "Software Standby/Snooze Mode returns by AGT1 compare match A interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Software Standby/Snooze Mode returns by AGT1 compare match A interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Agt1Cbwupen_SPEC;
    pub type Agt1Cbwupen = crate::EnumBitfieldStruct<u8, Agt1Cbwupen_SPEC>;
    impl Agt1Cbwupen {
        #[doc = "Software Standby/Snooze Mode returns by AGT1 compare match B interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Software Standby/Snooze Mode returns by AGT1 compare match B interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iic0Wupen_SPEC;
    pub type Iic0Wupen = crate::EnumBitfieldStruct<u8, Iic0Wupen_SPEC>;
    impl Iic0Wupen {
        #[doc = "Software Standby/Snooze Mode returns by IIC0 address match interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Software Standby/Snooze Mode returns by IIC0 address match interrupt is enabled"]
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
    #[doc = "AGT3 Underflow Interrupt Software Standby Return Enable bit"]
    #[inline(always)]
    pub fn agt3udwupen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        wupen1::Agt3Udwupen,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            wupen1::Agt3Udwupen,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "AGT3 Compare Match A Interrupt Software Standby Return Enable bit"]
    #[inline(always)]
    pub fn agt3cawupen(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        wupen1::Agt3Cawupen,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            wupen1::Agt3Cawupen,
            Wupen1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "AGT3 Compare Match B Interrupt Software Standby Return Enable bit"]
    #[inline(always)]
    pub fn agt3cbwupen(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        wupen1::Agt3Cbwupen,
        Wupen1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            wupen1::Agt3Cbwupen,
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
    pub struct Agt3Udwupen_SPEC;
    pub type Agt3Udwupen = crate::EnumBitfieldStruct<u8, Agt3Udwupen_SPEC>;
    impl Agt3Udwupen {
        #[doc = "Software standby returns by AGT3 underflow interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Software standby returns by AGT3 underflow interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Agt3Cawupen_SPEC;
    pub type Agt3Cawupen = crate::EnumBitfieldStruct<u8, Agt3Cawupen_SPEC>;
    impl Agt3Cawupen {
        #[doc = "Software standby returns by AGT3 compare match A interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Software standby returns by AGT3 compare match A interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Agt3Cbwupen_SPEC;
    pub type Agt3Cbwupen = crate::EnumBitfieldStruct<u8, Agt3Cbwupen_SPEC>;
    impl Agt3Cbwupen {
        #[doc = "Software standby returns by AGT3 compare match B interrupt is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Software standby returns by AGT3 compare match B interrupt is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Selsr0_SPEC;
impl crate::sealed::RegSpec for Selsr0_SPEC {
    type DataType = u16;
}
#[doc = "SYS Event Link Setting Register"]
pub type Selsr0 = crate::RegValueT<Selsr0_SPEC>;

impl NoBitfieldReg<Selsr0_SPEC> for Selsr0 {}
impl ::core::default::Default for Selsr0 {
    #[inline(always)]
    fn default() -> Selsr0 {
        <crate::RegValueT<Selsr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Delsr_SPEC;
impl crate::sealed::RegSpec for Delsr_SPEC {
    type DataType = u32;
}
#[doc = "DMAC Event Link Setting Register %s"]
pub type Delsr = crate::RegValueT<Delsr_SPEC>;

impl Delsr {
    #[doc = "DMAC Event Link Select"]
    #[inline(always)]
    pub fn dels(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, delsr::Dels, Delsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,delsr::Dels, Delsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DMAC Activation Request Status Flag"]
    #[inline(always)]
    pub fn ir(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, delsr::Ir, Delsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1,1,0,delsr::Ir, Delsr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Delsr {
    #[inline(always)]
    fn default() -> Delsr {
        <crate::RegValueT<Delsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod delsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dels_SPEC;
    pub type Dels = crate::EnumBitfieldStruct<u8, Dels_SPEC>;
    impl Dels {
        #[doc = "Disable interrupts to the associated DMAC module."]
        pub const _0_X_00: Self = Self::new(0);
        #[doc = "Event signal number to be linked. For details, see ."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ir_SPEC;
    pub type Ir = crate::EnumBitfieldStruct<u8, Ir_SPEC>;
    impl Ir {
        #[doc = "No DMAC activation request occurred."]
        pub const _0: Self = Self::new(0);
        #[doc = "DMAC activation request occurred."]
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
