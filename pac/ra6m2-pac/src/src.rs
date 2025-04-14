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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:20:07 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Sampling Rate Converter"]
unsafe impl ::core::marker::Send for super::Src {}
unsafe impl ::core::marker::Sync for super::Src {}
impl super::Src {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn srcid(&self) -> &'static crate::common::Reg<self::Srcid_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Srcid_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn srcod(&self) -> &'static crate::common::Reg<self::Srcod_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Srcod_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn srcidctrl(
        &self,
    ) -> &'static crate::common::Reg<self::Srcidctrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Srcidctrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn srcodctrl(
        &self,
    ) -> &'static crate::common::Reg<self::Srcodctrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Srcodctrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[inline(always)]
    pub const fn srcctrl(
        &self,
    ) -> &'static crate::common::Reg<self::Srcctrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Srcctrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn srcstat(
        &self,
    ) -> &'static crate::common::Reg<self::Srcstat_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Srcstat_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srcid_SPEC;
impl crate::sealed::RegSpec for Srcid_SPEC {
    type DataType = u32;
}

pub type Srcid = crate::RegValueT<Srcid_SPEC>;

impl Srcid {
    #[inline(always)]
    pub fn srcid(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Srcid_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Srcid_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Srcid {
    #[inline(always)]
    fn default() -> Srcid {
        <crate::RegValueT<Srcid_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srcod_SPEC;
impl crate::sealed::RegSpec for Srcod_SPEC {
    type DataType = u32;
}

pub type Srcod = crate::RegValueT<Srcod_SPEC>;

impl Srcod {
    #[inline(always)]
    pub fn srcod(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Srcod_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Srcod_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Srcod {
    #[inline(always)]
    fn default() -> Srcod {
        <crate::RegValueT<Srcod_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srcidctrl_SPEC;
impl crate::sealed::RegSpec for Srcidctrl_SPEC {
    type DataType = u16;
}

pub type Srcidctrl = crate::RegValueT<Srcidctrl_SPEC>;

impl Srcidctrl {
    #[inline(always)]
    pub fn ied(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        srcidctrl::Ied,
        srcidctrl::Ied,
        Srcidctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            srcidctrl::Ied,
            srcidctrl::Ied,
            Srcidctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ien(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        srcidctrl::Ien,
        srcidctrl::Ien,
        Srcidctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            srcidctrl::Ien,
            srcidctrl::Ien,
            Srcidctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn iftrg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        srcidctrl::Iftrg,
        srcidctrl::Iftrg,
        Srcidctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            srcidctrl::Iftrg,
            srcidctrl::Iftrg,
            Srcidctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Srcidctrl {
    #[inline(always)]
    fn default() -> Srcidctrl {
        <crate::RegValueT<Srcidctrl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod srcidctrl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ied_SPEC;
    pub type Ied = crate::EnumBitfieldStruct<u8, Ied_SPEC>;
    impl Ied {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ien_SPEC;
    pub type Ien = crate::EnumBitfieldStruct<u8, Ien_SPEC>;
    impl Ien {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iftrg_SPEC;
    pub type Iftrg = crate::EnumBitfieldStruct<u8, Iftrg_SPEC>;
    impl Iftrg {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srcodctrl_SPEC;
impl crate::sealed::RegSpec for Srcodctrl_SPEC {
    type DataType = u16;
}

pub type Srcodctrl = crate::RegValueT<Srcodctrl_SPEC>;

impl Srcodctrl {
    #[inline(always)]
    pub fn och(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        srcodctrl::Och,
        srcodctrl::Och,
        Srcodctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            srcodctrl::Och,
            srcodctrl::Och,
            Srcodctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn oed(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        srcodctrl::Oed,
        srcodctrl::Oed,
        Srcodctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            srcodctrl::Oed,
            srcodctrl::Oed,
            Srcodctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn oen(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        srcodctrl::Oen,
        srcodctrl::Oen,
        Srcodctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            srcodctrl::Oen,
            srcodctrl::Oen,
            Srcodctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn oftrg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        srcodctrl::Oftrg,
        srcodctrl::Oftrg,
        Srcodctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            srcodctrl::Oftrg,
            srcodctrl::Oftrg,
            Srcodctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Srcodctrl {
    #[inline(always)]
    fn default() -> Srcodctrl {
        <crate::RegValueT<Srcodctrl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod srcodctrl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Och_SPEC;
    pub type Och = crate::EnumBitfieldStruct<u8, Och_SPEC>;
    impl Och {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oed_SPEC;
    pub type Oed = crate::EnumBitfieldStruct<u8, Oed_SPEC>;
    impl Oed {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oen_SPEC;
    pub type Oen = crate::EnumBitfieldStruct<u8, Oen_SPEC>;
    impl Oen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oftrg_SPEC;
    pub type Oftrg = crate::EnumBitfieldStruct<u8, Oftrg_SPEC>;
    impl Oftrg {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srcctrl_SPEC;
impl crate::sealed::RegSpec for Srcctrl_SPEC {
    type DataType = u16;
}

pub type Srcctrl = crate::RegValueT<Srcctrl_SPEC>;

impl Srcctrl {
    #[inline(always)]
    pub fn ficrae(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        srcctrl::Ficrae,
        srcctrl::Ficrae,
        Srcctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            srcctrl::Ficrae,
            srcctrl::Ficrae,
            Srcctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ceen(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        srcctrl::Ceen,
        srcctrl::Ceen,
        Srcctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            srcctrl::Ceen,
            srcctrl::Ceen,
            Srcctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn srcen(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        srcctrl::Srcen,
        srcctrl::Srcen,
        Srcctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            srcctrl::Srcen,
            srcctrl::Srcen,
            Srcctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn uden(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        srcctrl::Uden,
        srcctrl::Uden,
        Srcctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            srcctrl::Uden,
            srcctrl::Uden,
            Srcctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn oven(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        srcctrl::Oven,
        srcctrl::Oven,
        Srcctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            srcctrl::Oven,
            srcctrl::Oven,
            Srcctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fl(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        srcctrl::Fl,
        srcctrl::Fl,
        Srcctrl_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            srcctrl::Fl,
            srcctrl::Fl,
            Srcctrl_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cl(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        srcctrl::Cl,
        srcctrl::Cl,
        Srcctrl_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            srcctrl::Cl,
            srcctrl::Cl,
            Srcctrl_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ifs(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xf,
        1,
        0,
        srcctrl::Ifs,
        srcctrl::Ifs,
        Srcctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            srcctrl::Ifs,
            srcctrl::Ifs,
            Srcctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ofs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        srcctrl::Ofs,
        srcctrl::Ofs,
        Srcctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            srcctrl::Ofs,
            srcctrl::Ofs,
            Srcctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Srcctrl {
    #[inline(always)]
    fn default() -> Srcctrl {
        <crate::RegValueT<Srcctrl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod srcctrl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ficrae_SPEC;
    pub type Ficrae = crate::EnumBitfieldStruct<u8, Ficrae_SPEC>;
    impl Ficrae {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ceen_SPEC;
    pub type Ceen = crate::EnumBitfieldStruct<u8, Ceen_SPEC>;
    impl Ceen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Srcen_SPEC;
    pub type Srcen = crate::EnumBitfieldStruct<u8, Srcen_SPEC>;
    impl Srcen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uden_SPEC;
    pub type Uden = crate::EnumBitfieldStruct<u8, Uden_SPEC>;
    impl Uden {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oven_SPEC;
    pub type Oven = crate::EnumBitfieldStruct<u8, Oven_SPEC>;
    impl Oven {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fl_SPEC;
    pub type Fl = crate::EnumBitfieldStruct<u8, Fl_SPEC>;
    impl Fl {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cl_SPEC;
    pub type Cl = crate::EnumBitfieldStruct<u8, Cl_SPEC>;
    impl Cl {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ifs_SPEC;
    pub type Ifs = crate::EnumBitfieldStruct<u8, Ifs_SPEC>;
    impl Ifs {
        pub const _0000: Self = Self::new(0);

        pub const _0001: Self = Self::new(1);

        pub const _0010: Self = Self::new(2);

        pub const _0011: Self = Self::new(3);

        pub const _0100: Self = Self::new(4);

        pub const _0101: Self = Self::new(5);

        pub const _0110: Self = Self::new(6);

        pub const _0111: Self = Self::new(7);

        pub const _1000: Self = Self::new(8);

        pub const _1001: Self = Self::new(9);

        pub const _1010: Self = Self::new(10);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ofs_SPEC;
    pub type Ofs = crate::EnumBitfieldStruct<u8, Ofs_SPEC>;
    impl Ofs {
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
pub struct Srcstat_SPEC;
impl crate::sealed::RegSpec for Srcstat_SPEC {
    type DataType = u16;
}

pub type Srcstat = crate::RegValueT<Srcstat_SPEC>;

impl Srcstat {
    #[inline(always)]
    pub fn ofdn(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1f,
        1,
        0,
        srcstat::Ofdn,
        srcstat::Ofdn,
        Srcstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            11,
            0x1f,
            1,
            0,
            srcstat::Ofdn,
            srcstat::Ofdn,
            Srcstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ifdn(
        self,
    ) -> crate::common::RegisterField<
        7,
        0xf,
        1,
        0,
        srcstat::Ifdn,
        srcstat::Ifdn,
        Srcstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0xf,
            1,
            0,
            srcstat::Ifdn,
            srcstat::Ifdn,
            Srcstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cef(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        srcstat::Cef,
        srcstat::Cef,
        Srcstat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            srcstat::Cef,
            srcstat::Cef,
            Srcstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn flf(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        srcstat::Flf,
        srcstat::Flf,
        Srcstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            srcstat::Flf,
            srcstat::Flf,
            Srcstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn udf(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        srcstat::Udf,
        srcstat::Udf,
        Srcstat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            srcstat::Udf,
            srcstat::Udf,
            Srcstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        srcstat::Ovf,
        srcstat::Ovf,
        Srcstat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            srcstat::Ovf,
            srcstat::Ovf,
            Srcstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn iint(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        srcstat::Iint,
        srcstat::Iint,
        Srcstat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            srcstat::Iint,
            srcstat::Iint,
            Srcstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn oint(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        srcstat::Oint,
        srcstat::Oint,
        Srcstat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            srcstat::Oint,
            srcstat::Oint,
            Srcstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Srcstat {
    #[inline(always)]
    fn default() -> Srcstat {
        <crate::RegValueT<Srcstat_SPEC> as RegisterValue<_>>::new(2)
    }
}
pub mod srcstat {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ofdn_SPEC;
    pub type Ofdn = crate::EnumBitfieldStruct<u8, Ofdn_SPEC>;
    impl Ofdn {
        pub const OFDN: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ifdn_SPEC;
    pub type Ifdn = crate::EnumBitfieldStruct<u8, Ifdn_SPEC>;
    impl Ifdn {
        pub const IFDN: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cef_SPEC;
    pub type Cef = crate::EnumBitfieldStruct<u8, Cef_SPEC>;
    impl Cef {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flf_SPEC;
    pub type Flf = crate::EnumBitfieldStruct<u8, Flf_SPEC>;
    impl Flf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Udf_SPEC;
    pub type Udf = crate::EnumBitfieldStruct<u8, Udf_SPEC>;
    impl Udf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovf_SPEC;
    pub type Ovf = crate::EnumBitfieldStruct<u8, Ovf_SPEC>;
    impl Ovf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iint_SPEC;
    pub type Iint = crate::EnumBitfieldStruct<u8, Iint_SPEC>;
    impl Iint {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oint_SPEC;
    pub type Oint = crate::EnumBitfieldStruct<u8, Oint_SPEC>;
    impl Oint {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
