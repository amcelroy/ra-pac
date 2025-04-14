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
#[doc = r"Realtime Clock"]
unsafe impl ::core::marker::Send for super::Rtc {}
unsafe impl ::core::marker::Sync for super::Rtc {}
impl super::Rtc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn r64cnt(&self) -> &'static crate::common::Reg<self::R64Cnt_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::R64Cnt_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rseccnt(
        &self,
    ) -> &'static crate::common::Reg<self::Rseccnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rseccnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bcnt0(&self) -> &'static crate::common::Reg<self::Bcnt0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcnt0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rmincnt(
        &self,
    ) -> &'static crate::common::Reg<self::Rmincnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rmincnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bcnt1(&self) -> &'static crate::common::Reg<self::Bcnt1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcnt1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rhrcnt(
        &self,
    ) -> &'static crate::common::Reg<self::Rhrcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rhrcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bcnt2(&self) -> &'static crate::common::Reg<self::Bcnt2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcnt2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rwkcnt(
        &self,
    ) -> &'static crate::common::Reg<self::Rwkcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rwkcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bcnt3(&self) -> &'static crate::common::Reg<self::Bcnt3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcnt3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rdaycnt(
        &self,
    ) -> &'static crate::common::Reg<self::Rdaycnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rdaycnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rmoncnt(
        &self,
    ) -> &'static crate::common::Reg<self::Rmoncnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rmoncnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ryrcnt(
        &self,
    ) -> &'static crate::common::Reg<self::Ryrcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ryrcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rsecar(
        &self,
    ) -> &'static crate::common::Reg<self::Rsecar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rsecar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bcnt0ar(
        &self,
    ) -> &'static crate::common::Reg<self::Bcnt0Ar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcnt0Ar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rminar(
        &self,
    ) -> &'static crate::common::Reg<self::Rminar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rminar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bcnt1ar(
        &self,
    ) -> &'static crate::common::Reg<self::Bcnt1Ar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcnt1Ar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rhrar(&self) -> &'static crate::common::Reg<self::Rhrar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rhrar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bcnt2ar(
        &self,
    ) -> &'static crate::common::Reg<self::Bcnt2Ar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcnt2Ar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rwkar(&self) -> &'static crate::common::Reg<self::Rwkar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rwkar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bcnt3ar(
        &self,
    ) -> &'static crate::common::Reg<self::Bcnt3Ar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcnt3Ar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rdayar(
        &self,
    ) -> &'static crate::common::Reg<self::Rdayar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rdayar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bcnt0aer(
        &self,
    ) -> &'static crate::common::Reg<self::Bcnt0Aer_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcnt0Aer_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rmonar(
        &self,
    ) -> &'static crate::common::Reg<self::Rmonar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rmonar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(26usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bcnt1aer(
        &self,
    ) -> &'static crate::common::Reg<self::Bcnt1Aer_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcnt1Aer_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(26usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ryrar(&self) -> &'static crate::common::Reg<self::Ryrar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ryrar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bcnt2aer(
        &self,
    ) -> &'static crate::common::Reg<self::Bcnt2Aer_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcnt2Aer_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ryraren(
        &self,
    ) -> &'static crate::common::Reg<self::Ryraren_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ryraren_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(30usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bcnt3aer(
        &self,
    ) -> &'static crate::common::Reg<self::Bcnt3Aer_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcnt3Aer_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(30usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rcr1(&self) -> &'static crate::common::Reg<self::Rcr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rcr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(34usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rcr2(&self) -> &'static crate::common::Reg<self::Rcr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rcr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rcr4(&self) -> &'static crate::common::Reg<self::Rcr4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rcr4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rfrh(&self) -> &'static crate::common::Reg<self::Rfrh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rfrh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(42usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rfrl(&self) -> &'static crate::common::Reg<self::Rfrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rfrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn radj(&self) -> &'static crate::common::Reg<self::Radj_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Radj_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(46usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct R64Cnt_SPEC;
impl crate::sealed::RegSpec for R64Cnt_SPEC {
    type DataType = u8;
}

pub type R64Cnt = crate::RegValueT<R64Cnt_SPEC>;

impl R64Cnt {
    #[inline(always)]
    pub fn f1hz(self) -> crate::common::RegisterFieldBool<6, 1, 0, R64Cnt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, R64Cnt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn f2hz(self) -> crate::common::RegisterFieldBool<5, 1, 0, R64Cnt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, R64Cnt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn f4hz(self) -> crate::common::RegisterFieldBool<4, 1, 0, R64Cnt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, R64Cnt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn f8hz(self) -> crate::common::RegisterFieldBool<3, 1, 0, R64Cnt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, R64Cnt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn f16hz(self) -> crate::common::RegisterFieldBool<2, 1, 0, R64Cnt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, R64Cnt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn f32hz(self) -> crate::common::RegisterFieldBool<1, 1, 0, R64Cnt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, R64Cnt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn f64hz(self) -> crate::common::RegisterFieldBool<0, 1, 0, R64Cnt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, R64Cnt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for R64Cnt {
    #[inline(always)]
    fn default() -> R64Cnt {
        <crate::RegValueT<R64Cnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rseccnt_SPEC;
impl crate::sealed::RegSpec for Rseccnt_SPEC {
    type DataType = u8;
}

pub type Rseccnt = crate::RegValueT<Rseccnt_SPEC>;

impl Rseccnt {
    #[inline(always)]
    pub fn sec10(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Rseccnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Rseccnt_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sec1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rseccnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rseccnt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rseccnt {
    #[inline(always)]
    fn default() -> Rseccnt {
        <crate::RegValueT<Rseccnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt0_SPEC;
impl crate::sealed::RegSpec for Bcnt0_SPEC {
    type DataType = u8;
}

pub type Bcnt0 = crate::RegValueT<Bcnt0_SPEC>;

impl Bcnt0 {
    #[inline(always)]
    pub fn bcnt0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Bcnt0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Bcnt0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bcnt0 {
    #[inline(always)]
    fn default() -> Bcnt0 {
        <crate::RegValueT<Bcnt0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rmincnt_SPEC;
impl crate::sealed::RegSpec for Rmincnt_SPEC {
    type DataType = u8;
}

pub type Rmincnt = crate::RegValueT<Rmincnt_SPEC>;

impl Rmincnt {
    #[inline(always)]
    pub fn min10(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Rmincnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Rmincnt_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn min1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rmincnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rmincnt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rmincnt {
    #[inline(always)]
    fn default() -> Rmincnt {
        <crate::RegValueT<Rmincnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt1_SPEC;
impl crate::sealed::RegSpec for Bcnt1_SPEC {
    type DataType = u8;
}

pub type Bcnt1 = crate::RegValueT<Bcnt1_SPEC>;

impl Bcnt1 {
    #[inline(always)]
    pub fn bcnt1(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Bcnt1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Bcnt1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bcnt1 {
    #[inline(always)]
    fn default() -> Bcnt1 {
        <crate::RegValueT<Bcnt1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rhrcnt_SPEC;
impl crate::sealed::RegSpec for Rhrcnt_SPEC {
    type DataType = u8;
}

pub type Rhrcnt = crate::RegValueT<Rhrcnt_SPEC>;

impl Rhrcnt {
    #[inline(always)]
    pub fn pm(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        rhrcnt::Pm,
        rhrcnt::Pm,
        Rhrcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            rhrcnt::Pm,
            rhrcnt::Pm,
            Rhrcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn hr10(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Rhrcnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Rhrcnt_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn hr1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rhrcnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rhrcnt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rhrcnt {
    #[inline(always)]
    fn default() -> Rhrcnt {
        <crate::RegValueT<Rhrcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rhrcnt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pm_SPEC;
    pub type Pm = crate::EnumBitfieldStruct<u8, Pm_SPEC>;
    impl Pm {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt2_SPEC;
impl crate::sealed::RegSpec for Bcnt2_SPEC {
    type DataType = u8;
}

pub type Bcnt2 = crate::RegValueT<Bcnt2_SPEC>;

impl Bcnt2 {
    #[inline(always)]
    pub fn bcnt2(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Bcnt2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Bcnt2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bcnt2 {
    #[inline(always)]
    fn default() -> Bcnt2 {
        <crate::RegValueT<Bcnt2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rwkcnt_SPEC;
impl crate::sealed::RegSpec for Rwkcnt_SPEC {
    type DataType = u8;
}

pub type Rwkcnt = crate::RegValueT<Rwkcnt_SPEC>;

impl Rwkcnt {
    #[inline(always)]
    pub fn dayw(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        rwkcnt::Dayw,
        rwkcnt::Dayw,
        Rwkcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            rwkcnt::Dayw,
            rwkcnt::Dayw,
            Rwkcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rwkcnt {
    #[inline(always)]
    fn default() -> Rwkcnt {
        <crate::RegValueT<Rwkcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rwkcnt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dayw_SPEC;
    pub type Dayw = crate::EnumBitfieldStruct<u8, Dayw_SPEC>;
    impl Dayw {
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
pub struct Bcnt3_SPEC;
impl crate::sealed::RegSpec for Bcnt3_SPEC {
    type DataType = u8;
}

pub type Bcnt3 = crate::RegValueT<Bcnt3_SPEC>;

impl Bcnt3 {
    #[inline(always)]
    pub fn bcnt3(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Bcnt3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Bcnt3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bcnt3 {
    #[inline(always)]
    fn default() -> Bcnt3 {
        <crate::RegValueT<Bcnt3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdaycnt_SPEC;
impl crate::sealed::RegSpec for Rdaycnt_SPEC {
    type DataType = u8;
}

pub type Rdaycnt = crate::RegValueT<Rdaycnt_SPEC>;

impl Rdaycnt {
    #[inline(always)]
    pub fn date10(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Rdaycnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Rdaycnt_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn date1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rdaycnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rdaycnt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rdaycnt {
    #[inline(always)]
    fn default() -> Rdaycnt {
        <crate::RegValueT<Rdaycnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rmoncnt_SPEC;
impl crate::sealed::RegSpec for Rmoncnt_SPEC {
    type DataType = u8;
}

pub type Rmoncnt = crate::RegValueT<Rmoncnt_SPEC>;

impl Rmoncnt {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, Rmoncnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,Rmoncnt_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mon10(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Rmoncnt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Rmoncnt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn mon1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rmoncnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rmoncnt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rmoncnt {
    #[inline(always)]
    fn default() -> Rmoncnt {
        <crate::RegValueT<Rmoncnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ryrcnt_SPEC;
impl crate::sealed::RegSpec for Ryrcnt_SPEC {
    type DataType = u16;
}

pub type Ryrcnt = crate::RegValueT<Ryrcnt_SPEC>;

impl Ryrcnt {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ryrcnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ryrcnt_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn yr10(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Ryrcnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Ryrcnt_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn yr1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Ryrcnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Ryrcnt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ryrcnt {
    #[inline(always)]
    fn default() -> Ryrcnt {
        <crate::RegValueT<Ryrcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsecar_SPEC;
impl crate::sealed::RegSpec for Rsecar_SPEC {
    type DataType = u8;
}

pub type Rsecar = crate::RegValueT<Rsecar_SPEC>;

impl Rsecar {
    #[inline(always)]
    pub fn enb(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        rsecar::Enb,
        rsecar::Enb,
        Rsecar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            rsecar::Enb,
            rsecar::Enb,
            Rsecar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sec10(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Rsecar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Rsecar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sec1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rsecar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rsecar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rsecar {
    #[inline(always)]
    fn default() -> Rsecar {
        <crate::RegValueT<Rsecar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rsecar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enb_SPEC;
    pub type Enb = crate::EnumBitfieldStruct<u8, Enb_SPEC>;
    impl Enb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt0Ar_SPEC;
impl crate::sealed::RegSpec for Bcnt0Ar_SPEC {
    type DataType = u8;
}

pub type Bcnt0Ar = crate::RegValueT<Bcnt0Ar_SPEC>;

impl Bcnt0Ar {
    #[inline(always)]
    pub fn bcnt0ar(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Bcnt0Ar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Bcnt0Ar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bcnt0Ar {
    #[inline(always)]
    fn default() -> Bcnt0Ar {
        <crate::RegValueT<Bcnt0Ar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rminar_SPEC;
impl crate::sealed::RegSpec for Rminar_SPEC {
    type DataType = u8;
}

pub type Rminar = crate::RegValueT<Rminar_SPEC>;

impl Rminar {
    #[inline(always)]
    pub fn enb(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        rminar::Enb,
        rminar::Enb,
        Rminar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            rminar::Enb,
            rminar::Enb,
            Rminar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn min10(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Rminar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Rminar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn min1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rminar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rminar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rminar {
    #[inline(always)]
    fn default() -> Rminar {
        <crate::RegValueT<Rminar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rminar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enb_SPEC;
    pub type Enb = crate::EnumBitfieldStruct<u8, Enb_SPEC>;
    impl Enb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt1Ar_SPEC;
impl crate::sealed::RegSpec for Bcnt1Ar_SPEC {
    type DataType = u8;
}

pub type Bcnt1Ar = crate::RegValueT<Bcnt1Ar_SPEC>;

impl Bcnt1Ar {
    #[inline(always)]
    pub fn bcnt1ar(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Bcnt1Ar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Bcnt1Ar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bcnt1Ar {
    #[inline(always)]
    fn default() -> Bcnt1Ar {
        <crate::RegValueT<Bcnt1Ar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rhrar_SPEC;
impl crate::sealed::RegSpec for Rhrar_SPEC {
    type DataType = u8;
}

pub type Rhrar = crate::RegValueT<Rhrar_SPEC>;

impl Rhrar {
    #[inline(always)]
    pub fn enb(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        rhrar::Enb,
        rhrar::Enb,
        Rhrar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            rhrar::Enb,
            rhrar::Enb,
            Rhrar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pm(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        rhrar::Pm,
        rhrar::Pm,
        Rhrar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            rhrar::Pm,
            rhrar::Pm,
            Rhrar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn hr10(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Rhrar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Rhrar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn hr1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rhrar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rhrar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rhrar {
    #[inline(always)]
    fn default() -> Rhrar {
        <crate::RegValueT<Rhrar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rhrar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enb_SPEC;
    pub type Enb = crate::EnumBitfieldStruct<u8, Enb_SPEC>;
    impl Enb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pm_SPEC;
    pub type Pm = crate::EnumBitfieldStruct<u8, Pm_SPEC>;
    impl Pm {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt2Ar_SPEC;
impl crate::sealed::RegSpec for Bcnt2Ar_SPEC {
    type DataType = u8;
}

pub type Bcnt2Ar = crate::RegValueT<Bcnt2Ar_SPEC>;

impl Bcnt2Ar {
    #[inline(always)]
    pub fn bcnt2ar(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Bcnt2Ar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Bcnt2Ar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bcnt2Ar {
    #[inline(always)]
    fn default() -> Bcnt2Ar {
        <crate::RegValueT<Bcnt2Ar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rwkar_SPEC;
impl crate::sealed::RegSpec for Rwkar_SPEC {
    type DataType = u8;
}

pub type Rwkar = crate::RegValueT<Rwkar_SPEC>;

impl Rwkar {
    #[inline(always)]
    pub fn enb(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        rwkar::Enb,
        rwkar::Enb,
        Rwkar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            rwkar::Enb,
            rwkar::Enb,
            Rwkar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0xf, 1, 0, u8, u8, Rwkar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0xf,1,0,u8,u8,Rwkar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dayw(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        rwkar::Dayw,
        rwkar::Dayw,
        Rwkar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            rwkar::Dayw,
            rwkar::Dayw,
            Rwkar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rwkar {
    #[inline(always)]
    fn default() -> Rwkar {
        <crate::RegValueT<Rwkar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rwkar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enb_SPEC;
    pub type Enb = crate::EnumBitfieldStruct<u8, Enb_SPEC>;
    impl Enb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dayw_SPEC;
    pub type Dayw = crate::EnumBitfieldStruct<u8, Dayw_SPEC>;
    impl Dayw {
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
pub struct Bcnt3Ar_SPEC;
impl crate::sealed::RegSpec for Bcnt3Ar_SPEC {
    type DataType = u8;
}

pub type Bcnt3Ar = crate::RegValueT<Bcnt3Ar_SPEC>;

impl Bcnt3Ar {
    #[inline(always)]
    pub fn bcnt3ar(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Bcnt3Ar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Bcnt3Ar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bcnt3Ar {
    #[inline(always)]
    fn default() -> Bcnt3Ar {
        <crate::RegValueT<Bcnt3Ar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdayar_SPEC;
impl crate::sealed::RegSpec for Rdayar_SPEC {
    type DataType = u8;
}

pub type Rdayar = crate::RegValueT<Rdayar_SPEC>;

impl Rdayar {
    #[inline(always)]
    pub fn enb(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        rdayar::Enb,
        rdayar::Enb,
        Rdayar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            rdayar::Enb,
            rdayar::Enb,
            Rdayar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Rdayar_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Rdayar_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn date10(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Rdayar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Rdayar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn date1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rdayar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rdayar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rdayar {
    #[inline(always)]
    fn default() -> Rdayar {
        <crate::RegValueT<Rdayar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rdayar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enb_SPEC;
    pub type Enb = crate::EnumBitfieldStruct<u8, Enb_SPEC>;
    impl Enb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt0Aer_SPEC;
impl crate::sealed::RegSpec for Bcnt0Aer_SPEC {
    type DataType = u8;
}

pub type Bcnt0Aer = crate::RegValueT<Bcnt0Aer_SPEC>;

impl Bcnt0Aer {
    #[inline(always)]
    pub fn enb(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Bcnt0Aer_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Bcnt0Aer_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bcnt0Aer {
    #[inline(always)]
    fn default() -> Bcnt0Aer {
        <crate::RegValueT<Bcnt0Aer_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rmonar_SPEC;
impl crate::sealed::RegSpec for Rmonar_SPEC {
    type DataType = u8;
}

pub type Rmonar = crate::RegValueT<Rmonar_SPEC>;

impl Rmonar {
    #[inline(always)]
    pub fn enb(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        rmonar::Enb,
        rmonar::Enb,
        Rmonar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            rmonar::Enb,
            rmonar::Enb,
            Rmonar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, Rmonar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,Rmonar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mon10(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Rmonar_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Rmonar_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn mon1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Rmonar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Rmonar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rmonar {
    #[inline(always)]
    fn default() -> Rmonar {
        <crate::RegValueT<Rmonar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rmonar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enb_SPEC;
    pub type Enb = crate::EnumBitfieldStruct<u8, Enb_SPEC>;
    impl Enb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt1Aer_SPEC;
impl crate::sealed::RegSpec for Bcnt1Aer_SPEC {
    type DataType = u8;
}

pub type Bcnt1Aer = crate::RegValueT<Bcnt1Aer_SPEC>;

impl Bcnt1Aer {
    #[inline(always)]
    pub fn enb(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Bcnt1Aer_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Bcnt1Aer_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bcnt1Aer {
    #[inline(always)]
    fn default() -> Bcnt1Aer {
        <crate::RegValueT<Bcnt1Aer_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ryrar_SPEC;
impl crate::sealed::RegSpec for Ryrar_SPEC {
    type DataType = u16;
}

pub type Ryrar = crate::RegValueT<Ryrar_SPEC>;

impl Ryrar {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ryrar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ryrar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn yr10(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Ryrar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Ryrar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn yr1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Ryrar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Ryrar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ryrar {
    #[inline(always)]
    fn default() -> Ryrar {
        <crate::RegValueT<Ryrar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt2Aer_SPEC;
impl crate::sealed::RegSpec for Bcnt2Aer_SPEC {
    type DataType = u16;
}

pub type Bcnt2Aer = crate::RegValueT<Bcnt2Aer_SPEC>;

impl Bcnt2Aer {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Bcnt2Aer_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Bcnt2Aer_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn enb(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Bcnt2Aer_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Bcnt2Aer_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bcnt2Aer {
    #[inline(always)]
    fn default() -> Bcnt2Aer {
        <crate::RegValueT<Bcnt2Aer_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ryraren_SPEC;
impl crate::sealed::RegSpec for Ryraren_SPEC {
    type DataType = u8;
}

pub type Ryraren = crate::RegValueT<Ryraren_SPEC>;

impl Ryraren {
    #[inline(always)]
    pub fn enb(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ryraren::Enb,
        ryraren::Enb,
        Ryraren_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ryraren::Enb,
            ryraren::Enb,
            Ryraren_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, Ryraren_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,Ryraren_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ryraren {
    #[inline(always)]
    fn default() -> Ryraren {
        <crate::RegValueT<Ryraren_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ryraren {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enb_SPEC;
    pub type Enb = crate::EnumBitfieldStruct<u8, Enb_SPEC>;
    impl Enb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt3Aer_SPEC;
impl crate::sealed::RegSpec for Bcnt3Aer_SPEC {
    type DataType = u8;
}

pub type Bcnt3Aer = crate::RegValueT<Bcnt3Aer_SPEC>;

impl Bcnt3Aer {
    #[inline(always)]
    pub fn enb(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Bcnt3Aer_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Bcnt3Aer_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bcnt3Aer {
    #[inline(always)]
    fn default() -> Bcnt3Aer {
        <crate::RegValueT<Bcnt3Aer_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr1_SPEC;
impl crate::sealed::RegSpec for Rcr1_SPEC {
    type DataType = u8;
}

pub type Rcr1 = crate::RegValueT<Rcr1_SPEC>;

impl Rcr1 {
    #[inline(always)]
    pub fn pes(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xf,
        1,
        0,
        rcr1::Pes,
        rcr1::Pes,
        Rcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            rcr1::Pes,
            rcr1::Pes,
            Rcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rtcos(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        rcr1::Rtcos,
        rcr1::Rtcos,
        Rcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            rcr1::Rtcos,
            rcr1::Rtcos,
            Rcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pie(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        rcr1::Pie,
        rcr1::Pie,
        Rcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            rcr1::Pie,
            rcr1::Pie,
            Rcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cie(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        rcr1::Cie,
        rcr1::Cie,
        Rcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            rcr1::Cie,
            rcr1::Cie,
            Rcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn aie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        rcr1::Aie,
        rcr1::Aie,
        Rcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            rcr1::Aie,
            rcr1::Aie,
            Rcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rcr1 {
    #[inline(always)]
    fn default() -> Rcr1 {
        <crate::RegValueT<Rcr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rcr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pes_SPEC;
    pub type Pes = crate::EnumBitfieldStruct<u8, Pes_SPEC>;
    impl Pes {
        pub const _0110: Self = Self::new(6);

        pub const _0111: Self = Self::new(7);

        pub const _1000: Self = Self::new(8);

        pub const _1001: Self = Self::new(9);

        pub const _1010: Self = Self::new(10);

        pub const _1011: Self = Self::new(11);

        pub const _1100: Self = Self::new(12);

        pub const _1101: Self = Self::new(13);

        pub const _1110: Self = Self::new(14);

        pub const _1111: Self = Self::new(15);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtcos_SPEC;
    pub type Rtcos = crate::EnumBitfieldStruct<u8, Rtcos_SPEC>;
    impl Rtcos {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pie_SPEC;
    pub type Pie = crate::EnumBitfieldStruct<u8, Pie_SPEC>;
    impl Pie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cie_SPEC;
    pub type Cie = crate::EnumBitfieldStruct<u8, Cie_SPEC>;
    impl Cie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aie_SPEC;
    pub type Aie = crate::EnumBitfieldStruct<u8, Aie_SPEC>;
    impl Aie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr2_SPEC;
impl crate::sealed::RegSpec for Rcr2_SPEC {
    type DataType = u8;
}

pub type Rcr2 = crate::RegValueT<Rcr2_SPEC>;

impl Rcr2 {
    #[inline(always)]
    pub fn cntmd(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        rcr2::Cntmd,
        rcr2::Cntmd,
        Rcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            rcr2::Cntmd,
            rcr2::Cntmd,
            Rcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn hr24(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        rcr2::Hr24,
        rcr2::Hr24,
        Rcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            rcr2::Hr24,
            rcr2::Hr24,
            Rcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn aadjp(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        rcr2::Aadjp,
        rcr2::Aadjp,
        Rcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            rcr2::Aadjp,
            rcr2::Aadjp,
            Rcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn aadje(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        rcr2::Aadje,
        rcr2::Aadje,
        Rcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            rcr2::Aadje,
            rcr2::Aadje,
            Rcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rtcoe(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        rcr2::Rtcoe,
        rcr2::Rtcoe,
        Rcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            rcr2::Rtcoe,
            rcr2::Rtcoe,
            Rcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adj30(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        rcr2::Adj30,
        rcr2::Adj30,
        Rcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            rcr2::Adj30,
            rcr2::Adj30,
            Rcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reset(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        rcr2::Reset,
        rcr2::Reset,
        Rcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            rcr2::Reset,
            rcr2::Reset,
            Rcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn start(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        rcr2::Start,
        rcr2::Start,
        Rcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            rcr2::Start,
            rcr2::Start,
            Rcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rcr2 {
    #[inline(always)]
    fn default() -> Rcr2 {
        <crate::RegValueT<Rcr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rcr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cntmd_SPEC;
    pub type Cntmd = crate::EnumBitfieldStruct<u8, Cntmd_SPEC>;
    impl Cntmd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hr24_SPEC;
    pub type Hr24 = crate::EnumBitfieldStruct<u8, Hr24_SPEC>;
    impl Hr24 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aadjp_SPEC;
    pub type Aadjp = crate::EnumBitfieldStruct<u8, Aadjp_SPEC>;
    impl Aadjp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aadje_SPEC;
    pub type Aadje = crate::EnumBitfieldStruct<u8, Aadje_SPEC>;
    impl Aadje {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtcoe_SPEC;
    pub type Rtcoe = crate::EnumBitfieldStruct<u8, Rtcoe_SPEC>;
    impl Rtcoe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adj30_SPEC;
    pub type Adj30 = crate::EnumBitfieldStruct<u8, Adj30_SPEC>;
    impl Adj30 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Reset_SPEC;
    pub type Reset = crate::EnumBitfieldStruct<u8, Reset_SPEC>;
    impl Reset {
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
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr4_SPEC;
impl crate::sealed::RegSpec for Rcr4_SPEC {
    type DataType = u8;
}

pub type Rcr4 = crate::RegValueT<Rcr4_SPEC>;

impl Rcr4 {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Rcr4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Rcr4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rcksel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        rcr4::Rcksel,
        rcr4::Rcksel,
        Rcr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            rcr4::Rcksel,
            rcr4::Rcksel,
            Rcr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rcr4 {
    #[inline(always)]
    fn default() -> Rcr4 {
        <crate::RegValueT<Rcr4_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rcr4 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcksel_SPEC;
    pub type Rcksel = crate::EnumBitfieldStruct<u8, Rcksel_SPEC>;
    impl Rcksel {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rfrh_SPEC;
impl crate::sealed::RegSpec for Rfrh_SPEC {
    type DataType = u16;
}

pub type Rfrh = crate::RegValueT<Rfrh_SPEC>;

impl Rfrh {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7fff, 1, 0, u16, u16, Rfrh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7fff,1,0,u16,u16,Rfrh_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfc16(self) -> crate::common::RegisterFieldBool<0, 1, 0, Rfrh_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Rfrh_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Rfrh {
    #[inline(always)]
    fn default() -> Rfrh {
        <crate::RegValueT<Rfrh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rfrl_SPEC;
impl crate::sealed::RegSpec for Rfrl_SPEC {
    type DataType = u16;
}

pub type Rfrl = crate::RegValueT<Rfrl_SPEC>;

impl Rfrl {
    #[inline(always)]
    pub fn rfc(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Rfrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Rfrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rfrl {
    #[inline(always)]
    fn default() -> Rfrl {
        <crate::RegValueT<Rfrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Radj_SPEC;
impl crate::sealed::RegSpec for Radj_SPEC {
    type DataType = u8;
}

pub type Radj = crate::RegValueT<Radj_SPEC>;

impl Radj {
    #[inline(always)]
    pub fn pmadj(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        radj::Pmadj,
        radj::Pmadj,
        Radj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            radj::Pmadj,
            radj::Pmadj,
            Radj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adj(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Radj_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Radj_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Radj {
    #[inline(always)]
    fn default() -> Radj {
        <crate::RegValueT<Radj_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod radj {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmadj_SPEC;
    pub type Pmadj = crate::EnumBitfieldStruct<u8, Pmadj_SPEC>;
    impl Pmadj {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
