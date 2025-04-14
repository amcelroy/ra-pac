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
#[doc = r"General PWM 32-bit Timer 4"]
unsafe impl ::core::marker::Send for super::Gpt324 {}
unsafe impl ::core::marker::Sync for super::Gpt324 {}
impl super::Gpt324 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn gtwp(&self) -> &'static crate::common::Reg<self::Gtwp_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtwp_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtstr(&self) -> &'static crate::common::Reg<self::Gtstr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtstr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtstp(&self) -> &'static crate::common::Reg<self::Gtstp_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtstp_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtclr(&self) -> &'static crate::common::Reg<self::Gtclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Gtclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtssr(&self) -> &'static crate::common::Reg<self::Gtssr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtssr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtpsr(&self) -> &'static crate::common::Reg<self::Gtpsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtpsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtcsr(&self) -> &'static crate::common::Reg<self::Gtcsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtcsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gticasr(
        &self,
    ) -> &'static crate::common::Reg<self::Gticasr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gticasr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gticbsr(
        &self,
    ) -> &'static crate::common::Reg<self::Gticbsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gticbsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtcr(&self) -> &'static crate::common::Reg<self::Gtcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtuddtyc(
        &self,
    ) -> &'static crate::common::Reg<self::Gtuddtyc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtuddtyc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtior(&self) -> &'static crate::common::Reg<self::Gtior_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtior_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtintad(
        &self,
    ) -> &'static crate::common::Reg<self::Gtintad_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtintad_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtst(&self) -> &'static crate::common::Reg<self::Gtst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtber(&self) -> &'static crate::common::Reg<self::Gtber_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtber_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtitc(&self) -> &'static crate::common::Reg<self::Gtitc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtitc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtcnt(&self) -> &'static crate::common::Reg<self::Gtcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtccra(
        &self,
    ) -> &'static crate::common::Reg<self::Gtccra_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtccra_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtccrb(
        &self,
    ) -> &'static crate::common::Reg<self::Gtccrb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtccrb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtccrc(
        &self,
    ) -> &'static crate::common::Reg<self::Gtccrc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtccrc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtccre(
        &self,
    ) -> &'static crate::common::Reg<self::Gtccre_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtccre_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtccrd(
        &self,
    ) -> &'static crate::common::Reg<self::Gtccrd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtccrd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtccrf(
        &self,
    ) -> &'static crate::common::Reg<self::Gtccrf_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtccrf_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtpr(&self) -> &'static crate::common::Reg<self::Gtpr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtpr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtpbr(&self) -> &'static crate::common::Reg<self::Gtpbr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtpbr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtpdbr(
        &self,
    ) -> &'static crate::common::Reg<self::Gtpdbr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtpdbr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtadtra(
        &self,
    ) -> &'static crate::common::Reg<self::Gtadtra_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtadtra_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtadtbra(
        &self,
    ) -> &'static crate::common::Reg<self::Gtadtbra_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtadtbra_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtadtdbra(
        &self,
    ) -> &'static crate::common::Reg<self::Gtadtdbra_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtadtdbra_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtadtrb(
        &self,
    ) -> &'static crate::common::Reg<self::Gtadtrb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtadtrb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtadtbrb(
        &self,
    ) -> &'static crate::common::Reg<self::Gtadtbrb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtadtbrb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtadtdbrb(
        &self,
    ) -> &'static crate::common::Reg<self::Gtadtdbrb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtadtdbrb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtdtcr(
        &self,
    ) -> &'static crate::common::Reg<self::Gtdtcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtdtcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(136usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtdvu(&self) -> &'static crate::common::Reg<self::Gtdvu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtdvu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtdvd(&self) -> &'static crate::common::Reg<self::Gtdvd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtdvd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtdbu(&self) -> &'static crate::common::Reg<self::Gtdbu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtdbu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtdbd(&self) -> &'static crate::common::Reg<self::Gtdbd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtdbd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtsos(&self) -> &'static crate::common::Reg<self::Gtsos_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gtsos_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(156usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtsotr(
        &self,
    ) -> &'static crate::common::Reg<self::Gtsotr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtsotr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtadsmr(
        &self,
    ) -> &'static crate::common::Reg<self::Gtadsmr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtadsmr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(164usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gteitc(
        &self,
    ) -> &'static crate::common::Reg<self::Gteitc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gteitc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(168usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gteitli1(
        &self,
    ) -> &'static crate::common::Reg<self::Gteitli1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gteitli1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(172usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gteitli2(
        &self,
    ) -> &'static crate::common::Reg<self::Gteitli2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gteitli2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(176usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gteitlb(
        &self,
    ) -> &'static crate::common::Reg<self::Gteitlb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gteitlb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(180usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gticlf(
        &self,
    ) -> &'static crate::common::Reg<self::Gticlf_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gticlf_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(184usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtadcmsc(
        &self,
    ) -> &'static crate::common::Reg<self::Gtadcmsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtadcmsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtadcmss(
        &self,
    ) -> &'static crate::common::Reg<self::Gtadcmss_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtadcmss_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(196usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtsecsr(
        &self,
    ) -> &'static crate::common::Reg<self::Gtsecsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtsecsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(208usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtsecr(
        &self,
    ) -> &'static crate::common::Reg<self::Gtsecr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtsecr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(212usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtber2(
        &self,
    ) -> &'static crate::common::Reg<self::Gtber2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtber2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(224usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gtolbr(
        &self,
    ) -> &'static crate::common::Reg<self::Gtolbr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtolbr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(228usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gticcr(
        &self,
    ) -> &'static crate::common::Reg<self::Gticcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gticcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(236usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtwp_SPEC;
impl crate::sealed::RegSpec for Gtwp_SPEC {
    type DataType = u32;
}

pub type Gtwp = crate::RegValueT<Gtwp_SPEC>;

impl Gtwp {
    #[inline(always)]
    pub fn wp(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, gtwp::Wp, gtwp::Wp, Gtwp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,gtwp::Wp,gtwp::Wp,Gtwp_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn strwp(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gtwp::Strwp,
        gtwp::Strwp,
        Gtwp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gtwp::Strwp,
            gtwp::Strwp,
            Gtwp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn stpwp(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gtwp::Stpwp,
        gtwp::Stpwp,
        Gtwp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gtwp::Stpwp,
            gtwp::Stpwp,
            Gtwp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn clrwp(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gtwp::Clrwp,
        gtwp::Clrwp,
        Gtwp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gtwp::Clrwp,
            gtwp::Clrwp,
            Gtwp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmnwp(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gtwp::Cmnwp,
        gtwp::Cmnwp,
        Gtwp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gtwp::Cmnwp,
            gtwp::Cmnwp,
            Gtwp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn prkey(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Gtwp_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Gtwp_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtwp {
    #[inline(always)]
    fn default() -> Gtwp {
        <crate::RegValueT<Gtwp_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtwp {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wp_SPEC;
    pub type Wp = crate::EnumBitfieldStruct<u8, Wp_SPEC>;
    impl Wp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Strwp_SPEC;
    pub type Strwp = crate::EnumBitfieldStruct<u8, Strwp_SPEC>;
    impl Strwp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stpwp_SPEC;
    pub type Stpwp = crate::EnumBitfieldStruct<u8, Stpwp_SPEC>;
    impl Stpwp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clrwp_SPEC;
    pub type Clrwp = crate::EnumBitfieldStruct<u8, Clrwp_SPEC>;
    impl Clrwp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmnwp_SPEC;
    pub type Cmnwp = crate::EnumBitfieldStruct<u8, Cmnwp_SPEC>;
    impl Cmnwp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtstr_SPEC;
impl crate::sealed::RegSpec for Gtstr_SPEC {
    type DataType = u32;
}

pub type Gtstr = crate::RegValueT<Gtstr_SPEC>;

impl Gtstr {
    #[inline(always)]
    pub fn cstrt0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtstr::Cstrt0,
        gtstr::Cstrt0,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtstr::Cstrt0,
            gtstr::Cstrt0,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstrt1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gtstr::Cstrt1,
        gtstr::Cstrt1,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gtstr::Cstrt1,
            gtstr::Cstrt1,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstrt2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gtstr::Cstrt2,
        gtstr::Cstrt2,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gtstr::Cstrt2,
            gtstr::Cstrt2,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstrt3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gtstr::Cstrt3,
        gtstr::Cstrt3,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gtstr::Cstrt3,
            gtstr::Cstrt3,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstrt4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gtstr::Cstrt4,
        gtstr::Cstrt4,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gtstr::Cstrt4,
            gtstr::Cstrt4,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstrt5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gtstr::Cstrt5,
        gtstr::Cstrt5,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gtstr::Cstrt5,
            gtstr::Cstrt5,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstrt6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        gtstr::Cstrt6,
        gtstr::Cstrt6,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            gtstr::Cstrt6,
            gtstr::Cstrt6,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstrt7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        gtstr::Cstrt7,
        gtstr::Cstrt7,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            gtstr::Cstrt7,
            gtstr::Cstrt7,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstrt8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gtstr::Cstrt8,
        gtstr::Cstrt8,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gtstr::Cstrt8,
            gtstr::Cstrt8,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstrt9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        gtstr::Cstrt9,
        gtstr::Cstrt9,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            gtstr::Cstrt9,
            gtstr::Cstrt9,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstrt10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        gtstr::Cstrt10,
        gtstr::Cstrt10,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            gtstr::Cstrt10,
            gtstr::Cstrt10,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstrt11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        gtstr::Cstrt11,
        gtstr::Cstrt11,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            gtstr::Cstrt11,
            gtstr::Cstrt11,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstrt12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        gtstr::Cstrt12,
        gtstr::Cstrt12,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            gtstr::Cstrt12,
            gtstr::Cstrt12,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstrt13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        gtstr::Cstrt13,
        gtstr::Cstrt13,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            gtstr::Cstrt13,
            gtstr::Cstrt13,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstrt14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        gtstr::Cstrt14,
        gtstr::Cstrt14,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            gtstr::Cstrt14,
            gtstr::Cstrt14,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstrt15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        gtstr::Cstrt15,
        gtstr::Cstrt15,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            gtstr::Cstrt15,
            gtstr::Cstrt15,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstrt16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        gtstr::Cstrt16,
        gtstr::Cstrt16,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gtstr::Cstrt16,
            gtstr::Cstrt16,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstrt17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        gtstr::Cstrt17,
        gtstr::Cstrt17,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            gtstr::Cstrt17,
            gtstr::Cstrt17,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstrt18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        gtstr::Cstrt18,
        gtstr::Cstrt18,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            gtstr::Cstrt18,
            gtstr::Cstrt18,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstrt19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        gtstr::Cstrt19,
        gtstr::Cstrt19,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            gtstr::Cstrt19,
            gtstr::Cstrt19,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstrt20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        gtstr::Cstrt20,
        gtstr::Cstrt20,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            gtstr::Cstrt20,
            gtstr::Cstrt20,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstrt21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        gtstr::Cstrt21,
        gtstr::Cstrt21,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            gtstr::Cstrt21,
            gtstr::Cstrt21,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstrt22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        gtstr::Cstrt22,
        gtstr::Cstrt22,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            gtstr::Cstrt22,
            gtstr::Cstrt22,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstrt23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        gtstr::Cstrt23,
        gtstr::Cstrt23,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            gtstr::Cstrt23,
            gtstr::Cstrt23,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstrt24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        gtstr::Cstrt24,
        gtstr::Cstrt24,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            gtstr::Cstrt24,
            gtstr::Cstrt24,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstrt25(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        gtstr::Cstrt25,
        gtstr::Cstrt25,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            gtstr::Cstrt25,
            gtstr::Cstrt25,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstrt26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        gtstr::Cstrt26,
        gtstr::Cstrt26,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            gtstr::Cstrt26,
            gtstr::Cstrt26,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstrt27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        gtstr::Cstrt27,
        gtstr::Cstrt27,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            gtstr::Cstrt27,
            gtstr::Cstrt27,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstrt28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        gtstr::Cstrt28,
        gtstr::Cstrt28,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            gtstr::Cstrt28,
            gtstr::Cstrt28,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstrt29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        gtstr::Cstrt29,
        gtstr::Cstrt29,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            gtstr::Cstrt29,
            gtstr::Cstrt29,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstrt30(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        gtstr::Cstrt30,
        gtstr::Cstrt30,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            gtstr::Cstrt30,
            gtstr::Cstrt30,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstrt31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        gtstr::Cstrt31,
        gtstr::Cstrt31,
        Gtstr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            gtstr::Cstrt31,
            gtstr::Cstrt31,
            Gtstr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtstr {
    #[inline(always)]
    fn default() -> Gtstr {
        <crate::RegValueT<Gtstr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtstr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt0_SPEC;
    pub type Cstrt0 = crate::EnumBitfieldStruct<u8, Cstrt0_SPEC>;
    impl Cstrt0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt1_SPEC;
    pub type Cstrt1 = crate::EnumBitfieldStruct<u8, Cstrt1_SPEC>;
    impl Cstrt1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt2_SPEC;
    pub type Cstrt2 = crate::EnumBitfieldStruct<u8, Cstrt2_SPEC>;
    impl Cstrt2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt3_SPEC;
    pub type Cstrt3 = crate::EnumBitfieldStruct<u8, Cstrt3_SPEC>;
    impl Cstrt3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt4_SPEC;
    pub type Cstrt4 = crate::EnumBitfieldStruct<u8, Cstrt4_SPEC>;
    impl Cstrt4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt5_SPEC;
    pub type Cstrt5 = crate::EnumBitfieldStruct<u8, Cstrt5_SPEC>;
    impl Cstrt5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt6_SPEC;
    pub type Cstrt6 = crate::EnumBitfieldStruct<u8, Cstrt6_SPEC>;
    impl Cstrt6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt7_SPEC;
    pub type Cstrt7 = crate::EnumBitfieldStruct<u8, Cstrt7_SPEC>;
    impl Cstrt7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt8_SPEC;
    pub type Cstrt8 = crate::EnumBitfieldStruct<u8, Cstrt8_SPEC>;
    impl Cstrt8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt9_SPEC;
    pub type Cstrt9 = crate::EnumBitfieldStruct<u8, Cstrt9_SPEC>;
    impl Cstrt9 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt10_SPEC;
    pub type Cstrt10 = crate::EnumBitfieldStruct<u8, Cstrt10_SPEC>;
    impl Cstrt10 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt11_SPEC;
    pub type Cstrt11 = crate::EnumBitfieldStruct<u8, Cstrt11_SPEC>;
    impl Cstrt11 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt12_SPEC;
    pub type Cstrt12 = crate::EnumBitfieldStruct<u8, Cstrt12_SPEC>;
    impl Cstrt12 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt13_SPEC;
    pub type Cstrt13 = crate::EnumBitfieldStruct<u8, Cstrt13_SPEC>;
    impl Cstrt13 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt14_SPEC;
    pub type Cstrt14 = crate::EnumBitfieldStruct<u8, Cstrt14_SPEC>;
    impl Cstrt14 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt15_SPEC;
    pub type Cstrt15 = crate::EnumBitfieldStruct<u8, Cstrt15_SPEC>;
    impl Cstrt15 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt16_SPEC;
    pub type Cstrt16 = crate::EnumBitfieldStruct<u8, Cstrt16_SPEC>;
    impl Cstrt16 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt17_SPEC;
    pub type Cstrt17 = crate::EnumBitfieldStruct<u8, Cstrt17_SPEC>;
    impl Cstrt17 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt18_SPEC;
    pub type Cstrt18 = crate::EnumBitfieldStruct<u8, Cstrt18_SPEC>;
    impl Cstrt18 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt19_SPEC;
    pub type Cstrt19 = crate::EnumBitfieldStruct<u8, Cstrt19_SPEC>;
    impl Cstrt19 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt20_SPEC;
    pub type Cstrt20 = crate::EnumBitfieldStruct<u8, Cstrt20_SPEC>;
    impl Cstrt20 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt21_SPEC;
    pub type Cstrt21 = crate::EnumBitfieldStruct<u8, Cstrt21_SPEC>;
    impl Cstrt21 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt22_SPEC;
    pub type Cstrt22 = crate::EnumBitfieldStruct<u8, Cstrt22_SPEC>;
    impl Cstrt22 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt23_SPEC;
    pub type Cstrt23 = crate::EnumBitfieldStruct<u8, Cstrt23_SPEC>;
    impl Cstrt23 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt24_SPEC;
    pub type Cstrt24 = crate::EnumBitfieldStruct<u8, Cstrt24_SPEC>;
    impl Cstrt24 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt25_SPEC;
    pub type Cstrt25 = crate::EnumBitfieldStruct<u8, Cstrt25_SPEC>;
    impl Cstrt25 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt26_SPEC;
    pub type Cstrt26 = crate::EnumBitfieldStruct<u8, Cstrt26_SPEC>;
    impl Cstrt26 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt27_SPEC;
    pub type Cstrt27 = crate::EnumBitfieldStruct<u8, Cstrt27_SPEC>;
    impl Cstrt27 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt28_SPEC;
    pub type Cstrt28 = crate::EnumBitfieldStruct<u8, Cstrt28_SPEC>;
    impl Cstrt28 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt29_SPEC;
    pub type Cstrt29 = crate::EnumBitfieldStruct<u8, Cstrt29_SPEC>;
    impl Cstrt29 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt30_SPEC;
    pub type Cstrt30 = crate::EnumBitfieldStruct<u8, Cstrt30_SPEC>;
    impl Cstrt30 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt31_SPEC;
    pub type Cstrt31 = crate::EnumBitfieldStruct<u8, Cstrt31_SPEC>;
    impl Cstrt31 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtstp_SPEC;
impl crate::sealed::RegSpec for Gtstp_SPEC {
    type DataType = u32;
}

pub type Gtstp = crate::RegValueT<Gtstp_SPEC>;

impl Gtstp {
    #[inline(always)]
    pub fn cstop0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtstp::Cstop0,
        gtstp::Cstop0,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtstp::Cstop0,
            gtstp::Cstop0,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstop1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gtstp::Cstop1,
        gtstp::Cstop1,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gtstp::Cstop1,
            gtstp::Cstop1,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstop2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gtstp::Cstop2,
        gtstp::Cstop2,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gtstp::Cstop2,
            gtstp::Cstop2,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstop3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gtstp::Cstop3,
        gtstp::Cstop3,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gtstp::Cstop3,
            gtstp::Cstop3,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstop4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gtstp::Cstop4,
        gtstp::Cstop4,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gtstp::Cstop4,
            gtstp::Cstop4,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstop5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gtstp::Cstop5,
        gtstp::Cstop5,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gtstp::Cstop5,
            gtstp::Cstop5,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstop6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        gtstp::Cstop6,
        gtstp::Cstop6,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            gtstp::Cstop6,
            gtstp::Cstop6,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstop7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        gtstp::Cstop7,
        gtstp::Cstop7,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            gtstp::Cstop7,
            gtstp::Cstop7,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstop8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gtstp::Cstop8,
        gtstp::Cstop8,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gtstp::Cstop8,
            gtstp::Cstop8,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstop9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        gtstp::Cstop9,
        gtstp::Cstop9,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            gtstp::Cstop9,
            gtstp::Cstop9,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstop10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        gtstp::Cstop10,
        gtstp::Cstop10,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            gtstp::Cstop10,
            gtstp::Cstop10,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstop11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        gtstp::Cstop11,
        gtstp::Cstop11,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            gtstp::Cstop11,
            gtstp::Cstop11,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstop12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        gtstp::Cstop12,
        gtstp::Cstop12,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            gtstp::Cstop12,
            gtstp::Cstop12,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstop13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        gtstp::Cstop13,
        gtstp::Cstop13,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            gtstp::Cstop13,
            gtstp::Cstop13,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstop14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        gtstp::Cstop14,
        gtstp::Cstop14,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            gtstp::Cstop14,
            gtstp::Cstop14,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstop15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        gtstp::Cstop15,
        gtstp::Cstop15,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            gtstp::Cstop15,
            gtstp::Cstop15,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstop16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        gtstp::Cstop16,
        gtstp::Cstop16,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gtstp::Cstop16,
            gtstp::Cstop16,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstop17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        gtstp::Cstop17,
        gtstp::Cstop17,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            gtstp::Cstop17,
            gtstp::Cstop17,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstop18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        gtstp::Cstop18,
        gtstp::Cstop18,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            gtstp::Cstop18,
            gtstp::Cstop18,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstop19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        gtstp::Cstop19,
        gtstp::Cstop19,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            gtstp::Cstop19,
            gtstp::Cstop19,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstop20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        gtstp::Cstop20,
        gtstp::Cstop20,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            gtstp::Cstop20,
            gtstp::Cstop20,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstop21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        gtstp::Cstop21,
        gtstp::Cstop21,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            gtstp::Cstop21,
            gtstp::Cstop21,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstop22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        gtstp::Cstop22,
        gtstp::Cstop22,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            gtstp::Cstop22,
            gtstp::Cstop22,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstop23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        gtstp::Cstop23,
        gtstp::Cstop23,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            gtstp::Cstop23,
            gtstp::Cstop23,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstop24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        gtstp::Cstop24,
        gtstp::Cstop24,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            gtstp::Cstop24,
            gtstp::Cstop24,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstop25(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        gtstp::Cstop25,
        gtstp::Cstop25,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            gtstp::Cstop25,
            gtstp::Cstop25,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstop26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        gtstp::Cstop26,
        gtstp::Cstop26,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            gtstp::Cstop26,
            gtstp::Cstop26,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstop27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        gtstp::Cstop27,
        gtstp::Cstop27,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            gtstp::Cstop27,
            gtstp::Cstop27,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstop28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        gtstp::Cstop28,
        gtstp::Cstop28,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            gtstp::Cstop28,
            gtstp::Cstop28,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstop29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        gtstp::Cstop29,
        gtstp::Cstop29,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            gtstp::Cstop29,
            gtstp::Cstop29,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstop30(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        gtstp::Cstop30,
        gtstp::Cstop30,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            gtstp::Cstop30,
            gtstp::Cstop30,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstop31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        gtstp::Cstop31,
        gtstp::Cstop31,
        Gtstp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            gtstp::Cstop31,
            gtstp::Cstop31,
            Gtstp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtstp {
    #[inline(always)]
    fn default() -> Gtstp {
        <crate::RegValueT<Gtstp_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod gtstp {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop0_SPEC;
    pub type Cstop0 = crate::EnumBitfieldStruct<u8, Cstop0_SPEC>;
    impl Cstop0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop1_SPEC;
    pub type Cstop1 = crate::EnumBitfieldStruct<u8, Cstop1_SPEC>;
    impl Cstop1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop2_SPEC;
    pub type Cstop2 = crate::EnumBitfieldStruct<u8, Cstop2_SPEC>;
    impl Cstop2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop3_SPEC;
    pub type Cstop3 = crate::EnumBitfieldStruct<u8, Cstop3_SPEC>;
    impl Cstop3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop4_SPEC;
    pub type Cstop4 = crate::EnumBitfieldStruct<u8, Cstop4_SPEC>;
    impl Cstop4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop5_SPEC;
    pub type Cstop5 = crate::EnumBitfieldStruct<u8, Cstop5_SPEC>;
    impl Cstop5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop6_SPEC;
    pub type Cstop6 = crate::EnumBitfieldStruct<u8, Cstop6_SPEC>;
    impl Cstop6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop7_SPEC;
    pub type Cstop7 = crate::EnumBitfieldStruct<u8, Cstop7_SPEC>;
    impl Cstop7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop8_SPEC;
    pub type Cstop8 = crate::EnumBitfieldStruct<u8, Cstop8_SPEC>;
    impl Cstop8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop9_SPEC;
    pub type Cstop9 = crate::EnumBitfieldStruct<u8, Cstop9_SPEC>;
    impl Cstop9 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop10_SPEC;
    pub type Cstop10 = crate::EnumBitfieldStruct<u8, Cstop10_SPEC>;
    impl Cstop10 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop11_SPEC;
    pub type Cstop11 = crate::EnumBitfieldStruct<u8, Cstop11_SPEC>;
    impl Cstop11 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop12_SPEC;
    pub type Cstop12 = crate::EnumBitfieldStruct<u8, Cstop12_SPEC>;
    impl Cstop12 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop13_SPEC;
    pub type Cstop13 = crate::EnumBitfieldStruct<u8, Cstop13_SPEC>;
    impl Cstop13 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop14_SPEC;
    pub type Cstop14 = crate::EnumBitfieldStruct<u8, Cstop14_SPEC>;
    impl Cstop14 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop15_SPEC;
    pub type Cstop15 = crate::EnumBitfieldStruct<u8, Cstop15_SPEC>;
    impl Cstop15 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop16_SPEC;
    pub type Cstop16 = crate::EnumBitfieldStruct<u8, Cstop16_SPEC>;
    impl Cstop16 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop17_SPEC;
    pub type Cstop17 = crate::EnumBitfieldStruct<u8, Cstop17_SPEC>;
    impl Cstop17 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop18_SPEC;
    pub type Cstop18 = crate::EnumBitfieldStruct<u8, Cstop18_SPEC>;
    impl Cstop18 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop19_SPEC;
    pub type Cstop19 = crate::EnumBitfieldStruct<u8, Cstop19_SPEC>;
    impl Cstop19 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop20_SPEC;
    pub type Cstop20 = crate::EnumBitfieldStruct<u8, Cstop20_SPEC>;
    impl Cstop20 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop21_SPEC;
    pub type Cstop21 = crate::EnumBitfieldStruct<u8, Cstop21_SPEC>;
    impl Cstop21 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop22_SPEC;
    pub type Cstop22 = crate::EnumBitfieldStruct<u8, Cstop22_SPEC>;
    impl Cstop22 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop23_SPEC;
    pub type Cstop23 = crate::EnumBitfieldStruct<u8, Cstop23_SPEC>;
    impl Cstop23 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop24_SPEC;
    pub type Cstop24 = crate::EnumBitfieldStruct<u8, Cstop24_SPEC>;
    impl Cstop24 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop25_SPEC;
    pub type Cstop25 = crate::EnumBitfieldStruct<u8, Cstop25_SPEC>;
    impl Cstop25 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop26_SPEC;
    pub type Cstop26 = crate::EnumBitfieldStruct<u8, Cstop26_SPEC>;
    impl Cstop26 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop27_SPEC;
    pub type Cstop27 = crate::EnumBitfieldStruct<u8, Cstop27_SPEC>;
    impl Cstop27 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop28_SPEC;
    pub type Cstop28 = crate::EnumBitfieldStruct<u8, Cstop28_SPEC>;
    impl Cstop28 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop29_SPEC;
    pub type Cstop29 = crate::EnumBitfieldStruct<u8, Cstop29_SPEC>;
    impl Cstop29 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop30_SPEC;
    pub type Cstop30 = crate::EnumBitfieldStruct<u8, Cstop30_SPEC>;
    impl Cstop30 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop31_SPEC;
    pub type Cstop31 = crate::EnumBitfieldStruct<u8, Cstop31_SPEC>;
    impl Cstop31 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtclr_SPEC;
impl crate::sealed::RegSpec for Gtclr_SPEC {
    type DataType = u32;
}

pub type Gtclr = crate::RegValueT<Gtclr_SPEC>;

impl Gtclr {
    #[inline(always)]
    pub fn cclr0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtclr::Cclr0,
        gtclr::Cclr0,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtclr::Cclr0,
            gtclr::Cclr0,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cclr1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gtclr::Cclr1,
        gtclr::Cclr1,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gtclr::Cclr1,
            gtclr::Cclr1,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cclr2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gtclr::Cclr2,
        gtclr::Cclr2,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gtclr::Cclr2,
            gtclr::Cclr2,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cclr3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gtclr::Cclr3,
        gtclr::Cclr3,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gtclr::Cclr3,
            gtclr::Cclr3,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cclr4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gtclr::Cclr4,
        gtclr::Cclr4,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gtclr::Cclr4,
            gtclr::Cclr4,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cclr5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gtclr::Cclr5,
        gtclr::Cclr5,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gtclr::Cclr5,
            gtclr::Cclr5,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cclr6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        gtclr::Cclr6,
        gtclr::Cclr6,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            gtclr::Cclr6,
            gtclr::Cclr6,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cclr7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        gtclr::Cclr7,
        gtclr::Cclr7,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            gtclr::Cclr7,
            gtclr::Cclr7,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cclr8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gtclr::Cclr8,
        gtclr::Cclr8,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gtclr::Cclr8,
            gtclr::Cclr8,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cclr9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        gtclr::Cclr9,
        gtclr::Cclr9,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            gtclr::Cclr9,
            gtclr::Cclr9,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cclr10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        gtclr::Cclr10,
        gtclr::Cclr10,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            gtclr::Cclr10,
            gtclr::Cclr10,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cclr11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        gtclr::Cclr11,
        gtclr::Cclr11,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            gtclr::Cclr11,
            gtclr::Cclr11,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cclr12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        gtclr::Cclr12,
        gtclr::Cclr12,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            gtclr::Cclr12,
            gtclr::Cclr12,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cclr13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        gtclr::Cclr13,
        gtclr::Cclr13,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            gtclr::Cclr13,
            gtclr::Cclr13,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cclr14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        gtclr::Cclr14,
        gtclr::Cclr14,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            gtclr::Cclr14,
            gtclr::Cclr14,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cclr15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        gtclr::Cclr15,
        gtclr::Cclr15,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            gtclr::Cclr15,
            gtclr::Cclr15,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cclr16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        gtclr::Cclr16,
        gtclr::Cclr16,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gtclr::Cclr16,
            gtclr::Cclr16,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cclr17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        gtclr::Cclr17,
        gtclr::Cclr17,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            gtclr::Cclr17,
            gtclr::Cclr17,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cclr18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        gtclr::Cclr18,
        gtclr::Cclr18,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            gtclr::Cclr18,
            gtclr::Cclr18,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cclr19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        gtclr::Cclr19,
        gtclr::Cclr19,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            gtclr::Cclr19,
            gtclr::Cclr19,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cclr20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        gtclr::Cclr20,
        gtclr::Cclr20,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            gtclr::Cclr20,
            gtclr::Cclr20,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cclr21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        gtclr::Cclr21,
        gtclr::Cclr21,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            gtclr::Cclr21,
            gtclr::Cclr21,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cclr22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        gtclr::Cclr22,
        gtclr::Cclr22,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            gtclr::Cclr22,
            gtclr::Cclr22,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cclr23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        gtclr::Cclr23,
        gtclr::Cclr23,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            gtclr::Cclr23,
            gtclr::Cclr23,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cclr24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        gtclr::Cclr24,
        gtclr::Cclr24,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            gtclr::Cclr24,
            gtclr::Cclr24,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cclr25(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        gtclr::Cclr25,
        gtclr::Cclr25,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            gtclr::Cclr25,
            gtclr::Cclr25,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cclr26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        gtclr::Cclr26,
        gtclr::Cclr26,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            gtclr::Cclr26,
            gtclr::Cclr26,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cclr27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        gtclr::Cclr27,
        gtclr::Cclr27,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            gtclr::Cclr27,
            gtclr::Cclr27,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cclr28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        gtclr::Cclr28,
        gtclr::Cclr28,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            gtclr::Cclr28,
            gtclr::Cclr28,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cclr29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        gtclr::Cclr29,
        gtclr::Cclr29,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            gtclr::Cclr29,
            gtclr::Cclr29,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cclr30(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        gtclr::Cclr30,
        gtclr::Cclr30,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            gtclr::Cclr30,
            gtclr::Cclr30,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cclr31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        gtclr::Cclr31,
        gtclr::Cclr31,
        Gtclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            gtclr::Cclr31,
            gtclr::Cclr31,
            Gtclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtclr {
    #[inline(always)]
    fn default() -> Gtclr {
        <crate::RegValueT<Gtclr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtclr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr0_SPEC;
    pub type Cclr0 = crate::EnumBitfieldStruct<u8, Cclr0_SPEC>;
    impl Cclr0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr1_SPEC;
    pub type Cclr1 = crate::EnumBitfieldStruct<u8, Cclr1_SPEC>;
    impl Cclr1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr2_SPEC;
    pub type Cclr2 = crate::EnumBitfieldStruct<u8, Cclr2_SPEC>;
    impl Cclr2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr3_SPEC;
    pub type Cclr3 = crate::EnumBitfieldStruct<u8, Cclr3_SPEC>;
    impl Cclr3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr4_SPEC;
    pub type Cclr4 = crate::EnumBitfieldStruct<u8, Cclr4_SPEC>;
    impl Cclr4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr5_SPEC;
    pub type Cclr5 = crate::EnumBitfieldStruct<u8, Cclr5_SPEC>;
    impl Cclr5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr6_SPEC;
    pub type Cclr6 = crate::EnumBitfieldStruct<u8, Cclr6_SPEC>;
    impl Cclr6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr7_SPEC;
    pub type Cclr7 = crate::EnumBitfieldStruct<u8, Cclr7_SPEC>;
    impl Cclr7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr8_SPEC;
    pub type Cclr8 = crate::EnumBitfieldStruct<u8, Cclr8_SPEC>;
    impl Cclr8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr9_SPEC;
    pub type Cclr9 = crate::EnumBitfieldStruct<u8, Cclr9_SPEC>;
    impl Cclr9 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr10_SPEC;
    pub type Cclr10 = crate::EnumBitfieldStruct<u8, Cclr10_SPEC>;
    impl Cclr10 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr11_SPEC;
    pub type Cclr11 = crate::EnumBitfieldStruct<u8, Cclr11_SPEC>;
    impl Cclr11 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr12_SPEC;
    pub type Cclr12 = crate::EnumBitfieldStruct<u8, Cclr12_SPEC>;
    impl Cclr12 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr13_SPEC;
    pub type Cclr13 = crate::EnumBitfieldStruct<u8, Cclr13_SPEC>;
    impl Cclr13 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr14_SPEC;
    pub type Cclr14 = crate::EnumBitfieldStruct<u8, Cclr14_SPEC>;
    impl Cclr14 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr15_SPEC;
    pub type Cclr15 = crate::EnumBitfieldStruct<u8, Cclr15_SPEC>;
    impl Cclr15 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr16_SPEC;
    pub type Cclr16 = crate::EnumBitfieldStruct<u8, Cclr16_SPEC>;
    impl Cclr16 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr17_SPEC;
    pub type Cclr17 = crate::EnumBitfieldStruct<u8, Cclr17_SPEC>;
    impl Cclr17 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr18_SPEC;
    pub type Cclr18 = crate::EnumBitfieldStruct<u8, Cclr18_SPEC>;
    impl Cclr18 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr19_SPEC;
    pub type Cclr19 = crate::EnumBitfieldStruct<u8, Cclr19_SPEC>;
    impl Cclr19 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr20_SPEC;
    pub type Cclr20 = crate::EnumBitfieldStruct<u8, Cclr20_SPEC>;
    impl Cclr20 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr21_SPEC;
    pub type Cclr21 = crate::EnumBitfieldStruct<u8, Cclr21_SPEC>;
    impl Cclr21 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr22_SPEC;
    pub type Cclr22 = crate::EnumBitfieldStruct<u8, Cclr22_SPEC>;
    impl Cclr22 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr23_SPEC;
    pub type Cclr23 = crate::EnumBitfieldStruct<u8, Cclr23_SPEC>;
    impl Cclr23 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr24_SPEC;
    pub type Cclr24 = crate::EnumBitfieldStruct<u8, Cclr24_SPEC>;
    impl Cclr24 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr25_SPEC;
    pub type Cclr25 = crate::EnumBitfieldStruct<u8, Cclr25_SPEC>;
    impl Cclr25 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr26_SPEC;
    pub type Cclr26 = crate::EnumBitfieldStruct<u8, Cclr26_SPEC>;
    impl Cclr26 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr27_SPEC;
    pub type Cclr27 = crate::EnumBitfieldStruct<u8, Cclr27_SPEC>;
    impl Cclr27 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr28_SPEC;
    pub type Cclr28 = crate::EnumBitfieldStruct<u8, Cclr28_SPEC>;
    impl Cclr28 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr29_SPEC;
    pub type Cclr29 = crate::EnumBitfieldStruct<u8, Cclr29_SPEC>;
    impl Cclr29 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr30_SPEC;
    pub type Cclr30 = crate::EnumBitfieldStruct<u8, Cclr30_SPEC>;
    impl Cclr30 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr31_SPEC;
    pub type Cclr31 = crate::EnumBitfieldStruct<u8, Cclr31_SPEC>;
    impl Cclr31 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtssr_SPEC;
impl crate::sealed::RegSpec for Gtssr_SPEC {
    type DataType = u32;
}

pub type Gtssr = crate::RegValueT<Gtssr_SPEC>;

impl Gtssr {
    #[inline(always)]
    pub fn ssgtrgar(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtssr::Ssgtrgar,
        gtssr::Ssgtrgar,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtssr::Ssgtrgar,
            gtssr::Ssgtrgar,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ssgtrgaf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gtssr::Ssgtrgaf,
        gtssr::Ssgtrgaf,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gtssr::Ssgtrgaf,
            gtssr::Ssgtrgaf,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ssgtrgbr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gtssr::Ssgtrgbr,
        gtssr::Ssgtrgbr,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gtssr::Ssgtrgbr,
            gtssr::Ssgtrgbr,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ssgtrgbf(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gtssr::Ssgtrgbf,
        gtssr::Ssgtrgbf,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gtssr::Ssgtrgbf,
            gtssr::Ssgtrgbf,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ssgtrgcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gtssr::Ssgtrgcr,
        gtssr::Ssgtrgcr,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gtssr::Ssgtrgcr,
            gtssr::Ssgtrgcr,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ssgtrgcf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gtssr::Ssgtrgcf,
        gtssr::Ssgtrgcf,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gtssr::Ssgtrgcf,
            gtssr::Ssgtrgcf,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ssgtrgdr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        gtssr::Ssgtrgdr,
        gtssr::Ssgtrgdr,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            gtssr::Ssgtrgdr,
            gtssr::Ssgtrgdr,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ssgtrgdf(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        gtssr::Ssgtrgdf,
        gtssr::Ssgtrgdf,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            gtssr::Ssgtrgdf,
            gtssr::Ssgtrgdf,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sscarbl(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gtssr::Sscarbl,
        gtssr::Sscarbl,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gtssr::Sscarbl,
            gtssr::Sscarbl,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sscarbh(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        gtssr::Sscarbh,
        gtssr::Sscarbh,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            gtssr::Sscarbh,
            gtssr::Sscarbh,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sscafbl(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        gtssr::Sscafbl,
        gtssr::Sscafbl,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            gtssr::Sscafbl,
            gtssr::Sscafbl,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sscafbh(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        gtssr::Sscafbh,
        gtssr::Sscafbh,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            gtssr::Sscafbh,
            gtssr::Sscafbh,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sscbral(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        gtssr::Sscbral,
        gtssr::Sscbral,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            gtssr::Sscbral,
            gtssr::Sscbral,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sscbrah(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        gtssr::Sscbrah,
        gtssr::Sscbrah,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            gtssr::Sscbrah,
            gtssr::Sscbrah,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sscbfal(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        gtssr::Sscbfal,
        gtssr::Sscbfal,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            gtssr::Sscbfal,
            gtssr::Sscbfal,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sscbfah(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        gtssr::Sscbfah,
        gtssr::Sscbfah,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            gtssr::Sscbfah,
            gtssr::Sscbfah,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sselca(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        gtssr::Sselca,
        gtssr::Sselca,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gtssr::Sselca,
            gtssr::Sselca,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sselcb(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        gtssr::Sselcb,
        gtssr::Sselcb,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            gtssr::Sselcb,
            gtssr::Sselcb,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sselcc(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        gtssr::Sselcc,
        gtssr::Sselcc,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            gtssr::Sselcc,
            gtssr::Sselcc,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sselcd(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        gtssr::Sselcd,
        gtssr::Sselcd,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            gtssr::Sselcd,
            gtssr::Sselcd,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sselce(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        gtssr::Sselce,
        gtssr::Sselce,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            gtssr::Sselce,
            gtssr::Sselce,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sselcf(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        gtssr::Sselcf,
        gtssr::Sselcf,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            gtssr::Sselcf,
            gtssr::Sselcf,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sselcg(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        gtssr::Sselcg,
        gtssr::Sselcg,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            gtssr::Sselcg,
            gtssr::Sselcg,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sselch(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        gtssr::Sselch,
        gtssr::Sselch,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            gtssr::Sselch,
            gtssr::Sselch,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstrt(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        gtssr::Cstrt,
        gtssr::Cstrt,
        Gtssr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            gtssr::Cstrt,
            gtssr::Cstrt,
            Gtssr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtssr {
    #[inline(always)]
    fn default() -> Gtssr {
        <crate::RegValueT<Gtssr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtssr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssgtrgar_SPEC;
    pub type Ssgtrgar = crate::EnumBitfieldStruct<u8, Ssgtrgar_SPEC>;
    impl Ssgtrgar {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssgtrgaf_SPEC;
    pub type Ssgtrgaf = crate::EnumBitfieldStruct<u8, Ssgtrgaf_SPEC>;
    impl Ssgtrgaf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssgtrgbr_SPEC;
    pub type Ssgtrgbr = crate::EnumBitfieldStruct<u8, Ssgtrgbr_SPEC>;
    impl Ssgtrgbr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssgtrgbf_SPEC;
    pub type Ssgtrgbf = crate::EnumBitfieldStruct<u8, Ssgtrgbf_SPEC>;
    impl Ssgtrgbf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssgtrgcr_SPEC;
    pub type Ssgtrgcr = crate::EnumBitfieldStruct<u8, Ssgtrgcr_SPEC>;
    impl Ssgtrgcr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssgtrgcf_SPEC;
    pub type Ssgtrgcf = crate::EnumBitfieldStruct<u8, Ssgtrgcf_SPEC>;
    impl Ssgtrgcf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssgtrgdr_SPEC;
    pub type Ssgtrgdr = crate::EnumBitfieldStruct<u8, Ssgtrgdr_SPEC>;
    impl Ssgtrgdr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssgtrgdf_SPEC;
    pub type Ssgtrgdf = crate::EnumBitfieldStruct<u8, Ssgtrgdf_SPEC>;
    impl Ssgtrgdf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sscarbl_SPEC;
    pub type Sscarbl = crate::EnumBitfieldStruct<u8, Sscarbl_SPEC>;
    impl Sscarbl {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sscarbh_SPEC;
    pub type Sscarbh = crate::EnumBitfieldStruct<u8, Sscarbh_SPEC>;
    impl Sscarbh {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sscafbl_SPEC;
    pub type Sscafbl = crate::EnumBitfieldStruct<u8, Sscafbl_SPEC>;
    impl Sscafbl {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sscafbh_SPEC;
    pub type Sscafbh = crate::EnumBitfieldStruct<u8, Sscafbh_SPEC>;
    impl Sscafbh {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sscbral_SPEC;
    pub type Sscbral = crate::EnumBitfieldStruct<u8, Sscbral_SPEC>;
    impl Sscbral {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sscbrah_SPEC;
    pub type Sscbrah = crate::EnumBitfieldStruct<u8, Sscbrah_SPEC>;
    impl Sscbrah {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sscbfal_SPEC;
    pub type Sscbfal = crate::EnumBitfieldStruct<u8, Sscbfal_SPEC>;
    impl Sscbfal {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sscbfah_SPEC;
    pub type Sscbfah = crate::EnumBitfieldStruct<u8, Sscbfah_SPEC>;
    impl Sscbfah {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sselca_SPEC;
    pub type Sselca = crate::EnumBitfieldStruct<u8, Sselca_SPEC>;
    impl Sselca {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sselcb_SPEC;
    pub type Sselcb = crate::EnumBitfieldStruct<u8, Sselcb_SPEC>;
    impl Sselcb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sselcc_SPEC;
    pub type Sselcc = crate::EnumBitfieldStruct<u8, Sselcc_SPEC>;
    impl Sselcc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sselcd_SPEC;
    pub type Sselcd = crate::EnumBitfieldStruct<u8, Sselcd_SPEC>;
    impl Sselcd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sselce_SPEC;
    pub type Sselce = crate::EnumBitfieldStruct<u8, Sselce_SPEC>;
    impl Sselce {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sselcf_SPEC;
    pub type Sselcf = crate::EnumBitfieldStruct<u8, Sselcf_SPEC>;
    impl Sselcf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sselcg_SPEC;
    pub type Sselcg = crate::EnumBitfieldStruct<u8, Sselcg_SPEC>;
    impl Sselcg {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sselch_SPEC;
    pub type Sselch = crate::EnumBitfieldStruct<u8, Sselch_SPEC>;
    impl Sselch {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt_SPEC;
    pub type Cstrt = crate::EnumBitfieldStruct<u8, Cstrt_SPEC>;
    impl Cstrt {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtpsr_SPEC;
impl crate::sealed::RegSpec for Gtpsr_SPEC {
    type DataType = u32;
}

pub type Gtpsr = crate::RegValueT<Gtpsr_SPEC>;

impl Gtpsr {
    #[inline(always)]
    pub fn psgtrgar(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtpsr::Psgtrgar,
        gtpsr::Psgtrgar,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtpsr::Psgtrgar,
            gtpsr::Psgtrgar,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psgtrgaf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gtpsr::Psgtrgaf,
        gtpsr::Psgtrgaf,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gtpsr::Psgtrgaf,
            gtpsr::Psgtrgaf,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psgtrgbr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gtpsr::Psgtrgbr,
        gtpsr::Psgtrgbr,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gtpsr::Psgtrgbr,
            gtpsr::Psgtrgbr,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psgtrgbf(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gtpsr::Psgtrgbf,
        gtpsr::Psgtrgbf,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gtpsr::Psgtrgbf,
            gtpsr::Psgtrgbf,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psgtrgcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gtpsr::Psgtrgcr,
        gtpsr::Psgtrgcr,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gtpsr::Psgtrgcr,
            gtpsr::Psgtrgcr,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psgtrgcf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gtpsr::Psgtrgcf,
        gtpsr::Psgtrgcf,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gtpsr::Psgtrgcf,
            gtpsr::Psgtrgcf,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psgtrgdr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        gtpsr::Psgtrgdr,
        gtpsr::Psgtrgdr,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            gtpsr::Psgtrgdr,
            gtpsr::Psgtrgdr,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psgtrgdf(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        gtpsr::Psgtrgdf,
        gtpsr::Psgtrgdf,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            gtpsr::Psgtrgdf,
            gtpsr::Psgtrgdf,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pscarbl(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gtpsr::Pscarbl,
        gtpsr::Pscarbl,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gtpsr::Pscarbl,
            gtpsr::Pscarbl,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pscarbh(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        gtpsr::Pscarbh,
        gtpsr::Pscarbh,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            gtpsr::Pscarbh,
            gtpsr::Pscarbh,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pscafbl(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        gtpsr::Pscafbl,
        gtpsr::Pscafbl,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            gtpsr::Pscafbl,
            gtpsr::Pscafbl,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pscafbh(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        gtpsr::Pscafbh,
        gtpsr::Pscafbh,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            gtpsr::Pscafbh,
            gtpsr::Pscafbh,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pscbral(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        gtpsr::Pscbral,
        gtpsr::Pscbral,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            gtpsr::Pscbral,
            gtpsr::Pscbral,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pscbrah(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        gtpsr::Pscbrah,
        gtpsr::Pscbrah,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            gtpsr::Pscbrah,
            gtpsr::Pscbrah,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pscbfal(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        gtpsr::Pscbfal,
        gtpsr::Pscbfal,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            gtpsr::Pscbfal,
            gtpsr::Pscbfal,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pscbfah(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        gtpsr::Pscbfah,
        gtpsr::Pscbfah,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            gtpsr::Pscbfah,
            gtpsr::Pscbfah,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pselca(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        gtpsr::Pselca,
        gtpsr::Pselca,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gtpsr::Pselca,
            gtpsr::Pselca,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pselcb(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        gtpsr::Pselcb,
        gtpsr::Pselcb,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            gtpsr::Pselcb,
            gtpsr::Pselcb,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pselcc(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        gtpsr::Pselcc,
        gtpsr::Pselcc,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            gtpsr::Pselcc,
            gtpsr::Pselcc,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pselcd(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        gtpsr::Pselcd,
        gtpsr::Pselcd,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            gtpsr::Pselcd,
            gtpsr::Pselcd,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pselce(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        gtpsr::Pselce,
        gtpsr::Pselce,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            gtpsr::Pselce,
            gtpsr::Pselce,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pselcf(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        gtpsr::Pselcf,
        gtpsr::Pselcf,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            gtpsr::Pselcf,
            gtpsr::Pselcf,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pselcg(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        gtpsr::Pselcg,
        gtpsr::Pselcg,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            gtpsr::Pselcg,
            gtpsr::Pselcg,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pselch(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        gtpsr::Pselch,
        gtpsr::Pselch,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            gtpsr::Pselch,
            gtpsr::Pselch,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cstop(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        gtpsr::Cstop,
        gtpsr::Cstop,
        Gtpsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            gtpsr::Cstop,
            gtpsr::Cstop,
            Gtpsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtpsr {
    #[inline(always)]
    fn default() -> Gtpsr {
        <crate::RegValueT<Gtpsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtpsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psgtrgar_SPEC;
    pub type Psgtrgar = crate::EnumBitfieldStruct<u8, Psgtrgar_SPEC>;
    impl Psgtrgar {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psgtrgaf_SPEC;
    pub type Psgtrgaf = crate::EnumBitfieldStruct<u8, Psgtrgaf_SPEC>;
    impl Psgtrgaf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psgtrgbr_SPEC;
    pub type Psgtrgbr = crate::EnumBitfieldStruct<u8, Psgtrgbr_SPEC>;
    impl Psgtrgbr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psgtrgbf_SPEC;
    pub type Psgtrgbf = crate::EnumBitfieldStruct<u8, Psgtrgbf_SPEC>;
    impl Psgtrgbf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psgtrgcr_SPEC;
    pub type Psgtrgcr = crate::EnumBitfieldStruct<u8, Psgtrgcr_SPEC>;
    impl Psgtrgcr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psgtrgcf_SPEC;
    pub type Psgtrgcf = crate::EnumBitfieldStruct<u8, Psgtrgcf_SPEC>;
    impl Psgtrgcf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psgtrgdr_SPEC;
    pub type Psgtrgdr = crate::EnumBitfieldStruct<u8, Psgtrgdr_SPEC>;
    impl Psgtrgdr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psgtrgdf_SPEC;
    pub type Psgtrgdf = crate::EnumBitfieldStruct<u8, Psgtrgdf_SPEC>;
    impl Psgtrgdf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pscarbl_SPEC;
    pub type Pscarbl = crate::EnumBitfieldStruct<u8, Pscarbl_SPEC>;
    impl Pscarbl {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pscarbh_SPEC;
    pub type Pscarbh = crate::EnumBitfieldStruct<u8, Pscarbh_SPEC>;
    impl Pscarbh {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pscafbl_SPEC;
    pub type Pscafbl = crate::EnumBitfieldStruct<u8, Pscafbl_SPEC>;
    impl Pscafbl {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pscafbh_SPEC;
    pub type Pscafbh = crate::EnumBitfieldStruct<u8, Pscafbh_SPEC>;
    impl Pscafbh {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pscbral_SPEC;
    pub type Pscbral = crate::EnumBitfieldStruct<u8, Pscbral_SPEC>;
    impl Pscbral {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pscbrah_SPEC;
    pub type Pscbrah = crate::EnumBitfieldStruct<u8, Pscbrah_SPEC>;
    impl Pscbrah {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pscbfal_SPEC;
    pub type Pscbfal = crate::EnumBitfieldStruct<u8, Pscbfal_SPEC>;
    impl Pscbfal {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pscbfah_SPEC;
    pub type Pscbfah = crate::EnumBitfieldStruct<u8, Pscbfah_SPEC>;
    impl Pscbfah {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pselca_SPEC;
    pub type Pselca = crate::EnumBitfieldStruct<u8, Pselca_SPEC>;
    impl Pselca {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pselcb_SPEC;
    pub type Pselcb = crate::EnumBitfieldStruct<u8, Pselcb_SPEC>;
    impl Pselcb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pselcc_SPEC;
    pub type Pselcc = crate::EnumBitfieldStruct<u8, Pselcc_SPEC>;
    impl Pselcc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pselcd_SPEC;
    pub type Pselcd = crate::EnumBitfieldStruct<u8, Pselcd_SPEC>;
    impl Pselcd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pselce_SPEC;
    pub type Pselce = crate::EnumBitfieldStruct<u8, Pselce_SPEC>;
    impl Pselce {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pselcf_SPEC;
    pub type Pselcf = crate::EnumBitfieldStruct<u8, Pselcf_SPEC>;
    impl Pselcf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pselcg_SPEC;
    pub type Pselcg = crate::EnumBitfieldStruct<u8, Pselcg_SPEC>;
    impl Pselcg {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pselch_SPEC;
    pub type Pselch = crate::EnumBitfieldStruct<u8, Pselch_SPEC>;
    impl Pselch {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop_SPEC;
    pub type Cstop = crate::EnumBitfieldStruct<u8, Cstop_SPEC>;
    impl Cstop {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtcsr_SPEC;
impl crate::sealed::RegSpec for Gtcsr_SPEC {
    type DataType = u32;
}

pub type Gtcsr = crate::RegValueT<Gtcsr_SPEC>;

impl Gtcsr {
    #[inline(always)]
    pub fn csgtrgar(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtcsr::Csgtrgar,
        gtcsr::Csgtrgar,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtcsr::Csgtrgar,
            gtcsr::Csgtrgar,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn csgtrgaf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gtcsr::Csgtrgaf,
        gtcsr::Csgtrgaf,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gtcsr::Csgtrgaf,
            gtcsr::Csgtrgaf,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn csgtrgbr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gtcsr::Csgtrgbr,
        gtcsr::Csgtrgbr,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gtcsr::Csgtrgbr,
            gtcsr::Csgtrgbr,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn csgtrgbf(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gtcsr::Csgtrgbf,
        gtcsr::Csgtrgbf,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gtcsr::Csgtrgbf,
            gtcsr::Csgtrgbf,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn csgtrgcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gtcsr::Csgtrgcr,
        gtcsr::Csgtrgcr,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gtcsr::Csgtrgcr,
            gtcsr::Csgtrgcr,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn csgtrgcf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gtcsr::Csgtrgcf,
        gtcsr::Csgtrgcf,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gtcsr::Csgtrgcf,
            gtcsr::Csgtrgcf,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn csgtrgdr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        gtcsr::Csgtrgdr,
        gtcsr::Csgtrgdr,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            gtcsr::Csgtrgdr,
            gtcsr::Csgtrgdr,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn csgtrgdf(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        gtcsr::Csgtrgdf,
        gtcsr::Csgtrgdf,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            gtcsr::Csgtrgdf,
            gtcsr::Csgtrgdf,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cscarbl(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gtcsr::Cscarbl,
        gtcsr::Cscarbl,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gtcsr::Cscarbl,
            gtcsr::Cscarbl,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cscarbh(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        gtcsr::Cscarbh,
        gtcsr::Cscarbh,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            gtcsr::Cscarbh,
            gtcsr::Cscarbh,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cscafbl(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        gtcsr::Cscafbl,
        gtcsr::Cscafbl,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            gtcsr::Cscafbl,
            gtcsr::Cscafbl,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cscafbh(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        gtcsr::Cscafbh,
        gtcsr::Cscafbh,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            gtcsr::Cscafbh,
            gtcsr::Cscafbh,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cscbral(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        gtcsr::Cscbral,
        gtcsr::Cscbral,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            gtcsr::Cscbral,
            gtcsr::Cscbral,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cscbrah(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        gtcsr::Cscbrah,
        gtcsr::Cscbrah,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            gtcsr::Cscbrah,
            gtcsr::Cscbrah,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cscbfal(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        gtcsr::Cscbfal,
        gtcsr::Cscbfal,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            gtcsr::Cscbfal,
            gtcsr::Cscbfal,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cscbfah(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        gtcsr::Cscbfah,
        gtcsr::Cscbfah,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            gtcsr::Cscbfah,
            gtcsr::Cscbfah,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cselca(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        gtcsr::Cselca,
        gtcsr::Cselca,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gtcsr::Cselca,
            gtcsr::Cselca,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cselcb(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        gtcsr::Cselcb,
        gtcsr::Cselcb,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            gtcsr::Cselcb,
            gtcsr::Cselcb,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cselcc(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        gtcsr::Cselcc,
        gtcsr::Cselcc,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            gtcsr::Cselcc,
            gtcsr::Cselcc,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cselcd(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        gtcsr::Cselcd,
        gtcsr::Cselcd,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            gtcsr::Cselcd,
            gtcsr::Cselcd,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cselce(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        gtcsr::Cselce,
        gtcsr::Cselce,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            gtcsr::Cselce,
            gtcsr::Cselce,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cselcf(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        gtcsr::Cselcf,
        gtcsr::Cselcf,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            gtcsr::Cselcf,
            gtcsr::Cselcf,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cselcg(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        gtcsr::Cselcg,
        gtcsr::Cselcg,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            gtcsr::Cselcg,
            gtcsr::Cselcg,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cselch(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        gtcsr::Cselch,
        gtcsr::Cselch,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            gtcsr::Cselch,
            gtcsr::Cselch,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cscmsc(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        gtcsr::Cscmsc,
        gtcsr::Cscmsc,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            gtcsr::Cscmsc,
            gtcsr::Cscmsc,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cp1cce(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        gtcsr::Cp1Cce,
        gtcsr::Cp1Cce,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            gtcsr::Cp1Cce,
            gtcsr::Cp1Cce,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cclr(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        gtcsr::Cclr,
        gtcsr::Cclr,
        Gtcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            gtcsr::Cclr,
            gtcsr::Cclr,
            Gtcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtcsr {
    #[inline(always)]
    fn default() -> Gtcsr {
        <crate::RegValueT<Gtcsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtcsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csgtrgar_SPEC;
    pub type Csgtrgar = crate::EnumBitfieldStruct<u8, Csgtrgar_SPEC>;
    impl Csgtrgar {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csgtrgaf_SPEC;
    pub type Csgtrgaf = crate::EnumBitfieldStruct<u8, Csgtrgaf_SPEC>;
    impl Csgtrgaf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csgtrgbr_SPEC;
    pub type Csgtrgbr = crate::EnumBitfieldStruct<u8, Csgtrgbr_SPEC>;
    impl Csgtrgbr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csgtrgbf_SPEC;
    pub type Csgtrgbf = crate::EnumBitfieldStruct<u8, Csgtrgbf_SPEC>;
    impl Csgtrgbf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csgtrgcr_SPEC;
    pub type Csgtrgcr = crate::EnumBitfieldStruct<u8, Csgtrgcr_SPEC>;
    impl Csgtrgcr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csgtrgcf_SPEC;
    pub type Csgtrgcf = crate::EnumBitfieldStruct<u8, Csgtrgcf_SPEC>;
    impl Csgtrgcf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csgtrgdr_SPEC;
    pub type Csgtrgdr = crate::EnumBitfieldStruct<u8, Csgtrgdr_SPEC>;
    impl Csgtrgdr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csgtrgdf_SPEC;
    pub type Csgtrgdf = crate::EnumBitfieldStruct<u8, Csgtrgdf_SPEC>;
    impl Csgtrgdf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cscarbl_SPEC;
    pub type Cscarbl = crate::EnumBitfieldStruct<u8, Cscarbl_SPEC>;
    impl Cscarbl {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cscarbh_SPEC;
    pub type Cscarbh = crate::EnumBitfieldStruct<u8, Cscarbh_SPEC>;
    impl Cscarbh {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cscafbl_SPEC;
    pub type Cscafbl = crate::EnumBitfieldStruct<u8, Cscafbl_SPEC>;
    impl Cscafbl {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cscafbh_SPEC;
    pub type Cscafbh = crate::EnumBitfieldStruct<u8, Cscafbh_SPEC>;
    impl Cscafbh {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cscbral_SPEC;
    pub type Cscbral = crate::EnumBitfieldStruct<u8, Cscbral_SPEC>;
    impl Cscbral {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cscbrah_SPEC;
    pub type Cscbrah = crate::EnumBitfieldStruct<u8, Cscbrah_SPEC>;
    impl Cscbrah {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cscbfal_SPEC;
    pub type Cscbfal = crate::EnumBitfieldStruct<u8, Cscbfal_SPEC>;
    impl Cscbfal {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cscbfah_SPEC;
    pub type Cscbfah = crate::EnumBitfieldStruct<u8, Cscbfah_SPEC>;
    impl Cscbfah {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cselca_SPEC;
    pub type Cselca = crate::EnumBitfieldStruct<u8, Cselca_SPEC>;
    impl Cselca {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cselcb_SPEC;
    pub type Cselcb = crate::EnumBitfieldStruct<u8, Cselcb_SPEC>;
    impl Cselcb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cselcc_SPEC;
    pub type Cselcc = crate::EnumBitfieldStruct<u8, Cselcc_SPEC>;
    impl Cselcc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cselcd_SPEC;
    pub type Cselcd = crate::EnumBitfieldStruct<u8, Cselcd_SPEC>;
    impl Cselcd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cselce_SPEC;
    pub type Cselce = crate::EnumBitfieldStruct<u8, Cselce_SPEC>;
    impl Cselce {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cselcf_SPEC;
    pub type Cselcf = crate::EnumBitfieldStruct<u8, Cselcf_SPEC>;
    impl Cselcf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cselcg_SPEC;
    pub type Cselcg = crate::EnumBitfieldStruct<u8, Cselcg_SPEC>;
    impl Cselcg {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cselch_SPEC;
    pub type Cselch = crate::EnumBitfieldStruct<u8, Cselch_SPEC>;
    impl Cselch {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cscmsc_SPEC;
    pub type Cscmsc = crate::EnumBitfieldStruct<u8, Cscmsc_SPEC>;
    impl Cscmsc {
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
    pub struct Cp1Cce_SPEC;
    pub type Cp1Cce = crate::EnumBitfieldStruct<u8, Cp1Cce_SPEC>;
    impl Cp1Cce {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr_SPEC;
    pub type Cclr = crate::EnumBitfieldStruct<u8, Cclr_SPEC>;
    impl Cclr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gticasr_SPEC;
impl crate::sealed::RegSpec for Gticasr_SPEC {
    type DataType = u32;
}

pub type Gticasr = crate::RegValueT<Gticasr_SPEC>;

impl Gticasr {
    #[inline(always)]
    pub fn asgtrgar(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gticasr::Asgtrgar,
        gticasr::Asgtrgar,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gticasr::Asgtrgar,
            gticasr::Asgtrgar,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn asgtrgaf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gticasr::Asgtrgaf,
        gticasr::Asgtrgaf,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gticasr::Asgtrgaf,
            gticasr::Asgtrgaf,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn asgtrgbr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gticasr::Asgtrgbr,
        gticasr::Asgtrgbr,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gticasr::Asgtrgbr,
            gticasr::Asgtrgbr,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn asgtrgbf(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gticasr::Asgtrgbf,
        gticasr::Asgtrgbf,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gticasr::Asgtrgbf,
            gticasr::Asgtrgbf,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn asgtrgcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gticasr::Asgtrgcr,
        gticasr::Asgtrgcr,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gticasr::Asgtrgcr,
            gticasr::Asgtrgcr,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn asgtrgcf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gticasr::Asgtrgcf,
        gticasr::Asgtrgcf,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gticasr::Asgtrgcf,
            gticasr::Asgtrgcf,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn asgtrgdr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        gticasr::Asgtrgdr,
        gticasr::Asgtrgdr,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            gticasr::Asgtrgdr,
            gticasr::Asgtrgdr,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn asgtrgdf(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        gticasr::Asgtrgdf,
        gticasr::Asgtrgdf,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            gticasr::Asgtrgdf,
            gticasr::Asgtrgdf,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ascarbl(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gticasr::Ascarbl,
        gticasr::Ascarbl,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gticasr::Ascarbl,
            gticasr::Ascarbl,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ascarbh(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        gticasr::Ascarbh,
        gticasr::Ascarbh,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            gticasr::Ascarbh,
            gticasr::Ascarbh,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ascafbl(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        gticasr::Ascafbl,
        gticasr::Ascafbl,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            gticasr::Ascafbl,
            gticasr::Ascafbl,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ascafbh(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        gticasr::Ascafbh,
        gticasr::Ascafbh,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            gticasr::Ascafbh,
            gticasr::Ascafbh,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ascbral(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        gticasr::Ascbral,
        gticasr::Ascbral,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            gticasr::Ascbral,
            gticasr::Ascbral,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ascbrah(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        gticasr::Ascbrah,
        gticasr::Ascbrah,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            gticasr::Ascbrah,
            gticasr::Ascbrah,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ascbfal(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        gticasr::Ascbfal,
        gticasr::Ascbfal,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            gticasr::Ascbfal,
            gticasr::Ascbfal,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ascbfah(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        gticasr::Ascbfah,
        gticasr::Ascbfah,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            gticasr::Ascbfah,
            gticasr::Ascbfah,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn aselca(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        gticasr::Aselca,
        gticasr::Aselca,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gticasr::Aselca,
            gticasr::Aselca,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn aselcb(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        gticasr::Aselcb,
        gticasr::Aselcb,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            gticasr::Aselcb,
            gticasr::Aselcb,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn aselcc(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        gticasr::Aselcc,
        gticasr::Aselcc,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            gticasr::Aselcc,
            gticasr::Aselcc,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn aselcd(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        gticasr::Aselcd,
        gticasr::Aselcd,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            gticasr::Aselcd,
            gticasr::Aselcd,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn aselce(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        gticasr::Aselce,
        gticasr::Aselce,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            gticasr::Aselce,
            gticasr::Aselce,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn aselcf(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        gticasr::Aselcf,
        gticasr::Aselcf,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            gticasr::Aselcf,
            gticasr::Aselcf,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn aselcg(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        gticasr::Aselcg,
        gticasr::Aselcg,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            gticasr::Aselcg,
            gticasr::Aselcg,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn aselch(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        gticasr::Aselch,
        gticasr::Aselch,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            gticasr::Aselch,
            gticasr::Aselch,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn asoc(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        gticasr::Asoc,
        gticasr::Asoc,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            gticasr::Asoc,
            gticasr::Asoc,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gticasr {
    #[inline(always)]
    fn default() -> Gticasr {
        <crate::RegValueT<Gticasr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gticasr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asgtrgar_SPEC;
    pub type Asgtrgar = crate::EnumBitfieldStruct<u8, Asgtrgar_SPEC>;
    impl Asgtrgar {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asgtrgaf_SPEC;
    pub type Asgtrgaf = crate::EnumBitfieldStruct<u8, Asgtrgaf_SPEC>;
    impl Asgtrgaf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asgtrgbr_SPEC;
    pub type Asgtrgbr = crate::EnumBitfieldStruct<u8, Asgtrgbr_SPEC>;
    impl Asgtrgbr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asgtrgbf_SPEC;
    pub type Asgtrgbf = crate::EnumBitfieldStruct<u8, Asgtrgbf_SPEC>;
    impl Asgtrgbf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asgtrgcr_SPEC;
    pub type Asgtrgcr = crate::EnumBitfieldStruct<u8, Asgtrgcr_SPEC>;
    impl Asgtrgcr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asgtrgcf_SPEC;
    pub type Asgtrgcf = crate::EnumBitfieldStruct<u8, Asgtrgcf_SPEC>;
    impl Asgtrgcf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asgtrgdr_SPEC;
    pub type Asgtrgdr = crate::EnumBitfieldStruct<u8, Asgtrgdr_SPEC>;
    impl Asgtrgdr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asgtrgdf_SPEC;
    pub type Asgtrgdf = crate::EnumBitfieldStruct<u8, Asgtrgdf_SPEC>;
    impl Asgtrgdf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ascarbl_SPEC;
    pub type Ascarbl = crate::EnumBitfieldStruct<u8, Ascarbl_SPEC>;
    impl Ascarbl {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ascarbh_SPEC;
    pub type Ascarbh = crate::EnumBitfieldStruct<u8, Ascarbh_SPEC>;
    impl Ascarbh {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ascafbl_SPEC;
    pub type Ascafbl = crate::EnumBitfieldStruct<u8, Ascafbl_SPEC>;
    impl Ascafbl {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ascafbh_SPEC;
    pub type Ascafbh = crate::EnumBitfieldStruct<u8, Ascafbh_SPEC>;
    impl Ascafbh {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ascbral_SPEC;
    pub type Ascbral = crate::EnumBitfieldStruct<u8, Ascbral_SPEC>;
    impl Ascbral {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ascbrah_SPEC;
    pub type Ascbrah = crate::EnumBitfieldStruct<u8, Ascbrah_SPEC>;
    impl Ascbrah {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ascbfal_SPEC;
    pub type Ascbfal = crate::EnumBitfieldStruct<u8, Ascbfal_SPEC>;
    impl Ascbfal {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ascbfah_SPEC;
    pub type Ascbfah = crate::EnumBitfieldStruct<u8, Ascbfah_SPEC>;
    impl Ascbfah {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aselca_SPEC;
    pub type Aselca = crate::EnumBitfieldStruct<u8, Aselca_SPEC>;
    impl Aselca {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aselcb_SPEC;
    pub type Aselcb = crate::EnumBitfieldStruct<u8, Aselcb_SPEC>;
    impl Aselcb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aselcc_SPEC;
    pub type Aselcc = crate::EnumBitfieldStruct<u8, Aselcc_SPEC>;
    impl Aselcc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aselcd_SPEC;
    pub type Aselcd = crate::EnumBitfieldStruct<u8, Aselcd_SPEC>;
    impl Aselcd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aselce_SPEC;
    pub type Aselce = crate::EnumBitfieldStruct<u8, Aselce_SPEC>;
    impl Aselce {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aselcf_SPEC;
    pub type Aselcf = crate::EnumBitfieldStruct<u8, Aselcf_SPEC>;
    impl Aselcf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aselcg_SPEC;
    pub type Aselcg = crate::EnumBitfieldStruct<u8, Aselcg_SPEC>;
    impl Aselcg {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aselch_SPEC;
    pub type Aselch = crate::EnumBitfieldStruct<u8, Aselch_SPEC>;
    impl Aselch {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asoc_SPEC;
    pub type Asoc = crate::EnumBitfieldStruct<u8, Asoc_SPEC>;
    impl Asoc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gticbsr_SPEC;
impl crate::sealed::RegSpec for Gticbsr_SPEC {
    type DataType = u32;
}

pub type Gticbsr = crate::RegValueT<Gticbsr_SPEC>;

impl Gticbsr {
    #[inline(always)]
    pub fn bsgtrgar(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gticbsr::Bsgtrgar,
        gticbsr::Bsgtrgar,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gticbsr::Bsgtrgar,
            gticbsr::Bsgtrgar,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bsgtrgaf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gticbsr::Bsgtrgaf,
        gticbsr::Bsgtrgaf,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gticbsr::Bsgtrgaf,
            gticbsr::Bsgtrgaf,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bsgtrgbr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gticbsr::Bsgtrgbr,
        gticbsr::Bsgtrgbr,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gticbsr::Bsgtrgbr,
            gticbsr::Bsgtrgbr,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bsgtrgbf(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gticbsr::Bsgtrgbf,
        gticbsr::Bsgtrgbf,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gticbsr::Bsgtrgbf,
            gticbsr::Bsgtrgbf,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bsgtrgcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gticbsr::Bsgtrgcr,
        gticbsr::Bsgtrgcr,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gticbsr::Bsgtrgcr,
            gticbsr::Bsgtrgcr,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bsgtrgcf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gticbsr::Bsgtrgcf,
        gticbsr::Bsgtrgcf,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gticbsr::Bsgtrgcf,
            gticbsr::Bsgtrgcf,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bsgtrgdr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        gticbsr::Bsgtrgdr,
        gticbsr::Bsgtrgdr,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            gticbsr::Bsgtrgdr,
            gticbsr::Bsgtrgdr,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bsgtrgdf(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        gticbsr::Bsgtrgdf,
        gticbsr::Bsgtrgdf,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            gticbsr::Bsgtrgdf,
            gticbsr::Bsgtrgdf,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bscarbl(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gticbsr::Bscarbl,
        gticbsr::Bscarbl,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gticbsr::Bscarbl,
            gticbsr::Bscarbl,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bscarbh(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        gticbsr::Bscarbh,
        gticbsr::Bscarbh,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            gticbsr::Bscarbh,
            gticbsr::Bscarbh,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bscafbl(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        gticbsr::Bscafbl,
        gticbsr::Bscafbl,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            gticbsr::Bscafbl,
            gticbsr::Bscafbl,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bscafbh(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        gticbsr::Bscafbh,
        gticbsr::Bscafbh,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            gticbsr::Bscafbh,
            gticbsr::Bscafbh,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bscbral(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        gticbsr::Bscbral,
        gticbsr::Bscbral,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            gticbsr::Bscbral,
            gticbsr::Bscbral,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bscbrah(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        gticbsr::Bscbrah,
        gticbsr::Bscbrah,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            gticbsr::Bscbrah,
            gticbsr::Bscbrah,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bscbfal(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        gticbsr::Bscbfal,
        gticbsr::Bscbfal,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            gticbsr::Bscbfal,
            gticbsr::Bscbfal,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bscbfah(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        gticbsr::Bscbfah,
        gticbsr::Bscbfah,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            gticbsr::Bscbfah,
            gticbsr::Bscbfah,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bselca(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        gticbsr::Bselca,
        gticbsr::Bselca,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gticbsr::Bselca,
            gticbsr::Bselca,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bselcb(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        gticbsr::Bselcb,
        gticbsr::Bselcb,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            gticbsr::Bselcb,
            gticbsr::Bselcb,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bselcc(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        gticbsr::Bselcc,
        gticbsr::Bselcc,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            gticbsr::Bselcc,
            gticbsr::Bselcc,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bselcd(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        gticbsr::Bselcd,
        gticbsr::Bselcd,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            gticbsr::Bselcd,
            gticbsr::Bselcd,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bselce(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        gticbsr::Bselce,
        gticbsr::Bselce,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            gticbsr::Bselce,
            gticbsr::Bselce,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bselcf(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        gticbsr::Bselcf,
        gticbsr::Bselcf,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            gticbsr::Bselcf,
            gticbsr::Bselcf,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bselcg(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        gticbsr::Bselcg,
        gticbsr::Bselcg,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            gticbsr::Bselcg,
            gticbsr::Bselcg,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bselch(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        gticbsr::Bselch,
        gticbsr::Bselch,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            gticbsr::Bselch,
            gticbsr::Bselch,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bsoc(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        gticbsr::Bsoc,
        gticbsr::Bsoc,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            gticbsr::Bsoc,
            gticbsr::Bsoc,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gticbsr {
    #[inline(always)]
    fn default() -> Gticbsr {
        <crate::RegValueT<Gticbsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gticbsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsgtrgar_SPEC;
    pub type Bsgtrgar = crate::EnumBitfieldStruct<u8, Bsgtrgar_SPEC>;
    impl Bsgtrgar {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsgtrgaf_SPEC;
    pub type Bsgtrgaf = crate::EnumBitfieldStruct<u8, Bsgtrgaf_SPEC>;
    impl Bsgtrgaf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsgtrgbr_SPEC;
    pub type Bsgtrgbr = crate::EnumBitfieldStruct<u8, Bsgtrgbr_SPEC>;
    impl Bsgtrgbr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsgtrgbf_SPEC;
    pub type Bsgtrgbf = crate::EnumBitfieldStruct<u8, Bsgtrgbf_SPEC>;
    impl Bsgtrgbf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsgtrgcr_SPEC;
    pub type Bsgtrgcr = crate::EnumBitfieldStruct<u8, Bsgtrgcr_SPEC>;
    impl Bsgtrgcr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsgtrgcf_SPEC;
    pub type Bsgtrgcf = crate::EnumBitfieldStruct<u8, Bsgtrgcf_SPEC>;
    impl Bsgtrgcf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsgtrgdr_SPEC;
    pub type Bsgtrgdr = crate::EnumBitfieldStruct<u8, Bsgtrgdr_SPEC>;
    impl Bsgtrgdr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsgtrgdf_SPEC;
    pub type Bsgtrgdf = crate::EnumBitfieldStruct<u8, Bsgtrgdf_SPEC>;
    impl Bsgtrgdf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bscarbl_SPEC;
    pub type Bscarbl = crate::EnumBitfieldStruct<u8, Bscarbl_SPEC>;
    impl Bscarbl {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bscarbh_SPEC;
    pub type Bscarbh = crate::EnumBitfieldStruct<u8, Bscarbh_SPEC>;
    impl Bscarbh {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bscafbl_SPEC;
    pub type Bscafbl = crate::EnumBitfieldStruct<u8, Bscafbl_SPEC>;
    impl Bscafbl {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bscafbh_SPEC;
    pub type Bscafbh = crate::EnumBitfieldStruct<u8, Bscafbh_SPEC>;
    impl Bscafbh {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bscbral_SPEC;
    pub type Bscbral = crate::EnumBitfieldStruct<u8, Bscbral_SPEC>;
    impl Bscbral {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bscbrah_SPEC;
    pub type Bscbrah = crate::EnumBitfieldStruct<u8, Bscbrah_SPEC>;
    impl Bscbrah {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bscbfal_SPEC;
    pub type Bscbfal = crate::EnumBitfieldStruct<u8, Bscbfal_SPEC>;
    impl Bscbfal {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bscbfah_SPEC;
    pub type Bscbfah = crate::EnumBitfieldStruct<u8, Bscbfah_SPEC>;
    impl Bscbfah {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bselca_SPEC;
    pub type Bselca = crate::EnumBitfieldStruct<u8, Bselca_SPEC>;
    impl Bselca {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bselcb_SPEC;
    pub type Bselcb = crate::EnumBitfieldStruct<u8, Bselcb_SPEC>;
    impl Bselcb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bselcc_SPEC;
    pub type Bselcc = crate::EnumBitfieldStruct<u8, Bselcc_SPEC>;
    impl Bselcc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bselcd_SPEC;
    pub type Bselcd = crate::EnumBitfieldStruct<u8, Bselcd_SPEC>;
    impl Bselcd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bselce_SPEC;
    pub type Bselce = crate::EnumBitfieldStruct<u8, Bselce_SPEC>;
    impl Bselce {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bselcf_SPEC;
    pub type Bselcf = crate::EnumBitfieldStruct<u8, Bselcf_SPEC>;
    impl Bselcf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bselcg_SPEC;
    pub type Bselcg = crate::EnumBitfieldStruct<u8, Bselcg_SPEC>;
    impl Bselcg {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bselch_SPEC;
    pub type Bselch = crate::EnumBitfieldStruct<u8, Bselch_SPEC>;
    impl Bselch {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsoc_SPEC;
    pub type Bsoc = crate::EnumBitfieldStruct<u8, Bsoc_SPEC>;
    impl Bsoc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtcr_SPEC;
impl crate::sealed::RegSpec for Gtcr_SPEC {
    type DataType = u32;
}

pub type Gtcr = crate::RegValueT<Gtcr_SPEC>;

impl Gtcr {
    #[inline(always)]
    pub fn cst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtcr::Cst,
        gtcr::Cst,
        Gtcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtcr::Cst,
            gtcr::Cst,
            Gtcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn icds(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gtcr::Icds,
        gtcr::Icds,
        Gtcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gtcr::Icds,
            gtcr::Icds,
            Gtcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn scgtioc(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        gtcr::Scgtioc,
        gtcr::Scgtioc,
        Gtcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            gtcr::Scgtioc,
            gtcr::Scgtioc,
            Gtcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sscgrp(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        gtcr::Sscgrp,
        gtcr::Sscgrp,
        Gtcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            gtcr::Sscgrp,
            gtcr::Sscgrp,
            Gtcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cpscd(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        gtcr::Cpscd,
        gtcr::Cpscd,
        Gtcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            gtcr::Cpscd,
            gtcr::Cpscd,
            Gtcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sscen(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        gtcr::Sscen,
        gtcr::Sscen,
        Gtcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            gtcr::Sscen,
            gtcr::Sscen,
            Gtcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn md(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, gtcr::Md, gtcr::Md, Gtcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            16,
            0xf,
            1,
            0,
            gtcr::Md,
            gtcr::Md,
            Gtcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tpcs(
        self,
    ) -> crate::common::RegisterField<
        23,
        0xf,
        1,
        0,
        gtcr::Tpcs,
        gtcr::Tpcs,
        Gtcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0xf,
            1,
            0,
            gtcr::Tpcs,
            gtcr::Tpcs,
            Gtcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ckeg(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x3,
        1,
        0,
        gtcr::Ckeg,
        gtcr::Ckeg,
        Gtcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x3,
            1,
            0,
            gtcr::Ckeg,
            gtcr::Ckeg,
            Gtcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtcr {
    #[inline(always)]
    fn default() -> Gtcr {
        <crate::RegValueT<Gtcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cst_SPEC;
    pub type Cst = crate::EnumBitfieldStruct<u8, Cst_SPEC>;
    impl Cst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icds_SPEC;
    pub type Icds = crate::EnumBitfieldStruct<u8, Icds_SPEC>;
    impl Icds {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scgtioc_SPEC;
    pub type Scgtioc = crate::EnumBitfieldStruct<u8, Scgtioc_SPEC>;
    impl Scgtioc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sscgrp_SPEC;
    pub type Sscgrp = crate::EnumBitfieldStruct<u8, Sscgrp_SPEC>;
    impl Sscgrp {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpscd_SPEC;
    pub type Cpscd = crate::EnumBitfieldStruct<u8, Cpscd_SPEC>;
    impl Cpscd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sscen_SPEC;
    pub type Sscen = crate::EnumBitfieldStruct<u8, Sscen_SPEC>;
    impl Sscen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Md_SPEC;
    pub type Md = crate::EnumBitfieldStruct<u8, Md_SPEC>;
    impl Md {
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

        pub const _1011: Self = Self::new(11);

        pub const _1100: Self = Self::new(12);

        pub const _1101: Self = Self::new(13);

        pub const _1110: Self = Self::new(14);

        pub const _1111: Self = Self::new(15);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tpcs_SPEC;
    pub type Tpcs = crate::EnumBitfieldStruct<u8, Tpcs_SPEC>;
    impl Tpcs {
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

        pub const _1011: Self = Self::new(11);

        pub const _1100: Self = Self::new(12);

        pub const _1101: Self = Self::new(13);

        pub const _1110: Self = Self::new(14);

        pub const _1111: Self = Self::new(15);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ckeg_SPEC;
    pub type Ckeg = crate::EnumBitfieldStruct<u8, Ckeg_SPEC>;
    impl Ckeg {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtuddtyc_SPEC;
impl crate::sealed::RegSpec for Gtuddtyc_SPEC {
    type DataType = u32;
}

pub type Gtuddtyc = crate::RegValueT<Gtuddtyc_SPEC>;

impl Gtuddtyc {
    #[inline(always)]
    pub fn ud(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtuddtyc::Ud,
        gtuddtyc::Ud,
        Gtuddtyc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtuddtyc::Ud,
            gtuddtyc::Ud,
            Gtuddtyc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn udf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gtuddtyc::Udf,
        gtuddtyc::Udf,
        Gtuddtyc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gtuddtyc::Udf,
            gtuddtyc::Udf,
            Gtuddtyc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn oadty(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        gtuddtyc::Oadty,
        gtuddtyc::Oadty,
        Gtuddtyc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            gtuddtyc::Oadty,
            gtuddtyc::Oadty,
            Gtuddtyc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn oadtyf(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        gtuddtyc::Oadtyf,
        gtuddtyc::Oadtyf,
        Gtuddtyc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            gtuddtyc::Oadtyf,
            gtuddtyc::Oadtyf,
            Gtuddtyc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn oadtyr(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        gtuddtyc::Oadtyr,
        gtuddtyc::Oadtyr,
        Gtuddtyc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            gtuddtyc::Oadtyr,
            gtuddtyc::Oadtyr,
            Gtuddtyc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn obdty(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x3,
        1,
        0,
        gtuddtyc::Obdty,
        gtuddtyc::Obdty,
        Gtuddtyc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x3,
            1,
            0,
            gtuddtyc::Obdty,
            gtuddtyc::Obdty,
            Gtuddtyc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn obdtyf(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        gtuddtyc::Obdtyf,
        gtuddtyc::Obdtyf,
        Gtuddtyc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            gtuddtyc::Obdtyf,
            gtuddtyc::Obdtyf,
            Gtuddtyc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn obdtyr(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        gtuddtyc::Obdtyr,
        gtuddtyc::Obdtyr,
        Gtuddtyc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            gtuddtyc::Obdtyr,
            gtuddtyc::Obdtyr,
            Gtuddtyc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtuddtyc {
    #[inline(always)]
    fn default() -> Gtuddtyc {
        <crate::RegValueT<Gtuddtyc_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod gtuddtyc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ud_SPEC;
    pub type Ud = crate::EnumBitfieldStruct<u8, Ud_SPEC>;
    impl Ud {
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
    pub struct Oadty_SPEC;
    pub type Oadty = crate::EnumBitfieldStruct<u8, Oadty_SPEC>;
    impl Oadty {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oadtyf_SPEC;
    pub type Oadtyf = crate::EnumBitfieldStruct<u8, Oadtyf_SPEC>;
    impl Oadtyf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oadtyr_SPEC;
    pub type Oadtyr = crate::EnumBitfieldStruct<u8, Oadtyr_SPEC>;
    impl Oadtyr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Obdty_SPEC;
    pub type Obdty = crate::EnumBitfieldStruct<u8, Obdty_SPEC>;
    impl Obdty {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Obdtyf_SPEC;
    pub type Obdtyf = crate::EnumBitfieldStruct<u8, Obdtyf_SPEC>;
    impl Obdtyf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Obdtyr_SPEC;
    pub type Obdtyr = crate::EnumBitfieldStruct<u8, Obdtyr_SPEC>;
    impl Obdtyr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtior_SPEC;
impl crate::sealed::RegSpec for Gtior_SPEC {
    type DataType = u32;
}

pub type Gtior = crate::RegValueT<Gtior_SPEC>;

impl Gtior {
    #[inline(always)]
    pub fn gtioa(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Gtior_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Gtior_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cpscir(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gtior::Cpscir,
        gtior::Cpscir,
        Gtior_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gtior::Cpscir,
            gtior::Cpscir,
            Gtior_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn oadflt(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        gtior::Oadflt,
        gtior::Oadflt,
        Gtior_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            gtior::Oadflt,
            gtior::Oadflt,
            Gtior_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn oahld(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        gtior::Oahld,
        gtior::Oahld,
        Gtior_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            gtior::Oahld,
            gtior::Oahld,
            Gtior_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn oae(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gtior::Oae,
        gtior::Oae,
        Gtior_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gtior::Oae,
            gtior::Oae,
            Gtior_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn oadf(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x3,
        1,
        0,
        gtior::Oadf,
        gtior::Oadf,
        Gtior_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x3,
            1,
            0,
            gtior::Oadf,
            gtior::Oadf,
            Gtior_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn oaeocd(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        gtior::Oaeocd,
        gtior::Oaeocd,
        Gtior_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            gtior::Oaeocd,
            gtior::Oaeocd,
            Gtior_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn psye(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        gtior::Psye,
        gtior::Psye,
        Gtior_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            gtior::Psye,
            gtior::Psye,
            Gtior_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn nfaen(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        gtior::Nfaen,
        gtior::Nfaen,
        Gtior_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            gtior::Nfaen,
            gtior::Nfaen,
            Gtior_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn nfcsa(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        gtior::Nfcsa,
        gtior::Nfcsa,
        Gtior_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            gtior::Nfcsa,
            gtior::Nfcsa,
            Gtior_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gtiob(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Gtior_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Gtior_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn obdflt(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        gtior::Obdflt,
        gtior::Obdflt,
        Gtior_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            gtior::Obdflt,
            gtior::Obdflt,
            Gtior_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn obhld(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        gtior::Obhld,
        gtior::Obhld,
        Gtior_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            gtior::Obhld,
            gtior::Obhld,
            Gtior_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn obe(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        gtior::Obe,
        gtior::Obe,
        Gtior_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            gtior::Obe,
            gtior::Obe,
            Gtior_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn obdf(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x3,
        1,
        0,
        gtior::Obdf,
        gtior::Obdf,
        Gtior_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x3,
            1,
            0,
            gtior::Obdf,
            gtior::Obdf,
            Gtior_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn obeocd(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        gtior::Obeocd,
        gtior::Obeocd,
        Gtior_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            gtior::Obeocd,
            gtior::Obeocd,
            Gtior_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn nfben(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        gtior::Nfben,
        gtior::Nfben,
        Gtior_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            gtior::Nfben,
            gtior::Nfben,
            Gtior_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn nfcsb(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x3,
        1,
        0,
        gtior::Nfcsb,
        gtior::Nfcsb,
        Gtior_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x3,
            1,
            0,
            gtior::Nfcsb,
            gtior::Nfcsb,
            Gtior_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtior {
    #[inline(always)]
    fn default() -> Gtior {
        <crate::RegValueT<Gtior_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtior {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpscir_SPEC;
    pub type Cpscir = crate::EnumBitfieldStruct<u8, Cpscir_SPEC>;
    impl Cpscir {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oadflt_SPEC;
    pub type Oadflt = crate::EnumBitfieldStruct<u8, Oadflt_SPEC>;
    impl Oadflt {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oahld_SPEC;
    pub type Oahld = crate::EnumBitfieldStruct<u8, Oahld_SPEC>;
    impl Oahld {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oae_SPEC;
    pub type Oae = crate::EnumBitfieldStruct<u8, Oae_SPEC>;
    impl Oae {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oadf_SPEC;
    pub type Oadf = crate::EnumBitfieldStruct<u8, Oadf_SPEC>;
    impl Oadf {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oaeocd_SPEC;
    pub type Oaeocd = crate::EnumBitfieldStruct<u8, Oaeocd_SPEC>;
    impl Oaeocd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psye_SPEC;
    pub type Psye = crate::EnumBitfieldStruct<u8, Psye_SPEC>;
    impl Psye {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfaen_SPEC;
    pub type Nfaen = crate::EnumBitfieldStruct<u8, Nfaen_SPEC>;
    impl Nfaen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfcsa_SPEC;
    pub type Nfcsa = crate::EnumBitfieldStruct<u8, Nfcsa_SPEC>;
    impl Nfcsa {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Obdflt_SPEC;
    pub type Obdflt = crate::EnumBitfieldStruct<u8, Obdflt_SPEC>;
    impl Obdflt {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Obhld_SPEC;
    pub type Obhld = crate::EnumBitfieldStruct<u8, Obhld_SPEC>;
    impl Obhld {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Obe_SPEC;
    pub type Obe = crate::EnumBitfieldStruct<u8, Obe_SPEC>;
    impl Obe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Obdf_SPEC;
    pub type Obdf = crate::EnumBitfieldStruct<u8, Obdf_SPEC>;
    impl Obdf {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Obeocd_SPEC;
    pub type Obeocd = crate::EnumBitfieldStruct<u8, Obeocd_SPEC>;
    impl Obeocd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfben_SPEC;
    pub type Nfben = crate::EnumBitfieldStruct<u8, Nfben_SPEC>;
    impl Nfben {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfcsb_SPEC;
    pub type Nfcsb = crate::EnumBitfieldStruct<u8, Nfcsb_SPEC>;
    impl Nfcsb {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtintad_SPEC;
impl crate::sealed::RegSpec for Gtintad_SPEC {
    type DataType = u32;
}

pub type Gtintad = crate::RegValueT<Gtintad_SPEC>;

impl Gtintad {
    #[inline(always)]
    pub fn scfa(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gtintad::Scfa,
        gtintad::Scfa,
        Gtintad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gtintad::Scfa,
            gtintad::Scfa,
            Gtintad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn scfb(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        gtintad::Scfb,
        gtintad::Scfb,
        Gtintad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            gtintad::Scfb,
            gtintad::Scfb,
            Gtintad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn scfc(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        gtintad::Scfc,
        gtintad::Scfc,
        Gtintad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            gtintad::Scfc,
            gtintad::Scfc,
            Gtintad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn scfd(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        gtintad::Scfd,
        gtintad::Scfd,
        Gtintad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            gtintad::Scfd,
            gtintad::Scfd,
            Gtintad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn scfe(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        gtintad::Scfe,
        gtintad::Scfe,
        Gtintad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            gtintad::Scfe,
            gtintad::Scfe,
            Gtintad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn scff(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        gtintad::Scff,
        gtintad::Scff,
        Gtintad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            gtintad::Scff,
            gtintad::Scff,
            Gtintad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn scfpo(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        gtintad::Scfpo,
        gtintad::Scfpo,
        Gtintad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            gtintad::Scfpo,
            gtintad::Scfpo,
            Gtintad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn scfpu(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        gtintad::Scfpu,
        gtintad::Scfpu,
        Gtintad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            gtintad::Scfpu,
            gtintad::Scfpu,
            Gtintad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adtrauen(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        gtintad::Adtrauen,
        gtintad::Adtrauen,
        Gtintad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gtintad::Adtrauen,
            gtintad::Adtrauen,
            Gtintad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adtraden(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        gtintad::Adtraden,
        gtintad::Adtraden,
        Gtintad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            gtintad::Adtraden,
            gtintad::Adtraden,
            Gtintad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adtrbuen(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        gtintad::Adtrbuen,
        gtintad::Adtrbuen,
        Gtintad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            gtintad::Adtrbuen,
            gtintad::Adtrbuen,
            Gtintad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adtrbden(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        gtintad::Adtrbden,
        gtintad::Adtrbden,
        Gtintad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            gtintad::Adtrbden,
            gtintad::Adtrbden,
            Gtintad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn grp(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x3,
        1,
        0,
        gtintad::Grp,
        gtintad::Grp,
        Gtintad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x3,
            1,
            0,
            gtintad::Grp,
            gtintad::Grp,
            Gtintad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn grpdte(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        gtintad::Grpdte,
        gtintad::Grpdte,
        Gtintad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            gtintad::Grpdte,
            gtintad::Grpdte,
            Gtintad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn grpabh(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        gtintad::Grpabh,
        gtintad::Grpabh,
        Gtintad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            gtintad::Grpabh,
            gtintad::Grpabh,
            Gtintad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn grpabl(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        gtintad::Grpabl,
        gtintad::Grpabl,
        Gtintad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            gtintad::Grpabl,
            gtintad::Grpabl,
            Gtintad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtintad {
    #[inline(always)]
    fn default() -> Gtintad {
        <crate::RegValueT<Gtintad_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtintad {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scfa_SPEC;
    pub type Scfa = crate::EnumBitfieldStruct<u8, Scfa_SPEC>;
    impl Scfa {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scfb_SPEC;
    pub type Scfb = crate::EnumBitfieldStruct<u8, Scfb_SPEC>;
    impl Scfb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scfc_SPEC;
    pub type Scfc = crate::EnumBitfieldStruct<u8, Scfc_SPEC>;
    impl Scfc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scfd_SPEC;
    pub type Scfd = crate::EnumBitfieldStruct<u8, Scfd_SPEC>;
    impl Scfd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scfe_SPEC;
    pub type Scfe = crate::EnumBitfieldStruct<u8, Scfe_SPEC>;
    impl Scfe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scff_SPEC;
    pub type Scff = crate::EnumBitfieldStruct<u8, Scff_SPEC>;
    impl Scff {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scfpo_SPEC;
    pub type Scfpo = crate::EnumBitfieldStruct<u8, Scfpo_SPEC>;
    impl Scfpo {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scfpu_SPEC;
    pub type Scfpu = crate::EnumBitfieldStruct<u8, Scfpu_SPEC>;
    impl Scfpu {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adtrauen_SPEC;
    pub type Adtrauen = crate::EnumBitfieldStruct<u8, Adtrauen_SPEC>;
    impl Adtrauen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adtraden_SPEC;
    pub type Adtraden = crate::EnumBitfieldStruct<u8, Adtraden_SPEC>;
    impl Adtraden {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adtrbuen_SPEC;
    pub type Adtrbuen = crate::EnumBitfieldStruct<u8, Adtrbuen_SPEC>;
    impl Adtrbuen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adtrbden_SPEC;
    pub type Adtrbden = crate::EnumBitfieldStruct<u8, Adtrbden_SPEC>;
    impl Adtrbden {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Grp_SPEC;
    pub type Grp = crate::EnumBitfieldStruct<u8, Grp_SPEC>;
    impl Grp {
        pub const _00: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Grpdte_SPEC;
    pub type Grpdte = crate::EnumBitfieldStruct<u8, Grpdte_SPEC>;
    impl Grpdte {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Grpabh_SPEC;
    pub type Grpabh = crate::EnumBitfieldStruct<u8, Grpabh_SPEC>;
    impl Grpabh {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Grpabl_SPEC;
    pub type Grpabl = crate::EnumBitfieldStruct<u8, Grpabl_SPEC>;
    impl Grpabl {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtst_SPEC;
impl crate::sealed::RegSpec for Gtst_SPEC {
    type DataType = u32;
}

pub type Gtst = crate::RegValueT<Gtst_SPEC>;

impl Gtst {
    #[inline(always)]
    pub fn tcfa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtst::Tcfa,
        gtst::Tcfa,
        Gtst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtst::Tcfa,
            gtst::Tcfa,
            Gtst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tcfb(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gtst::Tcfb,
        gtst::Tcfb,
        Gtst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gtst::Tcfb,
            gtst::Tcfb,
            Gtst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tcfc(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gtst::Tcfc,
        gtst::Tcfc,
        Gtst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gtst::Tcfc,
            gtst::Tcfc,
            Gtst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tcfd(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gtst::Tcfd,
        gtst::Tcfd,
        Gtst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gtst::Tcfd,
            gtst::Tcfd,
            Gtst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tcfe(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gtst::Tcfe,
        gtst::Tcfe,
        Gtst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gtst::Tcfe,
            gtst::Tcfe,
            Gtst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tcff(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gtst::Tcff,
        gtst::Tcff,
        Gtst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gtst::Tcff,
            gtst::Tcff,
            Gtst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tcfpo(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        gtst::Tcfpo,
        gtst::Tcfpo,
        Gtst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            gtst::Tcfpo,
            gtst::Tcfpo,
            Gtst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tcfpu(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        gtst::Tcfpu,
        gtst::Tcfpu,
        Gtst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            gtst::Tcfpu,
            gtst::Tcfpu,
            Gtst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn itcnt(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, Gtst_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,Gtst_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tucf(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        gtst::Tucf,
        gtst::Tucf,
        Gtst_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            gtst::Tucf,
            gtst::Tucf,
            Gtst_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adtrauf(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        gtst::Adtrauf,
        gtst::Adtrauf,
        Gtst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gtst::Adtrauf,
            gtst::Adtrauf,
            Gtst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adtradf(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        gtst::Adtradf,
        gtst::Adtradf,
        Gtst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            gtst::Adtradf,
            gtst::Adtradf,
            Gtst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adtrbuf(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        gtst::Adtrbuf,
        gtst::Adtrbuf,
        Gtst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            gtst::Adtrbuf,
            gtst::Adtrbuf,
            Gtst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adtrbdf(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        gtst::Adtrbdf,
        gtst::Adtrbdf,
        Gtst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            gtst::Adtrbdf,
            gtst::Adtrbdf,
            Gtst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn odf(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        gtst::Odf,
        gtst::Odf,
        Gtst_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            gtst::Odf,
            gtst::Odf,
            Gtst_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dtef(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        gtst::Dtef,
        gtst::Dtef,
        Gtst_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            gtst::Dtef,
            gtst::Dtef,
            Gtst_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn oabhf(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        gtst::Oabhf,
        gtst::Oabhf,
        Gtst_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            gtst::Oabhf,
            gtst::Oabhf,
            Gtst_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn oablf(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        gtst::Oablf,
        gtst::Oablf,
        Gtst_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            gtst::Oablf,
            gtst::Oablf,
            Gtst_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pcf(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        gtst::Pcf,
        gtst::Pcf,
        Gtst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            gtst::Pcf,
            gtst::Pcf,
            Gtst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtst {
    #[inline(always)]
    fn default() -> Gtst {
        <crate::RegValueT<Gtst_SPEC> as RegisterValue<_>>::new(32768)
    }
}
pub mod gtst {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcfa_SPEC;
    pub type Tcfa = crate::EnumBitfieldStruct<u8, Tcfa_SPEC>;
    impl Tcfa {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcfb_SPEC;
    pub type Tcfb = crate::EnumBitfieldStruct<u8, Tcfb_SPEC>;
    impl Tcfb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcfc_SPEC;
    pub type Tcfc = crate::EnumBitfieldStruct<u8, Tcfc_SPEC>;
    impl Tcfc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcfd_SPEC;
    pub type Tcfd = crate::EnumBitfieldStruct<u8, Tcfd_SPEC>;
    impl Tcfd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcfe_SPEC;
    pub type Tcfe = crate::EnumBitfieldStruct<u8, Tcfe_SPEC>;
    impl Tcfe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcff_SPEC;
    pub type Tcff = crate::EnumBitfieldStruct<u8, Tcff_SPEC>;
    impl Tcff {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcfpo_SPEC;
    pub type Tcfpo = crate::EnumBitfieldStruct<u8, Tcfpo_SPEC>;
    impl Tcfpo {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcfpu_SPEC;
    pub type Tcfpu = crate::EnumBitfieldStruct<u8, Tcfpu_SPEC>;
    impl Tcfpu {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tucf_SPEC;
    pub type Tucf = crate::EnumBitfieldStruct<u8, Tucf_SPEC>;
    impl Tucf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adtrauf_SPEC;
    pub type Adtrauf = crate::EnumBitfieldStruct<u8, Adtrauf_SPEC>;
    impl Adtrauf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adtradf_SPEC;
    pub type Adtradf = crate::EnumBitfieldStruct<u8, Adtradf_SPEC>;
    impl Adtradf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adtrbuf_SPEC;
    pub type Adtrbuf = crate::EnumBitfieldStruct<u8, Adtrbuf_SPEC>;
    impl Adtrbuf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adtrbdf_SPEC;
    pub type Adtrbdf = crate::EnumBitfieldStruct<u8, Adtrbdf_SPEC>;
    impl Adtrbdf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Odf_SPEC;
    pub type Odf = crate::EnumBitfieldStruct<u8, Odf_SPEC>;
    impl Odf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtef_SPEC;
    pub type Dtef = crate::EnumBitfieldStruct<u8, Dtef_SPEC>;
    impl Dtef {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oabhf_SPEC;
    pub type Oabhf = crate::EnumBitfieldStruct<u8, Oabhf_SPEC>;
    impl Oabhf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oablf_SPEC;
    pub type Oablf = crate::EnumBitfieldStruct<u8, Oablf_SPEC>;
    impl Oablf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcf_SPEC;
    pub type Pcf = crate::EnumBitfieldStruct<u8, Pcf_SPEC>;
    impl Pcf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtber_SPEC;
impl crate::sealed::RegSpec for Gtber_SPEC {
    type DataType = u32;
}

pub type Gtber = crate::RegValueT<Gtber_SPEC>;

impl Gtber {
    #[inline(always)]
    pub fn bd0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtber::Bd0,
        gtber::Bd0,
        Gtber_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtber::Bd0,
            gtber::Bd0,
            Gtber_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bd1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gtber::Bd1,
        gtber::Bd1,
        Gtber_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gtber::Bd1,
            gtber::Bd1,
            Gtber_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bd2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gtber::Bd2,
        gtber::Bd2,
        Gtber_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gtber::Bd2,
            gtber::Bd2,
            Gtber_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bd3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gtber::Bd3,
        gtber::Bd3,
        Gtber_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gtber::Bd3,
            gtber::Bd3,
            Gtber_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dbrteca(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gtber::Dbrteca,
        gtber::Dbrteca,
        Gtber_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gtber::Dbrteca,
            gtber::Dbrteca,
            Gtber_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dbrtecb(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        gtber::Dbrtecb,
        gtber::Dbrtecb,
        Gtber_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            gtber::Dbrtecb,
            gtber::Dbrtecb,
            Gtber_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ccra(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        gtber::Ccra,
        gtber::Ccra,
        Gtber_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            gtber::Ccra,
            gtber::Ccra,
            Gtber_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ccrb(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x3,
        1,
        0,
        gtber::Ccrb,
        gtber::Ccrb,
        Gtber_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x3,
            1,
            0,
            gtber::Ccrb,
            gtber::Ccrb,
            Gtber_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pr(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x3,
        1,
        0,
        gtber::Pr,
        gtber::Pr,
        Gtber_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x3,
            1,
            0,
            gtber::Pr,
            gtber::Pr,
            Gtber_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ccrswt(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Gtber_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, Gtber_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn adtta(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x3,
        1,
        0,
        gtber::Adtta,
        gtber::Adtta,
        Gtber_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x3,
            1,
            0,
            gtber::Adtta,
            gtber::Adtta,
            Gtber_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adtda(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        gtber::Adtda,
        gtber::Adtda,
        Gtber_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            gtber::Adtda,
            gtber::Adtda,
            Gtber_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adttb(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x3,
        1,
        0,
        gtber::Adttb,
        gtber::Adttb,
        Gtber_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x3,
            1,
            0,
            gtber::Adttb,
            gtber::Adttb,
            Gtber_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adtdb(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        gtber::Adtdb,
        gtber::Adtdb,
        Gtber_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            gtber::Adtdb,
            gtber::Adtdb,
            Gtber_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtber {
    #[inline(always)]
    fn default() -> Gtber {
        <crate::RegValueT<Gtber_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtber {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bd0_SPEC;
    pub type Bd0 = crate::EnumBitfieldStruct<u8, Bd0_SPEC>;
    impl Bd0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bd1_SPEC;
    pub type Bd1 = crate::EnumBitfieldStruct<u8, Bd1_SPEC>;
    impl Bd1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bd2_SPEC;
    pub type Bd2 = crate::EnumBitfieldStruct<u8, Bd2_SPEC>;
    impl Bd2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bd3_SPEC;
    pub type Bd3 = crate::EnumBitfieldStruct<u8, Bd3_SPEC>;
    impl Bd3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dbrteca_SPEC;
    pub type Dbrteca = crate::EnumBitfieldStruct<u8, Dbrteca_SPEC>;
    impl Dbrteca {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dbrtecb_SPEC;
    pub type Dbrtecb = crate::EnumBitfieldStruct<u8, Dbrtecb_SPEC>;
    impl Dbrtecb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccra_SPEC;
    pub type Ccra = crate::EnumBitfieldStruct<u8, Ccra_SPEC>;
    impl Ccra {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccrb_SPEC;
    pub type Ccrb = crate::EnumBitfieldStruct<u8, Ccrb_SPEC>;
    impl Ccrb {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pr_SPEC;
    pub type Pr = crate::EnumBitfieldStruct<u8, Pr_SPEC>;
    impl Pr {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adtta_SPEC;
    pub type Adtta = crate::EnumBitfieldStruct<u8, Adtta_SPEC>;
    impl Adtta {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adtda_SPEC;
    pub type Adtda = crate::EnumBitfieldStruct<u8, Adtda_SPEC>;
    impl Adtda {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adttb_SPEC;
    pub type Adttb = crate::EnumBitfieldStruct<u8, Adttb_SPEC>;
    impl Adttb {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adtdb_SPEC;
    pub type Adtdb = crate::EnumBitfieldStruct<u8, Adtdb_SPEC>;
    impl Adtdb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtitc_SPEC;
impl crate::sealed::RegSpec for Gtitc_SPEC {
    type DataType = u32;
}

pub type Gtitc = crate::RegValueT<Gtitc_SPEC>;

impl Gtitc {
    #[inline(always)]
    pub fn itla(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtitc::Itla,
        gtitc::Itla,
        Gtitc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtitc::Itla,
            gtitc::Itla,
            Gtitc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn itlb(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gtitc::Itlb,
        gtitc::Itlb,
        Gtitc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gtitc::Itlb,
            gtitc::Itlb,
            Gtitc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn itlc(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gtitc::Itlc,
        gtitc::Itlc,
        Gtitc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gtitc::Itlc,
            gtitc::Itlc,
            Gtitc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn itld(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gtitc::Itld,
        gtitc::Itld,
        Gtitc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gtitc::Itld,
            gtitc::Itld,
            Gtitc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn itle(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gtitc::Itle,
        gtitc::Itle,
        Gtitc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gtitc::Itle,
            gtitc::Itle,
            Gtitc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn itlf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gtitc::Itlf,
        gtitc::Itlf,
        Gtitc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gtitc::Itlf,
            gtitc::Itlf,
            Gtitc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ivtc(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        gtitc::Ivtc,
        gtitc::Ivtc,
        Gtitc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            gtitc::Ivtc,
            gtitc::Ivtc,
            Gtitc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ivtt(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        gtitc::Ivtt,
        gtitc::Ivtt,
        Gtitc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            gtitc::Ivtt,
            gtitc::Ivtt,
            Gtitc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adtal(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        gtitc::Adtal,
        gtitc::Adtal,
        Gtitc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            gtitc::Adtal,
            gtitc::Adtal,
            Gtitc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adtbl(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        gtitc::Adtbl,
        gtitc::Adtbl,
        Gtitc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            gtitc::Adtbl,
            gtitc::Adtbl,
            Gtitc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtitc {
    #[inline(always)]
    fn default() -> Gtitc {
        <crate::RegValueT<Gtitc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtitc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Itla_SPEC;
    pub type Itla = crate::EnumBitfieldStruct<u8, Itla_SPEC>;
    impl Itla {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Itlb_SPEC;
    pub type Itlb = crate::EnumBitfieldStruct<u8, Itlb_SPEC>;
    impl Itlb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Itlc_SPEC;
    pub type Itlc = crate::EnumBitfieldStruct<u8, Itlc_SPEC>;
    impl Itlc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Itld_SPEC;
    pub type Itld = crate::EnumBitfieldStruct<u8, Itld_SPEC>;
    impl Itld {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Itle_SPEC;
    pub type Itle = crate::EnumBitfieldStruct<u8, Itle_SPEC>;
    impl Itle {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Itlf_SPEC;
    pub type Itlf = crate::EnumBitfieldStruct<u8, Itlf_SPEC>;
    impl Itlf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ivtc_SPEC;
    pub type Ivtc = crate::EnumBitfieldStruct<u8, Ivtc_SPEC>;
    impl Ivtc {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ivtt_SPEC;
    pub type Ivtt = crate::EnumBitfieldStruct<u8, Ivtt_SPEC>;
    impl Ivtt {
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
    pub struct Adtal_SPEC;
    pub type Adtal = crate::EnumBitfieldStruct<u8, Adtal_SPEC>;
    impl Adtal {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adtbl_SPEC;
    pub type Adtbl = crate::EnumBitfieldStruct<u8, Adtbl_SPEC>;
    impl Adtbl {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtcnt_SPEC;
impl crate::sealed::RegSpec for Gtcnt_SPEC {
    type DataType = u32;
}

pub type Gtcnt = crate::RegValueT<Gtcnt_SPEC>;

impl NoBitfieldReg<Gtcnt_SPEC> for Gtcnt {}
impl ::core::default::Default for Gtcnt {
    #[inline(always)]
    fn default() -> Gtcnt {
        <crate::RegValueT<Gtcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtccra_SPEC;
impl crate::sealed::RegSpec for Gtccra_SPEC {
    type DataType = u32;
}

pub type Gtccra = crate::RegValueT<Gtccra_SPEC>;

impl NoBitfieldReg<Gtccra_SPEC> for Gtccra {}
impl ::core::default::Default for Gtccra {
    #[inline(always)]
    fn default() -> Gtccra {
        <crate::RegValueT<Gtccra_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtccrb_SPEC;
impl crate::sealed::RegSpec for Gtccrb_SPEC {
    type DataType = u32;
}

pub type Gtccrb = crate::RegValueT<Gtccrb_SPEC>;

impl NoBitfieldReg<Gtccrb_SPEC> for Gtccrb {}
impl ::core::default::Default for Gtccrb {
    #[inline(always)]
    fn default() -> Gtccrb {
        <crate::RegValueT<Gtccrb_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtccrc_SPEC;
impl crate::sealed::RegSpec for Gtccrc_SPEC {
    type DataType = u32;
}

pub type Gtccrc = crate::RegValueT<Gtccrc_SPEC>;

impl NoBitfieldReg<Gtccrc_SPEC> for Gtccrc {}
impl ::core::default::Default for Gtccrc {
    #[inline(always)]
    fn default() -> Gtccrc {
        <crate::RegValueT<Gtccrc_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtccre_SPEC;
impl crate::sealed::RegSpec for Gtccre_SPEC {
    type DataType = u32;
}

pub type Gtccre = crate::RegValueT<Gtccre_SPEC>;

impl NoBitfieldReg<Gtccre_SPEC> for Gtccre {}
impl ::core::default::Default for Gtccre {
    #[inline(always)]
    fn default() -> Gtccre {
        <crate::RegValueT<Gtccre_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtccrd_SPEC;
impl crate::sealed::RegSpec for Gtccrd_SPEC {
    type DataType = u32;
}

pub type Gtccrd = crate::RegValueT<Gtccrd_SPEC>;

impl NoBitfieldReg<Gtccrd_SPEC> for Gtccrd {}
impl ::core::default::Default for Gtccrd {
    #[inline(always)]
    fn default() -> Gtccrd {
        <crate::RegValueT<Gtccrd_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtccrf_SPEC;
impl crate::sealed::RegSpec for Gtccrf_SPEC {
    type DataType = u32;
}

pub type Gtccrf = crate::RegValueT<Gtccrf_SPEC>;

impl NoBitfieldReg<Gtccrf_SPEC> for Gtccrf {}
impl ::core::default::Default for Gtccrf {
    #[inline(always)]
    fn default() -> Gtccrf {
        <crate::RegValueT<Gtccrf_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtpr_SPEC;
impl crate::sealed::RegSpec for Gtpr_SPEC {
    type DataType = u32;
}

pub type Gtpr = crate::RegValueT<Gtpr_SPEC>;

impl NoBitfieldReg<Gtpr_SPEC> for Gtpr {}
impl ::core::default::Default for Gtpr {
    #[inline(always)]
    fn default() -> Gtpr {
        <crate::RegValueT<Gtpr_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtpbr_SPEC;
impl crate::sealed::RegSpec for Gtpbr_SPEC {
    type DataType = u32;
}

pub type Gtpbr = crate::RegValueT<Gtpbr_SPEC>;

impl NoBitfieldReg<Gtpbr_SPEC> for Gtpbr {}
impl ::core::default::Default for Gtpbr {
    #[inline(always)]
    fn default() -> Gtpbr {
        <crate::RegValueT<Gtpbr_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtpdbr_SPEC;
impl crate::sealed::RegSpec for Gtpdbr_SPEC {
    type DataType = u32;
}

pub type Gtpdbr = crate::RegValueT<Gtpdbr_SPEC>;

impl NoBitfieldReg<Gtpdbr_SPEC> for Gtpdbr {}
impl ::core::default::Default for Gtpdbr {
    #[inline(always)]
    fn default() -> Gtpdbr {
        <crate::RegValueT<Gtpdbr_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtadtra_SPEC;
impl crate::sealed::RegSpec for Gtadtra_SPEC {
    type DataType = u32;
}

pub type Gtadtra = crate::RegValueT<Gtadtra_SPEC>;

impl NoBitfieldReg<Gtadtra_SPEC> for Gtadtra {}
impl ::core::default::Default for Gtadtra {
    #[inline(always)]
    fn default() -> Gtadtra {
        <crate::RegValueT<Gtadtra_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtadtbra_SPEC;
impl crate::sealed::RegSpec for Gtadtbra_SPEC {
    type DataType = u32;
}

pub type Gtadtbra = crate::RegValueT<Gtadtbra_SPEC>;

impl NoBitfieldReg<Gtadtbra_SPEC> for Gtadtbra {}
impl ::core::default::Default for Gtadtbra {
    #[inline(always)]
    fn default() -> Gtadtbra {
        <crate::RegValueT<Gtadtbra_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtadtdbra_SPEC;
impl crate::sealed::RegSpec for Gtadtdbra_SPEC {
    type DataType = u32;
}

pub type Gtadtdbra = crate::RegValueT<Gtadtdbra_SPEC>;

impl NoBitfieldReg<Gtadtdbra_SPEC> for Gtadtdbra {}
impl ::core::default::Default for Gtadtdbra {
    #[inline(always)]
    fn default() -> Gtadtdbra {
        <crate::RegValueT<Gtadtdbra_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtadtrb_SPEC;
impl crate::sealed::RegSpec for Gtadtrb_SPEC {
    type DataType = u32;
}

pub type Gtadtrb = crate::RegValueT<Gtadtrb_SPEC>;

impl NoBitfieldReg<Gtadtrb_SPEC> for Gtadtrb {}
impl ::core::default::Default for Gtadtrb {
    #[inline(always)]
    fn default() -> Gtadtrb {
        <crate::RegValueT<Gtadtrb_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtadtbrb_SPEC;
impl crate::sealed::RegSpec for Gtadtbrb_SPEC {
    type DataType = u32;
}

pub type Gtadtbrb = crate::RegValueT<Gtadtbrb_SPEC>;

impl NoBitfieldReg<Gtadtbrb_SPEC> for Gtadtbrb {}
impl ::core::default::Default for Gtadtbrb {
    #[inline(always)]
    fn default() -> Gtadtbrb {
        <crate::RegValueT<Gtadtbrb_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtadtdbrb_SPEC;
impl crate::sealed::RegSpec for Gtadtdbrb_SPEC {
    type DataType = u32;
}

pub type Gtadtdbrb = crate::RegValueT<Gtadtdbrb_SPEC>;

impl NoBitfieldReg<Gtadtdbrb_SPEC> for Gtadtdbrb {}
impl ::core::default::Default for Gtadtdbrb {
    #[inline(always)]
    fn default() -> Gtadtdbrb {
        <crate::RegValueT<Gtadtdbrb_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtdtcr_SPEC;
impl crate::sealed::RegSpec for Gtdtcr_SPEC {
    type DataType = u32;
}

pub type Gtdtcr = crate::RegValueT<Gtdtcr_SPEC>;

impl Gtdtcr {
    #[inline(always)]
    pub fn tde(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtdtcr::Tde,
        gtdtcr::Tde,
        Gtdtcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtdtcr::Tde,
            gtdtcr::Tde,
            Gtdtcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tdbue(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gtdtcr::Tdbue,
        gtdtcr::Tdbue,
        Gtdtcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gtdtcr::Tdbue,
            gtdtcr::Tdbue,
            Gtdtcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tdbde(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gtdtcr::Tdbde,
        gtdtcr::Tdbde,
        Gtdtcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gtdtcr::Tdbde,
            gtdtcr::Tdbde,
            Gtdtcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tdfer(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gtdtcr::Tdfer,
        gtdtcr::Tdfer,
        Gtdtcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gtdtcr::Tdfer,
            gtdtcr::Tdfer,
            Gtdtcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtdtcr {
    #[inline(always)]
    fn default() -> Gtdtcr {
        <crate::RegValueT<Gtdtcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtdtcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tde_SPEC;
    pub type Tde = crate::EnumBitfieldStruct<u8, Tde_SPEC>;
    impl Tde {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdbue_SPEC;
    pub type Tdbue = crate::EnumBitfieldStruct<u8, Tdbue_SPEC>;
    impl Tdbue {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdbde_SPEC;
    pub type Tdbde = crate::EnumBitfieldStruct<u8, Tdbde_SPEC>;
    impl Tdbde {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdfer_SPEC;
    pub type Tdfer = crate::EnumBitfieldStruct<u8, Tdfer_SPEC>;
    impl Tdfer {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtdvu_SPEC;
impl crate::sealed::RegSpec for Gtdvu_SPEC {
    type DataType = u32;
}

pub type Gtdvu = crate::RegValueT<Gtdvu_SPEC>;

impl NoBitfieldReg<Gtdvu_SPEC> for Gtdvu {}
impl ::core::default::Default for Gtdvu {
    #[inline(always)]
    fn default() -> Gtdvu {
        <crate::RegValueT<Gtdvu_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtdvd_SPEC;
impl crate::sealed::RegSpec for Gtdvd_SPEC {
    type DataType = u32;
}

pub type Gtdvd = crate::RegValueT<Gtdvd_SPEC>;

impl NoBitfieldReg<Gtdvd_SPEC> for Gtdvd {}
impl ::core::default::Default for Gtdvd {
    #[inline(always)]
    fn default() -> Gtdvd {
        <crate::RegValueT<Gtdvd_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtdbu_SPEC;
impl crate::sealed::RegSpec for Gtdbu_SPEC {
    type DataType = u32;
}

pub type Gtdbu = crate::RegValueT<Gtdbu_SPEC>;

impl NoBitfieldReg<Gtdbu_SPEC> for Gtdbu {}
impl ::core::default::Default for Gtdbu {
    #[inline(always)]
    fn default() -> Gtdbu {
        <crate::RegValueT<Gtdbu_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtdbd_SPEC;
impl crate::sealed::RegSpec for Gtdbd_SPEC {
    type DataType = u32;
}

pub type Gtdbd = crate::RegValueT<Gtdbd_SPEC>;

impl NoBitfieldReg<Gtdbd_SPEC> for Gtdbd {}
impl ::core::default::Default for Gtdbd {
    #[inline(always)]
    fn default() -> Gtdbd {
        <crate::RegValueT<Gtdbd_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtsos_SPEC;
impl crate::sealed::RegSpec for Gtsos_SPEC {
    type DataType = u32;
}

pub type Gtsos = crate::RegValueT<Gtsos_SPEC>;

impl Gtsos {
    #[inline(always)]
    pub fn sos(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        gtsos::Sos,
        gtsos::Sos,
        Gtsos_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            gtsos::Sos,
            gtsos::Sos,
            Gtsos_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtsos {
    #[inline(always)]
    fn default() -> Gtsos {
        <crate::RegValueT<Gtsos_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtsos {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sos_SPEC;
    pub type Sos = crate::EnumBitfieldStruct<u8, Sos_SPEC>;
    impl Sos {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtsotr_SPEC;
impl crate::sealed::RegSpec for Gtsotr_SPEC {
    type DataType = u32;
}

pub type Gtsotr = crate::RegValueT<Gtsotr_SPEC>;

impl Gtsotr {
    #[inline(always)]
    pub fn sotr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtsotr::Sotr,
        gtsotr::Sotr,
        Gtsotr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtsotr::Sotr,
            gtsotr::Sotr,
            Gtsotr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtsotr {
    #[inline(always)]
    fn default() -> Gtsotr {
        <crate::RegValueT<Gtsotr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtsotr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sotr_SPEC;
    pub type Sotr = crate::EnumBitfieldStruct<u8, Sotr_SPEC>;
    impl Sotr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtadsmr_SPEC;
impl crate::sealed::RegSpec for Gtadsmr_SPEC {
    type DataType = u32;
}

pub type Gtadsmr = crate::RegValueT<Gtadsmr_SPEC>;

impl Gtadsmr {
    #[inline(always)]
    pub fn adsms0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        gtadsmr::Adsms0,
        gtadsmr::Adsms0,
        Gtadsmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            gtadsmr::Adsms0,
            gtadsmr::Adsms0,
            Gtadsmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adsmen0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gtadsmr::Adsmen0,
        gtadsmr::Adsmen0,
        Gtadsmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gtadsmr::Adsmen0,
            gtadsmr::Adsmen0,
            Gtadsmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adsms1(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        gtadsmr::Adsms1,
        gtadsmr::Adsms1,
        Gtadsmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            gtadsmr::Adsms1,
            gtadsmr::Adsms1,
            Gtadsmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adsmen1(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        gtadsmr::Adsmen1,
        gtadsmr::Adsmen1,
        Gtadsmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            gtadsmr::Adsmen1,
            gtadsmr::Adsmen1,
            Gtadsmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtadsmr {
    #[inline(always)]
    fn default() -> Gtadsmr {
        <crate::RegValueT<Gtadsmr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtadsmr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adsms0_SPEC;
    pub type Adsms0 = crate::EnumBitfieldStruct<u8, Adsms0_SPEC>;
    impl Adsms0 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adsmen0_SPEC;
    pub type Adsmen0 = crate::EnumBitfieldStruct<u8, Adsmen0_SPEC>;
    impl Adsmen0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adsms1_SPEC;
    pub type Adsms1 = crate::EnumBitfieldStruct<u8, Adsms1_SPEC>;
    impl Adsms1 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adsmen1_SPEC;
    pub type Adsmen1 = crate::EnumBitfieldStruct<u8, Adsmen1_SPEC>;
    impl Adsmen1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gteitc_SPEC;
impl crate::sealed::RegSpec for Gteitc_SPEC {
    type DataType = u32;
}

pub type Gteitc = crate::RegValueT<Gteitc_SPEC>;

impl Gteitc {
    #[inline(always)]
    pub fn eivtc1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        gteitc::Eivtc1,
        gteitc::Eivtc1,
        Gteitc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            gteitc::Eivtc1,
            gteitc::Eivtc1,
            Gteitc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn eivtt1(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Gteitc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Gteitc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eitcnt1(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Gteitc_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,Gteitc_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eivtc2(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        gteitc::Eivtc2,
        gteitc::Eivtc2,
        Gteitc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            gteitc::Eivtc2,
            gteitc::Eivtc2,
            Gteitc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn eivtt2(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Gteitc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8,u8,Gteitc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eitcnt2iv(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, u8, Gteitc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8,u8,Gteitc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eitcnt2(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, u8, Gteitc_SPEC, crate::common::R> {
        crate::common::RegisterField::<28,0xf,1,0,u8,u8,Gteitc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gteitc {
    #[inline(always)]
    fn default() -> Gteitc {
        <crate::RegValueT<Gteitc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gteitc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eivtc1_SPEC;
    pub type Eivtc1 = crate::EnumBitfieldStruct<u8, Eivtc1_SPEC>;
    impl Eivtc1 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eivtc2_SPEC;
    pub type Eivtc2 = crate::EnumBitfieldStruct<u8, Eivtc2_SPEC>;
    impl Eivtc2 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gteitli1_SPEC;
impl crate::sealed::RegSpec for Gteitli1_SPEC {
    type DataType = u32;
}

pub type Gteitli1 = crate::RegValueT<Gteitli1_SPEC>;

impl Gteitli1 {
    #[inline(always)]
    pub fn eitla(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Gteitli1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Gteitli1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eitlb(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Gteitli1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Gteitli1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eitlc(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, Gteitli1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,Gteitli1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eitld(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, u8, Gteitli1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x7,1,0,u8,u8,Gteitli1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eitle(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, Gteitli1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,Gteitli1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eitlf(
        self,
    ) -> crate::common::RegisterField<20, 0x7, 1, 0, u8, u8, Gteitli1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x7,1,0,u8,u8,Gteitli1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eitlv(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, u8, Gteitli1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7,1,0,u8,u8,Gteitli1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eitlu(
        self,
    ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, u8, Gteitli1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x7,1,0,u8,u8,Gteitli1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gteitli1 {
    #[inline(always)]
    fn default() -> Gteitli1 {
        <crate::RegValueT<Gteitli1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gteitli2_SPEC;
impl crate::sealed::RegSpec for Gteitli2_SPEC {
    type DataType = u32;
}

pub type Gteitli2 = crate::RegValueT<Gteitli2_SPEC>;

impl Gteitli2 {
    #[inline(always)]
    pub fn eadtal(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Gteitli2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Gteitli2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eadtbl(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Gteitli2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Gteitli2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gteitli2 {
    #[inline(always)]
    fn default() -> Gteitli2 {
        <crate::RegValueT<Gteitli2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gteitlb_SPEC;
impl crate::sealed::RegSpec for Gteitlb_SPEC {
    type DataType = u32;
}

pub type Gteitlb = crate::RegValueT<Gteitlb_SPEC>;

impl Gteitlb {
    #[inline(always)]
    pub fn ebtlca(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Gteitlb_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Gteitlb_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ebtlcb(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Gteitlb_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Gteitlb_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ebtlpr(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, Gteitlb_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,Gteitlb_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ebtlada(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, Gteitlb_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,Gteitlb_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ebtladb(
        self,
    ) -> crate::common::RegisterField<20, 0x7, 1, 0, u8, u8, Gteitlb_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x7,1,0,u8,u8,Gteitlb_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ebtldvu(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, u8, Gteitlb_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7,1,0,u8,u8,Gteitlb_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ebtldvd(
        self,
    ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, u8, Gteitlb_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x7,1,0,u8,u8,Gteitlb_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gteitlb {
    #[inline(always)]
    fn default() -> Gteitlb {
        <crate::RegValueT<Gteitlb_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gticlf_SPEC;
impl crate::sealed::RegSpec for Gticlf_SPEC {
    type DataType = u32;
}

pub type Gticlf = crate::RegValueT<Gticlf_SPEC>;

impl Gticlf {
    #[inline(always)]
    pub fn iclfa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        gticlf::Iclfa,
        gticlf::Iclfa,
        Gticlf_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            gticlf::Iclfa,
            gticlf::Iclfa,
            Gticlf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn iclfselc(
        self,
    ) -> crate::common::RegisterField<4, 0x3f, 1, 0, u8, u8, Gticlf_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3f,1,0,u8,u8,Gticlf_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn iclfb(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        gticlf::Iclfb,
        gticlf::Iclfb,
        Gticlf_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            gticlf::Iclfb,
            gticlf::Iclfb,
            Gticlf_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn iclfseld(
        self,
    ) -> crate::common::RegisterField<20, 0x3f, 1, 0, u8, u8, Gticlf_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3f,1,0,u8,u8,Gticlf_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gticlf {
    #[inline(always)]
    fn default() -> Gticlf {
        <crate::RegValueT<Gticlf_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gticlf {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iclfa_SPEC;
    pub type Iclfa = crate::EnumBitfieldStruct<u8, Iclfa_SPEC>;
    impl Iclfa {
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
    pub struct Iclfb_SPEC;
    pub type Iclfb = crate::EnumBitfieldStruct<u8, Iclfb_SPEC>;
    impl Iclfb {
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
pub struct Gtadcmsc_SPEC;
impl crate::sealed::RegSpec for Gtadcmsc_SPEC {
    type DataType = u32;
}

pub type Gtadcmsc = crate::RegValueT<Gtadcmsc_SPEC>;

impl Gtadcmsc {
    #[inline(always)]
    pub fn adcmsc1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        gtadcmsc::Adcmsc1,
        gtadcmsc::Adcmsc1,
        Gtadcmsc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            gtadcmsc::Adcmsc1,
            gtadcmsc::Adcmsc1,
            Gtadcmsc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adcmst1(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Gtadcmsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Gtadcmsc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn adcmscnt1iv(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, Gtadcmsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,Gtadcmsc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn adcmscnt1(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Gtadcmsc_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,Gtadcmsc_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn adcmsc2(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        gtadcmsc::Adcmsc2,
        gtadcmsc::Adcmsc2,
        Gtadcmsc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            gtadcmsc::Adcmsc2,
            gtadcmsc::Adcmsc2,
            Gtadcmsc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adcmst2(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Gtadcmsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8,u8,Gtadcmsc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn adcmscnt2iv(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, u8, Gtadcmsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8,u8,Gtadcmsc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn adcmscnt2(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, u8, Gtadcmsc_SPEC, crate::common::R> {
        crate::common::RegisterField::<28,0xf,1,0,u8,u8,Gtadcmsc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtadcmsc {
    #[inline(always)]
    fn default() -> Gtadcmsc {
        <crate::RegValueT<Gtadcmsc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtadcmsc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adcmsc1_SPEC;
    pub type Adcmsc1 = crate::EnumBitfieldStruct<u8, Adcmsc1_SPEC>;
    impl Adcmsc1 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adcmsc2_SPEC;
    pub type Adcmsc2 = crate::EnumBitfieldStruct<u8, Adcmsc2_SPEC>;
    impl Adcmsc2 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtadcmss_SPEC;
impl crate::sealed::RegSpec for Gtadcmss_SPEC {
    type DataType = u32;
}

pub type Gtadcmss = crate::RegValueT<Gtadcmss_SPEC>;

impl Gtadcmss {
    #[inline(always)]
    pub fn adcmsal(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Gtadcmss_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Gtadcmss_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn adcmsbl(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Gtadcmss_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Gtadcmss_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn adcmbsa(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, Gtadcmss_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,Gtadcmss_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn adcmbsb(
        self,
    ) -> crate::common::RegisterField<20, 0x7, 1, 0, u8, u8, Gtadcmss_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x7,1,0,u8,u8,Gtadcmss_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtadcmss {
    #[inline(always)]
    fn default() -> Gtadcmss {
        <crate::RegValueT<Gtadcmss_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtsecsr_SPEC;
impl crate::sealed::RegSpec for Gtsecsr_SPEC {
    type DataType = u32;
}

pub type Gtsecsr = crate::RegValueT<Gtsecsr_SPEC>;

impl Gtsecsr {
    #[inline(always)]
    pub fn secsel0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtsecsr::Secsel0,
        gtsecsr::Secsel0,
        Gtsecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtsecsr::Secsel0,
            gtsecsr::Secsel0,
            Gtsecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn secsel1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gtsecsr::Secsel1,
        gtsecsr::Secsel1,
        Gtsecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gtsecsr::Secsel1,
            gtsecsr::Secsel1,
            Gtsecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn secsel2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gtsecsr::Secsel2,
        gtsecsr::Secsel2,
        Gtsecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gtsecsr::Secsel2,
            gtsecsr::Secsel2,
            Gtsecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn secsel3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gtsecsr::Secsel3,
        gtsecsr::Secsel3,
        Gtsecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gtsecsr::Secsel3,
            gtsecsr::Secsel3,
            Gtsecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn secsel4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gtsecsr::Secsel4,
        gtsecsr::Secsel4,
        Gtsecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gtsecsr::Secsel4,
            gtsecsr::Secsel4,
            Gtsecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn secsel5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gtsecsr::Secsel5,
        gtsecsr::Secsel5,
        Gtsecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gtsecsr::Secsel5,
            gtsecsr::Secsel5,
            Gtsecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn secsel6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        gtsecsr::Secsel6,
        gtsecsr::Secsel6,
        Gtsecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            gtsecsr::Secsel6,
            gtsecsr::Secsel6,
            Gtsecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn secsel7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        gtsecsr::Secsel7,
        gtsecsr::Secsel7,
        Gtsecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            gtsecsr::Secsel7,
            gtsecsr::Secsel7,
            Gtsecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn secsel8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gtsecsr::Secsel8,
        gtsecsr::Secsel8,
        Gtsecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gtsecsr::Secsel8,
            gtsecsr::Secsel8,
            Gtsecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn secsel9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        gtsecsr::Secsel9,
        gtsecsr::Secsel9,
        Gtsecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            gtsecsr::Secsel9,
            gtsecsr::Secsel9,
            Gtsecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtsecsr {
    #[inline(always)]
    fn default() -> Gtsecsr {
        <crate::RegValueT<Gtsecsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtsecsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Secsel0_SPEC;
    pub type Secsel0 = crate::EnumBitfieldStruct<u8, Secsel0_SPEC>;
    impl Secsel0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Secsel1_SPEC;
    pub type Secsel1 = crate::EnumBitfieldStruct<u8, Secsel1_SPEC>;
    impl Secsel1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Secsel2_SPEC;
    pub type Secsel2 = crate::EnumBitfieldStruct<u8, Secsel2_SPEC>;
    impl Secsel2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Secsel3_SPEC;
    pub type Secsel3 = crate::EnumBitfieldStruct<u8, Secsel3_SPEC>;
    impl Secsel3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Secsel4_SPEC;
    pub type Secsel4 = crate::EnumBitfieldStruct<u8, Secsel4_SPEC>;
    impl Secsel4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Secsel5_SPEC;
    pub type Secsel5 = crate::EnumBitfieldStruct<u8, Secsel5_SPEC>;
    impl Secsel5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Secsel6_SPEC;
    pub type Secsel6 = crate::EnumBitfieldStruct<u8, Secsel6_SPEC>;
    impl Secsel6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Secsel7_SPEC;
    pub type Secsel7 = crate::EnumBitfieldStruct<u8, Secsel7_SPEC>;
    impl Secsel7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Secsel8_SPEC;
    pub type Secsel8 = crate::EnumBitfieldStruct<u8, Secsel8_SPEC>;
    impl Secsel8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Secsel9_SPEC;
    pub type Secsel9 = crate::EnumBitfieldStruct<u8, Secsel9_SPEC>;
    impl Secsel9 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtsecr_SPEC;
impl crate::sealed::RegSpec for Gtsecr_SPEC {
    type DataType = u32;
}

pub type Gtsecr = crate::RegValueT<Gtsecr_SPEC>;

impl Gtsecr {
    #[inline(always)]
    pub fn sbdce(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtsecr::Sbdce,
        gtsecr::Sbdce,
        Gtsecr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtsecr::Sbdce,
            gtsecr::Sbdce,
            Gtsecr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sbdpe(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gtsecr::Sbdpe,
        gtsecr::Sbdpe,
        Gtsecr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gtsecr::Sbdpe,
            gtsecr::Sbdpe,
            Gtsecr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sbdae(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gtsecr::Sbdae,
        gtsecr::Sbdae,
        Gtsecr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gtsecr::Sbdae,
            gtsecr::Sbdae,
            Gtsecr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sbdde(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gtsecr::Sbdde,
        gtsecr::Sbdde,
        Gtsecr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gtsecr::Sbdde,
            gtsecr::Sbdde,
            Gtsecr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sbdcd(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gtsecr::Sbdcd,
        gtsecr::Sbdcd,
        Gtsecr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gtsecr::Sbdcd,
            gtsecr::Sbdcd,
            Gtsecr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sbdpd(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        gtsecr::Sbdpd,
        gtsecr::Sbdpd,
        Gtsecr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            gtsecr::Sbdpd,
            gtsecr::Sbdpd,
            Gtsecr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sbdad(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        gtsecr::Sbdad,
        gtsecr::Sbdad,
        Gtsecr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            gtsecr::Sbdad,
            gtsecr::Sbdad,
            Gtsecr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sbddd(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        gtsecr::Sbddd,
        gtsecr::Sbddd,
        Gtsecr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            gtsecr::Sbddd,
            gtsecr::Sbddd,
            Gtsecr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spce(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        gtsecr::Spce,
        gtsecr::Spce,
        Gtsecr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gtsecr::Spce,
            gtsecr::Spce,
            Gtsecr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ssce(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        gtsecr::Ssce,
        gtsecr::Ssce,
        Gtsecr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            gtsecr::Ssce,
            gtsecr::Ssce,
            Gtsecr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spcd(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        gtsecr::Spcd,
        gtsecr::Spcd,
        Gtsecr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            gtsecr::Spcd,
            gtsecr::Spcd,
            Gtsecr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sscd(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        gtsecr::Sscd,
        gtsecr::Sscd,
        Gtsecr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            gtsecr::Sscd,
            gtsecr::Sscd,
            Gtsecr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtsecr {
    #[inline(always)]
    fn default() -> Gtsecr {
        <crate::RegValueT<Gtsecr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtsecr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbdce_SPEC;
    pub type Sbdce = crate::EnumBitfieldStruct<u8, Sbdce_SPEC>;
    impl Sbdce {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbdpe_SPEC;
    pub type Sbdpe = crate::EnumBitfieldStruct<u8, Sbdpe_SPEC>;
    impl Sbdpe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbdae_SPEC;
    pub type Sbdae = crate::EnumBitfieldStruct<u8, Sbdae_SPEC>;
    impl Sbdae {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbdde_SPEC;
    pub type Sbdde = crate::EnumBitfieldStruct<u8, Sbdde_SPEC>;
    impl Sbdde {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbdcd_SPEC;
    pub type Sbdcd = crate::EnumBitfieldStruct<u8, Sbdcd_SPEC>;
    impl Sbdcd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbdpd_SPEC;
    pub type Sbdpd = crate::EnumBitfieldStruct<u8, Sbdpd_SPEC>;
    impl Sbdpd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbdad_SPEC;
    pub type Sbdad = crate::EnumBitfieldStruct<u8, Sbdad_SPEC>;
    impl Sbdad {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbddd_SPEC;
    pub type Sbddd = crate::EnumBitfieldStruct<u8, Sbddd_SPEC>;
    impl Sbddd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spce_SPEC;
    pub type Spce = crate::EnumBitfieldStruct<u8, Spce_SPEC>;
    impl Spce {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssce_SPEC;
    pub type Ssce = crate::EnumBitfieldStruct<u8, Ssce_SPEC>;
    impl Ssce {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spcd_SPEC;
    pub type Spcd = crate::EnumBitfieldStruct<u8, Spcd_SPEC>;
    impl Spcd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sscd_SPEC;
    pub type Sscd = crate::EnumBitfieldStruct<u8, Sscd_SPEC>;
    impl Sscd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtber2_SPEC;
impl crate::sealed::RegSpec for Gtber2_SPEC {
    type DataType = u32;
}

pub type Gtber2 = crate::RegValueT<Gtber2_SPEC>;

impl Gtber2 {
    #[inline(always)]
    pub fn cctca(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gtber2::Cctca,
        gtber2::Cctca,
        Gtber2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gtber2::Cctca,
            gtber2::Cctca,
            Gtber2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cctcb(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gtber2::Cctcb,
        gtber2::Cctcb,
        Gtber2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gtber2::Cctcb,
            gtber2::Cctcb,
            Gtber2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cctpr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gtber2::Cctpr,
        gtber2::Cctpr,
        Gtber2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gtber2::Cctpr,
            gtber2::Cctpr,
            Gtber2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cctada(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gtber2::Cctada,
        gtber2::Cctada,
        Gtber2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gtber2::Cctada,
            gtber2::Cctada,
            Gtber2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cctadb(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gtber2::Cctadb,
        gtber2::Cctadb,
        Gtber2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gtber2::Cctadb,
            gtber2::Cctadb,
            Gtber2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cctdv(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gtber2::Cctdv,
        gtber2::Cctdv,
        Gtber2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gtber2::Cctdv,
            gtber2::Cctdv,
            Gtber2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmtca(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        gtber2::Cmtca,
        gtber2::Cmtca,
        Gtber2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            gtber2::Cmtca,
            gtber2::Cmtca,
            Gtber2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmtcb(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        gtber2::Cmtcb,
        gtber2::Cmtcb,
        Gtber2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            gtber2::Cmtcb,
            gtber2::Cmtcb,
            Gtber2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmtada(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        gtber2::Cmtada,
        gtber2::Cmtada,
        Gtber2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            gtber2::Cmtada,
            gtber2::Cmtada,
            Gtber2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmtadb(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        gtber2::Cmtadb,
        gtber2::Cmtadb,
        Gtber2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            gtber2::Cmtadb,
            gtber2::Cmtadb,
            Gtber2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cptca(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        gtber2::Cptca,
        gtber2::Cptca,
        Gtber2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gtber2::Cptca,
            gtber2::Cptca,
            Gtber2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cptcb(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        gtber2::Cptcb,
        gtber2::Cptcb,
        Gtber2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            gtber2::Cptcb,
            gtber2::Cptcb,
            Gtber2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cptpr(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        gtber2::Cptpr,
        gtber2::Cptpr,
        Gtber2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            gtber2::Cptpr,
            gtber2::Cptpr,
            Gtber2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cptada(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        gtber2::Cptada,
        gtber2::Cptada,
        Gtber2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            gtber2::Cptada,
            gtber2::Cptada,
            Gtber2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cptadb(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        gtber2::Cptadb,
        gtber2::Cptadb,
        Gtber2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            gtber2::Cptadb,
            gtber2::Cptadb,
            Gtber2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cptdv(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        gtber2::Cptdv,
        gtber2::Cptdv,
        Gtber2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            gtber2::Cptdv,
            gtber2::Cptdv,
            Gtber2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cp3db(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        gtber2::Cp3Db,
        gtber2::Cp3Db,
        Gtber2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            gtber2::Cp3Db,
            gtber2::Cp3Db,
            Gtber2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cpbtd(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        gtber2::Cpbtd,
        gtber2::Cpbtd,
        Gtber2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            gtber2::Cpbtd,
            gtber2::Cpbtd,
            Gtber2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn oltta(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x3,
        1,
        0,
        gtber2::Oltta,
        gtber2::Oltta,
        Gtber2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x3,
            1,
            0,
            gtber2::Oltta,
            gtber2::Oltta,
            Gtber2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn olttb(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x3,
        1,
        0,
        gtber2::Olttb,
        gtber2::Olttb,
        Gtber2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x3,
            1,
            0,
            gtber2::Olttb,
            gtber2::Olttb,
            Gtber2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gtber2 {
    #[inline(always)]
    fn default() -> Gtber2 {
        <crate::RegValueT<Gtber2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtber2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cctca_SPEC;
    pub type Cctca = crate::EnumBitfieldStruct<u8, Cctca_SPEC>;
    impl Cctca {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cctcb_SPEC;
    pub type Cctcb = crate::EnumBitfieldStruct<u8, Cctcb_SPEC>;
    impl Cctcb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cctpr_SPEC;
    pub type Cctpr = crate::EnumBitfieldStruct<u8, Cctpr_SPEC>;
    impl Cctpr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cctada_SPEC;
    pub type Cctada = crate::EnumBitfieldStruct<u8, Cctada_SPEC>;
    impl Cctada {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cctadb_SPEC;
    pub type Cctadb = crate::EnumBitfieldStruct<u8, Cctadb_SPEC>;
    impl Cctadb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cctdv_SPEC;
    pub type Cctdv = crate::EnumBitfieldStruct<u8, Cctdv_SPEC>;
    impl Cctdv {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmtca_SPEC;
    pub type Cmtca = crate::EnumBitfieldStruct<u8, Cmtca_SPEC>;
    impl Cmtca {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmtcb_SPEC;
    pub type Cmtcb = crate::EnumBitfieldStruct<u8, Cmtcb_SPEC>;
    impl Cmtcb {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmtada_SPEC;
    pub type Cmtada = crate::EnumBitfieldStruct<u8, Cmtada_SPEC>;
    impl Cmtada {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmtadb_SPEC;
    pub type Cmtadb = crate::EnumBitfieldStruct<u8, Cmtadb_SPEC>;
    impl Cmtadb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cptca_SPEC;
    pub type Cptca = crate::EnumBitfieldStruct<u8, Cptca_SPEC>;
    impl Cptca {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cptcb_SPEC;
    pub type Cptcb = crate::EnumBitfieldStruct<u8, Cptcb_SPEC>;
    impl Cptcb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cptpr_SPEC;
    pub type Cptpr = crate::EnumBitfieldStruct<u8, Cptpr_SPEC>;
    impl Cptpr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cptada_SPEC;
    pub type Cptada = crate::EnumBitfieldStruct<u8, Cptada_SPEC>;
    impl Cptada {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cptadb_SPEC;
    pub type Cptadb = crate::EnumBitfieldStruct<u8, Cptadb_SPEC>;
    impl Cptadb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cptdv_SPEC;
    pub type Cptdv = crate::EnumBitfieldStruct<u8, Cptdv_SPEC>;
    impl Cptdv {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cp3Db_SPEC;
    pub type Cp3Db = crate::EnumBitfieldStruct<u8, Cp3Db_SPEC>;
    impl Cp3Db {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpbtd_SPEC;
    pub type Cpbtd = crate::EnumBitfieldStruct<u8, Cpbtd_SPEC>;
    impl Cpbtd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oltta_SPEC;
    pub type Oltta = crate::EnumBitfieldStruct<u8, Oltta_SPEC>;
    impl Oltta {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Olttb_SPEC;
    pub type Olttb = crate::EnumBitfieldStruct<u8, Olttb_SPEC>;
    impl Olttb {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtolbr_SPEC;
impl crate::sealed::RegSpec for Gtolbr_SPEC {
    type DataType = u32;
}

pub type Gtolbr = crate::RegValueT<Gtolbr_SPEC>;

impl Gtolbr {
    #[inline(always)]
    pub fn gtioab(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Gtolbr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Gtolbr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gtiobb(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Gtolbr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Gtolbr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtolbr {
    #[inline(always)]
    fn default() -> Gtolbr {
        <crate::RegValueT<Gtolbr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gticcr_SPEC;
impl crate::sealed::RegSpec for Gticcr_SPEC {
    type DataType = u32;
}

pub type Gticcr = crate::RegValueT<Gticcr_SPEC>;

impl Gticcr {
    #[inline(always)]
    pub fn icafa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gticcr::Icafa,
        gticcr::Icafa,
        Gticcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gticcr::Icafa,
            gticcr::Icafa,
            Gticcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn icafb(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gticcr::Icafb,
        gticcr::Icafb,
        Gticcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gticcr::Icafb,
            gticcr::Icafb,
            Gticcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn icafc(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gticcr::Icafc,
        gticcr::Icafc,
        Gticcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gticcr::Icafc,
            gticcr::Icafc,
            Gticcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn icafd(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gticcr::Icafd,
        gticcr::Icafd,
        Gticcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gticcr::Icafd,
            gticcr::Icafd,
            Gticcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn icafe(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gticcr::Icafe,
        gticcr::Icafe,
        Gticcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gticcr::Icafe,
            gticcr::Icafe,
            Gticcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn icaff(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gticcr::Icaff,
        gticcr::Icaff,
        Gticcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gticcr::Icaff,
            gticcr::Icaff,
            Gticcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn icafpo(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        gticcr::Icafpo,
        gticcr::Icafpo,
        Gticcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            gticcr::Icafpo,
            gticcr::Icafpo,
            Gticcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn icafpu(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        gticcr::Icafpu,
        gticcr::Icafpu,
        Gticcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            gticcr::Icafpu,
            gticcr::Icafpu,
            Gticcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn icaclk(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        gticcr::Icaclk,
        gticcr::Icaclk,
        Gticcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gticcr::Icaclk,
            gticcr::Icaclk,
            Gticcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn icagrp(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        gticcr::Icagrp,
        gticcr::Icagrp,
        Gticcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            gticcr::Icagrp,
            gticcr::Icagrp,
            Gticcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn icbfa(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        gticcr::Icbfa,
        gticcr::Icbfa,
        Gticcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gticcr::Icbfa,
            gticcr::Icbfa,
            Gticcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn icbfb(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        gticcr::Icbfb,
        gticcr::Icbfb,
        Gticcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            gticcr::Icbfb,
            gticcr::Icbfb,
            Gticcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn icbfc(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        gticcr::Icbfc,
        gticcr::Icbfc,
        Gticcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            gticcr::Icbfc,
            gticcr::Icbfc,
            Gticcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn icbfd(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        gticcr::Icbfd,
        gticcr::Icbfd,
        Gticcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            gticcr::Icbfd,
            gticcr::Icbfd,
            Gticcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn icbfe(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        gticcr::Icbfe,
        gticcr::Icbfe,
        Gticcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            gticcr::Icbfe,
            gticcr::Icbfe,
            Gticcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn icbff(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        gticcr::Icbff,
        gticcr::Icbff,
        Gticcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            gticcr::Icbff,
            gticcr::Icbff,
            Gticcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn icbfpo(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        gticcr::Icbfpo,
        gticcr::Icbfpo,
        Gticcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            gticcr::Icbfpo,
            gticcr::Icbfpo,
            Gticcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn icbfpu(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        gticcr::Icbfpu,
        gticcr::Icbfpu,
        Gticcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            gticcr::Icbfpu,
            gticcr::Icbfpu,
            Gticcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn icbclk(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        gticcr::Icbclk,
        gticcr::Icbclk,
        Gticcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            gticcr::Icbclk,
            gticcr::Icbclk,
            Gticcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn icbgrp(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x3,
        1,
        0,
        gticcr::Icbgrp,
        gticcr::Icbgrp,
        Gticcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x3,
            1,
            0,
            gticcr::Icbgrp,
            gticcr::Icbgrp,
            Gticcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gticcr {
    #[inline(always)]
    fn default() -> Gticcr {
        <crate::RegValueT<Gticcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gticcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icafa_SPEC;
    pub type Icafa = crate::EnumBitfieldStruct<u8, Icafa_SPEC>;
    impl Icafa {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icafb_SPEC;
    pub type Icafb = crate::EnumBitfieldStruct<u8, Icafb_SPEC>;
    impl Icafb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icafc_SPEC;
    pub type Icafc = crate::EnumBitfieldStruct<u8, Icafc_SPEC>;
    impl Icafc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icafd_SPEC;
    pub type Icafd = crate::EnumBitfieldStruct<u8, Icafd_SPEC>;
    impl Icafd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icafe_SPEC;
    pub type Icafe = crate::EnumBitfieldStruct<u8, Icafe_SPEC>;
    impl Icafe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icaff_SPEC;
    pub type Icaff = crate::EnumBitfieldStruct<u8, Icaff_SPEC>;
    impl Icaff {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icafpo_SPEC;
    pub type Icafpo = crate::EnumBitfieldStruct<u8, Icafpo_SPEC>;
    impl Icafpo {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icafpu_SPEC;
    pub type Icafpu = crate::EnumBitfieldStruct<u8, Icafpu_SPEC>;
    impl Icafpu {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icaclk_SPEC;
    pub type Icaclk = crate::EnumBitfieldStruct<u8, Icaclk_SPEC>;
    impl Icaclk {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icagrp_SPEC;
    pub type Icagrp = crate::EnumBitfieldStruct<u8, Icagrp_SPEC>;
    impl Icagrp {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icbfa_SPEC;
    pub type Icbfa = crate::EnumBitfieldStruct<u8, Icbfa_SPEC>;
    impl Icbfa {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icbfb_SPEC;
    pub type Icbfb = crate::EnumBitfieldStruct<u8, Icbfb_SPEC>;
    impl Icbfb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icbfc_SPEC;
    pub type Icbfc = crate::EnumBitfieldStruct<u8, Icbfc_SPEC>;
    impl Icbfc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icbfd_SPEC;
    pub type Icbfd = crate::EnumBitfieldStruct<u8, Icbfd_SPEC>;
    impl Icbfd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icbfe_SPEC;
    pub type Icbfe = crate::EnumBitfieldStruct<u8, Icbfe_SPEC>;
    impl Icbfe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icbff_SPEC;
    pub type Icbff = crate::EnumBitfieldStruct<u8, Icbff_SPEC>;
    impl Icbff {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icbfpo_SPEC;
    pub type Icbfpo = crate::EnumBitfieldStruct<u8, Icbfpo_SPEC>;
    impl Icbfpo {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icbfpu_SPEC;
    pub type Icbfpu = crate::EnumBitfieldStruct<u8, Icbfpu_SPEC>;
    impl Icbfpu {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icbclk_SPEC;
    pub type Icbclk = crate::EnumBitfieldStruct<u8, Icbclk_SPEC>;
    impl Icbclk {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icbgrp_SPEC;
    pub type Icbgrp = crate::EnumBitfieldStruct<u8, Icbgrp_SPEC>;
    impl Icbgrp {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
