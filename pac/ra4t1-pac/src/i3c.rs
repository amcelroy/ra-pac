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
// Generated from SVD 1.20.00, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:19:04 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"I3C Bus Interface"]
unsafe impl ::core::marker::Send for super::I3C {}
unsafe impl ::core::marker::Sync for super::I3C {}
impl super::I3C {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn prts(&self) -> &'static crate::common::Reg<self::Prts_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Prts_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cectl(&self) -> &'static crate::common::Reg<self::Cectl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cectl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
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
    pub const fn msdvad(
        &self,
    ) -> &'static crate::common::Reg<self::Msdvad_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Msdvad_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
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
    pub const fn inst(&self) -> &'static crate::common::Reg<self::Inst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Inst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn inste(&self) -> &'static crate::common::Reg<self::Inste_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Inste_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn inie(&self) -> &'static crate::common::Reg<self::Inie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Inie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[inline(always)]
    pub const fn instfc(&self) -> &'static crate::common::Reg<self::Instfc_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Instfc_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dvct(&self) -> &'static crate::common::Reg<self::Dvct_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Dvct_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ibinctl(
        &self,
    ) -> &'static crate::common::Reg<self::Ibinctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ibinctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
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
    pub const fn bavlcdt(
        &self,
    ) -> &'static crate::common::Reg<self::Bavlcdt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bavlcdt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bidlcdt(
        &self,
    ) -> &'static crate::common::Reg<self::Bidlcdt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bidlcdt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
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
    pub const fn wuctl(&self) -> &'static crate::common::Reg<self::Wuctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Wuctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
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
    pub const fn scstlctl(
        &self,
    ) -> &'static crate::common::Reg<self::Scstlctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Scstlctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(176usize),
            )
        }
    }

    #[inline(always)]
    pub const fn svtdlg0(
        &self,
    ) -> &'static crate::common::Reg<self::Svtdlg0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Svtdlg0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }

    #[inline(always)]
    pub const fn stctl(&self) -> &'static crate::common::Reg<self::Stctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Stctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(288usize),
            )
        }
    }

    #[inline(always)]
    pub const fn atctl(&self) -> &'static crate::common::Reg<self::Atctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Atctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(292usize),
            )
        }
    }

    #[inline(always)]
    pub const fn attrg(&self) -> &'static crate::common::Reg<self::Attrg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Attrg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(296usize),
            )
        }
    }

    #[inline(always)]
    pub const fn atccnte(
        &self,
    ) -> &'static crate::common::Reg<self::Atccnte_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Atccnte_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(300usize),
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
    pub const fn ncmdqp(&self) -> &'static crate::common::Reg<self::Ncmdqp_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ncmdqp_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(336usize),
            )
        }
    }

    #[inline(always)]
    pub const fn nrspqp(&self) -> &'static crate::common::Reg<self::Nrspqp_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Nrspqp_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(340usize),
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
    pub const fn nibiqp(
        &self,
    ) -> &'static crate::common::Reg<self::Nibiqp_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Nibiqp_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(380usize),
            )
        }
    }

    #[inline(always)]
    pub const fn nrsqp(&self) -> &'static crate::common::Reg<self::Nrsqp_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Nrsqp_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(384usize),
            )
        }
    }

    #[inline(always)]
    pub const fn hcmdqp(&self) -> &'static crate::common::Reg<self::Hcmdqp_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Hcmdqp_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(388usize),
            )
        }
    }

    #[inline(always)]
    pub const fn hrspqp(&self) -> &'static crate::common::Reg<self::Hrspqp_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Hrspqp_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(392usize),
            )
        }
    }

    #[inline(always)]
    pub const fn htdtbp(
        &self,
    ) -> &'static crate::common::Reg<self::Htdtbp_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Htdtbp_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(396usize),
            )
        }
    }

    #[inline(always)]
    pub const fn nqthctl(
        &self,
    ) -> &'static crate::common::Reg<self::Nqthctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Nqthctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(400usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ntbthctl0(
        &self,
    ) -> &'static crate::common::Reg<self::Ntbthctl0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ntbthctl0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(404usize),
            )
        }
    }

    #[inline(always)]
    pub const fn nrqthctl(
        &self,
    ) -> &'static crate::common::Reg<self::Nrqthctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Nrqthctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(448usize),
            )
        }
    }

    #[inline(always)]
    pub const fn hqthctl(
        &self,
    ) -> &'static crate::common::Reg<self::Hqthctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Hqthctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(452usize),
            )
        }
    }

    #[inline(always)]
    pub const fn htbthctl(
        &self,
    ) -> &'static crate::common::Reg<self::Htbthctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Htbthctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(456usize),
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
    pub const fn ntstfc(
        &self,
    ) -> &'static crate::common::Reg<self::Ntstfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ntstfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(492usize),
            )
        }
    }

    #[inline(always)]
    pub const fn htst(&self) -> &'static crate::common::Reg<self::Htst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Htst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(512usize),
            )
        }
    }

    #[inline(always)]
    pub const fn htste(&self) -> &'static crate::common::Reg<self::Htste_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Htste_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(516usize),
            )
        }
    }

    #[inline(always)]
    pub const fn htie(&self) -> &'static crate::common::Reg<self::Htie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Htie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(520usize),
            )
        }
    }

    #[inline(always)]
    pub const fn htstfc(
        &self,
    ) -> &'static crate::common::Reg<self::Htstfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Htstfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(524usize),
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
    pub const fn wust(&self) -> &'static crate::common::Reg<self::Wust_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Wust_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(536usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mrccpt(&self) -> &'static crate::common::Reg<self::Mrccpt_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mrccpt_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(540usize),
            )
        }
    }

    #[inline(always)]
    pub const fn datbas(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Datbas_SPEC, crate::common::RW>,
        8,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x224usize))
        }
    }

    #[inline(always)]
    pub const fn exdatbas(
        &self,
    ) -> &'static crate::common::Reg<self::Exdatbas_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Exdatbas_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(672usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sdatbas0(
        &self,
    ) -> &'static crate::common::Reg<self::Sdatbas0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdatbas0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(688usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sdatbas1(
        &self,
    ) -> &'static crate::common::Reg<self::Sdatbas1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdatbas1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(688usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sdatbas2(
        &self,
    ) -> &'static crate::common::Reg<self::Sdatbas2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdatbas2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(688usize),
            )
        }
    }

    #[inline(always)]
    pub const fn msdct(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Msdct_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2d0usize))
        }
    }

    #[inline(always)]
    pub const fn svdct(&self) -> &'static crate::common::Reg<self::Svdct_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Svdct_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(800usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sdctpidl(
        &self,
    ) -> &'static crate::common::Reg<self::Sdctpidl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdctpidl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(804usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sdctpidh(
        &self,
    ) -> &'static crate::common::Reg<self::Sdctpidh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sdctpidh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(808usize),
            )
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
    pub const fn csecmd(
        &self,
    ) -> &'static crate::common::Reg<self::Csecmd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Csecmd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(848usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ceactst(
        &self,
    ) -> &'static crate::common::Reg<self::Ceactst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ceactst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(852usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cmwlg(&self) -> &'static crate::common::Reg<self::Cmwlg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cmwlg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(856usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cmrlg(&self) -> &'static crate::common::Reg<self::Cmrlg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cmrlg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(860usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cetstmd(
        &self,
    ) -> &'static crate::common::Reg<self::Cetstmd_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cetstmd_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(864usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cgdvst(
        &self,
    ) -> &'static crate::common::Reg<self::Cgdvst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cgdvst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(868usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cmdspw(
        &self,
    ) -> &'static crate::common::Reg<self::Cmdspw_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cmdspw_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(872usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cmdspr(
        &self,
    ) -> &'static crate::common::Reg<self::Cmdspr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cmdspr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(876usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cmdspt(
        &self,
    ) -> &'static crate::common::Reg<self::Cmdspt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cmdspt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(880usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cetsm(&self) -> &'static crate::common::Reg<self::Cetsm_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cetsm_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(884usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cetss(&self) -> &'static crate::common::Reg<self::Cetss_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cetss_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(888usize),
            )
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
    pub const fn nqstlv(&self) -> &'static crate::common::Reg<self::Nqstlv_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Nqstlv_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(916usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ndbstlv0(
        &self,
    ) -> &'static crate::common::Reg<self::Ndbstlv0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ndbstlv0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(920usize),
            )
        }
    }

    #[inline(always)]
    pub const fn nrsqstlv(
        &self,
    ) -> &'static crate::common::Reg<self::Nrsqstlv_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Nrsqstlv_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(960usize),
            )
        }
    }

    #[inline(always)]
    pub const fn hqstlv(&self) -> &'static crate::common::Reg<self::Hqstlv_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Hqstlv_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(964usize),
            )
        }
    }

    #[inline(always)]
    pub const fn hdbstlv(
        &self,
    ) -> &'static crate::common::Reg<self::Hdbstlv_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Hdbstlv_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(968usize),
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

    #[inline(always)]
    pub const fn mserrcnt(
        &self,
    ) -> &'static crate::common::Reg<self::Mserrcnt_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mserrcnt_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(976usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sc1cpt(&self) -> &'static crate::common::Reg<self::Sc1Cpt_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sc1Cpt_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(992usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sc2cpt(&self) -> &'static crate::common::Reg<self::Sc2Cpt_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sc2Cpt_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(996usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prts_SPEC;
impl crate::sealed::RegSpec for Prts_SPEC {
    type DataType = u32;
}

pub type Prts = crate::RegValueT<Prts_SPEC>;

impl Prts {
    #[inline(always)]
    pub fn prtmd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        prts::Prtmd,
        prts::Prtmd,
        Prts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            prts::Prtmd,
            prts::Prtmd,
            Prts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Prts {
    #[inline(always)]
    fn default() -> Prts {
        <crate::RegValueT<Prts_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod prts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prtmd_SPEC;
    pub type Prtmd = crate::EnumBitfieldStruct<u8, Prtmd_SPEC>;
    impl Prtmd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cectl_SPEC;
impl crate::sealed::RegSpec for Cectl_SPEC {
    type DataType = u32;
}

pub type Cectl = crate::RegValueT<Cectl_SPEC>;

impl Cectl {
    #[inline(always)]
    pub fn clke(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cectl::Clke,
        cectl::Clke,
        Cectl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cectl::Clke,
            cectl::Clke,
            Cectl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cectl {
    #[inline(always)]
    fn default() -> Cectl {
        <crate::RegValueT<Cectl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cectl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clke_SPEC;
    pub type Clke = crate::EnumBitfieldStruct<u8, Clke_SPEC>;
    impl Clke {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
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
    pub fn incba(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bctl::Incba,
        bctl::Incba,
        Bctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bctl::Incba,
            bctl::Incba,
            Bctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn abt(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        bctl::Abt,
        bctl::Abt,
        Bctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            bctl::Abt,
            bctl::Abt,
            Bctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rsm(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        bctl::Rsm,
        bctl::Rsm,
        Bctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            bctl::Rsm,
            bctl::Rsm,
            Bctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

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
    pub struct Incba_SPEC;
    pub type Incba = crate::EnumBitfieldStruct<u8, Incba_SPEC>;
    impl Incba {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Abt_SPEC;
    pub type Abt = crate::EnumBitfieldStruct<u8, Abt_SPEC>;
    impl Abt {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rsm_SPEC;
    pub type Rsm = crate::EnumBitfieldStruct<u8, Rsm_SPEC>;
    impl Rsm {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
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
pub struct Msdvad_SPEC;
impl crate::sealed::RegSpec for Msdvad_SPEC {
    type DataType = u32;
}

pub type Msdvad = crate::RegValueT<Msdvad_SPEC>;

impl Msdvad {
    #[inline(always)]
    pub fn mdyad(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, u8, Msdvad_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7f,1,0,u8,u8,Msdvad_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mdyadv(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        msdvad::Mdyadv,
        msdvad::Mdyadv,
        Msdvad_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            msdvad::Mdyadv,
            msdvad::Mdyadv,
            Msdvad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Msdvad {
    #[inline(always)]
    fn default() -> Msdvad {
        <crate::RegValueT<Msdvad_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod msdvad {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mdyadv_SPEC;
    pub type Mdyadv = crate::EnumBitfieldStruct<u8, Mdyadv_SPEC>;
    impl Mdyadv {
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
    pub fn ri3crst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        rstctl::Ri3Crst,
        rstctl::Ri3Crst,
        Rstctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            rstctl::Ri3Crst,
            rstctl::Ri3Crst,
            Rstctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmdqrst(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        rstctl::Cmdqrst,
        rstctl::Cmdqrst,
        Rstctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            rstctl::Cmdqrst,
            rstctl::Cmdqrst,
            Rstctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rspqrst(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        rstctl::Rspqrst,
        rstctl::Rspqrst,
        Rstctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            rstctl::Rspqrst,
            rstctl::Rspqrst,
            Rstctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tdbrst(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        rstctl::Tdbrst,
        rstctl::Tdbrst,
        Rstctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            rstctl::Tdbrst,
            rstctl::Tdbrst,
            Rstctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rdbrst(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        rstctl::Rdbrst,
        rstctl::Rdbrst,
        Rstctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            rstctl::Rdbrst,
            rstctl::Rdbrst,
            Rstctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ibiqrst(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        rstctl::Ibiqrst,
        rstctl::Ibiqrst,
        Rstctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            rstctl::Ibiqrst,
            rstctl::Ibiqrst,
            Rstctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rsqrst(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        rstctl::Rsqrst,
        rstctl::Rsqrst,
        Rstctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            rstctl::Rsqrst,
            rstctl::Rsqrst,
            Rstctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn hcmdqrst(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        rstctl::Hcmdqrst,
        rstctl::Hcmdqrst,
        Rstctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            rstctl::Hcmdqrst,
            rstctl::Hcmdqrst,
            Rstctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn hrspqrst(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        rstctl::Hrspqrst,
        rstctl::Hrspqrst,
        Rstctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            rstctl::Hrspqrst,
            rstctl::Hrspqrst,
            Rstctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn htdbrst(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        rstctl::Htdbrst,
        rstctl::Htdbrst,
        Rstctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            rstctl::Htdbrst,
            rstctl::Htdbrst,
            Rstctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn hrdbrst(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        rstctl::Hrdbrst,
        rstctl::Hrdbrst,
        Rstctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            rstctl::Hrdbrst,
            rstctl::Hrdbrst,
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
    pub struct Ri3Crst_SPEC;
    pub type Ri3Crst = crate::EnumBitfieldStruct<u8, Ri3Crst_SPEC>;
    impl Ri3Crst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmdqrst_SPEC;
    pub type Cmdqrst = crate::EnumBitfieldStruct<u8, Cmdqrst_SPEC>;
    impl Cmdqrst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rspqrst_SPEC;
    pub type Rspqrst = crate::EnumBitfieldStruct<u8, Rspqrst_SPEC>;
    impl Rspqrst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdbrst_SPEC;
    pub type Tdbrst = crate::EnumBitfieldStruct<u8, Tdbrst_SPEC>;
    impl Tdbrst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdbrst_SPEC;
    pub type Rdbrst = crate::EnumBitfieldStruct<u8, Rdbrst_SPEC>;
    impl Rdbrst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ibiqrst_SPEC;
    pub type Ibiqrst = crate::EnumBitfieldStruct<u8, Ibiqrst_SPEC>;
    impl Ibiqrst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rsqrst_SPEC;
    pub type Rsqrst = crate::EnumBitfieldStruct<u8, Rsqrst_SPEC>;
    impl Rsqrst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hcmdqrst_SPEC;
    pub type Hcmdqrst = crate::EnumBitfieldStruct<u8, Hcmdqrst_SPEC>;
    impl Hcmdqrst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hrspqrst_SPEC;
    pub type Hrspqrst = crate::EnumBitfieldStruct<u8, Hrspqrst_SPEC>;
    impl Hrspqrst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Htdbrst_SPEC;
    pub type Htdbrst = crate::EnumBitfieldStruct<u8, Htdbrst_SPEC>;
    impl Htdbrst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hrdbrst_SPEC;
    pub type Hrdbrst = crate::EnumBitfieldStruct<u8, Hrdbrst_SPEC>;
    impl Hrdbrst {
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
pub struct Inst_SPEC;
impl crate::sealed::RegSpec for Inst_SPEC {
    type DataType = u32;
}

pub type Inst = crate::RegValueT<Inst_SPEC>;

impl Inst {
    #[inline(always)]
    pub fn inef(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        inst::Inef,
        inst::Inef,
        Inst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            inst::Inef,
            inst::Inef,
            Inst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Inst {
    #[inline(always)]
    fn default() -> Inst {
        <crate::RegValueT<Inst_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod inst {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inef_SPEC;
    pub type Inef = crate::EnumBitfieldStruct<u8, Inef_SPEC>;
    impl Inef {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inste_SPEC;
impl crate::sealed::RegSpec for Inste_SPEC {
    type DataType = u32;
}

pub type Inste = crate::RegValueT<Inste_SPEC>;

impl Inste {
    #[inline(always)]
    pub fn inee(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        inste::Inee,
        inste::Inee,
        Inste_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            inste::Inee,
            inste::Inee,
            Inste_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Inste {
    #[inline(always)]
    fn default() -> Inste {
        <crate::RegValueT<Inste_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod inste {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inee_SPEC;
    pub type Inee = crate::EnumBitfieldStruct<u8, Inee_SPEC>;
    impl Inee {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inie_SPEC;
impl crate::sealed::RegSpec for Inie_SPEC {
    type DataType = u32;
}

pub type Inie = crate::RegValueT<Inie_SPEC>;

impl Inie {
    #[inline(always)]
    pub fn ineie(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        inie::Ineie,
        inie::Ineie,
        Inie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            inie::Ineie,
            inie::Ineie,
            Inie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Inie {
    #[inline(always)]
    fn default() -> Inie {
        <crate::RegValueT<Inie_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod inie {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ineie_SPEC;
    pub type Ineie = crate::EnumBitfieldStruct<u8, Ineie_SPEC>;
    impl Ineie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Instfc_SPEC;
impl crate::sealed::RegSpec for Instfc_SPEC {
    type DataType = u32;
}

pub type Instfc = crate::RegValueT<Instfc_SPEC>;

impl Instfc {
    #[inline(always)]
    pub fn inefc(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        instfc::Inefc,
        instfc::Inefc,
        Instfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            instfc::Inefc,
            instfc::Inefc,
            Instfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Instfc {
    #[inline(always)]
    fn default() -> Instfc {
        <crate::RegValueT<Instfc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod instfc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inefc_SPEC;
    pub type Inefc = crate::EnumBitfieldStruct<u8, Inefc_SPEC>;
    impl Inefc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dvct_SPEC;
impl crate::sealed::RegSpec for Dvct_SPEC {
    type DataType = u32;
}

pub type Dvct = crate::RegValueT<Dvct_SPEC>;

impl Dvct {
    #[inline(always)]
    pub fn idx(
        self,
    ) -> crate::common::RegisterField<19, 0x1f, 1, 0, u8, u8, Dvct_SPEC, crate::common::R> {
        crate::common::RegisterField::<19,0x1f,1,0,u8,u8,Dvct_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Dvct {
    #[inline(always)]
    fn default() -> Dvct {
        <crate::RegValueT<Dvct_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ibinctl_SPEC;
impl crate::sealed::RegSpec for Ibinctl_SPEC {
    type DataType = u32;
}

pub type Ibinctl = crate::RegValueT<Ibinctl_SPEC>;

impl Ibinctl {
    #[inline(always)]
    pub fn nrmrctl(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ibinctl::Nrmrctl,
        ibinctl::Nrmrctl,
        Ibinctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ibinctl::Nrmrctl,
            ibinctl::Nrmrctl,
            Ibinctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn nrsirctl(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ibinctl::Nrsirctl,
        ibinctl::Nrsirctl,
        Ibinctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ibinctl::Nrsirctl,
            ibinctl::Nrsirctl,
            Ibinctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ibinctl {
    #[inline(always)]
    fn default() -> Ibinctl {
        <crate::RegValueT<Ibinctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ibinctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nrmrctl_SPEC;
    pub type Nrmrctl = crate::EnumBitfieldStruct<u8, Nrmrctl_SPEC>;
    impl Nrmrctl {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nrsirctl_SPEC;
    pub type Nrsirctl = crate::EnumBitfieldStruct<u8, Nrsirctl_SPEC>;
    impl Nrsirctl {
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
    pub fn svae(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        svctl::Svae,
        svctl::Svae,
        Svctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            svctl::Svae,
            svctl::Svae,
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
    pub struct Svae_SPEC;
    pub type Svae = crate::EnumBitfieldStruct<u8, Svae_SPEC>;
    impl Svae {
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
    pub fn sbrlp(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, u8, Stdbr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3f,1,0,u8,u8,Stdbr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sbrhp(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, u8, Stdbr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3f,1,0,u8,u8,Stdbr_SPEC,crate::common::RW>::from_register(self,0)
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
        <crate::RegValueT<Stdbr_SPEC> as RegisterValue<_>>::new(1061158911)
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

    #[inline(always)]
    pub fn ebrlp(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, u8, Extbr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3f,1,0,u8,u8,Extbr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ebrhp(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, u8, Extbr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3f,1,0,u8,u8,Extbr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Extbr {
    #[inline(always)]
    fn default() -> Extbr {
        <crate::RegValueT<Extbr_SPEC> as RegisterValue<_>>::new(1061158911)
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
pub struct Bavlcdt_SPEC;
impl crate::sealed::RegSpec for Bavlcdt_SPEC {
    type DataType = u32;
}

pub type Bavlcdt = crate::RegValueT<Bavlcdt_SPEC>;

impl Bavlcdt {
    #[inline(always)]
    pub fn avlcyc(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Bavlcdt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Bavlcdt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bavlcdt {
    #[inline(always)]
    fn default() -> Bavlcdt {
        <crate::RegValueT<Bavlcdt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bidlcdt_SPEC;
impl crate::sealed::RegSpec for Bidlcdt_SPEC {
    type DataType = u32;
}

pub type Bidlcdt = crate::RegValueT<Bidlcdt_SPEC>;

impl Bidlcdt {
    #[inline(always)]
    pub fn idlcyc(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffff, 1, 0, u32, u32, Bidlcdt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ffff,1,0,u32,u32,Bidlcdt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bidlcdt {
    #[inline(always)]
    fn default() -> Bidlcdt {
        <crate::RegValueT<Bidlcdt_SPEC> as RegisterValue<_>>::new(0)
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
pub struct Wuctl_SPEC;
impl crate::sealed::RegSpec for Wuctl_SPEC {
    type DataType = u32;
}

pub type Wuctl = crate::RegValueT<Wuctl_SPEC>;

impl Wuctl {
    #[inline(always)]
    pub fn wuacks(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Wuctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Wuctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn wuanfs(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        wuctl::Wuanfs,
        wuctl::Wuanfs,
        Wuctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            wuctl::Wuanfs,
            wuctl::Wuanfs,
            Wuctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wufsyne(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        wuctl::Wufsyne,
        wuctl::Wufsyne,
        Wuctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            wuctl::Wufsyne,
            wuctl::Wufsyne,
            Wuctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wufe(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        wuctl::Wufe,
        wuctl::Wufe,
        Wuctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            wuctl::Wufe,
            wuctl::Wufe,
            Wuctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Wuctl {
    #[inline(always)]
    fn default() -> Wuctl {
        <crate::RegValueT<Wuctl_SPEC> as RegisterValue<_>>::new(65)
    }
}
pub mod wuctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wuanfs_SPEC;
    pub type Wuanfs = crate::EnumBitfieldStruct<u8, Wuanfs_SPEC>;
    impl Wuanfs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wufsyne_SPEC;
    pub type Wufsyne = crate::EnumBitfieldStruct<u8, Wufsyne_SPEC>;
    impl Wufsyne {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wufe_SPEC;
    pub type Wufe = crate::EnumBitfieldStruct<u8, Wufe_SPEC>;
    impl Wufe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
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
pub struct Scstlctl_SPEC;
impl crate::sealed::RegSpec for Scstlctl_SPEC {
    type DataType = u32;
}

pub type Scstlctl = crate::RegValueT<Scstlctl_SPEC>;

impl Scstlctl {
    #[inline(always)]
    pub fn stlcyc(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Scstlctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Scstlctl_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn aape(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        scstlctl::Aape,
        scstlctl::Aape,
        Scstlctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            scstlctl::Aape,
            scstlctl::Aape,
            Scstlctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn parpe(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        scstlctl::Parpe,
        scstlctl::Parpe,
        Scstlctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            scstlctl::Parpe,
            scstlctl::Parpe,
            Scstlctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ackpe(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        scstlctl::Ackpe,
        scstlctl::Ackpe,
        Scstlctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            scstlctl::Ackpe,
            scstlctl::Ackpe,
            Scstlctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Scstlctl {
    #[inline(always)]
    fn default() -> Scstlctl {
        <crate::RegValueT<Scstlctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod scstlctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aape_SPEC;
    pub type Aape = crate::EnumBitfieldStruct<u8, Aape_SPEC>;
    impl Aape {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Parpe_SPEC;
    pub type Parpe = crate::EnumBitfieldStruct<u8, Parpe_SPEC>;
    impl Parpe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ackpe_SPEC;
    pub type Ackpe = crate::EnumBitfieldStruct<u8, Ackpe_SPEC>;
    impl Ackpe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Svtdlg0_SPEC;
impl crate::sealed::RegSpec for Svtdlg0_SPEC {
    type DataType = u32;
}

pub type Svtdlg0 = crate::RegValueT<Svtdlg0_SPEC>;

impl Svtdlg0 {
    #[inline(always)]
    pub fn stdlg(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Svtdlg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Svtdlg0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Svtdlg0 {
    #[inline(always)]
    fn default() -> Svtdlg0 {
        <crate::RegValueT<Svtdlg0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stctl_SPEC;
impl crate::sealed::RegSpec for Stctl_SPEC {
    type DataType = u32;
}

pub type Stctl = crate::RegValueT<Stctl_SPEC>;

impl Stctl {
    #[inline(always)]
    pub fn stoe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        stctl::Stoe,
        stctl::Stoe,
        Stctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            stctl::Stoe,
            stctl::Stoe,
            Stctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Stctl {
    #[inline(always)]
    fn default() -> Stctl {
        <crate::RegValueT<Stctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod stctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stoe_SPEC;
    pub type Stoe = crate::EnumBitfieldStruct<u8, Stoe_SPEC>;
    impl Stoe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Atctl_SPEC;
impl crate::sealed::RegSpec for Atctl_SPEC {
    type DataType = u32;
}

pub type Atctl = crate::RegValueT<Atctl_SPEC>;

impl Atctl {
    #[inline(always)]
    pub fn attrgs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        atctl::Attrgs,
        atctl::Attrgs,
        Atctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            atctl::Attrgs,
            atctl::Attrgs,
            Atctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mrefoe(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        atctl::Mrefoe,
        atctl::Mrefoe,
        Atctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            atctl::Mrefoe,
            atctl::Mrefoe,
            Atctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ameoe(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        atctl::Ameoe,
        atctl::Ameoe,
        Atctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            atctl::Ameoe,
            atctl::Ameoe,
            Atctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cdiv(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Atctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Atctl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Atctl {
    #[inline(always)]
    fn default() -> Atctl {
        <crate::RegValueT<Atctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod atctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Attrgs_SPEC;
    pub type Attrgs = crate::EnumBitfieldStruct<u8, Attrgs_SPEC>;
    impl Attrgs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mrefoe_SPEC;
    pub type Mrefoe = crate::EnumBitfieldStruct<u8, Mrefoe_SPEC>;
    impl Mrefoe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ameoe_SPEC;
    pub type Ameoe = crate::EnumBitfieldStruct<u8, Ameoe_SPEC>;
    impl Ameoe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Attrg_SPEC;
impl crate::sealed::RegSpec for Attrg_SPEC {
    type DataType = u32;
}

pub type Attrg = crate::RegValueT<Attrg_SPEC>;

impl Attrg {
    #[inline(always)]
    pub fn atstrg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        attrg::Atstrg,
        attrg::Atstrg,
        Attrg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            attrg::Atstrg,
            attrg::Atstrg,
            Attrg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Attrg {
    #[inline(always)]
    fn default() -> Attrg {
        <crate::RegValueT<Attrg_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod attrg {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Atstrg_SPEC;
    pub type Atstrg = crate::EnumBitfieldStruct<u8, Atstrg_SPEC>;
    impl Atstrg {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Atccnte_SPEC;
impl crate::sealed::RegSpec for Atccnte_SPEC {
    type DataType = u32;
}

pub type Atccnte = crate::RegValueT<Atccnte_SPEC>;

impl Atccnte {
    #[inline(always)]
    pub fn atce(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        atccnte::Atce,
        atccnte::Atce,
        Atccnte_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            atccnte::Atce,
            atccnte::Atce,
            Atccnte_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Atccnte {
    #[inline(always)]
    fn default() -> Atccnte {
        <crate::RegValueT<Atccnte_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod atccnte {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Atce_SPEC;
    pub type Atce = crate::EnumBitfieldStruct<u8, Atce_SPEC>;
    impl Atce {
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
pub struct Ncmdqp_SPEC;
impl crate::sealed::RegSpec for Ncmdqp_SPEC {
    type DataType = u32;
}

pub type Ncmdqp = crate::RegValueT<Ncmdqp_SPEC>;

impl NoBitfieldReg<Ncmdqp_SPEC> for Ncmdqp {}
impl ::core::default::Default for Ncmdqp {
    #[inline(always)]
    fn default() -> Ncmdqp {
        <crate::RegValueT<Ncmdqp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nrspqp_SPEC;
impl crate::sealed::RegSpec for Nrspqp_SPEC {
    type DataType = u32;
}

pub type Nrspqp = crate::RegValueT<Nrspqp_SPEC>;

impl NoBitfieldReg<Nrspqp_SPEC> for Nrspqp {}
impl ::core::default::Default for Nrspqp {
    #[inline(always)]
    fn default() -> Nrspqp {
        <crate::RegValueT<Nrspqp_SPEC> as RegisterValue<_>>::new(0)
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
pub struct Nibiqp_SPEC;
impl crate::sealed::RegSpec for Nibiqp_SPEC {
    type DataType = u32;
}

pub type Nibiqp = crate::RegValueT<Nibiqp_SPEC>;

impl NoBitfieldReg<Nibiqp_SPEC> for Nibiqp {}
impl ::core::default::Default for Nibiqp {
    #[inline(always)]
    fn default() -> Nibiqp {
        <crate::RegValueT<Nibiqp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nrsqp_SPEC;
impl crate::sealed::RegSpec for Nrsqp_SPEC {
    type DataType = u32;
}

pub type Nrsqp = crate::RegValueT<Nrsqp_SPEC>;

impl NoBitfieldReg<Nrsqp_SPEC> for Nrsqp {}
impl ::core::default::Default for Nrsqp {
    #[inline(always)]
    fn default() -> Nrsqp {
        <crate::RegValueT<Nrsqp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hcmdqp_SPEC;
impl crate::sealed::RegSpec for Hcmdqp_SPEC {
    type DataType = u32;
}

pub type Hcmdqp = crate::RegValueT<Hcmdqp_SPEC>;

impl NoBitfieldReg<Hcmdqp_SPEC> for Hcmdqp {}
impl ::core::default::Default for Hcmdqp {
    #[inline(always)]
    fn default() -> Hcmdqp {
        <crate::RegValueT<Hcmdqp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hrspqp_SPEC;
impl crate::sealed::RegSpec for Hrspqp_SPEC {
    type DataType = u32;
}

pub type Hrspqp = crate::RegValueT<Hrspqp_SPEC>;

impl NoBitfieldReg<Hrspqp_SPEC> for Hrspqp {}
impl ::core::default::Default for Hrspqp {
    #[inline(always)]
    fn default() -> Hrspqp {
        <crate::RegValueT<Hrspqp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Htdtbp_SPEC;
impl crate::sealed::RegSpec for Htdtbp_SPEC {
    type DataType = u32;
}

pub type Htdtbp = crate::RegValueT<Htdtbp_SPEC>;

impl NoBitfieldReg<Htdtbp_SPEC> for Htdtbp {}
impl ::core::default::Default for Htdtbp {
    #[inline(always)]
    fn default() -> Htdtbp {
        <crate::RegValueT<Htdtbp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nqthctl_SPEC;
impl crate::sealed::RegSpec for Nqthctl_SPEC {
    type DataType = u32;
}

pub type Nqthctl = crate::RegValueT<Nqthctl_SPEC>;

impl Nqthctl {
    #[inline(always)]
    pub fn cmdqth(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        nqthctl::Cmdqth,
        nqthctl::Cmdqth,
        Nqthctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            nqthctl::Cmdqth,
            nqthctl::Cmdqth,
            Nqthctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rspqth(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        nqthctl::Rspqth,
        nqthctl::Rspqth,
        Nqthctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            nqthctl::Rspqth,
            nqthctl::Rspqth,
            Nqthctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ibidssz(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Nqthctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Nqthctl_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ibiqth(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        nqthctl::Ibiqth,
        nqthctl::Ibiqth,
        Nqthctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            nqthctl::Ibiqth,
            nqthctl::Ibiqth,
            Nqthctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Nqthctl {
    #[inline(always)]
    fn default() -> Nqthctl {
        <crate::RegValueT<Nqthctl_SPEC> as RegisterValue<_>>::new(16843009)
    }
}
pub mod nqthctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmdqth_SPEC;
    pub type Cmdqth = crate::EnumBitfieldStruct<u8, Cmdqth_SPEC>;
    impl Cmdqth {
        pub const _0_X_00: Self = Self::new(0);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rspqth_SPEC;
    pub type Rspqth = crate::EnumBitfieldStruct<u8, Rspqth_SPEC>;
    impl Rspqth {
        pub const _0_X_00: Self = Self::new(0);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ibiqth_SPEC;
    pub type Ibiqth = crate::EnumBitfieldStruct<u8, Ibiqth_SPEC>;
    impl Ibiqth {
        pub const _0_X_00: Self = Self::new(0);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ntbthctl0_SPEC;
impl crate::sealed::RegSpec for Ntbthctl0_SPEC {
    type DataType = u32;
}

pub type Ntbthctl0 = crate::RegValueT<Ntbthctl0_SPEC>;

impl Ntbthctl0 {
    #[inline(always)]
    pub fn txdbth(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        ntbthctl0::Txdbth,
        ntbthctl0::Txdbth,
        Ntbthctl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            ntbthctl0::Txdbth,
            ntbthctl0::Txdbth,
            Ntbthctl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rxdbth(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        ntbthctl0::Rxdbth,
        ntbthctl0::Rxdbth,
        Ntbthctl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            ntbthctl0::Rxdbth,
            ntbthctl0::Rxdbth,
            Ntbthctl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txstth(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        ntbthctl0::Txstth,
        ntbthctl0::Txstth,
        Ntbthctl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            ntbthctl0::Txstth,
            ntbthctl0::Txstth,
            Ntbthctl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rxstth(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        ntbthctl0::Rxstth,
        ntbthctl0::Rxstth,
        Ntbthctl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            ntbthctl0::Rxstth,
            ntbthctl0::Rxstth,
            Ntbthctl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ntbthctl0 {
    #[inline(always)]
    fn default() -> Ntbthctl0 {
        <crate::RegValueT<Ntbthctl0_SPEC> as RegisterValue<_>>::new(16843009)
    }
}
pub mod ntbthctl0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txdbth_SPEC;
    pub type Txdbth = crate::EnumBitfieldStruct<u8, Txdbth_SPEC>;
    impl Txdbth {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxdbth_SPEC;
    pub type Rxdbth = crate::EnumBitfieldStruct<u8, Rxdbth_SPEC>;
    impl Rxdbth {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txstth_SPEC;
    pub type Txstth = crate::EnumBitfieldStruct<u8, Txstth_SPEC>;
    impl Txstth {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxstth_SPEC;
    pub type Rxstth = crate::EnumBitfieldStruct<u8, Rxstth_SPEC>;
    impl Rxstth {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nrqthctl_SPEC;
impl crate::sealed::RegSpec for Nrqthctl_SPEC {
    type DataType = u32;
}

pub type Nrqthctl = crate::RegValueT<Nrqthctl_SPEC>;

impl Nrqthctl {
    #[inline(always)]
    pub fn rsqth(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        nrqthctl::Rsqth,
        nrqthctl::Rsqth,
        Nrqthctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            nrqthctl::Rsqth,
            nrqthctl::Rsqth,
            Nrqthctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Nrqthctl {
    #[inline(always)]
    fn default() -> Nrqthctl {
        <crate::RegValueT<Nrqthctl_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod nrqthctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rsqth_SPEC;
    pub type Rsqth = crate::EnumBitfieldStruct<u8, Rsqth_SPEC>;
    impl Rsqth {
        pub const _0_X_00: Self = Self::new(0);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hqthctl_SPEC;
impl crate::sealed::RegSpec for Hqthctl_SPEC {
    type DataType = u32;
}

pub type Hqthctl = crate::RegValueT<Hqthctl_SPEC>;

impl Hqthctl {
    #[inline(always)]
    pub fn cmdqth(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        hqthctl::Cmdqth,
        hqthctl::Cmdqth,
        Hqthctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            hqthctl::Cmdqth,
            hqthctl::Cmdqth,
            Hqthctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rspqth(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        hqthctl::Rspqth,
        hqthctl::Rspqth,
        Hqthctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            hqthctl::Rspqth,
            hqthctl::Rspqth,
            Hqthctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Hqthctl {
    #[inline(always)]
    fn default() -> Hqthctl {
        <crate::RegValueT<Hqthctl_SPEC> as RegisterValue<_>>::new(257)
    }
}
pub mod hqthctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmdqth_SPEC;
    pub type Cmdqth = crate::EnumBitfieldStruct<u8, Cmdqth_SPEC>;
    impl Cmdqth {
        pub const _0_X_00: Self = Self::new(0);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rspqth_SPEC;
    pub type Rspqth = crate::EnumBitfieldStruct<u8, Rspqth_SPEC>;
    impl Rspqth {
        pub const _0_X_00: Self = Self::new(0);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Htbthctl_SPEC;
impl crate::sealed::RegSpec for Htbthctl_SPEC {
    type DataType = u32;
}

pub type Htbthctl = crate::RegValueT<Htbthctl_SPEC>;

impl Htbthctl {
    #[inline(always)]
    pub fn txdbth(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        htbthctl::Txdbth,
        htbthctl::Txdbth,
        Htbthctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            htbthctl::Txdbth,
            htbthctl::Txdbth,
            Htbthctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rxdbth(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        htbthctl::Rxdbth,
        htbthctl::Rxdbth,
        Htbthctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            htbthctl::Rxdbth,
            htbthctl::Rxdbth,
            Htbthctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txstth(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        htbthctl::Txstth,
        htbthctl::Txstth,
        Htbthctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            htbthctl::Txstth,
            htbthctl::Txstth,
            Htbthctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rxstth(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        htbthctl::Rxstth,
        htbthctl::Rxstth,
        Htbthctl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            htbthctl::Rxstth,
            htbthctl::Rxstth,
            Htbthctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Htbthctl {
    #[inline(always)]
    fn default() -> Htbthctl {
        <crate::RegValueT<Htbthctl_SPEC> as RegisterValue<_>>::new(16843009)
    }
}
pub mod htbthctl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txdbth_SPEC;
    pub type Txdbth = crate::EnumBitfieldStruct<u8, Txdbth_SPEC>;
    impl Txdbth {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxdbth_SPEC;
    pub type Rxdbth = crate::EnumBitfieldStruct<u8, Rxdbth_SPEC>;
    impl Rxdbth {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txstth_SPEC;
    pub type Txstth = crate::EnumBitfieldStruct<u8, Txstth_SPEC>;
    impl Txstth {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxstth_SPEC;
    pub type Rxstth = crate::EnumBitfieldStruct<u8, Rxstth_SPEC>;
    impl Rxstth {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const OTHERS: Self = Self::new(0);
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
    pub fn hdrexdf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        bst::Hdrexdf,
        bst::Hdrexdf,
        Bst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            bst::Hdrexdf,
            bst::Hdrexdf,
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
    pub struct Hdrexdf_SPEC;
    pub type Hdrexdf = crate::EnumBitfieldStruct<u8, Hdrexdf_SPEC>;
    impl Hdrexdf {
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
    pub fn hdrexde(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        bste::Hdrexde,
        bste::Hdrexde,
        Bste_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            bste::Hdrexde,
            bste::Hdrexde,
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
    pub struct Hdrexde_SPEC;
    pub type Hdrexde = crate::EnumBitfieldStruct<u8, Hdrexde_SPEC>;
    impl Hdrexde {
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
    pub fn hdrexdie(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        bie::Hdrexdie,
        bie::Hdrexdie,
        Bie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            bie::Hdrexdie,
            bie::Hdrexdie,
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
    pub struct Hdrexdie_SPEC;
    pub type Hdrexdie = crate::EnumBitfieldStruct<u8, Hdrexdie_SPEC>;
    impl Hdrexdie {
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
    pub fn hdrexdfc(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        bstfc::Hdrexdfc,
        bstfc::Hdrexdfc,
        Bstfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            bstfc::Hdrexdfc,
            bstfc::Hdrexdfc,
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
    pub struct Hdrexdfc_SPEC;
    pub type Hdrexdfc = crate::EnumBitfieldStruct<u8, Hdrexdfc_SPEC>;
    impl Hdrexdfc {
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

    #[inline(always)]
    pub fn ibiqeff(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ntst::Ibiqeff,
        ntst::Ibiqeff,
        Ntst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ntst::Ibiqeff,
            ntst::Ibiqeff,
            Ntst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmdqef(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ntst::Cmdqef,
        ntst::Cmdqef,
        Ntst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ntst::Cmdqef,
            ntst::Cmdqef,
            Ntst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rspqff(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ntst::Rspqff,
        ntst::Rspqff,
        Ntst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ntst::Rspqff,
            ntst::Rspqff,
            Ntst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tabtf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ntst::Tabtf,
        ntst::Tabtf,
        Ntst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ntst::Tabtf,
            ntst::Tabtf,
            Ntst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tef(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ntst::Tef,
        ntst::Tef,
        Ntst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ntst::Tef,
            ntst::Tef,
            Ntst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rsqff(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        ntst::Rsqff,
        ntst::Rsqff,
        Ntst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            ntst::Rsqff,
            ntst::Rsqff,
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ibiqeff_SPEC;
    pub type Ibiqeff = crate::EnumBitfieldStruct<u8, Ibiqeff_SPEC>;
    impl Ibiqeff {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmdqef_SPEC;
    pub type Cmdqef = crate::EnumBitfieldStruct<u8, Cmdqef_SPEC>;
    impl Cmdqef {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rspqff_SPEC;
    pub type Rspqff = crate::EnumBitfieldStruct<u8, Rspqff_SPEC>;
    impl Rspqff {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tabtf_SPEC;
    pub type Tabtf = crate::EnumBitfieldStruct<u8, Tabtf_SPEC>;
    impl Tabtf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tef_SPEC;
    pub type Tef = crate::EnumBitfieldStruct<u8, Tef_SPEC>;
    impl Tef {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rsqff_SPEC;
    pub type Rsqff = crate::EnumBitfieldStruct<u8, Rsqff_SPEC>;
    impl Rsqff {
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

    #[inline(always)]
    pub fn ibiqefe(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ntste::Ibiqefe,
        ntste::Ibiqefe,
        Ntste_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ntste::Ibiqefe,
            ntste::Ibiqefe,
            Ntste_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmdqee(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ntste::Cmdqee,
        ntste::Cmdqee,
        Ntste_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ntste::Cmdqee,
            ntste::Cmdqee,
            Ntste_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rspqfe(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ntste::Rspqfe,
        ntste::Rspqfe,
        Ntste_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ntste::Rspqfe,
            ntste::Rspqfe,
            Ntste_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tabte(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ntste::Tabte,
        ntste::Tabte,
        Ntste_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ntste::Tabte,
            ntste::Tabte,
            Ntste_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tee(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ntste::Tee,
        ntste::Tee,
        Ntste_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ntste::Tee,
            ntste::Tee,
            Ntste_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rsqfe(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        ntste::Rsqfe,
        ntste::Rsqfe,
        Ntste_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            ntste::Rsqfe,
            ntste::Rsqfe,
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ibiqefe_SPEC;
    pub type Ibiqefe = crate::EnumBitfieldStruct<u8, Ibiqefe_SPEC>;
    impl Ibiqefe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmdqee_SPEC;
    pub type Cmdqee = crate::EnumBitfieldStruct<u8, Cmdqee_SPEC>;
    impl Cmdqee {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rspqfe_SPEC;
    pub type Rspqfe = crate::EnumBitfieldStruct<u8, Rspqfe_SPEC>;
    impl Rspqfe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tabte_SPEC;
    pub type Tabte = crate::EnumBitfieldStruct<u8, Tabte_SPEC>;
    impl Tabte {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tee_SPEC;
    pub type Tee = crate::EnumBitfieldStruct<u8, Tee_SPEC>;
    impl Tee {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rsqfe_SPEC;
    pub type Rsqfe = crate::EnumBitfieldStruct<u8, Rsqfe_SPEC>;
    impl Rsqfe {
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

    #[inline(always)]
    pub fn ibiqefie(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ntie::Ibiqefie,
        ntie::Ibiqefie,
        Ntie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ntie::Ibiqefie,
            ntie::Ibiqefie,
            Ntie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmdqeie(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ntie::Cmdqeie,
        ntie::Cmdqeie,
        Ntie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ntie::Cmdqeie,
            ntie::Cmdqeie,
            Ntie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rspqfie(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ntie::Rspqfie,
        ntie::Rspqfie,
        Ntie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ntie::Rspqfie,
            ntie::Rspqfie,
            Ntie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tabtie(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ntie::Tabtie,
        ntie::Tabtie,
        Ntie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ntie::Tabtie,
            ntie::Tabtie,
            Ntie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn teie(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ntie::Teie,
        ntie::Teie,
        Ntie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ntie::Teie,
            ntie::Teie,
            Ntie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rsqfie(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        ntie::Rsqfie,
        ntie::Rsqfie,
        Ntie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            ntie::Rsqfie,
            ntie::Rsqfie,
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ibiqefie_SPEC;
    pub type Ibiqefie = crate::EnumBitfieldStruct<u8, Ibiqefie_SPEC>;
    impl Ibiqefie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmdqeie_SPEC;
    pub type Cmdqeie = crate::EnumBitfieldStruct<u8, Cmdqeie_SPEC>;
    impl Cmdqeie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rspqfie_SPEC;
    pub type Rspqfie = crate::EnumBitfieldStruct<u8, Rspqfie_SPEC>;
    impl Rspqfie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tabtie_SPEC;
    pub type Tabtie = crate::EnumBitfieldStruct<u8, Tabtie_SPEC>;
    impl Tabtie {
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
    pub struct Rsqfie_SPEC;
    pub type Rsqfie = crate::EnumBitfieldStruct<u8, Rsqfie_SPEC>;
    impl Rsqfie {
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

    #[inline(always)]
    pub fn ibiqeffc(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ntstfc::Ibiqeffc,
        ntstfc::Ibiqeffc,
        Ntstfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ntstfc::Ibiqeffc,
            ntstfc::Ibiqeffc,
            Ntstfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmdqefc(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ntstfc::Cmdqefc,
        ntstfc::Cmdqefc,
        Ntstfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ntstfc::Cmdqefc,
            ntstfc::Cmdqefc,
            Ntstfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rspqffc(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ntstfc::Rspqffc,
        ntstfc::Rspqffc,
        Ntstfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ntstfc::Rspqffc,
            ntstfc::Rspqffc,
            Ntstfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tabtfc(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ntstfc::Tabtfc,
        ntstfc::Tabtfc,
        Ntstfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ntstfc::Tabtfc,
            ntstfc::Tabtfc,
            Ntstfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tefc(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ntstfc::Tefc,
        ntstfc::Tefc,
        Ntstfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ntstfc::Tefc,
            ntstfc::Tefc,
            Ntstfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rsqffc(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        ntstfc::Rsqffc,
        ntstfc::Rsqffc,
        Ntstfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            ntstfc::Rsqffc,
            ntstfc::Rsqffc,
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ibiqeffc_SPEC;
    pub type Ibiqeffc = crate::EnumBitfieldStruct<u8, Ibiqeffc_SPEC>;
    impl Ibiqeffc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmdqefc_SPEC;
    pub type Cmdqefc = crate::EnumBitfieldStruct<u8, Cmdqefc_SPEC>;
    impl Cmdqefc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rspqffc_SPEC;
    pub type Rspqffc = crate::EnumBitfieldStruct<u8, Rspqffc_SPEC>;
    impl Rspqffc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tabtfc_SPEC;
    pub type Tabtfc = crate::EnumBitfieldStruct<u8, Tabtfc_SPEC>;
    impl Tabtfc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tefc_SPEC;
    pub type Tefc = crate::EnumBitfieldStruct<u8, Tefc_SPEC>;
    impl Tefc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rsqffc_SPEC;
    pub type Rsqffc = crate::EnumBitfieldStruct<u8, Rsqffc_SPEC>;
    impl Rsqffc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Htst_SPEC;
impl crate::sealed::RegSpec for Htst_SPEC {
    type DataType = u32;
}

pub type Htst = crate::RegValueT<Htst_SPEC>;

impl Htst {
    #[inline(always)]
    pub fn tdbef(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        htst::Tdbef,
        htst::Tdbef,
        Htst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            htst::Tdbef,
            htst::Tdbef,
            Htst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rdbff(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        htst::Rdbff,
        htst::Rdbff,
        Htst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            htst::Rdbff,
            htst::Rdbff,
            Htst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmdqef(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        htst::Cmdqef,
        htst::Cmdqef,
        Htst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            htst::Cmdqef,
            htst::Cmdqef,
            Htst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rspqff(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        htst::Rspqff,
        htst::Rspqff,
        Htst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            htst::Rspqff,
            htst::Rspqff,
            Htst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tabtf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        htst::Tabtf,
        htst::Tabtf,
        Htst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            htst::Tabtf,
            htst::Tabtf,
            Htst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tef(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        htst::Tef,
        htst::Tef,
        Htst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            htst::Tef,
            htst::Tef,
            Htst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Htst {
    #[inline(always)]
    fn default() -> Htst {
        <crate::RegValueT<Htst_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod htst {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdbef_SPEC;
    pub type Tdbef = crate::EnumBitfieldStruct<u8, Tdbef_SPEC>;
    impl Tdbef {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdbff_SPEC;
    pub type Rdbff = crate::EnumBitfieldStruct<u8, Rdbff_SPEC>;
    impl Rdbff {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmdqef_SPEC;
    pub type Cmdqef = crate::EnumBitfieldStruct<u8, Cmdqef_SPEC>;
    impl Cmdqef {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rspqff_SPEC;
    pub type Rspqff = crate::EnumBitfieldStruct<u8, Rspqff_SPEC>;
    impl Rspqff {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tabtf_SPEC;
    pub type Tabtf = crate::EnumBitfieldStruct<u8, Tabtf_SPEC>;
    impl Tabtf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tef_SPEC;
    pub type Tef = crate::EnumBitfieldStruct<u8, Tef_SPEC>;
    impl Tef {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Htste_SPEC;
impl crate::sealed::RegSpec for Htste_SPEC {
    type DataType = u32;
}

pub type Htste = crate::RegValueT<Htste_SPEC>;

impl Htste {
    #[inline(always)]
    pub fn tdbee(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        htste::Tdbee,
        htste::Tdbee,
        Htste_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            htste::Tdbee,
            htste::Tdbee,
            Htste_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rdbfe(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        htste::Rdbfe,
        htste::Rdbfe,
        Htste_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            htste::Rdbfe,
            htste::Rdbfe,
            Htste_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmdqee(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        htste::Cmdqee,
        htste::Cmdqee,
        Htste_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            htste::Cmdqee,
            htste::Cmdqee,
            Htste_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rspqfe(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        htste::Rspqfe,
        htste::Rspqfe,
        Htste_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            htste::Rspqfe,
            htste::Rspqfe,
            Htste_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tabte(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        htste::Tabte,
        htste::Tabte,
        Htste_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            htste::Tabte,
            htste::Tabte,
            Htste_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tee(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        htste::Tee,
        htste::Tee,
        Htste_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            htste::Tee,
            htste::Tee,
            Htste_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Htste {
    #[inline(always)]
    fn default() -> Htste {
        <crate::RegValueT<Htste_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod htste {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdbee_SPEC;
    pub type Tdbee = crate::EnumBitfieldStruct<u8, Tdbee_SPEC>;
    impl Tdbee {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdbfe_SPEC;
    pub type Rdbfe = crate::EnumBitfieldStruct<u8, Rdbfe_SPEC>;
    impl Rdbfe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmdqee_SPEC;
    pub type Cmdqee = crate::EnumBitfieldStruct<u8, Cmdqee_SPEC>;
    impl Cmdqee {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rspqfe_SPEC;
    pub type Rspqfe = crate::EnumBitfieldStruct<u8, Rspqfe_SPEC>;
    impl Rspqfe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tabte_SPEC;
    pub type Tabte = crate::EnumBitfieldStruct<u8, Tabte_SPEC>;
    impl Tabte {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tee_SPEC;
    pub type Tee = crate::EnumBitfieldStruct<u8, Tee_SPEC>;
    impl Tee {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Htie_SPEC;
impl crate::sealed::RegSpec for Htie_SPEC {
    type DataType = u32;
}

pub type Htie = crate::RegValueT<Htie_SPEC>;

impl Htie {
    #[inline(always)]
    pub fn tdbeie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        htie::Tdbeie,
        htie::Tdbeie,
        Htie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            htie::Tdbeie,
            htie::Tdbeie,
            Htie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rdbfie(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        htie::Rdbfie,
        htie::Rdbfie,
        Htie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            htie::Rdbfie,
            htie::Rdbfie,
            Htie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmdqeie(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        htie::Cmdqeie,
        htie::Cmdqeie,
        Htie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            htie::Cmdqeie,
            htie::Cmdqeie,
            Htie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rspqfie(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        htie::Rspqfie,
        htie::Rspqfie,
        Htie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            htie::Rspqfie,
            htie::Rspqfie,
            Htie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tabtie(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        htie::Tabtie,
        htie::Tabtie,
        Htie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            htie::Tabtie,
            htie::Tabtie,
            Htie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn teie(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        htie::Teie,
        htie::Teie,
        Htie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            htie::Teie,
            htie::Teie,
            Htie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Htie {
    #[inline(always)]
    fn default() -> Htie {
        <crate::RegValueT<Htie_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod htie {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdbeie_SPEC;
    pub type Tdbeie = crate::EnumBitfieldStruct<u8, Tdbeie_SPEC>;
    impl Tdbeie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdbfie_SPEC;
    pub type Rdbfie = crate::EnumBitfieldStruct<u8, Rdbfie_SPEC>;
    impl Rdbfie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmdqeie_SPEC;
    pub type Cmdqeie = crate::EnumBitfieldStruct<u8, Cmdqeie_SPEC>;
    impl Cmdqeie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rspqfie_SPEC;
    pub type Rspqfie = crate::EnumBitfieldStruct<u8, Rspqfie_SPEC>;
    impl Rspqfie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tabtie_SPEC;
    pub type Tabtie = crate::EnumBitfieldStruct<u8, Tabtie_SPEC>;
    impl Tabtie {
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
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Htstfc_SPEC;
impl crate::sealed::RegSpec for Htstfc_SPEC {
    type DataType = u32;
}

pub type Htstfc = crate::RegValueT<Htstfc_SPEC>;

impl Htstfc {
    #[inline(always)]
    pub fn tdbefc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        htstfc::Tdbefc,
        htstfc::Tdbefc,
        Htstfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            htstfc::Tdbefc,
            htstfc::Tdbefc,
            Htstfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rdbffc(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        htstfc::Rdbffc,
        htstfc::Rdbffc,
        Htstfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            htstfc::Rdbffc,
            htstfc::Rdbffc,
            Htstfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmdqefc(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        htstfc::Cmdqefc,
        htstfc::Cmdqefc,
        Htstfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            htstfc::Cmdqefc,
            htstfc::Cmdqefc,
            Htstfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rspqffc(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        htstfc::Rspqffc,
        htstfc::Rspqffc,
        Htstfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            htstfc::Rspqffc,
            htstfc::Rspqffc,
            Htstfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tabtfc(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        htstfc::Tabtfc,
        htstfc::Tabtfc,
        Htstfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            htstfc::Tabtfc,
            htstfc::Tabtfc,
            Htstfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tefc(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        htstfc::Tefc,
        htstfc::Tefc,
        Htstfc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            htstfc::Tefc,
            htstfc::Tefc,
            Htstfc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Htstfc {
    #[inline(always)]
    fn default() -> Htstfc {
        <crate::RegValueT<Htstfc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod htstfc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdbefc_SPEC;
    pub type Tdbefc = crate::EnumBitfieldStruct<u8, Tdbefc_SPEC>;
    impl Tdbefc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdbffc_SPEC;
    pub type Rdbffc = crate::EnumBitfieldStruct<u8, Rdbffc_SPEC>;
    impl Rdbffc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmdqefc_SPEC;
    pub type Cmdqefc = crate::EnumBitfieldStruct<u8, Cmdqefc_SPEC>;
    impl Cmdqefc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rspqffc_SPEC;
    pub type Rspqffc = crate::EnumBitfieldStruct<u8, Rspqffc_SPEC>;
    impl Rspqffc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tabtfc_SPEC;
    pub type Tabtfc = crate::EnumBitfieldStruct<u8, Tabtfc_SPEC>;
    impl Tabtfc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tefc_SPEC;
    pub type Tefc = crate::EnumBitfieldStruct<u8, Tefc_SPEC>;
    impl Tefc {
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

    #[inline(always)]
    pub fn bavlf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        bcst::Bavlf,
        bcst::Bavlf,
        Bcst_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            bcst::Bavlf,
            bcst::Bavlf,
            Bcst_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bidlf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        bcst::Bidlf,
        bcst::Bidlf,
        Bcst_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            bcst::Bidlf,
            bcst::Bidlf,
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bavlf_SPEC;
    pub type Bavlf = crate::EnumBitfieldStruct<u8, Bavlf_SPEC>;
    impl Bavlf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bidlf_SPEC;
    pub type Bidlf = crate::EnumBitfieldStruct<u8, Bidlf_SPEC>;
    impl Bidlf {
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
    pub fn svaf(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        svst::Svaf,
        svst::Svaf,
        Svst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            svst::Svaf,
            svst::Svaf,
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
    pub struct Svaf_SPEC;
    pub type Svaf = crate::EnumBitfieldStruct<u8, Svaf_SPEC>;
    impl Svaf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wust_SPEC;
impl crate::sealed::RegSpec for Wust_SPEC {
    type DataType = u32;
}

pub type Wust = crate::RegValueT<Wust_SPEC>;

impl Wust {
    #[inline(always)]
    pub fn wuasynf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        wust::Wuasynf,
        wust::Wuasynf,
        Wust_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            wust::Wuasynf,
            wust::Wuasynf,
            Wust_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Wust {
    #[inline(always)]
    fn default() -> Wust {
        <crate::RegValueT<Wust_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod wust {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wuasynf_SPEC;
    pub type Wuasynf = crate::EnumBitfieldStruct<u8, Wuasynf_SPEC>;
    impl Wuasynf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrccpt_SPEC;
impl crate::sealed::RegSpec for Mrccpt_SPEC {
    type DataType = u32;
}

pub type Mrccpt = crate::RegValueT<Mrccpt_SPEC>;

impl NoBitfieldReg<Mrccpt_SPEC> for Mrccpt {}
impl ::core::default::Default for Mrccpt {
    #[inline(always)]
    fn default() -> Mrccpt {
        <crate::RegValueT<Mrccpt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Datbas_SPEC;
impl crate::sealed::RegSpec for Datbas_SPEC {
    type DataType = u32;
}

pub type Datbas = crate::RegValueT<Datbas_SPEC>;

impl Datbas {
    #[inline(always)]
    pub fn dvstad(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, Datbas_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,Datbas_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dvibipl(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        datbas::Dvibipl,
        datbas::Dvibipl,
        Datbas_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            datbas::Dvibipl,
            datbas::Dvibipl,
            Datbas_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dvsirrj(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        datbas::Dvsirrj,
        datbas::Dvsirrj,
        Datbas_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            datbas::Dvsirrj,
            datbas::Dvsirrj,
            Datbas_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dvmrrj(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        datbas::Dvmrrj,
        datbas::Dvmrrj,
        Datbas_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            datbas::Dvmrrj,
            datbas::Dvmrrj,
            Datbas_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dvibits(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        datbas::Dvibits,
        datbas::Dvibits,
        Datbas_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            datbas::Dvibits,
            datbas::Dvibits,
            Datbas_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dvdyad(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Datbas_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Datbas_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dvnack(
        self,
    ) -> crate::common::RegisterField<29, 0x3, 1, 0, u8, u8, Datbas_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x3,1,0,u8,u8,Datbas_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dvtyp(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        datbas::Dvtyp,
        datbas::Dvtyp,
        Datbas_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            datbas::Dvtyp,
            datbas::Dvtyp,
            Datbas_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Datbas {
    #[inline(always)]
    fn default() -> Datbas {
        <crate::RegValueT<Datbas_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod datbas {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvibipl_SPEC;
    pub type Dvibipl = crate::EnumBitfieldStruct<u8, Dvibipl_SPEC>;
    impl Dvibipl {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvsirrj_SPEC;
    pub type Dvsirrj = crate::EnumBitfieldStruct<u8, Dvsirrj_SPEC>;
    impl Dvsirrj {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvmrrj_SPEC;
    pub type Dvmrrj = crate::EnumBitfieldStruct<u8, Dvmrrj_SPEC>;
    impl Dvmrrj {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvibits_SPEC;
    pub type Dvibits = crate::EnumBitfieldStruct<u8, Dvibits_SPEC>;
    impl Dvibits {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvtyp_SPEC;
    pub type Dvtyp = crate::EnumBitfieldStruct<u8, Dvtyp_SPEC>;
    impl Dvtyp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Exdatbas_SPEC;
impl crate::sealed::RegSpec for Exdatbas_SPEC {
    type DataType = u32;
}

pub type Exdatbas = crate::RegValueT<Exdatbas_SPEC>;

impl Exdatbas {
    #[inline(always)]
    pub fn edstad(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, Exdatbas_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,Exdatbas_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eddyad(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Exdatbas_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Exdatbas_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ednack(
        self,
    ) -> crate::common::RegisterField<29, 0x3, 1, 0, u8, u8, Exdatbas_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x3,1,0,u8,u8,Exdatbas_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn edtyp(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        exdatbas::Edtyp,
        exdatbas::Edtyp,
        Exdatbas_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            exdatbas::Edtyp,
            exdatbas::Edtyp,
            Exdatbas_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Exdatbas {
    #[inline(always)]
    fn default() -> Exdatbas {
        <crate::RegValueT<Exdatbas_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod exdatbas {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Edtyp_SPEC;
    pub type Edtyp = crate::EnumBitfieldStruct<u8, Edtyp_SPEC>;
    impl Edtyp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdatbas0_SPEC;
impl crate::sealed::RegSpec for Sdatbas0_SPEC {
    type DataType = u32;
}

pub type Sdatbas0 = crate::RegValueT<Sdatbas0_SPEC>;

impl Sdatbas0 {
    #[inline(always)]
    pub fn sdstad(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Sdatbas0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Sdatbas0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sdadls(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        sdatbas0::Sdadls,
        sdatbas0::Sdadls,
        Sdatbas0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            sdatbas0::Sdadls,
            sdatbas0::Sdadls,
            Sdatbas0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sdibipl(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        sdatbas0::Sdibipl,
        sdatbas0::Sdibipl,
        Sdatbas0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            sdatbas0::Sdibipl,
            sdatbas0::Sdibipl,
            Sdatbas0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sddyad(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, u8, Sdatbas0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7f,1,0,u8,u8,Sdatbas0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sdatbas0 {
    #[inline(always)]
    fn default() -> Sdatbas0 {
        <crate::RegValueT<Sdatbas0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdatbas0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdadls_SPEC;
    pub type Sdadls = crate::EnumBitfieldStruct<u8, Sdadls_SPEC>;
    impl Sdadls {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdibipl_SPEC;
    pub type Sdibipl = crate::EnumBitfieldStruct<u8, Sdibipl_SPEC>;
    impl Sdibipl {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdatbas1_SPEC;
impl crate::sealed::RegSpec for Sdatbas1_SPEC {
    type DataType = u32;
}

pub type Sdatbas1 = crate::RegValueT<Sdatbas1_SPEC>;

impl Sdatbas1 {
    #[inline(always)]
    pub fn sdstad(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Sdatbas1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Sdatbas1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sdadls(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        sdatbas1::Sdadls,
        sdatbas1::Sdadls,
        Sdatbas1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            sdatbas1::Sdadls,
            sdatbas1::Sdadls,
            Sdatbas1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sdibipl(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        sdatbas1::Sdibipl,
        sdatbas1::Sdibipl,
        Sdatbas1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            sdatbas1::Sdibipl,
            sdatbas1::Sdibipl,
            Sdatbas1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sddyad(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, u8, Sdatbas1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7f,1,0,u8,u8,Sdatbas1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sdatbas1 {
    #[inline(always)]
    fn default() -> Sdatbas1 {
        <crate::RegValueT<Sdatbas1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdatbas1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdadls_SPEC;
    pub type Sdadls = crate::EnumBitfieldStruct<u8, Sdadls_SPEC>;
    impl Sdadls {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdibipl_SPEC;
    pub type Sdibipl = crate::EnumBitfieldStruct<u8, Sdibipl_SPEC>;
    impl Sdibipl {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdatbas2_SPEC;
impl crate::sealed::RegSpec for Sdatbas2_SPEC {
    type DataType = u32;
}

pub type Sdatbas2 = crate::RegValueT<Sdatbas2_SPEC>;

impl Sdatbas2 {
    #[inline(always)]
    pub fn sdstad(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Sdatbas2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Sdatbas2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sdadls(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        sdatbas2::Sdadls,
        sdatbas2::Sdadls,
        Sdatbas2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            sdatbas2::Sdadls,
            sdatbas2::Sdadls,
            Sdatbas2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sdibipl(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        sdatbas2::Sdibipl,
        sdatbas2::Sdibipl,
        Sdatbas2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            sdatbas2::Sdibipl,
            sdatbas2::Sdibipl,
            Sdatbas2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sddyad(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, u8, Sdatbas2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7f,1,0,u8,u8,Sdatbas2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sdatbas2 {
    #[inline(always)]
    fn default() -> Sdatbas2 {
        <crate::RegValueT<Sdatbas2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sdatbas2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdadls_SPEC;
    pub type Sdadls = crate::EnumBitfieldStruct<u8, Sdadls_SPEC>;
    impl Sdadls {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdibipl_SPEC;
    pub type Sdibipl = crate::EnumBitfieldStruct<u8, Sdibipl_SPEC>;
    impl Sdibipl {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msdct_SPEC;
impl crate::sealed::RegSpec for Msdct_SPEC {
    type DataType = u32;
}

pub type Msdct = crate::RegValueT<Msdct_SPEC>;

impl Msdct {
    #[inline(always)]
    pub fn rbcr0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        msdct::Rbcr0,
        msdct::Rbcr0,
        Msdct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            msdct::Rbcr0,
            msdct::Rbcr0,
            Msdct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rbcr1(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        msdct::Rbcr1,
        msdct::Rbcr1,
        Msdct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            msdct::Rbcr1,
            msdct::Rbcr1,
            Msdct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rbcr2(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        msdct::Rbcr2,
        msdct::Rbcr2,
        Msdct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            msdct::Rbcr2,
            msdct::Rbcr2,
            Msdct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rbcr3(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        msdct::Rbcr3,
        msdct::Rbcr3,
        Msdct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            msdct::Rbcr3,
            msdct::Rbcr3,
            Msdct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rbcr4(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        msdct::Rbcr4,
        msdct::Rbcr4,
        Msdct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            msdct::Rbcr4,
            msdct::Rbcr4,
            Msdct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rbcr76(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        msdct::Rbcr76,
        msdct::Rbcr76,
        Msdct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            msdct::Rbcr76,
            msdct::Rbcr76,
            Msdct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Msdct {
    #[inline(always)]
    fn default() -> Msdct {
        <crate::RegValueT<Msdct_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod msdct {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rbcr0_SPEC;
    pub type Rbcr0 = crate::EnumBitfieldStruct<u8, Rbcr0_SPEC>;
    impl Rbcr0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rbcr1_SPEC;
    pub type Rbcr1 = crate::EnumBitfieldStruct<u8, Rbcr1_SPEC>;
    impl Rbcr1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rbcr2_SPEC;
    pub type Rbcr2 = crate::EnumBitfieldStruct<u8, Rbcr2_SPEC>;
    impl Rbcr2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rbcr3_SPEC;
    pub type Rbcr3 = crate::EnumBitfieldStruct<u8, Rbcr3_SPEC>;
    impl Rbcr3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rbcr4_SPEC;
    pub type Rbcr4 = crate::EnumBitfieldStruct<u8, Rbcr4_SPEC>;
    impl Rbcr4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rbcr76_SPEC;
    pub type Rbcr76 = crate::EnumBitfieldStruct<u8, Rbcr76_SPEC>;
    impl Rbcr76 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Svdct_SPEC;
impl crate::sealed::RegSpec for Svdct_SPEC {
    type DataType = u32;
}

pub type Svdct = crate::RegValueT<Svdct_SPEC>;

impl Svdct {
    #[inline(always)]
    pub fn tdcr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Svdct_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Svdct_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tbcr0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        svdct::Tbcr0,
        svdct::Tbcr0,
        Svdct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            svdct::Tbcr0,
            svdct::Tbcr0,
            Svdct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tbcr1(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        svdct::Tbcr1,
        svdct::Tbcr1,
        Svdct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            svdct::Tbcr1,
            svdct::Tbcr1,
            Svdct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tbcr2(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        svdct::Tbcr2,
        svdct::Tbcr2,
        Svdct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            svdct::Tbcr2,
            svdct::Tbcr2,
            Svdct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tbcr3(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        svdct::Tbcr3,
        svdct::Tbcr3,
        Svdct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            svdct::Tbcr3,
            svdct::Tbcr3,
            Svdct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tbcr4(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        svdct::Tbcr4,
        svdct::Tbcr4,
        Svdct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            svdct::Tbcr4,
            svdct::Tbcr4,
            Svdct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tbcr76(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        svdct::Tbcr76,
        svdct::Tbcr76,
        Svdct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            svdct::Tbcr76,
            svdct::Tbcr76,
            Svdct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Svdct {
    #[inline(always)]
    fn default() -> Svdct {
        <crate::RegValueT<Svdct_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod svdct {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tbcr0_SPEC;
    pub type Tbcr0 = crate::EnumBitfieldStruct<u8, Tbcr0_SPEC>;
    impl Tbcr0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tbcr1_SPEC;
    pub type Tbcr1 = crate::EnumBitfieldStruct<u8, Tbcr1_SPEC>;
    impl Tbcr1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tbcr2_SPEC;
    pub type Tbcr2 = crate::EnumBitfieldStruct<u8, Tbcr2_SPEC>;
    impl Tbcr2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tbcr3_SPEC;
    pub type Tbcr3 = crate::EnumBitfieldStruct<u8, Tbcr3_SPEC>;
    impl Tbcr3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tbcr4_SPEC;
    pub type Tbcr4 = crate::EnumBitfieldStruct<u8, Tbcr4_SPEC>;
    impl Tbcr4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tbcr76_SPEC;
    pub type Tbcr76 = crate::EnumBitfieldStruct<u8, Tbcr76_SPEC>;
    impl Tbcr76 {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdctpidl_SPEC;
impl crate::sealed::RegSpec for Sdctpidl_SPEC {
    type DataType = u32;
}

pub type Sdctpidl = crate::RegValueT<Sdctpidl_SPEC>;

impl NoBitfieldReg<Sdctpidl_SPEC> for Sdctpidl {}
impl ::core::default::Default for Sdctpidl {
    #[inline(always)]
    fn default() -> Sdctpidl {
        <crate::RegValueT<Sdctpidl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdctpidh_SPEC;
impl crate::sealed::RegSpec for Sdctpidh_SPEC {
    type DataType = u32;
}

pub type Sdctpidh = crate::RegValueT<Sdctpidh_SPEC>;

impl NoBitfieldReg<Sdctpidh_SPEC> for Sdctpidh {}
impl ::core::default::Default for Sdctpidh {
    #[inline(always)]
    fn default() -> Sdctpidh {
        <crate::RegValueT<Sdctpidh_SPEC> as RegisterValue<_>>::new(0)
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

    #[inline(always)]
    pub fn sdyadv(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        svdvad::Sdyadv,
        svdvad::Sdyadv,
        Svdvad_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            svdvad::Sdyadv,
            svdvad::Sdyadv,
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdyadv_SPEC;
    pub type Sdyadv = crate::EnumBitfieldStruct<u8, Sdyadv_SPEC>;
    impl Sdyadv {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csecmd_SPEC;
impl crate::sealed::RegSpec for Csecmd_SPEC {
    type DataType = u32;
}

pub type Csecmd = crate::RegValueT<Csecmd_SPEC>;

impl Csecmd {
    #[inline(always)]
    pub fn svirqe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        csecmd::Svirqe,
        csecmd::Svirqe,
        Csecmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            csecmd::Svirqe,
            csecmd::Svirqe,
            Csecmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn msrqe(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        csecmd::Msrqe,
        csecmd::Msrqe,
        Csecmd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            csecmd::Msrqe,
            csecmd::Msrqe,
            Csecmd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Csecmd {
    #[inline(always)]
    fn default() -> Csecmd {
        <crate::RegValueT<Csecmd_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod csecmd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Svirqe_SPEC;
    pub type Svirqe = crate::EnumBitfieldStruct<u8, Svirqe_SPEC>;
    impl Svirqe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Msrqe_SPEC;
    pub type Msrqe = crate::EnumBitfieldStruct<u8, Msrqe_SPEC>;
    impl Msrqe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ceactst_SPEC;
impl crate::sealed::RegSpec for Ceactst_SPEC {
    type DataType = u32;
}

pub type Ceactst = crate::RegValueT<Ceactst_SPEC>;

impl Ceactst {
    #[inline(always)]
    pub fn actst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        ceactst::Actst,
        ceactst::Actst,
        Ceactst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            ceactst::Actst,
            ceactst::Actst,
            Ceactst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ceactst {
    #[inline(always)]
    fn default() -> Ceactst {
        <crate::RegValueT<Ceactst_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ceactst {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Actst_SPEC;
    pub type Actst = crate::EnumBitfieldStruct<u8, Actst_SPEC>;
    impl Actst {
        pub const _0_X_1: Self = Self::new(1);

        pub const _0_X_2: Self = Self::new(2);

        pub const _0_X_4: Self = Self::new(4);

        pub const _0_X_8: Self = Self::new(8);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmwlg_SPEC;
impl crate::sealed::RegSpec for Cmwlg_SPEC {
    type DataType = u32;
}

pub type Cmwlg = crate::RegValueT<Cmwlg_SPEC>;

impl Cmwlg {
    #[inline(always)]
    pub fn mwlg(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Cmwlg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Cmwlg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cmwlg {
    #[inline(always)]
    fn default() -> Cmwlg {
        <crate::RegValueT<Cmwlg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmrlg_SPEC;
impl crate::sealed::RegSpec for Cmrlg_SPEC {
    type DataType = u32;
}

pub type Cmrlg = crate::RegValueT<Cmrlg_SPEC>;

impl Cmrlg {
    #[inline(always)]
    pub fn mrlg(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Cmrlg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Cmrlg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ibipsz(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cmrlg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cmrlg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cmrlg {
    #[inline(always)]
    fn default() -> Cmrlg {
        <crate::RegValueT<Cmrlg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cetstmd_SPEC;
impl crate::sealed::RegSpec for Cetstmd_SPEC {
    type DataType = u32;
}

pub type Cetstmd = crate::RegValueT<Cetstmd_SPEC>;

impl Cetstmd {
    #[inline(always)]
    pub fn tstmd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        cetstmd::Tstmd,
        cetstmd::Tstmd,
        Cetstmd_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            cetstmd::Tstmd,
            cetstmd::Tstmd,
            Cetstmd_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cetstmd {
    #[inline(always)]
    fn default() -> Cetstmd {
        <crate::RegValueT<Cetstmd_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cetstmd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tstmd_SPEC;
    pub type Tstmd = crate::EnumBitfieldStruct<u8, Tstmd_SPEC>;
    impl Tstmd {
        pub const _0_X_00: Self = Self::new(0);

        pub const _0_X_01: Self = Self::new(1);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cgdvst_SPEC;
impl crate::sealed::RegSpec for Cgdvst_SPEC {
    type DataType = u32;
}

pub type Cgdvst = crate::RegValueT<Cgdvst_SPEC>;

impl Cgdvst {
    #[inline(always)]
    pub fn pndint(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Cgdvst_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Cgdvst_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn prte(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        cgdvst::Prte,
        cgdvst::Prte,
        Cgdvst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            cgdvst::Prte,
            cgdvst::Prte,
            Cgdvst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn actmd(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        cgdvst::Actmd,
        cgdvst::Actmd,
        Cgdvst_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            cgdvst::Actmd,
            cgdvst::Actmd,
            Cgdvst_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn vdrsv(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cgdvst_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cgdvst_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cgdvst {
    #[inline(always)]
    fn default() -> Cgdvst {
        <crate::RegValueT<Cgdvst_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cgdvst {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prte_SPEC;
    pub type Prte = crate::EnumBitfieldStruct<u8, Prte_SPEC>;
    impl Prte {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Actmd_SPEC;
    pub type Actmd = crate::EnumBitfieldStruct<u8, Actmd_SPEC>;
    impl Actmd {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdspw_SPEC;
impl crate::sealed::RegSpec for Cmdspw_SPEC {
    type DataType = u32;
}

pub type Cmdspw = crate::RegValueT<Cmdspw_SPEC>;

impl Cmdspw {
    #[inline(always)]
    pub fn mswdr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        cmdspw::Mswdr,
        cmdspw::Mswdr,
        Cmdspw_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            cmdspw::Mswdr,
            cmdspw::Mswdr,
            Cmdspw_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cmdspw {
    #[inline(always)]
    fn default() -> Cmdspw {
        <crate::RegValueT<Cmdspw_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cmdspw {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mswdr_SPEC;
    pub type Mswdr = crate::EnumBitfieldStruct<u8, Mswdr_SPEC>;
    impl Mswdr {
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
pub struct Cmdspr_SPEC;
impl crate::sealed::RegSpec for Cmdspr_SPEC {
    type DataType = u32;
}

pub type Cmdspr = crate::RegValueT<Cmdspr_SPEC>;

impl Cmdspr {
    #[inline(always)]
    pub fn msrdr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        cmdspr::Msrdr,
        cmdspr::Msrdr,
        Cmdspr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            cmdspr::Msrdr,
            cmdspr::Msrdr,
            Cmdspr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cdttim(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x7,
        1,
        0,
        cmdspr::Cdttim,
        cmdspr::Cdttim,
        Cmdspr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x7,
            1,
            0,
            cmdspr::Cdttim,
            cmdspr::Cdttim,
            Cmdspr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cmdspr {
    #[inline(always)]
    fn default() -> Cmdspr {
        <crate::RegValueT<Cmdspr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cmdspr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Msrdr_SPEC;
    pub type Msrdr = crate::EnumBitfieldStruct<u8, Msrdr_SPEC>;
    impl Msrdr {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cdttim_SPEC;
    pub type Cdttim = crate::EnumBitfieldStruct<u8, Cdttim_SPEC>;
    impl Cdttim {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _111: Self = Self::new(7);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdspt_SPEC;
impl crate::sealed::RegSpec for Cmdspt_SPEC {
    type DataType = u32;
}

pub type Cmdspt = crate::RegValueT<Cmdspt_SPEC>;

impl Cmdspt {
    #[inline(always)]
    pub fn mrttim(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, u32, Cmdspt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffff,1,0,u32,u32,Cmdspt_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mrte(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        cmdspt::Mrte,
        cmdspt::Mrte,
        Cmdspt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            cmdspt::Mrte,
            cmdspt::Mrte,
            Cmdspt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cmdspt {
    #[inline(always)]
    fn default() -> Cmdspt {
        <crate::RegValueT<Cmdspt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cmdspt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mrte_SPEC;
    pub type Mrte = crate::EnumBitfieldStruct<u8, Mrte_SPEC>;
    impl Mrte {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cetsm_SPEC;
impl crate::sealed::RegSpec for Cetsm_SPEC {
    type DataType = u32;
}

pub type Cetsm = crate::RegValueT<Cetsm_SPEC>;

impl Cetsm {
    #[inline(always)]
    pub fn sptsyn(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cetsm::Sptsyn,
        cetsm::Sptsyn,
        Cetsm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cetsm::Sptsyn,
            cetsm::Sptsyn,
            Cetsm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sptasyn0(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cetsm::Sptasyn0,
        cetsm::Sptasyn0,
        Cetsm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cetsm::Sptasyn0,
            cetsm::Sptasyn0,
            Cetsm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sptasyn1(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cetsm::Sptasyn1,
        cetsm::Sptasyn1,
        Cetsm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cetsm::Sptasyn1,
            cetsm::Sptasyn1,
            Cetsm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn freq(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cetsm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cetsm_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn inac(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cetsm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cetsm_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cetsm {
    #[inline(always)]
    fn default() -> Cetsm {
        <crate::RegValueT<Cetsm_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cetsm {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sptsyn_SPEC;
    pub type Sptsyn = crate::EnumBitfieldStruct<u8, Sptsyn_SPEC>;
    impl Sptsyn {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sptasyn0_SPEC;
    pub type Sptasyn0 = crate::EnumBitfieldStruct<u8, Sptasyn0_SPEC>;
    impl Sptasyn0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sptasyn1_SPEC;
    pub type Sptasyn1 = crate::EnumBitfieldStruct<u8, Sptasyn1_SPEC>;
    impl Sptasyn1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cetss_SPEC;
impl crate::sealed::RegSpec for Cetss_SPEC {
    type DataType = u32;
}

pub type Cetss = crate::RegValueT<Cetss_SPEC>;

impl Cetss {
    #[inline(always)]
    pub fn syne(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cetss::Syne,
        cetss::Syne,
        Cetss_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cetss::Syne,
            cetss::Syne,
            Cetss_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn asyne(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x3,
        1,
        0,
        cetss::Asyne,
        cetss::Asyne,
        Cetss_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x3,
            1,
            0,
            cetss::Asyne,
            cetss::Asyne,
            Cetss_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn icovf(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        cetss::Icovf,
        cetss::Icovf,
        Cetss_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            cetss::Icovf,
            cetss::Icovf,
            Cetss_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cetss {
    #[inline(always)]
    fn default() -> Cetss {
        <crate::RegValueT<Cetss_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cetss {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Syne_SPEC;
    pub type Syne = crate::EnumBitfieldStruct<u8, Syne_SPEC>;
    impl Syne {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asyne_SPEC;
    pub type Asyne = crate::EnumBitfieldStruct<u8, Asyne_SPEC>;
    impl Asyne {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icovf_SPEC;
    pub type Icovf = crate::EnumBitfieldStruct<u8, Icovf_SPEC>;
    impl Icovf {
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
pub struct Nqstlv_SPEC;
impl crate::sealed::RegSpec for Nqstlv_SPEC {
    type DataType = u32;
}

pub type Nqstlv = crate::RegValueT<Nqstlv_SPEC>;

impl Nqstlv {
    #[inline(always)]
    pub fn cmdqflv(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Nqstlv_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Nqstlv_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rspqlv(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Nqstlv_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Nqstlv_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ibiqlv(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Nqstlv_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Nqstlv_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ibiscnt(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, Nqstlv_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,Nqstlv_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Nqstlv {
    #[inline(always)]
    fn default() -> Nqstlv {
        <crate::RegValueT<Nqstlv_SPEC> as RegisterValue<_>>::new(4)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ndbstlv0_SPEC;
impl crate::sealed::RegSpec for Ndbstlv0_SPEC {
    type DataType = u32;
}

pub type Ndbstlv0 = crate::RegValueT<Ndbstlv0_SPEC>;

impl Ndbstlv0 {
    #[inline(always)]
    pub fn tdbflv(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Ndbstlv0_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Ndbstlv0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rdblv(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ndbstlv0_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ndbstlv0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ndbstlv0 {
    #[inline(always)]
    fn default() -> Ndbstlv0 {
        <crate::RegValueT<Ndbstlv0_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nrsqstlv_SPEC;
impl crate::sealed::RegSpec for Nrsqstlv_SPEC {
    type DataType = u32;
}

pub type Nrsqstlv = crate::RegValueT<Nrsqstlv_SPEC>;

impl Nrsqstlv {
    #[inline(always)]
    pub fn rsqlv(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Nrsqstlv_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Nrsqstlv_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Nrsqstlv {
    #[inline(always)]
    fn default() -> Nrsqstlv {
        <crate::RegValueT<Nrsqstlv_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hqstlv_SPEC;
impl crate::sealed::RegSpec for Hqstlv_SPEC {
    type DataType = u32;
}

pub type Hqstlv = crate::RegValueT<Hqstlv_SPEC>;

impl Hqstlv {
    #[inline(always)]
    pub fn cmdqlv(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Hqstlv_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Hqstlv_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rspqlv(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Hqstlv_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Hqstlv_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Hqstlv {
    #[inline(always)]
    fn default() -> Hqstlv {
        <crate::RegValueT<Hqstlv_SPEC> as RegisterValue<_>>::new(2)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hdbstlv_SPEC;
impl crate::sealed::RegSpec for Hdbstlv_SPEC {
    type DataType = u32;
}

pub type Hdbstlv = crate::RegValueT<Hdbstlv_SPEC>;

impl Hdbstlv {
    #[inline(always)]
    pub fn tdbflv(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Hdbstlv_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Hdbstlv_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rdblv(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Hdbstlv_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Hdbstlv_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Hdbstlv {
    #[inline(always)]
    fn default() -> Hdbstlv {
        <crate::RegValueT<Hdbstlv_SPEC> as RegisterValue<_>>::new(2)
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
        <crate::RegValueT<Prstdbg_SPEC> as RegisterValue<_>>::new(15)
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
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mserrcnt_SPEC;
impl crate::sealed::RegSpec for Mserrcnt_SPEC {
    type DataType = u32;
}

pub type Mserrcnt = crate::RegValueT<Mserrcnt_SPEC>;

impl Mserrcnt {
    #[inline(always)]
    pub fn m2ecnt(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Mserrcnt_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Mserrcnt_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mserrcnt {
    #[inline(always)]
    fn default() -> Mserrcnt {
        <crate::RegValueT<Mserrcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sc1Cpt_SPEC;
impl crate::sealed::RegSpec for Sc1Cpt_SPEC {
    type DataType = u32;
}

pub type Sc1Cpt = crate::RegValueT<Sc1Cpt_SPEC>;

impl Sc1Cpt {
    #[inline(always)]
    pub fn sc1c(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Sc1Cpt_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Sc1Cpt_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Sc1Cpt {
    #[inline(always)]
    fn default() -> Sc1Cpt {
        <crate::RegValueT<Sc1Cpt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sc2Cpt_SPEC;
impl crate::sealed::RegSpec for Sc2Cpt_SPEC {
    type DataType = u32;
}

pub type Sc2Cpt = crate::RegValueT<Sc2Cpt_SPEC>;

impl Sc2Cpt {
    #[inline(always)]
    pub fn sc2c(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Sc2Cpt_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Sc2Cpt_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Sc2Cpt {
    #[inline(always)]
    fn default() -> Sc2Cpt {
        <crate::RegValueT<Sc2Cpt_SPEC> as RegisterValue<_>>::new(0)
    }
}
