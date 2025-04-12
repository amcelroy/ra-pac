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
#[doc = r"Temperature Sensor"]
unsafe impl ::core::marker::Send for super::Tsn {}
unsafe impl ::core::marker::Sync for super::Tsn {}
impl super::Tsn {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "Temperature Sensor Control Register"]
    #[inline(always)]
    pub const fn tscr(&self) -> &'static crate::common::Reg<self::Tscr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tscr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "TSTRM0"]
    #[inline(always)]
    pub const fn tstrm0(
        &self,
    ) -> &'static crate::common::Reg<self::Tstrm0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tstrm0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1usize),
            )
        }
    }

    #[doc = "TSTRM1"]
    #[inline(always)]
    pub const fn tstrm1(
        &self,
    ) -> &'static crate::common::Reg<self::Tstrm1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tstrm1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "TSTST"]
    #[inline(always)]
    pub const fn tstst(&self) -> &'static crate::common::Reg<self::Tstst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tstst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tscr_SPEC;
impl crate::sealed::RegSpec for Tscr_SPEC {
    type DataType = u8;
}
#[doc = "Temperature Sensor Control Register"]
pub type Tscr = crate::RegValueT<Tscr_SPEC>;

impl Tscr {
    #[doc = "Temperature Sensor Enable"]
    #[inline(always)]
    pub fn tsoe(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, tscr::Tsoe, Tscr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,tscr::Tsoe, Tscr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, Tscr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x3,1,0,u8, Tscr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Temperature Sensor Output Enable"]
    #[inline(always)]
    pub fn tsen(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, tscr::Tsen, Tscr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,tscr::Tsen, Tscr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tscr {
    #[inline(always)]
    fn default() -> Tscr {
        <crate::RegValueT<Tscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsoe_SPEC;
    pub type Tsoe = crate::EnumBitfieldStruct<u8, Tsoe_SPEC>;
    impl Tsoe {
        #[doc = "Disables output from the temperature sensor to the 12-bit A/D converter."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables output from the temperature sensor to the 12-bit A/D converter."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsen_SPEC;
    pub type Tsen = crate::EnumBitfieldStruct<u8, Tsen_SPEC>;
    impl Tsen {
        #[doc = "Stops the temperature sensor."]
        pub const _0: Self = Self::new(0);
        #[doc = "Starts the temperature sensor."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tstrm0_SPEC;
impl crate::sealed::RegSpec for Tstrm0_SPEC {
    type DataType = u8;
}
#[doc = "TSTRM0"]
pub type Tstrm0 = crate::RegValueT<Tstrm0_SPEC>;

impl NoBitfieldReg<Tstrm0_SPEC> for Tstrm0 {}
impl ::core::default::Default for Tstrm0 {
    #[inline(always)]
    fn default() -> Tstrm0 {
        <crate::RegValueT<Tstrm0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tstrm1_SPEC;
impl crate::sealed::RegSpec for Tstrm1_SPEC {
    type DataType = u8;
}
#[doc = "TSTRM1"]
pub type Tstrm1 = crate::RegValueT<Tstrm1_SPEC>;

impl NoBitfieldReg<Tstrm1_SPEC> for Tstrm1 {}
impl ::core::default::Default for Tstrm1 {
    #[inline(always)]
    fn default() -> Tstrm1 {
        <crate::RegValueT<Tstrm1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tstst_SPEC;
impl crate::sealed::RegSpec for Tstst_SPEC {
    type DataType = u8;
}
#[doc = "TSTST"]
pub type Tstst = crate::RegValueT<Tstst_SPEC>;

impl NoBitfieldReg<Tstst_SPEC> for Tstst {}
impl ::core::default::Default for Tstst {
    #[inline(always)]
    fn default() -> Tstst {
        <crate::RegValueT<Tstst_SPEC> as RegisterValue<_>>::new(0)
    }
}
