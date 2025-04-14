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
// Generated from SVD 1.50.00, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:17:37 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Flash I/O Registers"]
unsafe impl ::core::marker::Send for super::Flcn {}
unsafe impl ::core::marker::Sync for super::Flcn {}
impl super::Flcn {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn dflctl(
        &self,
    ) -> &'static crate::common::Reg<self::Dflctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dflctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fpmcr(&self) -> &'static crate::common::Reg<self::Fpmcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fpmcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fasr(&self) -> &'static crate::common::Reg<self::Fasr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fasr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(260usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fsarl(&self) -> &'static crate::common::Reg<self::Fsarl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fsarl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(264usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fsarh(&self) -> &'static crate::common::Reg<self::Fsarh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fsarh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(272usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fcr(&self) -> &'static crate::common::Reg<self::Fcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(276usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fearl(&self) -> &'static crate::common::Reg<self::Fearl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fearl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(280usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fearh(&self) -> &'static crate::common::Reg<self::Fearh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fearh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(288usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fresetr(
        &self,
    ) -> &'static crate::common::Reg<self::Fresetr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fresetr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(292usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fstatr1(
        &self,
    ) -> &'static crate::common::Reg<self::Fstatr1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fstatr1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(300usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fwbl0(&self) -> &'static crate::common::Reg<self::Fwbl0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwbl0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(304usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fwbh0(&self) -> &'static crate::common::Reg<self::Fwbh0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fwbh0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(312usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fpr(&self) -> &'static crate::common::Reg<self::Fpr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fpr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(384usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fpsr(&self) -> &'static crate::common::Reg<self::Fpsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fpsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(388usize),
            )
        }
    }

    #[inline(always)]
    pub const fn frbl0(&self) -> &'static crate::common::Reg<self::Frbl0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Frbl0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(392usize),
            )
        }
    }

    #[inline(always)]
    pub const fn frbh0(&self) -> &'static crate::common::Reg<self::Frbh0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Frbh0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(400usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fscmr(&self) -> &'static crate::common::Reg<self::Fscmr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fscmr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(448usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fawsmr(&self) -> &'static crate::common::Reg<self::Fawsmr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fawsmr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(456usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fawemr(&self) -> &'static crate::common::Reg<self::Fawemr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fawemr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(464usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fisr(&self) -> &'static crate::common::Reg<self::Fisr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fisr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(472usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fexcr(&self) -> &'static crate::common::Reg<self::Fexcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fexcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(476usize),
            )
        }
    }

    #[inline(always)]
    pub const fn feaml(&self) -> &'static crate::common::Reg<self::Feaml_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Feaml_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(480usize),
            )
        }
    }

    #[inline(always)]
    pub const fn feamh(&self) -> &'static crate::common::Reg<self::Feamh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Feamh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(488usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fstatr2(
        &self,
    ) -> &'static crate::common::Reg<self::Fstatr2_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fstatr2_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(496usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tscdr(&self) -> &'static crate::common::Reg<self::Tscdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Tscdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(552usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsutrima(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsutrima_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsutrima_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(932usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsutrimb(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsutrimb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsutrimb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(936usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fentryr(
        &self,
    ) -> &'static crate::common::Reg<self::Fentryr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fentryr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16304usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fldwaitr(
        &self,
    ) -> &'static crate::common::Reg<self::Fldwaitr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fldwaitr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16324usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pfber(&self) -> &'static crate::common::Reg<self::Pfber_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pfber_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16328usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dflctl_SPEC;
impl crate::sealed::RegSpec for Dflctl_SPEC {
    type DataType = u8;
}

pub type Dflctl = crate::RegValueT<Dflctl_SPEC>;

impl Dflctl {
    #[inline(always)]
    pub fn dflen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dflctl::Dflen,
        dflctl::Dflen,
        Dflctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dflctl::Dflen,
            dflctl::Dflen,
            Dflctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dflctl {
    #[inline(always)]
    fn default() -> Dflctl {
        <crate::RegValueT<Dflctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dflctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dflen_SPEC;
    pub type Dflen = crate::EnumBitfieldStruct<u8, Dflen_SPEC>;
    impl Dflen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fpmcr_SPEC;
impl crate::sealed::RegSpec for Fpmcr_SPEC {
    type DataType = u8;
}

pub type Fpmcr = crate::RegValueT<Fpmcr_SPEC>;

impl Fpmcr {
    #[inline(always)]
    pub fn fms0(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        fpmcr::Fms0,
        fpmcr::Fms0,
        Fpmcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            fpmcr::Fms0,
            fpmcr::Fms0,
            Fpmcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rpdis(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        fpmcr::Rpdis,
        fpmcr::Rpdis,
        Fpmcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            fpmcr::Rpdis,
            fpmcr::Rpdis,
            Fpmcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fms1(self) -> crate::common::RegisterFieldBool<4, 1, 0, Fpmcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Fpmcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fpmcr {
    #[inline(always)]
    fn default() -> Fpmcr {
        <crate::RegValueT<Fpmcr_SPEC> as RegisterValue<_>>::new(8)
    }
}
pub mod fpmcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fms0_SPEC;
    pub type Fms0 = crate::EnumBitfieldStruct<u8, Fms0_SPEC>;
    impl Fms0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpdis_SPEC;
    pub type Rpdis = crate::EnumBitfieldStruct<u8, Rpdis_SPEC>;
    impl Rpdis {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fasr_SPEC;
impl crate::sealed::RegSpec for Fasr_SPEC {
    type DataType = u8;
}

pub type Fasr = crate::RegValueT<Fasr_SPEC>;

impl Fasr {
    #[inline(always)]
    pub fn exs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fasr::Exs,
        fasr::Exs,
        Fasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fasr::Exs,
            fasr::Exs,
            Fasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fasr {
    #[inline(always)]
    fn default() -> Fasr {
        <crate::RegValueT<Fasr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fasr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Exs_SPEC;
    pub type Exs = crate::EnumBitfieldStruct<u8, Exs_SPEC>;
    impl Exs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsarl_SPEC;
impl crate::sealed::RegSpec for Fsarl_SPEC {
    type DataType = u16;
}

pub type Fsarl = crate::RegValueT<Fsarl_SPEC>;

impl Fsarl {
    #[inline(always)]
    pub fn fsarl(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fsarl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fsarl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fsarl {
    #[inline(always)]
    fn default() -> Fsarl {
        <crate::RegValueT<Fsarl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsarh_SPEC;
impl crate::sealed::RegSpec for Fsarh_SPEC {
    type DataType = u16;
}

pub type Fsarh = crate::RegValueT<Fsarh_SPEC>;

impl Fsarh {
    #[inline(always)]
    pub fn fsarh(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fsarh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fsarh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fsarh {
    #[inline(always)]
    fn default() -> Fsarh {
        <crate::RegValueT<Fsarh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcr_SPEC;
impl crate::sealed::RegSpec for Fcr_SPEC {
    type DataType = u8;
}

pub type Fcr = crate::RegValueT<Fcr_SPEC>;

impl Fcr {
    #[inline(always)]
    pub fn cmd(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, fcr::Cmd, fcr::Cmd, Fcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,fcr::Cmd,fcr::Cmd,Fcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn drc(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, fcr::Drc, fcr::Drc, Fcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,fcr::Drc,fcr::Drc,Fcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn stop(self) -> crate::common::RegisterFieldBool<6, 1, 0, Fcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Fcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn opst(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, fcr::Opst, fcr::Opst, Fcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            fcr::Opst,
            fcr::Opst,
            Fcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fcr {
    #[inline(always)]
    fn default() -> Fcr {
        <crate::RegValueT<Fcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmd_SPEC;
    pub type Cmd = crate::EnumBitfieldStruct<u8, Cmd_SPEC>;
    impl Cmd {
        pub const _0_X_1: Self = Self::new(1);

        pub const _0_X_3: Self = Self::new(3);

        pub const _0_X_4: Self = Self::new(4);

        pub const _0_X_5: Self = Self::new(5);

        pub const _0_X_6: Self = Self::new(6);

        pub const _0_X_B: Self = Self::new(11);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Drc_SPEC;
    pub type Drc = crate::EnumBitfieldStruct<u8, Drc_SPEC>;
    impl Drc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Opst_SPEC;
    pub type Opst = crate::EnumBitfieldStruct<u8, Opst_SPEC>;
    impl Opst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fearl_SPEC;
impl crate::sealed::RegSpec for Fearl_SPEC {
    type DataType = u16;
}

pub type Fearl = crate::RegValueT<Fearl_SPEC>;

impl Fearl {
    #[inline(always)]
    pub fn fearl(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fearl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fearl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fearl {
    #[inline(always)]
    fn default() -> Fearl {
        <crate::RegValueT<Fearl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fearh_SPEC;
impl crate::sealed::RegSpec for Fearh_SPEC {
    type DataType = u16;
}

pub type Fearh = crate::RegValueT<Fearh_SPEC>;

impl Fearh {
    #[inline(always)]
    pub fn fearh(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fearh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fearh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fearh {
    #[inline(always)]
    fn default() -> Fearh {
        <crate::RegValueT<Fearh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fresetr_SPEC;
impl crate::sealed::RegSpec for Fresetr_SPEC {
    type DataType = u8;
}

pub type Fresetr = crate::RegValueT<Fresetr_SPEC>;

impl Fresetr {
    #[inline(always)]
    pub fn freset(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fresetr::Freset,
        fresetr::Freset,
        Fresetr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fresetr::Freset,
            fresetr::Freset,
            Fresetr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fresetr {
    #[inline(always)]
    fn default() -> Fresetr {
        <crate::RegValueT<Fresetr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fresetr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Freset_SPEC;
    pub type Freset = crate::EnumBitfieldStruct<u8, Freset_SPEC>;
    impl Freset {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fstatr1_SPEC;
impl crate::sealed::RegSpec for Fstatr1_SPEC {
    type DataType = u8;
}

pub type Fstatr1 = crate::RegValueT<Fstatr1_SPEC>;

impl Fstatr1 {
    #[inline(always)]
    pub fn drrdy(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        fstatr1::Drrdy,
        fstatr1::Drrdy,
        Fstatr1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            fstatr1::Drrdy,
            fstatr1::Drrdy,
            Fstatr1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn frdy(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        fstatr1::Frdy,
        fstatr1::Frdy,
        Fstatr1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            fstatr1::Frdy,
            fstatr1::Frdy,
            Fstatr1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn exrdy(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        fstatr1::Exrdy,
        fstatr1::Exrdy,
        Fstatr1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            fstatr1::Exrdy,
            fstatr1::Exrdy,
            Fstatr1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fstatr1 {
    #[inline(always)]
    fn default() -> Fstatr1 {
        <crate::RegValueT<Fstatr1_SPEC> as RegisterValue<_>>::new(4)
    }
}
pub mod fstatr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Drrdy_SPEC;
    pub type Drrdy = crate::EnumBitfieldStruct<u8, Drrdy_SPEC>;
    impl Drrdy {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Frdy_SPEC;
    pub type Frdy = crate::EnumBitfieldStruct<u8, Frdy_SPEC>;
    impl Frdy {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Exrdy_SPEC;
    pub type Exrdy = crate::EnumBitfieldStruct<u8, Exrdy_SPEC>;
    impl Exrdy {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwbl0_SPEC;
impl crate::sealed::RegSpec for Fwbl0_SPEC {
    type DataType = u16;
}

pub type Fwbl0 = crate::RegValueT<Fwbl0_SPEC>;

impl Fwbl0 {
    #[inline(always)]
    pub fn wdata(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwbl0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwbl0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwbl0 {
    #[inline(always)]
    fn default() -> Fwbl0 {
        <crate::RegValueT<Fwbl0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwbh0_SPEC;
impl crate::sealed::RegSpec for Fwbh0_SPEC {
    type DataType = u16;
}

pub type Fwbh0 = crate::RegValueT<Fwbh0_SPEC>;

impl Fwbh0 {
    #[inline(always)]
    pub fn wdata(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Fwbh0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fwbh0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwbh0 {
    #[inline(always)]
    fn default() -> Fwbh0 {
        <crate::RegValueT<Fwbh0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fpr_SPEC;
impl crate::sealed::RegSpec for Fpr_SPEC {
    type DataType = u8;
}

pub type Fpr = crate::RegValueT<Fpr_SPEC>;

impl Fpr {
    #[inline(always)]
    pub fn fpr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Fpr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Fpr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fpr {
    #[inline(always)]
    fn default() -> Fpr {
        <crate::RegValueT<Fpr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fpsr_SPEC;
impl crate::sealed::RegSpec for Fpsr_SPEC {
    type DataType = u8;
}

pub type Fpsr = crate::RegValueT<Fpsr_SPEC>;

impl Fpsr {
    #[inline(always)]
    pub fn perr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fpsr::Perr,
        fpsr::Perr,
        Fpsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fpsr::Perr,
            fpsr::Perr,
            Fpsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fpsr {
    #[inline(always)]
    fn default() -> Fpsr {
        <crate::RegValueT<Fpsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fpsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Perr_SPEC;
    pub type Perr = crate::EnumBitfieldStruct<u8, Perr_SPEC>;
    impl Perr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frbl0_SPEC;
impl crate::sealed::RegSpec for Frbl0_SPEC {
    type DataType = u16;
}

pub type Frbl0 = crate::RegValueT<Frbl0_SPEC>;

impl Frbl0 {
    #[inline(always)]
    pub fn rdata(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Frbl0_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Frbl0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Frbl0 {
    #[inline(always)]
    fn default() -> Frbl0 {
        <crate::RegValueT<Frbl0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frbh0_SPEC;
impl crate::sealed::RegSpec for Frbh0_SPEC {
    type DataType = u16;
}

pub type Frbh0 = crate::RegValueT<Frbh0_SPEC>;

impl Frbh0 {
    #[inline(always)]
    pub fn rdata(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Frbh0_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Frbh0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Frbh0 {
    #[inline(always)]
    fn default() -> Frbh0 {
        <crate::RegValueT<Frbh0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fscmr_SPEC;
impl crate::sealed::RegSpec for Fscmr_SPEC {
    type DataType = u16;
}

pub type Fscmr = crate::RegValueT<Fscmr_SPEC>;

impl Fscmr {
    #[inline(always)]
    pub fn sasmf(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        fscmr::Sasmf,
        fscmr::Sasmf,
        Fscmr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            fscmr::Sasmf,
            fscmr::Sasmf,
            Fscmr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fspr(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        fscmr::Fspr,
        fscmr::Fspr,
        Fscmr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            fscmr::Fspr,
            fscmr::Fspr,
            Fscmr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fscmr {
    #[inline(always)]
    fn default() -> Fscmr {
        <crate::RegValueT<Fscmr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fscmr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sasmf_SPEC;
    pub type Sasmf = crate::EnumBitfieldStruct<u8, Sasmf_SPEC>;
    impl Sasmf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fspr_SPEC;
    pub type Fspr = crate::EnumBitfieldStruct<u8, Fspr_SPEC>;
    impl Fspr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fawsmr_SPEC;
impl crate::sealed::RegSpec for Fawsmr_SPEC {
    type DataType = u16;
}

pub type Fawsmr = crate::RegValueT<Fawsmr_SPEC>;

impl Fawsmr {
    #[inline(always)]
    pub fn faws(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, Fawsmr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,Fawsmr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn fspr(self) -> crate::common::RegisterFieldBool<15, 1, 0, Fawsmr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Fawsmr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fawsmr {
    #[inline(always)]
    fn default() -> Fawsmr {
        <crate::RegValueT<Fawsmr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fawemr_SPEC;
impl crate::sealed::RegSpec for Fawemr_SPEC {
    type DataType = u16;
}

pub type Fawemr = crate::RegValueT<Fawemr_SPEC>;

impl Fawemr {
    #[inline(always)]
    pub fn fawe(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, Fawemr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,Fawemr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sasmf(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Fawemr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Fawemr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fawemr {
    #[inline(always)]
    fn default() -> Fawemr {
        <crate::RegValueT<Fawemr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fisr_SPEC;
impl crate::sealed::RegSpec for Fisr_SPEC {
    type DataType = u8;
}

pub type Fisr = crate::RegValueT<Fisr_SPEC>;

impl Fisr {
    #[inline(always)]
    pub fn pcka(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Fisr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Fisr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sas(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        fisr::Sas,
        fisr::Sas,
        Fisr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            fisr::Sas,
            fisr::Sas,
            Fisr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fisr {
    #[inline(always)]
    fn default() -> Fisr {
        <crate::RegValueT<Fisr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fisr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sas_SPEC;
    pub type Sas = crate::EnumBitfieldStruct<u8, Sas_SPEC>;
    impl Sas {
        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fexcr_SPEC;
impl crate::sealed::RegSpec for Fexcr_SPEC {
    type DataType = u8;
}

pub type Fexcr = crate::RegValueT<Fexcr_SPEC>;

impl Fexcr {
    #[inline(always)]
    pub fn cmd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        fexcr::Cmd,
        fexcr::Cmd,
        Fexcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            fexcr::Cmd,
            fexcr::Cmd,
            Fexcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn opst(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        fexcr::Opst,
        fexcr::Opst,
        Fexcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            fexcr::Opst,
            fexcr::Opst,
            Fexcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fexcr {
    #[inline(always)]
    fn default() -> Fexcr {
        <crate::RegValueT<Fexcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fexcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmd_SPEC;
    pub type Cmd = crate::EnumBitfieldStruct<u8, Cmd_SPEC>;
    impl Cmd {
        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _101: Self = Self::new(5);

        pub const _110: Self = Self::new(6);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Opst_SPEC;
    pub type Opst = crate::EnumBitfieldStruct<u8, Opst_SPEC>;
    impl Opst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Feaml_SPEC;
impl crate::sealed::RegSpec for Feaml_SPEC {
    type DataType = u16;
}

pub type Feaml = crate::RegValueT<Feaml_SPEC>;

impl Feaml {
    #[inline(always)]
    pub fn feaml(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Feaml_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Feaml_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Feaml {
    #[inline(always)]
    fn default() -> Feaml {
        <crate::RegValueT<Feaml_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Feamh_SPEC;
impl crate::sealed::RegSpec for Feamh_SPEC {
    type DataType = u16;
}

pub type Feamh = crate::RegValueT<Feamh_SPEC>;

impl Feamh {
    #[inline(always)]
    pub fn feamh(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Feamh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Feamh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Feamh {
    #[inline(always)]
    fn default() -> Feamh {
        <crate::RegValueT<Feamh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fstatr2_SPEC;
impl crate::sealed::RegSpec for Fstatr2_SPEC {
    type DataType = u16;
}

pub type Fstatr2 = crate::RegValueT<Fstatr2_SPEC>;

impl Fstatr2 {
    #[inline(always)]
    pub fn ererr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fstatr2::Ererr,
        fstatr2::Ererr,
        Fstatr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fstatr2::Ererr,
            fstatr2::Ererr,
            Fstatr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn prgerr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        fstatr2::Prgerr,
        fstatr2::Prgerr,
        Fstatr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            fstatr2::Prgerr,
            fstatr2::Prgerr,
            Fstatr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn prgerr01(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        fstatr2::Prgerr01,
        fstatr2::Prgerr01,
        Fstatr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            fstatr2::Prgerr01,
            fstatr2::Prgerr01,
            Fstatr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bcerr(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        fstatr2::Bcerr,
        fstatr2::Bcerr,
        Fstatr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            fstatr2::Bcerr,
            fstatr2::Bcerr,
            Fstatr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ilglerr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        fstatr2::Ilglerr,
        fstatr2::Ilglerr,
        Fstatr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            fstatr2::Ilglerr,
            fstatr2::Ilglerr,
            Fstatr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn eilglerr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        fstatr2::Eilglerr,
        fstatr2::Eilglerr,
        Fstatr2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            fstatr2::Eilglerr,
            fstatr2::Eilglerr,
            Fstatr2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fstatr2 {
    #[inline(always)]
    fn default() -> Fstatr2 {
        <crate::RegValueT<Fstatr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fstatr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ererr_SPEC;
    pub type Ererr = crate::EnumBitfieldStruct<u8, Ererr_SPEC>;
    impl Ererr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prgerr_SPEC;
    pub type Prgerr = crate::EnumBitfieldStruct<u8, Prgerr_SPEC>;
    impl Prgerr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prgerr01_SPEC;
    pub type Prgerr01 = crate::EnumBitfieldStruct<u8, Prgerr01_SPEC>;
    impl Prgerr01 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bcerr_SPEC;
    pub type Bcerr = crate::EnumBitfieldStruct<u8, Bcerr_SPEC>;
    impl Bcerr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ilglerr_SPEC;
    pub type Ilglerr = crate::EnumBitfieldStruct<u8, Ilglerr_SPEC>;
    impl Ilglerr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eilglerr_SPEC;
    pub type Eilglerr = crate::EnumBitfieldStruct<u8, Eilglerr_SPEC>;
    impl Eilglerr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tscdr_SPEC;
impl crate::sealed::RegSpec for Tscdr_SPEC {
    type DataType = u16;
}

pub type Tscdr = crate::RegValueT<Tscdr_SPEC>;

impl Tscdr {
    #[inline(always)]
    pub fn tscdr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Tscdr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Tscdr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Tscdr {
    #[inline(always)]
    fn default() -> Tscdr {
        <crate::RegValueT<Tscdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsutrima_SPEC;
impl crate::sealed::RegSpec for Ctsutrima_SPEC {
    type DataType = u32;
}

pub type Ctsutrima = crate::RegValueT<Ctsutrima_SPEC>;

impl Ctsutrima {
    #[inline(always)]
    pub fn rtrim(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ctsutrima_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ctsutrima_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dactrim(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ctsutrima_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ctsutrima_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn suadjd(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Ctsutrima_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Ctsutrima_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn suadjtrim(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Ctsutrima_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Ctsutrima_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsutrima {
    #[inline(always)]
    fn default() -> Ctsutrima {
        <crate::RegValueT<Ctsutrima_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsutrimb_SPEC;
impl crate::sealed::RegSpec for Ctsutrimb_SPEC {
    type DataType = u32;
}

pub type Ctsutrimb = crate::RegValueT<Ctsutrimb_SPEC>;

impl Ctsutrimb {
    #[inline(always)]
    pub fn tresult0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ctsutrimb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ctsutrimb_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tresult1(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ctsutrimb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ctsutrimb_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tresult2(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Ctsutrimb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Ctsutrimb_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tresult3(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Ctsutrimb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Ctsutrimb_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsutrimb {
    #[inline(always)]
    fn default() -> Ctsutrimb {
        <crate::RegValueT<Ctsutrimb_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fentryr_SPEC;
impl crate::sealed::RegSpec for Fentryr_SPEC {
    type DataType = u16;
}

pub type Fentryr = crate::RegValueT<Fentryr_SPEC>;

impl Fentryr {
    #[inline(always)]
    pub fn fentry0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fentryr::Fentry0,
        fentryr::Fentry0,
        Fentryr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fentryr::Fentry0,
            fentryr::Fentry0,
            Fentryr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fentryd(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        fentryr::Fentryd,
        fentryr::Fentryd,
        Fentryr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            fentryr::Fentryd,
            fentryr::Fentryd,
            Fentryr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fekey(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Fentryr_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Fentryr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Fentryr {
    #[inline(always)]
    fn default() -> Fentryr {
        <crate::RegValueT<Fentryr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fentryr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fentry0_SPEC;
    pub type Fentry0 = crate::EnumBitfieldStruct<u8, Fentry0_SPEC>;
    impl Fentry0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fentryd_SPEC;
    pub type Fentryd = crate::EnumBitfieldStruct<u8, Fentryd_SPEC>;
    impl Fentryd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fldwaitr_SPEC;
impl crate::sealed::RegSpec for Fldwaitr_SPEC {
    type DataType = u8;
}

pub type Fldwaitr = crate::RegValueT<Fldwaitr_SPEC>;

impl Fldwaitr {
    #[inline(always)]
    pub fn fldwait1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        fldwaitr::Fldwait1,
        fldwaitr::Fldwait1,
        Fldwaitr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fldwaitr::Fldwait1,
            fldwaitr::Fldwait1,
            Fldwaitr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Fldwaitr {
    #[inline(always)]
    fn default() -> Fldwaitr {
        <crate::RegValueT<Fldwaitr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fldwaitr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fldwait1_SPEC;
    pub type Fldwait1 = crate::EnumBitfieldStruct<u8, Fldwait1_SPEC>;
    impl Fldwait1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfber_SPEC;
impl crate::sealed::RegSpec for Pfber_SPEC {
    type DataType = u8;
}

pub type Pfber = crate::RegValueT<Pfber_SPEC>;

impl Pfber {
    #[inline(always)]
    pub fn pfbe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pfber::Pfbe,
        pfber::Pfbe,
        Pfber_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pfber::Pfbe,
            pfber::Pfbe,
            Pfber_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pfber {
    #[inline(always)]
    fn default() -> Pfber {
        <crate::RegValueT<Pfber_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pfber {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfbe_SPEC;
    pub type Pfbe = crate::EnumBitfieldStruct<u8, Pfbe_SPEC>;
    impl Pfbe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
