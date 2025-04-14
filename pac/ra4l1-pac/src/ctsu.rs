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
#[doc = r"Capacitive Touch Sensing Unit"]
unsafe impl ::core::marker::Send for super::Ctsu {}
unsafe impl ::core::marker::Sync for super::Ctsu {}
impl super::Ctsu {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn ctsucra(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsucra_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsucra_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsucral(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsucral_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsucral_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsucr0(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsucr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsucr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsucr1(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsucr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsucr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsucr2(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsucr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsucr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsucr3(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsucr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsucr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsucrb(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsucrb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsucrb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsucrbl(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsucrbl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsucrbl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsusdprs(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsusdprs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsusdprs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsusst(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsusst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsusst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsucrbh(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsucrbh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsucrbh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsudclkc(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsudclkc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsudclkc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(7usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsumch(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsumch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsumch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsumchl(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsumchl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsumchl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsumch0(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsumch0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsumch0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsumch1(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsumch1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsumch1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(9usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsumchh(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsumchh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsumchh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsumfaf(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsumfaf_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsumfaf_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuchaca(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuchacal(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchacal_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchacal_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuchac0(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchac0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchac0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuchac1(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchac1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchac1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(13usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuchtrca(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchtrca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchtrca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuchtrcal(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchtrcal_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchtrcal_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuchtrc0(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchtrc0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchtrc0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuchtrc1(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchtrc1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchtrc1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(21usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsusr(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsusr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsusr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsusrl(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsusrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsusrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsusr0(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsusr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsusr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsust(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsust_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsust_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(29usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsusrh(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsusrh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsusrh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(30usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsusr2(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsusr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsusr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(30usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuso(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuso_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuso_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuso0(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuso0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuso0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuso1(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuso1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuso1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(34usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuscnt(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuscnt_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ctsuscnt_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsusc(&self) -> &'static crate::common::Reg<self::Ctsusc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ctsusc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsucalib(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsucalib_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsucalib_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsudbgr0(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsudbgr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsudbgr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsudbgr1(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsudbgr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsudbgr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(42usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsusuclka(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsusuclka_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsusuclka_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsusuclk0(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsusuclk0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsusuclk0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsusuclk1(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsusuclk1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsusuclk1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(46usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsusuclkb(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsusuclkb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsusuclkb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsusuclk2(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsusuclk2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsusuclk2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsusuclk3(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsusuclk3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsusuclk3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(50usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuopt(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuopt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuopt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuoptl(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuoptl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuoptl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ac(&self) -> &'static crate::common::Reg<self::Ac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn aj(&self) -> &'static crate::common::Reg<self::Aj_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Aj_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuopth(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuopth_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuopth_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66usize),
            )
        }
    }

    #[inline(always)]
    pub const fn acsb(&self) -> &'static crate::common::Reg<self::Acsb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Acsb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuscntact(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuscntact_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuscntact_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuscntactl(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuscntactl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuscntactl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuscntacth(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuscntacth_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuscntacth_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(70usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsumact1(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsumact1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsumact1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsumact1l(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsumact1L_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsumact1L_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsumact1h(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsumact1H_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsumact1H_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(78usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsumact2(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsumact2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsumact2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsumact2l(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsumact2L_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsumact2L_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsumact2h(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsumact2H_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsumact2H_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(82usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsumact3(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsumact3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsumact3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsumact3l(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsumact3L_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsumact3L_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsumact3h(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsumact3H_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsumact3H_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(86usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuajcr(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuajcrl(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajcrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajcrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ajcr0(&self) -> &'static crate::common::Reg<self::Ajcr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ajcr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ajcr1(&self) -> &'static crate::common::Reg<self::Ajcr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ajcr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(89usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuajcrh(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajcrh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajcrh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(90usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ajcr2(&self) -> &'static crate::common::Reg<self::Ajcr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ajcr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(90usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ajcr3(&self) -> &'static crate::common::Reg<self::Ajcr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ajcr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(91usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuajthr(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajthr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajthr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuajthrl(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajthrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajthrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuajthrh(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajthrh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajthrh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(94usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuajmmar(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajmmar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajmmar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuajmmarl(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajmmarl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajmmarl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuajmmarh(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajmmarh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajmmarh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(98usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuajblact(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajblact_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajblact_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuajblactl(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajblactl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajblactl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuajblacth(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajblacth_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajblacth_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(102usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuajblar(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajblar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajblar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuajblarl(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajblarl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajblarl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuajblarh(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajblarh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajblarh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(106usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuajrr(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajrr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajrr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuajrrl(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajrrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajrrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuajrr0(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajrr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajrr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuajrr1(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuajrr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuajrr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(109usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsucra_SPEC;
impl crate::sealed::RegSpec for Ctsucra_SPEC {
    type DataType = u32;
}

pub type Ctsucra = crate::RegValueT<Ctsucra_SPEC>;

impl Ctsucra {
    #[inline(always)]
    pub fn strt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ctsucra::Strt,
        ctsucra::Strt,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ctsucra::Strt,
            ctsucra::Strt,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cap(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ctsucra::Cap,
        ctsucra::Cap,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ctsucra::Cap,
            ctsucra::Cap,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn snz(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ctsucra::Snz,
        ctsucra::Snz,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ctsucra::Snz,
            ctsucra::Snz,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn init(self) -> crate::common::RegisterFieldBool<4, 1, 0, Ctsucra_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ctsucra_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn pumpon(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ctsucra::Pumpon,
        ctsucra::Pumpon,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ctsucra::Pumpon,
            ctsucra::Pumpon,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txvsel(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        ctsucra::Txvsel,
        ctsucra::Txvsel,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            ctsucra::Txvsel,
            ctsucra::Txvsel,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pon(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        ctsucra::Pon,
        ctsucra::Pon,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            ctsucra::Pon,
            ctsucra::Pon,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn csw(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ctsucra::Csw,
        ctsucra::Csw,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ctsucra::Csw,
            ctsucra::Csw,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn atune0(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        ctsucra::Atune0,
        ctsucra::Atune0,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            ctsucra::Atune0,
            ctsucra::Atune0,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn atune1(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        ctsucra::Atune1,
        ctsucra::Atune1,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            ctsucra::Atune1,
            ctsucra::Atune1,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn clk(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        ctsucra::Clk,
        ctsucra::Clk,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            ctsucra::Clk,
            ctsucra::Clk,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn md0(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        ctsucra::Md0,
        ctsucra::Md0,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            ctsucra::Md0,
            ctsucra::Md0,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn md1(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        ctsucra::Md1,
        ctsucra::Md1,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            ctsucra::Md1,
            ctsucra::Md1,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn atune2(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        ctsucra::Atune2,
        ctsucra::Atune2,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            ctsucra::Atune2,
            ctsucra::Atune2,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn load(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x3,
        1,
        0,
        ctsucra::Load,
        ctsucra::Load,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x3,
            1,
            0,
            ctsucra::Load,
            ctsucra::Load,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn posel(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x3,
        1,
        0,
        ctsucra::Posel,
        ctsucra::Posel,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x3,
            1,
            0,
            ctsucra::Posel,
            ctsucra::Posel,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sdpsel(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        ctsucra::Sdpsel,
        ctsucra::Sdpsel,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            ctsucra::Sdpsel,
            ctsucra::Sdpsel,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pcsel(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        ctsucra::Pcsel,
        ctsucra::Pcsel,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            ctsucra::Pcsel,
            ctsucra::Pcsel,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn stclk(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, u8, Ctsucra_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3f,1,0,u8,u8,Ctsucra_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcmode(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        ctsucra::Dcmode,
        ctsucra::Dcmode,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            ctsucra::Dcmode,
            ctsucra::Dcmode,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dcback(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        ctsucra::Dcback,
        ctsucra::Dcback,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            ctsucra::Dcback,
            ctsucra::Dcback,
            Ctsucra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsucra {
    #[inline(always)]
    fn default() -> Ctsucra {
        <crate::RegValueT<Ctsucra_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctsucra {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Strt_SPEC;
    pub type Strt = crate::EnumBitfieldStruct<u8, Strt_SPEC>;
    impl Strt {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cap_SPEC;
    pub type Cap = crate::EnumBitfieldStruct<u8, Cap_SPEC>;
    impl Cap {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snz_SPEC;
    pub type Snz = crate::EnumBitfieldStruct<u8, Snz_SPEC>;
    impl Snz {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pumpon_SPEC;
    pub type Pumpon = crate::EnumBitfieldStruct<u8, Pumpon_SPEC>;
    impl Pumpon {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txvsel_SPEC;
    pub type Txvsel = crate::EnumBitfieldStruct<u8, Txvsel_SPEC>;
    impl Txvsel {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pon_SPEC;
    pub type Pon = crate::EnumBitfieldStruct<u8, Pon_SPEC>;
    impl Pon {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csw_SPEC;
    pub type Csw = crate::EnumBitfieldStruct<u8, Csw_SPEC>;
    impl Csw {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Atune0_SPEC;
    pub type Atune0 = crate::EnumBitfieldStruct<u8, Atune0_SPEC>;
    impl Atune0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Atune1_SPEC;
    pub type Atune1 = crate::EnumBitfieldStruct<u8, Atune1_SPEC>;
    impl Atune1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clk_SPEC;
    pub type Clk = crate::EnumBitfieldStruct<u8, Clk_SPEC>;
    impl Clk {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Atune2_SPEC;
    pub type Atune2 = crate::EnumBitfieldStruct<u8, Atune2_SPEC>;
    impl Atune2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Load_SPEC;
    pub type Load = crate::EnumBitfieldStruct<u8, Load_SPEC>;
    impl Load {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Posel_SPEC;
    pub type Posel = crate::EnumBitfieldStruct<u8, Posel_SPEC>;
    impl Posel {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdpsel_SPEC;
    pub type Sdpsel = crate::EnumBitfieldStruct<u8, Sdpsel_SPEC>;
    impl Sdpsel {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcsel_SPEC;
    pub type Pcsel = crate::EnumBitfieldStruct<u8, Pcsel_SPEC>;
    impl Pcsel {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcmode_SPEC;
    pub type Dcmode = crate::EnumBitfieldStruct<u8, Dcmode_SPEC>;
    impl Dcmode {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcback_SPEC;
    pub type Dcback = crate::EnumBitfieldStruct<u8, Dcback_SPEC>;
    impl Dcback {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsucral_SPEC;
impl crate::sealed::RegSpec for Ctsucral_SPEC {
    type DataType = u16;
}

pub type Ctsucral = crate::RegValueT<Ctsucral_SPEC>;

impl NoBitfieldReg<Ctsucral_SPEC> for Ctsucral {}
impl ::core::default::Default for Ctsucral {
    #[inline(always)]
    fn default() -> Ctsucral {
        <crate::RegValueT<Ctsucral_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsucr0_SPEC;
impl crate::sealed::RegSpec for Ctsucr0_SPEC {
    type DataType = u8;
}

pub type Ctsucr0 = crate::RegValueT<Ctsucr0_SPEC>;

impl NoBitfieldReg<Ctsucr0_SPEC> for Ctsucr0 {}
impl ::core::default::Default for Ctsucr0 {
    #[inline(always)]
    fn default() -> Ctsucr0 {
        <crate::RegValueT<Ctsucr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsucr1_SPEC;
impl crate::sealed::RegSpec for Ctsucr1_SPEC {
    type DataType = u8;
}

pub type Ctsucr1 = crate::RegValueT<Ctsucr1_SPEC>;

impl NoBitfieldReg<Ctsucr1_SPEC> for Ctsucr1 {}
impl ::core::default::Default for Ctsucr1 {
    #[inline(always)]
    fn default() -> Ctsucr1 {
        <crate::RegValueT<Ctsucr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsucr2_SPEC;
impl crate::sealed::RegSpec for Ctsucr2_SPEC {
    type DataType = u8;
}

pub type Ctsucr2 = crate::RegValueT<Ctsucr2_SPEC>;

impl NoBitfieldReg<Ctsucr2_SPEC> for Ctsucr2 {}
impl ::core::default::Default for Ctsucr2 {
    #[inline(always)]
    fn default() -> Ctsucr2 {
        <crate::RegValueT<Ctsucr2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsucr3_SPEC;
impl crate::sealed::RegSpec for Ctsucr3_SPEC {
    type DataType = u8;
}

pub type Ctsucr3 = crate::RegValueT<Ctsucr3_SPEC>;

impl NoBitfieldReg<Ctsucr3_SPEC> for Ctsucr3 {}
impl ::core::default::Default for Ctsucr3 {
    #[inline(always)]
    fn default() -> Ctsucr3 {
        <crate::RegValueT<Ctsucr3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsucrb_SPEC;
impl crate::sealed::RegSpec for Ctsucrb_SPEC {
    type DataType = u32;
}

pub type Ctsucrb = crate::RegValueT<Ctsucrb_SPEC>;

impl Ctsucrb {
    #[inline(always)]
    pub fn prratio(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Ctsucrb_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Ctsucrb_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn prmode(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        ctsucrb::Prmode,
        ctsucrb::Prmode,
        Ctsucrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            ctsucrb::Prmode,
            ctsucrb::Prmode,
            Ctsucrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn soff(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ctsucrb::Soff,
        ctsucrb::Soff,
        Ctsucrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ctsucrb::Soff,
            ctsucrb::Soff,
            Ctsucrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn proff(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ctsucrb::Proff,
        ctsucrb::Proff,
        Ctsucrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ctsucrb::Proff,
            ctsucrb::Proff,
            Ctsucrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sst(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ctsucrb_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ctsucrb_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ssmod(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        ctsucrb::Ssmod,
        ctsucrb::Ssmod,
        Ctsucrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            ctsucrb::Ssmod,
            ctsucrb::Ssmod,
            Ctsucrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sscnt(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x3,
        1,
        0,
        ctsucrb::Sscnt,
        ctsucrb::Sscnt,
        Ctsucrb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x3,
            1,
            0,
            ctsucrb::Sscnt,
            ctsucrb::Sscnt,
            Ctsucrb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsucrb {
    #[inline(always)]
    fn default() -> Ctsucrb {
        <crate::RegValueT<Ctsucrb_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctsucrb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prmode_SPEC;
    pub type Prmode = crate::EnumBitfieldStruct<u8, Prmode_SPEC>;
    impl Prmode {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Soff_SPEC;
    pub type Soff = crate::EnumBitfieldStruct<u8, Soff_SPEC>;
    impl Soff {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Proff_SPEC;
    pub type Proff = crate::EnumBitfieldStruct<u8, Proff_SPEC>;
    impl Proff {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssmod_SPEC;
    pub type Ssmod = crate::EnumBitfieldStruct<u8, Ssmod_SPEC>;
    impl Ssmod {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _111: Self = Self::new(7);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sscnt_SPEC;
    pub type Sscnt = crate::EnumBitfieldStruct<u8, Sscnt_SPEC>;
    impl Sscnt {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsucrbl_SPEC;
impl crate::sealed::RegSpec for Ctsucrbl_SPEC {
    type DataType = u16;
}

pub type Ctsucrbl = crate::RegValueT<Ctsucrbl_SPEC>;

impl NoBitfieldReg<Ctsucrbl_SPEC> for Ctsucrbl {}
impl ::core::default::Default for Ctsucrbl {
    #[inline(always)]
    fn default() -> Ctsucrbl {
        <crate::RegValueT<Ctsucrbl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusdprs_SPEC;
impl crate::sealed::RegSpec for Ctsusdprs_SPEC {
    type DataType = u8;
}

pub type Ctsusdprs = crate::RegValueT<Ctsusdprs_SPEC>;

impl NoBitfieldReg<Ctsusdprs_SPEC> for Ctsusdprs {}
impl ::core::default::Default for Ctsusdprs {
    #[inline(always)]
    fn default() -> Ctsusdprs {
        <crate::RegValueT<Ctsusdprs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusst_SPEC;
impl crate::sealed::RegSpec for Ctsusst_SPEC {
    type DataType = u8;
}

pub type Ctsusst = crate::RegValueT<Ctsusst_SPEC>;

impl NoBitfieldReg<Ctsusst_SPEC> for Ctsusst {}
impl ::core::default::Default for Ctsusst {
    #[inline(always)]
    fn default() -> Ctsusst {
        <crate::RegValueT<Ctsusst_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsucrbh_SPEC;
impl crate::sealed::RegSpec for Ctsucrbh_SPEC {
    type DataType = u16;
}

pub type Ctsucrbh = crate::RegValueT<Ctsucrbh_SPEC>;

impl NoBitfieldReg<Ctsucrbh_SPEC> for Ctsucrbh {}
impl ::core::default::Default for Ctsucrbh {
    #[inline(always)]
    fn default() -> Ctsucrbh {
        <crate::RegValueT<Ctsucrbh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsudclkc_SPEC;
impl crate::sealed::RegSpec for Ctsudclkc_SPEC {
    type DataType = u8;
}

pub type Ctsudclkc = crate::RegValueT<Ctsudclkc_SPEC>;

impl NoBitfieldReg<Ctsudclkc_SPEC> for Ctsudclkc {}
impl ::core::default::Default for Ctsudclkc {
    #[inline(always)]
    fn default() -> Ctsudclkc {
        <crate::RegValueT<Ctsudclkc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumch_SPEC;
impl crate::sealed::RegSpec for Ctsumch_SPEC {
    type DataType = u32;
}

pub type Ctsumch = crate::RegValueT<Ctsumch_SPEC>;

impl Ctsumch {
    #[inline(always)]
    pub fn mch0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        ctsumch::Mch0,
        ctsumch::Mch0,
        Ctsumch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            ctsumch::Mch0,
            ctsumch::Mch0,
            Ctsumch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mch1(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3f,
        1,
        0,
        ctsumch::Mch1,
        ctsumch::Mch1,
        Ctsumch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3f,
            1,
            0,
            ctsumch::Mch1,
            ctsumch::Mch1,
            Ctsumch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mca0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ctsumch::Mca0,
        ctsumch::Mca0,
        Ctsumch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ctsumch::Mca0,
            ctsumch::Mca0,
            Ctsumch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mca1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        ctsumch::Mca1,
        ctsumch::Mca1,
        Ctsumch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            ctsumch::Mca1,
            ctsumch::Mca1,
            Ctsumch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mca2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        ctsumch::Mca2,
        ctsumch::Mca2,
        Ctsumch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            ctsumch::Mca2,
            ctsumch::Mca2,
            Ctsumch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mca3(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        ctsumch::Mca3,
        ctsumch::Mca3,
        Ctsumch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            ctsumch::Mca3,
            ctsumch::Mca3,
            Ctsumch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsumch {
    #[inline(always)]
    fn default() -> Ctsumch {
        <crate::RegValueT<Ctsumch_SPEC> as RegisterValue<_>>::new(16191)
    }
}
pub mod ctsumch {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mch0_SPEC;
    pub type Mch0 = crate::EnumBitfieldStruct<u8, Mch0_SPEC>;
    impl Mch0 {
        pub const _0_X_00: Self = Self::new(0);

        pub const _0_X_01: Self = Self::new(1);

        pub const _0_X_02: Self = Self::new(2);

        pub const _0_X_03: Self = Self::new(3);

        pub const _0_X_04: Self = Self::new(4);

        pub const _0_X_05: Self = Self::new(5);

        pub const _0_X_06: Self = Self::new(6);

        pub const _0_X_07: Self = Self::new(7);

        pub const _0_X_08: Self = Self::new(8);

        pub const _0_X_09: Self = Self::new(9);

        pub const _0_X_0_A: Self = Self::new(10);

        pub const _0_X_0_B: Self = Self::new(11);

        pub const _0_X_3_F: Self = Self::new(63);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mch1_SPEC;
    pub type Mch1 = crate::EnumBitfieldStruct<u8, Mch1_SPEC>;
    impl Mch1 {
        pub const _0_X_00: Self = Self::new(0);

        pub const _0_X_01: Self = Self::new(1);

        pub const _0_X_02: Self = Self::new(2);

        pub const _0_X_03: Self = Self::new(3);

        pub const _0_X_04: Self = Self::new(4);

        pub const _0_X_05: Self = Self::new(5);

        pub const _0_X_06: Self = Self::new(6);

        pub const _0_X_07: Self = Self::new(7);

        pub const _0_X_08: Self = Self::new(8);

        pub const _0_X_09: Self = Self::new(9);

        pub const _0_X_0_A: Self = Self::new(10);

        pub const _0_X_0_B: Self = Self::new(11);

        pub const _0_X_3_F: Self = Self::new(63);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mca0_SPEC;
    pub type Mca0 = crate::EnumBitfieldStruct<u8, Mca0_SPEC>;
    impl Mca0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mca1_SPEC;
    pub type Mca1 = crate::EnumBitfieldStruct<u8, Mca1_SPEC>;
    impl Mca1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mca2_SPEC;
    pub type Mca2 = crate::EnumBitfieldStruct<u8, Mca2_SPEC>;
    impl Mca2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mca3_SPEC;
    pub type Mca3 = crate::EnumBitfieldStruct<u8, Mca3_SPEC>;
    impl Mca3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumchl_SPEC;
impl crate::sealed::RegSpec for Ctsumchl_SPEC {
    type DataType = u16;
}

pub type Ctsumchl = crate::RegValueT<Ctsumchl_SPEC>;

impl NoBitfieldReg<Ctsumchl_SPEC> for Ctsumchl {}
impl ::core::default::Default for Ctsumchl {
    #[inline(always)]
    fn default() -> Ctsumchl {
        <crate::RegValueT<Ctsumchl_SPEC> as RegisterValue<_>>::new(16191)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumch0_SPEC;
impl crate::sealed::RegSpec for Ctsumch0_SPEC {
    type DataType = u8;
}

pub type Ctsumch0 = crate::RegValueT<Ctsumch0_SPEC>;

impl NoBitfieldReg<Ctsumch0_SPEC> for Ctsumch0 {}
impl ::core::default::Default for Ctsumch0 {
    #[inline(always)]
    fn default() -> Ctsumch0 {
        <crate::RegValueT<Ctsumch0_SPEC> as RegisterValue<_>>::new(63)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumch1_SPEC;
impl crate::sealed::RegSpec for Ctsumch1_SPEC {
    type DataType = u8;
}

pub type Ctsumch1 = crate::RegValueT<Ctsumch1_SPEC>;

impl NoBitfieldReg<Ctsumch1_SPEC> for Ctsumch1 {}
impl ::core::default::Default for Ctsumch1 {
    #[inline(always)]
    fn default() -> Ctsumch1 {
        <crate::RegValueT<Ctsumch1_SPEC> as RegisterValue<_>>::new(63)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumchh_SPEC;
impl crate::sealed::RegSpec for Ctsumchh_SPEC {
    type DataType = u16;
}

pub type Ctsumchh = crate::RegValueT<Ctsumchh_SPEC>;

impl NoBitfieldReg<Ctsumchh_SPEC> for Ctsumchh {}
impl ::core::default::Default for Ctsumchh {
    #[inline(always)]
    fn default() -> Ctsumchh {
        <crate::RegValueT<Ctsumchh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumfaf_SPEC;
impl crate::sealed::RegSpec for Ctsumfaf_SPEC {
    type DataType = u8;
}

pub type Ctsumfaf = crate::RegValueT<Ctsumfaf_SPEC>;

impl NoBitfieldReg<Ctsumfaf_SPEC> for Ctsumfaf {}
impl ::core::default::Default for Ctsumfaf {
    #[inline(always)]
    fn default() -> Ctsumfaf {
        <crate::RegValueT<Ctsumfaf_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchaca_SPEC;
impl crate::sealed::RegSpec for Ctsuchaca_SPEC {
    type DataType = u32;
}

pub type Ctsuchaca = crate::RegValueT<Ctsuchaca_SPEC>;

impl Ctsuchaca {
    #[inline(always)]
    pub fn chac00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ctsuchaca::Chac00,
        ctsuchaca::Chac00,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ctsuchaca::Chac00,
            ctsuchaca::Chac00,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chac01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ctsuchaca::Chac01,
        ctsuchaca::Chac01,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ctsuchaca::Chac01,
            ctsuchaca::Chac01,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chac02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ctsuchaca::Chac02,
        ctsuchaca::Chac02,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ctsuchaca::Chac02,
            ctsuchaca::Chac02,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chac03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ctsuchaca::Chac03,
        ctsuchaca::Chac03,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ctsuchaca::Chac03,
            ctsuchaca::Chac03,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chac04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ctsuchaca::Chac04,
        ctsuchaca::Chac04,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ctsuchaca::Chac04,
            ctsuchaca::Chac04,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chac05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ctsuchaca::Chac05,
        ctsuchaca::Chac05,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ctsuchaca::Chac05,
            ctsuchaca::Chac05,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chac06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ctsuchaca::Chac06,
        ctsuchaca::Chac06,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ctsuchaca::Chac06,
            ctsuchaca::Chac06,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chac07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ctsuchaca::Chac07,
        ctsuchaca::Chac07,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ctsuchaca::Chac07,
            ctsuchaca::Chac07,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chac08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        ctsuchaca::Chac08,
        ctsuchaca::Chac08,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            ctsuchaca::Chac08,
            ctsuchaca::Chac08,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chac09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ctsuchaca::Chac09,
        ctsuchaca::Chac09,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ctsuchaca::Chac09,
            ctsuchaca::Chac09,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chac10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        ctsuchaca::Chac10,
        ctsuchaca::Chac10,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            ctsuchaca::Chac10,
            ctsuchaca::Chac10,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chac11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        ctsuchaca::Chac11,
        ctsuchaca::Chac11,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            ctsuchaca::Chac11,
            ctsuchaca::Chac11,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsuchaca {
    #[inline(always)]
    fn default() -> Ctsuchaca {
        <crate::RegValueT<Ctsuchaca_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctsuchaca {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac00_SPEC;
    pub type Chac00 = crate::EnumBitfieldStruct<u8, Chac00_SPEC>;
    impl Chac00 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac01_SPEC;
    pub type Chac01 = crate::EnumBitfieldStruct<u8, Chac01_SPEC>;
    impl Chac01 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac02_SPEC;
    pub type Chac02 = crate::EnumBitfieldStruct<u8, Chac02_SPEC>;
    impl Chac02 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac03_SPEC;
    pub type Chac03 = crate::EnumBitfieldStruct<u8, Chac03_SPEC>;
    impl Chac03 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac04_SPEC;
    pub type Chac04 = crate::EnumBitfieldStruct<u8, Chac04_SPEC>;
    impl Chac04 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac05_SPEC;
    pub type Chac05 = crate::EnumBitfieldStruct<u8, Chac05_SPEC>;
    impl Chac05 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac06_SPEC;
    pub type Chac06 = crate::EnumBitfieldStruct<u8, Chac06_SPEC>;
    impl Chac06 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac07_SPEC;
    pub type Chac07 = crate::EnumBitfieldStruct<u8, Chac07_SPEC>;
    impl Chac07 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac08_SPEC;
    pub type Chac08 = crate::EnumBitfieldStruct<u8, Chac08_SPEC>;
    impl Chac08 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac09_SPEC;
    pub type Chac09 = crate::EnumBitfieldStruct<u8, Chac09_SPEC>;
    impl Chac09 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac10_SPEC;
    pub type Chac10 = crate::EnumBitfieldStruct<u8, Chac10_SPEC>;
    impl Chac10 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac11_SPEC;
    pub type Chac11 = crate::EnumBitfieldStruct<u8, Chac11_SPEC>;
    impl Chac11 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchacal_SPEC;
impl crate::sealed::RegSpec for Ctsuchacal_SPEC {
    type DataType = u16;
}

pub type Ctsuchacal = crate::RegValueT<Ctsuchacal_SPEC>;

impl NoBitfieldReg<Ctsuchacal_SPEC> for Ctsuchacal {}
impl ::core::default::Default for Ctsuchacal {
    #[inline(always)]
    fn default() -> Ctsuchacal {
        <crate::RegValueT<Ctsuchacal_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchac0_SPEC;
impl crate::sealed::RegSpec for Ctsuchac0_SPEC {
    type DataType = u8;
}

pub type Ctsuchac0 = crate::RegValueT<Ctsuchac0_SPEC>;

impl NoBitfieldReg<Ctsuchac0_SPEC> for Ctsuchac0 {}
impl ::core::default::Default for Ctsuchac0 {
    #[inline(always)]
    fn default() -> Ctsuchac0 {
        <crate::RegValueT<Ctsuchac0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchac1_SPEC;
impl crate::sealed::RegSpec for Ctsuchac1_SPEC {
    type DataType = u8;
}

pub type Ctsuchac1 = crate::RegValueT<Ctsuchac1_SPEC>;

impl NoBitfieldReg<Ctsuchac1_SPEC> for Ctsuchac1 {}
impl ::core::default::Default for Ctsuchac1 {
    #[inline(always)]
    fn default() -> Ctsuchac1 {
        <crate::RegValueT<Ctsuchac1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchtrca_SPEC;
impl crate::sealed::RegSpec for Ctsuchtrca_SPEC {
    type DataType = u32;
}

pub type Ctsuchtrca = crate::RegValueT<Ctsuchtrca_SPEC>;

impl Ctsuchtrca {
    #[inline(always)]
    pub fn chtrc00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc00,
        ctsuchtrca::Chtrc00,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc00,
            ctsuchtrca::Chtrc00,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chtrc01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc01,
        ctsuchtrca::Chtrc01,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc01,
            ctsuchtrca::Chtrc01,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chtrc02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc02,
        ctsuchtrca::Chtrc02,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc02,
            ctsuchtrca::Chtrc02,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chtrc03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc03,
        ctsuchtrca::Chtrc03,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc03,
            ctsuchtrca::Chtrc03,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chtrc04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc04,
        ctsuchtrca::Chtrc04,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc04,
            ctsuchtrca::Chtrc04,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chtrc05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc05,
        ctsuchtrca::Chtrc05,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc05,
            ctsuchtrca::Chtrc05,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chtrc06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc06,
        ctsuchtrca::Chtrc06,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc06,
            ctsuchtrca::Chtrc06,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chtrc07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc07,
        ctsuchtrca::Chtrc07,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc07,
            ctsuchtrca::Chtrc07,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chtrc08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc08,
        ctsuchtrca::Chtrc08,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc08,
            ctsuchtrca::Chtrc08,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chtrc09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc09,
        ctsuchtrca::Chtrc09,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc09,
            ctsuchtrca::Chtrc09,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chtrc10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc10,
        ctsuchtrca::Chtrc10,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc10,
            ctsuchtrca::Chtrc10,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chtrc11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc11,
        ctsuchtrca::Chtrc11,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc11,
            ctsuchtrca::Chtrc11,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsuchtrca {
    #[inline(always)]
    fn default() -> Ctsuchtrca {
        <crate::RegValueT<Ctsuchtrca_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctsuchtrca {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc00_SPEC;
    pub type Chtrc00 = crate::EnumBitfieldStruct<u8, Chtrc00_SPEC>;
    impl Chtrc00 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc01_SPEC;
    pub type Chtrc01 = crate::EnumBitfieldStruct<u8, Chtrc01_SPEC>;
    impl Chtrc01 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc02_SPEC;
    pub type Chtrc02 = crate::EnumBitfieldStruct<u8, Chtrc02_SPEC>;
    impl Chtrc02 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc03_SPEC;
    pub type Chtrc03 = crate::EnumBitfieldStruct<u8, Chtrc03_SPEC>;
    impl Chtrc03 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc04_SPEC;
    pub type Chtrc04 = crate::EnumBitfieldStruct<u8, Chtrc04_SPEC>;
    impl Chtrc04 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc05_SPEC;
    pub type Chtrc05 = crate::EnumBitfieldStruct<u8, Chtrc05_SPEC>;
    impl Chtrc05 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc06_SPEC;
    pub type Chtrc06 = crate::EnumBitfieldStruct<u8, Chtrc06_SPEC>;
    impl Chtrc06 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc07_SPEC;
    pub type Chtrc07 = crate::EnumBitfieldStruct<u8, Chtrc07_SPEC>;
    impl Chtrc07 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc08_SPEC;
    pub type Chtrc08 = crate::EnumBitfieldStruct<u8, Chtrc08_SPEC>;
    impl Chtrc08 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc09_SPEC;
    pub type Chtrc09 = crate::EnumBitfieldStruct<u8, Chtrc09_SPEC>;
    impl Chtrc09 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc10_SPEC;
    pub type Chtrc10 = crate::EnumBitfieldStruct<u8, Chtrc10_SPEC>;
    impl Chtrc10 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc11_SPEC;
    pub type Chtrc11 = crate::EnumBitfieldStruct<u8, Chtrc11_SPEC>;
    impl Chtrc11 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchtrcal_SPEC;
impl crate::sealed::RegSpec for Ctsuchtrcal_SPEC {
    type DataType = u16;
}

pub type Ctsuchtrcal = crate::RegValueT<Ctsuchtrcal_SPEC>;

impl NoBitfieldReg<Ctsuchtrcal_SPEC> for Ctsuchtrcal {}
impl ::core::default::Default for Ctsuchtrcal {
    #[inline(always)]
    fn default() -> Ctsuchtrcal {
        <crate::RegValueT<Ctsuchtrcal_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchtrc0_SPEC;
impl crate::sealed::RegSpec for Ctsuchtrc0_SPEC {
    type DataType = u8;
}

pub type Ctsuchtrc0 = crate::RegValueT<Ctsuchtrc0_SPEC>;

impl NoBitfieldReg<Ctsuchtrc0_SPEC> for Ctsuchtrc0 {}
impl ::core::default::Default for Ctsuchtrc0 {
    #[inline(always)]
    fn default() -> Ctsuchtrc0 {
        <crate::RegValueT<Ctsuchtrc0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchtrc1_SPEC;
impl crate::sealed::RegSpec for Ctsuchtrc1_SPEC {
    type DataType = u8;
}

pub type Ctsuchtrc1 = crate::RegValueT<Ctsuchtrc1_SPEC>;

impl NoBitfieldReg<Ctsuchtrc1_SPEC> for Ctsuchtrc1 {}
impl ::core::default::Default for Ctsuchtrc1 {
    #[inline(always)]
    fn default() -> Ctsuchtrc1 {
        <crate::RegValueT<Ctsuchtrc1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusr_SPEC;
impl crate::sealed::RegSpec for Ctsusr_SPEC {
    type DataType = u32;
}

pub type Ctsusr = crate::RegValueT<Ctsusr_SPEC>;

impl Ctsusr {
    #[inline(always)]
    pub fn mfc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        ctsusr::Mfc,
        ctsusr::Mfc,
        Ctsusr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            ctsusr::Mfc,
            ctsusr::Mfc,
            Ctsusr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn icomprst(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Ctsusr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ctsusr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn icomp1(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ctsusr::Icomp1,
        ctsusr::Icomp1,
        Ctsusr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ctsusr::Icomp1,
            ctsusr::Icomp1,
            Ctsusr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn icomp0(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ctsusr::Icomp0,
        ctsusr::Icomp0,
        Ctsusr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ctsusr::Icomp0,
            ctsusr::Icomp0,
            Ctsusr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn stc(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        ctsusr::Stc,
        ctsusr::Stc,
        Ctsusr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            ctsusr::Stc,
            ctsusr::Stc,
            Ctsusr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dtsr(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        ctsusr::Dtsr,
        ctsusr::Dtsr,
        Ctsusr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            ctsusr::Dtsr,
            ctsusr::Dtsr,
            Ctsusr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sensovf(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        ctsusr::Sensovf,
        ctsusr::Sensovf,
        Ctsusr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            ctsusr::Sensovf,
            ctsusr::Sensovf,
            Ctsusr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn suckovf(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        ctsusr::Suckovf,
        ctsusr::Suckovf,
        Ctsusr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            ctsusr::Suckovf,
            ctsusr::Suckovf,
            Ctsusr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ps(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        ctsusr::Ps,
        ctsusr::Ps,
        Ctsusr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            ctsusr::Ps,
            ctsusr::Ps,
            Ctsusr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsusr {
    #[inline(always)]
    fn default() -> Ctsusr {
        <crate::RegValueT<Ctsusr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctsusr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mfc_SPEC;
    pub type Mfc = crate::EnumBitfieldStruct<u8, Mfc_SPEC>;
    impl Mfc {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icomp1_SPEC;
    pub type Icomp1 = crate::EnumBitfieldStruct<u8, Icomp1_SPEC>;
    impl Icomp1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icomp0_SPEC;
    pub type Icomp0 = crate::EnumBitfieldStruct<u8, Icomp0_SPEC>;
    impl Icomp0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stc_SPEC;
    pub type Stc = crate::EnumBitfieldStruct<u8, Stc_SPEC>;
    impl Stc {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _101: Self = Self::new(5);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtsr_SPEC;
    pub type Dtsr = crate::EnumBitfieldStruct<u8, Dtsr_SPEC>;
    impl Dtsr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sensovf_SPEC;
    pub type Sensovf = crate::EnumBitfieldStruct<u8, Sensovf_SPEC>;
    impl Sensovf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Suckovf_SPEC;
    pub type Suckovf = crate::EnumBitfieldStruct<u8, Suckovf_SPEC>;
    impl Suckovf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ps_SPEC;
    pub type Ps = crate::EnumBitfieldStruct<u8, Ps_SPEC>;
    impl Ps {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusrl_SPEC;
impl crate::sealed::RegSpec for Ctsusrl_SPEC {
    type DataType = u16;
}

pub type Ctsusrl = crate::RegValueT<Ctsusrl_SPEC>;

impl NoBitfieldReg<Ctsusrl_SPEC> for Ctsusrl {}
impl ::core::default::Default for Ctsusrl {
    #[inline(always)]
    fn default() -> Ctsusrl {
        <crate::RegValueT<Ctsusrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusr0_SPEC;
impl crate::sealed::RegSpec for Ctsusr0_SPEC {
    type DataType = u8;
}

pub type Ctsusr0 = crate::RegValueT<Ctsusr0_SPEC>;

impl NoBitfieldReg<Ctsusr0_SPEC> for Ctsusr0 {}
impl ::core::default::Default for Ctsusr0 {
    #[inline(always)]
    fn default() -> Ctsusr0 {
        <crate::RegValueT<Ctsusr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsust_SPEC;
impl crate::sealed::RegSpec for Ctsust_SPEC {
    type DataType = u8;
}

pub type Ctsust = crate::RegValueT<Ctsust_SPEC>;

impl NoBitfieldReg<Ctsust_SPEC> for Ctsust {}
impl ::core::default::Default for Ctsust {
    #[inline(always)]
    fn default() -> Ctsust {
        <crate::RegValueT<Ctsust_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusrh_SPEC;
impl crate::sealed::RegSpec for Ctsusrh_SPEC {
    type DataType = u16;
}

pub type Ctsusrh = crate::RegValueT<Ctsusrh_SPEC>;

impl NoBitfieldReg<Ctsusrh_SPEC> for Ctsusrh {}
impl ::core::default::Default for Ctsusrh {
    #[inline(always)]
    fn default() -> Ctsusrh {
        <crate::RegValueT<Ctsusrh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusr2_SPEC;
impl crate::sealed::RegSpec for Ctsusr2_SPEC {
    type DataType = u8;
}

pub type Ctsusr2 = crate::RegValueT<Ctsusr2_SPEC>;

impl NoBitfieldReg<Ctsusr2_SPEC> for Ctsusr2 {}
impl ::core::default::Default for Ctsusr2 {
    #[inline(always)]
    fn default() -> Ctsusr2 {
        <crate::RegValueT<Ctsusr2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuso_SPEC;
impl crate::sealed::RegSpec for Ctsuso_SPEC {
    type DataType = u32;
}

pub type Ctsuso = crate::RegValueT<Ctsuso_SPEC>;

impl Ctsuso {
    #[inline(always)]
    pub fn so(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Ctsuso_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Ctsuso_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn snum(
        self,
    ) -> crate::common::RegisterField<10, 0xff, 1, 0, u8, u8, Ctsuso_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0xff,1,0,u8,u8,Ctsuso_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ssdiv(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Ctsuso_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8,u8,Ctsuso_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sdpa(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Ctsuso_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Ctsuso_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuso {
    #[inline(always)]
    fn default() -> Ctsuso {
        <crate::RegValueT<Ctsuso_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuso0_SPEC;
impl crate::sealed::RegSpec for Ctsuso0_SPEC {
    type DataType = u16;
}

pub type Ctsuso0 = crate::RegValueT<Ctsuso0_SPEC>;

impl NoBitfieldReg<Ctsuso0_SPEC> for Ctsuso0 {}
impl ::core::default::Default for Ctsuso0 {
    #[inline(always)]
    fn default() -> Ctsuso0 {
        <crate::RegValueT<Ctsuso0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuso1_SPEC;
impl crate::sealed::RegSpec for Ctsuso1_SPEC {
    type DataType = u16;
}

pub type Ctsuso1 = crate::RegValueT<Ctsuso1_SPEC>;

impl NoBitfieldReg<Ctsuso1_SPEC> for Ctsuso1 {}
impl ::core::default::Default for Ctsuso1 {
    #[inline(always)]
    fn default() -> Ctsuso1 {
        <crate::RegValueT<Ctsuso1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuscnt_SPEC;
impl crate::sealed::RegSpec for Ctsuscnt_SPEC {
    type DataType = u32;
}

pub type Ctsuscnt = crate::RegValueT<Ctsuscnt_SPEC>;

impl Ctsuscnt {
    #[inline(always)]
    pub fn senscnt(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsuscnt_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Ctsuscnt_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn suckcnt(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Ctsuscnt_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Ctsuscnt_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuscnt {
    #[inline(always)]
    fn default() -> Ctsuscnt {
        <crate::RegValueT<Ctsuscnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusc_SPEC;
impl crate::sealed::RegSpec for Ctsusc_SPEC {
    type DataType = u16;
}

pub type Ctsusc = crate::RegValueT<Ctsusc_SPEC>;

impl NoBitfieldReg<Ctsusc_SPEC> for Ctsusc {}
impl ::core::default::Default for Ctsusc {
    #[inline(always)]
    fn default() -> Ctsusc {
        <crate::RegValueT<Ctsusc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsucalib_SPEC;
impl crate::sealed::RegSpec for Ctsucalib_SPEC {
    type DataType = u32;
}

pub type Ctsucalib = crate::RegValueT<Ctsucalib_SPEC>;

impl Ctsucalib {
    #[inline(always)]
    pub fn tsod(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ctsucalib::Tsod,
        ctsucalib::Tsod,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ctsucalib::Tsod,
            ctsucalib::Tsod,
            Ctsucalib_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn drv(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ctsucalib::Drv,
        ctsucalib::Drv,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ctsucalib::Drv,
            ctsucalib::Drv,
            Ctsucalib_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn clksel(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        ctsucalib::Clksel,
        ctsucalib::Clksel,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            ctsucalib::Clksel,
            ctsucalib::Clksel,
            Ctsucalib_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn suclken(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ctsucalib::Suclken,
        ctsucalib::Suclken,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ctsucalib::Suclken,
            ctsucalib::Suclken,
            Ctsucalib_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tsoc(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ctsucalib::Tsoc,
        ctsucalib::Tsoc,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ctsucalib::Tsoc,
            ctsucalib::Tsoc,
            Ctsucalib_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cntrdsel(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        ctsucalib::Cntrdsel,
        ctsucalib::Cntrdsel,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            ctsucalib::Cntrdsel,
            ctsucalib::Cntrdsel,
            Ctsucalib_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ioc(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Ctsucalib_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,Ctsucalib_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcoff(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        ctsucalib::Dcoff,
        ctsucalib::Dcoff,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            ctsucalib::Dcoff,
            ctsucalib::Dcoff,
            Ctsucalib_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn iocsel(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        ctsucalib::Iocsel,
        ctsucalib::Iocsel,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            ctsucalib::Iocsel,
            ctsucalib::Iocsel,
            Ctsucalib_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dacmsel(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        ctsucalib::Dacmsel,
        ctsucalib::Dacmsel,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            ctsucalib::Dacmsel,
            ctsucalib::Dacmsel,
            Ctsucalib_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn daccarry(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        ctsucalib::Daccarry,
        ctsucalib::Daccarry,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            ctsucalib::Daccarry,
            ctsucalib::Daccarry,
            Ctsucalib_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sumsel(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        ctsucalib::Sumsel,
        ctsucalib::Sumsel,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            ctsucalib::Sumsel,
            ctsucalib::Sumsel,
            Ctsucalib_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sucarry(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        ctsucalib::Sucarry,
        ctsucalib::Sucarry,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            ctsucalib::Sucarry,
            ctsucalib::Sucarry,
            Ctsucalib_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dacclk(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        ctsucalib::Dacclk,
        ctsucalib::Dacclk,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            ctsucalib::Dacclk,
            ctsucalib::Dacclk,
            Ctsucalib_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ccoclk(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        ctsucalib::Ccoclk,
        ctsucalib::Ccoclk,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            ctsucalib::Ccoclk,
            ctsucalib::Ccoclk,
            Ctsucalib_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ccocalib(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        ctsucalib::Ccocalib,
        ctsucalib::Ccocalib,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            ctsucalib::Ccocalib,
            ctsucalib::Ccocalib,
            Ctsucalib_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txrev(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        ctsucalib::Txrev,
        ctsucalib::Txrev,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            ctsucalib::Txrev,
            ctsucalib::Txrev,
            Ctsucalib_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsucalib {
    #[inline(always)]
    fn default() -> Ctsucalib {
        <crate::RegValueT<Ctsucalib_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctsucalib {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsod_SPEC;
    pub type Tsod = crate::EnumBitfieldStruct<u8, Tsod_SPEC>;
    impl Tsod {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Drv_SPEC;
    pub type Drv = crate::EnumBitfieldStruct<u8, Drv_SPEC>;
    impl Drv {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clksel_SPEC;
    pub type Clksel = crate::EnumBitfieldStruct<u8, Clksel_SPEC>;
    impl Clksel {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Suclken_SPEC;
    pub type Suclken = crate::EnumBitfieldStruct<u8, Suclken_SPEC>;
    impl Suclken {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsoc_SPEC;
    pub type Tsoc = crate::EnumBitfieldStruct<u8, Tsoc_SPEC>;
    impl Tsoc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cntrdsel_SPEC;
    pub type Cntrdsel = crate::EnumBitfieldStruct<u8, Cntrdsel_SPEC>;
    impl Cntrdsel {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcoff_SPEC;
    pub type Dcoff = crate::EnumBitfieldStruct<u8, Dcoff_SPEC>;
    impl Dcoff {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iocsel_SPEC;
    pub type Iocsel = crate::EnumBitfieldStruct<u8, Iocsel_SPEC>;
    impl Iocsel {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dacmsel_SPEC;
    pub type Dacmsel = crate::EnumBitfieldStruct<u8, Dacmsel_SPEC>;
    impl Dacmsel {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Daccarry_SPEC;
    pub type Daccarry = crate::EnumBitfieldStruct<u8, Daccarry_SPEC>;
    impl Daccarry {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sumsel_SPEC;
    pub type Sumsel = crate::EnumBitfieldStruct<u8, Sumsel_SPEC>;
    impl Sumsel {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sucarry_SPEC;
    pub type Sucarry = crate::EnumBitfieldStruct<u8, Sucarry_SPEC>;
    impl Sucarry {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dacclk_SPEC;
    pub type Dacclk = crate::EnumBitfieldStruct<u8, Dacclk_SPEC>;
    impl Dacclk {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccoclk_SPEC;
    pub type Ccoclk = crate::EnumBitfieldStruct<u8, Ccoclk_SPEC>;
    impl Ccoclk {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccocalib_SPEC;
    pub type Ccocalib = crate::EnumBitfieldStruct<u8, Ccocalib_SPEC>;
    impl Ccocalib {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txrev_SPEC;
    pub type Txrev = crate::EnumBitfieldStruct<u8, Txrev_SPEC>;
    impl Txrev {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsudbgr0_SPEC;
impl crate::sealed::RegSpec for Ctsudbgr0_SPEC {
    type DataType = u16;
}

pub type Ctsudbgr0 = crate::RegValueT<Ctsudbgr0_SPEC>;

impl NoBitfieldReg<Ctsudbgr0_SPEC> for Ctsudbgr0 {}
impl ::core::default::Default for Ctsudbgr0 {
    #[inline(always)]
    fn default() -> Ctsudbgr0 {
        <crate::RegValueT<Ctsudbgr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsudbgr1_SPEC;
impl crate::sealed::RegSpec for Ctsudbgr1_SPEC {
    type DataType = u16;
}

pub type Ctsudbgr1 = crate::RegValueT<Ctsudbgr1_SPEC>;

impl NoBitfieldReg<Ctsudbgr1_SPEC> for Ctsudbgr1 {}
impl ::core::default::Default for Ctsudbgr1 {
    #[inline(always)]
    fn default() -> Ctsudbgr1 {
        <crate::RegValueT<Ctsudbgr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusuclka_SPEC;
impl crate::sealed::RegSpec for Ctsusuclka_SPEC {
    type DataType = u32;
}

pub type Ctsusuclka = crate::RegValueT<Ctsusuclka_SPEC>;

impl Ctsusuclka {
    #[inline(always)]
    pub fn suadj0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ctsusuclka_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ctsusuclka_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sumulti0(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ctsusuclka_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ctsusuclka_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn suadj1(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Ctsusuclka_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Ctsusuclka_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sumulti1(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Ctsusuclka_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Ctsusuclka_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsusuclka {
    #[inline(always)]
    fn default() -> Ctsusuclka {
        <crate::RegValueT<Ctsusuclka_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusuclk0_SPEC;
impl crate::sealed::RegSpec for Ctsusuclk0_SPEC {
    type DataType = u16;
}

pub type Ctsusuclk0 = crate::RegValueT<Ctsusuclk0_SPEC>;

impl NoBitfieldReg<Ctsusuclk0_SPEC> for Ctsusuclk0 {}
impl ::core::default::Default for Ctsusuclk0 {
    #[inline(always)]
    fn default() -> Ctsusuclk0 {
        <crate::RegValueT<Ctsusuclk0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusuclk1_SPEC;
impl crate::sealed::RegSpec for Ctsusuclk1_SPEC {
    type DataType = u16;
}

pub type Ctsusuclk1 = crate::RegValueT<Ctsusuclk1_SPEC>;

impl NoBitfieldReg<Ctsusuclk1_SPEC> for Ctsusuclk1 {}
impl ::core::default::Default for Ctsusuclk1 {
    #[inline(always)]
    fn default() -> Ctsusuclk1 {
        <crate::RegValueT<Ctsusuclk1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusuclkb_SPEC;
impl crate::sealed::RegSpec for Ctsusuclkb_SPEC {
    type DataType = u32;
}

pub type Ctsusuclkb = crate::RegValueT<Ctsusuclkb_SPEC>;

impl Ctsusuclkb {
    #[inline(always)]
    pub fn suadj2(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ctsusuclkb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ctsusuclkb_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sumulti2(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ctsusuclkb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ctsusuclkb_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn suadj3(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Ctsusuclkb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Ctsusuclkb_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sumulti3(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Ctsusuclkb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Ctsusuclkb_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsusuclkb {
    #[inline(always)]
    fn default() -> Ctsusuclkb {
        <crate::RegValueT<Ctsusuclkb_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusuclk2_SPEC;
impl crate::sealed::RegSpec for Ctsusuclk2_SPEC {
    type DataType = u16;
}

pub type Ctsusuclk2 = crate::RegValueT<Ctsusuclk2_SPEC>;

impl NoBitfieldReg<Ctsusuclk2_SPEC> for Ctsusuclk2 {}
impl ::core::default::Default for Ctsusuclk2 {
    #[inline(always)]
    fn default() -> Ctsusuclk2 {
        <crate::RegValueT<Ctsusuclk2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsusuclk3_SPEC;
impl crate::sealed::RegSpec for Ctsusuclk3_SPEC {
    type DataType = u16;
}

pub type Ctsusuclk3 = crate::RegValueT<Ctsusuclk3_SPEC>;

impl NoBitfieldReg<Ctsusuclk3_SPEC> for Ctsusuclk3 {}
impl ::core::default::Default for Ctsusuclk3 {
    #[inline(always)]
    fn default() -> Ctsusuclk3 {
        <crate::RegValueT<Ctsusuclk3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuopt_SPEC;
impl crate::sealed::RegSpec for Ctsuopt_SPEC {
    type DataType = u32;
}

pub type Ctsuopt = crate::RegValueT<Ctsuopt_SPEC>;

impl Ctsuopt {
    #[inline(always)]
    pub fn ccocfen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ctsuopt::Ccocfen,
        ctsuopt::Ccocfen,
        Ctsuopt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ctsuopt::Ccocfen,
            ctsuopt::Ccocfen,
            Ctsuopt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mcacefn(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ctsuopt::Mcacefn,
        ctsuopt::Mcacefn,
        Ctsuopt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ctsuopt::Mcacefn,
            ctsuopt::Mcacefn,
            Ctsuopt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn majirimd(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ctsuopt::Majirimd,
        ctsuopt::Majirimd,
        Ctsuopt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ctsuopt::Majirimd,
            ctsuopt::Majirimd,
            Ctsuopt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dtcless(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ctsuopt::Dtcless,
        ctsuopt::Dtcless,
        Ctsuopt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ctsuopt::Dtcless,
            ctsuopt::Dtcless,
            Ctsuopt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mtucfen(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ctsuopt::Mtucfen,
        ctsuopt::Mtucfen,
        Ctsuopt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ctsuopt::Mtucfen,
            ctsuopt::Mtucfen,
            Ctsuopt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ajfen(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        ctsuopt::Ajfen,
        ctsuopt::Ajfen,
        Ctsuopt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            ctsuopt::Ajfen,
            ctsuopt::Ajfen,
            Ctsuopt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ajintc(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ctsuopt::Ajintc,
        ctsuopt::Ajintc,
        Ctsuopt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ctsuopt::Ajintc,
            ctsuopt::Ajintc,
            Ctsuopt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn scactb(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Ctsuopt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Ctsuopt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuopt {
    #[inline(always)]
    fn default() -> Ctsuopt {
        <crate::RegValueT<Ctsuopt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctsuopt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccocfen_SPEC;
    pub type Ccocfen = crate::EnumBitfieldStruct<u8, Ccocfen_SPEC>;
    impl Ccocfen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mcacefn_SPEC;
    pub type Mcacefn = crate::EnumBitfieldStruct<u8, Mcacefn_SPEC>;
    impl Mcacefn {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Majirimd_SPEC;
    pub type Majirimd = crate::EnumBitfieldStruct<u8, Majirimd_SPEC>;
    impl Majirimd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtcless_SPEC;
    pub type Dtcless = crate::EnumBitfieldStruct<u8, Dtcless_SPEC>;
    impl Dtcless {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mtucfen_SPEC;
    pub type Mtucfen = crate::EnumBitfieldStruct<u8, Mtucfen_SPEC>;
    impl Mtucfen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ajfen_SPEC;
    pub type Ajfen = crate::EnumBitfieldStruct<u8, Ajfen_SPEC>;
    impl Ajfen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ajintc_SPEC;
    pub type Ajintc = crate::EnumBitfieldStruct<u8, Ajintc_SPEC>;
    impl Ajintc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuoptl_SPEC;
impl crate::sealed::RegSpec for Ctsuoptl_SPEC {
    type DataType = u16;
}

pub type Ctsuoptl = crate::RegValueT<Ctsuoptl_SPEC>;

impl Ctsuoptl {
    #[inline(always)]
    pub fn ccocfen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ctsuoptl::Ccocfen,
        ctsuoptl::Ccocfen,
        Ctsuoptl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ctsuoptl::Ccocfen,
            ctsuoptl::Ccocfen,
            Ctsuoptl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mcacefn(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ctsuoptl::Mcacefn,
        ctsuoptl::Mcacefn,
        Ctsuoptl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ctsuoptl::Mcacefn,
            ctsuoptl::Mcacefn,
            Ctsuoptl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn majirimd(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ctsuoptl::Majirimd,
        ctsuoptl::Majirimd,
        Ctsuoptl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ctsuoptl::Majirimd,
            ctsuoptl::Majirimd,
            Ctsuoptl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dtcless(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ctsuoptl::Dtcless,
        ctsuoptl::Dtcless,
        Ctsuoptl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ctsuoptl::Dtcless,
            ctsuoptl::Dtcless,
            Ctsuoptl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mtucfen(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ctsuoptl::Mtucfen,
        ctsuoptl::Mtucfen,
        Ctsuoptl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ctsuoptl::Mtucfen,
            ctsuoptl::Mtucfen,
            Ctsuoptl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ajfen(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        ctsuoptl::Ajfen,
        ctsuoptl::Ajfen,
        Ctsuoptl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            ctsuoptl::Ajfen,
            ctsuoptl::Ajfen,
            Ctsuoptl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ajintc(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ctsuoptl::Ajintc,
        ctsuoptl::Ajintc,
        Ctsuoptl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ctsuoptl::Ajintc,
            ctsuoptl::Ajintc,
            Ctsuoptl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsuoptl {
    #[inline(always)]
    fn default() -> Ctsuoptl {
        <crate::RegValueT<Ctsuoptl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctsuoptl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccocfen_SPEC;
    pub type Ccocfen = crate::EnumBitfieldStruct<u8, Ccocfen_SPEC>;
    impl Ccocfen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mcacefn_SPEC;
    pub type Mcacefn = crate::EnumBitfieldStruct<u8, Mcacefn_SPEC>;
    impl Mcacefn {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Majirimd_SPEC;
    pub type Majirimd = crate::EnumBitfieldStruct<u8, Majirimd_SPEC>;
    impl Majirimd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtcless_SPEC;
    pub type Dtcless = crate::EnumBitfieldStruct<u8, Dtcless_SPEC>;
    impl Dtcless {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mtucfen_SPEC;
    pub type Mtucfen = crate::EnumBitfieldStruct<u8, Mtucfen_SPEC>;
    impl Mtucfen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ajfen_SPEC;
    pub type Ajfen = crate::EnumBitfieldStruct<u8, Ajfen_SPEC>;
    impl Ajfen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ajintc_SPEC;
    pub type Ajintc = crate::EnumBitfieldStruct<u8, Ajintc_SPEC>;
    impl Ajintc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ac_SPEC;
impl crate::sealed::RegSpec for Ac_SPEC {
    type DataType = u8;
}

pub type Ac = crate::RegValueT<Ac_SPEC>;

impl Ac {
    #[inline(always)]
    pub fn ccocfen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ac::Ccocfen,
        ac::Ccocfen,
        Ac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ac::Ccocfen,
            ac::Ccocfen,
            Ac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mcacefn(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ac::Mcacefn,
        ac::Mcacefn,
        Ac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ac::Mcacefn,
            ac::Mcacefn,
            Ac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn majirimd(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ac::Majirimd,
        ac::Majirimd,
        Ac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ac::Majirimd,
            ac::Majirimd,
            Ac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dtcless(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ac::Dtcless,
        ac::Dtcless,
        Ac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ac::Dtcless,
            ac::Dtcless,
            Ac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mtucfen(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ac::Mtucfen,
        ac::Mtucfen,
        Ac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ac::Mtucfen,
            ac::Mtucfen,
            Ac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ac {
    #[inline(always)]
    fn default() -> Ac {
        <crate::RegValueT<Ac_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ac {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccocfen_SPEC;
    pub type Ccocfen = crate::EnumBitfieldStruct<u8, Ccocfen_SPEC>;
    impl Ccocfen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mcacefn_SPEC;
    pub type Mcacefn = crate::EnumBitfieldStruct<u8, Mcacefn_SPEC>;
    impl Mcacefn {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Majirimd_SPEC;
    pub type Majirimd = crate::EnumBitfieldStruct<u8, Majirimd_SPEC>;
    impl Majirimd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtcless_SPEC;
    pub type Dtcless = crate::EnumBitfieldStruct<u8, Dtcless_SPEC>;
    impl Dtcless {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mtucfen_SPEC;
    pub type Mtucfen = crate::EnumBitfieldStruct<u8, Mtucfen_SPEC>;
    impl Mtucfen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aj_SPEC;
impl crate::sealed::RegSpec for Aj_SPEC {
    type DataType = u8;
}

pub type Aj = crate::RegValueT<Aj_SPEC>;

impl Aj {
    #[inline(always)]
    pub fn ajfen(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, aj::Ajfen, aj::Ajfen, Aj_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,aj::Ajfen,aj::Ajfen,Aj_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ajintc(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        aj::Ajintc,
        aj::Ajintc,
        Aj_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            aj::Ajintc,
            aj::Ajintc,
            Aj_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Aj {
    #[inline(always)]
    fn default() -> Aj {
        <crate::RegValueT<Aj_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod aj {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ajfen_SPEC;
    pub type Ajfen = crate::EnumBitfieldStruct<u8, Ajfen_SPEC>;
    impl Ajfen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ajintc_SPEC;
    pub type Ajintc = crate::EnumBitfieldStruct<u8, Ajintc_SPEC>;
    impl Ajintc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuopth_SPEC;
impl crate::sealed::RegSpec for Ctsuopth_SPEC {
    type DataType = u16;
}

pub type Ctsuopth = crate::RegValueT<Ctsuopth_SPEC>;

impl Ctsuopth {
    #[inline(always)]
    pub fn scactb(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Ctsuopth_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Ctsuopth_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuopth {
    #[inline(always)]
    fn default() -> Ctsuopth {
        <crate::RegValueT<Ctsuopth_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Acsb_SPEC;
impl crate::sealed::RegSpec for Acsb_SPEC {
    type DataType = u8;
}

pub type Acsb = crate::RegValueT<Acsb_SPEC>;

impl Acsb {
    #[inline(always)]
    pub fn scactb(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Acsb_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Acsb_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Acsb {
    #[inline(always)]
    fn default() -> Acsb {
        <crate::RegValueT<Acsb_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuscntact_SPEC;
impl crate::sealed::RegSpec for Ctsuscntact_SPEC {
    type DataType = u32;
}

pub type Ctsuscntact = crate::RegValueT<Ctsuscntact_SPEC>;

impl Ctsuscntact {
    #[inline(always)]
    pub fn scntaccoeff(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsuscntact_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Ctsuscntact_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn scntaccount(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Ctsuscntact_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            Ctsuscntact_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsuscntact {
    #[inline(always)]
    fn default() -> Ctsuscntact {
        <crate::RegValueT<Ctsuscntact_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuscntactl_SPEC;
impl crate::sealed::RegSpec for Ctsuscntactl_SPEC {
    type DataType = u16;
}

pub type Ctsuscntactl = crate::RegValueT<Ctsuscntactl_SPEC>;

impl Ctsuscntactl {
    #[inline(always)]
    pub fn scntaccoeff(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsuscntactl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Ctsuscntactl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsuscntactl {
    #[inline(always)]
    fn default() -> Ctsuscntactl {
        <crate::RegValueT<Ctsuscntactl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuscntacth_SPEC;
impl crate::sealed::RegSpec for Ctsuscntacth_SPEC {
    type DataType = u16;
}

pub type Ctsuscntacth = crate::RegValueT<Ctsuscntacth_SPEC>;

impl Ctsuscntacth {
    #[inline(always)]
    pub fn scntaccount(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsuscntacth_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Ctsuscntacth_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsuscntacth {
    #[inline(always)]
    fn default() -> Ctsuscntacth {
        <crate::RegValueT<Ctsuscntacth_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumact1_SPEC;
impl crate::sealed::RegSpec for Ctsumact1_SPEC {
    type DataType = u32;
}

pub type Ctsumact1 = crate::RegValueT<Ctsumact1_SPEC>;

impl Ctsumact1 {
    #[inline(always)]
    pub fn so(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Ctsumact1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Ctsumact1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn offsetcoeff(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Ctsumact1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Ctsumact1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsumact1 {
    #[inline(always)]
    fn default() -> Ctsumact1 {
        <crate::RegValueT<Ctsumact1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumact1L_SPEC;
impl crate::sealed::RegSpec for Ctsumact1L_SPEC {
    type DataType = u16;
}

pub type Ctsumact1L = crate::RegValueT<Ctsumact1L_SPEC>;

impl Ctsumact1L {
    #[inline(always)]
    pub fn so(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Ctsumact1L_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Ctsumact1L_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsumact1L {
    #[inline(always)]
    fn default() -> Ctsumact1L {
        <crate::RegValueT<Ctsumact1L_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumact1H_SPEC;
impl crate::sealed::RegSpec for Ctsumact1H_SPEC {
    type DataType = u16;
}

pub type Ctsumact1H = crate::RegValueT<Ctsumact1H_SPEC>;

impl Ctsumact1H {
    #[inline(always)]
    pub fn offsetcoeff(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsumact1H_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Ctsumact1H_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsumact1H {
    #[inline(always)]
    fn default() -> Ctsumact1H {
        <crate::RegValueT<Ctsumact1H_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumact2_SPEC;
impl crate::sealed::RegSpec for Ctsumact2_SPEC {
    type DataType = u32;
}

pub type Ctsumact2 = crate::RegValueT<Ctsumact2_SPEC>;

impl Ctsumact2 {
    #[inline(always)]
    pub fn so(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Ctsumact2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Ctsumact2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn offsetcoeff(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Ctsumact2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Ctsumact2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsumact2 {
    #[inline(always)]
    fn default() -> Ctsumact2 {
        <crate::RegValueT<Ctsumact2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumact2L_SPEC;
impl crate::sealed::RegSpec for Ctsumact2L_SPEC {
    type DataType = u16;
}

pub type Ctsumact2L = crate::RegValueT<Ctsumact2L_SPEC>;

impl Ctsumact2L {
    #[inline(always)]
    pub fn so(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Ctsumact2L_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Ctsumact2L_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsumact2L {
    #[inline(always)]
    fn default() -> Ctsumact2L {
        <crate::RegValueT<Ctsumact2L_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumact2H_SPEC;
impl crate::sealed::RegSpec for Ctsumact2H_SPEC {
    type DataType = u16;
}

pub type Ctsumact2H = crate::RegValueT<Ctsumact2H_SPEC>;

impl Ctsumact2H {
    #[inline(always)]
    pub fn offsetcoeff(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsumact2H_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Ctsumact2H_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsumact2H {
    #[inline(always)]
    fn default() -> Ctsumact2H {
        <crate::RegValueT<Ctsumact2H_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumact3_SPEC;
impl crate::sealed::RegSpec for Ctsumact3_SPEC {
    type DataType = u32;
}

pub type Ctsumact3 = crate::RegValueT<Ctsumact3_SPEC>;

impl Ctsumact3 {
    #[inline(always)]
    pub fn so(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Ctsumact3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Ctsumact3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn offsetcoeff(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Ctsumact3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Ctsumact3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsumact3 {
    #[inline(always)]
    fn default() -> Ctsumact3 {
        <crate::RegValueT<Ctsumact3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumact3L_SPEC;
impl crate::sealed::RegSpec for Ctsumact3L_SPEC {
    type DataType = u16;
}

pub type Ctsumact3L = crate::RegValueT<Ctsumact3L_SPEC>;

impl Ctsumact3L {
    #[inline(always)]
    pub fn so(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Ctsumact3L_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Ctsumact3L_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsumact3L {
    #[inline(always)]
    fn default() -> Ctsumact3L {
        <crate::RegValueT<Ctsumact3L_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsumact3H_SPEC;
impl crate::sealed::RegSpec for Ctsumact3H_SPEC {
    type DataType = u16;
}

pub type Ctsumact3H = crate::RegValueT<Ctsumact3H_SPEC>;

impl Ctsumact3H {
    #[inline(always)]
    pub fn offsetcoeff(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsumact3H_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Ctsumact3H_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsumact3H {
    #[inline(always)]
    fn default() -> Ctsumact3H {
        <crate::RegValueT<Ctsumact3H_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajcr_SPEC;
impl crate::sealed::RegSpec for Ctsuajcr_SPEC {
    type DataType = u32;
}

pub type Ctsuajcr = crate::RegValueT<Ctsuajcr_SPEC>;

impl Ctsuajcr {
    #[inline(always)]
    pub fn tlot(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ctsuajcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ctsuajcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn thot(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ctsuajcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ctsuajcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn blini(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ctsuajcr::Blini,
        ctsuajcr::Blini,
        Ctsuajcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ctsuajcr::Blini,
            ctsuajcr::Blini,
            Ctsuajcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn jc(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x3,
        1,
        0,
        ctsuajcr::Jc,
        ctsuajcr::Jc,
        Ctsuajcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x3,
            1,
            0,
            ctsuajcr::Jc,
            ctsuajcr::Jc,
            Ctsuajcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ajmmat(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, u8, Ctsuajcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8,u8,Ctsuajcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ajbmat(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, u8, Ctsuajcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0xf,1,0,u8,u8,Ctsuajcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuajcr {
    #[inline(always)]
    fn default() -> Ctsuajcr {
        <crate::RegValueT<Ctsuajcr_SPEC> as RegisterValue<_>>::new(65536)
    }
}
pub mod ctsuajcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Blini_SPEC;
    pub type Blini = crate::EnumBitfieldStruct<u8, Blini_SPEC>;
    impl Blini {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Jc_SPEC;
    pub type Jc = crate::EnumBitfieldStruct<u8, Jc_SPEC>;
    impl Jc {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajcrl_SPEC;
impl crate::sealed::RegSpec for Ctsuajcrl_SPEC {
    type DataType = u16;
}

pub type Ctsuajcrl = crate::RegValueT<Ctsuajcrl_SPEC>;

impl Ctsuajcrl {
    #[inline(always)]
    pub fn tlot(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ctsuajcrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ctsuajcrl_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn thot(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ctsuajcrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ctsuajcrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuajcrl {
    #[inline(always)]
    fn default() -> Ctsuajcrl {
        <crate::RegValueT<Ctsuajcrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ajcr0_SPEC;
impl crate::sealed::RegSpec for Ajcr0_SPEC {
    type DataType = u8;
}

pub type Ajcr0 = crate::RegValueT<Ajcr0_SPEC>;

impl Ajcr0 {
    #[inline(always)]
    pub fn tlot(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ajcr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ajcr0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ajcr0 {
    #[inline(always)]
    fn default() -> Ajcr0 {
        <crate::RegValueT<Ajcr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ajcr1_SPEC;
impl crate::sealed::RegSpec for Ajcr1_SPEC {
    type DataType = u8;
}

pub type Ajcr1 = crate::RegValueT<Ajcr1_SPEC>;

impl Ajcr1 {
    #[inline(always)]
    pub fn thot(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ajcr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ajcr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ajcr1 {
    #[inline(always)]
    fn default() -> Ajcr1 {
        <crate::RegValueT<Ajcr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajcrh_SPEC;
impl crate::sealed::RegSpec for Ctsuajcrh_SPEC {
    type DataType = u16;
}

pub type Ctsuajcrh = crate::RegValueT<Ctsuajcrh_SPEC>;

impl Ctsuajcrh {
    #[inline(always)]
    pub fn blini(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ctsuajcrh::Blini,
        ctsuajcrh::Blini,
        Ctsuajcrh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ctsuajcrh::Blini,
            ctsuajcrh::Blini,
            Ctsuajcrh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn jc(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        ctsuajcrh::Jc,
        ctsuajcrh::Jc,
        Ctsuajcrh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            ctsuajcrh::Jc,
            ctsuajcrh::Jc,
            Ctsuajcrh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ajmmat(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, Ctsuajcrh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,Ctsuajcrh_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ajbmat(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Ctsuajcrh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,Ctsuajcrh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuajcrh {
    #[inline(always)]
    fn default() -> Ctsuajcrh {
        <crate::RegValueT<Ctsuajcrh_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod ctsuajcrh {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Blini_SPEC;
    pub type Blini = crate::EnumBitfieldStruct<u8, Blini_SPEC>;
    impl Blini {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Jc_SPEC;
    pub type Jc = crate::EnumBitfieldStruct<u8, Jc_SPEC>;
    impl Jc {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ajcr2_SPEC;
impl crate::sealed::RegSpec for Ajcr2_SPEC {
    type DataType = u8;
}

pub type Ajcr2 = crate::RegValueT<Ajcr2_SPEC>;

impl Ajcr2 {
    #[inline(always)]
    pub fn blini(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ajcr2::Blini,
        ajcr2::Blini,
        Ajcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ajcr2::Blini,
            ajcr2::Blini,
            Ajcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn jc(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        ajcr2::Jc,
        ajcr2::Jc,
        Ajcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            ajcr2::Jc,
            ajcr2::Jc,
            Ajcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ajcr2 {
    #[inline(always)]
    fn default() -> Ajcr2 {
        <crate::RegValueT<Ajcr2_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod ajcr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Blini_SPEC;
    pub type Blini = crate::EnumBitfieldStruct<u8, Blini_SPEC>;
    impl Blini {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Jc_SPEC;
    pub type Jc = crate::EnumBitfieldStruct<u8, Jc_SPEC>;
    impl Jc {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ajcr3_SPEC;
impl crate::sealed::RegSpec for Ajcr3_SPEC {
    type DataType = u8;
}

pub type Ajcr3 = crate::RegValueT<Ajcr3_SPEC>;

impl Ajcr3 {
    #[inline(always)]
    pub fn ajmmat(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Ajcr3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Ajcr3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ajbmat(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Ajcr3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Ajcr3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ajcr3 {
    #[inline(always)]
    fn default() -> Ajcr3 {
        <crate::RegValueT<Ajcr3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajthr_SPEC;
impl crate::sealed::RegSpec for Ctsuajthr_SPEC {
    type DataType = u32;
}

pub type Ctsuajthr = crate::RegValueT<Ctsuajthr_SPEC>;

impl Ctsuajthr {
    #[inline(always)]
    pub fn ajthl(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsuajthr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Ctsuajthr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ajthh(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Ctsuajthr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Ctsuajthr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuajthr {
    #[inline(always)]
    fn default() -> Ctsuajthr {
        <crate::RegValueT<Ctsuajthr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajthrl_SPEC;
impl crate::sealed::RegSpec for Ctsuajthrl_SPEC {
    type DataType = u16;
}

pub type Ctsuajthrl = crate::RegValueT<Ctsuajthrl_SPEC>;

impl Ctsuajthrl {
    #[inline(always)]
    pub fn ajthl(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsuajthrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Ctsuajthrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuajthrl {
    #[inline(always)]
    fn default() -> Ctsuajthrl {
        <crate::RegValueT<Ctsuajthrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajthrh_SPEC;
impl crate::sealed::RegSpec for Ctsuajthrh_SPEC {
    type DataType = u16;
}

pub type Ctsuajthrh = crate::RegValueT<Ctsuajthrh_SPEC>;

impl Ctsuajthrh {
    #[inline(always)]
    pub fn ajthh(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsuajthrh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Ctsuajthrh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuajthrh {
    #[inline(always)]
    fn default() -> Ctsuajthrh {
        <crate::RegValueT<Ctsuajthrh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajmmar_SPEC;
impl crate::sealed::RegSpec for Ctsuajmmar_SPEC {
    type DataType = u32;
}

pub type Ctsuajmmar = crate::RegValueT<Ctsuajmmar_SPEC>;

impl Ctsuajmmar {
    #[inline(always)]
    pub fn ajmmati(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Ctsuajmmar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Ctsuajmmar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ajmmr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x7ffffff,
        1,
        0,
        u32,
        u32,
        Ctsuajmmar_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x7ffffff,
            1,
            0,
            u32,
            u32,
            Ctsuajmmar_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsuajmmar {
    #[inline(always)]
    fn default() -> Ctsuajmmar {
        <crate::RegValueT<Ctsuajmmar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajmmarl_SPEC;
impl crate::sealed::RegSpec for Ctsuajmmarl_SPEC {
    type DataType = u16;
}

pub type Ctsuajmmarl = crate::RegValueT<Ctsuajmmarl_SPEC>;

impl Ctsuajmmarl {
    #[inline(always)]
    pub fn ajmmati(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Ctsuajmmarl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Ctsuajmmarl_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ajmmr(
        self,
    ) -> crate::common::RegisterField<5, 0x7ff, 1, 0, u16, u16, Ctsuajmmarl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x7ff,1,0,u16,u16,Ctsuajmmarl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuajmmarl {
    #[inline(always)]
    fn default() -> Ctsuajmmarl {
        <crate::RegValueT<Ctsuajmmarl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajmmarh_SPEC;
impl crate::sealed::RegSpec for Ctsuajmmarh_SPEC {
    type DataType = u16;
}

pub type Ctsuajmmarh = crate::RegValueT<Ctsuajmmarh_SPEC>;

impl Ctsuajmmarh {
    #[inline(always)]
    pub fn ajmmr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsuajmmarh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Ctsuajmmarh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuajmmarh {
    #[inline(always)]
    fn default() -> Ctsuajmmarh {
        <crate::RegValueT<Ctsuajmmarh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajblact_SPEC;
impl crate::sealed::RegSpec for Ctsuajblact_SPEC {
    type DataType = u32;
}

pub type Ctsuajblact = crate::RegValueT<Ctsuajblact_SPEC>;

impl Ctsuajblact {
    #[inline(always)]
    pub fn ajblact(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Ctsuajblact_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Ctsuajblact_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsuajblact {
    #[inline(always)]
    fn default() -> Ctsuajblact {
        <crate::RegValueT<Ctsuajblact_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajblactl_SPEC;
impl crate::sealed::RegSpec for Ctsuajblactl_SPEC {
    type DataType = u16;
}

pub type Ctsuajblactl = crate::RegValueT<Ctsuajblactl_SPEC>;

impl Ctsuajblactl {
    #[inline(always)]
    pub fn ajblact(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsuajblactl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Ctsuajblactl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsuajblactl {
    #[inline(always)]
    fn default() -> Ctsuajblactl {
        <crate::RegValueT<Ctsuajblactl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajblacth_SPEC;
impl crate::sealed::RegSpec for Ctsuajblacth_SPEC {
    type DataType = u16;
}

pub type Ctsuajblacth = crate::RegValueT<Ctsuajblacth_SPEC>;

impl Ctsuajblacth {
    #[inline(always)]
    pub fn ajblact(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsuajblacth_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Ctsuajblacth_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsuajblacth {
    #[inline(always)]
    fn default() -> Ctsuajblacth {
        <crate::RegValueT<Ctsuajblacth_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajblar_SPEC;
impl crate::sealed::RegSpec for Ctsuajblar_SPEC {
    type DataType = u32;
}

pub type Ctsuajblar = crate::RegValueT<Ctsuajblar_SPEC>;

impl Ctsuajblar {
    #[inline(always)]
    pub fn ajblac(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsuajblar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Ctsuajblar_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ajblar(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Ctsuajblar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Ctsuajblar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuajblar {
    #[inline(always)]
    fn default() -> Ctsuajblar {
        <crate::RegValueT<Ctsuajblar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajblarl_SPEC;
impl crate::sealed::RegSpec for Ctsuajblarl_SPEC {
    type DataType = u16;
}

pub type Ctsuajblarl = crate::RegValueT<Ctsuajblarl_SPEC>;

impl Ctsuajblarl {
    #[inline(always)]
    pub fn ajblac(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsuajblarl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Ctsuajblarl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuajblarl {
    #[inline(always)]
    fn default() -> Ctsuajblarl {
        <crate::RegValueT<Ctsuajblarl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajblarh_SPEC;
impl crate::sealed::RegSpec for Ctsuajblarh_SPEC {
    type DataType = u16;
}

pub type Ctsuajblarh = crate::RegValueT<Ctsuajblarh_SPEC>;

impl Ctsuajblarh {
    #[inline(always)]
    pub fn ajblar(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsuajblarh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Ctsuajblarh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuajblarh {
    #[inline(always)]
    fn default() -> Ctsuajblarh {
        <crate::RegValueT<Ctsuajblarh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajrr_SPEC;
impl crate::sealed::RegSpec for Ctsuajrr_SPEC {
    type DataType = u32;
}

pub type Ctsuajrr = crate::RegValueT<Ctsuajrr_SPEC>;

impl Ctsuajrr {
    #[inline(always)]
    pub fn tjr0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ctsuajrr::Tjr0,
        ctsuajrr::Tjr0,
        Ctsuajrr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ctsuajrr::Tjr0,
            ctsuajrr::Tjr0,
            Ctsuajrr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tjr1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ctsuajrr::Tjr1,
        ctsuajrr::Tjr1,
        Ctsuajrr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ctsuajrr::Tjr1,
            ctsuajrr::Tjr1,
            Ctsuajrr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tjr2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ctsuajrr::Tjr2,
        ctsuajrr::Tjr2,
        Ctsuajrr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ctsuajrr::Tjr2,
            ctsuajrr::Tjr2,
            Ctsuajrr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tjr3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ctsuajrr::Tjr3,
        ctsuajrr::Tjr3,
        Ctsuajrr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ctsuajrr::Tjr3,
            ctsuajrr::Tjr3,
            Ctsuajrr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fjr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ctsuajrr::Fjr,
        ctsuajrr::Fjr,
        Ctsuajrr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ctsuajrr::Fjr,
            ctsuajrr::Fjr,
            Ctsuajrr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sjccr(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ctsuajrr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ctsuajrr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuajrr {
    #[inline(always)]
    fn default() -> Ctsuajrr {
        <crate::RegValueT<Ctsuajrr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctsuajrr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tjr0_SPEC;
    pub type Tjr0 = crate::EnumBitfieldStruct<u8, Tjr0_SPEC>;
    impl Tjr0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tjr1_SPEC;
    pub type Tjr1 = crate::EnumBitfieldStruct<u8, Tjr1_SPEC>;
    impl Tjr1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tjr2_SPEC;
    pub type Tjr2 = crate::EnumBitfieldStruct<u8, Tjr2_SPEC>;
    impl Tjr2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tjr3_SPEC;
    pub type Tjr3 = crate::EnumBitfieldStruct<u8, Tjr3_SPEC>;
    impl Tjr3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fjr_SPEC;
    pub type Fjr = crate::EnumBitfieldStruct<u8, Fjr_SPEC>;
    impl Fjr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajrrl_SPEC;
impl crate::sealed::RegSpec for Ctsuajrrl_SPEC {
    type DataType = u16;
}

pub type Ctsuajrrl = crate::RegValueT<Ctsuajrrl_SPEC>;

impl Ctsuajrrl {
    #[inline(always)]
    pub fn tjr0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ctsuajrrl::Tjr0,
        ctsuajrrl::Tjr0,
        Ctsuajrrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ctsuajrrl::Tjr0,
            ctsuajrrl::Tjr0,
            Ctsuajrrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tjr1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ctsuajrrl::Tjr1,
        ctsuajrrl::Tjr1,
        Ctsuajrrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ctsuajrrl::Tjr1,
            ctsuajrrl::Tjr1,
            Ctsuajrrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tjr2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ctsuajrrl::Tjr2,
        ctsuajrrl::Tjr2,
        Ctsuajrrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ctsuajrrl::Tjr2,
            ctsuajrrl::Tjr2,
            Ctsuajrrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tjr3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ctsuajrrl::Tjr3,
        ctsuajrrl::Tjr3,
        Ctsuajrrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ctsuajrrl::Tjr3,
            ctsuajrrl::Tjr3,
            Ctsuajrrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fjr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ctsuajrrl::Fjr,
        ctsuajrrl::Fjr,
        Ctsuajrrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ctsuajrrl::Fjr,
            ctsuajrrl::Fjr,
            Ctsuajrrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sjccr(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ctsuajrrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ctsuajrrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuajrrl {
    #[inline(always)]
    fn default() -> Ctsuajrrl {
        <crate::RegValueT<Ctsuajrrl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctsuajrrl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tjr0_SPEC;
    pub type Tjr0 = crate::EnumBitfieldStruct<u8, Tjr0_SPEC>;
    impl Tjr0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tjr1_SPEC;
    pub type Tjr1 = crate::EnumBitfieldStruct<u8, Tjr1_SPEC>;
    impl Tjr1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tjr2_SPEC;
    pub type Tjr2 = crate::EnumBitfieldStruct<u8, Tjr2_SPEC>;
    impl Tjr2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tjr3_SPEC;
    pub type Tjr3 = crate::EnumBitfieldStruct<u8, Tjr3_SPEC>;
    impl Tjr3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fjr_SPEC;
    pub type Fjr = crate::EnumBitfieldStruct<u8, Fjr_SPEC>;
    impl Fjr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajrr0_SPEC;
impl crate::sealed::RegSpec for Ctsuajrr0_SPEC {
    type DataType = u8;
}

pub type Ctsuajrr0 = crate::RegValueT<Ctsuajrr0_SPEC>;

impl Ctsuajrr0 {
    #[inline(always)]
    pub fn tjr0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ctsuajrr0::Tjr0,
        ctsuajrr0::Tjr0,
        Ctsuajrr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ctsuajrr0::Tjr0,
            ctsuajrr0::Tjr0,
            Ctsuajrr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tjr1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ctsuajrr0::Tjr1,
        ctsuajrr0::Tjr1,
        Ctsuajrr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ctsuajrr0::Tjr1,
            ctsuajrr0::Tjr1,
            Ctsuajrr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tjr2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ctsuajrr0::Tjr2,
        ctsuajrr0::Tjr2,
        Ctsuajrr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ctsuajrr0::Tjr2,
            ctsuajrr0::Tjr2,
            Ctsuajrr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tjr3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ctsuajrr0::Tjr3,
        ctsuajrr0::Tjr3,
        Ctsuajrr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ctsuajrr0::Tjr3,
            ctsuajrr0::Tjr3,
            Ctsuajrr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fjr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ctsuajrr0::Fjr,
        ctsuajrr0::Fjr,
        Ctsuajrr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ctsuajrr0::Fjr,
            ctsuajrr0::Fjr,
            Ctsuajrr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsuajrr0 {
    #[inline(always)]
    fn default() -> Ctsuajrr0 {
        <crate::RegValueT<Ctsuajrr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctsuajrr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tjr0_SPEC;
    pub type Tjr0 = crate::EnumBitfieldStruct<u8, Tjr0_SPEC>;
    impl Tjr0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tjr1_SPEC;
    pub type Tjr1 = crate::EnumBitfieldStruct<u8, Tjr1_SPEC>;
    impl Tjr1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tjr2_SPEC;
    pub type Tjr2 = crate::EnumBitfieldStruct<u8, Tjr2_SPEC>;
    impl Tjr2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tjr3_SPEC;
    pub type Tjr3 = crate::EnumBitfieldStruct<u8, Tjr3_SPEC>;
    impl Tjr3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fjr_SPEC;
    pub type Fjr = crate::EnumBitfieldStruct<u8, Fjr_SPEC>;
    impl Fjr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuajrr1_SPEC;
impl crate::sealed::RegSpec for Ctsuajrr1_SPEC {
    type DataType = u8;
}

pub type Ctsuajrr1 = crate::RegValueT<Ctsuajrr1_SPEC>;

impl Ctsuajrr1 {
    #[inline(always)]
    pub fn sjccr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ctsuajrr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ctsuajrr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsuajrr1 {
    #[inline(always)]
    fn default() -> Ctsuajrr1 {
        <crate::RegValueT<Ctsuajrr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
