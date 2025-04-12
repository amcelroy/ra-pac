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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:17:28 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"SRAM Control"]
unsafe impl ::core::marker::Send for super::Sram {}
unsafe impl ::core::marker::Sync for super::Sram {}
impl super::Sram {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "SRAM Parity Error Operation After Detection Register"]
    #[inline(always)]
    pub const fn parioad(
        &self,
    ) -> &'static crate::common::Reg<self::Parioad_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Parioad_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "SRAM Protection Register"]
    #[inline(always)]
    pub const fn sramprcr(
        &self,
    ) -> &'static crate::common::Reg<self::Sramprcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sramprcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "RAM Wait State Control Register"]
    #[inline(always)]
    pub const fn sramwtsc(
        &self,
    ) -> &'static crate::common::Reg<self::Sramwtsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sramwtsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "ECCRAM Operating Mode Control Register"]
    #[inline(always)]
    pub const fn eccmode(
        &self,
    ) -> &'static crate::common::Reg<self::Eccmode_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eccmode_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }

    #[doc = "ECCRAM 2-Bit Error Status Register"]
    #[inline(always)]
    pub const fn ecc2sts(
        &self,
    ) -> &'static crate::common::Reg<self::Ecc2Sts_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ecc2Sts_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(193usize),
            )
        }
    }

    #[doc = "ECCRAM 1-Bit Error Information Update Enable Register"]
    #[inline(always)]
    pub const fn ecc1stsen(
        &self,
    ) -> &'static crate::common::Reg<self::Ecc1Stsen_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ecc1Stsen_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(194usize),
            )
        }
    }

    #[doc = "ECCRAM 1-Bit Error Status Register"]
    #[inline(always)]
    pub const fn ecc1sts(
        &self,
    ) -> &'static crate::common::Reg<self::Ecc1Sts_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ecc1Sts_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(195usize),
            )
        }
    }

    #[doc = "ECCRAM Protection Register"]
    #[inline(always)]
    pub const fn eccprcr(
        &self,
    ) -> &'static crate::common::Reg<self::Eccprcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eccprcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(196usize),
            )
        }
    }

    #[doc = "ECC Test Control Register"]
    #[inline(always)]
    pub const fn eccetst(
        &self,
    ) -> &'static crate::common::Reg<self::Eccetst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eccetst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(212usize),
            )
        }
    }

    #[doc = "RAM ECC Error Operation After Detection Register"]
    #[inline(always)]
    pub const fn eccoad(
        &self,
    ) -> &'static crate::common::Reg<self::Eccoad_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eccoad_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(216usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Parioad_SPEC;
impl crate::sealed::RegSpec for Parioad_SPEC {
    type DataType = u8;
}
#[doc = "SRAM Parity Error Operation After Detection Register"]
pub type Parioad = crate::RegValueT<Parioad_SPEC>;

impl Parioad {
    #[doc = "Operation after Detection"]
    #[inline(always)]
    pub fn oad(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, parioad::Oad, Parioad_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,parioad::Oad, Parioad_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Parioad {
    #[inline(always)]
    fn default() -> Parioad {
        <crate::RegValueT<Parioad_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod parioad {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oad_SPEC;
    pub type Oad = crate::EnumBitfieldStruct<u8, Oad_SPEC>;
    impl Oad {
        #[doc = "Non-maskable interrupt"]
        pub const _0: Self = Self::new(0);
        #[doc = "Reset"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramprcr_SPEC;
impl crate::sealed::RegSpec for Sramprcr_SPEC {
    type DataType = u8;
}
#[doc = "SRAM Protection Register"]
pub type Sramprcr = crate::RegValueT<Sramprcr_SPEC>;

impl Sramprcr {
    #[doc = "Write Key Code"]
    #[inline(always)]
    pub fn kw(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, sramprcr::Kw, Sramprcr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<1,0x7f,1,0,sramprcr::Kw, Sramprcr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Register Write Control"]
    #[inline(always)]
    pub fn sramprcr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sramprcr::Sramprcr,
        Sramprcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sramprcr::Sramprcr,
            Sramprcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sramprcr {
    #[inline(always)]
    fn default() -> Sramprcr {
        <crate::RegValueT<Sramprcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sramprcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Kw_SPEC;
    pub type Kw = crate::EnumBitfieldStruct<u8, Kw_SPEC>;
    impl Kw {
        #[doc = "Writing to the RAMPRCR bit is valid, when the KEY bits are written 1111000b."]
        pub const _1111000: Self = Self::new(120);
        #[doc = "Writing to the RAMPRCR bit is invalid."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sramprcr_SPEC;
    pub type Sramprcr = crate::EnumBitfieldStruct<u8, Sramprcr_SPEC>;
    impl Sramprcr {
        #[doc = "Disable writes to protected registers"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable writes to protected registers"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramwtsc_SPEC;
impl crate::sealed::RegSpec for Sramwtsc_SPEC {
    type DataType = u8;
}
#[doc = "RAM Wait State Control Register"]
pub type Sramwtsc = crate::RegValueT<Sramwtsc_SPEC>;

impl Sramwtsc {
    #[doc = "SRAM1 Wait Enable"]
    #[inline(always)]
    pub fn sram1wten(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        sramwtsc::Sram1Wten,
        Sramwtsc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            sramwtsc::Sram1Wten,
            Sramwtsc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "SRAM0 Wait Enable"]
    #[inline(always)]
    pub fn sram0wten(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        sramwtsc::Sram0Wten,
        Sramwtsc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            sramwtsc::Sram0Wten,
            Sramwtsc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ECCRAM Read wait enable"]
    #[inline(always)]
    pub fn eccramrdwten(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        sramwtsc::Eccramrdwten,
        Sramwtsc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sramwtsc::Eccramrdwten,
            Sramwtsc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sramwtsc {
    #[inline(always)]
    fn default() -> Sramwtsc {
        <crate::RegValueT<Sramwtsc_SPEC> as RegisterValue<_>>::new(14)
    }
}
pub mod sramwtsc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sram1Wten_SPEC;
    pub type Sram1Wten = crate::EnumBitfieldStruct<u8, Sram1Wten_SPEC>;
    impl Sram1Wten {
        #[doc = "Not add wait state in read access cycle to SRAM1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Add wait state in read access cycle to SRAM1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sram0Wten_SPEC;
    pub type Sram0Wten = crate::EnumBitfieldStruct<u8, Sram0Wten_SPEC>;
    impl Sram0Wten {
        #[doc = "Not add wait state in read access cycle to SRAM0"]
        pub const _0: Self = Self::new(0);
        #[doc = "Add wait state in read access cycle to SRAM0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eccramrdwten_SPEC;
    pub type Eccramrdwten = crate::EnumBitfieldStruct<u8, Eccramrdwten_SPEC>;
    impl Eccramrdwten {
        #[doc = "Not add wait state in read access cycle to SRAM0 (ECC area)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Add wait state in read access cycle to SRAM0 (ECC area)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eccmode_SPEC;
impl crate::sealed::RegSpec for Eccmode_SPEC {
    type DataType = u8;
}
#[doc = "ECCRAM Operating Mode Control Register"]
pub type Eccmode = crate::RegValueT<Eccmode_SPEC>;

impl Eccmode {
    #[doc = "ECC Operating Mode Select"]
    #[inline(always)]
    pub fn eccmod(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, eccmode::Eccmod, Eccmode_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,eccmode::Eccmod, Eccmode_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Eccmode {
    #[inline(always)]
    fn default() -> Eccmode {
        <crate::RegValueT<Eccmode_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eccmode {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eccmod_SPEC;
    pub type Eccmod = crate::EnumBitfieldStruct<u8, Eccmod_SPEC>;
    impl Eccmod {
        #[doc = "Disable ECC function"]
        pub const _00: Self = Self::new(0);
        #[doc = "Setting prohibited"]
        pub const _01: Self = Self::new(1);
        #[doc = "Enable ECC function without error checking"]
        pub const _10: Self = Self::new(2);
        #[doc = "Enable ECC function with error checking."]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecc2Sts_SPEC;
impl crate::sealed::RegSpec for Ecc2Sts_SPEC {
    type DataType = u8;
}
#[doc = "ECCRAM 2-Bit Error Status Register"]
pub type Ecc2Sts = crate::RegValueT<Ecc2Sts_SPEC>;

impl Ecc2Sts {
    #[doc = "ECC 2-Bit Error Status"]
    #[inline(always)]
    pub fn ecc2err(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ecc2sts::Ecc2Err, Ecc2Sts_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ecc2sts::Ecc2Err,
            Ecc2Sts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ecc2Sts {
    #[inline(always)]
    fn default() -> Ecc2Sts {
        <crate::RegValueT<Ecc2Sts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ecc2sts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ecc2Err_SPEC;
    pub type Ecc2Err = crate::EnumBitfieldStruct<u8, Ecc2Err_SPEC>;
    impl Ecc2Err {
        #[doc = "No 2-bit ECC error occurred"]
        pub const _0: Self = Self::new(0);
        #[doc = "2-bit ECC error occurred"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecc1Stsen_SPEC;
impl crate::sealed::RegSpec for Ecc1Stsen_SPEC {
    type DataType = u8;
}
#[doc = "ECCRAM 1-Bit Error Information Update Enable Register"]
pub type Ecc1Stsen = crate::RegValueT<Ecc1Stsen_SPEC>;

impl Ecc1Stsen {
    #[doc = "ECC 1-Bit Error Information Update Enable"]
    #[inline(always)]
    pub fn e1stsen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ecc1stsen::E1Stsen,
        Ecc1Stsen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ecc1stsen::E1Stsen,
            Ecc1Stsen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ecc1Stsen {
    #[inline(always)]
    fn default() -> Ecc1Stsen {
        <crate::RegValueT<Ecc1Stsen_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ecc1stsen {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct E1Stsen_SPEC;
    pub type E1Stsen = crate::EnumBitfieldStruct<u8, E1Stsen_SPEC>;
    impl E1Stsen {
        #[doc = "Disables updating of the 1-bit ECC error information."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables updating of the 1-bit ECC error information."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecc1Sts_SPEC;
impl crate::sealed::RegSpec for Ecc1Sts_SPEC {
    type DataType = u8;
}
#[doc = "ECCRAM 1-Bit Error Status Register"]
pub type Ecc1Sts = crate::RegValueT<Ecc1Sts_SPEC>;

impl Ecc1Sts {
    #[doc = "ECC 1-Bit Error Status"]
    #[inline(always)]
    pub fn ecc1err(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ecc1sts::Ecc1Err, Ecc1Sts_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ecc1sts::Ecc1Err,
            Ecc1Sts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ecc1Sts {
    #[inline(always)]
    fn default() -> Ecc1Sts {
        <crate::RegValueT<Ecc1Sts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ecc1sts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ecc1Err_SPEC;
    pub type Ecc1Err = crate::EnumBitfieldStruct<u8, Ecc1Err_SPEC>;
    impl Ecc1Err {
        #[doc = "No 1-bit ECC error occurred"]
        pub const _0: Self = Self::new(0);
        #[doc = "1-bit ECC error occurred"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eccprcr_SPEC;
impl crate::sealed::RegSpec for Eccprcr_SPEC {
    type DataType = u8;
}
#[doc = "ECCRAM Protection Register"]
pub type Eccprcr = crate::RegValueT<Eccprcr_SPEC>;

impl Eccprcr {
    #[doc = "Write Key Code"]
    #[inline(always)]
    pub fn kw(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, eccprcr::Kw, Eccprcr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<1,0x7f,1,0,eccprcr::Kw, Eccprcr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "ECCRAMETST Register Write Control"]
    #[inline(always)]
    pub fn eccprcr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, eccprcr::Eccprcr, Eccprcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            eccprcr::Eccprcr,
            Eccprcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eccprcr {
    #[inline(always)]
    fn default() -> Eccprcr {
        <crate::RegValueT<Eccprcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eccprcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Kw_SPEC;
    pub type Kw = crate::EnumBitfieldStruct<u8, Kw_SPEC>;
    impl Kw {
        #[doc = "Writing to the ECCRAMPRCR bit is valid, when the KEY bits are written 1111000b."]
        pub const _1111000: Self = Self::new(120);
        #[doc = "Writing to the ECCRAMPRCR bit is invalid."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eccprcr_SPEC;
    pub type Eccprcr = crate::EnumBitfieldStruct<u8, Eccprcr_SPEC>;
    impl Eccprcr {
        #[doc = "Disable writes to protected registers"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable writes to protected registers"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eccetst_SPEC;
impl crate::sealed::RegSpec for Eccetst_SPEC {
    type DataType = u8;
}
#[doc = "ECC Test Control Register"]
pub type Eccetst = crate::RegValueT<Eccetst_SPEC>;

impl Eccetst {
    #[doc = "ECC Bypass Select"]
    #[inline(always)]
    pub fn tstbyp(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, eccetst::Tstbyp, Eccetst_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,eccetst::Tstbyp, Eccetst_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Eccetst {
    #[inline(always)]
    fn default() -> Eccetst {
        <crate::RegValueT<Eccetst_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eccetst {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tstbyp_SPEC;
    pub type Tstbyp = crate::EnumBitfieldStruct<u8, Tstbyp_SPEC>;
    impl Tstbyp {
        #[doc = "ECC bypass disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "ECC bypass enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eccoad_SPEC;
impl crate::sealed::RegSpec for Eccoad_SPEC {
    type DataType = u8;
}
#[doc = "RAM ECC Error Operation After Detection Register"]
pub type Eccoad = crate::RegValueT<Eccoad_SPEC>;

impl Eccoad {
    #[doc = "Operation after Detection"]
    #[inline(always)]
    pub fn oad(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, eccoad::Oad, Eccoad_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,eccoad::Oad, Eccoad_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Eccoad {
    #[inline(always)]
    fn default() -> Eccoad {
        <crate::RegValueT<Eccoad_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eccoad {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oad_SPEC;
    pub type Oad = crate::EnumBitfieldStruct<u8, Oad_SPEC>;
    impl Oad {
        #[doc = "Non maskable interrupt."]
        pub const _0: Self = Self::new(0);
        #[doc = "Internal reset."]
        pub const _1: Self = Self::new(1);
    }
}
