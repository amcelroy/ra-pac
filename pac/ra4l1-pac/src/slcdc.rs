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
// Generated from SVD 0.90.02, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:18:12 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Segment LCD Controller"]
unsafe impl ::core::marker::Send for super::Slcdc {}
unsafe impl ::core::marker::Sync for super::Slcdc {}
impl super::Slcdc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn lcdm0(&self) -> &'static crate::common::Reg<self::Lcdm0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lcdm0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdm1(&self) -> &'static crate::common::Reg<self::Lcdm1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lcdm1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc0(&self) -> &'static crate::common::Reg<self::Lcdc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lcdc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[inline(always)]
    pub const fn vlcd(&self) -> &'static crate::common::Reg<self::Vlcd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vlcd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3usize),
            )
        }
    }

    #[inline(always)]
    pub const fn seg(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Seg_SPEC, crate::common::RW>,
        52,
        0x1,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x100usize))
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcdm0_SPEC;
impl crate::sealed::RegSpec for Lcdm0_SPEC {
    type DataType = u8;
}

pub type Lcdm0 = crate::RegValueT<Lcdm0_SPEC>;

impl Lcdm0 {
    #[inline(always)]
    pub fn lbas(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        lcdm0::Lbas,
        lcdm0::Lbas,
        Lcdm0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            lcdm0::Lbas,
            lcdm0::Lbas,
            Lcdm0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ldty(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x7,
        1,
        0,
        lcdm0::Ldty,
        lcdm0::Ldty,
        Lcdm0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x7,
            1,
            0,
            lcdm0::Ldty,
            lcdm0::Ldty,
            Lcdm0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lwave(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        lcdm0::Lwave,
        lcdm0::Lwave,
        Lcdm0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            lcdm0::Lwave,
            lcdm0::Lwave,
            Lcdm0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mdset(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        lcdm0::Mdset,
        lcdm0::Mdset,
        Lcdm0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            lcdm0::Mdset,
            lcdm0::Mdset,
            Lcdm0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lcdm0 {
    #[inline(always)]
    fn default() -> Lcdm0 {
        <crate::RegValueT<Lcdm0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod lcdm0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lbas_SPEC;
    pub type Lbas = crate::EnumBitfieldStruct<u8, Lbas_SPEC>;
    impl Lbas {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ldty_SPEC;
    pub type Ldty = crate::EnumBitfieldStruct<u8, Ldty_SPEC>;
    impl Ldty {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _101: Self = Self::new(5);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lwave_SPEC;
    pub type Lwave = crate::EnumBitfieldStruct<u8, Lwave_SPEC>;
    impl Lwave {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mdset_SPEC;
    pub type Mdset = crate::EnumBitfieldStruct<u8, Mdset_SPEC>;
    impl Mdset {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcdm1_SPEC;
impl crate::sealed::RegSpec for Lcdm1_SPEC {
    type DataType = u8;
}

pub type Lcdm1 = crate::RegValueT<Lcdm1_SPEC>;

impl Lcdm1 {
    #[inline(always)]
    pub fn lcdvlm(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        lcdm1::Lcdvlm,
        lcdm1::Lcdvlm,
        Lcdm1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            lcdm1::Lcdvlm,
            lcdm1::Lcdvlm,
            Lcdm1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lcdsel(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        lcdm1::Lcdsel,
        lcdm1::Lcdsel,
        Lcdm1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            lcdm1::Lcdsel,
            lcdm1::Lcdsel,
            Lcdm1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn blon(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        lcdm1::Blon,
        lcdm1::Blon,
        Lcdm1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            lcdm1::Blon,
            lcdm1::Blon,
            Lcdm1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn vlcon(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        lcdm1::Vlcon,
        lcdm1::Vlcon,
        Lcdm1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            lcdm1::Vlcon,
            lcdm1::Vlcon,
            Lcdm1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn scoc(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        lcdm1::Scoc,
        lcdm1::Scoc,
        Lcdm1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            lcdm1::Scoc,
            lcdm1::Scoc,
            Lcdm1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lcdon(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        lcdm1::Lcdon,
        lcdm1::Lcdon,
        Lcdm1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            lcdm1::Lcdon,
            lcdm1::Lcdon,
            Lcdm1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lcdm1 {
    #[inline(always)]
    fn default() -> Lcdm1 {
        <crate::RegValueT<Lcdm1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod lcdm1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lcdvlm_SPEC;
    pub type Lcdvlm = crate::EnumBitfieldStruct<u8, Lcdvlm_SPEC>;
    impl Lcdvlm {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lcdsel_SPEC;
    pub type Lcdsel = crate::EnumBitfieldStruct<u8, Lcdsel_SPEC>;
    impl Lcdsel {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Blon_SPEC;
    pub type Blon = crate::EnumBitfieldStruct<u8, Blon_SPEC>;
    impl Blon {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vlcon_SPEC;
    pub type Vlcon = crate::EnumBitfieldStruct<u8, Vlcon_SPEC>;
    impl Vlcon {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scoc_SPEC;
    pub type Scoc = crate::EnumBitfieldStruct<u8, Scoc_SPEC>;
    impl Scoc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lcdon_SPEC;
    pub type Lcdon = crate::EnumBitfieldStruct<u8, Lcdon_SPEC>;
    impl Lcdon {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcdc0_SPEC;
impl crate::sealed::RegSpec for Lcdc0_SPEC {
    type DataType = u8;
}

pub type Lcdc0 = crate::RegValueT<Lcdc0_SPEC>;

impl Lcdc0 {
    #[inline(always)]
    pub fn lcdc0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        lcdc0::Lcdc0,
        lcdc0::Lcdc0,
        Lcdc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            lcdc0::Lcdc0,
            lcdc0::Lcdc0,
            Lcdc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lcdc0 {
    #[inline(always)]
    fn default() -> Lcdc0 {
        <crate::RegValueT<Lcdc0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod lcdc0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lcdc0_SPEC;
    pub type Lcdc0 = crate::EnumBitfieldStruct<u8, Lcdc0_SPEC>;
    impl Lcdc0 {
        pub const _0_X_1: Self = Self::new(1);

        pub const _0_X_2: Self = Self::new(2);

        pub const _0_X_3: Self = Self::new(3);

        pub const _0_X_4: Self = Self::new(4);

        pub const _0_X_5: Self = Self::new(5);

        pub const _0_X_6: Self = Self::new(6);

        pub const _0_X_7: Self = Self::new(7);

        pub const _0_X_8: Self = Self::new(8);

        pub const _0_X_9: Self = Self::new(9);

        pub const _0_X_11: Self = Self::new(17);

        pub const _0_X_12: Self = Self::new(18);

        pub const _0_X_13: Self = Self::new(19);

        pub const _0_X_14: Self = Self::new(20);

        pub const _0_X_15: Self = Self::new(21);

        pub const _0_X_16: Self = Self::new(22);

        pub const _0_X_17: Self = Self::new(23);

        pub const _0_X_18: Self = Self::new(24);

        pub const _0_X_19: Self = Self::new(25);

        pub const _0_X_1_A: Self = Self::new(26);

        pub const _0_X_1_B: Self = Self::new(27);

        pub const _0_X_2_B: Self = Self::new(43);

        pub const _0_X_3_B: Self = Self::new(59);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vlcd_SPEC;
impl crate::sealed::RegSpec for Vlcd_SPEC {
    type DataType = u8;
}

pub type Vlcd = crate::RegValueT<Vlcd_SPEC>;

impl Vlcd {
    #[inline(always)]
    pub fn vlcd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        vlcd::Vlcd,
        vlcd::Vlcd,
        Vlcd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            vlcd::Vlcd,
            vlcd::Vlcd,
            Vlcd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mdset2(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        vlcd::Mdset2,
        vlcd::Mdset2,
        Vlcd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            vlcd::Mdset2,
            vlcd::Mdset2,
            Vlcd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Vlcd {
    #[inline(always)]
    fn default() -> Vlcd {
        <crate::RegValueT<Vlcd_SPEC> as RegisterValue<_>>::new(4)
    }
}
pub mod vlcd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vlcd_SPEC;
    pub type Vlcd = crate::EnumBitfieldStruct<u8, Vlcd_SPEC>;
    impl Vlcd {
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

        pub const _0_X_10: Self = Self::new(16);

        pub const _0_X_11: Self = Self::new(17);

        pub const _0_X_12: Self = Self::new(18);

        pub const _0_X_13: Self = Self::new(19);

        pub const _0_X_14: Self = Self::new(20);

        pub const _0_X_15: Self = Self::new(21);

        pub const _0_X_16: Self = Self::new(22);

        pub const _0_X_17: Self = Self::new(23);

        pub const _0_X_18: Self = Self::new(24);

        pub const _0_X_19: Self = Self::new(25);

        pub const _0_X_1_A: Self = Self::new(26);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mdset2_SPEC;
    pub type Mdset2 = crate::EnumBitfieldStruct<u8, Mdset2_SPEC>;
    impl Mdset2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Seg_SPEC;
impl crate::sealed::RegSpec for Seg_SPEC {
    type DataType = u8;
}

pub type Seg = crate::RegValueT<Seg_SPEC>;

impl NoBitfieldReg<Seg_SPEC> for Seg {}
impl ::core::default::Default for Seg {
    #[inline(always)]
    fn default() -> Seg {
        <crate::RegValueT<Seg_SPEC> as RegisterValue<_>>::new(0)
    }
}
