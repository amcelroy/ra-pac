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
// Generated from SVD 1.40.00, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:21:24 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Inter-Integrated Circuit 0"]
unsafe impl ::core::marker::Send for super::IicB0 {}
unsafe impl ::core::marker::Sync for super::IicB0 {}
impl super::IicB0 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn bctl(&self) -> &'static crate::common::Reg<self::Bctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rstctl(
        &self,
    ) -> &'static crate::common::Reg<self::Rstctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rstctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn prsst(&self) -> &'static crate::common::Reg<self::Prsst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Prsst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bfctl(&self) -> &'static crate::common::Reg<self::Bfctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bfctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[inline(always)]
    pub const fn svctl(&self) -> &'static crate::common::Reg<self::Svctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Svctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[inline(always)]
    pub const fn refckctl(
        &self,
    ) -> &'static crate::common::Reg<self::Refckctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Refckctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[inline(always)]
    pub const fn stdbr(&self) -> &'static crate::common::Reg<self::Stdbr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Stdbr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[inline(always)]
    pub const fn extbr(&self) -> &'static crate::common::Reg<self::Extbr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Extbr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bfrecdt(
        &self,
    ) -> &'static crate::common::Reg<self::Bfrecdt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bfrecdt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }

    #[inline(always)]
    pub const fn outctl(
        &self,
    ) -> &'static crate::common::Reg<self::Outctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Outctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(136usize),
            )
        }
    }

    #[inline(always)]
    pub const fn inctl(&self) -> &'static crate::common::Reg<self::Inctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Inctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tmoctl(
        &self,
    ) -> &'static crate::common::Reg<self::Tmoctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmoctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ackctl(
        &self,
    ) -> &'static crate::common::Reg<self::Ackctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ackctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[inline(always)]
    pub const fn scstrctl(
        &self,
    ) -> &'static crate::common::Reg<self::Scstrctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Scstrctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(164usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cndctl(
        &self,
    ) -> &'static crate::common::Reg<self::Cndctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cndctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(320usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ntdtbp0(
        &self,
    ) -> &'static crate::common::Reg<self::Ntdtbp0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ntdtbp0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(344usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ntdtbp0_by(
        &self,
    ) -> &'static crate::common::Reg<self::Ntdtbp0By_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ntdtbp0By_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(344usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bst(&self) -> &'static crate::common::Reg<self::Bst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(464usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bste(&self) -> &'static crate::common::Reg<self::Bste_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bste_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(468usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bie(&self) -> &'static crate::common::Reg<self::Bie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(472usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bstfc(&self) -> &'static crate::common::Reg<self::Bstfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bstfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(476usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ntst(&self) -> &'static crate::common::Reg<self::Ntst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ntst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(480usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ntste(&self) -> &'static crate::common::Reg<self::Ntste_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ntste_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(484usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ntie(&self) -> &'static crate::common::Reg<self::Ntie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ntie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(488usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ntstfc(&self) -> &'static crate::common::Reg<self::Ntstfc_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ntstfc_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(492usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bcst(&self) -> &'static crate::common::Reg<self::Bcst_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Bcst_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(528usize),
            )
        }
    }

    #[inline(always)]
    pub const fn svst(&self) -> &'static crate::common::Reg<self::Svst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Svst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(532usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sdatbas(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Sdatbas_SPEC, crate::common::RW>,
        3,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2b0usize))
        }
    }

    #[inline(always)]
    pub const fn svdvad(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Svdvad_SPEC, crate::common::R>,
        3,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x330usize))
        }
    }

    #[inline(always)]
    pub const fn bitcnt(&self) -> &'static crate::common::Reg<self::Bitcnt_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Bitcnt_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(896usize),
            )
        }
    }

    #[inline(always)]
    pub const fn prstdbg(
        &self,
    ) -> &'static crate::common::Reg<self::Prstdbg_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Prstdbg_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(972usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bctl_SPEC;
impl crate::sealed::RegSpec for Bctl_SPEC {
    type DataType = u32;
}

pub type Bctl = crate::RegValueT<Bctl_SPEC>;

impl Bctl {
    #[inline(always)]
    pub fn buse(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        bctl::Buse,
        bctl::Buse,
        Bctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            bctl::Buse,
            bctl::Buse,
            Bctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bctl {
    #[inline(always)]
    fn default() -> Bctl {
        <crate::RegValueT<Bctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Buse_SPEC;
    pub type Buse = crate::EnumBitfieldStruct<u8, Buse_SPEC>;
    impl Buse {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rstctl_SPEC;
impl crate::sealed::RegSpec for Rstctl_SPEC {
    type DataType = u32;
}

pub type Rstctl = crate::RegValueT<Rstctl_SPEC>;

impl Rstctl {
    #[inline(always)]
    pub fn ri2crst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        rstctl::Ri2Crst,
        rstctl::Ri2Crst,
        Rstctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            rstctl::Ri2Crst,
            rstctl::Ri2Crst,
            Rstctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn intlrst(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        rstctl::Intlrst,
        rstctl::Intlrst,
        Rstctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            rstctl::Intlrst,
            rstctl::Intlrst,
            Rstctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rstctl {
    #[inline(always)]
    fn default() -> Rstctl {
        <crate::RegValueT<Rstctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rstctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ri2Crst_SPEC;
    pub type Ri2Crst = crate::EnumBitfieldStruct<u8, Ri2Crst_SPEC>;
    impl Ri2Crst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Intlrst_SPEC;
    pub type Intlrst = crate::EnumBitfieldStruct<u8, Intlrst_SPEC>;
    impl Intlrst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prsst_SPEC;
impl crate::sealed::RegSpec for Prsst_SPEC {
    type DataType = u32;
}

pub type Prsst = crate::RegValueT<Prsst_SPEC>;

impl Prsst {
    #[inline(always)]
    pub fn crms(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        prsst::Crms,
        prsst::Crms,
        Prsst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            prsst::Crms,
            prsst::Crms,
            Prsst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trmd(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        prsst::Trmd,
        prsst::Trmd,
        Prsst_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            prsst::Trmd,
            prsst::Trmd,
            Prsst_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn prsstwp(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        prsst::Prsstwp,
        prsst::Prsstwp,
        Prsst_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            prsst::Prsstwp,
            prsst::Prsstwp,
            Prsst_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Prsst {
    #[inline(always)]
    fn default() -> Prsst {
        <crate::RegValueT<Prsst_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod prsst {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Crms_SPEC;
    pub type Crms = crate::EnumBitfieldStruct<u8, Crms_SPEC>;
    impl Crms {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trmd_SPEC;
    pub type Trmd = crate::EnumBitfieldStruct<u8, Trmd_SPEC>;
    impl Trmd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prsstwp_SPEC;
    pub type Prsstwp = crate::EnumBitfieldStruct<u8, Prsstwp_SPEC>;
    impl Prsstwp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bfctl_SPEC;
impl crate::sealed::RegSpec for Bfctl_SPEC {
    type DataType = u32;
}

pub type Bfctl = crate::RegValueT<Bfctl_SPEC>;

impl Bfctl {
    #[inline(always)]
    pub fn male(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bfctl::Male,
        bfctl::Male,
        Bfctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bfctl::Male,
            bfctl::Male,
            Bfctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn nale(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        bfctl::Nale,
        bfctl::Nale,
        Bfctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            bfctl::Nale,
            bfctl::Nale,
            Bfctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sale(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        bfctl::Sale,
        bfctl::Sale,
        Bfctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            bfctl::Sale,
            bfctl::Sale,
            Bfctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn scsyne(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        bfctl::Scsyne,
        bfctl::Scsyne,
        Bfctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            bfctl::Scsyne,
            bfctl::Scsyne,
            Bfctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn smbs(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        bfctl::Smbs,
        bfctl::Smbs,
        Bfctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            bfctl::Smbs,
            bfctl::Smbs,
            Bfctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fmpe(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        bfctl::Fmpe,
        bfctl::Fmpe,
        Bfctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            bfctl::Fmpe,
            bfctl::Fmpe,
            Bfctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn hsme(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        bfctl::Hsme,
        bfctl::Hsme,
        Bfctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            bfctl::Hsme,
            bfctl::Hsme,
            Bfctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bfctl {
    #[inline(always)]
    fn default() -> Bfctl {
        <crate::RegValueT<Bfctl_SPEC> as RegisterValue<_>>::new(257)
    }
}
pub mod bfctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Male_SPEC;
    pub type Male = crate::EnumBitfieldStruct<u8, Male_SPEC>;
    impl Male {
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
    pub struct Sale_SPEC;
    pub type Sale = crate::EnumBitfieldStruct<u8, Sale_SPEC>;
    impl Sale {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scsyne_SPEC;
    pub type Scsyne = crate::EnumBitfieldStruct<u8, Scsyne_SPEC>;
    impl Scsyne {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Smbs_SPEC;
    pub type Smbs = crate::EnumBitfieldStruct<u8, Smbs_SPEC>;
    impl Smbs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fmpe_SPEC;
    pub type Fmpe = crate::EnumBitfieldStruct<u8, Fmpe_SPEC>;
    impl Fmpe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hsme_SPEC;
    pub type Hsme = crate::EnumBitfieldStruct<u8, Hsme_SPEC>;
    impl Hsme {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Svctl_SPEC;
impl crate::sealed::RegSpec for Svctl_SPEC {
    type DataType = u32;
}

pub type Svctl = crate::RegValueT<Svctl_SPEC>;

impl Svctl {
    #[inline(always)]
    pub fn gcae(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        svctl::Gcae,
        svctl::Gcae,
        Svctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            svctl::Gcae,
            svctl::Gcae,
            Svctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn hsmce(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        svctl::Hsmce,
        svctl::Hsmce,
        Svctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            svctl::Hsmce,
            svctl::Hsmce,
            Svctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dvide(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        svctl::Dvide,
        svctl::Dvide,
        Svctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            svctl::Dvide,
            svctl::Dvide,
            Svctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn hoae(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        svctl::Hoae,
        svctl::Hoae,
        Svctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            svctl::Hoae,
            svctl::Hoae,
            Svctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn svae0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        svctl::Svae0,
        svctl::Svae0,
        Svctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            svctl::Svae0,
            svctl::Svae0,
            Svctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn svae1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        svctl::Svae1,
        svctl::Svae1,
        Svctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            svctl::Svae1,
            svctl::Svae1,
            Svctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn svae2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        svctl::Svae2,
        svctl::Svae2,
        Svctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            svctl::Svae2,
            svctl::Svae2,
            Svctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Svctl {
    #[inline(always)]
    fn default() -> Svctl {
        <crate::RegValueT<Svctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod svctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gcae_SPEC;
    pub type Gcae = crate::EnumBitfieldStruct<u8, Gcae_SPEC>;
    impl Gcae {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hsmce_SPEC;
    pub type Hsmce = crate::EnumBitfieldStruct<u8, Hsmce_SPEC>;
    impl Hsmce {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvide_SPEC;
    pub type Dvide = crate::EnumBitfieldStruct<u8, Dvide_SPEC>;
    impl Dvide {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hoae_SPEC;
    pub type Hoae = crate::EnumBitfieldStruct<u8, Hoae_SPEC>;
    impl Hoae {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Svae0_SPEC;
    pub type Svae0 = crate::EnumBitfieldStruct<u8, Svae0_SPEC>;
    impl Svae0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Svae1_SPEC;
    pub type Svae1 = crate::EnumBitfieldStruct<u8, Svae1_SPEC>;
    impl Svae1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Svae2_SPEC;
    pub type Svae2 = crate::EnumBitfieldStruct<u8, Svae2_SPEC>;
    impl Svae2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Refckctl_SPEC;
impl crate::sealed::RegSpec for Refckctl_SPEC {
    type DataType = u32;
}

pub type Refckctl = crate::RegValueT<Refckctl_SPEC>;

impl Refckctl {
    #[inline(always)]
    pub fn irefcks(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        refckctl::Irefcks,
        refckctl::Irefcks,
        Refckctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            refckctl::Irefcks,
            refckctl::Irefcks,
            Refckctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Refckctl {
    #[inline(always)]
    fn default() -> Refckctl {
        <crate::RegValueT<Refckctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod refckctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irefcks_SPEC;
    pub type Irefcks = crate::EnumBitfieldStruct<u8, Irefcks_SPEC>;
    impl Irefcks {
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
pub struct Stdbr_SPEC;
impl crate::sealed::RegSpec for Stdbr_SPEC {
    type DataType = u32;
}

pub type Stdbr = crate::RegValueT<Stdbr_SPEC>;

impl Stdbr {
    #[inline(always)]
    pub fn sbrlo(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Stdbr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Stdbr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sbrho(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Stdbr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Stdbr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dsbrpo(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        stdbr::Dsbrpo,
        stdbr::Dsbrpo,
        Stdbr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            stdbr::Dsbrpo,
            stdbr::Dsbrpo,
            Stdbr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Stdbr {
    #[inline(always)]
    fn default() -> Stdbr {
        <crate::RegValueT<Stdbr_SPEC> as RegisterValue<_>>::new(65535)
    }
}
pub mod stdbr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dsbrpo_SPEC;
    pub type Dsbrpo = crate::EnumBitfieldStruct<u8, Dsbrpo_SPEC>;
    impl Dsbrpo {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Extbr_SPEC;
impl crate::sealed::RegSpec for Extbr_SPEC {
    type DataType = u32;
}

pub type Extbr = crate::RegValueT<Extbr_SPEC>;

impl Extbr {
    #[inline(always)]
    pub fn ebrlo(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Extbr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Extbr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ebrho(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Extbr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Extbr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Extbr {
    #[inline(always)]
    fn default() -> Extbr {
        <crate::RegValueT<Extbr_SPEC> as RegisterValue<_>>::new(65535)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bfrecdt_SPEC;
impl crate::sealed::RegSpec for Bfrecdt_SPEC {
    type DataType = u32;
}

pub type Bfrecdt = crate::RegValueT<Bfrecdt_SPEC>;

impl Bfrecdt {
    #[inline(always)]
    pub fn frecyc(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Bfrecdt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Bfrecdt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bfrecdt {
    #[inline(always)]
    fn default() -> Bfrecdt {
        <crate::RegValueT<Bfrecdt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Outctl_SPEC;
impl crate::sealed::RegSpec for Outctl_SPEC {
    type DataType = u32;
}

pub type Outctl = crate::RegValueT<Outctl_SPEC>;

impl Outctl {
    #[inline(always)]
    pub fn sdoc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        outctl::Sdoc,
        outctl::Sdoc,
        Outctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            outctl::Sdoc,
            outctl::Sdoc,
            Outctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn scoc(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        outctl::Scoc,
        outctl::Scoc,
        Outctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            outctl::Scoc,
            outctl::Scoc,
            Outctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn socwp(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        outctl::Socwp,
        outctl::Socwp,
        Outctl_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            outctl::Socwp,
            outctl::Socwp,
            Outctl_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn excyc(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        outctl::Excyc,
        outctl::Excyc,
        Outctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            outctl::Excyc,
            outctl::Excyc,
            Outctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sdod(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        outctl::Sdod,
        outctl::Sdod,
        Outctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            outctl::Sdod,
            outctl::Sdod,
            Outctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sdodcs(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        outctl::Sdodcs,
        outctl::Sdodcs,
        Outctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            outctl::Sdodcs,
            outctl::Sdodcs,
            Outctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Outctl {
    #[inline(always)]
    fn default() -> Outctl {
        <crate::RegValueT<Outctl_SPEC> as RegisterValue<_>>::new(3)
    }
}
pub mod outctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdoc_SPEC;
    pub type Sdoc = crate::EnumBitfieldStruct<u8, Sdoc_SPEC>;
    impl Sdoc {
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
    pub struct Socwp_SPEC;
    pub type Socwp = crate::EnumBitfieldStruct<u8, Socwp_SPEC>;
    impl Socwp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Excyc_SPEC;
    pub type Excyc = crate::EnumBitfieldStruct<u8, Excyc_SPEC>;
    impl Excyc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdod_SPEC;
    pub type Sdod = crate::EnumBitfieldStruct<u8, Sdod_SPEC>;
    impl Sdod {
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
    pub struct Sdodcs_SPEC;
    pub type Sdodcs = crate::EnumBitfieldStruct<u8, Sdodcs_SPEC>;
    impl Sdodcs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inctl_SPEC;
impl crate::sealed::RegSpec for Inctl_SPEC {
    type DataType = u32;
}

pub type Inctl = crate::RegValueT<Inctl_SPEC>;

impl Inctl {
    #[inline(always)]
    pub fn dnfs(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Inctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Inctl_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dnfe(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        inctl::Dnfe,
        inctl::Dnfe,
        Inctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            inctl::Dnfe,
            inctl::Dnfe,
            Inctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Inctl {
    #[inline(always)]
    fn default() -> Inctl {
        <crate::RegValueT<Inctl_SPEC> as RegisterValue<_>>::new(208)
    }
}
pub mod inctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dnfe_SPEC;
    pub type Dnfe = crate::EnumBitfieldStruct<u8, Dnfe_SPEC>;
    impl Dnfe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmoctl_SPEC;
impl crate::sealed::RegSpec for Tmoctl_SPEC {
    type DataType = u32;
}

pub type Tmoctl = crate::RegValueT<Tmoctl_SPEC>;

impl Tmoctl {
    #[inline(always)]
    pub fn todts(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        tmoctl::Todts,
        tmoctl::Todts,
        Tmoctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            tmoctl::Todts,
            tmoctl::Todts,
            Tmoctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tolctl(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        tmoctl::Tolctl,
        tmoctl::Tolctl,
        Tmoctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            tmoctl::Tolctl,
            tmoctl::Tolctl,
            Tmoctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tohctl(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        tmoctl::Tohctl,
        tmoctl::Tohctl,
        Tmoctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            tmoctl::Tohctl,
            tmoctl::Tohctl,
            Tmoctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tomds(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        tmoctl::Tomds,
        tmoctl::Tomds,
        Tmoctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            tmoctl::Tomds,
            tmoctl::Tomds,
            Tmoctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tmoctl {
    #[inline(always)]
    fn default() -> Tmoctl {
        <crate::RegValueT<Tmoctl_SPEC> as RegisterValue<_>>::new(48)
    }
}
pub mod tmoctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Todts_SPEC;
    pub type Todts = crate::EnumBitfieldStruct<u8, Todts_SPEC>;
    impl Todts {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tolctl_SPEC;
    pub type Tolctl = crate::EnumBitfieldStruct<u8, Tolctl_SPEC>;
    impl Tolctl {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tohctl_SPEC;
    pub type Tohctl = crate::EnumBitfieldStruct<u8, Tohctl_SPEC>;
    impl Tohctl {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tomds_SPEC;
    pub type Tomds = crate::EnumBitfieldStruct<u8, Tomds_SPEC>;
    impl Tomds {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ackctl_SPEC;
impl crate::sealed::RegSpec for Ackctl_SPEC {
    type DataType = u32;
}

pub type Ackctl = crate::RegValueT<Ackctl_SPEC>;

impl Ackctl {
    #[inline(always)]
    pub fn ackr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ackctl::Ackr,
        ackctl::Ackr,
        Ackctl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ackctl::Ackr,
            ackctl::Ackr,
            Ackctl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ackt(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ackctl::Ackt,
        ackctl::Ackt,
        Ackctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ackctl::Ackt,
            ackctl::Ackt,
            Ackctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn acktwp(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ackctl::Acktwp,
        ackctl::Acktwp,
        Ackctl_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ackctl::Acktwp,
            ackctl::Acktwp,
            Ackctl_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ackctl {
    #[inline(always)]
    fn default() -> Ackctl {
        <crate::RegValueT<Ackctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ackctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ackr_SPEC;
    pub type Ackr = crate::EnumBitfieldStruct<u8, Ackr_SPEC>;
    impl Ackr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ackt_SPEC;
    pub type Ackt = crate::EnumBitfieldStruct<u8, Ackt_SPEC>;
    impl Ackt {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Acktwp_SPEC;
    pub type Acktwp = crate::EnumBitfieldStruct<u8, Acktwp_SPEC>;
    impl Acktwp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scstrctl_SPEC;
impl crate::sealed::RegSpec for Scstrctl_SPEC {
    type DataType = u32;
}

pub type Scstrctl = crate::RegValueT<Scstrctl_SPEC>;

impl Scstrctl {
    #[inline(always)]
    pub fn acktwe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        scstrctl::Acktwe,
        scstrctl::Acktwe,
        Scstrctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            scstrctl::Acktwe,
            scstrctl::Acktwe,
            Scstrctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rwe(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        scstrctl::Rwe,
        scstrctl::Rwe,
        Scstrctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            scstrctl::Rwe,
            scstrctl::Rwe,
            Scstrctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Scstrctl {
    #[inline(always)]
    fn default() -> Scstrctl {
        <crate::RegValueT<Scstrctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod scstrctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Acktwe_SPEC;
    pub type Acktwe = crate::EnumBitfieldStruct<u8, Acktwe_SPEC>;
    impl Acktwe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rwe_SPEC;
    pub type Rwe = crate::EnumBitfieldStruct<u8, Rwe_SPEC>;
    impl Rwe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cndctl_SPEC;
impl crate::sealed::RegSpec for Cndctl_SPEC {
    type DataType = u32;
}

pub type Cndctl = crate::RegValueT<Cndctl_SPEC>;

impl Cndctl {
    #[inline(always)]
    pub fn stcnd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cndctl::Stcnd,
        cndctl::Stcnd,
        Cndctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cndctl::Stcnd,
            cndctl::Stcnd,
            Cndctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn srcnd(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cndctl::Srcnd,
        cndctl::Srcnd,
        Cndctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cndctl::Srcnd,
            cndctl::Srcnd,
            Cndctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spcnd(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cndctl::Spcnd,
        cndctl::Spcnd,
        Cndctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cndctl::Spcnd,
            cndctl::Spcnd,
            Cndctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cndctl {
    #[inline(always)]
    fn default() -> Cndctl {
        <crate::RegValueT<Cndctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cndctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stcnd_SPEC;
    pub type Stcnd = crate::EnumBitfieldStruct<u8, Stcnd_SPEC>;
    impl Stcnd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Srcnd_SPEC;
    pub type Srcnd = crate::EnumBitfieldStruct<u8, Srcnd_SPEC>;
    impl Srcnd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spcnd_SPEC;
    pub type Spcnd = crate::EnumBitfieldStruct<u8, Spcnd_SPEC>;
    impl Spcnd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ntdtbp0_SPEC;
impl crate::sealed::RegSpec for Ntdtbp0_SPEC {
    type DataType = u32;
}

pub type Ntdtbp0 = crate::RegValueT<Ntdtbp0_SPEC>;

impl NoBitfieldReg<Ntdtbp0_SPEC> for Ntdtbp0 {}
impl ::core::default::Default for Ntdtbp0 {
    #[inline(always)]
    fn default() -> Ntdtbp0 {
        <crate::RegValueT<Ntdtbp0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ntdtbp0By_SPEC;
impl crate::sealed::RegSpec for Ntdtbp0By_SPEC {
    type DataType = u8;
}

pub type Ntdtbp0By = crate::RegValueT<Ntdtbp0By_SPEC>;

impl NoBitfieldReg<Ntdtbp0By_SPEC> for Ntdtbp0By {}
impl ::core::default::Default for Ntdtbp0By {
    #[inline(always)]
    fn default() -> Ntdtbp0By {
        <crate::RegValueT<Ntdtbp0By_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bst_SPEC;
impl crate::sealed::RegSpec for Bst_SPEC {
    type DataType = u32;
}

pub type Bst = crate::RegValueT<Bst_SPEC>;

impl Bst {
    #[inline(always)]
    pub fn stcnddf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bst::Stcnddf,
        bst::Stcnddf,
        Bst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bst::Stcnddf,
            bst::Stcnddf,
            Bst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spcnddf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        bst::Spcnddf,
        bst::Spcnddf,
        Bst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            bst::Spcnddf,
            bst::Spcnddf,
            Bst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn nackdf(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        bst::Nackdf,
        bst::Nackdf,
        Bst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            bst::Nackdf,
            bst::Nackdf,
            Bst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tendf(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        bst::Tendf,
        bst::Tendf,
        Bst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            bst::Tendf,
            bst::Tendf,
            Bst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn alf(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, bst::Alf, bst::Alf, Bst_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,bst::Alf,bst::Alf,Bst_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn todf(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        bst::Todf,
        bst::Todf,
        Bst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            bst::Todf,
            bst::Todf,
            Bst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wucnddf(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        bst::Wucnddf,
        bst::Wucnddf,
        Bst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            bst::Wucnddf,
            bst::Wucnddf,
            Bst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bst {
    #[inline(always)]
    fn default() -> Bst {
        <crate::RegValueT<Bst_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bst {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stcnddf_SPEC;
    pub type Stcnddf = crate::EnumBitfieldStruct<u8, Stcnddf_SPEC>;
    impl Stcnddf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spcnddf_SPEC;
    pub type Spcnddf = crate::EnumBitfieldStruct<u8, Spcnddf_SPEC>;
    impl Spcnddf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nackdf_SPEC;
    pub type Nackdf = crate::EnumBitfieldStruct<u8, Nackdf_SPEC>;
    impl Nackdf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tendf_SPEC;
    pub type Tendf = crate::EnumBitfieldStruct<u8, Tendf_SPEC>;
    impl Tendf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Alf_SPEC;
    pub type Alf = crate::EnumBitfieldStruct<u8, Alf_SPEC>;
    impl Alf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Todf_SPEC;
    pub type Todf = crate::EnumBitfieldStruct<u8, Todf_SPEC>;
    impl Todf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wucnddf_SPEC;
    pub type Wucnddf = crate::EnumBitfieldStruct<u8, Wucnddf_SPEC>;
    impl Wucnddf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bste_SPEC;
impl crate::sealed::RegSpec for Bste_SPEC {
    type DataType = u32;
}

pub type Bste = crate::RegValueT<Bste_SPEC>;

impl Bste {
    #[inline(always)]
    pub fn stcndde(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bste::Stcndde,
        bste::Stcndde,
        Bste_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bste::Stcndde,
            bste::Stcndde,
            Bste_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spcndde(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        bste::Spcndde,
        bste::Spcndde,
        Bste_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            bste::Spcndde,
            bste::Spcndde,
            Bste_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn nackde(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        bste::Nackde,
        bste::Nackde,
        Bste_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            bste::Nackde,
            bste::Nackde,
            Bste_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tende(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        bste::Tende,
        bste::Tende,
        Bste_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            bste::Tende,
            bste::Tende,
            Bste_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ale(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        bste::Ale,
        bste::Ale,
        Bste_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            bste::Ale,
            bste::Ale,
            Bste_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tode(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        bste::Tode,
        bste::Tode,
        Bste_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            bste::Tode,
            bste::Tode,
            Bste_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wucndde(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        bste::Wucndde,
        bste::Wucndde,
        Bste_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            bste::Wucndde,
            bste::Wucndde,
            Bste_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bste {
    #[inline(always)]
    fn default() -> Bste {
        <crate::RegValueT<Bste_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bste {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stcndde_SPEC;
    pub type Stcndde = crate::EnumBitfieldStruct<u8, Stcndde_SPEC>;
    impl Stcndde {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spcndde_SPEC;
    pub type Spcndde = crate::EnumBitfieldStruct<u8, Spcndde_SPEC>;
    impl Spcndde {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nackde_SPEC;
    pub type Nackde = crate::EnumBitfieldStruct<u8, Nackde_SPEC>;
    impl Nackde {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tende_SPEC;
    pub type Tende = crate::EnumBitfieldStruct<u8, Tende_SPEC>;
    impl Tende {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ale_SPEC;
    pub type Ale = crate::EnumBitfieldStruct<u8, Ale_SPEC>;
    impl Ale {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tode_SPEC;
    pub type Tode = crate::EnumBitfieldStruct<u8, Tode_SPEC>;
    impl Tode {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wucndde_SPEC;
    pub type Wucndde = crate::EnumBitfieldStruct<u8, Wucndde_SPEC>;
    impl Wucndde {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bie_SPEC;
impl crate::sealed::RegSpec for Bie_SPEC {
    type DataType = u32;
}

pub type Bie = crate::RegValueT<Bie_SPEC>;

impl Bie {
    #[inline(always)]
    pub fn stcnddie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bie::Stcnddie,
        bie::Stcnddie,
        Bie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bie::Stcnddie,
            bie::Stcnddie,
            Bie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spcnddie(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        bie::Spcnddie,
        bie::Spcnddie,
        Bie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            bie::Spcnddie,
            bie::Spcnddie,
            Bie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn nackdie(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        bie::Nackdie,
        bie::Nackdie,
        Bie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            bie::Nackdie,
            bie::Nackdie,
            Bie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tendie(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        bie::Tendie,
        bie::Tendie,
        Bie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            bie::Tendie,
            bie::Tendie,
            Bie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn alie(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        bie::Alie,
        bie::Alie,
        Bie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            bie::Alie,
            bie::Alie,
            Bie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn todie(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        bie::Todie,
        bie::Todie,
        Bie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            bie::Todie,
            bie::Todie,
            Bie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wucnddie(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        bie::Wucnddie,
        bie::Wucnddie,
        Bie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            bie::Wucnddie,
            bie::Wucnddie,
            Bie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bie {
    #[inline(always)]
    fn default() -> Bie {
        <crate::RegValueT<Bie_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bie {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stcnddie_SPEC;
    pub type Stcnddie = crate::EnumBitfieldStruct<u8, Stcnddie_SPEC>;
    impl Stcnddie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spcnddie_SPEC;
    pub type Spcnddie = crate::EnumBitfieldStruct<u8, Spcnddie_SPEC>;
    impl Spcnddie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nackdie_SPEC;
    pub type Nackdie = crate::EnumBitfieldStruct<u8, Nackdie_SPEC>;
    impl Nackdie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tendie_SPEC;
    pub type Tendie = crate::EnumBitfieldStruct<u8, Tendie_SPEC>;
    impl Tendie {
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
    pub struct Todie_SPEC;
    pub type Todie = crate::EnumBitfieldStruct<u8, Todie_SPEC>;
    impl Todie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wucnddie_SPEC;
    pub type Wucnddie = crate::EnumBitfieldStruct<u8, Wucnddie_SPEC>;
    impl Wucnddie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bstfc_SPEC;
impl crate::sealed::RegSpec for Bstfc_SPEC {
    type DataType = u32;
}

pub type Bstfc = crate::RegValueT<Bstfc_SPEC>;

impl Bstfc {
    #[inline(always)]
    pub fn stcnddfc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bstfc::Stcnddfc,
        bstfc::Stcnddfc,
        Bstfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bstfc::Stcnddfc,
            bstfc::Stcnddfc,
            Bstfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spcnddfc(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        bstfc::Spcnddfc,
        bstfc::Spcnddfc,
        Bstfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            bstfc::Spcnddfc,
            bstfc::Spcnddfc,
            Bstfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn nackdfc(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        bstfc::Nackdfc,
        bstfc::Nackdfc,
        Bstfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            bstfc::Nackdfc,
            bstfc::Nackdfc,
            Bstfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tendfc(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        bstfc::Tendfc,
        bstfc::Tendfc,
        Bstfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            bstfc::Tendfc,
            bstfc::Tendfc,
            Bstfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn alfc(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        bstfc::Alfc,
        bstfc::Alfc,
        Bstfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            bstfc::Alfc,
            bstfc::Alfc,
            Bstfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn todfc(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        bstfc::Todfc,
        bstfc::Todfc,
        Bstfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            bstfc::Todfc,
            bstfc::Todfc,
            Bstfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wucnddfc(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        bstfc::Wucnddfc,
        bstfc::Wucnddfc,
        Bstfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            bstfc::Wucnddfc,
            bstfc::Wucnddfc,
            Bstfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bstfc {
    #[inline(always)]
    fn default() -> Bstfc {
        <crate::RegValueT<Bstfc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bstfc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stcnddfc_SPEC;
    pub type Stcnddfc = crate::EnumBitfieldStruct<u8, Stcnddfc_SPEC>;
    impl Stcnddfc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spcnddfc_SPEC;
    pub type Spcnddfc = crate::EnumBitfieldStruct<u8, Spcnddfc_SPEC>;
    impl Spcnddfc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nackdfc_SPEC;
    pub type Nackdfc = crate::EnumBitfieldStruct<u8, Nackdfc_SPEC>;
    impl Nackdfc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tendfc_SPEC;
    pub type Tendfc = crate::EnumBitfieldStruct<u8, Tendfc_SPEC>;
    impl Tendfc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Alfc_SPEC;
    pub type Alfc = crate::EnumBitfieldStruct<u8, Alfc_SPEC>;
    impl Alfc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Todfc_SPEC;
    pub type Todfc = crate::EnumBitfieldStruct<u8, Todfc_SPEC>;
    impl Todfc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wucnddfc_SPEC;
    pub type Wucnddfc = crate::EnumBitfieldStruct<u8, Wucnddfc_SPEC>;
    impl Wucnddfc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ntst_SPEC;
impl crate::sealed::RegSpec for Ntst_SPEC {
    type DataType = u32;
}

pub type Ntst = crate::RegValueT<Ntst_SPEC>;

impl Ntst {
    #[inline(always)]
    pub fn tdbef0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ntst::Tdbef0,
        ntst::Tdbef0,
        Ntst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ntst::Tdbef0,
            ntst::Tdbef0,
            Ntst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rdbff0(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ntst::Rdbff0,
        ntst::Rdbff0,
        Ntst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ntst::Rdbff0,
            ntst::Rdbff0,
            Ntst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ntst {
    #[inline(always)]
    fn default() -> Ntst {
        <crate::RegValueT<Ntst_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ntst {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdbef0_SPEC;
    pub type Tdbef0 = crate::EnumBitfieldStruct<u8, Tdbef0_SPEC>;
    impl Tdbef0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdbff0_SPEC;
    pub type Rdbff0 = crate::EnumBitfieldStruct<u8, Rdbff0_SPEC>;
    impl Rdbff0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ntste_SPEC;
impl crate::sealed::RegSpec for Ntste_SPEC {
    type DataType = u32;
}

pub type Ntste = crate::RegValueT<Ntste_SPEC>;

impl Ntste {
    #[inline(always)]
    pub fn tdbee0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ntste::Tdbee0,
        ntste::Tdbee0,
        Ntste_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ntste::Tdbee0,
            ntste::Tdbee0,
            Ntste_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rdbfe0(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ntste::Rdbfe0,
        ntste::Rdbfe0,
        Ntste_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ntste::Rdbfe0,
            ntste::Rdbfe0,
            Ntste_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ntste {
    #[inline(always)]
    fn default() -> Ntste {
        <crate::RegValueT<Ntste_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ntste {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdbee0_SPEC;
    pub type Tdbee0 = crate::EnumBitfieldStruct<u8, Tdbee0_SPEC>;
    impl Tdbee0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdbfe0_SPEC;
    pub type Rdbfe0 = crate::EnumBitfieldStruct<u8, Rdbfe0_SPEC>;
    impl Rdbfe0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ntie_SPEC;
impl crate::sealed::RegSpec for Ntie_SPEC {
    type DataType = u32;
}

pub type Ntie = crate::RegValueT<Ntie_SPEC>;

impl Ntie {
    #[inline(always)]
    pub fn tdbeie0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ntie::Tdbeie0,
        ntie::Tdbeie0,
        Ntie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ntie::Tdbeie0,
            ntie::Tdbeie0,
            Ntie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rdbfie0(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ntie::Rdbfie0,
        ntie::Rdbfie0,
        Ntie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ntie::Rdbfie0,
            ntie::Rdbfie0,
            Ntie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ntie {
    #[inline(always)]
    fn default() -> Ntie {
        <crate::RegValueT<Ntie_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ntie {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdbeie0_SPEC;
    pub type Tdbeie0 = crate::EnumBitfieldStruct<u8, Tdbeie0_SPEC>;
    impl Tdbeie0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdbfie0_SPEC;
    pub type Rdbfie0 = crate::EnumBitfieldStruct<u8, Rdbfie0_SPEC>;
    impl Rdbfie0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ntstfc_SPEC;
impl crate::sealed::RegSpec for Ntstfc_SPEC {
    type DataType = u32;
}

pub type Ntstfc = crate::RegValueT<Ntstfc_SPEC>;

impl Ntstfc {
    #[inline(always)]
    pub fn tdbefc0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ntstfc::Tdbefc0,
        ntstfc::Tdbefc0,
        Ntstfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ntstfc::Tdbefc0,
            ntstfc::Tdbefc0,
            Ntstfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rdbffc0(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ntstfc::Rdbffc0,
        ntstfc::Rdbffc0,
        Ntstfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ntstfc::Rdbffc0,
            ntstfc::Rdbffc0,
            Ntstfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ntstfc {
    #[inline(always)]
    fn default() -> Ntstfc {
        <crate::RegValueT<Ntstfc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ntstfc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdbefc0_SPEC;
    pub type Tdbefc0 = crate::EnumBitfieldStruct<u8, Tdbefc0_SPEC>;
    impl Tdbefc0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdbffc0_SPEC;
    pub type Rdbffc0 = crate::EnumBitfieldStruct<u8, Rdbffc0_SPEC>;
    impl Rdbffc0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcst_SPEC;
impl crate::sealed::RegSpec for Bcst_SPEC {
    type DataType = u32;
}

pub type Bcst = crate::RegValueT<Bcst_SPEC>;

impl Bcst {
    #[inline(always)]
    pub fn bfref(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bcst::Bfref,
        bcst::Bfref,
        Bcst_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bcst::Bfref,
            bcst::Bfref,
            Bcst_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bcst {
    #[inline(always)]
    fn default() -> Bcst {
        <crate::RegValueT<Bcst_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bcst {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bfref_SPEC;
    pub type Bfref = crate::EnumBitfieldStruct<u8, Bfref_SPEC>;
    impl Bfref {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Svst_SPEC;
impl crate::sealed::RegSpec for Svst_SPEC {
    type DataType = u32;
}

pub type Svst = crate::RegValueT<Svst_SPEC>;

impl Svst {
    #[inline(always)]
    pub fn gcaf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        svst::Gcaf,
        svst::Gcaf,
        Svst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            svst::Gcaf,
            svst::Gcaf,
            Svst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn hsmcf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        svst::Hsmcf,
        svst::Hsmcf,
        Svst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            svst::Hsmcf,
            svst::Hsmcf,
            Svst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dvidf(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        svst::Dvidf,
        svst::Dvidf,
        Svst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            svst::Dvidf,
            svst::Dvidf,
            Svst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn hoaf(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        svst::Hoaf,
        svst::Hoaf,
        Svst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            svst::Hoaf,
            svst::Hoaf,
            Svst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn svaf0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        svst::Svaf0,
        svst::Svaf0,
        Svst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            svst::Svaf0,
            svst::Svaf0,
            Svst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn svaf1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        svst::Svaf1,
        svst::Svaf1,
        Svst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            svst::Svaf1,
            svst::Svaf1,
            Svst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn svaf2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        svst::Svaf2,
        svst::Svaf2,
        Svst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            svst::Svaf2,
            svst::Svaf2,
            Svst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Svst {
    #[inline(always)]
    fn default() -> Svst {
        <crate::RegValueT<Svst_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod svst {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gcaf_SPEC;
    pub type Gcaf = crate::EnumBitfieldStruct<u8, Gcaf_SPEC>;
    impl Gcaf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hsmcf_SPEC;
    pub type Hsmcf = crate::EnumBitfieldStruct<u8, Hsmcf_SPEC>;
    impl Hsmcf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvidf_SPEC;
    pub type Dvidf = crate::EnumBitfieldStruct<u8, Dvidf_SPEC>;
    impl Dvidf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hoaf_SPEC;
    pub type Hoaf = crate::EnumBitfieldStruct<u8, Hoaf_SPEC>;
    impl Hoaf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Svaf0_SPEC;
    pub type Svaf0 = crate::EnumBitfieldStruct<u8, Svaf0_SPEC>;
    impl Svaf0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Svaf1_SPEC;
    pub type Svaf1 = crate::EnumBitfieldStruct<u8, Svaf1_SPEC>;
    impl Svaf1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Svaf2_SPEC;
    pub type Svaf2 = crate::EnumBitfieldStruct<u8, Svaf2_SPEC>;
    impl Svaf2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdatbas_SPEC;
impl crate::sealed::RegSpec for Sdatbas_SPEC {
    type DataType = u32;
}

pub type Sdatbas = crate::RegValueT<Sdatbas_SPEC>;

impl Sdatbas {
    #[inline(always)]
    pub fn sdstad(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Sdatbas_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Sdatbas_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sdadls(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        sdatbas::Sdadls,
        sdatbas::Sdadls,
        Sdatbas_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            sdatbas::Sdadls,
            sdatbas::Sdadls,
            Sdatbas_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sdatbas {
    #[inline(always)]
    fn default() -> Sdatbas {
        <crate::RegValueT<Sdatbas_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdatbas {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdadls_SPEC;
    pub type Sdadls = crate::EnumBitfieldStruct<u8, Sdadls_SPEC>;
    impl Sdadls {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Svdvad_SPEC;
impl crate::sealed::RegSpec for Svdvad_SPEC {
    type DataType = u32;
}

pub type Svdvad = crate::RegValueT<Svdvad_SPEC>;

impl Svdvad {
    #[inline(always)]
    pub fn svad(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, u16, Svdvad_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x3ff,1,0,u16,u16,Svdvad_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sadlg(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        svdvad::Sadlg,
        svdvad::Sadlg,
        Svdvad_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            svdvad::Sadlg,
            svdvad::Sadlg,
            Svdvad_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sstadv(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        svdvad::Sstadv,
        svdvad::Sstadv,
        Svdvad_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            svdvad::Sstadv,
            svdvad::Sstadv,
            Svdvad_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Svdvad {
    #[inline(always)]
    fn default() -> Svdvad {
        <crate::RegValueT<Svdvad_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod svdvad {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadlg_SPEC;
    pub type Sadlg = crate::EnumBitfieldStruct<u8, Sadlg_SPEC>;
    impl Sadlg {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sstadv_SPEC;
    pub type Sstadv = crate::EnumBitfieldStruct<u8, Sstadv_SPEC>;
    impl Sstadv {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bitcnt_SPEC;
impl crate::sealed::RegSpec for Bitcnt_SPEC {
    type DataType = u32;
}

pub type Bitcnt = crate::RegValueT<Bitcnt_SPEC>;

impl Bitcnt {
    #[inline(always)]
    pub fn bcnt(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Bitcnt_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Bitcnt_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Bitcnt {
    #[inline(always)]
    fn default() -> Bitcnt {
        <crate::RegValueT<Bitcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prstdbg_SPEC;
impl crate::sealed::RegSpec for Prstdbg_SPEC {
    type DataType = u32;
}

pub type Prstdbg = crate::RegValueT<Prstdbg_SPEC>;

impl Prstdbg {
    #[inline(always)]
    pub fn scilv(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Prstdbg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Prstdbg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn sdilv(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Prstdbg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Prstdbg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn scolv(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        prstdbg::Scolv,
        prstdbg::Scolv,
        Prstdbg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            prstdbg::Scolv,
            prstdbg::Scolv,
            Prstdbg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sdolv(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        prstdbg::Sdolv,
        prstdbg::Sdolv,
        Prstdbg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            prstdbg::Sdolv,
            prstdbg::Sdolv,
            Prstdbg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Prstdbg {
    #[inline(always)]
    fn default() -> Prstdbg {
        <crate::RegValueT<Prstdbg_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod prstdbg {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scolv_SPEC;
    pub type Scolv = crate::EnumBitfieldStruct<u8, Scolv_SPEC>;
    impl Scolv {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdolv_SPEC;
    pub type Sdolv = crate::EnumBitfieldStruct<u8, Sdolv_SPEC>;
    impl Sdolv {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
