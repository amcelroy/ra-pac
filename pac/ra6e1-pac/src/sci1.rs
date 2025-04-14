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
// Generated from SVD 1.20.00, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:19:27 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Serial Communication Interface 1"]
unsafe impl ::core::marker::Send for super::Sci1 {}
unsafe impl ::core::marker::Sync for super::Sci1 {}
impl super::Sci1 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn smr(&self) -> &'static crate::common::Reg<self::Smr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Smr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn smr_smci(
        &self,
    ) -> &'static crate::common::Reg<self::SmrSmci_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SmrSmci_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn brr(&self) -> &'static crate::common::Reg<self::Brr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Brr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1usize),
            )
        }
    }

    #[inline(always)]
    pub const fn scr(&self) -> &'static crate::common::Reg<self::Scr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Scr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[inline(always)]
    pub const fn scr_smci(
        &self,
    ) -> &'static crate::common::Reg<self::ScrSmci_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ScrSmci_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tdr(&self) -> &'static crate::common::Reg<self::Tdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ssr(&self) -> &'static crate::common::Reg<self::Ssr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ssr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ssr_fifo(
        &self,
    ) -> &'static crate::common::Reg<self::SsrFifo_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SsrFifo_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ssr_manc(
        &self,
    ) -> &'static crate::common::Reg<self::SsrManc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SsrManc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ssr_smci(
        &self,
    ) -> &'static crate::common::Reg<self::SsrSmci_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SsrSmci_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rdr(&self) -> &'static crate::common::Reg<self::Rdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(5usize),
            )
        }
    }

    #[inline(always)]
    pub const fn scmr(&self) -> &'static crate::common::Reg<self::Scmr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Scmr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[inline(always)]
    pub const fn snfr(&self) -> &'static crate::common::Reg<self::Snfr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Snfr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn simr1(&self) -> &'static crate::common::Reg<self::Simr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Simr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(9usize),
            )
        }
    }

    #[inline(always)]
    pub const fn simr2(&self) -> &'static crate::common::Reg<self::Simr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Simr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[inline(always)]
    pub const fn simr3(&self) -> &'static crate::common::Reg<self::Simr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Simr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(11usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sisr(&self) -> &'static crate::common::Reg<self::Sisr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sisr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn spmr(&self) -> &'static crate::common::Reg<self::Spmr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spmr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(13usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdrhl(&self) -> &'static crate::common::Reg<self::Ftdrhl_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ftdrhl_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tdrhl(&self) -> &'static crate::common::Reg<self::Tdrhl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tdrhl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdrh(&self) -> &'static crate::common::Reg<self::Ftdrh_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ftdrh_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdrl(&self) -> &'static crate::common::Reg<self::Ftdrl_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ftdrl_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(15usize),
            )
        }
    }

    #[inline(always)]
    pub const fn frdrhl(&self) -> &'static crate::common::Reg<self::Frdrhl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Frdrhl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rdrhl(&self) -> &'static crate::common::Reg<self::Rdrhl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rdrhl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn frdrh(&self) -> &'static crate::common::Reg<self::Frdrh_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Frdrh_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn frdrl(&self) -> &'static crate::common::Reg<self::Frdrl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Frdrl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(17usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mddr(&self) -> &'static crate::common::Reg<self::Mddr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mddr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dccr(&self) -> &'static crate::common::Reg<self::Dccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(19usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fcr(&self) -> &'static crate::common::Reg<self::Fcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fdr(&self) -> &'static crate::common::Reg<self::Fdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Fdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lsr(&self) -> &'static crate::common::Reg<self::Lsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Lsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cdr(&self) -> &'static crate::common::Reg<self::Cdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(26usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sptr(&self) -> &'static crate::common::Reg<self::Sptr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sptr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn actr(&self) -> &'static crate::common::Reg<self::Actr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Actr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(29usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mmr(&self) -> &'static crate::common::Reg<self::Mmr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tmpr(&self) -> &'static crate::common::Reg<self::Tmpr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmpr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(34usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rmpr(&self) -> &'static crate::common::Reg<self::Rmpr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rmpr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(35usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mesr(&self) -> &'static crate::common::Reg<self::Mesr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mesr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mecr(&self) -> &'static crate::common::Reg<self::Mecr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mecr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(37usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smr_SPEC;
impl crate::sealed::RegSpec for Smr_SPEC {
    type DataType = u8;
}

pub type Smr = crate::RegValueT<Smr_SPEC>;

impl Smr {
    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, smr::Cks, smr::Cks, Smr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,smr::Cks,smr::Cks,Smr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mp(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, smr::Mp, smr::Mp, Smr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,smr::Mp,smr::Mp,Smr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn stop(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, smr::Stop, smr::Stop, Smr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            smr::Stop,
            smr::Stop,
            Smr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pm(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, smr::Pm, smr::Pm, Smr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,smr::Pm,smr::Pm,Smr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pe(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, smr::Pe, smr::Pe, Smr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,smr::Pe,smr::Pe,Smr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn chr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, smr::Chr, smr::Chr, Smr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,smr::Chr,smr::Chr,Smr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cm(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, smr::Cm, smr::Cm, Smr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,smr::Cm,smr::Cm,Smr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Smr {
    #[inline(always)]
    fn default() -> Smr {
        <crate::RegValueT<Smr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod smr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mp_SPEC;
    pub type Mp = crate::EnumBitfieldStruct<u8, Mp_SPEC>;
    impl Mp {
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
    pub struct Pm_SPEC;
    pub type Pm = crate::EnumBitfieldStruct<u8, Pm_SPEC>;
    impl Pm {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pe_SPEC;
    pub type Pe = crate::EnumBitfieldStruct<u8, Pe_SPEC>;
    impl Pe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chr_SPEC;
    pub type Chr = crate::EnumBitfieldStruct<u8, Chr_SPEC>;
    impl Chr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cm_SPEC;
    pub type Cm = crate::EnumBitfieldStruct<u8, Cm_SPEC>;
    impl Cm {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmrSmci_SPEC;
impl crate::sealed::RegSpec for SmrSmci_SPEC {
    type DataType = u8;
}

pub type SmrSmci = crate::RegValueT<SmrSmci_SPEC>;

impl SmrSmci {
    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        smr_smci::Cks,
        smr_smci::Cks,
        SmrSmci_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            smr_smci::Cks,
            smr_smci::Cks,
            SmrSmci_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bcp(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, SmrSmci_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,SmrSmci_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pm(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        smr_smci::Pm,
        smr_smci::Pm,
        SmrSmci_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            smr_smci::Pm,
            smr_smci::Pm,
            SmrSmci_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pe(self) -> crate::common::RegisterFieldBool<5, 1, 0, SmrSmci_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, SmrSmci_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn blk(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        smr_smci::Blk,
        smr_smci::Blk,
        SmrSmci_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            smr_smci::Blk,
            smr_smci::Blk,
            SmrSmci_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gm(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        smr_smci::Gm,
        smr_smci::Gm,
        SmrSmci_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            smr_smci::Gm,
            smr_smci::Gm,
            SmrSmci_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SmrSmci {
    #[inline(always)]
    fn default() -> SmrSmci {
        <crate::RegValueT<SmrSmci_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod smr_smci {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pm_SPEC;
    pub type Pm = crate::EnumBitfieldStruct<u8, Pm_SPEC>;
    impl Pm {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Blk_SPEC;
    pub type Blk = crate::EnumBitfieldStruct<u8, Blk_SPEC>;
    impl Blk {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gm_SPEC;
    pub type Gm = crate::EnumBitfieldStruct<u8, Gm_SPEC>;
    impl Gm {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Brr_SPEC;
impl crate::sealed::RegSpec for Brr_SPEC {
    type DataType = u8;
}

pub type Brr = crate::RegValueT<Brr_SPEC>;

impl NoBitfieldReg<Brr_SPEC> for Brr {}
impl ::core::default::Default for Brr {
    #[inline(always)]
    fn default() -> Brr {
        <crate::RegValueT<Brr_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr_SPEC;
impl crate::sealed::RegSpec for Scr_SPEC {
    type DataType = u8;
}

pub type Scr = crate::RegValueT<Scr_SPEC>;

impl Scr {
    #[inline(always)]
    pub fn cke(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, scr::Cke, scr::Cke, Scr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,scr::Cke,scr::Cke,Scr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn teie(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, scr::Teie, scr::Teie, Scr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            scr::Teie,
            scr::Teie,
            Scr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mpie(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, scr::Mpie, scr::Mpie, Scr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            scr::Mpie,
            scr::Mpie,
            Scr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn re(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, scr::Re, scr::Re, Scr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,scr::Re,scr::Re,Scr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn te(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, scr::Te, scr::Te, Scr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,scr::Te,scr::Te,Scr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rie(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, scr::Rie, scr::Rie, Scr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,scr::Rie,scr::Rie,Scr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tie(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, scr::Tie, scr::Tie, Scr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,scr::Tie,scr::Tie,Scr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Scr {
    #[inline(always)]
    fn default() -> Scr {
        <crate::RegValueT<Scr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod scr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cke_SPEC;
    pub type Cke = crate::EnumBitfieldStruct<u8, Cke_SPEC>;
    impl Cke {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Teie_SPEC;
    pub type Teie = crate::EnumBitfieldStruct<u8, Teie_SPEC>;
    impl Teie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpie_SPEC;
    pub type Mpie = crate::EnumBitfieldStruct<u8, Mpie_SPEC>;
    impl Mpie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Re_SPEC;
    pub type Re = crate::EnumBitfieldStruct<u8, Re_SPEC>;
    impl Re {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Te_SPEC;
    pub type Te = crate::EnumBitfieldStruct<u8, Te_SPEC>;
    impl Te {
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
    pub struct Tie_SPEC;
    pub type Tie = crate::EnumBitfieldStruct<u8, Tie_SPEC>;
    impl Tie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ScrSmci_SPEC;
impl crate::sealed::RegSpec for ScrSmci_SPEC {
    type DataType = u8;
}

pub type ScrSmci = crate::RegValueT<ScrSmci_SPEC>;

impl ScrSmci {
    #[inline(always)]
    pub fn cke(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        scr_smci::Cke,
        scr_smci::Cke,
        ScrSmci_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            scr_smci::Cke,
            scr_smci::Cke,
            ScrSmci_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn teie(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ScrSmci_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, ScrSmci_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn mpie(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ScrSmci_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, ScrSmci_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn re(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        scr_smci::Re,
        scr_smci::Re,
        ScrSmci_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            scr_smci::Re,
            scr_smci::Re,
            ScrSmci_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn te(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        scr_smci::Te,
        scr_smci::Te,
        ScrSmci_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            scr_smci::Te,
            scr_smci::Te,
            ScrSmci_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rie(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        scr_smci::Rie,
        scr_smci::Rie,
        ScrSmci_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            scr_smci::Rie,
            scr_smci::Rie,
            ScrSmci_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tie(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        scr_smci::Tie,
        scr_smci::Tie,
        ScrSmci_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            scr_smci::Tie,
            scr_smci::Tie,
            ScrSmci_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ScrSmci {
    #[inline(always)]
    fn default() -> ScrSmci {
        <crate::RegValueT<ScrSmci_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod scr_smci {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cke_SPEC;
    pub type Cke = crate::EnumBitfieldStruct<u8, Cke_SPEC>;
    impl Cke {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Re_SPEC;
    pub type Re = crate::EnumBitfieldStruct<u8, Re_SPEC>;
    impl Re {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Te_SPEC;
    pub type Te = crate::EnumBitfieldStruct<u8, Te_SPEC>;
    impl Te {
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
    pub struct Tie_SPEC;
    pub type Tie = crate::EnumBitfieldStruct<u8, Tie_SPEC>;
    impl Tie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdr_SPEC;
impl crate::sealed::RegSpec for Tdr_SPEC {
    type DataType = u8;
}

pub type Tdr = crate::RegValueT<Tdr_SPEC>;

impl NoBitfieldReg<Tdr_SPEC> for Tdr {}
impl ::core::default::Default for Tdr {
    #[inline(always)]
    fn default() -> Tdr {
        <crate::RegValueT<Tdr_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssr_SPEC;
impl crate::sealed::RegSpec for Ssr_SPEC {
    type DataType = u8;
}

pub type Ssr = crate::RegValueT<Ssr_SPEC>;

impl Ssr {
    #[inline(always)]
    pub fn mpbt(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ssr::Mpbt, ssr::Mpbt, Ssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ssr::Mpbt,
            ssr::Mpbt,
            Ssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mpb(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ssr::Mpb, ssr::Mpb, Ssr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,ssr::Mpb,ssr::Mpb,Ssr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tend(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, ssr::Tend, ssr::Tend, Ssr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x1,1,0,ssr::Tend,ssr::Tend,Ssr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn per(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, ssr::Per, ssr::Per, Ssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,ssr::Per,ssr::Per,Ssr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn fer(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, ssr::Fer, ssr::Fer, Ssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,ssr::Fer,ssr::Fer,Ssr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn orer(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, ssr::Orer, ssr::Orer, Ssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ssr::Orer,
            ssr::Orer,
            Ssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rdrf(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, ssr::Rdrf, ssr::Rdrf, Ssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ssr::Rdrf,
            ssr::Rdrf,
            Ssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tdre(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, ssr::Tdre, ssr::Tdre, Ssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ssr::Tdre,
            ssr::Tdre,
            Ssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ssr {
    #[inline(always)]
    fn default() -> Ssr {
        <crate::RegValueT<Ssr_SPEC> as RegisterValue<_>>::new(132)
    }
}
pub mod ssr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpbt_SPEC;
    pub type Mpbt = crate::EnumBitfieldStruct<u8, Mpbt_SPEC>;
    impl Mpbt {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpb_SPEC;
    pub type Mpb = crate::EnumBitfieldStruct<u8, Mpb_SPEC>;
    impl Mpb {
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
    pub struct Per_SPEC;
    pub type Per = crate::EnumBitfieldStruct<u8, Per_SPEC>;
    impl Per {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fer_SPEC;
    pub type Fer = crate::EnumBitfieldStruct<u8, Fer_SPEC>;
    impl Fer {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Orer_SPEC;
    pub type Orer = crate::EnumBitfieldStruct<u8, Orer_SPEC>;
    impl Orer {
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
    pub struct Tdre_SPEC;
    pub type Tdre = crate::EnumBitfieldStruct<u8, Tdre_SPEC>;
    impl Tdre {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SsrFifo_SPEC;
impl crate::sealed::RegSpec for SsrFifo_SPEC {
    type DataType = u8;
}

pub type SsrFifo = crate::RegValueT<SsrFifo_SPEC>;

impl SsrFifo {
    #[inline(always)]
    pub fn dr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ssr_fifo::Dr,
        ssr_fifo::Dr,
        SsrFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ssr_fifo::Dr,
            ssr_fifo::Dr,
            SsrFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tend(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ssr_fifo::Tend,
        ssr_fifo::Tend,
        SsrFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ssr_fifo::Tend,
            ssr_fifo::Tend,
            SsrFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn per(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ssr_fifo::Per,
        ssr_fifo::Per,
        SsrFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ssr_fifo::Per,
            ssr_fifo::Per,
            SsrFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fer(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ssr_fifo::Fer,
        ssr_fifo::Fer,
        SsrFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ssr_fifo::Fer,
            ssr_fifo::Fer,
            SsrFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn orer(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ssr_fifo::Orer,
        ssr_fifo::Orer,
        SsrFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ssr_fifo::Orer,
            ssr_fifo::Orer,
            SsrFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rdf(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ssr_fifo::Rdf,
        ssr_fifo::Rdf,
        SsrFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ssr_fifo::Rdf,
            ssr_fifo::Rdf,
            SsrFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tdfe(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ssr_fifo::Tdfe,
        ssr_fifo::Tdfe,
        SsrFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ssr_fifo::Tdfe,
            ssr_fifo::Tdfe,
            SsrFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SsrFifo {
    #[inline(always)]
    fn default() -> SsrFifo {
        <crate::RegValueT<SsrFifo_SPEC> as RegisterValue<_>>::new(128)
    }
}
pub mod ssr_fifo {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dr_SPEC;
    pub type Dr = crate::EnumBitfieldStruct<u8, Dr_SPEC>;
    impl Dr {
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
    pub struct Per_SPEC;
    pub type Per = crate::EnumBitfieldStruct<u8, Per_SPEC>;
    impl Per {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fer_SPEC;
    pub type Fer = crate::EnumBitfieldStruct<u8, Fer_SPEC>;
    impl Fer {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Orer_SPEC;
    pub type Orer = crate::EnumBitfieldStruct<u8, Orer_SPEC>;
    impl Orer {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdf_SPEC;
    pub type Rdf = crate::EnumBitfieldStruct<u8, Rdf_SPEC>;
    impl Rdf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdfe_SPEC;
    pub type Tdfe = crate::EnumBitfieldStruct<u8, Tdfe_SPEC>;
    impl Tdfe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SsrManc_SPEC;
impl crate::sealed::RegSpec for SsrManc_SPEC {
    type DataType = u8;
}

pub type SsrManc = crate::RegValueT<SsrManc_SPEC>;

impl SsrManc {
    #[inline(always)]
    pub fn mer(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ssr_manc::Mer,
        ssr_manc::Mer,
        SsrManc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ssr_manc::Mer,
            ssr_manc::Mer,
            SsrManc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mpb(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ssr_manc::Mpb,
        ssr_manc::Mpb,
        SsrManc_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ssr_manc::Mpb,
            ssr_manc::Mpb,
            SsrManc_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tend(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ssr_manc::Tend,
        ssr_manc::Tend,
        SsrManc_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ssr_manc::Tend,
            ssr_manc::Tend,
            SsrManc_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn per(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ssr_manc::Per,
        ssr_manc::Per,
        SsrManc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ssr_manc::Per,
            ssr_manc::Per,
            SsrManc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fer(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ssr_manc::Fer,
        ssr_manc::Fer,
        SsrManc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ssr_manc::Fer,
            ssr_manc::Fer,
            SsrManc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn orer(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ssr_manc::Orer,
        ssr_manc::Orer,
        SsrManc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ssr_manc::Orer,
            ssr_manc::Orer,
            SsrManc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rdrf(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ssr_manc::Rdrf,
        ssr_manc::Rdrf,
        SsrManc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ssr_manc::Rdrf,
            ssr_manc::Rdrf,
            SsrManc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tdre(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ssr_manc::Tdre,
        ssr_manc::Tdre,
        SsrManc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ssr_manc::Tdre,
            ssr_manc::Tdre,
            SsrManc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SsrManc {
    #[inline(always)]
    fn default() -> SsrManc {
        <crate::RegValueT<SsrManc_SPEC> as RegisterValue<_>>::new(132)
    }
}
pub mod ssr_manc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mer_SPEC;
    pub type Mer = crate::EnumBitfieldStruct<u8, Mer_SPEC>;
    impl Mer {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpb_SPEC;
    pub type Mpb = crate::EnumBitfieldStruct<u8, Mpb_SPEC>;
    impl Mpb {
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
    pub struct Per_SPEC;
    pub type Per = crate::EnumBitfieldStruct<u8, Per_SPEC>;
    impl Per {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fer_SPEC;
    pub type Fer = crate::EnumBitfieldStruct<u8, Fer_SPEC>;
    impl Fer {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Orer_SPEC;
    pub type Orer = crate::EnumBitfieldStruct<u8, Orer_SPEC>;
    impl Orer {
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
    pub struct Tdre_SPEC;
    pub type Tdre = crate::EnumBitfieldStruct<u8, Tdre_SPEC>;
    impl Tdre {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SsrSmci_SPEC;
impl crate::sealed::RegSpec for SsrSmci_SPEC {
    type DataType = u8;
}

pub type SsrSmci = crate::RegValueT<SsrSmci_SPEC>;

impl SsrSmci {
    #[inline(always)]
    pub fn mpbt(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SsrSmci_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, SsrSmci_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn mpb(self) -> crate::common::RegisterFieldBool<1, 1, 0, SsrSmci_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, SsrSmci_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn tend(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ssr_smci::Tend,
        ssr_smci::Tend,
        SsrSmci_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ssr_smci::Tend,
            ssr_smci::Tend,
            SsrSmci_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn per(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ssr_smci::Per,
        ssr_smci::Per,
        SsrSmci_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ssr_smci::Per,
            ssr_smci::Per,
            SsrSmci_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ers(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ssr_smci::Ers,
        ssr_smci::Ers,
        SsrSmci_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ssr_smci::Ers,
            ssr_smci::Ers,
            SsrSmci_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn orer(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ssr_smci::Orer,
        ssr_smci::Orer,
        SsrSmci_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ssr_smci::Orer,
            ssr_smci::Orer,
            SsrSmci_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rdrf(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ssr_smci::Rdrf,
        ssr_smci::Rdrf,
        SsrSmci_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ssr_smci::Rdrf,
            ssr_smci::Rdrf,
            SsrSmci_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tdre(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ssr_smci::Tdre,
        ssr_smci::Tdre,
        SsrSmci_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ssr_smci::Tdre,
            ssr_smci::Tdre,
            SsrSmci_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SsrSmci {
    #[inline(always)]
    fn default() -> SsrSmci {
        <crate::RegValueT<SsrSmci_SPEC> as RegisterValue<_>>::new(132)
    }
}
pub mod ssr_smci {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tend_SPEC;
    pub type Tend = crate::EnumBitfieldStruct<u8, Tend_SPEC>;
    impl Tend {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Per_SPEC;
    pub type Per = crate::EnumBitfieldStruct<u8, Per_SPEC>;
    impl Per {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ers_SPEC;
    pub type Ers = crate::EnumBitfieldStruct<u8, Ers_SPEC>;
    impl Ers {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Orer_SPEC;
    pub type Orer = crate::EnumBitfieldStruct<u8, Orer_SPEC>;
    impl Orer {
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
    pub struct Tdre_SPEC;
    pub type Tdre = crate::EnumBitfieldStruct<u8, Tdre_SPEC>;
    impl Tdre {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdr_SPEC;
impl crate::sealed::RegSpec for Rdr_SPEC {
    type DataType = u8;
}

pub type Rdr = crate::RegValueT<Rdr_SPEC>;

impl NoBitfieldReg<Rdr_SPEC> for Rdr {}
impl ::core::default::Default for Rdr {
    #[inline(always)]
    fn default() -> Rdr {
        <crate::RegValueT<Rdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scmr_SPEC;
impl crate::sealed::RegSpec for Scmr_SPEC {
    type DataType = u8;
}

pub type Scmr = crate::RegValueT<Scmr_SPEC>;

impl Scmr {
    #[inline(always)]
    pub fn smif(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        scmr::Smif,
        scmr::Smif,
        Scmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            scmr::Smif,
            scmr::Smif,
            Scmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sinv(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        scmr::Sinv,
        scmr::Sinv,
        Scmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            scmr::Sinv,
            scmr::Sinv,
            Scmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sdir(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        scmr::Sdir,
        scmr::Sdir,
        Scmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            scmr::Sdir,
            scmr::Sdir,
            Scmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chr1(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        scmr::Chr1,
        scmr::Chr1,
        Scmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            scmr::Chr1,
            scmr::Chr1,
            Scmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bcp2(self) -> crate::common::RegisterFieldBool<7, 1, 0, Scmr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Scmr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Scmr {
    #[inline(always)]
    fn default() -> Scmr {
        <crate::RegValueT<Scmr_SPEC> as RegisterValue<_>>::new(242)
    }
}
pub mod scmr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Smif_SPEC;
    pub type Smif = crate::EnumBitfieldStruct<u8, Smif_SPEC>;
    impl Smif {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sinv_SPEC;
    pub type Sinv = crate::EnumBitfieldStruct<u8, Sinv_SPEC>;
    impl Sinv {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdir_SPEC;
    pub type Sdir = crate::EnumBitfieldStruct<u8, Sdir_SPEC>;
    impl Sdir {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chr1_SPEC;
    pub type Chr1 = crate::EnumBitfieldStruct<u8, Chr1_SPEC>;
    impl Chr1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Snfr_SPEC;
impl crate::sealed::RegSpec for Snfr_SPEC {
    type DataType = u8;
}

pub type Snfr = crate::RegValueT<Snfr_SPEC>;

impl Snfr {
    #[inline(always)]
    pub fn nfcs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        snfr::Nfcs,
        snfr::Nfcs,
        Snfr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            snfr::Nfcs,
            snfr::Nfcs,
            Snfr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Snfr {
    #[inline(always)]
    fn default() -> Snfr {
        <crate::RegValueT<Snfr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod snfr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfcs_SPEC;
    pub type Nfcs = crate::EnumBitfieldStruct<u8, Nfcs_SPEC>;
    impl Nfcs {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Simr1_SPEC;
impl crate::sealed::RegSpec for Simr1_SPEC {
    type DataType = u8;
}

pub type Simr1 = crate::RegValueT<Simr1_SPEC>;

impl Simr1 {
    #[inline(always)]
    pub fn iicm(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        simr1::Iicm,
        simr1::Iicm,
        Simr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            simr1::Iicm,
            simr1::Iicm,
            Simr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn iicdl(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1f,
        1,
        0,
        simr1::Iicdl,
        simr1::Iicdl,
        Simr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1f,
            1,
            0,
            simr1::Iicdl,
            simr1::Iicdl,
            Simr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Simr1 {
    #[inline(always)]
    fn default() -> Simr1 {
        <crate::RegValueT<Simr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod simr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicm_SPEC;
    pub type Iicm = crate::EnumBitfieldStruct<u8, Iicm_SPEC>;
    impl Iicm {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicdl_SPEC;
    pub type Iicdl = crate::EnumBitfieldStruct<u8, Iicdl_SPEC>;
    impl Iicdl {
        pub const _0_X_00: Self = Self::new(0);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Simr2_SPEC;
impl crate::sealed::RegSpec for Simr2_SPEC {
    type DataType = u8;
}

pub type Simr2 = crate::RegValueT<Simr2_SPEC>;

impl Simr2 {
    #[inline(always)]
    pub fn iicintm(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        simr2::Iicintm,
        simr2::Iicintm,
        Simr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            simr2::Iicintm,
            simr2::Iicintm,
            Simr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn iiccsc(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        simr2::Iiccsc,
        simr2::Iiccsc,
        Simr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            simr2::Iiccsc,
            simr2::Iiccsc,
            Simr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn iicackt(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        simr2::Iicackt,
        simr2::Iicackt,
        Simr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            simr2::Iicackt,
            simr2::Iicackt,
            Simr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Simr2 {
    #[inline(always)]
    fn default() -> Simr2 {
        <crate::RegValueT<Simr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod simr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicintm_SPEC;
    pub type Iicintm = crate::EnumBitfieldStruct<u8, Iicintm_SPEC>;
    impl Iicintm {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iiccsc_SPEC;
    pub type Iiccsc = crate::EnumBitfieldStruct<u8, Iiccsc_SPEC>;
    impl Iiccsc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicackt_SPEC;
    pub type Iicackt = crate::EnumBitfieldStruct<u8, Iicackt_SPEC>;
    impl Iicackt {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Simr3_SPEC;
impl crate::sealed::RegSpec for Simr3_SPEC {
    type DataType = u8;
}

pub type Simr3 = crate::RegValueT<Simr3_SPEC>;

impl Simr3 {
    #[inline(always)]
    pub fn iicstareq(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        simr3::Iicstareq,
        simr3::Iicstareq,
        Simr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            simr3::Iicstareq,
            simr3::Iicstareq,
            Simr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn iicrstareq(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        simr3::Iicrstareq,
        simr3::Iicrstareq,
        Simr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            simr3::Iicrstareq,
            simr3::Iicrstareq,
            Simr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn iicstpreq(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        simr3::Iicstpreq,
        simr3::Iicstpreq,
        Simr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            simr3::Iicstpreq,
            simr3::Iicstpreq,
            Simr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn iicstif(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        simr3::Iicstif,
        simr3::Iicstif,
        Simr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            simr3::Iicstif,
            simr3::Iicstif,
            Simr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn iicsdas(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        simr3::Iicsdas,
        simr3::Iicsdas,
        Simr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            simr3::Iicsdas,
            simr3::Iicsdas,
            Simr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn iicscls(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        simr3::Iicscls,
        simr3::Iicscls,
        Simr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            simr3::Iicscls,
            simr3::Iicscls,
            Simr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Simr3 {
    #[inline(always)]
    fn default() -> Simr3 {
        <crate::RegValueT<Simr3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod simr3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicstareq_SPEC;
    pub type Iicstareq = crate::EnumBitfieldStruct<u8, Iicstareq_SPEC>;
    impl Iicstareq {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicrstareq_SPEC;
    pub type Iicrstareq = crate::EnumBitfieldStruct<u8, Iicrstareq_SPEC>;
    impl Iicrstareq {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicstpreq_SPEC;
    pub type Iicstpreq = crate::EnumBitfieldStruct<u8, Iicstpreq_SPEC>;
    impl Iicstpreq {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicstif_SPEC;
    pub type Iicstif = crate::EnumBitfieldStruct<u8, Iicstif_SPEC>;
    impl Iicstif {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicsdas_SPEC;
    pub type Iicsdas = crate::EnumBitfieldStruct<u8, Iicsdas_SPEC>;
    impl Iicsdas {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicscls_SPEC;
    pub type Iicscls = crate::EnumBitfieldStruct<u8, Iicscls_SPEC>;
    impl Iicscls {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sisr_SPEC;
impl crate::sealed::RegSpec for Sisr_SPEC {
    type DataType = u8;
}

pub type Sisr = crate::RegValueT<Sisr_SPEC>;

impl Sisr {
    #[inline(always)]
    pub fn iicackr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sisr::Iicackr,
        sisr::Iicackr,
        Sisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sisr::Iicackr,
            sisr::Iicackr,
            Sisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sisr {
    #[inline(always)]
    fn default() -> Sisr {
        <crate::RegValueT<Sisr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sisr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicackr_SPEC;
    pub type Iicackr = crate::EnumBitfieldStruct<u8, Iicackr_SPEC>;
    impl Iicackr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spmr_SPEC;
impl crate::sealed::RegSpec for Spmr_SPEC {
    type DataType = u8;
}

pub type Spmr = crate::RegValueT<Spmr_SPEC>;

impl Spmr {
    #[inline(always)]
    pub fn sse(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        spmr::Sse,
        spmr::Sse,
        Spmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            spmr::Sse,
            spmr::Sse,
            Spmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ctse(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        spmr::Ctse,
        spmr::Ctse,
        Spmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            spmr::Ctse,
            spmr::Ctse,
            Spmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mss(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        spmr::Mss,
        spmr::Mss,
        Spmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            spmr::Mss,
            spmr::Mss,
            Spmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ctspen(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        spmr::Ctspen,
        spmr::Ctspen,
        Spmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            spmr::Ctspen,
            spmr::Ctspen,
            Spmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mff(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        spmr::Mff,
        spmr::Mff,
        Spmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            spmr::Mff,
            spmr::Mff,
            Spmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ckpol(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        spmr::Ckpol,
        spmr::Ckpol,
        Spmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            spmr::Ckpol,
            spmr::Ckpol,
            Spmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ckph(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        spmr::Ckph,
        spmr::Ckph,
        Spmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            spmr::Ckph,
            spmr::Ckph,
            Spmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Spmr {
    #[inline(always)]
    fn default() -> Spmr {
        <crate::RegValueT<Spmr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod spmr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sse_SPEC;
    pub type Sse = crate::EnumBitfieldStruct<u8, Sse_SPEC>;
    impl Sse {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctse_SPEC;
    pub type Ctse = crate::EnumBitfieldStruct<u8, Ctse_SPEC>;
    impl Ctse {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mss_SPEC;
    pub type Mss = crate::EnumBitfieldStruct<u8, Mss_SPEC>;
    impl Mss {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctspen_SPEC;
    pub type Ctspen = crate::EnumBitfieldStruct<u8, Ctspen_SPEC>;
    impl Ctspen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mff_SPEC;
    pub type Mff = crate::EnumBitfieldStruct<u8, Mff_SPEC>;
    impl Mff {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ckpol_SPEC;
    pub type Ckpol = crate::EnumBitfieldStruct<u8, Ckpol_SPEC>;
    impl Ckpol {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ckph_SPEC;
    pub type Ckph = crate::EnumBitfieldStruct<u8, Ckph_SPEC>;
    impl Ckph {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ftdrhl_SPEC;
impl crate::sealed::RegSpec for Ftdrhl_SPEC {
    type DataType = u16;
}

pub type Ftdrhl = crate::RegValueT<Ftdrhl_SPEC>;

impl Ftdrhl {
    #[inline(always)]
    pub fn tdat(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Ftdrhl_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Ftdrhl_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mpbt(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ftdrhl::Mpbt,
        ftdrhl::Mpbt,
        Ftdrhl_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ftdrhl::Mpbt,
            ftdrhl::Mpbt,
            Ftdrhl_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ftdrhl {
    #[inline(always)]
    fn default() -> Ftdrhl {
        <crate::RegValueT<Ftdrhl_SPEC> as RegisterValue<_>>::new(65535)
    }
}
pub mod ftdrhl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpbt_SPEC;
    pub type Mpbt = crate::EnumBitfieldStruct<u8, Mpbt_SPEC>;
    impl Mpbt {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdrhl_SPEC;
impl crate::sealed::RegSpec for Tdrhl_SPEC {
    type DataType = u16;
}

pub type Tdrhl = crate::RegValueT<Tdrhl_SPEC>;

impl Tdrhl {
    #[inline(always)]
    pub fn tdat(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Tdrhl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Tdrhl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tdrhl {
    #[inline(always)]
    fn default() -> Tdrhl {
        <crate::RegValueT<Tdrhl_SPEC> as RegisterValue<_>>::new(65535)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ftdrh_SPEC;
impl crate::sealed::RegSpec for Ftdrh_SPEC {
    type DataType = u8;
}

pub type Ftdrh = crate::RegValueT<Ftdrh_SPEC>;

impl Ftdrh {
    #[inline(always)]
    pub fn tdat(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ftdrh_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ftdrh_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn mpbt(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ftdrh::Mpbt,
        ftdrh::Mpbt,
        Ftdrh_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ftdrh::Mpbt,
            ftdrh::Mpbt,
            Ftdrh_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ftdrh {
    #[inline(always)]
    fn default() -> Ftdrh {
        <crate::RegValueT<Ftdrh_SPEC> as RegisterValue<_>>::new(255)
    }
}
pub mod ftdrh {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpbt_SPEC;
    pub type Mpbt = crate::EnumBitfieldStruct<u8, Mpbt_SPEC>;
    impl Mpbt {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ftdrl_SPEC;
impl crate::sealed::RegSpec for Ftdrl_SPEC {
    type DataType = u8;
}

pub type Ftdrl = crate::RegValueT<Ftdrl_SPEC>;

impl Ftdrl {
    #[inline(always)]
    pub fn tdat(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ftdrl_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ftdrl_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Ftdrl {
    #[inline(always)]
    fn default() -> Ftdrl {
        <crate::RegValueT<Ftdrl_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frdrhl_SPEC;
impl crate::sealed::RegSpec for Frdrhl_SPEC {
    type DataType = u16;
}

pub type Frdrhl = crate::RegValueT<Frdrhl_SPEC>;

impl Frdrhl {
    #[inline(always)]
    pub fn rdat(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Frdrhl_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Frdrhl_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mpb(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        frdrhl::Mpb,
        frdrhl::Mpb,
        Frdrhl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            frdrhl::Mpb,
            frdrhl::Mpb,
            Frdrhl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        frdrhl::Dr,
        frdrhl::Dr,
        Frdrhl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            frdrhl::Dr,
            frdrhl::Dr,
            Frdrhl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn per(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        frdrhl::Per,
        frdrhl::Per,
        Frdrhl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            frdrhl::Per,
            frdrhl::Per,
            Frdrhl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fer(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        frdrhl::Fer,
        frdrhl::Fer,
        Frdrhl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            frdrhl::Fer,
            frdrhl::Fer,
            Frdrhl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn orer(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        frdrhl::Orer,
        frdrhl::Orer,
        Frdrhl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            frdrhl::Orer,
            frdrhl::Orer,
            Frdrhl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rdf(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        frdrhl::Rdf,
        frdrhl::Rdf,
        Frdrhl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            frdrhl::Rdf,
            frdrhl::Rdf,
            Frdrhl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Frdrhl {
    #[inline(always)]
    fn default() -> Frdrhl {
        <crate::RegValueT<Frdrhl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod frdrhl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpb_SPEC;
    pub type Mpb = crate::EnumBitfieldStruct<u8, Mpb_SPEC>;
    impl Mpb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dr_SPEC;
    pub type Dr = crate::EnumBitfieldStruct<u8, Dr_SPEC>;
    impl Dr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Per_SPEC;
    pub type Per = crate::EnumBitfieldStruct<u8, Per_SPEC>;
    impl Per {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fer_SPEC;
    pub type Fer = crate::EnumBitfieldStruct<u8, Fer_SPEC>;
    impl Fer {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Orer_SPEC;
    pub type Orer = crate::EnumBitfieldStruct<u8, Orer_SPEC>;
    impl Orer {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdf_SPEC;
    pub type Rdf = crate::EnumBitfieldStruct<u8, Rdf_SPEC>;
    impl Rdf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdrhl_SPEC;
impl crate::sealed::RegSpec for Rdrhl_SPEC {
    type DataType = u16;
}

pub type Rdrhl = crate::RegValueT<Rdrhl_SPEC>;

impl Rdrhl {
    #[inline(always)]
    pub fn rdat(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Rdrhl_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Rdrhl_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Rdrhl {
    #[inline(always)]
    fn default() -> Rdrhl {
        <crate::RegValueT<Rdrhl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frdrh_SPEC;
impl crate::sealed::RegSpec for Frdrh_SPEC {
    type DataType = u8;
}

pub type Frdrh = crate::RegValueT<Frdrh_SPEC>;

impl Frdrh {
    #[inline(always)]
    pub fn rdat(self) -> crate::common::RegisterFieldBool<0, 1, 0, Frdrh_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Frdrh_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn mpb(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        frdrh::Mpb,
        frdrh::Mpb,
        Frdrh_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            frdrh::Mpb,
            frdrh::Mpb,
            Frdrh_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        frdrh::Dr,
        frdrh::Dr,
        Frdrh_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            frdrh::Dr,
            frdrh::Dr,
            Frdrh_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn per(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        frdrh::Per,
        frdrh::Per,
        Frdrh_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            frdrh::Per,
            frdrh::Per,
            Frdrh_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fer(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        frdrh::Fer,
        frdrh::Fer,
        Frdrh_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            frdrh::Fer,
            frdrh::Fer,
            Frdrh_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn orer(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        frdrh::Orer,
        frdrh::Orer,
        Frdrh_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            frdrh::Orer,
            frdrh::Orer,
            Frdrh_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rdf(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        frdrh::Rdf,
        frdrh::Rdf,
        Frdrh_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            frdrh::Rdf,
            frdrh::Rdf,
            Frdrh_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Frdrh {
    #[inline(always)]
    fn default() -> Frdrh {
        <crate::RegValueT<Frdrh_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod frdrh {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpb_SPEC;
    pub type Mpb = crate::EnumBitfieldStruct<u8, Mpb_SPEC>;
    impl Mpb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dr_SPEC;
    pub type Dr = crate::EnumBitfieldStruct<u8, Dr_SPEC>;
    impl Dr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Per_SPEC;
    pub type Per = crate::EnumBitfieldStruct<u8, Per_SPEC>;
    impl Per {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fer_SPEC;
    pub type Fer = crate::EnumBitfieldStruct<u8, Fer_SPEC>;
    impl Fer {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Orer_SPEC;
    pub type Orer = crate::EnumBitfieldStruct<u8, Orer_SPEC>;
    impl Orer {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdf_SPEC;
    pub type Rdf = crate::EnumBitfieldStruct<u8, Rdf_SPEC>;
    impl Rdf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frdrl_SPEC;
impl crate::sealed::RegSpec for Frdrl_SPEC {
    type DataType = u8;
}

pub type Frdrl = crate::RegValueT<Frdrl_SPEC>;

impl Frdrl {
    #[inline(always)]
    pub fn rdat(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Frdrl_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Frdrl_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Frdrl {
    #[inline(always)]
    fn default() -> Frdrl {
        <crate::RegValueT<Frdrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mddr_SPEC;
impl crate::sealed::RegSpec for Mddr_SPEC {
    type DataType = u8;
}

pub type Mddr = crate::RegValueT<Mddr_SPEC>;

impl NoBitfieldReg<Mddr_SPEC> for Mddr {}
impl ::core::default::Default for Mddr {
    #[inline(always)]
    fn default() -> Mddr {
        <crate::RegValueT<Mddr_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dccr_SPEC;
impl crate::sealed::RegSpec for Dccr_SPEC {
    type DataType = u8;
}

pub type Dccr = crate::RegValueT<Dccr_SPEC>;

impl Dccr {
    #[inline(always)]
    pub fn dcmf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dccr::Dcmf,
        dccr::Dcmf,
        Dccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dccr::Dcmf,
            dccr::Dcmf,
            Dccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dper(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dccr::Dper,
        dccr::Dper,
        Dccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dccr::Dper,
            dccr::Dper,
            Dccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dfer(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dccr::Dfer,
        dccr::Dfer,
        Dccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dccr::Dfer,
            dccr::Dfer,
            Dccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn idsel(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        dccr::Idsel,
        dccr::Idsel,
        Dccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            dccr::Idsel,
            dccr::Idsel,
            Dccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dcme(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dccr::Dcme,
        dccr::Dcme,
        Dccr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dccr::Dcme,
            dccr::Dcme,
            Dccr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dccr {
    #[inline(always)]
    fn default() -> Dccr {
        <crate::RegValueT<Dccr_SPEC> as RegisterValue<_>>::new(64)
    }
}
pub mod dccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcmf_SPEC;
    pub type Dcmf = crate::EnumBitfieldStruct<u8, Dcmf_SPEC>;
    impl Dcmf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dper_SPEC;
    pub type Dper = crate::EnumBitfieldStruct<u8, Dper_SPEC>;
    impl Dper {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dfer_SPEC;
    pub type Dfer = crate::EnumBitfieldStruct<u8, Dfer_SPEC>;
    impl Dfer {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idsel_SPEC;
    pub type Idsel = crate::EnumBitfieldStruct<u8, Idsel_SPEC>;
    impl Idsel {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcme_SPEC;
    pub type Dcme = crate::EnumBitfieldStruct<u8, Dcme_SPEC>;
    impl Dcme {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcr_SPEC;
impl crate::sealed::RegSpec for Fcr_SPEC {
    type DataType = u16;
}

pub type Fcr = crate::RegValueT<Fcr_SPEC>;

impl Fcr {
    #[inline(always)]
    pub fn fm(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, fcr::Fm, fcr::Fm, Fcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,fcr::Fm,fcr::Fm,Fcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfrst(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        fcr::Rfrst,
        fcr::Rfrst,
        Fcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            fcr::Rfrst,
            fcr::Rfrst,
            Fcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tfrst(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        fcr::Tfrst,
        fcr::Tfrst,
        Fcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            fcr::Tfrst,
            fcr::Tfrst,
            Fcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dres(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, fcr::Dres, fcr::Dres, Fcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            fcr::Dres,
            fcr::Dres,
            Fcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ttrg(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Fcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Fcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rtrg(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, Fcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,Fcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rstrg(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Fcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,Fcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fcr {
    #[inline(always)]
    fn default() -> Fcr {
        <crate::RegValueT<Fcr_SPEC> as RegisterValue<_>>::new(63488)
    }
}
pub mod fcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fm_SPEC;
    pub type Fm = crate::EnumBitfieldStruct<u8, Fm_SPEC>;
    impl Fm {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfrst_SPEC;
    pub type Rfrst = crate::EnumBitfieldStruct<u8, Rfrst_SPEC>;
    impl Rfrst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tfrst_SPEC;
    pub type Tfrst = crate::EnumBitfieldStruct<u8, Tfrst_SPEC>;
    impl Tfrst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dres_SPEC;
    pub type Dres = crate::EnumBitfieldStruct<u8, Dres_SPEC>;
    impl Dres {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdr_SPEC;
impl crate::sealed::RegSpec for Fdr_SPEC {
    type DataType = u16;
}

pub type Fdr = crate::RegValueT<Fdr_SPEC>;

impl Fdr {
    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Fdr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Fdr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn t(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, Fdr_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,Fdr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fdr {
    #[inline(always)]
    fn default() -> Fdr {
        <crate::RegValueT<Fdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lsr_SPEC;
impl crate::sealed::RegSpec for Lsr_SPEC {
    type DataType = u16;
}

pub type Lsr = crate::RegValueT<Lsr_SPEC>;

impl Lsr {
    #[inline(always)]
    pub fn orer(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, lsr::Orer, lsr::Orer, Lsr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,lsr::Orer,lsr::Orer,Lsr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn fnum(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, u8, Lsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x1f,1,0,u8,u8,Lsr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pnum(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, Lsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,Lsr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Lsr {
    #[inline(always)]
    fn default() -> Lsr {
        <crate::RegValueT<Lsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod lsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Orer_SPEC;
    pub type Orer = crate::EnumBitfieldStruct<u8, Orer_SPEC>;
    impl Orer {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdr_SPEC;
impl crate::sealed::RegSpec for Cdr_SPEC {
    type DataType = u16;
}

pub type Cdr = crate::RegValueT<Cdr_SPEC>;

impl Cdr {
    #[inline(always)]
    pub fn cmpd(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Cdr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Cdr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cdr {
    #[inline(always)]
    fn default() -> Cdr {
        <crate::RegValueT<Cdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sptr_SPEC;
impl crate::sealed::RegSpec for Sptr_SPEC {
    type DataType = u8;
}

pub type Sptr = crate::RegValueT<Sptr_SPEC>;

impl Sptr {
    #[inline(always)]
    pub fn rxdmon(self) -> crate::common::RegisterFieldBool<0, 1, 0, Sptr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Sptr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn spb2dt(self) -> crate::common::RegisterFieldBool<1, 1, 0, Sptr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Sptr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn spb2io(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        sptr::Spb2Io,
        sptr::Spb2Io,
        Sptr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            sptr::Spb2Io,
            sptr::Spb2Io,
            Sptr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rinv(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sptr::Rinv,
        sptr::Rinv,
        Sptr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sptr::Rinv,
            sptr::Rinv,
            Sptr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tinv(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        sptr::Tinv,
        sptr::Tinv,
        Sptr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            sptr::Tinv,
            sptr::Tinv,
            Sptr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn asen(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        sptr::Asen,
        sptr::Asen,
        Sptr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            sptr::Asen,
            sptr::Asen,
            Sptr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn aten(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        sptr::Aten,
        sptr::Aten,
        Sptr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            sptr::Aten,
            sptr::Aten,
            Sptr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sptr {
    #[inline(always)]
    fn default() -> Sptr {
        <crate::RegValueT<Sptr_SPEC> as RegisterValue<_>>::new(3)
    }
}
pub mod sptr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spb2Io_SPEC;
    pub type Spb2Io = crate::EnumBitfieldStruct<u8, Spb2Io_SPEC>;
    impl Spb2Io {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rinv_SPEC;
    pub type Rinv = crate::EnumBitfieldStruct<u8, Rinv_SPEC>;
    impl Rinv {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tinv_SPEC;
    pub type Tinv = crate::EnumBitfieldStruct<u8, Tinv_SPEC>;
    impl Tinv {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asen_SPEC;
    pub type Asen = crate::EnumBitfieldStruct<u8, Asen_SPEC>;
    impl Asen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aten_SPEC;
    pub type Aten = crate::EnumBitfieldStruct<u8, Aten_SPEC>;
    impl Aten {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Actr_SPEC;
impl crate::sealed::RegSpec for Actr_SPEC {
    type DataType = u8;
}

pub type Actr = crate::RegValueT<Actr_SPEC>;

impl Actr {
    #[inline(always)]
    pub fn ast(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Actr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Actr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ajd(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        actr::Ajd,
        actr::Ajd,
        Actr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            actr::Ajd,
            actr::Ajd,
            Actr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn att(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Actr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Actr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn aet(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        actr::Aet,
        actr::Aet,
        Actr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            actr::Aet,
            actr::Aet,
            Actr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Actr {
    #[inline(always)]
    fn default() -> Actr {
        <crate::RegValueT<Actr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod actr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ajd_SPEC;
    pub type Ajd = crate::EnumBitfieldStruct<u8, Ajd_SPEC>;
    impl Ajd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aet_SPEC;
    pub type Aet = crate::EnumBitfieldStruct<u8, Aet_SPEC>;
    impl Aet {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmr_SPEC;
impl crate::sealed::RegSpec for Mmr_SPEC {
    type DataType = u8;
}

pub type Mmr = crate::RegValueT<Mmr_SPEC>;

impl Mmr {
    #[inline(always)]
    pub fn rmpol(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmr::Rmpol,
        mmr::Rmpol,
        Mmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmr::Rmpol,
            mmr::Rmpol,
            Mmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tmpol(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mmr::Tmpol,
        mmr::Tmpol,
        Mmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mmr::Tmpol,
            mmr::Tmpol,
            Mmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn erten(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mmr::Erten,
        mmr::Erten,
        Mmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mmr::Erten,
            mmr::Erten,
            Mmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn synval(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        mmr::Synval,
        mmr::Synval,
        Mmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            mmr::Synval,
            mmr::Synval,
            Mmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn synsel(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        mmr::Synsel,
        mmr::Synsel,
        Mmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            mmr::Synsel,
            mmr::Synsel,
            Mmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sbsel(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        mmr::Sbsel,
        mmr::Sbsel,
        Mmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            mmr::Sbsel,
            mmr::Sbsel,
            Mmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn manen(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        mmr::Manen,
        mmr::Manen,
        Mmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            mmr::Manen,
            mmr::Manen,
            Mmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mmr {
    #[inline(always)]
    fn default() -> Mmr {
        <crate::RegValueT<Mmr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmpol_SPEC;
    pub type Rmpol = crate::EnumBitfieldStruct<u8, Rmpol_SPEC>;
    impl Rmpol {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmpol_SPEC;
    pub type Tmpol = crate::EnumBitfieldStruct<u8, Tmpol_SPEC>;
    impl Tmpol {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Erten_SPEC;
    pub type Erten = crate::EnumBitfieldStruct<u8, Erten_SPEC>;
    impl Erten {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Synval_SPEC;
    pub type Synval = crate::EnumBitfieldStruct<u8, Synval_SPEC>;
    impl Synval {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Synsel_SPEC;
    pub type Synsel = crate::EnumBitfieldStruct<u8, Synsel_SPEC>;
    impl Synsel {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbsel_SPEC;
    pub type Sbsel = crate::EnumBitfieldStruct<u8, Sbsel_SPEC>;
    impl Sbsel {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Manen_SPEC;
    pub type Manen = crate::EnumBitfieldStruct<u8, Manen_SPEC>;
    impl Manen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmpr_SPEC;
impl crate::sealed::RegSpec for Tmpr_SPEC {
    type DataType = u8;
}

pub type Tmpr = crate::RegValueT<Tmpr_SPEC>;

impl Tmpr {
    #[inline(always)]
    pub fn tplen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        tmpr::Tplen,
        tmpr::Tplen,
        Tmpr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            tmpr::Tplen,
            tmpr::Tplen,
            Tmpr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tppat(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        tmpr::Tppat,
        tmpr::Tppat,
        Tmpr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            tmpr::Tppat,
            tmpr::Tppat,
            Tmpr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tmpr {
    #[inline(always)]
    fn default() -> Tmpr {
        <crate::RegValueT<Tmpr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tmpr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tplen_SPEC;
    pub type Tplen = crate::EnumBitfieldStruct<u8, Tplen_SPEC>;
    impl Tplen {
        pub const _0_X_0: Self = Self::new(0);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tppat_SPEC;
    pub type Tppat = crate::EnumBitfieldStruct<u8, Tppat_SPEC>;
    impl Tppat {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rmpr_SPEC;
impl crate::sealed::RegSpec for Rmpr_SPEC {
    type DataType = u8;
}

pub type Rmpr = crate::RegValueT<Rmpr_SPEC>;

impl Rmpr {
    #[inline(always)]
    pub fn rplen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        rmpr::Rplen,
        rmpr::Rplen,
        Rmpr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            rmpr::Rplen,
            rmpr::Rplen,
            Rmpr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rppat(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        rmpr::Rppat,
        rmpr::Rppat,
        Rmpr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            rmpr::Rppat,
            rmpr::Rppat,
            Rmpr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rmpr {
    #[inline(always)]
    fn default() -> Rmpr {
        <crate::RegValueT<Rmpr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rmpr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rplen_SPEC;
    pub type Rplen = crate::EnumBitfieldStruct<u8, Rplen_SPEC>;
    impl Rplen {
        pub const _0: Self = Self::new(0);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rppat_SPEC;
    pub type Rppat = crate::EnumBitfieldStruct<u8, Rppat_SPEC>;
    impl Rppat {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mesr_SPEC;
impl crate::sealed::RegSpec for Mesr_SPEC {
    type DataType = u8;
}

pub type Mesr = crate::RegValueT<Mesr_SPEC>;

impl Mesr {
    #[inline(always)]
    pub fn pfer(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mesr::Pfer,
        mesr::Pfer,
        Mesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mesr::Pfer,
            mesr::Pfer,
            Mesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn syer(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mesr::Syer,
        mesr::Syer,
        Mesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mesr::Syer,
            mesr::Syer,
            Mesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sber(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mesr::Sber,
        mesr::Sber,
        Mesr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mesr::Sber,
            mesr::Sber,
            Mesr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mesr {
    #[inline(always)]
    fn default() -> Mesr {
        <crate::RegValueT<Mesr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mesr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfer_SPEC;
    pub type Pfer = crate::EnumBitfieldStruct<u8, Pfer_SPEC>;
    impl Pfer {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Syer_SPEC;
    pub type Syer = crate::EnumBitfieldStruct<u8, Syer_SPEC>;
    impl Syer {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sber_SPEC;
    pub type Sber = crate::EnumBitfieldStruct<u8, Sber_SPEC>;
    impl Sber {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mecr_SPEC;
impl crate::sealed::RegSpec for Mecr_SPEC {
    type DataType = u8;
}

pub type Mecr = crate::RegValueT<Mecr_SPEC>;

impl Mecr {
    #[inline(always)]
    pub fn pferen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mecr::Pferen,
        mecr::Pferen,
        Mecr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mecr::Pferen,
            mecr::Pferen,
            Mecr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn syeren(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mecr::Syeren,
        mecr::Syeren,
        Mecr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mecr::Syeren,
            mecr::Syeren,
            Mecr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sberen(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mecr::Sberen,
        mecr::Sberen,
        Mecr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mecr::Sberen,
            mecr::Sberen,
            Mecr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mecr {
    #[inline(always)]
    fn default() -> Mecr {
        <crate::RegValueT<Mecr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mecr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pferen_SPEC;
    pub type Pferen = crate::EnumBitfieldStruct<u8, Pferen_SPEC>;
    impl Pferen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Syeren_SPEC;
    pub type Syeren = crate::EnumBitfieldStruct<u8, Syeren_SPEC>;
    impl Syeren {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sberen_SPEC;
    pub type Sberen = crate::EnumBitfieldStruct<u8, Sberen_SPEC>;
    impl Sberen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
