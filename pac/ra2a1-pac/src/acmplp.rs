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
// Generated from SVD 1.1, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:53 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Low-Power Analog Comparator"]
unsafe impl ::core::marker::Send for super::Acmplp {}
unsafe impl ::core::marker::Sync for super::Acmplp {}
impl super::Acmplp {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn compmdr(
        &self,
    ) -> &'static crate::common::Reg<self::Compmdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Compmdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn compfir(
        &self,
    ) -> &'static crate::common::Reg<self::Compfir_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Compfir_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1usize),
            )
        }
    }

    #[inline(always)]
    pub const fn compocr(
        &self,
    ) -> &'static crate::common::Reg<self::Compocr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Compocr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[inline(always)]
    pub const fn compsel0(
        &self,
    ) -> &'static crate::common::Reg<self::Compsel0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Compsel0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn compsel1(
        &self,
    ) -> &'static crate::common::Reg<self::Compsel1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Compsel1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Compmdr_SPEC;
impl crate::sealed::RegSpec for Compmdr_SPEC {
    type DataType = u8;
}

pub type Compmdr = crate::RegValueT<Compmdr_SPEC>;

impl Compmdr {
    #[inline(always)]
    pub fn c1mon(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        compmdr::C1Mon,
        compmdr::C1Mon,
        Compmdr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            compmdr::C1Mon,
            compmdr::C1Mon,
            Compmdr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn c1vrf(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        compmdr::C1Vrf,
        compmdr::C1Vrf,
        Compmdr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            compmdr::C1Vrf,
            compmdr::C1Vrf,
            Compmdr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn c1wde(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        compmdr::C1Wde,
        compmdr::C1Wde,
        Compmdr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            compmdr::C1Wde,
            compmdr::C1Wde,
            Compmdr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn c1enb(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        compmdr::C1Enb,
        compmdr::C1Enb,
        Compmdr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            compmdr::C1Enb,
            compmdr::C1Enb,
            Compmdr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn c0mon(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        compmdr::C0Mon,
        compmdr::C0Mon,
        Compmdr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            compmdr::C0Mon,
            compmdr::C0Mon,
            Compmdr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn c0vrf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        compmdr::C0Vrf,
        compmdr::C0Vrf,
        Compmdr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            compmdr::C0Vrf,
            compmdr::C0Vrf,
            Compmdr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn c0wde(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        compmdr::C0Wde,
        compmdr::C0Wde,
        Compmdr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            compmdr::C0Wde,
            compmdr::C0Wde,
            Compmdr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn c0enb(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        compmdr::C0Enb,
        compmdr::C0Enb,
        Compmdr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            compmdr::C0Enb,
            compmdr::C0Enb,
            Compmdr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Compmdr {
    #[inline(always)]
    fn default() -> Compmdr {
        <crate::RegValueT<Compmdr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod compmdr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct C1Mon_SPEC;
    pub type C1Mon = crate::EnumBitfieldStruct<u8, C1Mon_SPEC>;
    impl C1Mon {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct C1Vrf_SPEC;
    pub type C1Vrf = crate::EnumBitfieldStruct<u8, C1Vrf_SPEC>;
    impl C1Vrf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct C1Wde_SPEC;
    pub type C1Wde = crate::EnumBitfieldStruct<u8, C1Wde_SPEC>;
    impl C1Wde {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct C1Enb_SPEC;
    pub type C1Enb = crate::EnumBitfieldStruct<u8, C1Enb_SPEC>;
    impl C1Enb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct C0Mon_SPEC;
    pub type C0Mon = crate::EnumBitfieldStruct<u8, C0Mon_SPEC>;
    impl C0Mon {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct C0Vrf_SPEC;
    pub type C0Vrf = crate::EnumBitfieldStruct<u8, C0Vrf_SPEC>;
    impl C0Vrf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct C0Wde_SPEC;
    pub type C0Wde = crate::EnumBitfieldStruct<u8, C0Wde_SPEC>;
    impl C0Wde {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct C0Enb_SPEC;
    pub type C0Enb = crate::EnumBitfieldStruct<u8, C0Enb_SPEC>;
    impl C0Enb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Compfir_SPEC;
impl crate::sealed::RegSpec for Compfir_SPEC {
    type DataType = u8;
}

pub type Compfir = crate::RegValueT<Compfir_SPEC>;

impl Compfir {
    #[inline(always)]
    pub fn c1edg(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        compfir::C1Edg,
        compfir::C1Edg,
        Compfir_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            compfir::C1Edg,
            compfir::C1Edg,
            Compfir_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn c1epo(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        compfir::C1Epo,
        compfir::C1Epo,
        Compfir_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            compfir::C1Epo,
            compfir::C1Epo,
            Compfir_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn c1fck(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        compfir::C1Fck,
        compfir::C1Fck,
        Compfir_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            compfir::C1Fck,
            compfir::C1Fck,
            Compfir_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn c0edg(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        compfir::C0Edg,
        compfir::C0Edg,
        Compfir_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            compfir::C0Edg,
            compfir::C0Edg,
            Compfir_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn c0epo(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        compfir::C0Epo,
        compfir::C0Epo,
        Compfir_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            compfir::C0Epo,
            compfir::C0Epo,
            Compfir_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn c0fck(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        compfir::C0Fck,
        compfir::C0Fck,
        Compfir_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            compfir::C0Fck,
            compfir::C0Fck,
            Compfir_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Compfir {
    #[inline(always)]
    fn default() -> Compfir {
        <crate::RegValueT<Compfir_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod compfir {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct C1Edg_SPEC;
    pub type C1Edg = crate::EnumBitfieldStruct<u8, C1Edg_SPEC>;
    impl C1Edg {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct C1Epo_SPEC;
    pub type C1Epo = crate::EnumBitfieldStruct<u8, C1Epo_SPEC>;
    impl C1Epo {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct C1Fck_SPEC;
    pub type C1Fck = crate::EnumBitfieldStruct<u8, C1Fck_SPEC>;
    impl C1Fck {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct C0Edg_SPEC;
    pub type C0Edg = crate::EnumBitfieldStruct<u8, C0Edg_SPEC>;
    impl C0Edg {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct C0Epo_SPEC;
    pub type C0Epo = crate::EnumBitfieldStruct<u8, C0Epo_SPEC>;
    impl C0Epo {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct C0Fck_SPEC;
    pub type C0Fck = crate::EnumBitfieldStruct<u8, C0Fck_SPEC>;
    impl C0Fck {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Compocr_SPEC;
impl crate::sealed::RegSpec for Compocr_SPEC {
    type DataType = u8;
}

pub type Compocr = crate::RegValueT<Compocr_SPEC>;

impl Compocr {
    #[inline(always)]
    pub fn spdmd(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        compocr::Spdmd,
        compocr::Spdmd,
        Compocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            compocr::Spdmd,
            compocr::Spdmd,
            Compocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn c1op(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        compocr::C1Op,
        compocr::C1Op,
        Compocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            compocr::C1Op,
            compocr::C1Op,
            Compocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn c1oe(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        compocr::C1Oe,
        compocr::C1Oe,
        Compocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            compocr::C1Oe,
            compocr::C1Oe,
            Compocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn c0op(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        compocr::C0Op,
        compocr::C0Op,
        Compocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            compocr::C0Op,
            compocr::C0Op,
            Compocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn c0oe(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        compocr::C0Oe,
        compocr::C0Oe,
        Compocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            compocr::C0Oe,
            compocr::C0Oe,
            Compocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Compocr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Compocr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Compocr {
    #[inline(always)]
    fn default() -> Compocr {
        <crate::RegValueT<Compocr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod compocr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spdmd_SPEC;
    pub type Spdmd = crate::EnumBitfieldStruct<u8, Spdmd_SPEC>;
    impl Spdmd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct C1Op_SPEC;
    pub type C1Op = crate::EnumBitfieldStruct<u8, C1Op_SPEC>;
    impl C1Op {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct C1Oe_SPEC;
    pub type C1Oe = crate::EnumBitfieldStruct<u8, C1Oe_SPEC>;
    impl C1Oe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct C0Op_SPEC;
    pub type C0Op = crate::EnumBitfieldStruct<u8, C0Op_SPEC>;
    impl C0Op {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct C0Oe_SPEC;
    pub type C0Oe = crate::EnumBitfieldStruct<u8, C0Oe_SPEC>;
    impl C0Oe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Compsel0_SPEC;
impl crate::sealed::RegSpec for Compsel0_SPEC {
    type DataType = u8;
}

pub type Compsel0 = crate::RegValueT<Compsel0_SPEC>;

impl Compsel0 {
    #[inline(always)]
    pub fn cmpsel54(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        compsel0::Cmpsel54,
        compsel0::Cmpsel54,
        Compsel0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            compsel0::Cmpsel54,
            compsel0::Cmpsel54,
            Compsel0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, Compsel0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,Compsel0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmpsel10(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        compsel0::Cmpsel10,
        compsel0::Cmpsel10,
        Compsel0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            compsel0::Cmpsel10,
            compsel0::Cmpsel10,
            Compsel0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Compsel0 {
    #[inline(always)]
    fn default() -> Compsel0 {
        <crate::RegValueT<Compsel0_SPEC> as RegisterValue<_>>::new(17)
    }
}
pub mod compsel0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpsel54_SPEC;
    pub type Cmpsel54 = crate::EnumBitfieldStruct<u8, Cmpsel54_SPEC>;
    impl Cmpsel54 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpsel10_SPEC;
    pub type Cmpsel10 = crate::EnumBitfieldStruct<u8, Cmpsel10_SPEC>;
    impl Cmpsel10 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Compsel1_SPEC;
impl crate::sealed::RegSpec for Compsel1_SPEC {
    type DataType = u8;
}

pub type Compsel1 = crate::RegValueT<Compsel1_SPEC>;

impl Compsel1 {
    #[inline(always)]
    pub fn c1vrf2(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        compsel1::C1Vrf2,
        compsel1::C1Vrf2,
        Compsel1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            compsel1::C1Vrf2,
            compsel1::C1Vrf2,
            Compsel1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn crvs54(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        compsel1::Crvs54,
        compsel1::Crvs54,
        Compsel1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            compsel1::Crvs54,
            compsel1::Crvs54,
            Compsel1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, Compsel1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,Compsel1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn crvs10(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        compsel1::Crvs10,
        compsel1::Crvs10,
        Compsel1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            compsel1::Crvs10,
            compsel1::Crvs10,
            Compsel1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Compsel1 {
    #[inline(always)]
    fn default() -> Compsel1 {
        <crate::RegValueT<Compsel1_SPEC> as RegisterValue<_>>::new(145)
    }
}
pub mod compsel1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct C1Vrf2_SPEC;
    pub type C1Vrf2 = crate::EnumBitfieldStruct<u8, C1Vrf2_SPEC>;
    impl C1Vrf2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Crvs54_SPEC;
    pub type Crvs54 = crate::EnumBitfieldStruct<u8, Crvs54_SPEC>;
    impl Crvs54 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Crvs10_SPEC;
    pub type Crvs10 = crate::EnumBitfieldStruct<u8, Crvs10_SPEC>;
    impl Crvs10 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);
    }
}
