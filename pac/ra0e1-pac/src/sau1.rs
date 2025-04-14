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
#[doc = r"Serial Array Unit 1"]
unsafe impl ::core::marker::Send for super::Sau1 {}
unsafe impl ::core::marker::Sync for super::Sau1 {}
impl super::Sau1 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn sdr1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Sdr1_SPEC, crate::common::RW>,
        2,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x0usize))
        }
    }

    #[inline(always)]
    pub const fn ssr10(&self) -> &'static crate::common::Reg<self::Ssr10_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ssr10_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ssr11(&self) -> &'static crate::common::Reg<self::Ssr11_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ssr11_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(258usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sir10(&self) -> &'static crate::common::Reg<self::Sir10_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sir10_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(264usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sir11(&self) -> &'static crate::common::Reg<self::Sir11_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sir11_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(266usize),
            )
        }
    }

    #[inline(always)]
    pub const fn smr10(&self) -> &'static crate::common::Reg<self::Smr10_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Smr10_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(272usize),
            )
        }
    }

    #[inline(always)]
    pub const fn smr11(&self) -> &'static crate::common::Reg<self::Smr11_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Smr11_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(274usize),
            )
        }
    }

    #[inline(always)]
    pub const fn scr10(&self) -> &'static crate::common::Reg<self::Scr10_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Scr10_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(280usize),
            )
        }
    }

    #[inline(always)]
    pub const fn scr11(&self) -> &'static crate::common::Reg<self::Scr11_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Scr11_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(282usize),
            )
        }
    }

    #[inline(always)]
    pub const fn se1(&self) -> &'static crate::common::Reg<self::Se1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Se1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(288usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ss1(&self) -> &'static crate::common::Reg<self::Ss1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ss1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(290usize),
            )
        }
    }

    #[inline(always)]
    pub const fn st1(&self) -> &'static crate::common::Reg<self::St1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::St1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(292usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sps1(&self) -> &'static crate::common::Reg<self::Sps1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sps1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(294usize),
            )
        }
    }

    #[inline(always)]
    pub const fn so1(&self) -> &'static crate::common::Reg<self::So1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::So1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(296usize),
            )
        }
    }

    #[inline(always)]
    pub const fn soe1(&self) -> &'static crate::common::Reg<self::Soe1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Soe1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(298usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sol1(&self) -> &'static crate::common::Reg<self::Sol1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sol1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(308usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdr1_SPEC;
impl crate::sealed::RegSpec for Sdr1_SPEC {
    type DataType = u16;
}

pub type Sdr1 = crate::RegValueT<Sdr1_SPEC>;

impl Sdr1 {
    #[inline(always)]
    pub fn dat(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Sdr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Sdr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn stclk(
        self,
    ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, u8, Sdr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x7f,1,0,u8,u8,Sdr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sdr1 {
    #[inline(always)]
    fn default() -> Sdr1 {
        <crate::RegValueT<Sdr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssr10_SPEC;
impl crate::sealed::RegSpec for Ssr10_SPEC {
    type DataType = u16;
}

pub type Ssr10 = crate::RegValueT<Ssr10_SPEC>;

impl Ssr10 {
    #[inline(always)]
    pub fn ovf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ssr10::Ovf,
        ssr10::Ovf,
        Ssr10_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ssr10::Ovf,
            ssr10::Ovf,
            Ssr10_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pef(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ssr10::Pef,
        ssr10::Pef,
        Ssr10_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ssr10::Pef,
            ssr10::Pef,
            Ssr10_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bff(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ssr10::Bff,
        ssr10::Bff,
        Ssr10_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ssr10::Bff,
            ssr10::Bff,
            Ssr10_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tsf(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ssr10::Tsf,
        ssr10::Tsf,
        Ssr10_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ssr10::Tsf,
            ssr10::Tsf,
            Ssr10_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ssr10 {
    #[inline(always)]
    fn default() -> Ssr10 {
        <crate::RegValueT<Ssr10_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ssr10 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovf_SPEC;
    pub type Ovf = crate::EnumBitfieldStruct<u8, Ovf_SPEC>;
    impl Ovf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pef_SPEC;
    pub type Pef = crate::EnumBitfieldStruct<u8, Pef_SPEC>;
    impl Pef {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bff_SPEC;
    pub type Bff = crate::EnumBitfieldStruct<u8, Bff_SPEC>;
    impl Bff {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsf_SPEC;
    pub type Tsf = crate::EnumBitfieldStruct<u8, Tsf_SPEC>;
    impl Tsf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssr11_SPEC;
impl crate::sealed::RegSpec for Ssr11_SPEC {
    type DataType = u16;
}

pub type Ssr11 = crate::RegValueT<Ssr11_SPEC>;

impl Ssr11 {
    #[inline(always)]
    pub fn ovf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ssr11::Ovf,
        ssr11::Ovf,
        Ssr11_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ssr11::Ovf,
            ssr11::Ovf,
            Ssr11_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pef(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ssr11::Pef,
        ssr11::Pef,
        Ssr11_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ssr11::Pef,
            ssr11::Pef,
            Ssr11_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fef(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ssr11::Fef,
        ssr11::Fef,
        Ssr11_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ssr11::Fef,
            ssr11::Fef,
            Ssr11_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bff(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ssr11::Bff,
        ssr11::Bff,
        Ssr11_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ssr11::Bff,
            ssr11::Bff,
            Ssr11_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tsf(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ssr11::Tsf,
        ssr11::Tsf,
        Ssr11_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ssr11::Tsf,
            ssr11::Tsf,
            Ssr11_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ssr11 {
    #[inline(always)]
    fn default() -> Ssr11 {
        <crate::RegValueT<Ssr11_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ssr11 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovf_SPEC;
    pub type Ovf = crate::EnumBitfieldStruct<u8, Ovf_SPEC>;
    impl Ovf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pef_SPEC;
    pub type Pef = crate::EnumBitfieldStruct<u8, Pef_SPEC>;
    impl Pef {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fef_SPEC;
    pub type Fef = crate::EnumBitfieldStruct<u8, Fef_SPEC>;
    impl Fef {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bff_SPEC;
    pub type Bff = crate::EnumBitfieldStruct<u8, Bff_SPEC>;
    impl Bff {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsf_SPEC;
    pub type Tsf = crate::EnumBitfieldStruct<u8, Tsf_SPEC>;
    impl Tsf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sir10_SPEC;
impl crate::sealed::RegSpec for Sir10_SPEC {
    type DataType = u16;
}

pub type Sir10 = crate::RegValueT<Sir10_SPEC>;

impl Sir10 {
    #[inline(always)]
    pub fn ovct(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sir10::Ovct,
        sir10::Ovct,
        Sir10_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sir10::Ovct,
            sir10::Ovct,
            Sir10_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pect(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        sir10::Pect,
        sir10::Pect,
        Sir10_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sir10::Pect,
            sir10::Pect,
            Sir10_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sir10 {
    #[inline(always)]
    fn default() -> Sir10 {
        <crate::RegValueT<Sir10_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sir10 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovct_SPEC;
    pub type Ovct = crate::EnumBitfieldStruct<u8, Ovct_SPEC>;
    impl Ovct {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pect_SPEC;
    pub type Pect = crate::EnumBitfieldStruct<u8, Pect_SPEC>;
    impl Pect {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sir11_SPEC;
impl crate::sealed::RegSpec for Sir11_SPEC {
    type DataType = u16;
}

pub type Sir11 = crate::RegValueT<Sir11_SPEC>;

impl Sir11 {
    #[inline(always)]
    pub fn ovct(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sir11::Ovct,
        sir11::Ovct,
        Sir11_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sir11::Ovct,
            sir11::Ovct,
            Sir11_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pect(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        sir11::Pect,
        sir11::Pect,
        Sir11_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sir11::Pect,
            sir11::Pect,
            Sir11_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fect(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        sir11::Fect,
        sir11::Fect,
        Sir11_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            sir11::Fect,
            sir11::Fect,
            Sir11_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sir11 {
    #[inline(always)]
    fn default() -> Sir11 {
        <crate::RegValueT<Sir11_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sir11 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovct_SPEC;
    pub type Ovct = crate::EnumBitfieldStruct<u8, Ovct_SPEC>;
    impl Ovct {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pect_SPEC;
    pub type Pect = crate::EnumBitfieldStruct<u8, Pect_SPEC>;
    impl Pect {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fect_SPEC;
    pub type Fect = crate::EnumBitfieldStruct<u8, Fect_SPEC>;
    impl Fect {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smr10_SPEC;
impl crate::sealed::RegSpec for Smr10_SPEC {
    type DataType = u16;
}

pub type Smr10 = crate::RegValueT<Smr10_SPEC>;

impl Smr10 {
    #[inline(always)]
    pub fn md0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        smr10::Md0,
        smr10::Md0,
        Smr10_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            smr10::Md0,
            smr10::Md0,
            Smr10_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn md1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x3,
        1,
        0,
        smr10::Md1,
        smr10::Md1,
        Smr10_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x3,
            1,
            0,
            smr10::Md1,
            smr10::Md1,
            Smr10_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ccs(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        smr10::Ccs,
        smr10::Ccs,
        Smr10_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            smr10::Ccs,
            smr10::Ccs,
            Smr10_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        smr10::Cks,
        smr10::Cks,
        Smr10_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            smr10::Cks,
            smr10::Cks,
            Smr10_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Smr10 {
    #[inline(always)]
    fn default() -> Smr10 {
        <crate::RegValueT<Smr10_SPEC> as RegisterValue<_>>::new(32)
    }
}
pub mod smr10 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Md0_SPEC;
    pub type Md0 = crate::EnumBitfieldStruct<u8, Md0_SPEC>;
    impl Md0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Md1_SPEC;
    pub type Md1 = crate::EnumBitfieldStruct<u8, Md1_SPEC>;
    impl Md1 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccs_SPEC;
    pub type Ccs = crate::EnumBitfieldStruct<u8, Ccs_SPEC>;
    impl Ccs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smr11_SPEC;
impl crate::sealed::RegSpec for Smr11_SPEC {
    type DataType = u16;
}

pub type Smr11 = crate::RegValueT<Smr11_SPEC>;

impl Smr11 {
    #[inline(always)]
    pub fn md0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        smr11::Md0,
        smr11::Md0,
        Smr11_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            smr11::Md0,
            smr11::Md0,
            Smr11_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn md1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x3,
        1,
        0,
        smr11::Md1,
        smr11::Md1,
        Smr11_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x3,
            1,
            0,
            smr11::Md1,
            smr11::Md1,
            Smr11_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sis0(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        smr11::Sis0,
        smr11::Sis0,
        Smr11_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            smr11::Sis0,
            smr11::Sis0,
            Smr11_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sts(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        smr11::Sts,
        smr11::Sts,
        Smr11_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            smr11::Sts,
            smr11::Sts,
            Smr11_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ccs(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        smr11::Ccs,
        smr11::Ccs,
        Smr11_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            smr11::Ccs,
            smr11::Ccs,
            Smr11_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        smr11::Cks,
        smr11::Cks,
        Smr11_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            smr11::Cks,
            smr11::Cks,
            Smr11_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Smr11 {
    #[inline(always)]
    fn default() -> Smr11 {
        <crate::RegValueT<Smr11_SPEC> as RegisterValue<_>>::new(32)
    }
}
pub mod smr11 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Md0_SPEC;
    pub type Md0 = crate::EnumBitfieldStruct<u8, Md0_SPEC>;
    impl Md0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Md1_SPEC;
    pub type Md1 = crate::EnumBitfieldStruct<u8, Md1_SPEC>;
    impl Md1 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sis0_SPEC;
    pub type Sis0 = crate::EnumBitfieldStruct<u8, Sis0_SPEC>;
    impl Sis0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sts_SPEC;
    pub type Sts = crate::EnumBitfieldStruct<u8, Sts_SPEC>;
    impl Sts {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccs_SPEC;
    pub type Ccs = crate::EnumBitfieldStruct<u8, Ccs_SPEC>;
    impl Ccs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr10_SPEC;
impl crate::sealed::RegSpec for Scr10_SPEC {
    type DataType = u16;
}

pub type Scr10 = crate::RegValueT<Scr10_SPEC>;

impl Scr10 {
    #[inline(always)]
    pub fn dls(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        scr10::Dls,
        scr10::Dls,
        Scr10_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            scr10::Dls,
            scr10::Dls,
            Scr10_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn slc(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        scr10::Slc,
        scr10::Slc,
        Scr10_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            scr10::Slc,
            scr10::Slc,
            Scr10_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dir(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        scr10::Dir,
        scr10::Dir,
        Scr10_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            scr10::Dir,
            scr10::Dir,
            Scr10_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ptc(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        scr10::Ptc,
        scr10::Ptc,
        Scr10_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            scr10::Ptc,
            scr10::Ptc,
            Scr10_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dcp(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        scr10::Dcp,
        scr10::Dcp,
        Scr10_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            scr10::Dcp,
            scr10::Dcp,
            Scr10_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trxe(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        scr10::Trxe,
        scr10::Trxe,
        Scr10_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            scr10::Trxe,
            scr10::Trxe,
            Scr10_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Scr10 {
    #[inline(always)]
    fn default() -> Scr10 {
        <crate::RegValueT<Scr10_SPEC> as RegisterValue<_>>::new(135)
    }
}
pub mod scr10 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dls_SPEC;
    pub type Dls = crate::EnumBitfieldStruct<u8, Dls_SPEC>;
    impl Dls {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slc_SPEC;
    pub type Slc = crate::EnumBitfieldStruct<u8, Slc_SPEC>;
    impl Slc {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dir_SPEC;
    pub type Dir = crate::EnumBitfieldStruct<u8, Dir_SPEC>;
    impl Dir {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ptc_SPEC;
    pub type Ptc = crate::EnumBitfieldStruct<u8, Ptc_SPEC>;
    impl Ptc {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcp_SPEC;
    pub type Dcp = crate::EnumBitfieldStruct<u8, Dcp_SPEC>;
    impl Dcp {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trxe_SPEC;
    pub type Trxe = crate::EnumBitfieldStruct<u8, Trxe_SPEC>;
    impl Trxe {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr11_SPEC;
impl crate::sealed::RegSpec for Scr11_SPEC {
    type DataType = u16;
}

pub type Scr11 = crate::RegValueT<Scr11_SPEC>;

impl Scr11 {
    #[inline(always)]
    pub fn dls(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        scr11::Dls,
        scr11::Dls,
        Scr11_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            scr11::Dls,
            scr11::Dls,
            Scr11_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn slc(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        scr11::Slc,
        scr11::Slc,
        Scr11_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            scr11::Slc,
            scr11::Slc,
            Scr11_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dir(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        scr11::Dir,
        scr11::Dir,
        Scr11_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            scr11::Dir,
            scr11::Dir,
            Scr11_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ptc(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        scr11::Ptc,
        scr11::Ptc,
        Scr11_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            scr11::Ptc,
            scr11::Ptc,
            Scr11_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn eoc(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        scr11::Eoc,
        scr11::Eoc,
        Scr11_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            scr11::Eoc,
            scr11::Eoc,
            Scr11_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dcp(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        scr11::Dcp,
        scr11::Dcp,
        Scr11_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            scr11::Dcp,
            scr11::Dcp,
            Scr11_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trxe(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        scr11::Trxe,
        scr11::Trxe,
        Scr11_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            scr11::Trxe,
            scr11::Trxe,
            Scr11_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Scr11 {
    #[inline(always)]
    fn default() -> Scr11 {
        <crate::RegValueT<Scr11_SPEC> as RegisterValue<_>>::new(135)
    }
}
pub mod scr11 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dls_SPEC;
    pub type Dls = crate::EnumBitfieldStruct<u8, Dls_SPEC>;
    impl Dls {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slc_SPEC;
    pub type Slc = crate::EnumBitfieldStruct<u8, Slc_SPEC>;
    impl Slc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dir_SPEC;
    pub type Dir = crate::EnumBitfieldStruct<u8, Dir_SPEC>;
    impl Dir {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ptc_SPEC;
    pub type Ptc = crate::EnumBitfieldStruct<u8, Ptc_SPEC>;
    impl Ptc {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eoc_SPEC;
    pub type Eoc = crate::EnumBitfieldStruct<u8, Eoc_SPEC>;
    impl Eoc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcp_SPEC;
    pub type Dcp = crate::EnumBitfieldStruct<u8, Dcp_SPEC>;
    impl Dcp {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trxe_SPEC;
    pub type Trxe = crate::EnumBitfieldStruct<u8, Trxe_SPEC>;
    impl Trxe {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Se1_SPEC;
impl crate::sealed::RegSpec for Se1_SPEC {
    type DataType = u16;
}

pub type Se1 = crate::RegValueT<Se1_SPEC>;

impl Se1 {
    #[inline(always)]
    pub fn se(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, se1::Se, se1::Se, Se1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3,1,0,se1::Se,se1::Se,Se1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Se1 {
    #[inline(always)]
    fn default() -> Se1 {
        <crate::RegValueT<Se1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod se1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Se_SPEC;
    pub type Se = crate::EnumBitfieldStruct<u8, Se_SPEC>;
    impl Se {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ss1_SPEC;
impl crate::sealed::RegSpec for Ss1_SPEC {
    type DataType = u16;
}

pub type Ss1 = crate::RegValueT<Ss1_SPEC>;

impl Ss1 {
    #[inline(always)]
    pub fn ss(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, ss1::Ss, ss1::Ss, Ss1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,ss1::Ss,ss1::Ss,Ss1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ss1 {
    #[inline(always)]
    fn default() -> Ss1 {
        <crate::RegValueT<Ss1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ss1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ss_SPEC;
    pub type Ss = crate::EnumBitfieldStruct<u8, Ss_SPEC>;
    impl Ss {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct St1_SPEC;
impl crate::sealed::RegSpec for St1_SPEC {
    type DataType = u16;
}

pub type St1 = crate::RegValueT<St1_SPEC>;

impl St1 {
    #[inline(always)]
    pub fn st(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, st1::St, st1::St, St1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,st1::St,st1::St,St1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for St1 {
    #[inline(always)]
    fn default() -> St1 {
        <crate::RegValueT<St1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod st1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct St_SPEC;
    pub type St = crate::EnumBitfieldStruct<u8, St_SPEC>;
    impl St {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sps1_SPEC;
impl crate::sealed::RegSpec for Sps1_SPEC {
    type DataType = u16;
}

pub type Sps1 = crate::RegValueT<Sps1_SPEC>;

impl Sps1 {
    #[inline(always)]
    pub fn prs0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        sps1::Prs0,
        sps1::Prs0,
        Sps1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            sps1::Prs0,
            sps1::Prs0,
            Sps1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn prs1(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xf,
        1,
        0,
        sps1::Prs1,
        sps1::Prs1,
        Sps1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            sps1::Prs1,
            sps1::Prs1,
            Sps1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sps1 {
    #[inline(always)]
    fn default() -> Sps1 {
        <crate::RegValueT<Sps1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sps1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prs0_SPEC;
    pub type Prs0 = crate::EnumBitfieldStruct<u8, Prs0_SPEC>;
    impl Prs0 {
        pub const _0_X_0: Self = Self::new(0);

        pub const _0_X_1: Self = Self::new(1);

        pub const _0_X_2: Self = Self::new(2);

        pub const _0_X_3: Self = Self::new(3);

        pub const _0_X_4: Self = Self::new(4);

        pub const _0_X_5: Self = Self::new(5);

        pub const _0_X_6: Self = Self::new(6);

        pub const _0_X_7: Self = Self::new(7);

        pub const _0_X_8: Self = Self::new(8);

        pub const _0_X_9: Self = Self::new(9);

        pub const _0_X_A: Self = Self::new(10);

        pub const _0_X_B: Self = Self::new(11);

        pub const _0_X_C: Self = Self::new(12);

        pub const _0_X_D: Self = Self::new(13);

        pub const _0_X_E: Self = Self::new(14);

        pub const _0_X_F: Self = Self::new(15);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prs1_SPEC;
    pub type Prs1 = crate::EnumBitfieldStruct<u8, Prs1_SPEC>;
    impl Prs1 {
        pub const _0_X_0: Self = Self::new(0);

        pub const _0_X_1: Self = Self::new(1);

        pub const _0_X_2: Self = Self::new(2);

        pub const _0_X_3: Self = Self::new(3);

        pub const _0_X_4: Self = Self::new(4);

        pub const _0_X_5: Self = Self::new(5);

        pub const _0_X_6: Self = Self::new(6);

        pub const _0_X_7: Self = Self::new(7);

        pub const _0_X_8: Self = Self::new(8);

        pub const _0_X_9: Self = Self::new(9);

        pub const _0_X_A: Self = Self::new(10);

        pub const _0_X_B: Self = Self::new(11);

        pub const _0_X_C: Self = Self::new(12);

        pub const _0_X_D: Self = Self::new(13);

        pub const _0_X_E: Self = Self::new(14);

        pub const _0_X_F: Self = Self::new(15);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct So1_SPEC;
impl crate::sealed::RegSpec for So1_SPEC {
    type DataType = u16;
}

pub type So1 = crate::RegValueT<So1_SPEC>;

impl So1 {
    #[inline(always)]
    pub fn so(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, so1::So, so1::So, So1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,so1::So,so1::So,So1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cko(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, so1::Cko, so1::Cko, So1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,so1::Cko,so1::Cko,So1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for So1 {
    #[inline(always)]
    fn default() -> So1 {
        <crate::RegValueT<So1_SPEC> as RegisterValue<_>>::new(771)
    }
}
pub mod so1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct So_SPEC;
    pub type So = crate::EnumBitfieldStruct<u8, So_SPEC>;
    impl So {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cko_SPEC;
    pub type Cko = crate::EnumBitfieldStruct<u8, Cko_SPEC>;
    impl Cko {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soe1_SPEC;
impl crate::sealed::RegSpec for Soe1_SPEC {
    type DataType = u16;
}

pub type Soe1 = crate::RegValueT<Soe1_SPEC>;

impl Soe1 {
    #[inline(always)]
    pub fn soe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        soe1::Soe,
        soe1::Soe,
        Soe1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            soe1::Soe,
            soe1::Soe,
            Soe1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Soe1 {
    #[inline(always)]
    fn default() -> Soe1 {
        <crate::RegValueT<Soe1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod soe1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Soe_SPEC;
    pub type Soe = crate::EnumBitfieldStruct<u8, Soe_SPEC>;
    impl Soe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sol1_SPEC;
impl crate::sealed::RegSpec for Sol1_SPEC {
    type DataType = u16;
}

pub type Sol1 = crate::RegValueT<Sol1_SPEC>;

impl Sol1 {
    #[inline(always)]
    pub fn sol0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sol1::Sol0,
        sol1::Sol0,
        Sol1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sol1::Sol0,
            sol1::Sol0,
            Sol1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sol1 {
    #[inline(always)]
    fn default() -> Sol1 {
        <crate::RegValueT<Sol1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sol1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sol0_SPEC;
    pub type Sol0 = crate::EnumBitfieldStruct<u8, Sol0_SPEC>;
    impl Sol0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
