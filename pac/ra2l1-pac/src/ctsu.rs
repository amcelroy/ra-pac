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
    pub const fn ctsucrah(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsucrah_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsucrah_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
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
    pub const fn ctsuchacah(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchacah_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchacah_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuchac2(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchac2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchac2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuchac3(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchac3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchac3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(15usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuchacb(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchacb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchacb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuchacbl(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchacbl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchacbl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuchac4(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchac4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchac4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
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
    pub const fn ctsuchtrcah(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchtrcah_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchtrcah_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuchtrc2(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchtrc2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchtrc2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuchtrc3(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchtrc3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchtrc3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(23usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuchtrcb(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchtrcb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchtrcb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuchtrcbl(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchtrcbl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchtrcbl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsuchtrc4(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsuchtrc4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctsuchtrc4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
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
    pub const fn ctsucfccnt(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsucfccnt_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ctsucfccnt_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ctsucfccntl(
        &self,
    ) -> &'static crate::common::Reg<self::Ctsucfccntl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ctsucfccntl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
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
    pub fn cfcon(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ctsucra::Cfcon,
        ctsucra::Cfcon,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ctsucra::Cfcon,
            ctsucra::Cfcon,
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
    pub fn md2(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ctsucra::Md2,
        ctsucra::Md2,
        Ctsucra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ctsucra::Md2,
            ctsucra::Md2,
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
    pub struct Cfcon_SPEC;
    pub type Cfcon = crate::EnumBitfieldStruct<u8, Cfcon_SPEC>;
    impl Cfcon {
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
    pub struct Md2_SPEC;
    pub type Md2 = crate::EnumBitfieldStruct<u8, Md2_SPEC>;
    impl Md2 {
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
pub struct Ctsucrah_SPEC;
impl crate::sealed::RegSpec for Ctsucrah_SPEC {
    type DataType = u16;
}

pub type Ctsucrah = crate::RegValueT<Ctsucrah_SPEC>;

impl NoBitfieldReg<Ctsucrah_SPEC> for Ctsucrah {}
impl ::core::default::Default for Ctsucrah {
    #[inline(always)]
    fn default() -> Ctsucrah {
        <crate::RegValueT<Ctsucrah_SPEC> as RegisterValue<_>>::new(0)
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

        pub const _0_X_02: Self = Self::new(2);

        pub const _0_X_04: Self = Self::new(4);

        pub const _0_X_05: Self = Self::new(5);

        pub const _0_X_06: Self = Self::new(6);

        pub const _0_X_07: Self = Self::new(7);

        pub const _0_X_08: Self = Self::new(8);

        pub const _0_X_09: Self = Self::new(9);

        pub const _0_X_0_A: Self = Self::new(10);

        pub const _0_X_0_B: Self = Self::new(11);

        pub const _0_X_0_C: Self = Self::new(12);

        pub const _0_X_0_D: Self = Self::new(13);

        pub const _0_X_0_E: Self = Self::new(14);

        pub const _0_X_0_F: Self = Self::new(15);

        pub const _0_X_10: Self = Self::new(16);

        pub const _0_X_11: Self = Self::new(17);

        pub const _0_X_12: Self = Self::new(18);

        pub const _0_X_15: Self = Self::new(21);

        pub const _0_X_16: Self = Self::new(22);

        pub const _0_X_17: Self = Self::new(23);

        pub const _0_X_18: Self = Self::new(24);

        pub const _0_X_19: Self = Self::new(25);

        pub const _0_X_1_A: Self = Self::new(26);

        pub const _0_X_1_B: Self = Self::new(27);

        pub const _0_X_1_C: Self = Self::new(28);

        pub const _0_X_1_D: Self = Self::new(29);

        pub const _0_X_1_E: Self = Self::new(30);

        pub const _0_X_1_F: Self = Self::new(31);

        pub const _0_X_20: Self = Self::new(32);

        pub const _0_X_21: Self = Self::new(33);

        pub const _0_X_22: Self = Self::new(34);

        pub const _0_X_23: Self = Self::new(35);

        pub const _0_X_3_F: Self = Self::new(63);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mch1_SPEC;
    pub type Mch1 = crate::EnumBitfieldStruct<u8, Mch1_SPEC>;
    impl Mch1 {
        pub const _0_X_00: Self = Self::new(0);

        pub const _0_X_02: Self = Self::new(2);

        pub const _0_X_04: Self = Self::new(4);

        pub const _0_X_05: Self = Self::new(5);

        pub const _0_X_06: Self = Self::new(6);

        pub const _0_X_07: Self = Self::new(7);

        pub const _0_X_08: Self = Self::new(8);

        pub const _0_X_09: Self = Self::new(9);

        pub const _0_X_0_A: Self = Self::new(10);

        pub const _0_X_0_B: Self = Self::new(11);

        pub const _0_X_0_C: Self = Self::new(12);

        pub const _0_X_0_D: Self = Self::new(13);

        pub const _0_X_0_E: Self = Self::new(14);

        pub const _0_X_0_F: Self = Self::new(15);

        pub const _0_X_10: Self = Self::new(16);

        pub const _0_X_11: Self = Self::new(17);

        pub const _0_X_12: Self = Self::new(18);

        pub const _0_X_15: Self = Self::new(21);

        pub const _0_X_16: Self = Self::new(22);

        pub const _0_X_17: Self = Self::new(23);

        pub const _0_X_18: Self = Self::new(24);

        pub const _0_X_19: Self = Self::new(25);

        pub const _0_X_1_A: Self = Self::new(26);

        pub const _0_X_1_B: Self = Self::new(27);

        pub const _0_X_1_C: Self = Self::new(28);

        pub const _0_X_1_D: Self = Self::new(29);

        pub const _0_X_1_E: Self = Self::new(30);

        pub const _0_X_1_F: Self = Self::new(31);

        pub const _0_X_20: Self = Self::new(32);

        pub const _0_X_21: Self = Self::new(33);

        pub const _0_X_22: Self = Self::new(34);

        pub const _0_X_23: Self = Self::new(35);

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
        <crate::RegValueT<Ctsumchl_SPEC> as RegisterValue<_>>::new(0)
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
        <crate::RegValueT<Ctsumch0_SPEC> as RegisterValue<_>>::new(0)
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
        <crate::RegValueT<Ctsumch1_SPEC> as RegisterValue<_>>::new(0)
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
        <crate::RegValueT<Ctsumchh_SPEC> as RegisterValue<_>>::new(16191)
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
        <crate::RegValueT<Ctsumfaf_SPEC> as RegisterValue<_>>::new(63)
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

    #[inline(always)]
    pub fn chac12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        ctsuchaca::Chac12,
        ctsuchaca::Chac12,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            ctsuchaca::Chac12,
            ctsuchaca::Chac12,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chac13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        ctsuchaca::Chac13,
        ctsuchaca::Chac13,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            ctsuchaca::Chac13,
            ctsuchaca::Chac13,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chac14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        ctsuchaca::Chac14,
        ctsuchaca::Chac14,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            ctsuchaca::Chac14,
            ctsuchaca::Chac14,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chac15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        ctsuchaca::Chac15,
        ctsuchaca::Chac15,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            ctsuchaca::Chac15,
            ctsuchaca::Chac15,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chac16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ctsuchaca::Chac16,
        ctsuchaca::Chac16,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ctsuchaca::Chac16,
            ctsuchaca::Chac16,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chac17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        ctsuchaca::Chac17,
        ctsuchaca::Chac17,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            ctsuchaca::Chac17,
            ctsuchaca::Chac17,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chac18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        ctsuchaca::Chac18,
        ctsuchaca::Chac18,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            ctsuchaca::Chac18,
            ctsuchaca::Chac18,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chac21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        ctsuchaca::Chac21,
        ctsuchaca::Chac21,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            ctsuchaca::Chac21,
            ctsuchaca::Chac21,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chac22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        ctsuchaca::Chac22,
        ctsuchaca::Chac22,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            ctsuchaca::Chac22,
            ctsuchaca::Chac22,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chac23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        ctsuchaca::Chac23,
        ctsuchaca::Chac23,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            ctsuchaca::Chac23,
            ctsuchaca::Chac23,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chac24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        ctsuchaca::Chac24,
        ctsuchaca::Chac24,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            ctsuchaca::Chac24,
            ctsuchaca::Chac24,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chac25(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        ctsuchaca::Chac25,
        ctsuchaca::Chac25,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            ctsuchaca::Chac25,
            ctsuchaca::Chac25,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chac26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        ctsuchaca::Chac26,
        ctsuchaca::Chac26,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            ctsuchaca::Chac26,
            ctsuchaca::Chac26,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chac27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        ctsuchaca::Chac27,
        ctsuchaca::Chac27,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            ctsuchaca::Chac27,
            ctsuchaca::Chac27,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chac28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        ctsuchaca::Chac28,
        ctsuchaca::Chac28,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            ctsuchaca::Chac28,
            ctsuchaca::Chac28,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chac29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        ctsuchaca::Chac29,
        ctsuchaca::Chac29,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            ctsuchaca::Chac29,
            ctsuchaca::Chac29,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chac30(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        ctsuchaca::Chac30,
        ctsuchaca::Chac30,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            ctsuchaca::Chac30,
            ctsuchaca::Chac30,
            Ctsuchaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chac31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        ctsuchaca::Chac31,
        ctsuchaca::Chac31,
        Ctsuchaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            ctsuchaca::Chac31,
            ctsuchaca::Chac31,
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
    pub struct Chac02_SPEC;
    pub type Chac02 = crate::EnumBitfieldStruct<u8, Chac02_SPEC>;
    impl Chac02 {
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac12_SPEC;
    pub type Chac12 = crate::EnumBitfieldStruct<u8, Chac12_SPEC>;
    impl Chac12 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac13_SPEC;
    pub type Chac13 = crate::EnumBitfieldStruct<u8, Chac13_SPEC>;
    impl Chac13 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac14_SPEC;
    pub type Chac14 = crate::EnumBitfieldStruct<u8, Chac14_SPEC>;
    impl Chac14 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac15_SPEC;
    pub type Chac15 = crate::EnumBitfieldStruct<u8, Chac15_SPEC>;
    impl Chac15 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac16_SPEC;
    pub type Chac16 = crate::EnumBitfieldStruct<u8, Chac16_SPEC>;
    impl Chac16 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac17_SPEC;
    pub type Chac17 = crate::EnumBitfieldStruct<u8, Chac17_SPEC>;
    impl Chac17 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac18_SPEC;
    pub type Chac18 = crate::EnumBitfieldStruct<u8, Chac18_SPEC>;
    impl Chac18 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac21_SPEC;
    pub type Chac21 = crate::EnumBitfieldStruct<u8, Chac21_SPEC>;
    impl Chac21 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac22_SPEC;
    pub type Chac22 = crate::EnumBitfieldStruct<u8, Chac22_SPEC>;
    impl Chac22 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac23_SPEC;
    pub type Chac23 = crate::EnumBitfieldStruct<u8, Chac23_SPEC>;
    impl Chac23 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac24_SPEC;
    pub type Chac24 = crate::EnumBitfieldStruct<u8, Chac24_SPEC>;
    impl Chac24 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac25_SPEC;
    pub type Chac25 = crate::EnumBitfieldStruct<u8, Chac25_SPEC>;
    impl Chac25 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac26_SPEC;
    pub type Chac26 = crate::EnumBitfieldStruct<u8, Chac26_SPEC>;
    impl Chac26 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac27_SPEC;
    pub type Chac27 = crate::EnumBitfieldStruct<u8, Chac27_SPEC>;
    impl Chac27 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac28_SPEC;
    pub type Chac28 = crate::EnumBitfieldStruct<u8, Chac28_SPEC>;
    impl Chac28 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac29_SPEC;
    pub type Chac29 = crate::EnumBitfieldStruct<u8, Chac29_SPEC>;
    impl Chac29 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac30_SPEC;
    pub type Chac30 = crate::EnumBitfieldStruct<u8, Chac30_SPEC>;
    impl Chac30 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac31_SPEC;
    pub type Chac31 = crate::EnumBitfieldStruct<u8, Chac31_SPEC>;
    impl Chac31 {
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
pub struct Ctsuchacah_SPEC;
impl crate::sealed::RegSpec for Ctsuchacah_SPEC {
    type DataType = u16;
}

pub type Ctsuchacah = crate::RegValueT<Ctsuchacah_SPEC>;

impl NoBitfieldReg<Ctsuchacah_SPEC> for Ctsuchacah {}
impl ::core::default::Default for Ctsuchacah {
    #[inline(always)]
    fn default() -> Ctsuchacah {
        <crate::RegValueT<Ctsuchacah_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchac2_SPEC;
impl crate::sealed::RegSpec for Ctsuchac2_SPEC {
    type DataType = u8;
}

pub type Ctsuchac2 = crate::RegValueT<Ctsuchac2_SPEC>;

impl NoBitfieldReg<Ctsuchac2_SPEC> for Ctsuchac2 {}
impl ::core::default::Default for Ctsuchac2 {
    #[inline(always)]
    fn default() -> Ctsuchac2 {
        <crate::RegValueT<Ctsuchac2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchac3_SPEC;
impl crate::sealed::RegSpec for Ctsuchac3_SPEC {
    type DataType = u8;
}

pub type Ctsuchac3 = crate::RegValueT<Ctsuchac3_SPEC>;

impl NoBitfieldReg<Ctsuchac3_SPEC> for Ctsuchac3 {}
impl ::core::default::Default for Ctsuchac3 {
    #[inline(always)]
    fn default() -> Ctsuchac3 {
        <crate::RegValueT<Ctsuchac3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchacb_SPEC;
impl crate::sealed::RegSpec for Ctsuchacb_SPEC {
    type DataType = u32;
}

pub type Ctsuchacb = crate::RegValueT<Ctsuchacb_SPEC>;

impl Ctsuchacb {
    #[inline(always)]
    pub fn chac32(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ctsuchacb::Chac32,
        ctsuchacb::Chac32,
        Ctsuchacb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ctsuchacb::Chac32,
            ctsuchacb::Chac32,
            Ctsuchacb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chac33(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ctsuchacb::Chac33,
        ctsuchacb::Chac33,
        Ctsuchacb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ctsuchacb::Chac33,
            ctsuchacb::Chac33,
            Ctsuchacb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chac34(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ctsuchacb::Chac34,
        ctsuchacb::Chac34,
        Ctsuchacb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ctsuchacb::Chac34,
            ctsuchacb::Chac34,
            Ctsuchacb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chac35(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ctsuchacb::Chac35,
        ctsuchacb::Chac35,
        Ctsuchacb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ctsuchacb::Chac35,
            ctsuchacb::Chac35,
            Ctsuchacb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsuchacb {
    #[inline(always)]
    fn default() -> Ctsuchacb {
        <crate::RegValueT<Ctsuchacb_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctsuchacb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac32_SPEC;
    pub type Chac32 = crate::EnumBitfieldStruct<u8, Chac32_SPEC>;
    impl Chac32 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac33_SPEC;
    pub type Chac33 = crate::EnumBitfieldStruct<u8, Chac33_SPEC>;
    impl Chac33 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac34_SPEC;
    pub type Chac34 = crate::EnumBitfieldStruct<u8, Chac34_SPEC>;
    impl Chac34 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chac35_SPEC;
    pub type Chac35 = crate::EnumBitfieldStruct<u8, Chac35_SPEC>;
    impl Chac35 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchacbl_SPEC;
impl crate::sealed::RegSpec for Ctsuchacbl_SPEC {
    type DataType = u16;
}

pub type Ctsuchacbl = crate::RegValueT<Ctsuchacbl_SPEC>;

impl NoBitfieldReg<Ctsuchacbl_SPEC> for Ctsuchacbl {}
impl ::core::default::Default for Ctsuchacbl {
    #[inline(always)]
    fn default() -> Ctsuchacbl {
        <crate::RegValueT<Ctsuchacbl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchac4_SPEC;
impl crate::sealed::RegSpec for Ctsuchac4_SPEC {
    type DataType = u8;
}

pub type Ctsuchac4 = crate::RegValueT<Ctsuchac4_SPEC>;

impl NoBitfieldReg<Ctsuchac4_SPEC> for Ctsuchac4 {}
impl ::core::default::Default for Ctsuchac4 {
    #[inline(always)]
    fn default() -> Ctsuchac4 {
        <crate::RegValueT<Ctsuchac4_SPEC> as RegisterValue<_>>::new(0)
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

    #[inline(always)]
    pub fn chtrc12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc12,
        ctsuchtrca::Chtrc12,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc12,
            ctsuchtrca::Chtrc12,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chtrc13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc13,
        ctsuchtrca::Chtrc13,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc13,
            ctsuchtrca::Chtrc13,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chtrc14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc14,
        ctsuchtrca::Chtrc14,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc14,
            ctsuchtrca::Chtrc14,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chtrc15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc15,
        ctsuchtrca::Chtrc15,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc15,
            ctsuchtrca::Chtrc15,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chtrc16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc16,
        ctsuchtrca::Chtrc16,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc16,
            ctsuchtrca::Chtrc16,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chtrc17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc17,
        ctsuchtrca::Chtrc17,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc17,
            ctsuchtrca::Chtrc17,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chtrc18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc18,
        ctsuchtrca::Chtrc18,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc18,
            ctsuchtrca::Chtrc18,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chtrc21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc21,
        ctsuchtrca::Chtrc21,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc21,
            ctsuchtrca::Chtrc21,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chtrc22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc22,
        ctsuchtrca::Chtrc22,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc22,
            ctsuchtrca::Chtrc22,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chtrc23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc23,
        ctsuchtrca::Chtrc23,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc23,
            ctsuchtrca::Chtrc23,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chtrc24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc24,
        ctsuchtrca::Chtrc24,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc24,
            ctsuchtrca::Chtrc24,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chtrc25(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc25,
        ctsuchtrca::Chtrc25,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc25,
            ctsuchtrca::Chtrc25,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chtrc26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc26,
        ctsuchtrca::Chtrc26,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc26,
            ctsuchtrca::Chtrc26,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chtrc27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc27,
        ctsuchtrca::Chtrc27,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc27,
            ctsuchtrca::Chtrc27,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chtrc28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc28,
        ctsuchtrca::Chtrc28,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc28,
            ctsuchtrca::Chtrc28,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chtrc29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc29,
        ctsuchtrca::Chtrc29,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc29,
            ctsuchtrca::Chtrc29,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chtrc30(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc30,
        ctsuchtrca::Chtrc30,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc30,
            ctsuchtrca::Chtrc30,
            Ctsuchtrca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chtrc31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        ctsuchtrca::Chtrc31,
        ctsuchtrca::Chtrc31,
        Ctsuchtrca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            ctsuchtrca::Chtrc31,
            ctsuchtrca::Chtrc31,
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
    pub struct Chtrc02_SPEC;
    pub type Chtrc02 = crate::EnumBitfieldStruct<u8, Chtrc02_SPEC>;
    impl Chtrc02 {
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc12_SPEC;
    pub type Chtrc12 = crate::EnumBitfieldStruct<u8, Chtrc12_SPEC>;
    impl Chtrc12 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc13_SPEC;
    pub type Chtrc13 = crate::EnumBitfieldStruct<u8, Chtrc13_SPEC>;
    impl Chtrc13 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc14_SPEC;
    pub type Chtrc14 = crate::EnumBitfieldStruct<u8, Chtrc14_SPEC>;
    impl Chtrc14 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc15_SPEC;
    pub type Chtrc15 = crate::EnumBitfieldStruct<u8, Chtrc15_SPEC>;
    impl Chtrc15 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc16_SPEC;
    pub type Chtrc16 = crate::EnumBitfieldStruct<u8, Chtrc16_SPEC>;
    impl Chtrc16 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc17_SPEC;
    pub type Chtrc17 = crate::EnumBitfieldStruct<u8, Chtrc17_SPEC>;
    impl Chtrc17 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc18_SPEC;
    pub type Chtrc18 = crate::EnumBitfieldStruct<u8, Chtrc18_SPEC>;
    impl Chtrc18 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc21_SPEC;
    pub type Chtrc21 = crate::EnumBitfieldStruct<u8, Chtrc21_SPEC>;
    impl Chtrc21 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc22_SPEC;
    pub type Chtrc22 = crate::EnumBitfieldStruct<u8, Chtrc22_SPEC>;
    impl Chtrc22 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc23_SPEC;
    pub type Chtrc23 = crate::EnumBitfieldStruct<u8, Chtrc23_SPEC>;
    impl Chtrc23 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc24_SPEC;
    pub type Chtrc24 = crate::EnumBitfieldStruct<u8, Chtrc24_SPEC>;
    impl Chtrc24 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc25_SPEC;
    pub type Chtrc25 = crate::EnumBitfieldStruct<u8, Chtrc25_SPEC>;
    impl Chtrc25 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc26_SPEC;
    pub type Chtrc26 = crate::EnumBitfieldStruct<u8, Chtrc26_SPEC>;
    impl Chtrc26 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc27_SPEC;
    pub type Chtrc27 = crate::EnumBitfieldStruct<u8, Chtrc27_SPEC>;
    impl Chtrc27 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc28_SPEC;
    pub type Chtrc28 = crate::EnumBitfieldStruct<u8, Chtrc28_SPEC>;
    impl Chtrc28 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc29_SPEC;
    pub type Chtrc29 = crate::EnumBitfieldStruct<u8, Chtrc29_SPEC>;
    impl Chtrc29 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc30_SPEC;
    pub type Chtrc30 = crate::EnumBitfieldStruct<u8, Chtrc30_SPEC>;
    impl Chtrc30 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc31_SPEC;
    pub type Chtrc31 = crate::EnumBitfieldStruct<u8, Chtrc31_SPEC>;
    impl Chtrc31 {
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
pub struct Ctsuchtrcah_SPEC;
impl crate::sealed::RegSpec for Ctsuchtrcah_SPEC {
    type DataType = u16;
}

pub type Ctsuchtrcah = crate::RegValueT<Ctsuchtrcah_SPEC>;

impl NoBitfieldReg<Ctsuchtrcah_SPEC> for Ctsuchtrcah {}
impl ::core::default::Default for Ctsuchtrcah {
    #[inline(always)]
    fn default() -> Ctsuchtrcah {
        <crate::RegValueT<Ctsuchtrcah_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchtrc2_SPEC;
impl crate::sealed::RegSpec for Ctsuchtrc2_SPEC {
    type DataType = u8;
}

pub type Ctsuchtrc2 = crate::RegValueT<Ctsuchtrc2_SPEC>;

impl NoBitfieldReg<Ctsuchtrc2_SPEC> for Ctsuchtrc2 {}
impl ::core::default::Default for Ctsuchtrc2 {
    #[inline(always)]
    fn default() -> Ctsuchtrc2 {
        <crate::RegValueT<Ctsuchtrc2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchtrc3_SPEC;
impl crate::sealed::RegSpec for Ctsuchtrc3_SPEC {
    type DataType = u8;
}

pub type Ctsuchtrc3 = crate::RegValueT<Ctsuchtrc3_SPEC>;

impl NoBitfieldReg<Ctsuchtrc3_SPEC> for Ctsuchtrc3 {}
impl ::core::default::Default for Ctsuchtrc3 {
    #[inline(always)]
    fn default() -> Ctsuchtrc3 {
        <crate::RegValueT<Ctsuchtrc3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchtrcb_SPEC;
impl crate::sealed::RegSpec for Ctsuchtrcb_SPEC {
    type DataType = u32;
}

pub type Ctsuchtrcb = crate::RegValueT<Ctsuchtrcb_SPEC>;

impl Ctsuchtrcb {
    #[inline(always)]
    pub fn chtrc32(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ctsuchtrcb::Chtrc32,
        ctsuchtrcb::Chtrc32,
        Ctsuchtrcb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ctsuchtrcb::Chtrc32,
            ctsuchtrcb::Chtrc32,
            Ctsuchtrcb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chtrc33(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ctsuchtrcb::Chtrc33,
        ctsuchtrcb::Chtrc33,
        Ctsuchtrcb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ctsuchtrcb::Chtrc33,
            ctsuchtrcb::Chtrc33,
            Ctsuchtrcb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chtrc34(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ctsuchtrcb::Chtrc34,
        ctsuchtrcb::Chtrc34,
        Ctsuchtrcb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ctsuchtrcb::Chtrc34,
            ctsuchtrcb::Chtrc34,
            Ctsuchtrcb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chtrc35(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ctsuchtrcb::Chtrc35,
        ctsuchtrcb::Chtrc35,
        Ctsuchtrcb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ctsuchtrcb::Chtrc35,
            ctsuchtrcb::Chtrc35,
            Ctsuchtrcb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctsuchtrcb {
    #[inline(always)]
    fn default() -> Ctsuchtrcb {
        <crate::RegValueT<Ctsuchtrcb_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctsuchtrcb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc32_SPEC;
    pub type Chtrc32 = crate::EnumBitfieldStruct<u8, Chtrc32_SPEC>;
    impl Chtrc32 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc33_SPEC;
    pub type Chtrc33 = crate::EnumBitfieldStruct<u8, Chtrc33_SPEC>;
    impl Chtrc33 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc34_SPEC;
    pub type Chtrc34 = crate::EnumBitfieldStruct<u8, Chtrc34_SPEC>;
    impl Chtrc34 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chtrc35_SPEC;
    pub type Chtrc35 = crate::EnumBitfieldStruct<u8, Chtrc35_SPEC>;
    impl Chtrc35 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchtrcbl_SPEC;
impl crate::sealed::RegSpec for Ctsuchtrcbl_SPEC {
    type DataType = u16;
}

pub type Ctsuchtrcbl = crate::RegValueT<Ctsuchtrcbl_SPEC>;

impl NoBitfieldReg<Ctsuchtrcbl_SPEC> for Ctsuchtrcbl {}
impl ::core::default::Default for Ctsuchtrcbl {
    #[inline(always)]
    fn default() -> Ctsuchtrcbl {
        <crate::RegValueT<Ctsuchtrcbl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsuchtrc4_SPEC;
impl crate::sealed::RegSpec for Ctsuchtrc4_SPEC {
    type DataType = u8;
}

pub type Ctsuchtrc4 = crate::RegValueT<Ctsuchtrc4_SPEC>;

impl NoBitfieldReg<Ctsuchtrc4_SPEC> for Ctsuchtrc4 {}
impl ::core::default::Default for Ctsuchtrc4 {
    #[inline(always)]
    fn default() -> Ctsuchtrc4 {
        <crate::RegValueT<Ctsuchtrc4_SPEC> as RegisterValue<_>>::new(0)
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
    pub fn suovf(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        ctsusr::Suovf,
        ctsusr::Suovf,
        Ctsusr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            ctsusr::Suovf,
            ctsusr::Suovf,
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

    #[inline(always)]
    pub fn cfcrdch(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3f,
        1,
        0,
        ctsusr::Cfcrdch,
        ctsusr::Cfcrdch,
        Ctsusr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3f,
            1,
            0,
            ctsusr::Cfcrdch,
            ctsusr::Cfcrdch,
            Ctsusr_SPEC,
            crate::common::RW,
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
    pub struct Suovf_SPEC;
    pub type Suovf = crate::EnumBitfieldStruct<u8, Suovf_SPEC>;
    impl Suovf {
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfcrdch_SPEC;
    pub type Cfcrdch = crate::EnumBitfieldStruct<u8, Cfcrdch_SPEC>;
    impl Cfcrdch {
        pub const _0_X_00: Self = Self::new(0);

        pub const _0_X_02: Self = Self::new(2);

        pub const _0_X_04: Self = Self::new(4);

        pub const _0_X_05: Self = Self::new(5);

        pub const _0_X_06: Self = Self::new(6);

        pub const _0_X_07: Self = Self::new(7);

        pub const _0_X_08: Self = Self::new(8);

        pub const _0_X_09: Self = Self::new(9);

        pub const _0_X_0_A: Self = Self::new(10);

        pub const _0_X_0_B: Self = Self::new(11);

        pub const _0_X_0_C: Self = Self::new(12);

        pub const _0_X_0_D: Self = Self::new(13);

        pub const _0_X_0_E: Self = Self::new(14);

        pub const _0_X_0_F: Self = Self::new(15);

        pub const _0_X_10: Self = Self::new(16);

        pub const _0_X_11: Self = Self::new(17);

        pub const _0_X_12: Self = Self::new(18);

        pub const _0_X_15: Self = Self::new(21);

        pub const _0_X_16: Self = Self::new(22);

        pub const _0_X_17: Self = Self::new(23);

        pub const _0_X_18: Self = Self::new(24);

        pub const _0_X_19: Self = Self::new(25);

        pub const _0_X_1_A: Self = Self::new(26);

        pub const _0_X_1_B: Self = Self::new(27);

        pub const _0_X_1_C: Self = Self::new(28);

        pub const _0_X_1_D: Self = Self::new(29);

        pub const _0_X_1_E: Self = Self::new(30);

        pub const _0_X_1_F: Self = Self::new(31);

        pub const _0_X_20: Self = Self::new(32);

        pub const _0_X_21: Self = Self::new(33);

        pub const _0_X_22: Self = Self::new(34);

        pub const _0_X_23: Self = Self::new(35);
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
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ctsucalib::Ioc,
        ctsucalib::Ioc,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ctsucalib::Ioc,
            ctsucalib::Ioc,
            Ctsucalib_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfcrdmd(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        ctsucalib::Cfcrdmd,
        ctsucalib::Cfcrdmd,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            ctsucalib::Cfcrdmd,
            ctsucalib::Cfcrdmd,
            Ctsucalib_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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
    pub fn cfcmode(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        ctsucalib::Cfcmode,
        ctsucalib::Cfcmode,
        Ctsucalib_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            ctsucalib::Cfcmode,
            ctsucalib::Cfcmode,
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
    pub struct Ioc_SPEC;
    pub type Ioc = crate::EnumBitfieldStruct<u8, Ioc_SPEC>;
    impl Ioc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfcrdmd_SPEC;
    pub type Cfcrdmd = crate::EnumBitfieldStruct<u8, Cfcrdmd_SPEC>;
    impl Cfcrdmd {
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
    pub struct Cfcmode_SPEC;
    pub type Cfcmode = crate::EnumBitfieldStruct<u8, Cfcmode_SPEC>;
    impl Cfcmode {
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

impl NoBitfieldReg<Ctsusuclka_SPEC> for Ctsusuclka {}
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
pub struct Ctsucfccnt_SPEC;
impl crate::sealed::RegSpec for Ctsucfccnt_SPEC {
    type DataType = u32;
}

pub type Ctsucfccnt = crate::RegValueT<Ctsucfccnt_SPEC>;

impl Ctsucfccnt {
    #[inline(always)]
    pub fn cfccnt(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ctsucfccnt_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Ctsucfccnt_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctsucfccnt {
    #[inline(always)]
    fn default() -> Ctsucfccnt {
        <crate::RegValueT<Ctsucfccnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsucfccntl_SPEC;
impl crate::sealed::RegSpec for Ctsucfccntl_SPEC {
    type DataType = u16;
}

pub type Ctsucfccntl = crate::RegValueT<Ctsucfccntl_SPEC>;

impl NoBitfieldReg<Ctsucfccntl_SPEC> for Ctsucfccntl {}
impl ::core::default::Default for Ctsucfccntl {
    #[inline(always)]
    fn default() -> Ctsucfccntl {
        <crate::RegValueT<Ctsucfccntl_SPEC> as RegisterValue<_>>::new(0)
    }
}
