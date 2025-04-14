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
#[doc = r"Inter-Integrated Circuit 0"]
unsafe impl ::core::marker::Send for super::Iic0 {}
unsafe impl ::core::marker::Sync for super::Iic0 {}
impl super::Iic0 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn iccr1(&self) -> &'static crate::common::Reg<self::Iccr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iccr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn iccr2(&self) -> &'static crate::common::Reg<self::Iccr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iccr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1usize),
            )
        }
    }

    #[inline(always)]
    pub const fn icmr1(&self) -> &'static crate::common::Reg<self::Icmr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icmr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[inline(always)]
    pub const fn icmr2(&self) -> &'static crate::common::Reg<self::Icmr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icmr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3usize),
            )
        }
    }

    #[inline(always)]
    pub const fn icmr3(&self) -> &'static crate::common::Reg<self::Icmr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icmr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn icfer(&self) -> &'static crate::common::Reg<self::Icfer_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icfer_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5usize),
            )
        }
    }

    #[inline(always)]
    pub const fn icser(&self) -> &'static crate::common::Reg<self::Icser_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icser_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[inline(always)]
    pub const fn icier(&self) -> &'static crate::common::Reg<self::Icier_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icier_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(7usize),
            )
        }
    }

    #[inline(always)]
    pub const fn icsr1(&self) -> &'static crate::common::Reg<self::Icsr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icsr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn icsr2(&self) -> &'static crate::common::Reg<self::Icsr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icsr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(9usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sarl(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Sarl_SPEC, crate::common::RW>,
        3,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xausize))
        }
    }

    #[inline(always)]
    pub const fn saru(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Saru_SPEC, crate::common::RW>,
        3,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xbusize))
        }
    }

    #[inline(always)]
    pub const fn icbrl(&self) -> &'static crate::common::Reg<self::Icbrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icbrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn icbrh(&self) -> &'static crate::common::Reg<self::Icbrh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icbrh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(17usize),
            )
        }
    }

    #[inline(always)]
    pub const fn icdrt(&self) -> &'static crate::common::Reg<self::Icdrt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icdrt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[inline(always)]
    pub const fn icdrr(&self) -> &'static crate::common::Reg<self::Icdrr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Icdrr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(19usize),
            )
        }
    }

    #[inline(always)]
    pub const fn icwur(&self) -> &'static crate::common::Reg<self::Icwur_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icwur_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }

    #[inline(always)]
    pub const fn icwur2(&self) -> &'static crate::common::Reg<self::Icwur2_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Icwur2_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(23usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iccr1_SPEC;
impl crate::sealed::RegSpec for Iccr1_SPEC {
    type DataType = u8;
}

pub type Iccr1 = crate::RegValueT<Iccr1_SPEC>;

impl Iccr1 {
    #[inline(always)]
    pub fn ice(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        iccr1::Ice,
        iccr1::Ice,
        Iccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            iccr1::Ice,
            iccr1::Ice,
            Iccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn iicrst(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        iccr1::Iicrst,
        iccr1::Iicrst,
        Iccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            iccr1::Iicrst,
            iccr1::Iicrst,
            Iccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn clo(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        iccr1::Clo,
        iccr1::Clo,
        Iccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            iccr1::Clo,
            iccr1::Clo,
            Iccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sowp(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        iccr1::Sowp,
        iccr1::Sowp,
        Iccr1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            iccr1::Sowp,
            iccr1::Sowp,
            Iccr1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sclo(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        iccr1::Sclo,
        iccr1::Sclo,
        Iccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            iccr1::Sclo,
            iccr1::Sclo,
            Iccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sdao(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        iccr1::Sdao,
        iccr1::Sdao,
        Iccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            iccr1::Sdao,
            iccr1::Sdao,
            Iccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn scli(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        iccr1::Scli,
        iccr1::Scli,
        Iccr1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            iccr1::Scli,
            iccr1::Scli,
            Iccr1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sdai(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        iccr1::Sdai,
        iccr1::Sdai,
        Iccr1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            iccr1::Sdai,
            iccr1::Sdai,
            Iccr1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iccr1 {
    #[inline(always)]
    fn default() -> Iccr1 {
        <crate::RegValueT<Iccr1_SPEC> as RegisterValue<_>>::new(31)
    }
}
pub mod iccr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ice_SPEC;
    pub type Ice = crate::EnumBitfieldStruct<u8, Ice_SPEC>;
    impl Ice {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicrst_SPEC;
    pub type Iicrst = crate::EnumBitfieldStruct<u8, Iicrst_SPEC>;
    impl Iicrst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clo_SPEC;
    pub type Clo = crate::EnumBitfieldStruct<u8, Clo_SPEC>;
    impl Clo {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sowp_SPEC;
    pub type Sowp = crate::EnumBitfieldStruct<u8, Sowp_SPEC>;
    impl Sowp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sclo_SPEC;
    pub type Sclo = crate::EnumBitfieldStruct<u8, Sclo_SPEC>;
    impl Sclo {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdao_SPEC;
    pub type Sdao = crate::EnumBitfieldStruct<u8, Sdao_SPEC>;
    impl Sdao {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scli_SPEC;
    pub type Scli = crate::EnumBitfieldStruct<u8, Scli_SPEC>;
    impl Scli {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdai_SPEC;
    pub type Sdai = crate::EnumBitfieldStruct<u8, Sdai_SPEC>;
    impl Sdai {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iccr2_SPEC;
impl crate::sealed::RegSpec for Iccr2_SPEC {
    type DataType = u8;
}

pub type Iccr2 = crate::RegValueT<Iccr2_SPEC>;

impl Iccr2 {
    #[inline(always)]
    pub fn bbsy(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        iccr2::Bbsy,
        iccr2::Bbsy,
        Iccr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            iccr2::Bbsy,
            iccr2::Bbsy,
            Iccr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mst(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        iccr2::Mst,
        iccr2::Mst,
        Iccr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            iccr2::Mst,
            iccr2::Mst,
            Iccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trs(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        iccr2::Trs,
        iccr2::Trs,
        Iccr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            iccr2::Trs,
            iccr2::Trs,
            Iccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sp(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        iccr2::Sp,
        iccr2::Sp,
        Iccr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            iccr2::Sp,
            iccr2::Sp,
            Iccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rs(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        iccr2::Rs,
        iccr2::Rs,
        Iccr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            iccr2::Rs,
            iccr2::Rs,
            Iccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn st(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        iccr2::St,
        iccr2::St,
        Iccr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            iccr2::St,
            iccr2::St,
            Iccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Iccr2 {
    #[inline(always)]
    fn default() -> Iccr2 {
        <crate::RegValueT<Iccr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iccr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bbsy_SPEC;
    pub type Bbsy = crate::EnumBitfieldStruct<u8, Bbsy_SPEC>;
    impl Bbsy {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mst_SPEC;
    pub type Mst = crate::EnumBitfieldStruct<u8, Mst_SPEC>;
    impl Mst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trs_SPEC;
    pub type Trs = crate::EnumBitfieldStruct<u8, Trs_SPEC>;
    impl Trs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sp_SPEC;
    pub type Sp = crate::EnumBitfieldStruct<u8, Sp_SPEC>;
    impl Sp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rs_SPEC;
    pub type Rs = crate::EnumBitfieldStruct<u8, Rs_SPEC>;
    impl Rs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
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
pub struct Icmr1_SPEC;
impl crate::sealed::RegSpec for Icmr1_SPEC {
    type DataType = u8;
}

pub type Icmr1 = crate::RegValueT<Icmr1_SPEC>;

impl Icmr1 {
    #[inline(always)]
    pub fn mtwp(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        icmr1::Mtwp,
        icmr1::Mtwp,
        Icmr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            icmr1::Mtwp,
            icmr1::Mtwp,
            Icmr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x7,
        1,
        0,
        icmr1::Cks,
        icmr1::Cks,
        Icmr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            icmr1::Cks,
            icmr1::Cks,
            Icmr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bcwp(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        icmr1::Bcwp,
        icmr1::Bcwp,
        Icmr1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            icmr1::Bcwp,
            icmr1::Bcwp,
            Icmr1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        icmr1::Bc,
        icmr1::Bc,
        Icmr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            icmr1::Bc,
            icmr1::Bc,
            Icmr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icmr1 {
    #[inline(always)]
    fn default() -> Icmr1 {
        <crate::RegValueT<Icmr1_SPEC> as RegisterValue<_>>::new(8)
    }
}
pub mod icmr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mtwp_SPEC;
    pub type Mtwp = crate::EnumBitfieldStruct<u8, Mtwp_SPEC>;
    impl Mtwp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
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
    pub struct Bcwp_SPEC;
    pub type Bcwp = crate::EnumBitfieldStruct<u8, Bcwp_SPEC>;
    impl Bcwp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bc_SPEC;
    pub type Bc = crate::EnumBitfieldStruct<u8, Bc_SPEC>;
    impl Bc {
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
pub struct Icmr2_SPEC;
impl crate::sealed::RegSpec for Icmr2_SPEC {
    type DataType = u8;
}

pub type Icmr2 = crate::RegValueT<Icmr2_SPEC>;

impl Icmr2 {
    #[inline(always)]
    pub fn dlcs(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        icmr2::Dlcs,
        icmr2::Dlcs,
        Icmr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            icmr2::Dlcs,
            icmr2::Dlcs,
            Icmr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sddl(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x7,
        1,
        0,
        icmr2::Sddl,
        icmr2::Sddl,
        Icmr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            icmr2::Sddl,
            icmr2::Sddl,
            Icmr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tmoh(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        icmr2::Tmoh,
        icmr2::Tmoh,
        Icmr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            icmr2::Tmoh,
            icmr2::Tmoh,
            Icmr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tmol(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        icmr2::Tmol,
        icmr2::Tmol,
        Icmr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            icmr2::Tmol,
            icmr2::Tmol,
            Icmr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tmos(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        icmr2::Tmos,
        icmr2::Tmos,
        Icmr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            icmr2::Tmos,
            icmr2::Tmos,
            Icmr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icmr2 {
    #[inline(always)]
    fn default() -> Icmr2 {
        <crate::RegValueT<Icmr2_SPEC> as RegisterValue<_>>::new(6)
    }
}
pub mod icmr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlcs_SPEC;
    pub type Dlcs = crate::EnumBitfieldStruct<u8, Dlcs_SPEC>;
    impl Dlcs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sddl_SPEC;
    pub type Sddl = crate::EnumBitfieldStruct<u8, Sddl_SPEC>;
    impl Sddl {
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
    pub struct Tmoh_SPEC;
    pub type Tmoh = crate::EnumBitfieldStruct<u8, Tmoh_SPEC>;
    impl Tmoh {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmol_SPEC;
    pub type Tmol = crate::EnumBitfieldStruct<u8, Tmol_SPEC>;
    impl Tmol {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmos_SPEC;
    pub type Tmos = crate::EnumBitfieldStruct<u8, Tmos_SPEC>;
    impl Tmos {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icmr3_SPEC;
impl crate::sealed::RegSpec for Icmr3_SPEC {
    type DataType = u8;
}

pub type Icmr3 = crate::RegValueT<Icmr3_SPEC>;

impl Icmr3 {
    #[inline(always)]
    pub fn smbs(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        icmr3::Smbs,
        icmr3::Smbs,
        Icmr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            icmr3::Smbs,
            icmr3::Smbs,
            Icmr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wait(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        icmr3::Wait,
        icmr3::Wait,
        Icmr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            icmr3::Wait,
            icmr3::Wait,
            Icmr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rdrfs(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        icmr3::Rdrfs,
        icmr3::Rdrfs,
        Icmr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            icmr3::Rdrfs,
            icmr3::Rdrfs,
            Icmr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ackwp(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        icmr3::Ackwp,
        icmr3::Ackwp,
        Icmr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            icmr3::Ackwp,
            icmr3::Ackwp,
            Icmr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ackbt(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        icmr3::Ackbt,
        icmr3::Ackbt,
        Icmr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            icmr3::Ackbt,
            icmr3::Ackbt,
            Icmr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ackbr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        icmr3::Ackbr,
        icmr3::Ackbr,
        Icmr3_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            icmr3::Ackbr,
            icmr3::Ackbr,
            Icmr3_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn nf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        icmr3::Nf,
        icmr3::Nf,
        Icmr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            icmr3::Nf,
            icmr3::Nf,
            Icmr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icmr3 {
    #[inline(always)]
    fn default() -> Icmr3 {
        <crate::RegValueT<Icmr3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icmr3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Smbs_SPEC;
    pub type Smbs = crate::EnumBitfieldStruct<u8, Smbs_SPEC>;
    impl Smbs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wait_SPEC;
    pub type Wait = crate::EnumBitfieldStruct<u8, Wait_SPEC>;
    impl Wait {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdrfs_SPEC;
    pub type Rdrfs = crate::EnumBitfieldStruct<u8, Rdrfs_SPEC>;
    impl Rdrfs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ackwp_SPEC;
    pub type Ackwp = crate::EnumBitfieldStruct<u8, Ackwp_SPEC>;
    impl Ackwp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ackbt_SPEC;
    pub type Ackbt = crate::EnumBitfieldStruct<u8, Ackbt_SPEC>;
    impl Ackbt {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ackbr_SPEC;
    pub type Ackbr = crate::EnumBitfieldStruct<u8, Ackbr_SPEC>;
    impl Ackbr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nf_SPEC;
    pub type Nf = crate::EnumBitfieldStruct<u8, Nf_SPEC>;
    impl Nf {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icfer_SPEC;
impl crate::sealed::RegSpec for Icfer_SPEC {
    type DataType = u8;
}

pub type Icfer = crate::RegValueT<Icfer_SPEC>;

impl Icfer {
    #[inline(always)]
    pub fn fmpe(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        icfer::Fmpe,
        icfer::Fmpe,
        Icfer_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            icfer::Fmpe,
            icfer::Fmpe,
            Icfer_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn scle(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        icfer::Scle,
        icfer::Scle,
        Icfer_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            icfer::Scle,
            icfer::Scle,
            Icfer_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn nfe(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        icfer::Nfe,
        icfer::Nfe,
        Icfer_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            icfer::Nfe,
            icfer::Nfe,
            Icfer_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn nacke(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        icfer::Nacke,
        icfer::Nacke,
        Icfer_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            icfer::Nacke,
            icfer::Nacke,
            Icfer_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sale(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        icfer::Sale,
        icfer::Sale,
        Icfer_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            icfer::Sale,
            icfer::Sale,
            Icfer_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn nale(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        icfer::Nale,
        icfer::Nale,
        Icfer_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            icfer::Nale,
            icfer::Nale,
            Icfer_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn male(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        icfer::Male,
        icfer::Male,
        Icfer_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            icfer::Male,
            icfer::Male,
            Icfer_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tmoe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        icfer::Tmoe,
        icfer::Tmoe,
        Icfer_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            icfer::Tmoe,
            icfer::Tmoe,
            Icfer_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icfer {
    #[inline(always)]
    fn default() -> Icfer {
        <crate::RegValueT<Icfer_SPEC> as RegisterValue<_>>::new(114)
    }
}
pub mod icfer {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fmpe_SPEC;
    pub type Fmpe = crate::EnumBitfieldStruct<u8, Fmpe_SPEC>;
    impl Fmpe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scle_SPEC;
    pub type Scle = crate::EnumBitfieldStruct<u8, Scle_SPEC>;
    impl Scle {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfe_SPEC;
    pub type Nfe = crate::EnumBitfieldStruct<u8, Nfe_SPEC>;
    impl Nfe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nacke_SPEC;
    pub type Nacke = crate::EnumBitfieldStruct<u8, Nacke_SPEC>;
    impl Nacke {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sale_SPEC;
    pub type Sale = crate::EnumBitfieldStruct<u8, Sale_SPEC>;
    impl Sale {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nale_SPEC;
    pub type Nale = crate::EnumBitfieldStruct<u8, Nale_SPEC>;
    impl Nale {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Male_SPEC;
    pub type Male = crate::EnumBitfieldStruct<u8, Male_SPEC>;
    impl Male {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmoe_SPEC;
    pub type Tmoe = crate::EnumBitfieldStruct<u8, Tmoe_SPEC>;
    impl Tmoe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icser_SPEC;
impl crate::sealed::RegSpec for Icser_SPEC {
    type DataType = u8;
}

pub type Icser = crate::RegValueT<Icser_SPEC>;

impl Icser {
    #[inline(always)]
    pub fn hoae(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        icser::Hoae,
        icser::Hoae,
        Icser_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            icser::Hoae,
            icser::Hoae,
            Icser_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dide(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        icser::Dide,
        icser::Dide,
        Icser_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            icser::Dide,
            icser::Dide,
            Icser_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gcae(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        icser::Gcae,
        icser::Gcae,
        Icser_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            icser::Gcae,
            icser::Gcae,
            Icser_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sar2e(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        icser::Sar2E,
        icser::Sar2E,
        Icser_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            icser::Sar2E,
            icser::Sar2E,
            Icser_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sar1e(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        icser::Sar1E,
        icser::Sar1E,
        Icser_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            icser::Sar1E,
            icser::Sar1E,
            Icser_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sar0e(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        icser::Sar0E,
        icser::Sar0E,
        Icser_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            icser::Sar0E,
            icser::Sar0E,
            Icser_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icser {
    #[inline(always)]
    fn default() -> Icser {
        <crate::RegValueT<Icser_SPEC> as RegisterValue<_>>::new(9)
    }
}
pub mod icser {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hoae_SPEC;
    pub type Hoae = crate::EnumBitfieldStruct<u8, Hoae_SPEC>;
    impl Hoae {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dide_SPEC;
    pub type Dide = crate::EnumBitfieldStruct<u8, Dide_SPEC>;
    impl Dide {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gcae_SPEC;
    pub type Gcae = crate::EnumBitfieldStruct<u8, Gcae_SPEC>;
    impl Gcae {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sar2E_SPEC;
    pub type Sar2E = crate::EnumBitfieldStruct<u8, Sar2E_SPEC>;
    impl Sar2E {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sar1E_SPEC;
    pub type Sar1E = crate::EnumBitfieldStruct<u8, Sar1E_SPEC>;
    impl Sar1E {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sar0E_SPEC;
    pub type Sar0E = crate::EnumBitfieldStruct<u8, Sar0E_SPEC>;
    impl Sar0E {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icier_SPEC;
impl crate::sealed::RegSpec for Icier_SPEC {
    type DataType = u8;
}

pub type Icier = crate::RegValueT<Icier_SPEC>;

impl Icier {
    #[inline(always)]
    pub fn tie(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        icier::Tie,
        icier::Tie,
        Icier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            icier::Tie,
            icier::Tie,
            Icier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn teie(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        icier::Teie,
        icier::Teie,
        Icier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            icier::Teie,
            icier::Teie,
            Icier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rie(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        icier::Rie,
        icier::Rie,
        Icier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            icier::Rie,
            icier::Rie,
            Icier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn nakie(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        icier::Nakie,
        icier::Nakie,
        Icier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            icier::Nakie,
            icier::Nakie,
            Icier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spie(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        icier::Spie,
        icier::Spie,
        Icier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            icier::Spie,
            icier::Spie,
            Icier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn stie(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        icier::Stie,
        icier::Stie,
        Icier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            icier::Stie,
            icier::Stie,
            Icier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn alie(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        icier::Alie,
        icier::Alie,
        Icier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            icier::Alie,
            icier::Alie,
            Icier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tmoie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        icier::Tmoie,
        icier::Tmoie,
        Icier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            icier::Tmoie,
            icier::Tmoie,
            Icier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icier {
    #[inline(always)]
    fn default() -> Icier {
        <crate::RegValueT<Icier_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icier {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tie_SPEC;
    pub type Tie = crate::EnumBitfieldStruct<u8, Tie_SPEC>;
    impl Tie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Teie_SPEC;
    pub type Teie = crate::EnumBitfieldStruct<u8, Teie_SPEC>;
    impl Teie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rie_SPEC;
    pub type Rie = crate::EnumBitfieldStruct<u8, Rie_SPEC>;
    impl Rie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nakie_SPEC;
    pub type Nakie = crate::EnumBitfieldStruct<u8, Nakie_SPEC>;
    impl Nakie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spie_SPEC;
    pub type Spie = crate::EnumBitfieldStruct<u8, Spie_SPEC>;
    impl Spie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stie_SPEC;
    pub type Stie = crate::EnumBitfieldStruct<u8, Stie_SPEC>;
    impl Stie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Alie_SPEC;
    pub type Alie = crate::EnumBitfieldStruct<u8, Alie_SPEC>;
    impl Alie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmoie_SPEC;
    pub type Tmoie = crate::EnumBitfieldStruct<u8, Tmoie_SPEC>;
    impl Tmoie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icsr1_SPEC;
impl crate::sealed::RegSpec for Icsr1_SPEC {
    type DataType = u8;
}

pub type Icsr1 = crate::RegValueT<Icsr1_SPEC>;

impl Icsr1 {
    #[inline(always)]
    pub fn hoa(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        icsr1::Hoa,
        icsr1::Hoa,
        Icsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            icsr1::Hoa,
            icsr1::Hoa,
            Icsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn did(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        icsr1::Did,
        icsr1::Did,
        Icsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            icsr1::Did,
            icsr1::Did,
            Icsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gca(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        icsr1::Gca,
        icsr1::Gca,
        Icsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            icsr1::Gca,
            icsr1::Gca,
            Icsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn aas2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        icsr1::Aas2,
        icsr1::Aas2,
        Icsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            icsr1::Aas2,
            icsr1::Aas2,
            Icsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn aas1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        icsr1::Aas1,
        icsr1::Aas1,
        Icsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            icsr1::Aas1,
            icsr1::Aas1,
            Icsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn aas0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        icsr1::Aas0,
        icsr1::Aas0,
        Icsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            icsr1::Aas0,
            icsr1::Aas0,
            Icsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icsr1 {
    #[inline(always)]
    fn default() -> Icsr1 {
        <crate::RegValueT<Icsr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icsr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hoa_SPEC;
    pub type Hoa = crate::EnumBitfieldStruct<u8, Hoa_SPEC>;
    impl Hoa {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Did_SPEC;
    pub type Did = crate::EnumBitfieldStruct<u8, Did_SPEC>;
    impl Did {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gca_SPEC;
    pub type Gca = crate::EnumBitfieldStruct<u8, Gca_SPEC>;
    impl Gca {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aas2_SPEC;
    pub type Aas2 = crate::EnumBitfieldStruct<u8, Aas2_SPEC>;
    impl Aas2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aas1_SPEC;
    pub type Aas1 = crate::EnumBitfieldStruct<u8, Aas1_SPEC>;
    impl Aas1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aas0_SPEC;
    pub type Aas0 = crate::EnumBitfieldStruct<u8, Aas0_SPEC>;
    impl Aas0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icsr2_SPEC;
impl crate::sealed::RegSpec for Icsr2_SPEC {
    type DataType = u8;
}

pub type Icsr2 = crate::RegValueT<Icsr2_SPEC>;

impl Icsr2 {
    #[inline(always)]
    pub fn tdre(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        icsr2::Tdre,
        icsr2::Tdre,
        Icsr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            icsr2::Tdre,
            icsr2::Tdre,
            Icsr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tend(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        icsr2::Tend,
        icsr2::Tend,
        Icsr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            icsr2::Tend,
            icsr2::Tend,
            Icsr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rdrf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        icsr2::Rdrf,
        icsr2::Rdrf,
        Icsr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            icsr2::Rdrf,
            icsr2::Rdrf,
            Icsr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn nackf(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        icsr2::Nackf,
        icsr2::Nackf,
        Icsr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            icsr2::Nackf,
            icsr2::Nackf,
            Icsr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn stop(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        icsr2::Stop,
        icsr2::Stop,
        Icsr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            icsr2::Stop,
            icsr2::Stop,
            Icsr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn start(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        icsr2::Start,
        icsr2::Start,
        Icsr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            icsr2::Start,
            icsr2::Start,
            Icsr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn al(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        icsr2::Al,
        icsr2::Al,
        Icsr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            icsr2::Al,
            icsr2::Al,
            Icsr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tmof(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        icsr2::Tmof,
        icsr2::Tmof,
        Icsr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            icsr2::Tmof,
            icsr2::Tmof,
            Icsr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icsr2 {
    #[inline(always)]
    fn default() -> Icsr2 {
        <crate::RegValueT<Icsr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icsr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdre_SPEC;
    pub type Tdre = crate::EnumBitfieldStruct<u8, Tdre_SPEC>;
    impl Tdre {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tend_SPEC;
    pub type Tend = crate::EnumBitfieldStruct<u8, Tend_SPEC>;
    impl Tend {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdrf_SPEC;
    pub type Rdrf = crate::EnumBitfieldStruct<u8, Rdrf_SPEC>;
    impl Rdrf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nackf_SPEC;
    pub type Nackf = crate::EnumBitfieldStruct<u8, Nackf_SPEC>;
    impl Nackf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stop_SPEC;
    pub type Stop = crate::EnumBitfieldStruct<u8, Stop_SPEC>;
    impl Stop {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Start_SPEC;
    pub type Start = crate::EnumBitfieldStruct<u8, Start_SPEC>;
    impl Start {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Al_SPEC;
    pub type Al = crate::EnumBitfieldStruct<u8, Al_SPEC>;
    impl Al {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmof_SPEC;
    pub type Tmof = crate::EnumBitfieldStruct<u8, Tmof_SPEC>;
    impl Tmof {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sarl_SPEC;
impl crate::sealed::RegSpec for Sarl_SPEC {
    type DataType = u8;
}

pub type Sarl = crate::RegValueT<Sarl_SPEC>;

impl Sarl {
    #[inline(always)]
    pub fn sva(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Sarl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Sarl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sarl {
    #[inline(always)]
    fn default() -> Sarl {
        <crate::RegValueT<Sarl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Saru_SPEC;
impl crate::sealed::RegSpec for Saru_SPEC {
    type DataType = u8;
}

pub type Saru = crate::RegValueT<Saru_SPEC>;

impl Saru {
    #[inline(always)]
    pub fn sva9(self) -> crate::common::RegisterFieldBool<2, 1, 0, Saru_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Saru_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn sva8(self) -> crate::common::RegisterFieldBool<1, 1, 0, Saru_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Saru_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn fs(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, saru::Fs, saru::Fs, Saru_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,saru::Fs,saru::Fs,Saru_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Saru {
    #[inline(always)]
    fn default() -> Saru {
        <crate::RegValueT<Saru_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod saru {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fs_SPEC;
    pub type Fs = crate::EnumBitfieldStruct<u8, Fs_SPEC>;
    impl Fs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icbrl_SPEC;
impl crate::sealed::RegSpec for Icbrl_SPEC {
    type DataType = u8;
}

pub type Icbrl = crate::RegValueT<Icbrl_SPEC>;

impl Icbrl {
    #[inline(always)]
    pub fn brl(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Icbrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Icbrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Icbrl {
    #[inline(always)]
    fn default() -> Icbrl {
        <crate::RegValueT<Icbrl_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icbrh_SPEC;
impl crate::sealed::RegSpec for Icbrh_SPEC {
    type DataType = u8;
}

pub type Icbrh = crate::RegValueT<Icbrh_SPEC>;

impl Icbrh {
    #[inline(always)]
    pub fn brh(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Icbrh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Icbrh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Icbrh {
    #[inline(always)]
    fn default() -> Icbrh {
        <crate::RegValueT<Icbrh_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icdrt_SPEC;
impl crate::sealed::RegSpec for Icdrt_SPEC {
    type DataType = u8;
}

pub type Icdrt = crate::RegValueT<Icdrt_SPEC>;

impl Icdrt {
    #[inline(always)]
    pub fn icdrt(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Icdrt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Icdrt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Icdrt {
    #[inline(always)]
    fn default() -> Icdrt {
        <crate::RegValueT<Icdrt_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icdrr_SPEC;
impl crate::sealed::RegSpec for Icdrr_SPEC {
    type DataType = u8;
}

pub type Icdrr = crate::RegValueT<Icdrr_SPEC>;

impl Icdrr {
    #[inline(always)]
    pub fn icdrr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Icdrr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Icdrr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Icdrr {
    #[inline(always)]
    fn default() -> Icdrr {
        <crate::RegValueT<Icdrr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icwur_SPEC;
impl crate::sealed::RegSpec for Icwur_SPEC {
    type DataType = u8;
}

pub type Icwur = crate::RegValueT<Icwur_SPEC>;

impl Icwur {
    #[inline(always)]
    pub fn wue(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        icwur::Wue,
        icwur::Wue,
        Icwur_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            icwur::Wue,
            icwur::Wue,
            Icwur_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wuie(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        icwur::Wuie,
        icwur::Wuie,
        Icwur_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            icwur::Wuie,
            icwur::Wuie,
            Icwur_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wuf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        icwur::Wuf,
        icwur::Wuf,
        Icwur_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            icwur::Wuf,
            icwur::Wuf,
            Icwur_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wuack(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        icwur::Wuack,
        icwur::Wuack,
        Icwur_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            icwur::Wuack,
            icwur::Wuack,
            Icwur_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wuafa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        icwur::Wuafa,
        icwur::Wuafa,
        Icwur_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            icwur::Wuafa,
            icwur::Wuafa,
            Icwur_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icwur {
    #[inline(always)]
    fn default() -> Icwur {
        <crate::RegValueT<Icwur_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icwur {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wue_SPEC;
    pub type Wue = crate::EnumBitfieldStruct<u8, Wue_SPEC>;
    impl Wue {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wuie_SPEC;
    pub type Wuie = crate::EnumBitfieldStruct<u8, Wuie_SPEC>;
    impl Wuie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wuf_SPEC;
    pub type Wuf = crate::EnumBitfieldStruct<u8, Wuf_SPEC>;
    impl Wuf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wuack_SPEC;
    pub type Wuack = crate::EnumBitfieldStruct<u8, Wuack_SPEC>;
    impl Wuack {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wuafa_SPEC;
    pub type Wuafa = crate::EnumBitfieldStruct<u8, Wuafa_SPEC>;
    impl Wuafa {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icwur2_SPEC;
impl crate::sealed::RegSpec for Icwur2_SPEC {
    type DataType = u8;
}

pub type Icwur2 = crate::RegValueT<Icwur2_SPEC>;

impl Icwur2 {
    #[inline(always)]
    pub fn wusyf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        icwur2::Wusyf,
        icwur2::Wusyf,
        Icwur2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            icwur2::Wusyf,
            icwur2::Wusyf,
            Icwur2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wuasyf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        icwur2::Wuasyf,
        icwur2::Wuasyf,
        Icwur2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            icwur2::Wuasyf,
            icwur2::Wuasyf,
            Icwur2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wusen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        icwur2::Wusen,
        icwur2::Wusen,
        Icwur2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            icwur2::Wusen,
            icwur2::Wusen,
            Icwur2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Icwur2 {
    #[inline(always)]
    fn default() -> Icwur2 {
        <crate::RegValueT<Icwur2_SPEC> as RegisterValue<_>>::new(3)
    }
}
pub mod icwur2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wusyf_SPEC;
    pub type Wusyf = crate::EnumBitfieldStruct<u8, Wusyf_SPEC>;
    impl Wusyf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wuasyf_SPEC;
    pub type Wuasyf = crate::EnumBitfieldStruct<u8, Wuasyf_SPEC>;
    impl Wuasyf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wusen_SPEC;
    pub type Wusen = crate::EnumBitfieldStruct<u8, Wusen_SPEC>;
    impl Wusen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
