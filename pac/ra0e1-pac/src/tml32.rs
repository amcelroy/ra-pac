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
// Generated from SVD 1.10.00, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:47 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"32-bit Interval Timer"]
unsafe impl ::core::marker::Send for super::Tml32 {}
unsafe impl ::core::marker::Sync for super::Tml32 {}
impl super::Tml32 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn itlcmp0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Itlcmp0_SPEC, crate::common::RW>,
        2,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x0usize))
        }
    }

    #[inline(always)]
    pub const fn itlcmp0_l(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Itlcmp0L_SPEC, crate::common::RW>,
        2,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x0usize))
        }
    }

    #[inline(always)]
    pub const fn itlcmp0_h(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Itlcmp0H_SPEC, crate::common::RW>,
        2,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1usize))
        }
    }

    #[inline(always)]
    pub const fn itlcap00(
        &self,
    ) -> &'static crate::common::Reg<self::Itlcap00_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Itlcap00_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn itlctl0(
        &self,
    ) -> &'static crate::common::Reg<self::Itlctl0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Itlctl0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[inline(always)]
    pub const fn itlcsel0(
        &self,
    ) -> &'static crate::common::Reg<self::Itlcsel0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Itlcsel0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(7usize),
            )
        }
    }

    #[inline(always)]
    pub const fn itlfdiv00(
        &self,
    ) -> &'static crate::common::Reg<self::Itlfdiv00_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Itlfdiv00_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn itlfdiv01(
        &self,
    ) -> &'static crate::common::Reg<self::Itlfdiv01_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Itlfdiv01_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(9usize),
            )
        }
    }

    #[inline(always)]
    pub const fn itlcc0(
        &self,
    ) -> &'static crate::common::Reg<self::Itlcc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Itlcc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[inline(always)]
    pub const fn itls0(&self) -> &'static crate::common::Reg<self::Itls0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Itls0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(11usize),
            )
        }
    }

    #[inline(always)]
    pub const fn itlmkf0(
        &self,
    ) -> &'static crate::common::Reg<self::Itlmkf0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Itlmkf0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Itlcmp0_SPEC;
impl crate::sealed::RegSpec for Itlcmp0_SPEC {
    type DataType = u16;
}

pub type Itlcmp0 = crate::RegValueT<Itlcmp0_SPEC>;

impl NoBitfieldReg<Itlcmp0_SPEC> for Itlcmp0 {}
impl ::core::default::Default for Itlcmp0 {
    #[inline(always)]
    fn default() -> Itlcmp0 {
        <crate::RegValueT<Itlcmp0_SPEC> as RegisterValue<_>>::new(65535)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Itlcmp0L_SPEC;
impl crate::sealed::RegSpec for Itlcmp0L_SPEC {
    type DataType = u8;
}

pub type Itlcmp0L = crate::RegValueT<Itlcmp0L_SPEC>;

impl NoBitfieldReg<Itlcmp0L_SPEC> for Itlcmp0L {}
impl ::core::default::Default for Itlcmp0L {
    #[inline(always)]
    fn default() -> Itlcmp0L {
        <crate::RegValueT<Itlcmp0L_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Itlcmp0H_SPEC;
impl crate::sealed::RegSpec for Itlcmp0H_SPEC {
    type DataType = u8;
}

pub type Itlcmp0H = crate::RegValueT<Itlcmp0H_SPEC>;

impl NoBitfieldReg<Itlcmp0H_SPEC> for Itlcmp0H {}
impl ::core::default::Default for Itlcmp0H {
    #[inline(always)]
    fn default() -> Itlcmp0H {
        <crate::RegValueT<Itlcmp0H_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Itlcap00_SPEC;
impl crate::sealed::RegSpec for Itlcap00_SPEC {
    type DataType = u16;
}

pub type Itlcap00 = crate::RegValueT<Itlcap00_SPEC>;

impl NoBitfieldReg<Itlcap00_SPEC> for Itlcap00 {}
impl ::core::default::Default for Itlcap00 {
    #[inline(always)]
    fn default() -> Itlcap00 {
        <crate::RegValueT<Itlcap00_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Itlctl0_SPEC;
impl crate::sealed::RegSpec for Itlctl0_SPEC {
    type DataType = u8;
}

pub type Itlctl0 = crate::RegValueT<Itlctl0_SPEC>;

impl Itlctl0 {
    #[inline(always)]
    pub fn en0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        itlctl0::En0,
        itlctl0::En0,
        Itlctl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            itlctl0::En0,
            itlctl0::En0,
            Itlctl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn en1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        itlctl0::En1,
        itlctl0::En1,
        Itlctl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            itlctl0::En1,
            itlctl0::En1,
            Itlctl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn en2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        itlctl0::En2,
        itlctl0::En2,
        Itlctl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            itlctl0::En2,
            itlctl0::En2,
            Itlctl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn en3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        itlctl0::En3,
        itlctl0::En3,
        Itlctl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            itlctl0::En3,
            itlctl0::En3,
            Itlctl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn md(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        itlctl0::Md,
        itlctl0::Md,
        Itlctl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            itlctl0::Md,
            itlctl0::Md,
            Itlctl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Itlctl0 {
    #[inline(always)]
    fn default() -> Itlctl0 {
        <crate::RegValueT<Itlctl0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod itlctl0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En0_SPEC;
    pub type En0 = crate::EnumBitfieldStruct<u8, En0_SPEC>;
    impl En0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En1_SPEC;
    pub type En1 = crate::EnumBitfieldStruct<u8, En1_SPEC>;
    impl En1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En2_SPEC;
    pub type En2 = crate::EnumBitfieldStruct<u8, En2_SPEC>;
    impl En2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En3_SPEC;
    pub type En3 = crate::EnumBitfieldStruct<u8, En3_SPEC>;
    impl En3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Md_SPEC;
    pub type Md = crate::EnumBitfieldStruct<u8, Md_SPEC>;
    impl Md {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Itlcsel0_SPEC;
impl crate::sealed::RegSpec for Itlcsel0_SPEC {
    type DataType = u8;
}

pub type Itlcsel0 = crate::RegValueT<Itlcsel0_SPEC>;

impl Itlcsel0 {
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        itlcsel0::Isel,
        itlcsel0::Isel,
        Itlcsel0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            itlcsel0::Isel,
            itlcsel0::Isel,
            Itlcsel0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn csel(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x7,
        1,
        0,
        itlcsel0::Csel,
        itlcsel0::Csel,
        Itlcsel0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            itlcsel0::Csel,
            itlcsel0::Csel,
            Itlcsel0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Itlcsel0 {
    #[inline(always)]
    fn default() -> Itlcsel0 {
        <crate::RegValueT<Itlcsel0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod itlcsel0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _101: Self = Self::new(5);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csel_SPEC;
    pub type Csel = crate::EnumBitfieldStruct<u8, Csel_SPEC>;
    impl Csel {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _101: Self = Self::new(5);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Itlfdiv00_SPEC;
impl crate::sealed::RegSpec for Itlfdiv00_SPEC {
    type DataType = u8;
}

pub type Itlfdiv00 = crate::RegValueT<Itlfdiv00_SPEC>;

impl Itlfdiv00 {
    #[inline(always)]
    pub fn fdiv0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        itlfdiv00::Fdiv0,
        itlfdiv00::Fdiv0,
        Itlfdiv00_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            itlfdiv00::Fdiv0,
            itlfdiv00::Fdiv0,
            Itlfdiv00_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fdiv1(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x7,
        1,
        0,
        itlfdiv00::Fdiv1,
        itlfdiv00::Fdiv1,
        Itlfdiv00_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            itlfdiv00::Fdiv1,
            itlfdiv00::Fdiv1,
            Itlfdiv00_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Itlfdiv00 {
    #[inline(always)]
    fn default() -> Itlfdiv00 {
        <crate::RegValueT<Itlfdiv00_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod itlfdiv00 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fdiv0_SPEC;
    pub type Fdiv0 = crate::EnumBitfieldStruct<u8, Fdiv0_SPEC>;
    impl Fdiv0 {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _101: Self = Self::new(5);

        pub const _110: Self = Self::new(6);

        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fdiv1_SPEC;
    pub type Fdiv1 = crate::EnumBitfieldStruct<u8, Fdiv1_SPEC>;
    impl Fdiv1 {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _101: Self = Self::new(5);

        pub const _110: Self = Self::new(6);

        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Itlfdiv01_SPEC;
impl crate::sealed::RegSpec for Itlfdiv01_SPEC {
    type DataType = u8;
}

pub type Itlfdiv01 = crate::RegValueT<Itlfdiv01_SPEC>;

impl Itlfdiv01 {
    #[inline(always)]
    pub fn fdiv2(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        itlfdiv01::Fdiv2,
        itlfdiv01::Fdiv2,
        Itlfdiv01_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            itlfdiv01::Fdiv2,
            itlfdiv01::Fdiv2,
            Itlfdiv01_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fdiv3(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x7,
        1,
        0,
        itlfdiv01::Fdiv3,
        itlfdiv01::Fdiv3,
        Itlfdiv01_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            itlfdiv01::Fdiv3,
            itlfdiv01::Fdiv3,
            Itlfdiv01_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Itlfdiv01 {
    #[inline(always)]
    fn default() -> Itlfdiv01 {
        <crate::RegValueT<Itlfdiv01_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod itlfdiv01 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fdiv2_SPEC;
    pub type Fdiv2 = crate::EnumBitfieldStruct<u8, Fdiv2_SPEC>;
    impl Fdiv2 {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _101: Self = Self::new(5);

        pub const _110: Self = Self::new(6);

        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fdiv3_SPEC;
    pub type Fdiv3 = crate::EnumBitfieldStruct<u8, Fdiv3_SPEC>;
    impl Fdiv3 {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _101: Self = Self::new(5);

        pub const _110: Self = Self::new(6);

        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Itlcc0_SPEC;
impl crate::sealed::RegSpec for Itlcc0_SPEC {
    type DataType = u8;
}

pub type Itlcc0 = crate::RegValueT<Itlcc0_SPEC>;

impl Itlcc0 {
    #[inline(always)]
    pub fn ctrs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        itlcc0::Ctrs,
        itlcc0::Ctrs,
        Itlcc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            itlcc0::Ctrs,
            itlcc0::Ctrs,
            Itlcc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn capccr(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        itlcc0::Capccr,
        itlcc0::Capccr,
        Itlcc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            itlcc0::Capccr,
            itlcc0::Capccr,
            Itlcc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn capr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        itlcc0::Capr,
        itlcc0::Capr,
        Itlcc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            itlcc0::Capr,
            itlcc0::Capr,
            Itlcc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn capf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        itlcc0::Capf,
        itlcc0::Capf,
        Itlcc0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            itlcc0::Capf,
            itlcc0::Capf,
            Itlcc0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn capfcr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        itlcc0::Capfcr,
        itlcc0::Capfcr,
        Itlcc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            itlcc0::Capfcr,
            itlcc0::Capfcr,
            Itlcc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn capen(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        itlcc0::Capen,
        itlcc0::Capen,
        Itlcc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            itlcc0::Capen,
            itlcc0::Capen,
            Itlcc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Itlcc0 {
    #[inline(always)]
    fn default() -> Itlcc0 {
        <crate::RegValueT<Itlcc0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod itlcc0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctrs_SPEC;
    pub type Ctrs = crate::EnumBitfieldStruct<u8, Ctrs_SPEC>;
    impl Ctrs {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Capccr_SPEC;
    pub type Capccr = crate::EnumBitfieldStruct<u8, Capccr_SPEC>;
    impl Capccr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Capr_SPEC;
    pub type Capr = crate::EnumBitfieldStruct<u8, Capr_SPEC>;
    impl Capr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Capf_SPEC;
    pub type Capf = crate::EnumBitfieldStruct<u8, Capf_SPEC>;
    impl Capf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Capfcr_SPEC;
    pub type Capfcr = crate::EnumBitfieldStruct<u8, Capfcr_SPEC>;
    impl Capfcr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Capen_SPEC;
    pub type Capen = crate::EnumBitfieldStruct<u8, Capen_SPEC>;
    impl Capen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Itls0_SPEC;
impl crate::sealed::RegSpec for Itls0_SPEC {
    type DataType = u8;
}

pub type Itls0 = crate::RegValueT<Itls0_SPEC>;

impl Itls0 {
    #[inline(always)]
    pub fn itf00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        itls0::Itf00,
        itls0::Itf00,
        Itls0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            itls0::Itf00,
            itls0::Itf00,
            Itls0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn itf01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        itls0::Itf01,
        itls0::Itf01,
        Itls0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            itls0::Itf01,
            itls0::Itf01,
            Itls0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn itf02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        itls0::Itf02,
        itls0::Itf02,
        Itls0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            itls0::Itf02,
            itls0::Itf02,
            Itls0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn itf03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        itls0::Itf03,
        itls0::Itf03,
        Itls0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            itls0::Itf03,
            itls0::Itf03,
            Itls0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn itf0c(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        itls0::Itf0C,
        itls0::Itf0C,
        Itls0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            itls0::Itf0C,
            itls0::Itf0C,
            Itls0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Itls0 {
    #[inline(always)]
    fn default() -> Itls0 {
        <crate::RegValueT<Itls0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod itls0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Itf00_SPEC;
    pub type Itf00 = crate::EnumBitfieldStruct<u8, Itf00_SPEC>;
    impl Itf00 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Itf01_SPEC;
    pub type Itf01 = crate::EnumBitfieldStruct<u8, Itf01_SPEC>;
    impl Itf01 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Itf02_SPEC;
    pub type Itf02 = crate::EnumBitfieldStruct<u8, Itf02_SPEC>;
    impl Itf02 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Itf03_SPEC;
    pub type Itf03 = crate::EnumBitfieldStruct<u8, Itf03_SPEC>;
    impl Itf03 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Itf0C_SPEC;
    pub type Itf0C = crate::EnumBitfieldStruct<u8, Itf0C_SPEC>;
    impl Itf0C {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Itlmkf0_SPEC;
impl crate::sealed::RegSpec for Itlmkf0_SPEC {
    type DataType = u8;
}

pub type Itlmkf0 = crate::RegValueT<Itlmkf0_SPEC>;

impl Itlmkf0 {
    #[inline(always)]
    pub fn mkf00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        itlmkf0::Mkf00,
        itlmkf0::Mkf00,
        Itlmkf0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            itlmkf0::Mkf00,
            itlmkf0::Mkf00,
            Itlmkf0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mkf01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        itlmkf0::Mkf01,
        itlmkf0::Mkf01,
        Itlmkf0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            itlmkf0::Mkf01,
            itlmkf0::Mkf01,
            Itlmkf0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mkf02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        itlmkf0::Mkf02,
        itlmkf0::Mkf02,
        Itlmkf0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            itlmkf0::Mkf02,
            itlmkf0::Mkf02,
            Itlmkf0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mkf03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        itlmkf0::Mkf03,
        itlmkf0::Mkf03,
        Itlmkf0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            itlmkf0::Mkf03,
            itlmkf0::Mkf03,
            Itlmkf0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mkf0c(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        itlmkf0::Mkf0C,
        itlmkf0::Mkf0C,
        Itlmkf0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            itlmkf0::Mkf0C,
            itlmkf0::Mkf0C,
            Itlmkf0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Itlmkf0 {
    #[inline(always)]
    fn default() -> Itlmkf0 {
        <crate::RegValueT<Itlmkf0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod itlmkf0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mkf00_SPEC;
    pub type Mkf00 = crate::EnumBitfieldStruct<u8, Mkf00_SPEC>;
    impl Mkf00 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mkf01_SPEC;
    pub type Mkf01 = crate::EnumBitfieldStruct<u8, Mkf01_SPEC>;
    impl Mkf01 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mkf02_SPEC;
    pub type Mkf02 = crate::EnumBitfieldStruct<u8, Mkf02_SPEC>;
    impl Mkf02 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mkf03_SPEC;
    pub type Mkf03 = crate::EnumBitfieldStruct<u8, Mkf03_SPEC>;
    impl Mkf03 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mkf0C_SPEC;
    pub type Mkf0C = crate::EnumBitfieldStruct<u8, Mkf0C_SPEC>;
    impl Mkf0C {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
