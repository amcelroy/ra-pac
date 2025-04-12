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
// Generated from SVD 0.90.02, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:15:45 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"I3C"]
unsafe impl ::core::marker::Send for super::I3C {}
unsafe impl ::core::marker::Sync for super::I3C {}
impl super::I3C {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "Protocol Selection Register"]
    #[inline(always)]
    pub const fn prts(&self) -> &'static crate::common::Reg<self::Prts_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Prts_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Bus Control Register"]
    #[inline(always)]
    pub const fn bctl(&self) -> &'static crate::common::Reg<self::Bctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Master Device Address Register"]
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

    #[doc = "Reset Control Register"]
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

    #[doc = "Present State Register"]
    #[inline(always)]
    pub const fn prsst(&self) -> &'static crate::common::Reg<self::Prsst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Prsst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "Internal Status Register"]
    #[inline(always)]
    pub const fn inst(&self) -> &'static crate::common::Reg<self::Inst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Inst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "Internal Status Enable Register"]
    #[inline(always)]
    pub const fn inste(&self) -> &'static crate::common::Reg<self::Inste_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Inste_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "Internal Interrupt Enable Register"]
    #[inline(always)]
    pub const fn inie(&self) -> &'static crate::common::Reg<self::Inie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Inie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "Internal Status Force Register"]
    #[inline(always)]
    pub const fn instfc(&self) -> &'static crate::common::Reg<self::Instfc_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Instfc_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "Device Characteristic Table Register"]
    #[inline(always)]
    pub const fn dvct(&self) -> &'static crate::common::Reg<self::Dvct_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Dvct_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = "IBI Notify Control Register"]
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

    #[doc = "Bus Function Control Register"]
    #[inline(always)]
    pub const fn bfctl(&self) -> &'static crate::common::Reg<self::Bfctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bfctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[doc = "Slave Control Register"]
    #[inline(always)]
    pub const fn svctl(&self) -> &'static crate::common::Reg<self::Svctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Svctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[doc = "Reference Clock Control Register"]
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

    #[doc = "Standard Bit Rate Register"]
    #[inline(always)]
    pub const fn stdbr(&self) -> &'static crate::common::Reg<self::Stdbr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Stdbr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[doc = "Extended Bit Rate Register"]
    #[inline(always)]
    pub const fn extbr(&self) -> &'static crate::common::Reg<self::Extbr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Extbr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[doc = "Bus Free Condition Detection Time Register"]
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

    #[doc = "Bus Available Condition Detection Time Register"]
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

    #[doc = "Bus Idle Condition Detection Time Register"]
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

    #[doc = "Output Control Register"]
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

    #[doc = "Input Control Register"]
    #[inline(always)]
    pub const fn inctl(&self) -> &'static crate::common::Reg<self::Inctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Inctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
            )
        }
    }

    #[doc = "Timeout Control Register"]
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

    #[doc = "Wake-up Unit Control Register"]
    #[inline(always)]
    pub const fn wuctl(&self) -> &'static crate::common::Reg<self::Wuctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Wuctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
            )
        }
    }

    #[doc = "Acknowledge Control Register"]
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

    #[doc = "SCL Stretch Control Register"]
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

    #[doc = "SCL Stalling Control Register"]
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

    #[doc = "Slave Transfer Data Length Register 0"]
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

    #[doc = "Condition Control Register"]
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

    #[doc = "Normal Command Queue Port Register"]
    #[inline(always)]
    pub const fn ncmdqp(&self) -> &'static crate::common::Reg<self::Ncmdqp_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ncmdqp_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(336usize),
            )
        }
    }

    #[doc = "Normal Response Queue Port Register"]
    #[inline(always)]
    pub const fn nrspqp(&self) -> &'static crate::common::Reg<self::Nrspqp_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Nrspqp_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(340usize),
            )
        }
    }

    #[doc = "Normal Transfer Data Buffer Port Register 0"]
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

    #[doc = "Normal Transfer Data Buffer Port Register 0"]
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

    #[doc = "Normal IBI Queue Port Register"]
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

    #[doc = "Normal Receive Status Queue Port Register"]
    #[inline(always)]
    pub const fn nrsqp(&self) -> &'static crate::common::Reg<self::Nrsqp_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Nrsqp_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(384usize),
            )
        }
    }

    #[doc = "Normal Queue Threshold Control Register"]
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

    #[doc = "Normal Transfer Data Buffer Threshold Control Register 0"]
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

    #[doc = "Normal Receive Status Queue Threshold Control Register"]
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

    #[doc = "Bus Status Register"]
    #[inline(always)]
    pub const fn bst(&self) -> &'static crate::common::Reg<self::Bst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(464usize),
            )
        }
    }

    #[doc = "Bus Status Enable Register"]
    #[inline(always)]
    pub const fn bste(&self) -> &'static crate::common::Reg<self::Bste_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bste_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(468usize),
            )
        }
    }

    #[doc = "Bus Interrupt Enable Register"]
    #[inline(always)]
    pub const fn bie(&self) -> &'static crate::common::Reg<self::Bie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(472usize),
            )
        }
    }

    #[doc = "Bus Status Force Register"]
    #[inline(always)]
    pub const fn bstfc(&self) -> &'static crate::common::Reg<self::Bstfc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bstfc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(476usize),
            )
        }
    }

    #[doc = "Normal Transfer Status Register"]
    #[inline(always)]
    pub const fn ntst(&self) -> &'static crate::common::Reg<self::Ntst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ntst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(480usize),
            )
        }
    }

    #[doc = "Normal Transfer Status Enable Register"]
    #[inline(always)]
    pub const fn ntste(&self) -> &'static crate::common::Reg<self::Ntste_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ntste_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(484usize),
            )
        }
    }

    #[doc = "Normal Transfer Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ntie(&self) -> &'static crate::common::Reg<self::Ntie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ntie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(488usize),
            )
        }
    }

    #[doc = "Normal Transfer Status Force Register"]
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

    #[doc = "Bus Condition Status Register"]
    #[inline(always)]
    pub const fn bcst(&self) -> &'static crate::common::Reg<self::Bcst_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Bcst_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(528usize),
            )
        }
    }

    #[doc = "Slave Status Register"]
    #[inline(always)]
    pub const fn svst(&self) -> &'static crate::common::Reg<self::Svst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Svst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(532usize),
            )
        }
    }

    #[doc = "Wake-up Unit Operating Status Register"]
    #[inline(always)]
    pub const fn wust(&self) -> &'static crate::common::Reg<self::Wust_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Wust_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(536usize),
            )
        }
    }

    #[doc = "Device Address Table Basic Register %s"]
    #[inline(always)]
    pub const fn datbas(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Datbas_SPEC, crate::common::RW>,
        4,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x224usize))
        }
    }

    #[doc = "Extended Device Address Table Basic Register"]
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

    #[doc = "Slave Device Address Table Basic Register 0"]
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

    #[doc = "Master Device Characteristic Table Register %s"]
    #[inline(always)]
    pub const fn msdct(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Msdct_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2d0usize))
        }
    }

    #[doc = "Slave Device Characteristic Table Register"]
    #[inline(always)]
    pub const fn svdct(&self) -> &'static crate::common::Reg<self::Svdct_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Svdct_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(800usize),
            )
        }
    }

    #[doc = "Slave Device Characteristic Table Provisional ID Low Register"]
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

    #[doc = "Slave Device Characteristic Table Provisional ID High Register"]
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

    #[doc = "Slave Device Address Register 0"]
    #[inline(always)]
    pub const fn svdvad0(
        &self,
    ) -> &'static crate::common::Reg<self::Svdvad0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Svdvad0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(816usize),
            )
        }
    }

    #[doc = "CCC Slave Events Command Register"]
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

    #[doc = "CCC Enter Activity State Register"]
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

    #[doc = "CCC Max Write Length Register"]
    #[inline(always)]
    pub const fn cmwlg(&self) -> &'static crate::common::Reg<self::Cmwlg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cmwlg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(856usize),
            )
        }
    }

    #[doc = "CCC Max Read Length Register"]
    #[inline(always)]
    pub const fn cmrlg(&self) -> &'static crate::common::Reg<self::Cmrlg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cmrlg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(860usize),
            )
        }
    }

    #[doc = "CCC Enter Test Mode Register"]
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

    #[doc = "CCC Get Device Status Register"]
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

    #[doc = "CCC Max Data Speed W (Write) Register"]
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

    #[doc = "CCC Max Data Speed R (Read) Register"]
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

    #[doc = "CCC Max Data Speed T (Turnaround) Register"]
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

    #[doc = "CCC Exchange Timing Support Information M (Mode) Register"]
    #[inline(always)]
    pub const fn cetsm(&self) -> &'static crate::common::Reg<self::Cetsm_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cetsm_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(884usize),
            )
        }
    }

    #[doc = "CCC Get HDR Capability Register"]
    #[inline(always)]
    pub const fn cghdrcap(
        &self,
    ) -> &'static crate::common::Reg<self::Cghdrcap_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cghdrcap_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(892usize),
            )
        }
    }

    #[doc = "Bit Count Register"]
    #[inline(always)]
    pub const fn bitcnt(&self) -> &'static crate::common::Reg<self::Bitcnt_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Bitcnt_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(896usize),
            )
        }
    }

    #[doc = "Normal Queue Status Level Register"]
    #[inline(always)]
    pub const fn nqstlv(&self) -> &'static crate::common::Reg<self::Nqstlv_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Nqstlv_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(916usize),
            )
        }
    }

    #[doc = "Normal Data Buffer Status Level Register 0"]
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

    #[doc = "Normal Receive Status Queue Status Level Register"]
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

    #[doc = "Present State Debug Register"]
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

    #[doc = "Master Error Counters Register"]
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
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prts_SPEC;
impl crate::sealed::RegSpec for Prts_SPEC {
    type DataType = u32;
}
#[doc = "Protocol Selection Register"]
pub type Prts = crate::RegValueT<Prts_SPEC>;

impl Prts {
    #[doc = "Protocol Mode"]
    #[inline(always)]
    pub fn prtmd(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, prts::Prtmd, Prts_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,prts::Prtmd, Prts_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "I3C protocol mode"]
        pub const _0: Self = Self::new(0);
        #[doc = "I2C protocol mode"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bctl_SPEC;
impl crate::sealed::RegSpec for Bctl_SPEC {
    type DataType = u32;
}
#[doc = "Bus Control Register"]
pub type Bctl = crate::RegValueT<Bctl_SPEC>;

impl Bctl {
    #[doc = "Include I3C Broadcast Address"]
    #[inline(always)]
    pub fn incba(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, bctl::Incba, Bctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,bctl::Incba, Bctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bus Mode Selection"]
    #[inline(always)]
    pub fn bmds(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, bctl::Bmds, Bctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,bctl::Bmds, Bctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Hot-Join Acknowledge Control"]
    #[inline(always)]
    pub fn hjackctl(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, bctl::Hjackctl, Bctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,bctl::Hjackctl, Bctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Abort"]
    #[inline(always)]
    pub fn abt(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, bctl::Abt, Bctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x1,1,0,bctl::Abt, Bctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Resume"]
    #[inline(always)]
    pub fn rsm(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, bctl::Rsm, Bctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x1,1,0,bctl::Rsm, Bctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bus Enable"]
    #[inline(always)]
    pub fn buse(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, bctl::Buse, Bctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<31,0x1,1,0,bctl::Buse, Bctl_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Do not include I3C broadcast address for private transfers"]
        pub const _0: Self = Self::new(0);
        #[doc = "Include I3C broadcast address for private transfers"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bmds_SPEC;
    pub type Bmds = crate::EnumBitfieldStruct<u8, Bmds_SPEC>;
    impl Bmds {
        #[doc = "Legacy inclusive Bus mode disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Legacy inclusive (mix) Bus mode enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hjackctl_SPEC;
    pub type Hjackctl = crate::EnumBitfieldStruct<u8, Hjackctl_SPEC>;
    impl Hjackctl {
        #[doc = "ACK the Hot-Join request"]
        pub const _0: Self = Self::new(0);
        #[doc = "NACK and send broadcast CCC to disable Hot-Join"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Abt_SPEC;
    pub type Abt = crate::EnumBitfieldStruct<u8, Abt_SPEC>;
    impl Abt {
        #[doc = "I3C is running."]
        pub const _0: Self = Self::new(0);
        #[doc = "I3C has aborted a transfer."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rsm_SPEC;
    pub type Rsm = crate::EnumBitfieldStruct<u8, Rsm_SPEC>;
    impl Rsm {
        #[doc = "I3C is running."]
        pub const _0: Self = Self::new(0);
        #[doc = "I3C is suspended."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Buse_SPEC;
    pub type Buse = crate::EnumBitfieldStruct<u8, Buse_SPEC>;
    impl Buse {
        #[doc = "I3C bus operation is disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "I3C bus operation is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msdvad_SPEC;
impl crate::sealed::RegSpec for Msdvad_SPEC {
    type DataType = u32;
}
#[doc = "Master Device Address Register"]
pub type Msdvad = crate::RegValueT<Msdvad_SPEC>;

impl Msdvad {
    #[doc = "Master Dynamic Address"]
    #[inline(always)]
    pub fn mdyad(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, Msdvad_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7f,1,0,u8, Msdvad_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Master Dynamic Address Valid"]
    #[inline(always)]
    pub fn mdyadv(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, msdvad::Mdyadv, Msdvad_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,msdvad::Mdyadv, Msdvad_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "The master dynamic address field is not valid."]
        pub const _0: Self = Self::new(0);
        #[doc = "The master dynamic address field is valid."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rstctl_SPEC;
impl crate::sealed::RegSpec for Rstctl_SPEC {
    type DataType = u32;
}
#[doc = "Reset Control Register"]
pub type Rstctl = crate::RegValueT<Rstctl_SPEC>;

impl Rstctl {
    #[doc = "I3C Software Reset"]
    #[inline(always)]
    pub fn ri3crst(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, rstctl::Ri3Crst, Rstctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,rstctl::Ri3Crst, Rstctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Normal Command Queue Software Reset"]
    #[inline(always)]
    pub fn cmdqrst(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, rstctl::Cmdqrst, Rstctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,rstctl::Cmdqrst, Rstctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Normal Response Queue Software Reset"]
    #[inline(always)]
    pub fn rspqrst(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, rstctl::Rspqrst, Rstctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,rstctl::Rspqrst, Rstctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Normal Tx Data Buffer Software Reset"]
    #[inline(always)]
    pub fn tdbrst(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, rstctl::Tdbrst, Rstctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,rstctl::Tdbrst, Rstctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Normal Rx Data Buffer Software Reset"]
    #[inline(always)]
    pub fn rdbrst(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, rstctl::Rdbrst, Rstctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,rstctl::Rdbrst, Rstctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Normal IBI Queue Software Reset"]
    #[inline(always)]
    pub fn ibiqrst(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, rstctl::Ibiqrst, Rstctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,rstctl::Ibiqrst, Rstctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Normal Receive Status Queue Software Reset"]
    #[inline(always)]
    pub fn rsqrst(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, rstctl::Rsqrst, Rstctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,rstctl::Rsqrst, Rstctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Internal Software Reset"]
    #[inline(always)]
    pub fn intlrst(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, rstctl::Intlrst, Rstctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,rstctl::Intlrst, Rstctl_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Release I3C reset."]
        pub const _0: Self = Self::new(0);
        #[doc = "Initiate I3C reset."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmdqrst_SPEC;
    pub type Cmdqrst = crate::EnumBitfieldStruct<u8, Cmdqrst_SPEC>;
    impl Cmdqrst {
        #[doc = "The Normal Command Queues in I3C is not flushed."]
        pub const _0: Self = Self::new(0);
        #[doc = "The Normal Command Queues in I3C is flushed."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rspqrst_SPEC;
    pub type Rspqrst = crate::EnumBitfieldStruct<u8, Rspqrst_SPEC>;
    impl Rspqrst {
        #[doc = "The Normal Response Queues in I3C is not flushed."]
        pub const _0: Self = Self::new(0);
        #[doc = "The Normal Response Queues in I3C is flushed."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdbrst_SPEC;
    pub type Tdbrst = crate::EnumBitfieldStruct<u8, Tdbrst_SPEC>;
    impl Tdbrst {
        #[doc = "The Normal Tx Data buffers in I3C is not flushed."]
        pub const _0: Self = Self::new(0);
        #[doc = "The Normal Tx Data buffers in I3C is flushed."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdbrst_SPEC;
    pub type Rdbrst = crate::EnumBitfieldStruct<u8, Rdbrst_SPEC>;
    impl Rdbrst {
        #[doc = "The Normal Rx Data buffers in I3C is not flushed."]
        pub const _0: Self = Self::new(0);
        #[doc = "The Normal Rx Data buffers in I3C is flushed."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ibiqrst_SPEC;
    pub type Ibiqrst = crate::EnumBitfieldStruct<u8, Ibiqrst_SPEC>;
    impl Ibiqrst {
        #[doc = "The Normal IBI Queues in I3C is not flushed."]
        pub const _0: Self = Self::new(0);
        #[doc = "The Normal IBI Queues in I3C is flushed."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rsqrst_SPEC;
    pub type Rsqrst = crate::EnumBitfieldStruct<u8, Rsqrst_SPEC>;
    impl Rsqrst {
        #[doc = "The Normal Receive Status Queue in I3C is not flushed."]
        pub const _0: Self = Self::new(0);
        #[doc = "The Normal Receive Status Queue in I3C is flushed."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Intlrst_SPEC;
    pub type Intlrst = crate::EnumBitfieldStruct<u8, Intlrst_SPEC>;
    impl Intlrst {
        #[doc = "Releases of some registers and internal state."]
        pub const _0: Self = Self::new(0);
        #[doc = "Resets of some registers and internal state."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prsst_SPEC;
impl crate::sealed::RegSpec for Prsst_SPEC {
    type DataType = u32;
}
#[doc = "Present State Register"]
pub type Prsst = crate::RegValueT<Prsst_SPEC>;

impl Prsst {
    #[doc = "Current Master"]
    #[inline(always)]
    pub fn crms(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, prsst::Crms, Prsst_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,prsst::Crms, Prsst_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmit/Receive Mode"]
    #[inline(always)]
    pub fn trmd(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, prsst::Trmd, Prsst_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x1,1,0,prsst::Trmd, Prsst_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Present State Write Protect"]
    #[inline(always)]
    pub fn prsstwp(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, prsst::Prsstwp, Prsst_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<7,0x1,1,0,prsst::Prsstwp, Prsst_SPEC,crate::common::W>::from_register(self,0)
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
        #[doc = "The Master is not the Current Master, and must request and acquire bus ownership before initiating any transfer."]
        pub const _0: Self = Self::new(0);
        #[doc = "The Master is the Current Master, and as a result can initiate transfers."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trmd_SPEC;
    pub type Trmd = crate::EnumBitfieldStruct<u8, Trmd_SPEC>;
    impl Trmd {
        #[doc = "Receive mode"]
        pub const _0: Self = Self::new(0);
        #[doc = "Transmit mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prsstwp_SPEC;
    pub type Prsstwp = crate::EnumBitfieldStruct<u8, Prsstwp_SPEC>;
    impl Prsstwp {
        #[doc = "CRMS bit is protected."]
        pub const _0: Self = Self::new(0);
        #[doc = "CRMS bit can be written when writing simultaneously with the value of the target bit."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inst_SPEC;
impl crate::sealed::RegSpec for Inst_SPEC {
    type DataType = u32;
}
#[doc = "Internal Status Register"]
pub type Inst = crate::RegValueT<Inst_SPEC>;

impl Inst {
    #[doc = "Internal Error Flag"]
    #[inline(always)]
    pub fn inef(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, inst::Inef, Inst_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x1,1,0,inst::Inef, Inst_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "I3C Internal Error has not detected."]
        pub const _0: Self = Self::new(0);
        #[doc = "I3C Internal Error has detected."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inste_SPEC;
impl crate::sealed::RegSpec for Inste_SPEC {
    type DataType = u32;
}
#[doc = "Internal Status Enable Register"]
pub type Inste = crate::RegValueT<Inste_SPEC>;

impl Inste {
    #[doc = "Internal Error Enable"]
    #[inline(always)]
    pub fn inee(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, inste::Inee, Inste_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,inste::Inee, Inste_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Disable INST.INEF"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable INST.INEF"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inie_SPEC;
impl crate::sealed::RegSpec for Inie_SPEC {
    type DataType = u32;
}
#[doc = "Internal Interrupt Enable Register"]
pub type Inie = crate::RegValueT<Inie_SPEC>;

impl Inie {
    #[doc = "Internal Error Interrupt Enable"]
    #[inline(always)]
    pub fn ineie(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, inie::Ineie, Inie_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,inie::Ineie, Inie_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Disables Non-recoverable Internal Error Interrupt Signal."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables Non-recoverable Internal Error Interrupt Signal."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Instfc_SPEC;
impl crate::sealed::RegSpec for Instfc_SPEC {
    type DataType = u32;
}
#[doc = "Internal Status Force Register"]
pub type Instfc = crate::RegValueT<Instfc_SPEC>;

impl Instfc {
    #[doc = "Internal Error Force"]
    #[inline(always)]
    pub fn inefc(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, instfc::Inefc, Instfc_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<10,0x1,1,0,instfc::Inefc, Instfc_SPEC,crate::common::W>::from_register(self,0)
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
        #[doc = "Not force a specific interrupt"]
        pub const _0: Self = Self::new(0);
        #[doc = "Force a specific interrupt"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dvct_SPEC;
impl crate::sealed::RegSpec for Dvct_SPEC {
    type DataType = u32;
}
#[doc = "Device Characteristic Table Register"]
pub type Dvct = crate::RegValueT<Dvct_SPEC>;

impl Dvct {
    #[doc = "DCT Table Index"]
    #[inline(always)]
    pub fn idx(
        self,
    ) -> crate::common::RegisterField<19, 0x1f, 1, 0, u8, Dvct_SPEC, crate::common::R> {
        crate::common::RegisterField::<19,0x1f,1,0,u8, Dvct_SPEC,crate::common::R>::from_register(self,0)
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
#[doc = "IBI Notify Control Register"]
pub type Ibinctl = crate::RegValueT<Ibinctl_SPEC>;

impl Ibinctl {
    #[doc = "Notify Rejected Hot-Join Control"]
    #[inline(always)]
    pub fn nrhjctl(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ibinctl::Nrhjctl, Ibinctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ibinctl::Nrhjctl,
            Ibinctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Notify Rejected Master Request Control"]
    #[inline(always)]
    pub fn nrmrctl(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ibinctl::Nrmrctl, Ibinctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ibinctl::Nrmrctl,
            Ibinctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Notify Rejected Slave Interrupt Request Control"]
    #[inline(always)]
    pub fn nrsirctl(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
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
    pub struct Nrhjctl_SPEC;
    pub type Nrhjctl = crate::EnumBitfieldStruct<u8, Nrhjctl_SPEC>;
    impl Nrhjctl {
        #[doc = "Do not pass rejected IBI Status to IBI Queue, if the incoming HotJoin request is NACKed and is autodisabled based on field HJACKCTL of BCTL."]
        pub const _0: Self = Self::new(0);
        #[doc = "Pass rejected IBI Status to the IBI Queue, if the incoming Hot Join request is NACKed and is autodisabled based on field HJACKCTL of BCTL."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nrmrctl_SPEC;
    pub type Nrmrctl = crate::EnumBitfieldStruct<u8, Nrmrctl_SPEC>;
    impl Nrmrctl {
        #[doc = "Do not pass rejected IBI Status to the Normal IBI Queue, if the incoming Master Request is NACKed and is auto-disabled based on DVMRRJ field in relevant DAT entry."]
        pub const _0: Self = Self::new(0);
        #[doc = "Pass rejected IBI Status to the Normal IBI Queue if the incoming Master Request is NACKed and is autodisabled based on DVMRRJ field in relevant DAT entry."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nrsirctl_SPEC;
    pub type Nrsirctl = crate::EnumBitfieldStruct<u8, Nrsirctl_SPEC>;
    impl Nrsirctl {
        #[doc = "Do not pass rejected IBI Status to the Normal IBI Queue, if the incoming SIR is NACKed and is auto-disabled based on DVSIRRJ field in relevant DAT entry."]
        pub const _0: Self = Self::new(0);
        #[doc = "Pass rejected IBI Status to the Normal IBI Queue, if the incoming SIR is NACKed and is auto-disabled based on DVSIRRJ field in relevant DAT entry."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bfctl_SPEC;
impl crate::sealed::RegSpec for Bfctl_SPEC {
    type DataType = u32;
}
#[doc = "Bus Function Control Register"]
pub type Bfctl = crate::RegValueT<Bfctl_SPEC>;

impl Bfctl {
    #[doc = "Master Arbitration-Lost Detection Enable"]
    #[inline(always)]
    pub fn male(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, bfctl::Male, Bfctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,bfctl::Male, Bfctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "NACK Transmission Arbitration-Lost Detection Enable"]
    #[inline(always)]
    pub fn nale(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, bfctl::Nale, Bfctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,bfctl::Nale, Bfctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Slave Arbitration-Lost Detection Enable"]
    #[inline(always)]
    pub fn sale(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, bfctl::Sale, Bfctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,bfctl::Sale, Bfctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SCL Synchronous Circuit Enable"]
    #[inline(always)]
    pub fn scsyne(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, bfctl::Scsyne, Bfctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,bfctl::Scsyne, Bfctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SMBus/I2C Bus Selection"]
    #[inline(always)]
    pub fn smbs(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, bfctl::Smbs, Bfctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,bfctl::Smbs, Bfctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fast-mode Plus Enable"]
    #[inline(always)]
    pub fn fmpe(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, bfctl::Fmpe, Bfctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,bfctl::Fmpe, Bfctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "High Speed Mode Enable"]
    #[inline(always)]
    pub fn hsme(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, bfctl::Hsme, Bfctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,bfctl::Hsme, Bfctl_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Master arbitration-lost detection disables. Disables the arbitration-lost detection function and does not clear the CRMS and TRMD bits in PRSST automatically when arbitration is lost."]
        pub const _0: Self = Self::new(0);
        #[doc = "Master arbitration-lost detection enables. Enables the arbitration-lost detection function and clears the CRMS and TRMD bits in PRSST automatically when arbitration is lost."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nale_SPEC;
    pub type Nale = crate::EnumBitfieldStruct<u8, Nale_SPEC>;
    impl Nale {
        #[doc = "NACK transmission arbitration-lost detection disables."]
        pub const _0: Self = Self::new(0);
        #[doc = "NACK transmission arbitration-lost detection enables."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sale_SPEC;
    pub type Sale = crate::EnumBitfieldStruct<u8, Sale_SPEC>;
    impl Sale {
        #[doc = "Slave arbitration-lost detection disables."]
        pub const _0: Self = Self::new(0);
        #[doc = "Slave arbitration-lost detection enables."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scsyne_SPEC;
    pub type Scsyne = crate::EnumBitfieldStruct<u8, Scsyne_SPEC>;
    impl Scsyne {
        #[doc = "No SCL synchronous circuit uses."]
        pub const _0: Self = Self::new(0);
        #[doc = "An SCL synchronous circuit uses."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Smbs_SPEC;
    pub type Smbs = crate::EnumBitfieldStruct<u8, Smbs_SPEC>;
    impl Smbs {
        #[doc = "The I2C bus select."]
        pub const _0: Self = Self::new(0);
        #[doc = "The SMBus select."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fmpe_SPEC;
    pub type Fmpe = crate::EnumBitfieldStruct<u8, Fmpe_SPEC>;
    impl Fmpe {
        #[doc = "No Fm+ slope control circuit uses for the I3C_SCL pin and I3C_SDA pin. (n = 0)"]
        pub const _0: Self = Self::new(0);
        #[doc = "An Fm+ slope control circuit uses for the I3C_SCL pin and I3C_SDA pin. (n = 0)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hsme_SPEC;
    pub type Hsme = crate::EnumBitfieldStruct<u8, Hsme_SPEC>;
    impl Hsme {
        #[doc = "Disable High Speed Mode."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable High Speed Mode."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Svctl_SPEC;
impl crate::sealed::RegSpec for Svctl_SPEC {
    type DataType = u32;
}
#[doc = "Slave Control Register"]
pub type Svctl = crate::RegValueT<Svctl_SPEC>;

impl Svctl {
    #[doc = "General Call Address Enable"]
    #[inline(always)]
    pub fn gcae(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, svctl::Gcae, Svctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,svctl::Gcae, Svctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Hs-mode Master Code Enable"]
    #[inline(always)]
    pub fn hsmce(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, svctl::Hsmce, Svctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,svctl::Hsmce, Svctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Device-ID Address Enable"]
    #[inline(always)]
    pub fn dvide(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, svctl::Dvide, Svctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,svctl::Dvide, Svctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Host Address Enable"]
    #[inline(always)]
    pub fn hoae(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, svctl::Hoae, Svctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,svctl::Hoae, Svctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Slave Address Enable 0"]
    #[inline(always)]
    pub fn svae0(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, svctl::Svae0, Svctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,svctl::Svae0, Svctl_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "General call address detection disables."]
        pub const _0: Self = Self::new(0);
        #[doc = "General call address detection enables."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hsmce_SPEC;
    pub type Hsmce = crate::EnumBitfieldStruct<u8, Hsmce_SPEC>;
    impl Hsmce {
        #[doc = "Hs-mode Master Code Detection disables."]
        pub const _0: Self = Self::new(0);
        #[doc = "Hs-mode Master Code Detection enables."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvide_SPEC;
    pub type Dvide = crate::EnumBitfieldStruct<u8, Dvide_SPEC>;
    impl Dvide {
        #[doc = "Device-ID address detection disables."]
        pub const _0: Self = Self::new(0);
        #[doc = "Device-ID address detection enables."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hoae_SPEC;
    pub type Hoae = crate::EnumBitfieldStruct<u8, Hoae_SPEC>;
    impl Hoae {
        #[doc = "Host address detection disables."]
        pub const _0: Self = Self::new(0);
        #[doc = "Host address detection enables."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Svae0_SPEC;
    pub type Svae0 = crate::EnumBitfieldStruct<u8, Svae0_SPEC>;
    impl Svae0 {
        #[doc = "Slave n disables"]
        pub const _0: Self = Self::new(0);
        #[doc = "Slave n enables"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Refckctl_SPEC;
impl crate::sealed::RegSpec for Refckctl_SPEC {
    type DataType = u32;
}
#[doc = "Reference Clock Control Register"]
pub type Refckctl = crate::RegValueT<Refckctl_SPEC>;

impl Refckctl {
    #[doc = "Internal Reference Clock Selection"]
    #[inline(always)]
    pub fn irefcks(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
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
        #[doc = "TCLK/1 clock"]
        pub const _000: Self = Self::new(0);
        #[doc = "TCLK/2 clock"]
        pub const _001: Self = Self::new(1);
        #[doc = "TCLK/4 clock"]
        pub const _010: Self = Self::new(2);
        #[doc = "TCLK/8 clock"]
        pub const _011: Self = Self::new(3);
        #[doc = "TCLK/16 clock"]
        pub const _100: Self = Self::new(4);
        #[doc = "TCLK/32 clock"]
        pub const _101: Self = Self::new(5);
        #[doc = "TCLK/64 clock"]
        pub const _110: Self = Self::new(6);
        #[doc = "TCLK/128 clock"]
        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stdbr_SPEC;
impl crate::sealed::RegSpec for Stdbr_SPEC {
    type DataType = u32;
}
#[doc = "Standard Bit Rate Register"]
pub type Stdbr = crate::RegValueT<Stdbr_SPEC>;

impl Stdbr {
    #[doc = "Standard Bit Rate Low-Level Period Open-Drain"]
    #[inline(always)]
    pub fn sbrlo(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Stdbr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Stdbr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Standard Bit Rate High-Level Period Open-Drain"]
    #[inline(always)]
    pub fn sbrho(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Stdbr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Stdbr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Standard Bit Rate Low-level Period Push-Pull"]
    #[inline(always)]
    pub fn sbrlp(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, Stdbr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3f,1,0,u8, Stdbr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Standard Bit Rate High-Level Period Push-Pull"]
    #[inline(always)]
    pub fn sbrhp(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, Stdbr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3f,1,0,u8, Stdbr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Double the Standard Bit Rate Period for Open-Drain"]
    #[inline(always)]
    pub fn dsbrpo(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, stdbr::Dsbrpo, Stdbr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,stdbr::Dsbrpo, Stdbr_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "The time period set for SBRHO\\[7:0\\] and SBRLO\\[7:0\\] is not doubled."]
        pub const _0: Self = Self::new(0);
        #[doc = "The time period set for SBRHO\\[7:0\\] and SBRLO\\[7:0\\] is doubled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Extbr_SPEC;
impl crate::sealed::RegSpec for Extbr_SPEC {
    type DataType = u32;
}
#[doc = "Extended Bit Rate Register"]
pub type Extbr = crate::RegValueT<Extbr_SPEC>;

impl Extbr {
    #[doc = "Extended Bit Rate Low-Level Period Open-Drain"]
    #[inline(always)]
    pub fn ebrlo(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Extbr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Extbr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Extended Bit Rate High-Level Period Open-Drain"]
    #[inline(always)]
    pub fn ebrho(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Extbr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Extbr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Extended Bit Rate Low-Level Period Push-Pull"]
    #[inline(always)]
    pub fn ebrlp(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, Extbr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3f,1,0,u8, Extbr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Extended Bit Rate High-Level Period Push-Pull"]
    #[inline(always)]
    pub fn ebrhp(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, Extbr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3f,1,0,u8, Extbr_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Bus Free Condition Detection Time Register"]
pub type Bfrecdt = crate::RegValueT<Bfrecdt_SPEC>;

impl Bfrecdt {
    #[doc = "Bus Free Condition Detection Cycle"]
    #[inline(always)]
    pub fn frecyc(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, Bfrecdt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16, Bfrecdt_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Bus Available Condition Detection Time Register"]
pub type Bavlcdt = crate::RegValueT<Bavlcdt_SPEC>;

impl Bavlcdt {
    #[doc = "Bus Available Condition Detection Cycle"]
    #[inline(always)]
    pub fn avlcyc(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, Bavlcdt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16, Bavlcdt_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Bus Idle Condition Detection Time Register"]
pub type Bidlcdt = crate::RegValueT<Bidlcdt_SPEC>;

impl Bidlcdt {
    #[doc = "Bus Idle Condition Detection Cycle"]
    #[inline(always)]
    pub fn idlcyc(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffff, 1, 0, u32, Bidlcdt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ffff,1,0,u32, Bidlcdt_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Output Control Register"]
pub type Outctl = crate::RegValueT<Outctl_SPEC>;

impl Outctl {
    #[doc = "SDA Output Control"]
    #[inline(always)]
    pub fn sdoc(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, outctl::Sdoc, Outctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,outctl::Sdoc, Outctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SCL Output Control"]
    #[inline(always)]
    pub fn scoc(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, outctl::Scoc, Outctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,outctl::Scoc, Outctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SCL/SDA Output Control Write Protect"]
    #[inline(always)]
    pub fn socwp(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, outctl::Socwp, Outctl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<2,0x1,1,0,outctl::Socwp, Outctl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Extra SCL Clock Cycle Output"]
    #[inline(always)]
    pub fn excyc(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, outctl::Excyc, Outctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,outctl::Excyc, Outctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SDA Output Delay"]
    #[inline(always)]
    pub fn sdod(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, outctl::Sdod, Outctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,outctl::Sdod, Outctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SDA Output Delay Clock Source Selection"]
    #[inline(always)]
    pub fn sdodcs(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, outctl::Sdodcs, Outctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,outctl::Sdodcs, Outctl_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "I3C drives the I3C_SDA pin low."]
        pub const _0: Self = Self::new(0);
        #[doc = "I3C releases the I3C_SDA pin."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scoc_SPEC;
    pub type Scoc = crate::EnumBitfieldStruct<u8, Scoc_SPEC>;
    impl Scoc {
        #[doc = "I3C drives the I3C_SCL pin low."]
        pub const _0: Self = Self::new(0);
        #[doc = "I3C releases the I3C_SCL pin."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Socwp_SPEC;
    pub type Socwp = crate::EnumBitfieldStruct<u8, Socwp_SPEC>;
    impl Socwp {
        #[doc = "Bits SCOC and SDOC are protected."]
        pub const _0: Self = Self::new(0);
        #[doc = "Bits SCOC and SDOC can be written (When writing simultaneously with the value of the target bit). This bit is read as 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Excyc_SPEC;
    pub type Excyc = crate::EnumBitfieldStruct<u8, Excyc_SPEC>;
    impl Excyc {
        #[doc = "Does not output an extra SCL clock cycle (default)."]
        pub const _0: Self = Self::new(0);
        #[doc = "Outputs an extra SCL clock cycle."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdod_SPEC;
    pub type Sdod = crate::EnumBitfieldStruct<u8, Sdod_SPEC>;
    impl Sdod {
        #[doc = "No output delay"]
        pub const _000: Self = Self::new(0);
        #[doc = "1 I3Cφ cycle (When OUTCTL.SDODCS = 0 (I3Cφ)) 1 or 2 I3Cφ cycles (When OUTCTL.SDODCS = 1 (I3Cφ/2))"]
        pub const _001: Self = Self::new(1);
        #[doc = "2 I3Cφ cycles (When OUTCTL.SDODCS = 0 (I3Cφ)) 3 or 4 I3Cφ cycles (When OUTCTL.SDODCS = 1 (I3Cφ/2))"]
        pub const _010: Self = Self::new(2);
        #[doc = "3 I3Cφ cycles (When OUTCTL.SDODCS = 0 (I3Cφ)) 5 or 6 I3Cφ cycles (When OUTCTL.SDODCS = 1 (I3Cφ/2))"]
        pub const _011: Self = Self::new(3);
        #[doc = "4 I3Cφ cycles (When OUTCTL.SDODCS = 0 (I3Cφ)) 7 or 8 I3Cφ cycles (When OUTCTL.SDODCS = 1 (I3Cφ/2))"]
        pub const _100: Self = Self::new(4);
        #[doc = "5 I3Cφ cycles (When OUTCTL.SDODCS = 0 (I3Cφ)) 9 or 10 I3Cφ cycles (When OUTCTL.SDODCS = 1 (I3Cφ/2))"]
        pub const _101: Self = Self::new(5);
        #[doc = "6 I3Cφ cycles (When OUTCTL.SDODCS = 0 (I3Cφ)) 11 or 12 I3Cφ cycles (When OUTCTL.SDODCS = 1 (I3Cφ/2))"]
        pub const _110: Self = Self::new(6);
        #[doc = "7 I3Cφ cycles (When OUTCTL.SDODCS = 0 (I3Cφ)) 13 or 14 I3Cφ cycles (When OUTCTL.SDODCS = 1 (I3Cφ/2))"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdodcs_SPEC;
    pub type Sdodcs = crate::EnumBitfieldStruct<u8, Sdodcs_SPEC>;
    impl Sdodcs {
        #[doc = "The internal reference clock (I3Cφ) is selected as the clock source of the SDA output delay counter."]
        pub const _0: Self = Self::new(0);
        #[doc = "The internal reference clock divided by 2 (I3Cφ/2) is selected as the clock source of the SDA output delay counter."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inctl_SPEC;
impl crate::sealed::RegSpec for Inctl_SPEC {
    type DataType = u32;
}
#[doc = "Input Control Register"]
pub type Inctl = crate::RegValueT<Inctl_SPEC>;

impl Inctl {
    #[doc = "Digital Noise Filter Stage Selection"]
    #[inline(always)]
    pub fn dnfs(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Inctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Inctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Digital Noise Filter Circuit Enable"]
    #[inline(always)]
    pub fn dnfe(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, inctl::Dnfe, Inctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,inctl::Dnfe, Inctl_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "No digital noise filter circuit is used."]
        pub const _0: Self = Self::new(0);
        #[doc = "A digital noise filter circuit is used."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmoctl_SPEC;
impl crate::sealed::RegSpec for Tmoctl_SPEC {
    type DataType = u32;
}
#[doc = "Timeout Control Register"]
pub type Tmoctl = crate::RegValueT<Tmoctl_SPEC>;

impl Tmoctl {
    #[doc = "Timeout Detection Time Selection"]
    #[inline(always)]
    pub fn todts(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, tmoctl::Todts, Tmoctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,tmoctl::Todts, Tmoctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timeout L Count Control"]
    #[inline(always)]
    pub fn tolctl(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, tmoctl::Tolctl, Tmoctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,tmoctl::Tolctl, Tmoctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timeout H Count Control"]
    #[inline(always)]
    pub fn tohctl(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, tmoctl::Tohctl, Tmoctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,tmoctl::Tohctl, Tmoctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timeout Operation Mode Selection"]
    #[inline(always)]
    pub fn tomds(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, tmoctl::Tomds, Tmoctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,tmoctl::Tomds, Tmoctl_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "16bit-timeout"]
        pub const _00: Self = Self::new(0);
        #[doc = "14bit-timeout"]
        pub const _01: Self = Self::new(1);
        #[doc = "8bit-timeout"]
        pub const _10: Self = Self::new(2);
        #[doc = "6bit-timeout"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tolctl_SPEC;
    pub type Tolctl = crate::EnumBitfieldStruct<u8, Tolctl_SPEC>;
    impl Tolctl {
        #[doc = "Count is disabled while the I3C_SCL line is at a low level."]
        pub const _0: Self = Self::new(0);
        #[doc = "Count is enabled while the I3C_SCL line is at a low level."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tohctl_SPEC;
    pub type Tohctl = crate::EnumBitfieldStruct<u8, Tohctl_SPEC>;
    impl Tohctl {
        #[doc = "Count is disabled while the I3C_SCL line is at a high level."]
        pub const _0: Self = Self::new(0);
        #[doc = "Count is enabled while the I3C_SCL line is at a high level."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tomds_SPEC;
    pub type Tomds = crate::EnumBitfieldStruct<u8, Tomds_SPEC>;
    impl Tomds {
        #[doc = "Timeout is detected during the following conditions: The bus is busy (BCST.BFREF = 0) in master mode.I3C’s own slave address is detected and the bus is busy in slave mode.The bus is free (BCST.BFREF = 1) while generation of a START condition is requested (CNDCTL.STCND = 1)."]
        pub const _00: Self = Self::new(0);
        #[doc = "Timeout is detected while the bus is busy."]
        pub const _01: Self = Self::new(1);
        #[doc = "Timeout is detected while the bus is free."]
        pub const _10: Self = Self::new(2);
        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wuctl_SPEC;
impl crate::sealed::RegSpec for Wuctl_SPEC {
    type DataType = u32;
}
#[doc = "Wake-up Unit Control Register"]
pub type Wuctl = crate::RegValueT<Wuctl_SPEC>;

impl Wuctl {
    #[doc = "Wake-up Acknowledge Selection"]
    #[inline(always)]
    pub fn wuacks(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Wuctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Wuctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Wake-up Analog Noise Filter Selection"]
    #[inline(always)]
    pub fn wuanfs(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, wuctl::Wuanfs, Wuctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,wuctl::Wuanfs, Wuctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Wake-up function PCLK Synchronous Enable"]
    #[inline(always)]
    pub fn wufsyne(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, wuctl::Wufsyne, Wuctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,wuctl::Wufsyne, Wuctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Wake-up function Enable"]
    #[inline(always)]
    pub fn wufe(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, wuctl::Wufe, Wuctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,wuctl::Wufe, Wuctl_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Do not add the Wake-up analog filter."]
        pub const _0: Self = Self::new(0);
        #[doc = "Add the Wake-up analog filter."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wufsyne_SPEC;
    pub type Wufsyne = crate::EnumBitfieldStruct<u8, Wufsyne_SPEC>;
    impl Wufsyne {
        #[doc = "I3C asynchronous circuit enable"]
        pub const _0: Self = Self::new(0);
        #[doc = "I3C synchronous circuit enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wufe_SPEC;
    pub type Wufe = crate::EnumBitfieldStruct<u8, Wufe_SPEC>;
    impl Wufe {
        #[doc = "Wake-up function disables"]
        pub const _0: Self = Self::new(0);
        #[doc = "Wake-up function enables"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ackctl_SPEC;
impl crate::sealed::RegSpec for Ackctl_SPEC {
    type DataType = u32;
}
#[doc = "Acknowledge Control Register"]
pub type Ackctl = crate::RegValueT<Ackctl_SPEC>;

impl Ackctl {
    #[doc = "Acknowledge Reception"]
    #[inline(always)]
    pub fn ackr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ackctl::Ackr, Ackctl_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,ackctl::Ackr, Ackctl_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Acknowledge Transmission"]
    #[inline(always)]
    pub fn ackt(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ackctl::Ackt, Ackctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,ackctl::Ackt, Ackctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ACKT Write Protect"]
    #[inline(always)]
    pub fn acktwp(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, ackctl::Acktwp, Ackctl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<2,0x1,1,0,ackctl::Acktwp, Ackctl_SPEC,crate::common::W>::from_register(self,0)
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
        #[doc = "A 0 is received as the acknowledge bit (ACK reception)."]
        pub const _0: Self = Self::new(0);
        #[doc = "A 1 is received as the acknowledge bit (NACK reception)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ackt_SPEC;
    pub type Ackt = crate::EnumBitfieldStruct<u8, Ackt_SPEC>;
    impl Ackt {
        #[doc = "A 0 is sent as the acknowledge bit (ACK transmission)."]
        pub const _0: Self = Self::new(0);
        #[doc = "A 1 is sent as the acknowledge bit (NACK transmission)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Acktwp_SPEC;
    pub type Acktwp = crate::EnumBitfieldStruct<u8, Acktwp_SPEC>;
    impl Acktwp {
        #[doc = "The ACKT bit are protected."]
        pub const _0: Self = Self::new(0);
        #[doc = "The ACKT bit can be written (when writing simultaneously with the value of the target bit). This bit is read as 0."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scstrctl_SPEC;
impl crate::sealed::RegSpec for Scstrctl_SPEC {
    type DataType = u32;
}
#[doc = "SCL Stretch Control Register"]
pub type Scstrctl = crate::RegValueT<Scstrctl_SPEC>;

impl Scstrctl {
    #[doc = "Acknowledge Transmission Wait Enable"]
    #[inline(always)]
    pub fn acktwe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
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
            Scstrctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Receive Wait Enable"]
    #[inline(always)]
    pub fn rwe(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, scstrctl::Rwe, Scstrctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,scstrctl::Rwe, Scstrctl_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "NTST.RDBFF0 is set at the rising edge of the ninth SCL clock cycle. (The I3C_SCL line is not held low at the falling edge of the eighth clock cycle.)"]
        pub const _0: Self = Self::new(0);
        #[doc = "NTST.RDBFF0 is set at the rising edge of the eighth SCL clock cycle. (The I3C_SCL line is held low at the falling edge of the eighth clock cycle.) Low-hold is released by writing a value to the ACKCTL.ACKT bit."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rwe_SPEC;
    pub type Rwe = crate::EnumBitfieldStruct<u8, Rwe_SPEC>;
    impl Rwe {
        #[doc = "No WAIT (The period between ninth clock cycle and first clock cycle is not held low.)"]
        pub const _0: Self = Self::new(0);
        #[doc = "WAIT (The period between ninth clock cycle and first clock cycle is held low.) Low-hold is released by reading NTDTBP0."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scstlctl_SPEC;
impl crate::sealed::RegSpec for Scstlctl_SPEC {
    type DataType = u32;
}
#[doc = "SCL Stalling Control Register"]
pub type Scstlctl = crate::RegValueT<Scstlctl_SPEC>;

impl Scstlctl {
    #[doc = "Stalling Cycle"]
    #[inline(always)]
    pub fn stlcyc(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Scstlctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Scstlctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Assigned Address Phase Enable"]
    #[inline(always)]
    pub fn aape(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, scstlctl::Aape, Scstlctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            scstlctl::Aape,
            Scstlctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Parity Phase Enable"]
    #[inline(always)]
    pub fn parpe(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
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
            Scstlctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ACK phase Enable"]
    #[inline(always)]
    pub fn ackpe(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
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
        #[doc = "Does not stall the SCL clock during the address assignment phase."]
        pub const _0: Self = Self::new(0);
        #[doc = "Stall the SCL clock during address assignment phase."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Parpe_SPEC;
    pub type Parpe = crate::EnumBitfieldStruct<u8, Parpe_SPEC>;
    impl Parpe {
        #[doc = "Does not stall the SCL clock during the parity bit period."]
        pub const _0: Self = Self::new(0);
        #[doc = "Stall the SCL clock during the parity bit period."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ackpe_SPEC;
    pub type Ackpe = crate::EnumBitfieldStruct<u8, Ackpe_SPEC>;
    impl Ackpe {
        #[doc = "Does not stall the SCL clock during the ACK/NACK phase."]
        pub const _0: Self = Self::new(0);
        #[doc = "Stall the SCL clock during the ACK/NACK phase."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Svtdlg0_SPEC;
impl crate::sealed::RegSpec for Svtdlg0_SPEC {
    type DataType = u32;
}
#[doc = "Slave Transfer Data Length Register 0"]
pub type Svtdlg0 = crate::RegValueT<Svtdlg0_SPEC>;

impl Svtdlg0 {
    #[doc = "Slave Transfer Data Length"]
    #[inline(always)]
    pub fn stdlg(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Svtdlg0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Svtdlg0_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct Cndctl_SPEC;
impl crate::sealed::RegSpec for Cndctl_SPEC {
    type DataType = u32;
}
#[doc = "Condition Control Register"]
pub type Cndctl = crate::RegValueT<Cndctl_SPEC>;

impl Cndctl {
    #[doc = "START (S) Condition Issuance"]
    #[inline(always)]
    pub fn stcnd(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, cndctl::Stcnd, Cndctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,cndctl::Stcnd, Cndctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Repeated START (Sr) Condition Issuance"]
    #[inline(always)]
    pub fn srcnd(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, cndctl::Srcnd, Cndctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,cndctl::Srcnd, Cndctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "STOP (P) Condition Issuance"]
    #[inline(always)]
    pub fn spcnd(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, cndctl::Spcnd, Cndctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,cndctl::Spcnd, Cndctl_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Does not request to issue a START condition."]
        pub const _0: Self = Self::new(0);
        #[doc = "Requests to issue a START condition."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Srcnd_SPEC;
    pub type Srcnd = crate::EnumBitfieldStruct<u8, Srcnd_SPEC>;
    impl Srcnd {
        #[doc = "Does not request to issue a Repeated START condition."]
        pub const _0: Self = Self::new(0);
        #[doc = "Requests to issue a Repeated START condition."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spcnd_SPEC;
    pub type Spcnd = crate::EnumBitfieldStruct<u8, Spcnd_SPEC>;
    impl Spcnd {
        #[doc = "Does not request to issue a STOP condition."]
        pub const _0: Self = Self::new(0);
        #[doc = "Requests to issue a STOP condition."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ncmdqp_SPEC;
impl crate::sealed::RegSpec for Ncmdqp_SPEC {
    type DataType = u32;
}
#[doc = "Normal Command Queue Port Register"]
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
#[doc = "Normal Response Queue Port Register"]
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
#[doc = "Normal Transfer Data Buffer Port Register 0"]
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
#[doc = "Normal Transfer Data Buffer Port Register 0"]
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
#[doc = "Normal IBI Queue Port Register"]
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
#[doc = "Normal Receive Status Queue Port Register"]
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
pub struct Nqthctl_SPEC;
impl crate::sealed::RegSpec for Nqthctl_SPEC {
    type DataType = u32;
}
#[doc = "Normal Queue Threshold Control Register"]
pub type Nqthctl = crate::RegValueT<Nqthctl_SPEC>;

impl Nqthctl {
    #[doc = "Normal Command Queue Threshold"]
    #[inline(always)]
    pub fn cmdqth(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, nqthctl::Cmdqth, Nqthctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            nqthctl::Cmdqth,
            Nqthctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Normal Response Queue Threshold"]
    #[inline(always)]
    pub fn rspqth(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, nqthctl::Rspqth, Nqthctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            nqthctl::Rspqth,
            Nqthctl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Normal IBI Data Segment Size"]
    #[inline(always)]
    pub fn ibidssz(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Nqthctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Nqthctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Normal IBI Queue Threshold"]
    #[inline(always)]
    pub fn ibiqth(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
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
        #[doc = "Interrupt is issued when Normal Command Queue is completely empty."]
        pub const _0_X_00: Self = Self::new(0);
        #[doc = "Interrupt is issued when Normal Command Queue contains N empties. (N = CMDQTH\\[7:0\\])"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rspqth_SPEC;
    pub type Rspqth = crate::EnumBitfieldStruct<u8, Rspqth_SPEC>;
    impl Rspqth {
        #[doc = "Interrupt is issued when Normal Response Queue contains 1 entry (DWORD)."]
        pub const _0_X_00: Self = Self::new(0);
        #[doc = "Interrupt is triggered when Normal Response Queue contains N+1 entries (DWORD). (N = CMDQTH\\[7:0\\])"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ibiqth_SPEC;
    pub type Ibiqth = crate::EnumBitfieldStruct<u8, Ibiqth_SPEC>;
    impl Ibiqth {
        #[doc = "I3C Protocol mode (Master): Interrupt is generated when the Outstanding IBI Status count is 1 or more. I3C Protocol mode (Slave): Interrupt is issued when IBI Data Buffer is completely empty."]
        pub const _0_X_00: Self = Self::new(0);
        #[doc = "I3C Protocol mode (Master): Interrupt is generated when the Outstanding IBI Status count is N + 1 or more. (N = CMDQTH\\[7:0\\]) I3C Protocol mode (Slave): Interrupt is issued when IBI Data Buffer contains N empties."]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ntbthctl0_SPEC;
impl crate::sealed::RegSpec for Ntbthctl0_SPEC {
    type DataType = u32;
}
#[doc = "Normal Transfer Data Buffer Threshold Control Register 0"]
pub type Ntbthctl0 = crate::RegValueT<Ntbthctl0_SPEC>;

impl Ntbthctl0 {
    #[doc = "Normal Tx Data Buffer Threshold"]
    #[inline(always)]
    pub fn txdbth(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
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
            Ntbthctl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Normal Rx Data Buffer Threshold"]
    #[inline(always)]
    pub fn rxdbth(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
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
            Ntbthctl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Normal Tx Start Threshold"]
    #[inline(always)]
    pub fn txstth(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
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
            Ntbthctl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Normal Rx Start Threshold"]
    #[inline(always)]
    pub fn rxstth(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
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
        #[doc = "Interrupt triggers at 2 Tx Buffer empties, DWORDs"]
        pub const _000: Self = Self::new(0);
        #[doc = "Reserved"]
        pub const _001: Self = Self::new(1);
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxdbth_SPEC;
    pub type Rxdbth = crate::EnumBitfieldStruct<u8, Rxdbth_SPEC>;
    impl Rxdbth {
        #[doc = "Interrupt triggers at 2 Rx Buffer entries, DWORDs"]
        pub const _000: Self = Self::new(0);
        #[doc = "Reserved"]
        pub const _001: Self = Self::new(1);
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txstth_SPEC;
    pub type Txstth = crate::EnumBitfieldStruct<u8, Txstth_SPEC>;
    impl Txstth {
        #[doc = "Wait for 2 entry DWORDs"]
        pub const _000: Self = Self::new(0);
        #[doc = "Reserved"]
        pub const _001: Self = Self::new(1);
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxstth_SPEC;
    pub type Rxstth = crate::EnumBitfieldStruct<u8, Rxstth_SPEC>;
    impl Rxstth {
        #[doc = "Wait for 2 empty DWORDs"]
        pub const _000: Self = Self::new(0);
        #[doc = "Reserved"]
        pub const _001: Self = Self::new(1);
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nrqthctl_SPEC;
impl crate::sealed::RegSpec for Nrqthctl_SPEC {
    type DataType = u32;
}
#[doc = "Normal Receive Status Queue Threshold Control Register"]
pub type Nrqthctl = crate::RegValueT<Nrqthctl_SPEC>;

impl Nrqthctl {
    #[doc = "Normal Receive Status Queue Threshold"]
    #[inline(always)]
    pub fn rsqth(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
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
        #[doc = "Interrupt is issued when Normal Receive Status Queue contains 1 entry (DWORD)."]
        pub const _0_X_00: Self = Self::new(0);
        #[doc = "Interrupt is triggered when Normal Receive Status Queue contains N+1 entries (DWORD). (N = RSQTH\\[7:0\\])"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bst_SPEC;
impl crate::sealed::RegSpec for Bst_SPEC {
    type DataType = u32;
}
#[doc = "Bus Status Register"]
pub type Bst = crate::RegValueT<Bst_SPEC>;

impl Bst {
    #[doc = "START Condition Detection Flag"]
    #[inline(always)]
    pub fn stcnddf(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, bst::Stcnddf, Bst_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,bst::Stcnddf, Bst_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "STOP Condition Detection Flag"]
    #[inline(always)]
    pub fn spcnddf(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, bst::Spcnddf, Bst_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,bst::Spcnddf, Bst_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HDR Exit Pattern Detection Flag"]
    #[inline(always)]
    pub fn hdrexdf(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, bst::Hdrexdf, Bst_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,bst::Hdrexdf, Bst_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "NACK Detection Flag"]
    #[inline(always)]
    pub fn nackdf(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, bst::Nackdf, Bst_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,bst::Nackdf, Bst_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmit End Flag"]
    #[inline(always)]
    pub fn tendf(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, bst::Tendf, Bst_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1,1,0,bst::Tendf, Bst_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Arbitration Lost Flag"]
    #[inline(always)]
    pub fn alf(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, bst::Alf, Bst_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1,1,0,bst::Alf, Bst_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timeout Detection Flag"]
    #[inline(always)]
    pub fn todf(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, bst::Todf, Bst_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x1,1,0,bst::Todf, Bst_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Wake-up Condition Detection Flag"]
    #[inline(always)]
    pub fn wucnddf(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, bst::Wucnddf, Bst_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,bst::Wucnddf, Bst_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "START condition is not detected."]
        pub const _0: Self = Self::new(0);
        #[doc = "START condition is detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spcnddf_SPEC;
    pub type Spcnddf = crate::EnumBitfieldStruct<u8, Spcnddf_SPEC>;
    impl Spcnddf {
        #[doc = "STOP condition is not detected."]
        pub const _0: Self = Self::new(0);
        #[doc = "STOP condition is detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hdrexdf_SPEC;
    pub type Hdrexdf = crate::EnumBitfieldStruct<u8, Hdrexdf_SPEC>;
    impl Hdrexdf {
        #[doc = "HDR Exit Pattern is not detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "HDR Exit Pattern is detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nackdf_SPEC;
    pub type Nackdf = crate::EnumBitfieldStruct<u8, Nackdf_SPEC>;
    impl Nackdf {
        #[doc = "NACK is not detected."]
        pub const _0: Self = Self::new(0);
        #[doc = "NACK is detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tendf_SPEC;
    pub type Tendf = crate::EnumBitfieldStruct<u8, Tendf_SPEC>;
    impl Tendf {
        #[doc = "Data is being transmitted."]
        pub const _0: Self = Self::new(0);
        #[doc = "Data has been transmitted."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Alf_SPEC;
    pub type Alf = crate::EnumBitfieldStruct<u8, Alf_SPEC>;
    impl Alf {
        #[doc = "Arbitration is not lost"]
        pub const _0: Self = Self::new(0);
        #[doc = "Arbitration is lost."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Todf_SPEC;
    pub type Todf = crate::EnumBitfieldStruct<u8, Todf_SPEC>;
    impl Todf {
        #[doc = "Timeout is not detected."]
        pub const _0: Self = Self::new(0);
        #[doc = "Timeout is detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wucnddf_SPEC;
    pub type Wucnddf = crate::EnumBitfieldStruct<u8, Wucnddf_SPEC>;
    impl Wucnddf {
        #[doc = "Wake-up is not detected."]
        pub const _0: Self = Self::new(0);
        #[doc = "Wake-up is detected."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bste_SPEC;
impl crate::sealed::RegSpec for Bste_SPEC {
    type DataType = u32;
}
#[doc = "Bus Status Enable Register"]
pub type Bste = crate::RegValueT<Bste_SPEC>;

impl Bste {
    #[doc = "START Condition Detection Enable"]
    #[inline(always)]
    pub fn stcndde(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, bste::Stcndde, Bste_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,bste::Stcndde, Bste_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "STOP Condition Detection Enable"]
    #[inline(always)]
    pub fn spcndde(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, bste::Spcndde, Bste_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,bste::Spcndde, Bste_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HDR Exit Pattern Detection Enable"]
    #[inline(always)]
    pub fn hdrexde(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, bste::Hdrexde, Bste_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,bste::Hdrexde, Bste_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "NACK Detection Enable"]
    #[inline(always)]
    pub fn nackde(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, bste::Nackde, Bste_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,bste::Nackde, Bste_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmit End Enable"]
    #[inline(always)]
    pub fn tende(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, bste::Tende, Bste_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1,1,0,bste::Tende, Bste_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Arbitration Lost Enable"]
    #[inline(always)]
    pub fn ale(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, bste::Ale, Bste_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1,1,0,bste::Ale, Bste_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timeout Detection Enable"]
    #[inline(always)]
    pub fn tode(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, bste::Tode, Bste_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x1,1,0,bste::Tode, Bste_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Wake-up Condition Detection Enable"]
    #[inline(always)]
    pub fn wucndde(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, bste::Wucndde, Bste_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,bste::Wucndde, Bste_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Disables START condition Detection Interrupt Status logging."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables START condition Detection Interrupt Status logging."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spcndde_SPEC;
    pub type Spcndde = crate::EnumBitfieldStruct<u8, Spcndde_SPEC>;
    impl Spcndde {
        #[doc = "Disables STOP condition Detection Interrupt Status logging."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables STOP condition Detection Interrupt Status logging."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hdrexde_SPEC;
    pub type Hdrexde = crate::EnumBitfieldStruct<u8, Hdrexde_SPEC>;
    impl Hdrexde {
        #[doc = "Disables HDR Exit Pattern Detection Interrupt Status logging."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables HDR Exit Pattern Detection Interrupt Status logging."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nackde_SPEC;
    pub type Nackde = crate::EnumBitfieldStruct<u8, Nackde_SPEC>;
    impl Nackde {
        #[doc = "Disables NACK Detection Interrupt Status logging."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables NACK Detection Interrupt Status logging."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tende_SPEC;
    pub type Tende = crate::EnumBitfieldStruct<u8, Tende_SPEC>;
    impl Tende {
        #[doc = "Disables Transmit End Interrupt Status logging."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables Transmit End Interrupt Status logging."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ale_SPEC;
    pub type Ale = crate::EnumBitfieldStruct<u8, Ale_SPEC>;
    impl Ale {
        #[doc = "Disables Arbitration Lost Interrupt Status logging."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables Arbitration Lost Interrupt Status logging."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tode_SPEC;
    pub type Tode = crate::EnumBitfieldStruct<u8, Tode_SPEC>;
    impl Tode {
        #[doc = "Disables Timeout Detection Interrupt Status logging."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables Timeout Detection Interrupt Status logging."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wucndde_SPEC;
    pub type Wucndde = crate::EnumBitfieldStruct<u8, Wucndde_SPEC>;
    impl Wucndde {
        #[doc = "Disables Wake-up Condition Detection Status logging."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables Wake-up Condition Detection Status logging."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bie_SPEC;
impl crate::sealed::RegSpec for Bie_SPEC {
    type DataType = u32;
}
#[doc = "Bus Interrupt Enable Register"]
pub type Bie = crate::RegValueT<Bie_SPEC>;

impl Bie {
    #[doc = "START Condition Detection Interrupt Enable"]
    #[inline(always)]
    pub fn stcnddie(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, bie::Stcnddie, Bie_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,bie::Stcnddie, Bie_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "STOP Condition Detection Interrupt Enable"]
    #[inline(always)]
    pub fn spcnddie(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, bie::Spcnddie, Bie_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,bie::Spcnddie, Bie_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HDR Exit Pattern Detection Interrupt Enable"]
    #[inline(always)]
    pub fn hdrexdie(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, bie::Hdrexdie, Bie_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,bie::Hdrexdie, Bie_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "NACK Detection Interrupt Enable"]
    #[inline(always)]
    pub fn nackdie(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, bie::Nackdie, Bie_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,bie::Nackdie, Bie_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmit End Interrupt Enable"]
    #[inline(always)]
    pub fn tendie(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, bie::Tendie, Bie_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1,1,0,bie::Tendie, Bie_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    pub fn alie(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, bie::Alie, Bie_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1,1,0,bie::Alie, Bie_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timeout Detection Interrupt Enable"]
    #[inline(always)]
    pub fn todie(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, bie::Todie, Bie_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x1,1,0,bie::Todie, Bie_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Wake-up Condition Detection Interrupt Enable"]
    #[inline(always)]
    pub fn wucnddie(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, bie::Wucnddie, Bie_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,bie::Wucnddie, Bie_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Disables START condition Detection Interrupt Signal."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables START condition Detection Interrupt Signal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spcnddie_SPEC;
    pub type Spcnddie = crate::EnumBitfieldStruct<u8, Spcnddie_SPEC>;
    impl Spcnddie {
        #[doc = "Disables STOP condition Detection Interrupt Signal."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables STOP condition Detection Interrupt Signal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hdrexdie_SPEC;
    pub type Hdrexdie = crate::EnumBitfieldStruct<u8, Hdrexdie_SPEC>;
    impl Hdrexdie {
        #[doc = "Disables HDR Exit Pattern Detection Interrupt Signal."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables HDR Exit Pattern Detection Interrupt Signal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nackdie_SPEC;
    pub type Nackdie = crate::EnumBitfieldStruct<u8, Nackdie_SPEC>;
    impl Nackdie {
        #[doc = "Disables NACK Detection Interrupt Signal."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables NACK Detection Interrupt Signal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tendie_SPEC;
    pub type Tendie = crate::EnumBitfieldStruct<u8, Tendie_SPEC>;
    impl Tendie {
        #[doc = "Disables Transmit End Interrupt Signal."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables Transmit End Interrupt Signal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Alie_SPEC;
    pub type Alie = crate::EnumBitfieldStruct<u8, Alie_SPEC>;
    impl Alie {
        #[doc = "Disables Arbitration Lost Interrupt Signal."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables Arbitration Lost Interrupt Signal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Todie_SPEC;
    pub type Todie = crate::EnumBitfieldStruct<u8, Todie_SPEC>;
    impl Todie {
        #[doc = "Disables Timeout Detection Interrupt Signal."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables Timeout Detection Interrupt Signal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wucnddie_SPEC;
    pub type Wucnddie = crate::EnumBitfieldStruct<u8, Wucnddie_SPEC>;
    impl Wucnddie {
        #[doc = "Disables Wake-up Condition Detection Interrupt Signal."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables Wake-up Condition Detection Interrupt Signal."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bstfc_SPEC;
impl crate::sealed::RegSpec for Bstfc_SPEC {
    type DataType = u32;
}
#[doc = "Bus Status Force Register"]
pub type Bstfc = crate::RegValueT<Bstfc_SPEC>;

impl Bstfc {
    #[doc = "START condition Detection Force"]
    #[inline(always)]
    pub fn stcnddfc(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, bstfc::Stcnddfc, Bstfc_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0x1,1,0,bstfc::Stcnddfc, Bstfc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "STOP condition Detection Force"]
    #[inline(always)]
    pub fn spcnddfc(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, bstfc::Spcnddfc, Bstfc_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<1,0x1,1,0,bstfc::Spcnddfc, Bstfc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "HDR Exit Pattern Detection Force"]
    #[inline(always)]
    pub fn hdrexdfc(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, bstfc::Hdrexdfc, Bstfc_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<2,0x1,1,0,bstfc::Hdrexdfc, Bstfc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "NACK Detection Force"]
    #[inline(always)]
    pub fn nackdfc(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, bstfc::Nackdfc, Bstfc_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<4,0x1,1,0,bstfc::Nackdfc, Bstfc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Transmit End Force"]
    #[inline(always)]
    pub fn tendfc(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, bstfc::Tendfc, Bstfc_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0x1,1,0,bstfc::Tendfc, Bstfc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Arbitration Lost Force"]
    #[inline(always)]
    pub fn alfc(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, bstfc::Alfc, Bstfc_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<16,0x1,1,0,bstfc::Alfc, Bstfc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Timeout Detection Force"]
    #[inline(always)]
    pub fn todfc(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, bstfc::Todfc, Bstfc_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<20,0x1,1,0,bstfc::Todfc, Bstfc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Wake-up Condition Detection Force"]
    #[inline(always)]
    pub fn wucnddfc(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, bstfc::Wucnddfc, Bstfc_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<24,0x1,1,0,bstfc::Wucnddfc, Bstfc_SPEC,crate::common::W>::from_register(self,0)
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
        #[doc = "Not Force START condition Detection Interrupt for software testing."]
        pub const _0: Self = Self::new(0);
        #[doc = "Force START condition Detection Interrupt for software testing."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spcnddfc_SPEC;
    pub type Spcnddfc = crate::EnumBitfieldStruct<u8, Spcnddfc_SPEC>;
    impl Spcnddfc {
        #[doc = "Not Force STOP condition Detection Interrupt for software testing."]
        pub const _0: Self = Self::new(0);
        #[doc = "Force STOP condition Detection Interrupt for software testing."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hdrexdfc_SPEC;
    pub type Hdrexdfc = crate::EnumBitfieldStruct<u8, Hdrexdfc_SPEC>;
    impl Hdrexdfc {
        #[doc = "Not Force HDR Exit Pattern Detection Interrupt for software testing."]
        pub const _0: Self = Self::new(0);
        #[doc = "Force HDR Exit Pattern Detection Interrupt for software testing."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nackdfc_SPEC;
    pub type Nackdfc = crate::EnumBitfieldStruct<u8, Nackdfc_SPEC>;
    impl Nackdfc {
        #[doc = "Not Force NACK Detection Interrupt for software testing."]
        pub const _0: Self = Self::new(0);
        #[doc = "Force NACK Detection Interrupt for software testing."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tendfc_SPEC;
    pub type Tendfc = crate::EnumBitfieldStruct<u8, Tendfc_SPEC>;
    impl Tendfc {
        #[doc = "Not Force Transmit End Interrupt for software testing."]
        pub const _0: Self = Self::new(0);
        #[doc = "Force Transmit End Interrupt for software testing."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Alfc_SPEC;
    pub type Alfc = crate::EnumBitfieldStruct<u8, Alfc_SPEC>;
    impl Alfc {
        #[doc = "Not Force Arbitration Lost Interrupt for software testing."]
        pub const _0: Self = Self::new(0);
        #[doc = "Force Arbitration Lost Interrupt for software testing."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Todfc_SPEC;
    pub type Todfc = crate::EnumBitfieldStruct<u8, Todfc_SPEC>;
    impl Todfc {
        #[doc = "Not Force Timeout Detection Interrupt for software testing."]
        pub const _0: Self = Self::new(0);
        #[doc = "Force Timeout Detection Interrupt for software testing."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wucnddfc_SPEC;
    pub type Wucnddfc = crate::EnumBitfieldStruct<u8, Wucnddfc_SPEC>;
    impl Wucnddfc {
        #[doc = "Not Force Wake-up Condition Detection Interrupt for software testing."]
        pub const _0: Self = Self::new(0);
        #[doc = "Force Wake-up Condition Detection Interrupt for software testing."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ntst_SPEC;
impl crate::sealed::RegSpec for Ntst_SPEC {
    type DataType = u32;
}
#[doc = "Normal Transfer Status Register"]
pub type Ntst = crate::RegValueT<Ntst_SPEC>;

impl Ntst {
    #[doc = "Normal Tx Data Buffer Empty Flag 0"]
    #[inline(always)]
    pub fn tdbef0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ntst::Tdbef0, Ntst_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,ntst::Tdbef0, Ntst_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Normal Rx Data Buffer Full Flag 0"]
    #[inline(always)]
    pub fn rdbff0(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ntst::Rdbff0, Ntst_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,ntst::Rdbff0, Ntst_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Normal IBI Queue Empty/Full Flag"]
    #[inline(always)]
    pub fn ibiqeff(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, ntst::Ibiqeff, Ntst_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,ntst::Ibiqeff, Ntst_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Normal Command Queue Empty Flag"]
    #[inline(always)]
    pub fn cmdqef(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, ntst::Cmdqef, Ntst_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,ntst::Cmdqef, Ntst_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Normal Response Queue Full Flag"]
    #[inline(always)]
    pub fn rspqff(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, ntst::Rspqff, Ntst_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,ntst::Rspqff, Ntst_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Normal Transfer Abort Flag"]
    #[inline(always)]
    pub fn tabtf(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, ntst::Tabtf, Ntst_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1,1,0,ntst::Tabtf, Ntst_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Normal Transfer Error Flag"]
    #[inline(always)]
    pub fn tef(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, ntst::Tef, Ntst_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x1,1,0,ntst::Tef, Ntst_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Normal Receive Status Queue Full Flag"]
    #[inline(always)]
    pub fn rsqff(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, ntst::Rsqff, Ntst_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,ntst::Rsqff, Ntst_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "For I2C protocol mode: PRTS.PRTMD bit = 1. Normal Tx Data Buffer contains transmit data. For I3C protocol mode: PRTS.PRTMD bit = 0. The number of empties in the Normal Tx Data Buffer is less than the NTBTHCTL0.TXDBTH\\[2:0\\] threshold."]
        pub const _0: Self = Self::new(0);
        #[doc = "For I2C protocol mode: PRTS.PRTMD bit = 1. Normal Tx Data Buffer contains no transmit data. For I3C protocol mode: PRTS.PRTMD bit = 0. The number of empties in the Normal Tx Data Buffer is the NTBTHCTL0.TXDBTH\\[2:0\\] threshold or more."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdbff0_SPEC;
    pub type Rdbff0 = crate::EnumBitfieldStruct<u8, Rdbff0_SPEC>;
    impl Rdbff0 {
        #[doc = "For I2C protocol mode: PRTS.PRTMD bit = 1. Normal Rx Data Buffer contains no receive data. For I3C Protocol mode: PRTS.PRTMD bit = 0. The number of entries in the Normal Rx Data Buffer is less than the NTBTHCTL0.RXDBTH\\[2:0\\] threshold."]
        pub const _0: Self = Self::new(0);
        #[doc = "For I2C protocol mode: PRTS.PRTMD bit = 1. Normal Rx Data Buffer contains receive data. For I3C Protocol mode: PRTS.PRTMD bit = 0. The number of entries in the Normal Rx Data Buffer is the NTBTHCTL0.RXDBTH\\[2:0\\] threshold or more."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ibiqeff_SPEC;
    pub type Ibiqeff = crate::EnumBitfieldStruct<u8, Ibiqeff_SPEC>;
    impl Ibiqeff {
        #[doc = "For I3C protocol mode (Master): PRTS.PRTMD bit = 0, PRSST.CRMS bit = 1. The number of Normal IBI Queue entries is the NQTHCTL.IBIQTH threshold or less. For I3C protocol mode (Slave) : PRTS.PRTMD bit = 0, PRSST.CRMS bit = 0. If the NQTHCTL.IBIQTH = 0: The number of IBI Data Buffer empties is less than the IBI Data Buffer size. If the NQTHCTL.IBIQTH is other than 0: The number of IBI Data Buffer empties is less than the NQTHCTL.IBIQTH threshold."]
        pub const _0: Self = Self::new(0);
        #[doc = "For I3C protocol mode (Master): PRTS.PRTMD bit = 0, PRSST.CRMS bit = 1. The number of Normal IBI Queue entries is more than the NQTHCTL.IBIQTH threshold. For I3C protocol mode (Slave) : PRTS.PRTMD bit = 0, PRSST.CRMS bit = 0. If the NQTHCTL.IBIQTH = 0: The number of IBI Data Buffer empties is the IBI Data Buffer size. If the NQTHCTL.IBIQTH is other than 0: The number of IBI Data Buffer empties is the NQTHCTL.IBIQTH threshold or more."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmdqef_SPEC;
    pub type Cmdqef = crate::EnumBitfieldStruct<u8, Cmdqef_SPEC>;
    impl Cmdqef {
        #[doc = "If the NQTHCTL.CMDQTH = 0: The number of Normal Command Queue empties is less than the Normal Command Queue size. If the NQTHCTL.CMDQTH is other than 0: The number of Normal Command Queue empties is less than the NQTHCTL.CMDQTH threshold."]
        pub const _0: Self = Self::new(0);
        #[doc = "If the NQTHCTL.CMDQTH = 0: The number of Normal Command Queue empties is the Normal Command Queue size. If the NQTHCTL.CMDQTH is other than 0: 1: The number of Normal Command Queue empties is the NQTHCTL.CMDQTH threshold or more."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rspqff_SPEC;
    pub type Rspqff = crate::EnumBitfieldStruct<u8, Rspqff_SPEC>;
    impl Rspqff {
        #[doc = "The number of Normal Response Queue entries is the NQTHCTL.RSPQTH threshold or less."]
        pub const _0: Self = Self::new(0);
        #[doc = "The number of Normal Response Queue entries is more than the NQTHCTL.RSPQTH threshold."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tabtf_SPEC;
    pub type Tabtf = crate::EnumBitfieldStruct<u8, Tabtf_SPEC>;
    impl Tabtf {
        #[doc = "Normal Transfer Abort does not occur."]
        pub const _0: Self = Self::new(0);
        #[doc = "Normal Transfer Abort occur. To clear, write 0 to this bit after 1 state is read."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tef_SPEC;
    pub type Tef = crate::EnumBitfieldStruct<u8, Tef_SPEC>;
    impl Tef {
        #[doc = "Normal Transfer Error does not occur."]
        pub const _0: Self = Self::new(0);
        #[doc = "Normal Transfer Error occurs. To clear, write 0 to this bit after 1 state is read."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rsqff_SPEC;
    pub type Rsqff = crate::EnumBitfieldStruct<u8, Rsqff_SPEC>;
    impl Rsqff {
        #[doc = "The number of Normal Receive Status Queue entries is the NRQTHCTL.RSQTH threshold or less."]
        pub const _0: Self = Self::new(0);
        #[doc = "The number of Normal Receive Status Queue entries is more than the NRQTHCTL.RSQTH threshold."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ntste_SPEC;
impl crate::sealed::RegSpec for Ntste_SPEC {
    type DataType = u32;
}
#[doc = "Normal Transfer Status Enable Register"]
pub type Ntste = crate::RegValueT<Ntste_SPEC>;

impl Ntste {
    #[doc = "Normal Tx Data Buffer Empty Enable 0"]
    #[inline(always)]
    pub fn tdbee0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ntste::Tdbee0, Ntste_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,ntste::Tdbee0, Ntste_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Normal Rx Data Buffer Full Enable 0"]
    #[inline(always)]
    pub fn rdbfe0(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ntste::Rdbfe0, Ntste_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,ntste::Rdbfe0, Ntste_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Normal IBI Queue Empty/Full Enable"]
    #[inline(always)]
    pub fn ibiqefe(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, ntste::Ibiqefe, Ntste_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,ntste::Ibiqefe, Ntste_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Normal Command Queue Empty Enable"]
    #[inline(always)]
    pub fn cmdqee(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, ntste::Cmdqee, Ntste_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,ntste::Cmdqee, Ntste_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Normal Response Queue Full Enable"]
    #[inline(always)]
    pub fn rspqfe(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, ntste::Rspqfe, Ntste_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,ntste::Rspqfe, Ntste_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Normal Transfer Abort Enable"]
    #[inline(always)]
    pub fn tabte(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, ntste::Tabte, Ntste_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,ntste::Tabte, Ntste_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Normal Transfer Error Enable"]
    #[inline(always)]
    pub fn tee(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, ntste::Tee, Ntste_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x1,1,0,ntste::Tee, Ntste_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Normal Receive Status Queue Full Enable"]
    #[inline(always)]
    pub fn rsqfe(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, ntste::Rsqfe, Ntste_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,ntste::Rsqfe, Ntste_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Disables Normal Tx Data Buffer Empty Interrupt Status logging."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables Normal Tx Data Buffer Empty Interrupt Status logging."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdbfe0_SPEC;
    pub type Rdbfe0 = crate::EnumBitfieldStruct<u8, Rdbfe0_SPEC>;
    impl Rdbfe0 {
        #[doc = "Disables Normal Rx Data Buffer Full Interrupt Status logging."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables Normal Rx Data Buffer Full Interrupt Status logging."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ibiqefe_SPEC;
    pub type Ibiqefe = crate::EnumBitfieldStruct<u8, Ibiqefe_SPEC>;
    impl Ibiqefe {
        #[doc = "Disables Normal IBI Queue Empty/Full Interrupt Status logging."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables Normal IBI Queue Empty/Full Interrupt Status logging."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmdqee_SPEC;
    pub type Cmdqee = crate::EnumBitfieldStruct<u8, Cmdqee_SPEC>;
    impl Cmdqee {
        #[doc = "Disables Normal Command Queue Empty Interrupt Status logging."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables Normal Command Queue Empty Interrupt Status logging."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rspqfe_SPEC;
    pub type Rspqfe = crate::EnumBitfieldStruct<u8, Rspqfe_SPEC>;
    impl Rspqfe {
        #[doc = "Disables Normal Response queue Full Interrupt Status logging."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables Normal Response queue Full Interrupt Status logging."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tabte_SPEC;
    pub type Tabte = crate::EnumBitfieldStruct<u8, Tabte_SPEC>;
    impl Tabte {
        #[doc = "Disables Normal Transfer Abort Interrupt Status logging."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables Normal Transfer Abort Interrupt Status logging."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tee_SPEC;
    pub type Tee = crate::EnumBitfieldStruct<u8, Tee_SPEC>;
    impl Tee {
        #[doc = "Disables Normal Transfer Error Interrupt Status logging."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables Normal Transfer Error Interrupt Status logging."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rsqfe_SPEC;
    pub type Rsqfe = crate::EnumBitfieldStruct<u8, Rsqfe_SPEC>;
    impl Rsqfe {
        #[doc = "Disables Normal Receive Status Queue Full Interrupt Status logging."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables Normal Receive Status Queue Full Interrupt Status logging."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ntie_SPEC;
impl crate::sealed::RegSpec for Ntie_SPEC {
    type DataType = u32;
}
#[doc = "Normal Transfer Interrupt Enable Register"]
pub type Ntie = crate::RegValueT<Ntie_SPEC>;

impl Ntie {
    #[doc = "Normal Tx Data Buffer Empty Interrupt Enable 0"]
    #[inline(always)]
    pub fn tdbeie0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ntie::Tdbeie0, Ntie_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,ntie::Tdbeie0, Ntie_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Normal Rx Data Buffer Full Interrupt Enable 0"]
    #[inline(always)]
    pub fn rdbfie0(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ntie::Rdbfie0, Ntie_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,ntie::Rdbfie0, Ntie_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Normal IBI Queue Empty/Full Interrupt Enable"]
    #[inline(always)]
    pub fn ibiqefie(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, ntie::Ibiqefie, Ntie_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,ntie::Ibiqefie, Ntie_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Normal Command Queue Empty Interrupt Enable"]
    #[inline(always)]
    pub fn cmdqeie(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, ntie::Cmdqeie, Ntie_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,ntie::Cmdqeie, Ntie_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Normal Response Queue Full Interrupt Enable"]
    #[inline(always)]
    pub fn rspqfie(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, ntie::Rspqfie, Ntie_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,ntie::Rspqfie, Ntie_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Normal Transfer Abort Interrupt Enable"]
    #[inline(always)]
    pub fn tabtie(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, ntie::Tabtie, Ntie_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,ntie::Tabtie, Ntie_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Normal Transfer Error Interrupt Enable"]
    #[inline(always)]
    pub fn teie(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, ntie::Teie, Ntie_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x1,1,0,ntie::Teie, Ntie_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Normal Receive Status Queue Full Interrupt Enable"]
    #[inline(always)]
    pub fn rsqfie(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, ntie::Rsqfie, Ntie_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,ntie::Rsqfie, Ntie_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Disables Normal Tx Data Buffer Empty Interrupt Signal."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables Normal Tx Data Buffer Empty Interrupt Signal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdbfie0_SPEC;
    pub type Rdbfie0 = crate::EnumBitfieldStruct<u8, Rdbfie0_SPEC>;
    impl Rdbfie0 {
        #[doc = "Disables Normal Rx Data Buffer Full Interrupt Signal."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables Normal Rx Data Buffer Full Interrupt Signal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ibiqefie_SPEC;
    pub type Ibiqefie = crate::EnumBitfieldStruct<u8, Ibiqefie_SPEC>;
    impl Ibiqefie {
        #[doc = "Disables Normal IBI Queue Empty/Full Interrupt Signal."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables Normal IBI Queue Empty/Full Interrupt Signal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmdqeie_SPEC;
    pub type Cmdqeie = crate::EnumBitfieldStruct<u8, Cmdqeie_SPEC>;
    impl Cmdqeie {
        #[doc = "Disables Normal Command Queue Empty Interrupt Signal."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables Normal Command Queue Empty Interrupt Signal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rspqfie_SPEC;
    pub type Rspqfie = crate::EnumBitfieldStruct<u8, Rspqfie_SPEC>;
    impl Rspqfie {
        #[doc = "Disables Normal Response Queue Full Interrupt Signal."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables Normal Response Queue Full Interrupt Signal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tabtie_SPEC;
    pub type Tabtie = crate::EnumBitfieldStruct<u8, Tabtie_SPEC>;
    impl Tabtie {
        #[doc = "Disables Normal Transfer Abort Interrupt Signal."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables Normal Transfer Abort Interrupt Signal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Teie_SPEC;
    pub type Teie = crate::EnumBitfieldStruct<u8, Teie_SPEC>;
    impl Teie {
        #[doc = "Disables Normal Transfer Error Interrupt Signal."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables Normal Transfer Error Interrupt Signal."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rsqfie_SPEC;
    pub type Rsqfie = crate::EnumBitfieldStruct<u8, Rsqfie_SPEC>;
    impl Rsqfie {
        #[doc = "Disables Normal Receive Status Queue Full Interrupt Signal."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables Normal Receive Status Queue Full Interrupt Signal."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ntstfc_SPEC;
impl crate::sealed::RegSpec for Ntstfc_SPEC {
    type DataType = u32;
}
#[doc = "Normal Transfer Status Force Register"]
pub type Ntstfc = crate::RegValueT<Ntstfc_SPEC>;

impl Ntstfc {
    #[doc = "Normal Tx Data Buffer Empty Force 0"]
    #[inline(always)]
    pub fn tdbefc0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ntstfc::Tdbefc0, Ntstfc_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0x1,1,0,ntstfc::Tdbefc0, Ntstfc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Normal Rx Data Buffer Full Force 0"]
    #[inline(always)]
    pub fn rdbffc0(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ntstfc::Rdbffc0, Ntstfc_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<1,0x1,1,0,ntstfc::Rdbffc0, Ntstfc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Normal IBI Queue Empty/Full Force"]
    #[inline(always)]
    pub fn ibiqeffc(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, ntstfc::Ibiqeffc, Ntstfc_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<2,0x1,1,0,ntstfc::Ibiqeffc, Ntstfc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Normal Command Queue Empty Force"]
    #[inline(always)]
    pub fn cmdqefc(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, ntstfc::Cmdqefc, Ntstfc_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<3,0x1,1,0,ntstfc::Cmdqefc, Ntstfc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Normal Response Queue Full Force"]
    #[inline(always)]
    pub fn rspqffc(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, ntstfc::Rspqffc, Ntstfc_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<4,0x1,1,0,ntstfc::Rspqffc, Ntstfc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Normal Transfer Abort Force"]
    #[inline(always)]
    pub fn tabtfc(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, ntstfc::Tabtfc, Ntstfc_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<5,0x1,1,0,ntstfc::Tabtfc, Ntstfc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Normal Transfer Error Force"]
    #[inline(always)]
    pub fn tefc(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, ntstfc::Tefc, Ntstfc_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<9,0x1,1,0,ntstfc::Tefc, Ntstfc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Normal Receive Status Queue Full Force"]
    #[inline(always)]
    pub fn rsqffc(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, ntstfc::Rsqffc, Ntstfc_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<20,0x1,1,0,ntstfc::Rsqffc, Ntstfc_SPEC,crate::common::W>::from_register(self,0)
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
        #[doc = "Not Force Normal Tx Data Buffer Empty Interrupt for software testing."]
        pub const _0: Self = Self::new(0);
        #[doc = "Force Normal Tx Data Buffer Empty Interrupt for software testing."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdbffc0_SPEC;
    pub type Rdbffc0 = crate::EnumBitfieldStruct<u8, Rdbffc0_SPEC>;
    impl Rdbffc0 {
        #[doc = "Not Force Normal Rx Data Buffer Full Interrupt for software testing."]
        pub const _0: Self = Self::new(0);
        #[doc = "Force Normal Rx Data Buffer Full Interrupt for software testing."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ibiqeffc_SPEC;
    pub type Ibiqeffc = crate::EnumBitfieldStruct<u8, Ibiqeffc_SPEC>;
    impl Ibiqeffc {
        #[doc = "Not Force Normal IBI Queue Empty/Full Interrupt for software testing."]
        pub const _0: Self = Self::new(0);
        #[doc = "Force Normal IBI Queue Empty/Full Interrupt for software testing."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmdqefc_SPEC;
    pub type Cmdqefc = crate::EnumBitfieldStruct<u8, Cmdqefc_SPEC>;
    impl Cmdqefc {
        #[doc = "Not Force Normal Command Queue Empty Interrupt for software testing."]
        pub const _0: Self = Self::new(0);
        #[doc = "Force Normal Command Queue Empty Interrupt for software testing."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rspqffc_SPEC;
    pub type Rspqffc = crate::EnumBitfieldStruct<u8, Rspqffc_SPEC>;
    impl Rspqffc {
        #[doc = "Not Force Normal Response Queue Full Interrupt for software testing."]
        pub const _0: Self = Self::new(0);
        #[doc = "Force Normal Response Queue Full Interrupt for software testing."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tabtfc_SPEC;
    pub type Tabtfc = crate::EnumBitfieldStruct<u8, Tabtfc_SPEC>;
    impl Tabtfc {
        #[doc = "Not Force Normal Transfer Abort Interrupt for software testing."]
        pub const _0: Self = Self::new(0);
        #[doc = "Force Normal Transfer Abort Interrupt for software testing."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tefc_SPEC;
    pub type Tefc = crate::EnumBitfieldStruct<u8, Tefc_SPEC>;
    impl Tefc {
        #[doc = "Not Force Normal Transfer Error Interrupt for software testing."]
        pub const _0: Self = Self::new(0);
        #[doc = "Force Normal Transfer Error Interrupt for software testing."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rsqffc_SPEC;
    pub type Rsqffc = crate::EnumBitfieldStruct<u8, Rsqffc_SPEC>;
    impl Rsqffc {
        #[doc = "Not Force Normal Receive Status Queue Full Interrupt for software testing."]
        pub const _0: Self = Self::new(0);
        #[doc = "Force Normal Receive Status Queue Full Interrupt for software testing."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcst_SPEC;
impl crate::sealed::RegSpec for Bcst_SPEC {
    type DataType = u32;
}
#[doc = "Bus Condition Status Register"]
pub type Bcst = crate::RegValueT<Bcst_SPEC>;

impl Bcst {
    #[doc = "Bus Free Detection Flag"]
    #[inline(always)]
    pub fn bfref(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, bcst::Bfref, Bcst_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1,1,0,bcst::Bfref, Bcst_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Bus Available Detection Flag"]
    #[inline(always)]
    pub fn bavlf(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, bcst::Bavlf, Bcst_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,bcst::Bavlf, Bcst_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Bus Idle Detection Flag"]
    #[inline(always)]
    pub fn bidlf(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, bcst::Bidlf, Bcst_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x1,1,0,bcst::Bidlf, Bcst_SPEC,crate::common::R>::from_register(self,0)
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
        #[doc = "Have not Detected Bus Free"]
        pub const _0: Self = Self::new(0);
        #[doc = "Have Detected Bus Free"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bavlf_SPEC;
    pub type Bavlf = crate::EnumBitfieldStruct<u8, Bavlf_SPEC>;
    impl Bavlf {
        #[doc = "Have not Detected Bus Available"]
        pub const _0: Self = Self::new(0);
        #[doc = "Have Detected Bus Available"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bidlf_SPEC;
    pub type Bidlf = crate::EnumBitfieldStruct<u8, Bidlf_SPEC>;
    impl Bidlf {
        #[doc = "Have not Detected Bus Idle"]
        pub const _0: Self = Self::new(0);
        #[doc = "Have Detected Bus Idle"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Svst_SPEC;
impl crate::sealed::RegSpec for Svst_SPEC {
    type DataType = u32;
}
#[doc = "Slave Status Register"]
pub type Svst = crate::RegValueT<Svst_SPEC>;

impl Svst {
    #[doc = "General Call Address Detection Flag"]
    #[inline(always)]
    pub fn gcaf(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, svst::Gcaf, Svst_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,svst::Gcaf, Svst_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Hs-mode Master Code Detection Flag"]
    #[inline(always)]
    pub fn hsmcf(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, svst::Hsmcf, Svst_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1,1,0,svst::Hsmcf, Svst_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Device-ID Address Detection Flag"]
    #[inline(always)]
    pub fn dvidf(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, svst::Dvidf, Svst_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x1,1,0,svst::Dvidf, Svst_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Host Address Detection Flag"]
    #[inline(always)]
    pub fn hoaf(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, svst::Hoaf, Svst_SPEC, crate::common::RW> {
        crate::common::RegisterField::<15,0x1,1,0,svst::Hoaf, Svst_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Slave Address Detection Flag 0"]
    #[inline(always)]
    pub fn svaf0(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, svst::Svaf0, Svst_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,svst::Svaf0, Svst_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "General call address does not detect."]
        pub const _0: Self = Self::new(0);
        #[doc = "General call address detects."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hsmcf_SPEC;
    pub type Hsmcf = crate::EnumBitfieldStruct<u8, Hsmcf_SPEC>;
    impl Hsmcf {
        #[doc = "Hs-mode Master Code does not detect."]
        pub const _0: Self = Self::new(0);
        #[doc = "Hs-mode Master Code detects."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvidf_SPEC;
    pub type Dvidf = crate::EnumBitfieldStruct<u8, Dvidf_SPEC>;
    impl Dvidf {
        #[doc = "Device-ID command does not detect."]
        pub const _0: Self = Self::new(0);
        #[doc = "Device-ID command detects. This bit set to 1 when the first frame received immediately after a START condition is detected matches a value of (device ID (1111 100) + 0\\[W\\])."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hoaf_SPEC;
    pub type Hoaf = crate::EnumBitfieldStruct<u8, Hoaf_SPEC>;
    impl Hoaf {
        #[doc = "Host address does not detect."]
        pub const _0: Self = Self::new(0);
        #[doc = "Host address detects. This bit set to 1 when the received slave address matches the host address (0001 000)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Svaf0_SPEC;
    pub type Svaf0 = crate::EnumBitfieldStruct<u8, Svaf0_SPEC>;
    impl Svaf0 {
        #[doc = "Slave 0 does not detect"]
        pub const _0: Self = Self::new(0);
        #[doc = "Slave 0 detect"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wust_SPEC;
impl crate::sealed::RegSpec for Wust_SPEC {
    type DataType = u32;
}
#[doc = "Wake-up Unit Operating Status Register"]
pub type Wust = crate::RegValueT<Wust_SPEC>;

impl Wust {
    #[doc = "Wake-up function asynchronous operation status flag"]
    #[inline(always)]
    pub fn wuasynf(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, wust::Wuasynf, Wust_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,wust::Wuasynf, Wust_SPEC,crate::common::R>::from_register(self,0)
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
        #[doc = "I3C synchronous circuit enable condition."]
        pub const _0: Self = Self::new(0);
        #[doc = "I3C asynchronous circuit enable condition."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Datbas_SPEC;
impl crate::sealed::RegSpec for Datbas_SPEC {
    type DataType = u32;
}
#[doc = "Device Address Table Basic Register %s"]
pub type Datbas = crate::RegValueT<Datbas_SPEC>;

impl Datbas {
    #[doc = "Device Static Address"]
    #[inline(always)]
    pub fn dvstad(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, Datbas_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8, Datbas_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Device IBI Payload"]
    #[inline(always)]
    pub fn dvibipl(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, datbas::Dvibipl, Datbas_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,datbas::Dvibipl, Datbas_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Device In-Band Slave Interrupt Request Reject"]
    #[inline(always)]
    pub fn dvsirrj(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, datbas::Dvsirrj, Datbas_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,datbas::Dvsirrj, Datbas_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Device In-Band Master Request Reject"]
    #[inline(always)]
    pub fn dvmrrj(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, datbas::Dvmrrj, Datbas_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,datbas::Dvmrrj, Datbas_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Device IBI Time-stamp"]
    #[inline(always)]
    pub fn dvibits(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, datbas::Dvibits, Datbas_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,datbas::Dvibits, Datbas_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Device I3C Dynamic Address"]
    #[inline(always)]
    pub fn dvdyad(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Datbas_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Datbas_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Device NACK Retry Count"]
    #[inline(always)]
    pub fn dvnack(
        self,
    ) -> crate::common::RegisterField<29, 0x3, 1, 0, u8, Datbas_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x3,1,0,u8, Datbas_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Device Type"]
    #[inline(always)]
    pub fn dvtyp(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, datbas::Dvtyp, Datbas_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,datbas::Dvtyp, Datbas_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "IBIs from this Device do not carry a Data Payload."]
        pub const _0: Self = Self::new(0);
        #[doc = "IBIs from this Device do carry a Data Payload."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvsirrj_SPEC;
    pub type Dvsirrj = crate::EnumBitfieldStruct<u8, Dvsirrj_SPEC>;
    impl Dvsirrj {
        #[doc = "This Device shall ACK the SIR."]
        pub const _0: Self = Self::new(0);
        #[doc = "This Device shall NACK the SIR and send the auto-disable CCC."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvmrrj_SPEC;
    pub type Dvmrrj = crate::EnumBitfieldStruct<u8, Dvmrrj_SPEC>;
    impl Dvmrrj {
        #[doc = "This Device shall ACK Master Requests."]
        pub const _0: Self = Self::new(0);
        #[doc = "This Device shall NACK Master Requests and send the auto-disable command."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvibits_SPEC;
    pub type Dvibits = crate::EnumBitfieldStruct<u8, Dvibits_SPEC>;
    impl Dvibits {
        #[doc = "The Master shall not time-stamp IBIs from this Device with Master Time-stamps."]
        pub const _0: Self = Self::new(0);
        #[doc = "The Master shall time-stamp IBIs for this Device with Master Time-stamps."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvtyp_SPEC;
    pub type Dvtyp = crate::EnumBitfieldStruct<u8, Dvtyp_SPEC>;
    impl Dvtyp {
        #[doc = "I3C Device"]
        pub const _0: Self = Self::new(0);
        #[doc = "I2C Device"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Exdatbas_SPEC;
impl crate::sealed::RegSpec for Exdatbas_SPEC {
    type DataType = u32;
}
#[doc = "Extended Device Address Table Basic Register"]
pub type Exdatbas = crate::RegValueT<Exdatbas_SPEC>;

impl Exdatbas {
    #[doc = "Extended Device Static Address"]
    #[inline(always)]
    pub fn edstad(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, Exdatbas_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8, Exdatbas_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Extended Device I3C Dynamic Address"]
    #[inline(always)]
    pub fn eddyad(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Exdatbas_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Exdatbas_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Extended Device NACK Retry Count"]
    #[inline(always)]
    pub fn ednack(
        self,
    ) -> crate::common::RegisterField<29, 0x3, 1, 0, u8, Exdatbas_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x3,1,0,u8, Exdatbas_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Extended Device Type"]
    #[inline(always)]
    pub fn edtyp(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
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
        #[doc = "I3C Device"]
        pub const _0: Self = Self::new(0);
        #[doc = "I2C Device"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdatbas0_SPEC;
impl crate::sealed::RegSpec for Sdatbas0_SPEC {
    type DataType = u32;
}
#[doc = "Slave Device Address Table Basic Register 0"]
pub type Sdatbas0 = crate::RegValueT<Sdatbas0_SPEC>;

impl Sdatbas0 {
    #[doc = "Slave Device Static Address"]
    #[inline(always)]
    pub fn sdstad(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Sdatbas0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, Sdatbas0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Slave Device Address Length Selection"]
    #[inline(always)]
    pub fn sdadls(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
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
            Sdatbas0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Slave Device IBI Payload"]
    #[inline(always)]
    pub fn sdibipl(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
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
            Sdatbas0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Slave Device I3C Dynamic Address"]
    #[inline(always)]
    pub fn sddyad(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, Sdatbas0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7f,1,0,u8, Sdatbas0_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Slave device address length 7 bits selected."]
        pub const _0: Self = Self::new(0);
        #[doc = "Slave device address length 10 bits selected. (I2C device only)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdibipl_SPEC;
    pub type Sdibipl = crate::EnumBitfieldStruct<u8, Sdibipl_SPEC>;
    impl Sdibipl {
        #[doc = "IBIs from this device do not carry a data payload."]
        pub const _0: Self = Self::new(0);
        #[doc = "IBIs from this device carry a data payload."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msdct_SPEC;
impl crate::sealed::RegSpec for Msdct_SPEC {
    type DataType = u32;
}
#[doc = "Master Device Characteristic Table Register %s"]
pub type Msdct = crate::RegValueT<Msdct_SPEC>;

impl Msdct {
    #[doc = "Max Data Speed Limitation"]
    #[inline(always)]
    pub fn rbcr0(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, msdct::Rbcr0, Msdct_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,msdct::Rbcr0, Msdct_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IBI Request Capable"]
    #[inline(always)]
    pub fn rbcr1(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, msdct::Rbcr1, Msdct_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,msdct::Rbcr1, Msdct_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IBI Payload"]
    #[inline(always)]
    pub fn rbcr2(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, msdct::Rbcr2, Msdct_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,msdct::Rbcr2, Msdct_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Offline Capable"]
    #[inline(always)]
    pub fn rbcr3(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, msdct::Rbcr3, Msdct_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,msdct::Rbcr3, Msdct_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bridge Identifier"]
    #[inline(always)]
    pub fn rbcr4(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, msdct::Rbcr4, Msdct_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,msdct::Rbcr4, Msdct_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SDR Only / SDR and HDR Capable"]
    #[inline(always)]
    pub fn rbcr5(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, msdct::Rbcr5, Msdct_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,msdct::Rbcr5, Msdct_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Device Role"]
    #[inline(always)]
    pub fn rbcr76(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, msdct::Rbcr76, Msdct_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,msdct::Rbcr76, Msdct_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "No Limitation"]
        pub const _0: Self = Self::new(0);
        #[doc = "Limitation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rbcr1_SPEC;
    pub type Rbcr1 = crate::EnumBitfieldStruct<u8, Rbcr1_SPEC>;
    impl Rbcr1 {
        #[doc = "Not Capable"]
        pub const _0: Self = Self::new(0);
        #[doc = "Capable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rbcr2_SPEC;
    pub type Rbcr2 = crate::EnumBitfieldStruct<u8, Rbcr2_SPEC>;
    impl Rbcr2 {
        #[doc = "No data byte follows the accepted IBI."]
        pub const _0: Self = Self::new(0);
        #[doc = "Mandatory one or more data bytes follow the accepted IBI. Data byte continuation is indicated by T-Bit."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rbcr3_SPEC;
    pub type Rbcr3 = crate::EnumBitfieldStruct<u8, Rbcr3_SPEC>;
    impl Rbcr3 {
        #[doc = "Device will always respond to I3C bus commands."]
        pub const _0: Self = Self::new(0);
        #[doc = "Device will not always respond to I3C bus commands."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rbcr4_SPEC;
    pub type Rbcr4 = crate::EnumBitfieldStruct<u8, Rbcr4_SPEC>;
    impl Rbcr4 {
        #[doc = "Not a Bridge Device"]
        pub const _0: Self = Self::new(0);
        #[doc = "A Bridge Device"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rbcr5_SPEC;
    pub type Rbcr5 = crate::EnumBitfieldStruct<u8, Rbcr5_SPEC>;
    impl Rbcr5 {
        #[doc = "SDR only"]
        pub const _0: Self = Self::new(0);
        #[doc = "HDR Capable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rbcr76_SPEC;
    pub type Rbcr76 = crate::EnumBitfieldStruct<u8, Rbcr76_SPEC>;
    impl Rbcr76 {
        #[doc = "I3C Slave"]
        pub const _00: Self = Self::new(0);
        #[doc = "I3C Master"]
        pub const _01: Self = Self::new(1);
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Svdct_SPEC;
impl crate::sealed::RegSpec for Svdct_SPEC {
    type DataType = u32;
}
#[doc = "Slave Device Characteristic Table Register"]
pub type Svdct = crate::RegValueT<Svdct_SPEC>;

impl Svdct {
    #[doc = "Transfer Device Characteristic Register"]
    #[inline(always)]
    pub fn tdcr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Svdct_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Svdct_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Max Data Speed Limitation"]
    #[inline(always)]
    pub fn tbcr0(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, svdct::Tbcr0, Svdct_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,svdct::Tbcr0, Svdct_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IBI Request Capable"]
    #[inline(always)]
    pub fn tbcr1(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, svdct::Tbcr1, Svdct_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,svdct::Tbcr1, Svdct_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IBI Payload"]
    #[inline(always)]
    pub fn tbcr2(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, svdct::Tbcr2, Svdct_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,svdct::Tbcr2, Svdct_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Offline Capable"]
    #[inline(always)]
    pub fn tbcr3(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, svdct::Tbcr3, Svdct_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,svdct::Tbcr3, Svdct_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bridge Identifier"]
    #[inline(always)]
    pub fn tbcr4(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, svdct::Tbcr4, Svdct_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,svdct::Tbcr4, Svdct_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SDR Only / SDR and HDR Capable"]
    #[inline(always)]
    pub fn tbcr5(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, svdct::Tbcr5, Svdct_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,svdct::Tbcr5, Svdct_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Device Role"]
    #[inline(always)]
    pub fn tbcr76(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, svdct::Tbcr76, Svdct_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,svdct::Tbcr76, Svdct_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "No Limitation"]
        pub const _0: Self = Self::new(0);
        #[doc = "Limitation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tbcr1_SPEC;
    pub type Tbcr1 = crate::EnumBitfieldStruct<u8, Tbcr1_SPEC>;
    impl Tbcr1 {
        #[doc = "Not Capable"]
        pub const _0: Self = Self::new(0);
        #[doc = "Capable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tbcr2_SPEC;
    pub type Tbcr2 = crate::EnumBitfieldStruct<u8, Tbcr2_SPEC>;
    impl Tbcr2 {
        #[doc = "No data byte follows the accepted IBI."]
        pub const _0: Self = Self::new(0);
        #[doc = "Mandatory one or more data bytes follow the accepted IBI. Data byte continuation is indicated by T-Bit."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tbcr3_SPEC;
    pub type Tbcr3 = crate::EnumBitfieldStruct<u8, Tbcr3_SPEC>;
    impl Tbcr3 {
        #[doc = "Device will always respond to I3C bus commands."]
        pub const _0: Self = Self::new(0);
        #[doc = "Device will not always respond to I3C bus commands."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tbcr4_SPEC;
    pub type Tbcr4 = crate::EnumBitfieldStruct<u8, Tbcr4_SPEC>;
    impl Tbcr4 {
        #[doc = "Not a Bridge Device"]
        pub const _0: Self = Self::new(0);
        #[doc = "A Bridge Device"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tbcr5_SPEC;
    pub type Tbcr5 = crate::EnumBitfieldStruct<u8, Tbcr5_SPEC>;
    impl Tbcr5 {
        #[doc = "SDR only"]
        pub const _0: Self = Self::new(0);
        #[doc = "HDR Capable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tbcr76_SPEC;
    pub type Tbcr76 = crate::EnumBitfieldStruct<u8, Tbcr76_SPEC>;
    impl Tbcr76 {
        #[doc = "I3C Slave"]
        pub const _00: Self = Self::new(0);
        #[doc = "I3C Master"]
        pub const _01: Self = Self::new(1);
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdctpidl_SPEC;
impl crate::sealed::RegSpec for Sdctpidl_SPEC {
    type DataType = u32;
}
#[doc = "Slave Device Characteristic Table Provisional ID Low Register"]
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
#[doc = "Slave Device Characteristic Table Provisional ID High Register"]
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
pub struct Svdvad0_SPEC;
impl crate::sealed::RegSpec for Svdvad0_SPEC {
    type DataType = u32;
}
#[doc = "Slave Device Address Register 0"]
pub type Svdvad0 = crate::RegValueT<Svdvad0_SPEC>;

impl Svdvad0 {
    #[doc = "Slave Address"]
    #[inline(always)]
    pub fn svad(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, Svdvad0_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3ff,1,0,u16, Svdvad0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Slave Address Length"]
    #[inline(always)]
    pub fn sadlg(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, svdvad0::Sadlg, Svdvad0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<27,0x1,1,0,svdvad0::Sadlg, Svdvad0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Slave Static Address Valid"]
    #[inline(always)]
    pub fn sstadv(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, svdvad0::Sstadv, Svdvad0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<30,0x1,1,0,svdvad0::Sstadv, Svdvad0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Slave Dynamic Address Valid"]
    #[inline(always)]
    pub fn sdyadv(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, svdvad0::Sdyadv, Svdvad0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<31,0x1,1,0,svdvad0::Sdyadv, Svdvad0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Svdvad0 {
    #[inline(always)]
    fn default() -> Svdvad0 {
        <crate::RegValueT<Svdvad0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod svdvad0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sadlg_SPEC;
    pub type Sadlg = crate::EnumBitfieldStruct<u8, Sadlg_SPEC>;
    impl Sadlg {
        #[doc = "The 7-bit address format is selected."]
        pub const _0: Self = Self::new(0);
        #[doc = "The 10-bit address format is selected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sstadv_SPEC;
    pub type Sstadv = crate::EnumBitfieldStruct<u8, Sstadv_SPEC>;
    impl Sstadv {
        #[doc = "Slave address is disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "Slave address is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdyadv_SPEC;
    pub type Sdyadv = crate::EnumBitfieldStruct<u8, Sdyadv_SPEC>;
    impl Sdyadv {
        #[doc = "Dynamic Address is disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "Dynamic Address is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csecmd_SPEC;
impl crate::sealed::RegSpec for Csecmd_SPEC {
    type DataType = u32;
}
#[doc = "CCC Slave Events Command Register"]
pub type Csecmd = crate::RegValueT<Csecmd_SPEC>;

impl Csecmd {
    #[doc = "Slave Interrupt Requests Enable"]
    #[inline(always)]
    pub fn svirqe(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, csecmd::Svirqe, Csecmd_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,csecmd::Svirqe, Csecmd_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Mastership Requests Enable"]
    #[inline(always)]
    pub fn msrqe(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, csecmd::Msrqe, Csecmd_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,csecmd::Msrqe, Csecmd_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Hot-Join Event Enable"]
    #[inline(always)]
    pub fn hjeve(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, csecmd::Hjeve, Csecmd_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,csecmd::Hjeve, Csecmd_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "DISABLED: Slave-initiated Interrupts is Disabled by the Master to control."]
        pub const _0: Self = Self::new(0);
        #[doc = "ENABLED: Slave-initiated Interrupts is Enabled by the Master to control."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Msrqe_SPEC;
    pub type Msrqe = crate::EnumBitfieldStruct<u8, Msrqe_SPEC>;
    impl Msrqe {
        #[doc = "DISABLED: Mastership requests from Secondary Masters is Disabled by the Current Master to control."]
        pub const _0: Self = Self::new(0);
        #[doc = "ENABLED: Mastership requests from Secondary Masters is Enabled by the Current Master to control."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hjeve_SPEC;
    pub type Hjeve = crate::EnumBitfieldStruct<u8, Hjeve_SPEC>;
    impl Hjeve {
        #[doc = "DISABLED: Slave-initiated Hot-Join is Disabled by the Master to control."]
        pub const _0: Self = Self::new(0);
        #[doc = "ENABLED: Slave-initiated Hot-Join is Enabled by the Master to control."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ceactst_SPEC;
impl crate::sealed::RegSpec for Ceactst_SPEC {
    type DataType = u32;
}
#[doc = "CCC Enter Activity State Register"]
pub type Ceactst = crate::RegValueT<Ceactst_SPEC>;

impl Ceactst {
    #[doc = "Activity State"]
    #[inline(always)]
    pub fn actst(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, ceactst::Actst, Ceactst_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,ceactst::Actst, Ceactst_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "ENTAS0 (1µs: Latency-free operation)"]
        pub const _0_X_1: Self = Self::new(1);
        #[doc = "ENTAS1 (100 µs)"]
        pub const _0_X_2: Self = Self::new(2);
        #[doc = "ENTAS2 (2 ms)"]
        pub const _0_X_4: Self = Self::new(4);
        #[doc = "ENTAS3 (50 ms: Lowest-activity operation)"]
        pub const _0_X_8: Self = Self::new(8);
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmwlg_SPEC;
impl crate::sealed::RegSpec for Cmwlg_SPEC {
    type DataType = u32;
}
#[doc = "CCC Max Write Length Register"]
pub type Cmwlg = crate::RegValueT<Cmwlg_SPEC>;

impl Cmwlg {
    #[doc = "Max Write Length"]
    #[inline(always)]
    pub fn mwlg(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Cmwlg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Cmwlg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "CCC Max Read Length Register"]
pub type Cmrlg = crate::RegValueT<Cmrlg_SPEC>;

impl Cmrlg {
    #[doc = "Max Read Length"]
    #[inline(always)]
    pub fn mrlg(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Cmrlg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Cmrlg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IBI Payload Size"]
    #[inline(always)]
    pub fn ibipsz(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cmrlg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cmrlg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "CCC Enter Test Mode Register"]
pub type Cetstmd = crate::RegValueT<Cetstmd_SPEC>;

impl Cetstmd {
    #[doc = "Test Mode"]
    #[inline(always)]
    pub fn tstmd(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, cetstmd::Tstmd, Cetstmd_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,cetstmd::Tstmd, Cetstmd_SPEC,crate::common::R>::from_register(self,0)
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
        #[doc = "Exit Test Mode This value removes all I3C devices from Test Mode."]
        pub const _0_X_00: Self = Self::new(0);
        #[doc = "Vendor Test Mode This value indicates that I3C devices shall return a random 32bit value in the provisional ID during the Dynamic Address Assignment procedure."]
        pub const _0_X_01: Self = Self::new(1);
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cgdvst_SPEC;
impl crate::sealed::RegSpec for Cgdvst_SPEC {
    type DataType = u32;
}
#[doc = "CCC Get Device Status Register"]
pub type Cgdvst = crate::RegValueT<Cgdvst_SPEC>;

impl Cgdvst {
    #[doc = "Pending Interrupt"]
    #[inline(always)]
    pub fn pndint(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Cgdvst_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Cgdvst_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Protocol Error"]
    #[inline(always)]
    pub fn prte(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, cgdvst::Prte, Cgdvst_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,cgdvst::Prte, Cgdvst_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Slave Device’s current Activity Mode"]
    #[inline(always)]
    pub fn actmd(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, cgdvst::Actmd, Cgdvst_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,cgdvst::Actmd, Cgdvst_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Vendor Reserved"]
    #[inline(always)]
    pub fn vdrsv(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cgdvst_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cgdvst_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "The Slave has not detected a protocol error since the last Status read."]
        pub const _0: Self = Self::new(0);
        #[doc = "The Slave has detected a protocol error since the last Status read."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Actmd_SPEC;
    pub type Actmd = crate::EnumBitfieldStruct<u8, Actmd_SPEC>;
    impl Actmd {
        #[doc = "Activity Mode 0"]
        pub const _00: Self = Self::new(0);
        #[doc = "Activity Mode 1"]
        pub const _01: Self = Self::new(1);
        #[doc = "Activity Mode 2"]
        pub const _10: Self = Self::new(2);
        #[doc = "Activity Mode 3"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdspw_SPEC;
impl crate::sealed::RegSpec for Cmdspw_SPEC {
    type DataType = u32;
}
#[doc = "CCC Max Data Speed W (Write) Register"]
pub type Cmdspw = crate::RegValueT<Cmdspw_SPEC>;

impl Cmdspw {
    #[doc = "Maximum Sustained Write Data Rate"]
    #[inline(always)]
    pub fn mswdr(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, cmdspw::Mswdr, Cmdspw_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,cmdspw::Mswdr, Cmdspw_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "fscl Max (default value)"]
        pub const _000: Self = Self::new(0);
        #[doc = "8 MHz"]
        pub const _001: Self = Self::new(1);
        #[doc = "6 MHz"]
        pub const _010: Self = Self::new(2);
        #[doc = "4 MHz"]
        pub const _011: Self = Self::new(3);
        #[doc = "2 MHz"]
        pub const _100: Self = Self::new(4);
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdspr_SPEC;
impl crate::sealed::RegSpec for Cmdspr_SPEC {
    type DataType = u32;
}
#[doc = "CCC Max Data Speed R (Read) Register"]
pub type Cmdspr = crate::RegValueT<Cmdspr_SPEC>;

impl Cmdspr {
    #[doc = "Maximum Sustained Read Data Rate"]
    #[inline(always)]
    pub fn msrdr(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, cmdspr::Msrdr, Cmdspr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,cmdspr::Msrdr, Cmdspr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock to Data Turnaround Time (TSCO)"]
    #[inline(always)]
    pub fn cdttim(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, cmdspr::Cdttim, Cmdspr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x7,1,0,cmdspr::Cdttim, Cmdspr_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "fscl Max (default value)"]
        pub const _000: Self = Self::new(0);
        #[doc = "8 MHz"]
        pub const _001: Self = Self::new(1);
        #[doc = "6 MHz"]
        pub const _010: Self = Self::new(2);
        #[doc = "4 MHz"]
        pub const _011: Self = Self::new(3);
        #[doc = "2 MHz"]
        pub const _100: Self = Self::new(4);
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cdttim_SPEC;
    pub type Cdttim = crate::EnumBitfieldStruct<u8, Cdttim_SPEC>;
    impl Cdttim {
        #[doc = "8 ns or less (default value)"]
        pub const _000: Self = Self::new(0);
        #[doc = "9 ns or less"]
        pub const _001: Self = Self::new(1);
        #[doc = "10 ns or less"]
        pub const _010: Self = Self::new(2);
        #[doc = "11 ns or less"]
        pub const _011: Self = Self::new(3);
        #[doc = "12 ns or less"]
        pub const _100: Self = Self::new(4);
        #[doc = "TSCO is more than 12 ns, and is reported by private agreement."]
        pub const _111: Self = Self::new(7);
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdspt_SPEC;
impl crate::sealed::RegSpec for Cmdspt_SPEC {
    type DataType = u32;
}
#[doc = "CCC Max Data Speed T (Turnaround) Register"]
pub type Cmdspt = crate::RegValueT<Cmdspt_SPEC>;

impl Cmdspt {
    #[doc = "Maximum Read Turnaround Time"]
    #[inline(always)]
    pub fn mrttim(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, Cmdspt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffff,1,0,u32, Cmdspt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Maximum Read Turnaround Time Enable"]
    #[inline(always)]
    pub fn mrte(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, cmdspt::Mrte, Cmdspt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,cmdspt::Mrte, Cmdspt_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Disables transmission of the Maximum Read Turnaround Time. (GETMXDS Format 1: Without Turnaround)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables transmission of the Maximum Read Turnaround Time. (GETMXDS Format 2: With Turnaround)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cetsm_SPEC;
impl crate::sealed::RegSpec for Cetsm_SPEC {
    type DataType = u32;
}
#[doc = "CCC Exchange Timing Support Information M (Mode) Register"]
pub type Cetsm = crate::RegValueT<Cetsm_SPEC>;

impl Cetsm {
    #[doc = "Frequency Byte"]
    #[inline(always)]
    pub fn freq(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cetsm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cetsm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Inaccuracy Byte"]
    #[inline(always)]
    pub fn inac(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cetsm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cetsm_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cetsm {
    #[inline(always)]
    fn default() -> Cetsm {
        <crate::RegValueT<Cetsm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cghdrcap_SPEC;
impl crate::sealed::RegSpec for Cghdrcap_SPEC {
    type DataType = u32;
}
#[doc = "CCC Get HDR Capability Register"]
pub type Cghdrcap = crate::RegValueT<Cghdrcap_SPEC>;

impl Cghdrcap {
    #[doc = "HDR-DDR Operation Enable"]
    #[inline(always)]
    pub fn ddren(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, cghdrcap::Ddren, Cghdrcap_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cghdrcap::Ddren,
            Cghdrcap_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "HDR-TSP Operation Enable"]
    #[inline(always)]
    pub fn tspen(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, cghdrcap::Tspen, Cghdrcap_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cghdrcap::Tspen,
            Cghdrcap_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "HDR-TSL Operation Enable"]
    #[inline(always)]
    pub fn tslen(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, cghdrcap::Tslen, Cghdrcap_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cghdrcap::Tslen,
            Cghdrcap_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cghdrcap {
    #[inline(always)]
    fn default() -> Cghdrcap {
        <crate::RegValueT<Cghdrcap_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cghdrcap {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ddren_SPEC;
    pub type Ddren = crate::EnumBitfieldStruct<u8, Ddren_SPEC>;
    impl Ddren {
        #[doc = "HDR-DDR Operation is Disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "HDR-DDR Operation is Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tspen_SPEC;
    pub type Tspen = crate::EnumBitfieldStruct<u8, Tspen_SPEC>;
    impl Tspen {
        #[doc = "HDR-TSP Operation is Disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "HDR-TSP Operation is Enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tslen_SPEC;
    pub type Tslen = crate::EnumBitfieldStruct<u8, Tslen_SPEC>;
    impl Tslen {
        #[doc = "HDR-TSL Operation is Disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "HDR-TSL Operation is Enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bitcnt_SPEC;
impl crate::sealed::RegSpec for Bitcnt_SPEC {
    type DataType = u32;
}
#[doc = "Bit Count Register"]
pub type Bitcnt = crate::RegValueT<Bitcnt_SPEC>;

impl Bitcnt {
    #[doc = "Bit Counter"]
    #[inline(always)]
    pub fn bcnt(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, Bitcnt_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, Bitcnt_SPEC,crate::common::R>::from_register(self,0)
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
#[doc = "Normal Queue Status Level Register"]
pub type Nqstlv = crate::RegValueT<Nqstlv_SPEC>;

impl Nqstlv {
    #[doc = "Normal Command Queue Free Level"]
    #[inline(always)]
    pub fn cmdqflv(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Nqstlv_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Nqstlv_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Normal Response Queue Level"]
    #[inline(always)]
    pub fn rspqlv(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Nqstlv_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Nqstlv_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Normal IBI Queue Level"]
    #[inline(always)]
    pub fn ibiqlv(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Nqstlv_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Nqstlv_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Normal IBI Status Count"]
    #[inline(always)]
    pub fn ibiscnt(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, Nqstlv_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, Nqstlv_SPEC,crate::common::R>::from_register(self,0)
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
#[doc = "Normal Data Buffer Status Level Register 0"]
pub type Ndbstlv0 = crate::RegValueT<Ndbstlv0_SPEC>;

impl Ndbstlv0 {
    #[doc = "Normal Tx Data Buffer Free Level"]
    #[inline(always)]
    pub fn tdbflv(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Ndbstlv0_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Ndbstlv0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Normal Rx Data Buffer Level"]
    #[inline(always)]
    pub fn rdblv(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Ndbstlv0_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Ndbstlv0_SPEC,crate::common::R>::from_register(self,0)
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
#[doc = "Normal Receive Status Queue Status Level Register"]
pub type Nrsqstlv = crate::RegValueT<Nrsqstlv_SPEC>;

impl Nrsqstlv {
    #[doc = "Normal Receive Status Queue Level"]
    #[inline(always)]
    pub fn rsqlv(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Nrsqstlv_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Nrsqstlv_SPEC,crate::common::R>::from_register(self,0)
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
pub struct Prstdbg_SPEC;
impl crate::sealed::RegSpec for Prstdbg_SPEC {
    type DataType = u32;
}
#[doc = "Present State Debug Register"]
pub type Prstdbg = crate::RegValueT<Prstdbg_SPEC>;

impl Prstdbg {
    #[doc = "I3C_SCL Line Signal Level"]
    #[inline(always)]
    pub fn scilv(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Prstdbg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Prstdbg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "I3C_SDA Line Signal Level"]
    #[inline(always)]
    pub fn sdilv(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Prstdbg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Prstdbg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "SCL Output Level"]
    #[inline(always)]
    pub fn scolv(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, prstdbg::Scolv, Prstdbg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x1,1,0,prstdbg::Scolv, Prstdbg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "SDA Output Level"]
    #[inline(always)]
    pub fn sdolv(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, prstdbg::Sdolv, Prstdbg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<3,0x1,1,0,prstdbg::Sdolv, Prstdbg_SPEC,crate::common::R>::from_register(self,0)
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
        #[doc = "I3C has driven the I3C_SCL pin low."]
        pub const _0: Self = Self::new(0);
        #[doc = "I3C has released the I3C_SCL pin."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdolv_SPEC;
    pub type Sdolv = crate::EnumBitfieldStruct<u8, Sdolv_SPEC>;
    impl Sdolv {
        #[doc = "I3C has driven the I3C_SDA pin low."]
        pub const _0: Self = Self::new(0);
        #[doc = "I3C has released the I3C_SDA pin."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mserrcnt_SPEC;
impl crate::sealed::RegSpec for Mserrcnt_SPEC {
    type DataType = u32;
}
#[doc = "Master Error Counters Register"]
pub type Mserrcnt = crate::RegValueT<Mserrcnt_SPEC>;

impl Mserrcnt {
    #[doc = "M2 Error Counter"]
    #[inline(always)]
    pub fn m2ecnt(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Mserrcnt_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Mserrcnt_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mserrcnt {
    #[inline(always)]
    fn default() -> Mserrcnt {
        <crate::RegValueT<Mserrcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}
