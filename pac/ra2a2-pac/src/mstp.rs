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
// Generated from SVD 1.20.02, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:14:46 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Module Stop Control A, B, C, D"]
unsafe impl ::core::marker::Send for super::Mstp {}
unsafe impl ::core::marker::Sync for super::Mstp {}
impl super::Mstp {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "Module Stop Control Register B"]
    #[inline(always)]
    pub const fn mstpcrb(
        &self,
    ) -> &'static crate::common::Reg<self::Mstpcrb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mstpcrb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Module Stop Control Register C"]
    #[inline(always)]
    pub const fn mstpcrc(
        &self,
    ) -> &'static crate::common::Reg<self::Mstpcrc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mstpcrc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Module Stop Control Register D"]
    #[inline(always)]
    pub const fn mstpcrd(
        &self,
    ) -> &'static crate::common::Reg<self::Mstpcrd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mstpcrd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Low Speed Module R/W Disable Control Register"]
    #[inline(always)]
    pub const fn lsmrwdis(
        &self,
    ) -> &'static crate::common::Reg<self::Lsmrwdis_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lsmrwdis_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstpcrb_SPEC;
impl crate::sealed::RegSpec for Mstpcrb_SPEC {
    type DataType = u32;
}
#[doc = "Module Stop Control Register B"]
pub type Mstpcrb = crate::RegValueT<Mstpcrb_SPEC>;

impl Mstpcrb {
    #[doc = "I2C Bus Interface 1 Module Stop"]
    #[inline(always)]
    pub fn mstpb8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, mstpcrb::Mstpb8, Mstpcrb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,mstpcrb::Mstpb8, Mstpcrb_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "I2C Bus Interface 0 Module Stop"]
    #[inline(always)]
    pub fn mstpb9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, mstpcrb::Mstpb9, Mstpcrb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,mstpcrb::Mstpb9, Mstpcrb_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Serial Peripheral Interface 0 Module Stop"]
    #[inline(always)]
    pub fn mstpb19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        mstpcrb::Mstpb19,
        Mstpcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            mstpcrb::Mstpb19,
            Mstpcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Serial Communication Interface 9 Module Stop"]
    #[inline(always)]
    pub fn mstpb22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        mstpcrb::Mstpb22,
        Mstpcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            mstpcrb::Mstpb22,
            Mstpcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Serial Communication Interface 3 Module Stop"]
    #[inline(always)]
    pub fn mstpb28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        mstpcrb::Mstpb28,
        Mstpcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            mstpcrb::Mstpb28,
            Mstpcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Serial Communication Interface 2 Module Stop"]
    #[inline(always)]
    pub fn mstpb29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        mstpcrb::Mstpb29,
        Mstpcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            mstpcrb::Mstpb29,
            Mstpcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Serial Communication Interface 1 Module Stop"]
    #[inline(always)]
    pub fn mstpb30(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        mstpcrb::Mstpb30,
        Mstpcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            mstpcrb::Mstpb30,
            Mstpcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Serial Communication Interface 0 Module Stop"]
    #[inline(always)]
    pub fn mstpb31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        mstpcrb::Mstpb31,
        Mstpcrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            mstpcrb::Mstpb31,
            Mstpcrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mstpcrb {
    #[inline(always)]
    fn default() -> Mstpcrb {
        <crate::RegValueT<Mstpcrb_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod mstpcrb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb8_SPEC;
    pub type Mstpb8 = crate::EnumBitfieldStruct<u8, Mstpb8_SPEC>;
    impl Mstpb8 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb9_SPEC;
    pub type Mstpb9 = crate::EnumBitfieldStruct<u8, Mstpb9_SPEC>;
    impl Mstpb9 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb19_SPEC;
    pub type Mstpb19 = crate::EnumBitfieldStruct<u8, Mstpb19_SPEC>;
    impl Mstpb19 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb22_SPEC;
    pub type Mstpb22 = crate::EnumBitfieldStruct<u8, Mstpb22_SPEC>;
    impl Mstpb22 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb28_SPEC;
    pub type Mstpb28 = crate::EnumBitfieldStruct<u8, Mstpb28_SPEC>;
    impl Mstpb28 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb29_SPEC;
    pub type Mstpb29 = crate::EnumBitfieldStruct<u8, Mstpb29_SPEC>;
    impl Mstpb29 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb30_SPEC;
    pub type Mstpb30 = crate::EnumBitfieldStruct<u8, Mstpb30_SPEC>;
    impl Mstpb30 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpb31_SPEC;
    pub type Mstpb31 = crate::EnumBitfieldStruct<u8, Mstpb31_SPEC>;
    impl Mstpb31 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstpcrc_SPEC;
impl crate::sealed::RegSpec for Mstpcrc_SPEC {
    type DataType = u32;
}
#[doc = "Module Stop Control Register C"]
pub type Mstpcrc = crate::RegValueT<Mstpcrc_SPEC>;

impl Mstpcrc {
    #[doc = "Clock Frequency Accuracy Measurement Circuit Module Stop"]
    #[inline(always)]
    pub fn mstpc0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, mstpcrc::Mstpc0, Mstpcrc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,mstpcrc::Mstpc0, Mstpcrc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Cyclic Redundancy Check Calculator Module Stop"]
    #[inline(always)]
    pub fn mstpc1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, mstpcrc::Mstpc1, Mstpcrc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,mstpcrc::Mstpc1, Mstpcrc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Segment LCD Controller Module Stop"]
    #[inline(always)]
    pub fn mstpc4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, mstpcrc::Mstpc4, Mstpcrc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,mstpcrc::Mstpc4, Mstpcrc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Data Operation Circuit Module Stop"]
    #[inline(always)]
    pub fn mstpc13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        mstpcrc::Mstpc13,
        Mstpcrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            mstpcrc::Mstpc13,
            Mstpcrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Event Link Controller Module Stop"]
    #[inline(always)]
    pub fn mstpc14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        mstpcrc::Mstpc14,
        Mstpcrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            mstpcrc::Mstpc14,
            Mstpcrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "32-bit Multiply-Accumulator Module Stop"]
    #[inline(always)]
    pub fn mstpc15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        mstpcrc::Mstpc15,
        Mstpcrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            mstpcrc::Mstpc15,
            Mstpcrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "True Random Number Generator Module Stop"]
    #[inline(always)]
    pub fn mstpc28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        mstpcrc::Mstpc28,
        Mstpcrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            mstpcrc::Mstpc28,
            Mstpcrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "AES Module Stop"]
    #[inline(always)]
    pub fn mstpc31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        mstpcrc::Mstpc31,
        Mstpcrc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            mstpcrc::Mstpc31,
            Mstpcrc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mstpcrc {
    #[inline(always)]
    fn default() -> Mstpcrc {
        <crate::RegValueT<Mstpcrc_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod mstpcrc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpc0_SPEC;
    pub type Mstpc0 = crate::EnumBitfieldStruct<u8, Mstpc0_SPEC>;
    impl Mstpc0 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpc1_SPEC;
    pub type Mstpc1 = crate::EnumBitfieldStruct<u8, Mstpc1_SPEC>;
    impl Mstpc1 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpc4_SPEC;
    pub type Mstpc4 = crate::EnumBitfieldStruct<u8, Mstpc4_SPEC>;
    impl Mstpc4 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpc13_SPEC;
    pub type Mstpc13 = crate::EnumBitfieldStruct<u8, Mstpc13_SPEC>;
    impl Mstpc13 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpc14_SPEC;
    pub type Mstpc14 = crate::EnumBitfieldStruct<u8, Mstpc14_SPEC>;
    impl Mstpc14 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpc15_SPEC;
    pub type Mstpc15 = crate::EnumBitfieldStruct<u8, Mstpc15_SPEC>;
    impl Mstpc15 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpc28_SPEC;
    pub type Mstpc28 = crate::EnumBitfieldStruct<u8, Mstpc28_SPEC>;
    impl Mstpc28 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpc31_SPEC;
    pub type Mstpc31 = crate::EnumBitfieldStruct<u8, Mstpc31_SPEC>;
    impl Mstpc31 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstpcrd_SPEC;
impl crate::sealed::RegSpec for Mstpcrd_SPEC {
    type DataType = u32;
}
#[doc = "Module Stop Control Register D"]
pub type Mstpcrd = crate::RegValueT<Mstpcrd_SPEC>;

impl Mstpcrd {
    #[doc = "16-bit Low Power Asynchronous General Purpose Timer 3 Module Stop"]
    #[inline(always)]
    pub fn mstpd0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, mstpcrd::Mstpd0, Mstpcrd_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,mstpcrd::Mstpd0, Mstpcrd_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "16-bit Low Power Asynchronous General Purpose Timer 2 Module Stop"]
    #[inline(always)]
    pub fn mstpd1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, mstpcrd::Mstpd1, Mstpcrd_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,mstpcrd::Mstpd1, Mstpcrd_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "32-bit Low Power Asynchronous General Purpose Timer 1 Module Stop"]
    #[inline(always)]
    pub fn mstpd2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, mstpcrd::Mstpd2, Mstpcrd_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,mstpcrd::Mstpd2, Mstpcrd_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "32-bit Low Power Asynchronous General Purpose Timer 0 Module Stop"]
    #[inline(always)]
    pub fn mstpd3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, mstpcrd::Mstpd3, Mstpcrd_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,mstpcrd::Mstpd3, Mstpcrd_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "General PWM Timer 164 to 169 and PWM Delay Generation Circuit Module Stop"]
    #[inline(always)]
    pub fn mstpd6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, mstpcrd::Mstpd6, Mstpcrd_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,mstpcrd::Mstpd6, Mstpcrd_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "16-bit Low Power Asynchronous General Purpose Timer 7 Module Stop"]
    #[inline(always)]
    pub fn mstpd7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, mstpcrd::Mstpd7, Mstpcrd_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,mstpcrd::Mstpd7, Mstpcrd_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "16-bit Low Power Asynchronous General Purpose Timer 6 Module Stop"]
    #[inline(always)]
    pub fn mstpd8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, mstpcrd::Mstpd8, Mstpcrd_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,mstpcrd::Mstpd8, Mstpcrd_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "16-bit Low Power Asynchronous General Purpose Timer 5 Module Stop"]
    #[inline(always)]
    pub fn mstpd9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, mstpcrd::Mstpd9, Mstpcrd_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,mstpcrd::Mstpd9, Mstpcrd_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "16-bit Low Power Asynchronous General Purpose Timer 4 Module Stop"]
    #[inline(always)]
    pub fn mstpd10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        mstpcrd::Mstpd10,
        Mstpcrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            mstpcrd::Mstpd10,
            Mstpcrd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Port Output Enable for GPT Module Stop"]
    #[inline(always)]
    pub fn mstpd14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        mstpcrd::Mstpd14,
        Mstpcrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            mstpcrd::Mstpd14,
            Mstpcrd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "12-bit A/D Converter Module Stop"]
    #[inline(always)]
    pub fn mstpd16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        mstpcrd::Mstpd16,
        Mstpcrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            mstpcrd::Mstpd16,
            Mstpcrd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "24-bit Sigma-Delta A/D Converter Module Stop"]
    #[inline(always)]
    pub fn mstpd17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        mstpcrd::Mstpd17,
        Mstpcrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            mstpcrd::Mstpd17,
            Mstpcrd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "16-bit Low Power Asynchronous General Purpose Timer 1 Module Stop"]
    #[inline(always)]
    pub fn mstpd18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        mstpcrd::Mstpd18,
        Mstpcrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            mstpcrd::Mstpd18,
            Mstpcrd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "16-bit Low Power Asynchronous General Purpose Timer 0 Module Stop"]
    #[inline(always)]
    pub fn mstpd19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        mstpcrd::Mstpd19,
        Mstpcrd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            mstpcrd::Mstpd19,
            Mstpcrd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mstpcrd {
    #[inline(always)]
    fn default() -> Mstpcrd {
        <crate::RegValueT<Mstpcrd_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod mstpcrd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd0_SPEC;
    pub type Mstpd0 = crate::EnumBitfieldStruct<u8, Mstpd0_SPEC>;
    impl Mstpd0 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd1_SPEC;
    pub type Mstpd1 = crate::EnumBitfieldStruct<u8, Mstpd1_SPEC>;
    impl Mstpd1 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd2_SPEC;
    pub type Mstpd2 = crate::EnumBitfieldStruct<u8, Mstpd2_SPEC>;
    impl Mstpd2 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd3_SPEC;
    pub type Mstpd3 = crate::EnumBitfieldStruct<u8, Mstpd3_SPEC>;
    impl Mstpd3 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd6_SPEC;
    pub type Mstpd6 = crate::EnumBitfieldStruct<u8, Mstpd6_SPEC>;
    impl Mstpd6 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd7_SPEC;
    pub type Mstpd7 = crate::EnumBitfieldStruct<u8, Mstpd7_SPEC>;
    impl Mstpd7 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd8_SPEC;
    pub type Mstpd8 = crate::EnumBitfieldStruct<u8, Mstpd8_SPEC>;
    impl Mstpd8 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd9_SPEC;
    pub type Mstpd9 = crate::EnumBitfieldStruct<u8, Mstpd9_SPEC>;
    impl Mstpd9 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd10_SPEC;
    pub type Mstpd10 = crate::EnumBitfieldStruct<u8, Mstpd10_SPEC>;
    impl Mstpd10 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd14_SPEC;
    pub type Mstpd14 = crate::EnumBitfieldStruct<u8, Mstpd14_SPEC>;
    impl Mstpd14 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd16_SPEC;
    pub type Mstpd16 = crate::EnumBitfieldStruct<u8, Mstpd16_SPEC>;
    impl Mstpd16 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd17_SPEC;
    pub type Mstpd17 = crate::EnumBitfieldStruct<u8, Mstpd17_SPEC>;
    impl Mstpd17 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd18_SPEC;
    pub type Mstpd18 = crate::EnumBitfieldStruct<u8, Mstpd18_SPEC>;
    impl Mstpd18 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpd19_SPEC;
    pub type Mstpd19 = crate::EnumBitfieldStruct<u8, Mstpd19_SPEC>;
    impl Mstpd19 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lsmrwdis_SPEC;
impl crate::sealed::RegSpec for Lsmrwdis_SPEC {
    type DataType = u16;
}
#[doc = "Low Speed Module R/W Disable Control Register"]
pub type Lsmrwdis = crate::RegValueT<Lsmrwdis_SPEC>;

impl Lsmrwdis {
    #[doc = "RTC Register R/W Enable Control"]
    #[inline(always)]
    pub fn rtcrwdis(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        lsmrwdis::Rtcrwdis,
        Lsmrwdis_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            lsmrwdis::Rtcrwdis,
            Lsmrwdis_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "WDT Operate Clock Control"]
    #[inline(always)]
    pub fn wdtdis(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        lsmrwdis::Wdtdis,
        Lsmrwdis_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            lsmrwdis::Wdtdis,
            Lsmrwdis_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "IWDT Register Clock Control"]
    #[inline(always)]
    pub fn iwdtids(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        lsmrwdis::Iwdtids,
        Lsmrwdis_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            lsmrwdis::Iwdtids,
            Lsmrwdis_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Write Enable for bits \\[2:0\\]"]
    #[inline(always)]
    pub fn wren(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, lsmrwdis::Wren, Lsmrwdis_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,lsmrwdis::Wren, Lsmrwdis_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LSMRWDIS Key Code"]
    #[inline(always)]
    pub fn prkey(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Lsmrwdis_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Lsmrwdis_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Lsmrwdis {
    #[inline(always)]
    fn default() -> Lsmrwdis {
        <crate::RegValueT<Lsmrwdis_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod lsmrwdis {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtcrwdis_SPEC;
    pub type Rtcrwdis = crate::EnumBitfieldStruct<u8, Rtcrwdis_SPEC>;
    impl Rtcrwdis {
        #[doc = "RTC register R/W clock always on"]
        pub const _0: Self = Self::new(0);
        #[doc = "RTC register R/W clock stops"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wdtdis_SPEC;
    pub type Wdtdis = crate::EnumBitfieldStruct<u8, Wdtdis_SPEC>;
    impl Wdtdis {
        #[doc = "WDT operates as normal"]
        pub const _0: Self = Self::new(0);
        #[doc = "Stop the WDT clock and register R/W clock"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iwdtids_SPEC;
    pub type Iwdtids = crate::EnumBitfieldStruct<u8, Iwdtids_SPEC>;
    impl Iwdtids {
        #[doc = "IWDT operates as normal"]
        pub const _0: Self = Self::new(0);
        #[doc = "Stop the IWDT register R/W clock"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wren_SPEC;
    pub type Wren = crate::EnumBitfieldStruct<u8, Wren_SPEC>;
    impl Wren {
        #[doc = "Write protect for bits \\[2:0\\]"]
        pub const _0: Self = Self::new(0);
        #[doc = "Write enable for bits \\[2:0\\]"]
        pub const _1: Self = Self::new(1);
    }
}
