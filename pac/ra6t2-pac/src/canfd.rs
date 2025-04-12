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
// Generated from SVD 1.40.00, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:18:35 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"CANFD Module Control"]
unsafe impl ::core::marker::Send for super::Canfd {}
unsafe impl ::core::marker::Sync for super::Canfd {}
impl super::Canfd {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "Channel 0 Nominal Bitrate Configuration Register"]
    #[inline(always)]
    pub const fn cfdc0ncfg(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdc0Ncfg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdc0Ncfg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Channel 0 Control Register"]
    #[inline(always)]
    pub const fn cfdc0ctr(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdc0Ctr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdc0Ctr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Channel 0 Status Register"]
    #[inline(always)]
    pub const fn cfdc0sts(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdc0Sts_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdc0Sts_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Channel 0 Error Flag Register"]
    #[inline(always)]
    pub const fn cfdc0erfl(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdc0Erfl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdc0Erfl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Global Configuration Register"]
    #[inline(always)]
    pub const fn cfdgcfg(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdgcfg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdgcfg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Global Control Register"]
    #[inline(always)]
    pub const fn cfdgctr(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdgctr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdgctr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Global Status Register"]
    #[inline(always)]
    pub const fn cfdgsts(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdgsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cfdgsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Global Error Flag Register"]
    #[inline(always)]
    pub const fn cfdgerfl(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdgerfl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdgerfl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Global Timestamp Counter Register"]
    #[inline(always)]
    pub const fn cfdgtsc(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdgtsc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cfdgtsc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "Global Acceptance Filter List Entry Control Register"]
    #[inline(always)]
    pub const fn cfdgaflectr(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdgaflectr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdgaflectr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "Global Acceptance Filter List Configuration Register"]
    #[inline(always)]
    pub const fn cfdgaflcfg(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdgaflcfg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdgaflcfg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "RX Message Buffer Number Register"]
    #[inline(always)]
    pub const fn cfdrmnb(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdrmnb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdrmnb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "RX Message Buffer New Data Register"]
    #[inline(always)]
    pub const fn cfdrmnd(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdrmnd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdrmnd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "RX Message Buffer Interrupt Enable Configuration Register"]
    #[inline(always)]
    pub const fn cfdrmiec(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdrmiec_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdrmiec_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "RX FIFO Configuration/Control Registers %s"]
    #[inline(always)]
    pub const fn cfdrfcc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfcc_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x3cusize))
        }
    }

    #[doc = "RX FIFO Status Registers %s"]
    #[inline(always)]
    pub const fn cfdrfsts(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfsts_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x44usize))
        }
    }

    #[doc = "RX FIFO Pointer Control Registers %s"]
    #[inline(always)]
    pub const fn cfdrfpctr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfpctr_SPEC, crate::common::W>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x4cusize))
        }
    }

    #[doc = "Common FIFO Configuration/Control Register"]
    #[inline(always)]
    pub const fn cfdcfcc(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcfcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdcfcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[doc = "Common FIFO Status Register"]
    #[inline(always)]
    pub const fn cfdcfsts(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcfsts_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdcfsts_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[doc = "Common FIFO Pointer Control Register"]
    #[inline(always)]
    pub const fn cfdcfpctr(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcfpctr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Cfdcfpctr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[doc = "FIFO Empty Status Register"]
    #[inline(always)]
    pub const fn cfdfests(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdfests_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cfdfests_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[doc = "FIFO Full Status Register"]
    #[inline(always)]
    pub const fn cfdffsts(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdffsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cfdffsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[doc = "FIFO Message Lost Status Register"]
    #[inline(always)]
    pub const fn cfdfmsts(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdfmsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cfdfmsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[doc = "RX FIFO Interrupt Flag Status Register"]
    #[inline(always)]
    pub const fn cfdrfists(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdrfists_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cfdrfists_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[doc = "TX Message Buffer Control Registers %s"]
    #[inline(always)]
    pub const fn cfdtmc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmc_SPEC, crate::common::RW>,
        4,
        0x1,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x70usize))
        }
    }

    #[doc = "TX Message Buffer Status Registers %s"]
    #[inline(always)]
    pub const fn cfdtmsts(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmsts_SPEC, crate::common::RW>,
        4,
        0x1,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x74usize))
        }
    }

    #[doc = "TX Message Buffer Transmission Request Status Register"]
    #[inline(always)]
    pub const fn cfdtmtrsts(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdtmtrsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cfdtmtrsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[doc = "TX Message Buffer Transmission Abort Request Status Register"]
    #[inline(always)]
    pub const fn cfdtmtarsts(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdtmtarsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cfdtmtarsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }

    #[doc = "TX Message Buffer Transmission Completion Status Register"]
    #[inline(always)]
    pub const fn cfdtmtcsts(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdtmtcsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cfdtmtcsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[doc = "TX Message Buffer Transmission Abort Status Register"]
    #[inline(always)]
    pub const fn cfdtmtasts(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdtmtasts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cfdtmtasts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[doc = "TX Message Buffer Interrupt Enable Configuration Register"]
    #[inline(always)]
    pub const fn cfdtmiec(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdtmiec_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdtmiec_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(136usize),
            )
        }
    }

    #[doc = "TX Queue Configuration/Control Register"]
    #[inline(always)]
    pub const fn cfdtxqcc(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdtxqcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdtxqcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
            )
        }
    }

    #[doc = "TX Queue Status Register"]
    #[inline(always)]
    pub const fn cfdtxqsts(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdtxqsts_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdtxqsts_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[doc = "TX Queue Pointer Control Register"]
    #[inline(always)]
    pub const fn cfdtxqpctr(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdtxqpctr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Cfdtxqpctr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[doc = "TX History List Configuration/Control Register"]
    #[inline(always)]
    pub const fn cfdthlcc(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdthlcc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdthlcc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
            )
        }
    }

    #[doc = "TX History List Status Register"]
    #[inline(always)]
    pub const fn cfdthlsts(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdthlsts_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdthlsts_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(156usize),
            )
        }
    }

    #[doc = "TX History List Pointer Control Register"]
    #[inline(always)]
    pub const fn cfdthlpctr(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdthlpctr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Cfdthlpctr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[doc = "Global TX Interrupt Status Register"]
    #[inline(always)]
    pub const fn cfdgtintsts(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdgtintsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cfdgtintsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(164usize),
            )
        }
    }

    #[doc = "Global Test Configuration Register"]
    #[inline(always)]
    pub const fn cfdgtstcfg(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdgtstcfg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdgtstcfg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(168usize),
            )
        }
    }

    #[doc = "Global Test Control Register"]
    #[inline(always)]
    pub const fn cfdgtstctr(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdgtstctr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdgtstctr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(172usize),
            )
        }
    }

    #[doc = "Global FD Configuration Register"]
    #[inline(always)]
    pub const fn cfdgfdcfg(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdgfdcfg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdgfdcfg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(176usize),
            )
        }
    }

    #[doc = "Global Lock Key Register"]
    #[inline(always)]
    pub const fn cfdglockk(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdglockk_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Cfdglockk_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(184usize),
            )
        }
    }

    #[doc = "Global AFL Ignore Entry Register"]
    #[inline(always)]
    pub const fn cfdgaflignent(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdgaflignent_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdgaflignent_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }

    #[doc = "Global AFL Ignore Control Register"]
    #[inline(always)]
    pub const fn cfdgaflignctr(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdgaflignctr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdgaflignctr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(196usize),
            )
        }
    }

    #[doc = "DMA Transfer Control Register"]
    #[inline(always)]
    pub const fn cfdcdtct(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcdtct_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdcdtct_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(200usize),
            )
        }
    }

    #[doc = "DMA Transfer Status Register"]
    #[inline(always)]
    pub const fn cfdcdtsts(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcdtsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cfdcdtsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(204usize),
            )
        }
    }

    #[doc = "Global SW reset Register"]
    #[inline(always)]
    pub const fn cfdgrstc(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdgrstc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdgrstc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(216usize),
            )
        }
    }

    #[doc = "Channel 0 Data Bitrate Configuration Register"]
    #[inline(always)]
    pub const fn cfdc0dcfg(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdc0Dcfg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdc0Dcfg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[doc = "Channel 0 CANFD Configuration Register"]
    #[inline(always)]
    pub const fn cfdc0fdcfg(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdc0Fdcfg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdc0Fdcfg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(260usize),
            )
        }
    }

    #[doc = "Channel 0 CANFD Control Register"]
    #[inline(always)]
    pub const fn cfdc0fdctr(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdc0Fdctr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdc0Fdctr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(264usize),
            )
        }
    }

    #[doc = "Channel 0 CANFD Status Register"]
    #[inline(always)]
    pub const fn cfdc0fdsts(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdc0Fdsts_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdc0Fdsts_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(268usize),
            )
        }
    }

    #[doc = "Channel 0 CANFD CRC Register"]
    #[inline(always)]
    pub const fn cfdc0fdcrc(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdc0Fdcrc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdc0Fdcrc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(272usize),
            )
        }
    }

    #[doc = "Global Acceptance Filter List ID Registers"]
    #[inline(always)]
    pub const fn cfdgaflid(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdgaflid_SPEC, crate::common::RW>,
        16,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x120usize))
        }
    }

    #[doc = "Global Acceptance Filter List Mask Registers"]
    #[inline(always)]
    pub const fn cfdgaflm(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdgaflm_SPEC, crate::common::RW>,
        16,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x124usize))
        }
    }

    #[doc = "Global Acceptance Filter List Pointer 0 Registers"]
    #[inline(always)]
    pub const fn cfdgaflp0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdgaflp0_SPEC, crate::common::RW>,
        16,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x128usize))
        }
    }

    #[doc = "Global Acceptance Filter List Pointer 1 Registers"]
    #[inline(always)]
    pub const fn cfdgaflp1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdgaflp1_SPEC, crate::common::RW>,
        16,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x12cusize))
        }
    }

    #[doc = "RAM Test Page Access Registers %s"]
    #[inline(always)]
    pub const fn cfdrpgacc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrpgacc_SPEC, crate::common::RW>,
        64,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x280usize))
        }
    }

    #[doc = "RX FIFO Access ID Register %s"]
    #[inline(always)]
    pub const fn cfdrfid(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfid_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x520usize))
        }
    }

    #[doc = "RX FIFO Access Pointer Register %s"]
    #[inline(always)]
    pub const fn cfdrfptr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfptr_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x524usize))
        }
    }

    #[doc = "RX FIFO Access CANFD Status Register %s"]
    #[inline(always)]
    pub const fn cfdrffdsts(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrffdsts_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x528usize))
        }
    }

    #[doc = "RX FIFO Access Data Field 0 Register %s"]
    #[inline(always)]
    pub const fn cfdrfdf_0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf0_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x52cusize))
        }
    }

    #[doc = "RX FIFO Access Data Field 1 Register %s"]
    #[inline(always)]
    pub const fn cfdrfdf_1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf1_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x530usize))
        }
    }

    #[doc = "RX FIFO Access Data Field 2 Register %s"]
    #[inline(always)]
    pub const fn cfdrfdf_2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf2_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x534usize))
        }
    }

    #[doc = "RX FIFO Access Data Field 3 Register %s"]
    #[inline(always)]
    pub const fn cfdrfdf_3(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf3_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x538usize))
        }
    }

    #[doc = "RX FIFO Access Data Field 4 Register %s"]
    #[inline(always)]
    pub const fn cfdrfdf_4(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf4_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x53cusize))
        }
    }

    #[doc = "RX FIFO Access Data Field 5 Register %s"]
    #[inline(always)]
    pub const fn cfdrfdf_5(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf5_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x540usize))
        }
    }

    #[doc = "RX FIFO Access Data Field 6 Register %s"]
    #[inline(always)]
    pub const fn cfdrfdf_6(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf6_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x544usize))
        }
    }

    #[doc = "RX FIFO Access Data Field 7 Register %s"]
    #[inline(always)]
    pub const fn cfdrfdf_7(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf7_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x548usize))
        }
    }

    #[doc = "RX FIFO Access Data Field 8 Register %s"]
    #[inline(always)]
    pub const fn cfdrfdf_8(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf8_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x54cusize))
        }
    }

    #[doc = "RX FIFO Access Data Field 9 Register %s"]
    #[inline(always)]
    pub const fn cfdrfdf_9(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf9_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x550usize))
        }
    }

    #[doc = "RX FIFO Access Data Field 10 Register %s"]
    #[inline(always)]
    pub const fn cfdrfdf_10(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf10_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x554usize))
        }
    }

    #[doc = "RX FIFO Access Data Field 11 Register %s"]
    #[inline(always)]
    pub const fn cfdrfdf_11(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf11_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x558usize))
        }
    }

    #[doc = "RX FIFO Access Data Field 12 Register %s"]
    #[inline(always)]
    pub const fn cfdrfdf_12(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf12_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x55cusize))
        }
    }

    #[doc = "RX FIFO Access Data Field 13 Register %s"]
    #[inline(always)]
    pub const fn cfdrfdf_13(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf13_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x560usize))
        }
    }

    #[doc = "RX FIFO Access Data Field 14 Register %s"]
    #[inline(always)]
    pub const fn cfdrfdf_14(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf14_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x564usize))
        }
    }

    #[doc = "RX FIFO Access Data Field 15 Register %s"]
    #[inline(always)]
    pub const fn cfdrfdf_15(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf15_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x568usize))
        }
    }

    #[doc = "Common FIFO Access ID Register"]
    #[inline(always)]
    pub const fn cfdcfid(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcfid_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdcfid_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1464usize),
            )
        }
    }

    #[doc = "Common FIFO Access Pointer Register"]
    #[inline(always)]
    pub const fn cfdcfptr(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcfptr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdcfptr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1468usize),
            )
        }
    }

    #[doc = "Common FIFO Access CANFD Control/Status Register"]
    #[inline(always)]
    pub const fn cfdcffdcsts(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcffdcsts_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdcffdcsts_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1472usize),
            )
        }
    }

    #[doc = "Common FIFO Access Data Field %s Registers"]
    #[inline(always)]
    pub const fn cfdcfdf(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdf_SPEC, crate::common::RW>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x5c4usize))
        }
    }

    #[doc = "TX Message Buffer ID Registers"]
    #[inline(always)]
    pub const fn cfdtmid(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmid_SPEC, crate::common::RW>,
        4,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x604usize))
        }
    }

    #[doc = "TX Message Buffer Pointer Register"]
    #[inline(always)]
    pub const fn cfdtmptr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmptr_SPEC, crate::common::RW>,
        4,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x608usize))
        }
    }

    #[doc = "TX Message Buffer CANFD Control Register"]
    #[inline(always)]
    pub const fn cfdtmfdctr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmfdctr_SPEC, crate::common::RW>,
        4,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x60cusize))
        }
    }

    #[doc = "TX Message Buffer Data Field Register"]
    #[inline(always)]
    pub const fn cfdtmdf_0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf0_SPEC, crate::common::RW>,
        4,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x610usize))
        }
    }

    #[doc = "TX Message Buffer Data Field Register"]
    #[inline(always)]
    pub const fn cfdtmdf_1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf1_SPEC, crate::common::RW>,
        4,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x614usize))
        }
    }

    #[doc = "TX Message Buffer Data Field Register"]
    #[inline(always)]
    pub const fn cfdtmdf_2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf2_SPEC, crate::common::RW>,
        4,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x618usize))
        }
    }

    #[doc = "TX Message Buffer Data Field Register"]
    #[inline(always)]
    pub const fn cfdtmdf_3(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf3_SPEC, crate::common::RW>,
        4,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x61cusize))
        }
    }

    #[doc = "TX Message Buffer Data Field Register"]
    #[inline(always)]
    pub const fn cfdtmdf_4(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf4_SPEC, crate::common::RW>,
        4,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x620usize))
        }
    }

    #[doc = "TX Message Buffer Data Field Register"]
    #[inline(always)]
    pub const fn cfdtmdf_5(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf5_SPEC, crate::common::RW>,
        4,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x624usize))
        }
    }

    #[doc = "TX Message Buffer Data Field Register"]
    #[inline(always)]
    pub const fn cfdtmdf_6(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf6_SPEC, crate::common::RW>,
        4,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x628usize))
        }
    }

    #[doc = "TX Message Buffer Data Field Register"]
    #[inline(always)]
    pub const fn cfdtmdf_7(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf7_SPEC, crate::common::RW>,
        4,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x62cusize))
        }
    }

    #[doc = "TX Message Buffer Data Field Register"]
    #[inline(always)]
    pub const fn cfdtmdf_8(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf8_SPEC, crate::common::RW>,
        4,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x630usize))
        }
    }

    #[doc = "TX Message Buffer Data Field Register"]
    #[inline(always)]
    pub const fn cfdtmdf_9(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf9_SPEC, crate::common::RW>,
        4,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x634usize))
        }
    }

    #[doc = "TX Message Buffer Data Field Register"]
    #[inline(always)]
    pub const fn cfdtmdf_10(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf10_SPEC, crate::common::RW>,
        4,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x638usize))
        }
    }

    #[doc = "TX Message Buffer Data Field Register"]
    #[inline(always)]
    pub const fn cfdtmdf_11(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf11_SPEC, crate::common::RW>,
        4,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x63cusize))
        }
    }

    #[doc = "TX Message Buffer Data Field Register"]
    #[inline(always)]
    pub const fn cfdtmdf_12(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf12_SPEC, crate::common::RW>,
        4,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x640usize))
        }
    }

    #[doc = "TX Message Buffer Data Field Register"]
    #[inline(always)]
    pub const fn cfdtmdf_13(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf13_SPEC, crate::common::RW>,
        4,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x644usize))
        }
    }

    #[doc = "TX Message Buffer Data Field Register"]
    #[inline(always)]
    pub const fn cfdtmdf_14(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf14_SPEC, crate::common::RW>,
        4,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x648usize))
        }
    }

    #[doc = "TX Message Buffer Data Field Register"]
    #[inline(always)]
    pub const fn cfdtmdf_15(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf15_SPEC, crate::common::RW>,
        4,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x64cusize))
        }
    }

    #[doc = "TX History List Access Register 0"]
    #[inline(always)]
    pub const fn cfdthlacc0(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdthlacc0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cfdthlacc0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1856usize),
            )
        }
    }

    #[doc = "TX History List Access Register 1"]
    #[inline(always)]
    pub const fn cfdthlacc1(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdthlacc1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cfdthlacc1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(1860usize),
            )
        }
    }

    #[doc = "RX Message Buffer ID Registers"]
    #[inline(always)]
    pub const fn cfdrmid(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmid_SPEC, crate::common::R>,
        8,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1520usize))
        }
    }

    #[doc = "RX Message Buffer Pointer Registers"]
    #[inline(always)]
    pub const fn cfdrmptr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmptr_SPEC, crate::common::R>,
        8,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1524usize))
        }
    }

    #[doc = "RX Message Buffer CANFD Status Registers"]
    #[inline(always)]
    pub const fn cfdrmfdsts(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmfdsts_SPEC, crate::common::R>,
        8,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1528usize))
        }
    }

    #[doc = "RX Message Buffer Data Field 0 Registers"]
    #[inline(always)]
    pub const fn cfdrmdf_0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf0_SPEC, crate::common::R>,
        8,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x152cusize))
        }
    }

    #[doc = "RX Message Buffer Data Field 1 Registers"]
    #[inline(always)]
    pub const fn cfdrmdf_1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf1_SPEC, crate::common::R>,
        8,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1530usize))
        }
    }

    #[doc = "RX Message Buffer Data Field 2 Registers"]
    #[inline(always)]
    pub const fn cfdrmdf_2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf2_SPEC, crate::common::R>,
        8,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1534usize))
        }
    }

    #[doc = "RX Message Buffer Data Field 3 Registers"]
    #[inline(always)]
    pub const fn cfdrmdf_3(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf3_SPEC, crate::common::R>,
        8,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1538usize))
        }
    }

    #[doc = "RX Message Buffer Data Field 4 Registers"]
    #[inline(always)]
    pub const fn cfdrmdf_4(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf4_SPEC, crate::common::R>,
        8,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x153cusize))
        }
    }

    #[doc = "RX Message Buffer Data Field 5 Registers"]
    #[inline(always)]
    pub const fn cfdrmdf_5(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf5_SPEC, crate::common::R>,
        8,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1540usize))
        }
    }

    #[doc = "RX Message Buffer Data Field 6 Registers"]
    #[inline(always)]
    pub const fn cfdrmdf_6(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf6_SPEC, crate::common::R>,
        8,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1544usize))
        }
    }

    #[doc = "RX Message Buffer Data Field 7 Registers"]
    #[inline(always)]
    pub const fn cfdrmdf_7(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf7_SPEC, crate::common::R>,
        8,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1548usize))
        }
    }

    #[doc = "RX Message Buffer Data Field 8 Registers"]
    #[inline(always)]
    pub const fn cfdrmdf_8(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf8_SPEC, crate::common::R>,
        8,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x154cusize))
        }
    }

    #[doc = "RX Message Buffer Data Field 9 Registers"]
    #[inline(always)]
    pub const fn cfdrmdf_9(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf9_SPEC, crate::common::R>,
        8,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1550usize))
        }
    }

    #[doc = "RX Message Buffer Data Field 10 Registers"]
    #[inline(always)]
    pub const fn cfdrmdf_10(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf10_SPEC, crate::common::R>,
        8,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1554usize))
        }
    }

    #[doc = "RX Message Buffer Data Field 11 Registers"]
    #[inline(always)]
    pub const fn cfdrmdf_11(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf11_SPEC, crate::common::R>,
        8,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1558usize))
        }
    }

    #[doc = "RX Message Buffer Data Field 12 Registers"]
    #[inline(always)]
    pub const fn cfdrmdf_12(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf12_SPEC, crate::common::R>,
        8,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x155cusize))
        }
    }

    #[doc = "RX Message Buffer Data Field 13 Registers"]
    #[inline(always)]
    pub const fn cfdrmdf_13(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf13_SPEC, crate::common::R>,
        8,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1560usize))
        }
    }

    #[doc = "RX Message Buffer Data Field 14 Registers"]
    #[inline(always)]
    pub const fn cfdrmdf_14(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf14_SPEC, crate::common::R>,
        8,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1564usize))
        }
    }

    #[doc = "RX Message Buffer Data Field 15 Registers"]
    #[inline(always)]
    pub const fn cfdrmdf_15(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf15_SPEC, crate::common::R>,
        8,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1568usize))
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdc0Ncfg_SPEC;
impl crate::sealed::RegSpec for Cfdc0Ncfg_SPEC {
    type DataType = u32;
}
#[doc = "Channel 0 Nominal Bitrate Configuration Register"]
pub type Cfdc0Ncfg = crate::RegValueT<Cfdc0Ncfg_SPEC>;

impl Cfdc0Ncfg {
    #[doc = "Channel Nominal Baud Rate Prescaler"]
    #[inline(always)]
    pub fn nbrp(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Cfdc0Ncfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, Cfdc0Ncfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Resynchronization Jump Width"]
    #[inline(always)]
    pub fn nsjw(
        self,
    ) -> crate::common::RegisterField<10, 0x7f, 1, 0, u8, Cfdc0Ncfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x7f,1,0,u8, Cfdc0Ncfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timing Segment 1"]
    #[inline(always)]
    pub fn ntseg1(
        self,
    ) -> crate::common::RegisterField<17, 0xff, 1, 0, u8, Cfdc0Ncfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<17,0xff,1,0,u8, Cfdc0Ncfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timing Segment 2"]
    #[inline(always)]
    pub fn ntseg2(
        self,
    ) -> crate::common::RegisterField<25, 0x7f, 1, 0, u8, Cfdc0Ncfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<25,0x7f,1,0,u8, Cfdc0Ncfg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdc0Ncfg {
    #[inline(always)]
    fn default() -> Cfdc0Ncfg {
        <crate::RegValueT<Cfdc0Ncfg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdc0Ctr_SPEC;
impl crate::sealed::RegSpec for Cfdc0Ctr_SPEC {
    type DataType = u32;
}
#[doc = "Channel 0 Control Register"]
pub type Cfdc0Ctr = crate::RegValueT<Cfdc0Ctr_SPEC>;

impl Cfdc0Ctr {
    #[doc = "Channel Mode Control"]
    #[inline(always)]
    pub fn chmdc(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, cfdc0ctr::Chmdc, Cfdc0Ctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            cfdc0ctr::Chmdc,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Channel Sleep Request"]
    #[inline(always)]
    pub fn cslpr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, cfdc0ctr::Cslpr, Cfdc0Ctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdc0ctr::Cslpr,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Return from Bus-Off"]
    #[inline(always)]
    pub fn rtbo(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, cfdc0ctr::Rtbo, Cfdc0Ctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,cfdc0ctr::Rtbo, Cfdc0Ctr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bus Error Interrupt Enable"]
    #[inline(always)]
    pub fn beie(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, cfdc0ctr::Beie, Cfdc0Ctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,cfdc0ctr::Beie, Cfdc0Ctr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error Warning Interrupt Enable"]
    #[inline(always)]
    pub fn ewie(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, cfdc0ctr::Ewie, Cfdc0Ctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,cfdc0ctr::Ewie, Cfdc0Ctr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error Passive Interrupt Enable"]
    #[inline(always)]
    pub fn epie(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, cfdc0ctr::Epie, Cfdc0Ctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            cfdc0ctr::Epie,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Bus-Off Entry Interrupt Enable"]
    #[inline(always)]
    pub fn boeie(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        cfdc0ctr::Boeie,
        Cfdc0Ctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            cfdc0ctr::Boeie,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Bus-Off Recovery Interrupt Enable"]
    #[inline(always)]
    pub fn borie(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        cfdc0ctr::Borie,
        Cfdc0Ctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            cfdc0ctr::Borie,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Overload Interrupt Enable"]
    #[inline(always)]
    pub fn olie(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, cfdc0ctr::Olie, Cfdc0Ctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            cfdc0ctr::Olie,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Bus Lock Interrupt Enable"]
    #[inline(always)]
    pub fn blie(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, cfdc0ctr::Blie, Cfdc0Ctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            cfdc0ctr::Blie,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    pub fn alie(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, cfdc0ctr::Alie, Cfdc0Ctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            cfdc0ctr::Alie,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Transmission Abort Interrupt Enable"]
    #[inline(always)]
    pub fn taie(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, cfdc0ctr::Taie, Cfdc0Ctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            cfdc0ctr::Taie,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Error Occurrence Counter Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn eocoie(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        cfdc0ctr::Eocoie,
        Cfdc0Ctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            cfdc0ctr::Eocoie,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Successful Occurrence Counter Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn socoie(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        cfdc0ctr::Socoie,
        Cfdc0Ctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            cfdc0ctr::Socoie,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Transceiver Delay Compensation Violation Interrupt Enable"]
    #[inline(always)]
    pub fn tdcvfie(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        cfdc0ctr::Tdcvfie,
        Cfdc0Ctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            cfdc0ctr::Tdcvfie,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Channel Bus-Off Mode"]
    #[inline(always)]
    pub fn bom(
        self,
    ) -> crate::common::RegisterField<21, 0x3, 1, 0, cfdc0ctr::Bom, Cfdc0Ctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x3,1,0,cfdc0ctr::Bom, Cfdc0Ctr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel Error Display"]
    #[inline(always)]
    pub fn errd(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, cfdc0ctr::Errd, Cfdc0Ctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            cfdc0ctr::Errd,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Channel Test Mode Enable"]
    #[inline(always)]
    pub fn ctme(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, cfdc0ctr::Ctme, Cfdc0Ctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            cfdc0ctr::Ctme,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Channel Test Mode Select"]
    #[inline(always)]
    pub fn ctms(
        self,
    ) -> crate::common::RegisterField<25, 0x3, 1, 0, cfdc0ctr::Ctms, Cfdc0Ctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            25,
            0x3,
            1,
            0,
            cfdc0ctr::Ctms,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Bit Flip Test"]
    #[inline(always)]
    pub fn bft(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, cfdc0ctr::Bft, Cfdc0Ctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,cfdc0ctr::Bft, Cfdc0Ctr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Restricted Operation Mode"]
    #[inline(always)]
    pub fn rom(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, cfdc0ctr::Rom, Cfdc0Ctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,cfdc0ctr::Rom, Cfdc0Ctr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdc0Ctr {
    #[inline(always)]
    fn default() -> Cfdc0Ctr {
        <crate::RegValueT<Cfdc0Ctr_SPEC> as RegisterValue<_>>::new(5)
    }
}
pub mod cfdc0ctr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chmdc_SPEC;
    pub type Chmdc = crate::EnumBitfieldStruct<u8, Chmdc_SPEC>;
    impl Chmdc {
        #[doc = "Channel operation mode request"]
        pub const _00: Self = Self::new(0);
        #[doc = "Channel reset request"]
        pub const _01: Self = Self::new(1);
        #[doc = "Channel halt request"]
        pub const _10: Self = Self::new(2);
        #[doc = "Keep current value"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cslpr_SPEC;
    pub type Cslpr = crate::EnumBitfieldStruct<u8, Cslpr_SPEC>;
    impl Cslpr {
        #[doc = "Channel sleep request disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Channel sleep request enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtbo_SPEC;
    pub type Rtbo = crate::EnumBitfieldStruct<u8, Rtbo_SPEC>;
    impl Rtbo {
        #[doc = "Channel is not forced to return from bus-off"]
        pub const _0: Self = Self::new(0);
        #[doc = "Channel is forced to return from bus-off"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Beie_SPEC;
    pub type Beie = crate::EnumBitfieldStruct<u8, Beie_SPEC>;
    impl Beie {
        #[doc = "Bus error interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Bus error interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ewie_SPEC;
    pub type Ewie = crate::EnumBitfieldStruct<u8, Ewie_SPEC>;
    impl Ewie {
        #[doc = "Error warning interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Error warning interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Epie_SPEC;
    pub type Epie = crate::EnumBitfieldStruct<u8, Epie_SPEC>;
    impl Epie {
        #[doc = "Error passive interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Error passive interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Boeie_SPEC;
    pub type Boeie = crate::EnumBitfieldStruct<u8, Boeie_SPEC>;
    impl Boeie {
        #[doc = "Bus-off entry interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Bus-off entry interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Borie_SPEC;
    pub type Borie = crate::EnumBitfieldStruct<u8, Borie_SPEC>;
    impl Borie {
        #[doc = "Bus-off recovery interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Bus-off recovery interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Olie_SPEC;
    pub type Olie = crate::EnumBitfieldStruct<u8, Olie_SPEC>;
    impl Olie {
        #[doc = "Overload interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Overload interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Blie_SPEC;
    pub type Blie = crate::EnumBitfieldStruct<u8, Blie_SPEC>;
    impl Blie {
        #[doc = "Bus lock interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Bus lock interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Alie_SPEC;
    pub type Alie = crate::EnumBitfieldStruct<u8, Alie_SPEC>;
    impl Alie {
        #[doc = "Arbitration lost interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Arbitration lost interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Taie_SPEC;
    pub type Taie = crate::EnumBitfieldStruct<u8, Taie_SPEC>;
    impl Taie {
        #[doc = "TX abort interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "TX abort interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eocoie_SPEC;
    pub type Eocoie = crate::EnumBitfieldStruct<u8, Eocoie_SPEC>;
    impl Eocoie {
        #[doc = "Error occurrence counter overflow interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Error occurrence counter overflow interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Socoie_SPEC;
    pub type Socoie = crate::EnumBitfieldStruct<u8, Socoie_SPEC>;
    impl Socoie {
        #[doc = "Successful occurrence counter overflow interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Successful occurrence counter overflow interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdcvfie_SPEC;
    pub type Tdcvfie = crate::EnumBitfieldStruct<u8, Tdcvfie_SPEC>;
    impl Tdcvfie {
        #[doc = "Transceiver delay compensation violation interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Transceiver delay compensation violation interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bom_SPEC;
    pub type Bom = crate::EnumBitfieldStruct<u8, Bom_SPEC>;
    impl Bom {
        #[doc = "Normal mode (comply with ISO 11898-1)"]
        pub const _00: Self = Self::new(0);
        #[doc = "Entry to Halt mode automatically at bus-off start"]
        pub const _01: Self = Self::new(1);
        #[doc = "Entry to Halt mode automatically at bus-off end"]
        pub const _10: Self = Self::new(2);
        #[doc = "Entry to Halt mode (during bus-off recovery period) by software"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Errd_SPEC;
    pub type Errd = crate::EnumBitfieldStruct<u8, Errd_SPEC>;
    impl Errd {
        #[doc = "Only the first set of error codes displayed"]
        pub const _0: Self = Self::new(0);
        #[doc = "Accumulated error codes displayed"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctme_SPEC;
    pub type Ctme = crate::EnumBitfieldStruct<u8, Ctme_SPEC>;
    impl Ctme {
        #[doc = "Channel test mode disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Channel test mode enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctms_SPEC;
    pub type Ctms = crate::EnumBitfieldStruct<u8, Ctms_SPEC>;
    impl Ctms {
        #[doc = "Basic test mode"]
        pub const _00: Self = Self::new(0);
        #[doc = "Listen-only mode"]
        pub const _01: Self = Self::new(1);
        #[doc = "Self-test mode 0 (External loopback mode)"]
        pub const _10: Self = Self::new(2);
        #[doc = "Self-test mode 1 (Internal loopback mode)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bft_SPEC;
    pub type Bft = crate::EnumBitfieldStruct<u8, Bft_SPEC>;
    impl Bft {
        #[doc = "First data bit of reception stream not inverted"]
        pub const _0: Self = Self::new(0);
        #[doc = "First data bit of reception stream inverted"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rom_SPEC;
    pub type Rom = crate::EnumBitfieldStruct<u8, Rom_SPEC>;
    impl Rom {
        #[doc = "Restricted operation mode disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Restricted operation mode enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdc0Sts_SPEC;
impl crate::sealed::RegSpec for Cfdc0Sts_SPEC {
    type DataType = u32;
}
#[doc = "Channel 0 Status Register"]
pub type Cfdc0Sts = crate::RegValueT<Cfdc0Sts_SPEC>;

impl Cfdc0Sts {
    #[doc = "Channel Reset Status"]
    #[inline(always)]
    pub fn crststs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdc0sts::Crststs,
        Cfdc0Sts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdc0sts::Crststs,
            Cfdc0Sts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Channel Halt Status"]
    #[inline(always)]
    pub fn chltsts(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdc0sts::Chltsts,
        Cfdc0Sts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdc0sts::Chltsts,
            Cfdc0Sts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Channel Sleep Status"]
    #[inline(always)]
    pub fn cslpsts(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cfdc0sts::Cslpsts,
        Cfdc0Sts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdc0sts::Cslpsts,
            Cfdc0Sts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Channel Error Passive Status"]
    #[inline(always)]
    pub fn epsts(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, cfdc0sts::Epsts, Cfdc0Sts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<3,0x1,1,0,cfdc0sts::Epsts, Cfdc0Sts_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Channel Bus-Off Status"]
    #[inline(always)]
    pub fn bosts(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, cfdc0sts::Bosts, Cfdc0Sts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0x1,1,0,cfdc0sts::Bosts, Cfdc0Sts_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Channel Transmit Status"]
    #[inline(always)]
    pub fn trmsts(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, cfdc0sts::Trmsts, Cfdc0Sts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            cfdc0sts::Trmsts,
            Cfdc0Sts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Channel Receive Status"]
    #[inline(always)]
    pub fn recsts(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, cfdc0sts::Recsts, Cfdc0Sts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            cfdc0sts::Recsts,
            Cfdc0Sts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Channel Communication Status"]
    #[inline(always)]
    pub fn comsts(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, cfdc0sts::Comsts, Cfdc0Sts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            cfdc0sts::Comsts,
            Cfdc0Sts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Error State Indication Flag"]
    #[inline(always)]
    pub fn esif(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, cfdc0sts::Esif, Cfdc0Sts_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,cfdc0sts::Esif, Cfdc0Sts_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Reception Error Count"]
    #[inline(always)]
    pub fn rec(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdc0Sts_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdc0Sts_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmission Error Count"]
    #[inline(always)]
    pub fn tec(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdc0Sts_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdc0Sts_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdc0Sts {
    #[inline(always)]
    fn default() -> Cfdc0Sts {
        <crate::RegValueT<Cfdc0Sts_SPEC> as RegisterValue<_>>::new(5)
    }
}
pub mod cfdc0sts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Crststs_SPEC;
    pub type Crststs = crate::EnumBitfieldStruct<u8, Crststs_SPEC>;
    impl Crststs {
        #[doc = "Channel not in Reset mode"]
        pub const _0: Self = Self::new(0);
        #[doc = "Channel in Reset mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chltsts_SPEC;
    pub type Chltsts = crate::EnumBitfieldStruct<u8, Chltsts_SPEC>;
    impl Chltsts {
        #[doc = "Channel not in Halt mode"]
        pub const _0: Self = Self::new(0);
        #[doc = "Channel in Halt mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cslpsts_SPEC;
    pub type Cslpsts = crate::EnumBitfieldStruct<u8, Cslpsts_SPEC>;
    impl Cslpsts {
        #[doc = "Channel not in Sleep mode"]
        pub const _0: Self = Self::new(0);
        #[doc = "Channel in Sleep mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Epsts_SPEC;
    pub type Epsts = crate::EnumBitfieldStruct<u8, Epsts_SPEC>;
    impl Epsts {
        #[doc = "Channel not in error passive state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Channel in error passive state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bosts_SPEC;
    pub type Bosts = crate::EnumBitfieldStruct<u8, Bosts_SPEC>;
    impl Bosts {
        #[doc = "Channel not in bus-off state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Channel in bus-off state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trmsts_SPEC;
    pub type Trmsts = crate::EnumBitfieldStruct<u8, Trmsts_SPEC>;
    impl Trmsts {
        #[doc = "Channel is not transmitting"]
        pub const _0: Self = Self::new(0);
        #[doc = "Channel is transmitting"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Recsts_SPEC;
    pub type Recsts = crate::EnumBitfieldStruct<u8, Recsts_SPEC>;
    impl Recsts {
        #[doc = "Channel is not receiving"]
        pub const _0: Self = Self::new(0);
        #[doc = "Channel is receiving"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Comsts_SPEC;
    pub type Comsts = crate::EnumBitfieldStruct<u8, Comsts_SPEC>;
    impl Comsts {
        #[doc = "Channel is not ready for communication"]
        pub const _0: Self = Self::new(0);
        #[doc = "Channel is ready for communication"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Esif_SPEC;
    pub type Esif = crate::EnumBitfieldStruct<u8, Esif_SPEC>;
    impl Esif {
        #[doc = "No CANFD message has been received when the ESI flag was set"]
        pub const _0: Self = Self::new(0);
        #[doc = "At least one CANFD message was received when the ESI flag was set"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdc0Erfl_SPEC;
impl crate::sealed::RegSpec for Cfdc0Erfl_SPEC {
    type DataType = u32;
}
#[doc = "Channel 0 Error Flag Register"]
pub type Cfdc0Erfl = crate::RegValueT<Cfdc0Erfl_SPEC>;

impl Cfdc0Erfl {
    #[doc = "Bus Error Flag"]
    #[inline(always)]
    pub fn bef(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, cfdc0erfl::Bef, Cfdc0Erfl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdc0erfl::Bef,
            Cfdc0Erfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Error Warning Flag"]
    #[inline(always)]
    pub fn ewf(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, cfdc0erfl::Ewf, Cfdc0Erfl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdc0erfl::Ewf,
            Cfdc0Erfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Error Passive Flag"]
    #[inline(always)]
    pub fn epf(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, cfdc0erfl::Epf, Cfdc0Erfl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdc0erfl::Epf,
            Cfdc0Erfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Bus-Off Entry Flag"]
    #[inline(always)]
    pub fn boef(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        cfdc0erfl::Boef,
        Cfdc0Erfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            cfdc0erfl::Boef,
            Cfdc0Erfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Bus-Off Recovery Flag"]
    #[inline(always)]
    pub fn borf(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        cfdc0erfl::Borf,
        Cfdc0Erfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            cfdc0erfl::Borf,
            Cfdc0Erfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Overload Flag"]
    #[inline(always)]
    pub fn ovlf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        cfdc0erfl::Ovlf,
        Cfdc0Erfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            cfdc0erfl::Ovlf,
            Cfdc0Erfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Bus Lock Flag"]
    #[inline(always)]
    pub fn blf(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, cfdc0erfl::Blf, Cfdc0Erfl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            cfdc0erfl::Blf,
            Cfdc0Erfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Arbitration Lost Flag"]
    #[inline(always)]
    pub fn alf(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, cfdc0erfl::Alf, Cfdc0Erfl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            cfdc0erfl::Alf,
            Cfdc0Erfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Stuff Error"]
    #[inline(always)]
    pub fn serr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cfdc0erfl::Serr,
        Cfdc0Erfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cfdc0erfl::Serr,
            Cfdc0Erfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Form Error"]
    #[inline(always)]
    pub fn ferr(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        cfdc0erfl::Ferr,
        Cfdc0Erfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            cfdc0erfl::Ferr,
            Cfdc0Erfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Acknowledge Error"]
    #[inline(always)]
    pub fn aerr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        cfdc0erfl::Aerr,
        Cfdc0Erfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            cfdc0erfl::Aerr,
            Cfdc0Erfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CRC Error"]
    #[inline(always)]
    pub fn cerr(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        cfdc0erfl::Cerr,
        Cfdc0Erfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            cfdc0erfl::Cerr,
            Cfdc0Erfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Bit 1 Error"]
    #[inline(always)]
    pub fn b1err(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        cfdc0erfl::B1Err,
        Cfdc0Erfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            cfdc0erfl::B1Err,
            Cfdc0Erfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Bit 0 Error"]
    #[inline(always)]
    pub fn b0err(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        cfdc0erfl::B0Err,
        Cfdc0Erfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            cfdc0erfl::B0Err,
            Cfdc0Erfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Acknowledge Delimiter Error"]
    #[inline(always)]
    pub fn aderr(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        cfdc0erfl::Aderr,
        Cfdc0Erfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            cfdc0erfl::Aderr,
            Cfdc0Erfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CRC Register value"]
    #[inline(always)]
    pub fn crcreg(
        self,
    ) -> crate::common::RegisterField<16, 0x7fff, 1, 0, u16, Cfdc0Erfl_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7fff,1,0,u16, Cfdc0Erfl_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdc0Erfl {
    #[inline(always)]
    fn default() -> Cfdc0Erfl {
        <crate::RegValueT<Cfdc0Erfl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdc0erfl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bef_SPEC;
    pub type Bef = crate::EnumBitfieldStruct<u8, Bef_SPEC>;
    impl Bef {
        #[doc = "Channel bus error not detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "Channel bus error detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ewf_SPEC;
    pub type Ewf = crate::EnumBitfieldStruct<u8, Ewf_SPEC>;
    impl Ewf {
        #[doc = "Channel error warning not detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "Channel error warning detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Epf_SPEC;
    pub type Epf = crate::EnumBitfieldStruct<u8, Epf_SPEC>;
    impl Epf {
        #[doc = "Channel error passive not detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "Channel error passive detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Boef_SPEC;
    pub type Boef = crate::EnumBitfieldStruct<u8, Boef_SPEC>;
    impl Boef {
        #[doc = "Channel bus-off entry not detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "Channel bus-off entry detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Borf_SPEC;
    pub type Borf = crate::EnumBitfieldStruct<u8, Borf_SPEC>;
    impl Borf {
        #[doc = "Channel bus-off recovery not detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "Channel bus-off recovery detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovlf_SPEC;
    pub type Ovlf = crate::EnumBitfieldStruct<u8, Ovlf_SPEC>;
    impl Ovlf {
        #[doc = "Channel overload not detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "Channel overload detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Blf_SPEC;
    pub type Blf = crate::EnumBitfieldStruct<u8, Blf_SPEC>;
    impl Blf {
        #[doc = "Channel bus lock not detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "Channel bus lock detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Alf_SPEC;
    pub type Alf = crate::EnumBitfieldStruct<u8, Alf_SPEC>;
    impl Alf {
        #[doc = "Channel arbitration lost not detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "Channel arbitration lost detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Serr_SPEC;
    pub type Serr = crate::EnumBitfieldStruct<u8, Serr_SPEC>;
    impl Serr {
        #[doc = "Channel stuff error not detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "Channel stuff error detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ferr_SPEC;
    pub type Ferr = crate::EnumBitfieldStruct<u8, Ferr_SPEC>;
    impl Ferr {
        #[doc = "Channel form error not detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "Channel form error detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aerr_SPEC;
    pub type Aerr = crate::EnumBitfieldStruct<u8, Aerr_SPEC>;
    impl Aerr {
        #[doc = "Channel acknowledge error not detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "Channel acknowledge error detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cerr_SPEC;
    pub type Cerr = crate::EnumBitfieldStruct<u8, Cerr_SPEC>;
    impl Cerr {
        #[doc = "Channel CRC error not detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "Channel CRC error detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct B1Err_SPEC;
    pub type B1Err = crate::EnumBitfieldStruct<u8, B1Err_SPEC>;
    impl B1Err {
        #[doc = "Channel bit 1 error not detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "Channel bit 1 error detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct B0Err_SPEC;
    pub type B0Err = crate::EnumBitfieldStruct<u8, B0Err_SPEC>;
    impl B0Err {
        #[doc = "Channel bit 0 error not detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "Channel bit 0 error detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aderr_SPEC;
    pub type Aderr = crate::EnumBitfieldStruct<u8, Aderr_SPEC>;
    impl Aderr {
        #[doc = "Channel acknowledge delimiter error not detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "Channel acknowledge delimiter error detected"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgcfg_SPEC;
impl crate::sealed::RegSpec for Cfdgcfg_SPEC {
    type DataType = u32;
}
#[doc = "Global Configuration Register"]
pub type Cfdgcfg = crate::RegValueT<Cfdgcfg_SPEC>;

impl Cfdgcfg {
    #[doc = "Transmission Priority"]
    #[inline(always)]
    pub fn tpri(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, cfdgcfg::Tpri, Cfdgcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,cfdgcfg::Tpri, Cfdgcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DLC Check Enable"]
    #[inline(always)]
    pub fn dce(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, cfdgcfg::Dce, Cfdgcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,cfdgcfg::Dce, Cfdgcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DLC Replacement Enable"]
    #[inline(always)]
    pub fn dre(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, cfdgcfg::Dre, Cfdgcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,cfdgcfg::Dre, Cfdgcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Mirror Mode Enable"]
    #[inline(always)]
    pub fn mme(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, cfdgcfg::Mme, Cfdgcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,cfdgcfg::Mme, Cfdgcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Data Link Controller Clock Select"]
    #[inline(always)]
    pub fn dcs(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, cfdgcfg::Dcs, Cfdgcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,cfdgcfg::Dcs, Cfdgcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CANFD Message Payload Overflow Configuration"]
    #[inline(always)]
    pub fn cmpoc(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, cfdgcfg::Cmpoc, Cfdgcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,cfdgcfg::Cmpoc, Cfdgcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timestamp Prescaler"]
    #[inline(always)]
    pub fn tsp(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Cfdgcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Cfdgcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timestamp Source Select"]
    #[inline(always)]
    pub fn tsss(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, cfdgcfg::Tsss, Cfdgcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,cfdgcfg::Tsss, Cfdgcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interval Timer Reference Clock Prescaler"]
    #[inline(always)]
    pub fn itrcp(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Cfdgcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Cfdgcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdgcfg {
    #[inline(always)]
    fn default() -> Cfdgcfg {
        <crate::RegValueT<Cfdgcfg_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdgcfg {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tpri_SPEC;
    pub type Tpri = crate::EnumBitfieldStruct<u8, Tpri_SPEC>;
    impl Tpri {
        #[doc = "ID priority"]
        pub const _0: Self = Self::new(0);
        #[doc = "Message buffer number priority"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dce_SPEC;
    pub type Dce = crate::EnumBitfieldStruct<u8, Dce_SPEC>;
    impl Dce {
        #[doc = "DLC check disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "DLC check enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dre_SPEC;
    pub type Dre = crate::EnumBitfieldStruct<u8, Dre_SPEC>;
    impl Dre {
        #[doc = "DLC replacement disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "DLC replacement enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mme_SPEC;
    pub type Mme = crate::EnumBitfieldStruct<u8, Mme_SPEC>;
    impl Mme {
        #[doc = "Mirror mode disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mirror mode enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcs_SPEC;
    pub type Dcs = crate::EnumBitfieldStruct<u8, Dcs_SPEC>;
    impl Dcs {
        #[doc = "Internal clean clock"]
        pub const _0: Self = Self::new(0);
        #[doc = "External clock source connected to CANMCLK pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpoc_SPEC;
    pub type Cmpoc = crate::EnumBitfieldStruct<u8, Cmpoc_SPEC>;
    impl Cmpoc {
        #[doc = "Message is rejected"]
        pub const _0: Self = Self::new(0);
        #[doc = "Message payload is cut to fit to configured message size"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsss_SPEC;
    pub type Tsss = crate::EnumBitfieldStruct<u8, Tsss_SPEC>;
    impl Tsss {
        #[doc = "Source clock for timestamp counter is peripheral clock"]
        pub const _0: Self = Self::new(0);
        #[doc = "Source clock for timestamp counter is bit time clock"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgctr_SPEC;
impl crate::sealed::RegSpec for Cfdgctr_SPEC {
    type DataType = u32;
}
#[doc = "Global Control Register"]
pub type Cfdgctr = crate::RegValueT<Cfdgctr_SPEC>;

impl Cfdgctr {
    #[doc = "Global Mode Control"]
    #[inline(always)]
    pub fn gmdc(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, cfdgctr::Gmdc, Cfdgctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,cfdgctr::Gmdc, Cfdgctr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Global Sleep Request"]
    #[inline(always)]
    pub fn gslpr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, cfdgctr::Gslpr, Cfdgctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,cfdgctr::Gslpr, Cfdgctr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DLC Check Interrupt Enable"]
    #[inline(always)]
    pub fn deie(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, cfdgctr::Deie, Cfdgctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,cfdgctr::Deie, Cfdgctr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Lost Error Interrupt Enable"]
    #[inline(always)]
    pub fn meie(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, cfdgctr::Meie, Cfdgctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,cfdgctr::Meie, Cfdgctr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX History List Entry Lost Interrupt Enable"]
    #[inline(always)]
    pub fn thleie(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, cfdgctr::Thleie, Cfdgctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            cfdgctr::Thleie,
            Cfdgctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CANFD Message Payload Overflow Flag Interrupt Enable"]
    #[inline(always)]
    pub fn cmpofie(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        cfdgctr::Cmpofie,
        Cfdgctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            cfdgctr::Cmpofie,
            Cfdgctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Timestamp Reset"]
    #[inline(always)]
    pub fn tsrst(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, cfdgctr::Tsrst, Cfdgctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,cfdgctr::Tsrst, Cfdgctr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdgctr {
    #[inline(always)]
    fn default() -> Cfdgctr {
        <crate::RegValueT<Cfdgctr_SPEC> as RegisterValue<_>>::new(5)
    }
}
pub mod cfdgctr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gmdc_SPEC;
    pub type Gmdc = crate::EnumBitfieldStruct<u8, Gmdc_SPEC>;
    impl Gmdc {
        #[doc = "Global operation mode request"]
        pub const _00: Self = Self::new(0);
        #[doc = "Global reset mode request"]
        pub const _01: Self = Self::new(1);
        #[doc = "Global halt mode request"]
        pub const _10: Self = Self::new(2);
        #[doc = "Keep current value"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gslpr_SPEC;
    pub type Gslpr = crate::EnumBitfieldStruct<u8, Gslpr_SPEC>;
    impl Gslpr {
        #[doc = "Global sleep request disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Global sleep request enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Deie_SPEC;
    pub type Deie = crate::EnumBitfieldStruct<u8, Deie_SPEC>;
    impl Deie {
        #[doc = "DLC check interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "DLC check interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Meie_SPEC;
    pub type Meie = crate::EnumBitfieldStruct<u8, Meie_SPEC>;
    impl Meie {
        #[doc = "Message lost error interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Message lost error interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Thleie_SPEC;
    pub type Thleie = crate::EnumBitfieldStruct<u8, Thleie_SPEC>;
    impl Thleie {
        #[doc = "TX history list entry lost interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "TX history list entry lost interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpofie_SPEC;
    pub type Cmpofie = crate::EnumBitfieldStruct<u8, Cmpofie_SPEC>;
    impl Cmpofie {
        #[doc = "CANFD message payload overflow flag interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "CANFD message payload overflow flag interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsrst_SPEC;
    pub type Tsrst = crate::EnumBitfieldStruct<u8, Tsrst_SPEC>;
    impl Tsrst {
        #[doc = "Timestamp not reset"]
        pub const _0: Self = Self::new(0);
        #[doc = "Timestamp reset"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgsts_SPEC;
impl crate::sealed::RegSpec for Cfdgsts_SPEC {
    type DataType = u32;
}
#[doc = "Global Status Register"]
pub type Cfdgsts = crate::RegValueT<Cfdgsts_SPEC>;

impl Cfdgsts {
    #[doc = "Global Reset Status"]
    #[inline(always)]
    pub fn grststs(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, cfdgsts::Grststs, Cfdgsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,cfdgsts::Grststs, Cfdgsts_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Global Halt Status"]
    #[inline(always)]
    pub fn ghltsts(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, cfdgsts::Ghltsts, Cfdgsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,cfdgsts::Ghltsts, Cfdgsts_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Global Sleep Status"]
    #[inline(always)]
    pub fn gslpsts(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, cfdgsts::Gslpsts, Cfdgsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x1,1,0,cfdgsts::Gslpsts, Cfdgsts_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Global RAM Initialization"]
    #[inline(always)]
    pub fn graminit(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, cfdgsts::Graminit, Cfdgsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            cfdgsts::Graminit,
            Cfdgsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdgsts {
    #[inline(always)]
    fn default() -> Cfdgsts {
        <crate::RegValueT<Cfdgsts_SPEC> as RegisterValue<_>>::new(13)
    }
}
pub mod cfdgsts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Grststs_SPEC;
    pub type Grststs = crate::EnumBitfieldStruct<u8, Grststs_SPEC>;
    impl Grststs {
        #[doc = "Not in Reset mode"]
        pub const _0: Self = Self::new(0);
        #[doc = "In Reset mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ghltsts_SPEC;
    pub type Ghltsts = crate::EnumBitfieldStruct<u8, Ghltsts_SPEC>;
    impl Ghltsts {
        #[doc = "Not in Halt mode"]
        pub const _0: Self = Self::new(0);
        #[doc = "In Halt mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gslpsts_SPEC;
    pub type Gslpsts = crate::EnumBitfieldStruct<u8, Gslpsts_SPEC>;
    impl Gslpsts {
        #[doc = "Not in Sleep mode"]
        pub const _0: Self = Self::new(0);
        #[doc = "In Sleep mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Graminit_SPEC;
    pub type Graminit = crate::EnumBitfieldStruct<u8, Graminit_SPEC>;
    impl Graminit {
        #[doc = "RAM initialization is complete"]
        pub const _0: Self = Self::new(0);
        #[doc = "RAM initialization is ongoing"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgerfl_SPEC;
impl crate::sealed::RegSpec for Cfdgerfl_SPEC {
    type DataType = u32;
}
#[doc = "Global Error Flag Register"]
pub type Cfdgerfl = crate::RegValueT<Cfdgerfl_SPEC>;

impl Cfdgerfl {
    #[doc = "DLC Error Flag"]
    #[inline(always)]
    pub fn def(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, cfdgerfl::Def, Cfdgerfl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,cfdgerfl::Def, Cfdgerfl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Lost Error Status"]
    #[inline(always)]
    pub fn mes(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, cfdgerfl::Mes, Cfdgerfl_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,cfdgerfl::Mes, Cfdgerfl_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "TX History List Entry Lost Error Status"]
    #[inline(always)]
    pub fn thles(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, cfdgerfl::Thles, Cfdgerfl_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x1,1,0,cfdgerfl::Thles, Cfdgerfl_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "CANFD Message Payload Overflow Flag"]
    #[inline(always)]
    pub fn cmpof(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, cfdgerfl::Cmpof, Cfdgerfl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            cfdgerfl::Cmpof,
            Cfdgerfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ECC Error Flag"]
    #[inline(always)]
    pub fn eef0(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, cfdgerfl::Eef0, Cfdgerfl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            cfdgerfl::Eef0,
            Cfdgerfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdgerfl {
    #[inline(always)]
    fn default() -> Cfdgerfl {
        <crate::RegValueT<Cfdgerfl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdgerfl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Def_SPEC;
    pub type Def = crate::EnumBitfieldStruct<u8, Def_SPEC>;
    impl Def {
        #[doc = "DLC error not detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "DLC error detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mes_SPEC;
    pub type Mes = crate::EnumBitfieldStruct<u8, Mes_SPEC>;
    impl Mes {
        #[doc = "Message lost error not detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "Message lost error detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Thles_SPEC;
    pub type Thles = crate::EnumBitfieldStruct<u8, Thles_SPEC>;
    impl Thles {
        #[doc = "TX history list entry lost error not detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "TX history list entry lost error detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpof_SPEC;
    pub type Cmpof = crate::EnumBitfieldStruct<u8, Cmpof_SPEC>;
    impl Cmpof {
        #[doc = "CANFD message payload overflow not detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "CANFD message payload overflow detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eef0_SPEC;
    pub type Eef0 = crate::EnumBitfieldStruct<u8, Eef0_SPEC>;
    impl Eef0 {
        #[doc = "ECC error not detected during TX-SCAN"]
        pub const _0: Self = Self::new(0);
        #[doc = "ECC error detected during TX-SCAN"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgtsc_SPEC;
impl crate::sealed::RegSpec for Cfdgtsc_SPEC {
    type DataType = u32;
}
#[doc = "Global Timestamp Counter Register"]
pub type Cfdgtsc = crate::RegValueT<Cfdgtsc_SPEC>;

impl Cfdgtsc {
    #[doc = "Timestamp value"]
    #[inline(always)]
    pub fn ts(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Cfdgtsc_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Cfdgtsc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdgtsc {
    #[inline(always)]
    fn default() -> Cfdgtsc {
        <crate::RegValueT<Cfdgtsc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgaflectr_SPEC;
impl crate::sealed::RegSpec for Cfdgaflectr_SPEC {
    type DataType = u32;
}
#[doc = "Global Acceptance Filter List Entry Control Register"]
pub type Cfdgaflectr = crate::RegValueT<Cfdgaflectr_SPEC>;

impl Cfdgaflectr {
    #[doc = "Acceptance Filter List Page Number"]
    #[inline(always)]
    pub fn aflpn(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Cfdgaflectr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Cfdgaflectr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Acceptance Filter List Data Access Enable"]
    #[inline(always)]
    pub fn afldae(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cfdgaflectr::Afldae,
        Cfdgaflectr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cfdgaflectr::Afldae,
            Cfdgaflectr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdgaflectr {
    #[inline(always)]
    fn default() -> Cfdgaflectr {
        <crate::RegValueT<Cfdgaflectr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdgaflectr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Afldae_SPEC;
    pub type Afldae = crate::EnumBitfieldStruct<u8, Afldae_SPEC>;
    impl Afldae {
        #[doc = "Acceptance Filter List data access disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Acceptance Filter List data access enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgaflcfg_SPEC;
impl crate::sealed::RegSpec for Cfdgaflcfg_SPEC {
    type DataType = u32;
}
#[doc = "Global Acceptance Filter List Configuration Register"]
pub type Cfdgaflcfg = crate::RegValueT<Cfdgaflcfg_SPEC>;

impl Cfdgaflcfg {
    #[doc = "Rule Number"]
    #[inline(always)]
    pub fn rnc0(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, Cfdgaflcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3f,1,0,u8, Cfdgaflcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdgaflcfg {
    #[inline(always)]
    fn default() -> Cfdgaflcfg {
        <crate::RegValueT<Cfdgaflcfg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmnb_SPEC;
impl crate::sealed::RegSpec for Cfdrmnb_SPEC {
    type DataType = u32;
}
#[doc = "RX Message Buffer Number Register"]
pub type Cfdrmnb = crate::RegValueT<Cfdrmnb_SPEC>;

impl Cfdrmnb {
    #[doc = "Number of RX Message Buffers"]
    #[inline(always)]
    pub fn nrxmb(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Cfdrmnb_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, Cfdrmnb_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Reception Message Buffer Payload Data Size"]
    #[inline(always)]
    pub fn rmpls(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, cfdrmnb::Rmpls, Cfdrmnb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,cfdrmnb::Rmpls, Cfdrmnb_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmnb {
    #[inline(always)]
    fn default() -> Cfdrmnb {
        <crate::RegValueT<Cfdrmnb_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmnb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmpls_SPEC;
    pub type Rmpls = crate::EnumBitfieldStruct<u8, Rmpls_SPEC>;
    impl Rmpls {
        #[doc = "8 bytes"]
        pub const _000: Self = Self::new(0);
        #[doc = "12 bytes"]
        pub const _001: Self = Self::new(1);
        #[doc = "16 bytes"]
        pub const _010: Self = Self::new(2);
        #[doc = "20 bytes"]
        pub const _011: Self = Self::new(3);
        #[doc = "24 bytes"]
        pub const _100: Self = Self::new(4);
        #[doc = "32 bytes"]
        pub const _101: Self = Self::new(5);
        #[doc = "48 bytes"]
        pub const _110: Self = Self::new(6);
        #[doc = "64 bytes"]
        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmnd_SPEC;
impl crate::sealed::RegSpec for Cfdrmnd_SPEC {
    type DataType = u32;
}
#[doc = "RX Message Buffer New Data Register"]
pub type Cfdrmnd = crate::RegValueT<Cfdrmnd_SPEC>;

impl Cfdrmnd {
    #[doc = "RX Message Buffer New Data Status"]
    #[inline(always)]
    pub fn rmns(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        cfdrmnd::Rmns,
        Cfdrmnd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            cfdrmnd::Rmns,
            Cfdrmnd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmnd {
    #[inline(always)]
    fn default() -> Cfdrmnd {
        <crate::RegValueT<Cfdrmnd_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmnd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmns_SPEC;
    pub type Rmns = crate::EnumBitfieldStruct<u8, Rmns_SPEC>;
    impl Rmns {
        #[doc = "New data not stored in corresponding RX message buffer"]
        pub const _0: Self = Self::new(0);
        #[doc = "New data stored in corresponding RX message buffer"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmiec_SPEC;
impl crate::sealed::RegSpec for Cfdrmiec_SPEC {
    type DataType = u32;
}
#[doc = "RX Message Buffer Interrupt Enable Configuration Register"]
pub type Cfdrmiec = crate::RegValueT<Cfdrmiec_SPEC>;

impl Cfdrmiec {
    #[doc = "RX Message Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn rmieg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        cfdrmiec::RmiEg,
        Cfdrmiec_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            cfdrmiec::RmiEg,
            Cfdrmiec_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmiec {
    #[inline(always)]
    fn default() -> Cfdrmiec {
        <crate::RegValueT<Cfdrmiec_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmiec {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct RmiEg_SPEC;
    pub type RmiEg = crate::EnumBitfieldStruct<u8, RmiEg_SPEC>;
    impl RmiEg {
        #[doc = "RX Message Buffer Interrupt disabled for corresponding RX message buffer"]
        pub const _0: Self = Self::new(0);
        #[doc = "RX Message Buffer Interrupt enabled for corresponding RX message buffer"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfcc_SPEC;
impl crate::sealed::RegSpec for Cfdrfcc_SPEC {
    type DataType = u32;
}
#[doc = "RX FIFO Configuration/Control Registers %s"]
pub type Cfdrfcc = crate::RegValueT<Cfdrfcc_SPEC>;

impl Cfdrfcc {
    #[doc = "RX FIFO Enable"]
    #[inline(always)]
    pub fn rfe(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, cfdrfcc::Rfe, Cfdrfcc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,cfdrfcc::Rfe, Cfdrfcc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "RX FIFO Interrupt Enable"]
    #[inline(always)]
    pub fn rfie(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, cfdrfcc::Rfie, Cfdrfcc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,cfdrfcc::Rfie, Cfdrfcc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Rx FIFO Payload Data Size Configuration"]
    #[inline(always)]
    pub fn rfpls(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, cfdrfcc::Rfpls, Cfdrfcc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x7,1,0,cfdrfcc::Rfpls, Cfdrfcc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "RX FIFO Depth Configuration"]
    #[inline(always)]
    pub fn rfdc(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, cfdrfcc::Rfdc, Cfdrfcc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,cfdrfcc::Rfdc, Cfdrfcc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "RX FIFO Interrupt Mode"]
    #[inline(always)]
    pub fn rfim(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, cfdrfcc::Rfim, Cfdrfcc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,cfdrfcc::Rfim, Cfdrfcc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "RX FIFO Interrupt Generation Counter Value"]
    #[inline(always)]
    pub fn rfigcv(
        self,
    ) -> crate::common::RegisterField<13, 0x7, 1, 0, cfdrfcc::Rfigcv, Cfdrfcc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            13,
            0x7,
            1,
            0,
            cfdrfcc::Rfigcv,
            Cfdrfcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrfcc {
    #[inline(always)]
    fn default() -> Cfdrfcc {
        <crate::RegValueT<Cfdrfcc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrfcc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfe_SPEC;
    pub type Rfe = crate::EnumBitfieldStruct<u8, Rfe_SPEC>;
    impl Rfe {
        #[doc = "FIFO disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "FIFO enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfie_SPEC;
    pub type Rfie = crate::EnumBitfieldStruct<u8, Rfie_SPEC>;
    impl Rfie {
        #[doc = "FIFO interrupt generation disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "FIFO interrupt generation enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfpls_SPEC;
    pub type Rfpls = crate::EnumBitfieldStruct<u8, Rfpls_SPEC>;
    impl Rfpls {
        #[doc = "8 bytes"]
        pub const _000: Self = Self::new(0);
        #[doc = "12 bytes"]
        pub const _001: Self = Self::new(1);
        #[doc = "16 bytes"]
        pub const _010: Self = Self::new(2);
        #[doc = "20 bytes"]
        pub const _011: Self = Self::new(3);
        #[doc = "24 bytes"]
        pub const _100: Self = Self::new(4);
        #[doc = "32 bytes"]
        pub const _101: Self = Self::new(5);
        #[doc = "48 bytes"]
        pub const _110: Self = Self::new(6);
        #[doc = "64 bytes"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdc_SPEC;
    pub type Rfdc = crate::EnumBitfieldStruct<u8, Rfdc_SPEC>;
    impl Rfdc {
        #[doc = "FIFO Depth = 0 message"]
        pub const _000: Self = Self::new(0);
        #[doc = "FIFO Depth = 4 messages"]
        pub const _001: Self = Self::new(1);
        #[doc = "FIFO Depth = 8 messages"]
        pub const _010: Self = Self::new(2);
        #[doc = "FIFO Depth = 16 messages"]
        pub const _011: Self = Self::new(3);
        #[doc = "FIFO Depth = 32 messages"]
        pub const _100: Self = Self::new(4);
        #[doc = "FIFO Depth = 48 messages"]
        pub const _101: Self = Self::new(5);
        #[doc = "Reserved"]
        pub const _110: Self = Self::new(6);
        #[doc = "Reserved"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfim_SPEC;
    pub type Rfim = crate::EnumBitfieldStruct<u8, Rfim_SPEC>;
    impl Rfim {
        #[doc = "Interrupt generated when RX FIFO counter reaches RFIGCV value from values smaller than RFIGCV"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt generated at the end of every received message storage"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfigcv_SPEC;
    pub type Rfigcv = crate::EnumBitfieldStruct<u8, Rfigcv_SPEC>;
    impl Rfigcv {
        #[doc = "Interrupt generated when FIFO is 1/8th full"]
        pub const _000: Self = Self::new(0);
        #[doc = "Interrupt generated when FIFO is 1/4th full"]
        pub const _001: Self = Self::new(1);
        #[doc = "Interrupt generated when FIFO is 3/8th full"]
        pub const _010: Self = Self::new(2);
        #[doc = "Interrupt generated when FIFO is 1/2 full"]
        pub const _011: Self = Self::new(3);
        #[doc = "Interrupt generated when FIFO is 5/8th full"]
        pub const _100: Self = Self::new(4);
        #[doc = "Interrupt generated when FIFO is 3/4th full"]
        pub const _101: Self = Self::new(5);
        #[doc = "Interrupt generated when FIFO is 7/8th full"]
        pub const _110: Self = Self::new(6);
        #[doc = "Interrupt generated when FIFO is full"]
        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfsts_SPEC;
impl crate::sealed::RegSpec for Cfdrfsts_SPEC {
    type DataType = u32;
}
#[doc = "RX FIFO Status Registers %s"]
pub type Cfdrfsts = crate::RegValueT<Cfdrfsts_SPEC>;

impl Cfdrfsts {
    #[doc = "RX FIFO Empty"]
    #[inline(always)]
    pub fn rfemp(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, cfdrfsts::Rfemp, Cfdrfsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,cfdrfsts::Rfemp, Cfdrfsts_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Full"]
    #[inline(always)]
    pub fn rffll(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, cfdrfsts::Rffll, Cfdrfsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,cfdrfsts::Rffll, Cfdrfsts_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Message Lost"]
    #[inline(always)]
    pub fn rfmlt(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, cfdrfsts::Rfmlt, Cfdrfsts_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdrfsts::Rfmlt,
            Cfdrfsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "RX FIFO Interrupt Flag"]
    #[inline(always)]
    pub fn rfif(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, cfdrfsts::Rfif, Cfdrfsts_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,cfdrfsts::Rfif, Cfdrfsts_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "RX FIFO Message Count"]
    #[inline(always)]
    pub fn rfmc(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, Cfdrfsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x3f,1,0,u8, Cfdrfsts_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrfsts {
    #[inline(always)]
    fn default() -> Cfdrfsts {
        <crate::RegValueT<Cfdrfsts_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod cfdrfsts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfemp_SPEC;
    pub type Rfemp = crate::EnumBitfieldStruct<u8, Rfemp_SPEC>;
    impl Rfemp {
        #[doc = "FIFO not empty"]
        pub const _0: Self = Self::new(0);
        #[doc = "FIFO empty"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rffll_SPEC;
    pub type Rffll = crate::EnumBitfieldStruct<u8, Rffll_SPEC>;
    impl Rffll {
        #[doc = "FIFO not full"]
        pub const _0: Self = Self::new(0);
        #[doc = "FIFO full"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfmlt_SPEC;
    pub type Rfmlt = crate::EnumBitfieldStruct<u8, Rfmlt_SPEC>;
    impl Rfmlt {
        #[doc = "No message lost in FIFO"]
        pub const _0: Self = Self::new(0);
        #[doc = "FIFO message lost"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfif_SPEC;
    pub type Rfif = crate::EnumBitfieldStruct<u8, Rfif_SPEC>;
    impl Rfif {
        #[doc = "FIFO interrupt condition not satisfied"]
        pub const _0: Self = Self::new(0);
        #[doc = "FIFO interrupt condition satisfied"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfpctr_SPEC;
impl crate::sealed::RegSpec for Cfdrfpctr_SPEC {
    type DataType = u32;
}
#[doc = "RX FIFO Pointer Control Registers %s"]
pub type Cfdrfpctr = crate::RegValueT<Cfdrfpctr_SPEC>;

impl Cfdrfpctr {
    #[doc = "RX FIFO Pointer Control"]
    #[inline(always)]
    pub fn rfpc(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdrfpctr_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdrfpctr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrfpctr {
    #[inline(always)]
    fn default() -> Cfdrfpctr {
        <crate::RegValueT<Cfdrfpctr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfcc_SPEC;
impl crate::sealed::RegSpec for Cfdcfcc_SPEC {
    type DataType = u32;
}
#[doc = "Common FIFO Configuration/Control Register"]
pub type Cfdcfcc = crate::RegValueT<Cfdcfcc_SPEC>;

impl Cfdcfcc {
    #[doc = "Common FIFO Enable"]
    #[inline(always)]
    pub fn cfe(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, cfdcfcc::Cfe, Cfdcfcc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,cfdcfcc::Cfe, Cfdcfcc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Common FIFO RX Interrupt Enable"]
    #[inline(always)]
    pub fn cfrxie(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, cfdcfcc::Cfrxie, Cfdcfcc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,cfdcfcc::Cfrxie, Cfdcfcc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Common FIFO TX Interrupt Enable"]
    #[inline(always)]
    pub fn cftxie(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, cfdcfcc::Cftxie, Cfdcfcc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,cfdcfcc::Cftxie, Cfdcfcc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Common FIFO Payload Data Size Configuration"]
    #[inline(always)]
    pub fn cfpls(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, cfdcfcc::Cfpls, Cfdcfcc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x7,1,0,cfdcfcc::Cfpls, Cfdcfcc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Common FIFO Mode"]
    #[inline(always)]
    pub fn cfm(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, cfdcfcc::Cfm, Cfdcfcc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,cfdcfcc::Cfm, Cfdcfcc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Common FIFO Interval Timer Source Select"]
    #[inline(always)]
    pub fn cfitss(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, cfdcfcc::Cfitss, Cfdcfcc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            cfdcfcc::Cfitss,
            Cfdcfcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Common FIFO Interval Timer Resolution"]
    #[inline(always)]
    pub fn cfitr(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, cfdcfcc::Cfitr, Cfdcfcc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,cfdcfcc::Cfitr, Cfdcfcc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Common FIFO Interrupt Mode"]
    #[inline(always)]
    pub fn cfim(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, cfdcfcc::Cfim, Cfdcfcc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,cfdcfcc::Cfim, Cfdcfcc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Common FIFO Interrupt Generation Counter Value"]
    #[inline(always)]
    pub fn cfigcv(
        self,
    ) -> crate::common::RegisterField<13, 0x7, 1, 0, cfdcfcc::Cfigcv, Cfdcfcc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            13,
            0x7,
            1,
            0,
            cfdcfcc::Cfigcv,
            Cfdcfcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Common FIFO TX Message Buffer Link"]
    #[inline(always)]
    pub fn cftml(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, Cfdcfcc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8, Cfdcfcc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Common FIFO Depth Configuration"]
    #[inline(always)]
    pub fn cfdc(
        self,
    ) -> crate::common::RegisterField<21, 0x7, 1, 0, cfdcfcc::Cfdc, Cfdcfcc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x7,1,0,cfdcfcc::Cfdc, Cfdcfcc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Common FIFO Interval Transmission Time"]
    #[inline(always)]
    pub fn cfitt(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdcfcc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdcfcc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfcc {
    #[inline(always)]
    fn default() -> Cfdcfcc {
        <crate::RegValueT<Cfdcfcc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcfcc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfe_SPEC;
    pub type Cfe = crate::EnumBitfieldStruct<u8, Cfe_SPEC>;
    impl Cfe {
        #[doc = "FIFO disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "FIFO enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfrxie_SPEC;
    pub type Cfrxie = crate::EnumBitfieldStruct<u8, Cfrxie_SPEC>;
    impl Cfrxie {
        #[doc = "FIFO interrupt generation disabled for Frame RX"]
        pub const _0: Self = Self::new(0);
        #[doc = "FIFO interrupt generation enabled for Frame RX"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cftxie_SPEC;
    pub type Cftxie = crate::EnumBitfieldStruct<u8, Cftxie_SPEC>;
    impl Cftxie {
        #[doc = "FIFO interrupt generation disabled for Frame TX"]
        pub const _0: Self = Self::new(0);
        #[doc = "FIFO interrupt generation enabled for Frame TX"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfpls_SPEC;
    pub type Cfpls = crate::EnumBitfieldStruct<u8, Cfpls_SPEC>;
    impl Cfpls {
        #[doc = "8 bytes"]
        pub const _000: Self = Self::new(0);
        #[doc = "12 bytes"]
        pub const _001: Self = Self::new(1);
        #[doc = "16 bytes"]
        pub const _010: Self = Self::new(2);
        #[doc = "20 bytes"]
        pub const _011: Self = Self::new(3);
        #[doc = "24 bytes"]
        pub const _100: Self = Self::new(4);
        #[doc = "32 bytes"]
        pub const _101: Self = Self::new(5);
        #[doc = "48 bytes"]
        pub const _110: Self = Self::new(6);
        #[doc = "64 bytes"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfm_SPEC;
    pub type Cfm = crate::EnumBitfieldStruct<u8, Cfm_SPEC>;
    impl Cfm {
        #[doc = "RX FIFO mode"]
        pub const _0: Self = Self::new(0);
        #[doc = "TX FIFO mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfitss_SPEC;
    pub type Cfitss = crate::EnumBitfieldStruct<u8, Cfitss_SPEC>;
    impl Cfitss {
        #[doc = "Reference clock (× 1 / × 10 period)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Bit time clock of related channel (FIFO is linked to fixed channel)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfitr_SPEC;
    pub type Cfitr = crate::EnumBitfieldStruct<u8, Cfitr_SPEC>;
    impl Cfitr {
        #[doc = "Reference clock period × 1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Reference clock period × 10"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfim_SPEC;
    pub type Cfim = crate::EnumBitfieldStruct<u8, Cfim_SPEC>;
    impl Cfim {
        #[doc = "RX FIFO mode: RX interrupt generated when Common FIFO counter reaches CFIGCV value from a lower value TX FIFO mode: TX interrupt generated when Common FIFO transmits the last message successfully"]
        pub const _0: Self = Self::new(0);
        #[doc = "RX FIFO mode: RX interrupt generated at the end of every received message storage TX FIFO mode: interrupt generated for every successfully transmitted message"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfigcv_SPEC;
    pub type Cfigcv = crate::EnumBitfieldStruct<u8, Cfigcv_SPEC>;
    impl Cfigcv {
        #[doc = "Interrupt generated when FIFO is 1/8th full"]
        pub const _000: Self = Self::new(0);
        #[doc = "Interrupt generated when FIFO is 1/4th full"]
        pub const _001: Self = Self::new(1);
        #[doc = "Interrupt generated when FIFO is 3/8th full"]
        pub const _010: Self = Self::new(2);
        #[doc = "Interrupt generated when FIFO is 1/2 full"]
        pub const _011: Self = Self::new(3);
        #[doc = "Interrupt generated when FIFO is 5/8th full"]
        pub const _100: Self = Self::new(4);
        #[doc = "Interrupt generated when FIFO is 3/4th full"]
        pub const _101: Self = Self::new(5);
        #[doc = "Interrupt generated when FIFO is 7/8th full"]
        pub const _110: Self = Self::new(6);
        #[doc = "Interrupt generated when FIFO is full"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdc_SPEC;
    pub type Cfdc = crate::EnumBitfieldStruct<u8, Cfdc_SPEC>;
    impl Cfdc {
        #[doc = "FIFO Depth = 0 message"]
        pub const _000: Self = Self::new(0);
        #[doc = "FIFO Depth = 4 messages"]
        pub const _001: Self = Self::new(1);
        #[doc = "FIFO Depth = 8 messages"]
        pub const _010: Self = Self::new(2);
        #[doc = "FIFO Depth = 16 messages"]
        pub const _011: Self = Self::new(3);
        #[doc = "FIFO Depth = 32 messages"]
        pub const _100: Self = Self::new(4);
        #[doc = "FIFO Depth = 48 messages"]
        pub const _101: Self = Self::new(5);
        #[doc = "FIFO Depth = Reserved"]
        pub const _110: Self = Self::new(6);
        #[doc = "FIFO Depth = Reserved"]
        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfsts_SPEC;
impl crate::sealed::RegSpec for Cfdcfsts_SPEC {
    type DataType = u32;
}
#[doc = "Common FIFO Status Register"]
pub type Cfdcfsts = crate::RegValueT<Cfdcfsts_SPEC>;

impl Cfdcfsts {
    #[doc = "Common FIFO Empty"]
    #[inline(always)]
    pub fn cfemp(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, cfdcfsts::Cfemp, Cfdcfsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,cfdcfsts::Cfemp, Cfdcfsts_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Common FIFO Full"]
    #[inline(always)]
    pub fn cffll(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, cfdcfsts::Cffll, Cfdcfsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,cfdcfsts::Cffll, Cfdcfsts_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Common FIFO Message Lost"]
    #[inline(always)]
    pub fn cfmlt(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, cfdcfsts::Cfmlt, Cfdcfsts_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdcfsts::Cfmlt,
            Cfdcfsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Common RX FIFO Interrupt Flag"]
    #[inline(always)]
    pub fn cfrxif(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        cfdcfsts::Cfrxif,
        Cfdcfsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            cfdcfsts::Cfrxif,
            Cfdcfsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Common TX FIFO Interrupt Flag"]
    #[inline(always)]
    pub fn cftxif(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        cfdcfsts::Cftxif,
        Cfdcfsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            cfdcfsts::Cftxif,
            Cfdcfsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Common FIFO Message Count"]
    #[inline(always)]
    pub fn cfmc(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, Cfdcfsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x3f,1,0,u8, Cfdcfsts_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfsts {
    #[inline(always)]
    fn default() -> Cfdcfsts {
        <crate::RegValueT<Cfdcfsts_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod cfdcfsts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfemp_SPEC;
    pub type Cfemp = crate::EnumBitfieldStruct<u8, Cfemp_SPEC>;
    impl Cfemp {
        #[doc = "FIFO not empty"]
        pub const _0: Self = Self::new(0);
        #[doc = "FIFO empty"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cffll_SPEC;
    pub type Cffll = crate::EnumBitfieldStruct<u8, Cffll_SPEC>;
    impl Cffll {
        #[doc = "FIFO not full"]
        pub const _0: Self = Self::new(0);
        #[doc = "FIFO full"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfmlt_SPEC;
    pub type Cfmlt = crate::EnumBitfieldStruct<u8, Cfmlt_SPEC>;
    impl Cfmlt {
        #[doc = "Number of message lost in FIFO"]
        pub const _0: Self = Self::new(0);
        #[doc = "FIFO message lost"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfrxif_SPEC;
    pub type Cfrxif = crate::EnumBitfieldStruct<u8, Cfrxif_SPEC>;
    impl Cfrxif {
        #[doc = "FIFO interrupt condition not satisfied after frame reception"]
        pub const _0: Self = Self::new(0);
        #[doc = "FIFO interrupt condition satisfied after frame reception"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cftxif_SPEC;
    pub type Cftxif = crate::EnumBitfieldStruct<u8, Cftxif_SPEC>;
    impl Cftxif {
        #[doc = "FIFO interrupt condition not satisfied after frame transmission"]
        pub const _0: Self = Self::new(0);
        #[doc = "FIFO Interrupt condition satisfied after frame transmission"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfpctr_SPEC;
impl crate::sealed::RegSpec for Cfdcfpctr_SPEC {
    type DataType = u32;
}
#[doc = "Common FIFO Pointer Control Register"]
pub type Cfdcfpctr = crate::RegValueT<Cfdcfpctr_SPEC>;

impl Cfdcfpctr {
    #[doc = "Common FIFO Pointer Control"]
    #[inline(always)]
    pub fn cfpc(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdcfpctr_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdcfpctr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfpctr {
    #[inline(always)]
    fn default() -> Cfdcfpctr {
        <crate::RegValueT<Cfdcfpctr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdfests_SPEC;
impl crate::sealed::RegSpec for Cfdfests_SPEC {
    type DataType = u32;
}
#[doc = "FIFO Empty Status Register"]
pub type Cfdfests = crate::RegValueT<Cfdfests_SPEC>;

impl Cfdfests {
    #[doc = "RX FIFO Empty Status"]
    #[inline(always)]
    pub fn rfxemp(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, cfdfests::Rfxemp, Cfdfests_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            cfdfests::Rfxemp,
            Cfdfests_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Common FIFO Empty Status"]
    #[inline(always)]
    pub fn cfemp(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, cfdfests::Cfemp, Cfdfests_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x1,1,0,cfdfests::Cfemp, Cfdfests_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdfests {
    #[inline(always)]
    fn default() -> Cfdfests {
        <crate::RegValueT<Cfdfests_SPEC> as RegisterValue<_>>::new(259)
    }
}
pub mod cfdfests {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfxemp_SPEC;
    pub type Rfxemp = crate::EnumBitfieldStruct<u8, Rfxemp_SPEC>;
    impl Rfxemp {
        #[doc = "Corresponding FIFO not empty"]
        pub const _0: Self = Self::new(0);
        #[doc = "Corresponding FIFO empty"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfemp_SPEC;
    pub type Cfemp = crate::EnumBitfieldStruct<u8, Cfemp_SPEC>;
    impl Cfemp {
        #[doc = "Corresponding FIFO not empty"]
        pub const _0: Self = Self::new(0);
        #[doc = "Corresponding FIFO empty"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdffsts_SPEC;
impl crate::sealed::RegSpec for Cfdffsts_SPEC {
    type DataType = u32;
}
#[doc = "FIFO Full Status Register"]
pub type Cfdffsts = crate::RegValueT<Cfdffsts_SPEC>;

impl Cfdffsts {
    #[doc = "RX FIF0 Full Status"]
    #[inline(always)]
    pub fn rfxfll(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, cfdffsts::Rfxfll, Cfdffsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            cfdffsts::Rfxfll,
            Cfdffsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Common FIF0 Full Status"]
    #[inline(always)]
    pub fn cffll(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, cfdffsts::Cffll, Cfdffsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x1,1,0,cfdffsts::Cffll, Cfdffsts_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdffsts {
    #[inline(always)]
    fn default() -> Cfdffsts {
        <crate::RegValueT<Cfdffsts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdffsts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfxfll_SPEC;
    pub type Rfxfll = crate::EnumBitfieldStruct<u8, Rfxfll_SPEC>;
    impl Rfxfll {
        #[doc = "Corresponding FIFO not full"]
        pub const _0: Self = Self::new(0);
        #[doc = "Corresponding FIFO full"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cffll_SPEC;
    pub type Cffll = crate::EnumBitfieldStruct<u8, Cffll_SPEC>;
    impl Cffll {
        #[doc = "Corresponding FIFO not full"]
        pub const _0: Self = Self::new(0);
        #[doc = "Corresponding FIFO full"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdfmsts_SPEC;
impl crate::sealed::RegSpec for Cfdfmsts_SPEC {
    type DataType = u32;
}
#[doc = "FIFO Message Lost Status Register"]
pub type Cfdfmsts = crate::RegValueT<Cfdfmsts_SPEC>;

impl Cfdfmsts {
    #[doc = "RX FIFO Message Lost Status"]
    #[inline(always)]
    pub fn rfxmlt(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, cfdfmsts::Rfxmlt, Cfdfmsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            cfdfmsts::Rfxmlt,
            Cfdfmsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Common FIFO Message Lost Status"]
    #[inline(always)]
    pub fn cfmlt(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, cfdfmsts::Cfmlt, Cfdfmsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x1,1,0,cfdfmsts::Cfmlt, Cfdfmsts_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdfmsts {
    #[inline(always)]
    fn default() -> Cfdfmsts {
        <crate::RegValueT<Cfdfmsts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdfmsts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfxmlt_SPEC;
    pub type Rfxmlt = crate::EnumBitfieldStruct<u8, Rfxmlt_SPEC>;
    impl Rfxmlt {
        #[doc = "Corresponding FIFO Message Lost flag not set"]
        pub const _0: Self = Self::new(0);
        #[doc = "Corresponding FIFO Message Lost flag set"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfmlt_SPEC;
    pub type Cfmlt = crate::EnumBitfieldStruct<u8, Cfmlt_SPEC>;
    impl Cfmlt {
        #[doc = "Corresponding FIFO Message Lost flag not set"]
        pub const _0: Self = Self::new(0);
        #[doc = "Corresponding FIFO Message Lost flag set"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfists_SPEC;
impl crate::sealed::RegSpec for Cfdrfists_SPEC {
    type DataType = u32;
}
#[doc = "RX FIFO Interrupt Flag Status Register"]
pub type Cfdrfists = crate::RegValueT<Cfdrfists_SPEC>;

impl Cfdrfists {
    #[doc = "RX FIFO\\[x\\] Interrupt Flag Status"]
    #[inline(always)]
    pub fn rfxif(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        cfdrfists::Rfxif,
        Cfdrfists_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            cfdrfists::Rfxif,
            Cfdrfists_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrfists {
    #[inline(always)]
    fn default() -> Cfdrfists {
        <crate::RegValueT<Cfdrfists_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrfists {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfxif_SPEC;
    pub type Rfxif = crate::EnumBitfieldStruct<u8, Rfxif_SPEC>;
    impl Rfxif {
        #[doc = "Corresponding RX FIFO Interrupt flag not set"]
        pub const _0: Self = Self::new(0);
        #[doc = "Corresponding RX FIFO Interrupt flag set"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmc_SPEC;
impl crate::sealed::RegSpec for Cfdtmc_SPEC {
    type DataType = u8;
}
#[doc = "TX Message Buffer Control Registers %s"]
pub type Cfdtmc = crate::RegValueT<Cfdtmc_SPEC>;

impl Cfdtmc {
    #[doc = "TX Message Buffer Transmission Request"]
    #[inline(always)]
    pub fn tmtr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, cfdtmc::Tmtr, Cfdtmc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,cfdtmc::Tmtr, Cfdtmc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Transmission Abort Request"]
    #[inline(always)]
    pub fn tmtar(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, cfdtmc::Tmtar, Cfdtmc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,cfdtmc::Tmtar, Cfdtmc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer One-shot Mode"]
    #[inline(always)]
    pub fn tmom(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, cfdtmc::Tmom, Cfdtmc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,cfdtmc::Tmom, Cfdtmc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmc {
    #[inline(always)]
    fn default() -> Cfdtmc {
        <crate::RegValueT<Cfdtmc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdtmc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmtr_SPEC;
    pub type Tmtr = crate::EnumBitfieldStruct<u8, Tmtr_SPEC>;
    impl Tmtr {
        #[doc = "TX Message buffer transmission not requested"]
        pub const _0: Self = Self::new(0);
        #[doc = "TX message buffer transmission requested"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmtar_SPEC;
    pub type Tmtar = crate::EnumBitfieldStruct<u8, Tmtar_SPEC>;
    impl Tmtar {
        #[doc = "TX message buffer transmission request abort not requested"]
        pub const _0: Self = Self::new(0);
        #[doc = "TX message buffer transmission request abort requested"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmom_SPEC;
    pub type Tmom = crate::EnumBitfieldStruct<u8, Tmom_SPEC>;
    impl Tmom {
        #[doc = "TX message buffer not configured in one-shot mode"]
        pub const _0: Self = Self::new(0);
        #[doc = "TX message buffer configured in one-shot mode"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmsts_SPEC;
impl crate::sealed::RegSpec for Cfdtmsts_SPEC {
    type DataType = u8;
}
#[doc = "TX Message Buffer Status Registers %s"]
pub type Cfdtmsts = crate::RegValueT<Cfdtmsts_SPEC>;

impl Cfdtmsts {
    #[doc = "TX Message Buffer Transmission Status"]
    #[inline(always)]
    pub fn tmtsts(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, cfdtmsts::Tmtsts, Cfdtmsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdtmsts::Tmtsts,
            Cfdtmsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "TX Message Buffer Transmission Result Flag"]
    #[inline(always)]
    pub fn tmtrf(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, cfdtmsts::Tmtrf, Cfdtmsts_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            1,
            0x3,
            1,
            0,
            cfdtmsts::Tmtrf,
            Cfdtmsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "TX Message Buffer Transmission Request Mirrored"]
    #[inline(always)]
    pub fn tmtrm(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, cfdtmsts::Tmtrm, Cfdtmsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<3,0x1,1,0,cfdtmsts::Tmtrm, Cfdtmsts_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Transmission Abort Request Mirrored"]
    #[inline(always)]
    pub fn tmtarm(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, cfdtmsts::Tmtarm, Cfdtmsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            cfdtmsts::Tmtarm,
            Cfdtmsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdtmsts {
    #[inline(always)]
    fn default() -> Cfdtmsts {
        <crate::RegValueT<Cfdtmsts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdtmsts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmtsts_SPEC;
    pub type Tmtsts = crate::EnumBitfieldStruct<u8, Tmtsts_SPEC>;
    impl Tmtsts {
        #[doc = "No on-going transmission"]
        pub const _0: Self = Self::new(0);
        #[doc = "On-going transmission"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmtrf_SPEC;
    pub type Tmtrf = crate::EnumBitfieldStruct<u8, Tmtrf_SPEC>;
    impl Tmtrf {
        #[doc = "No result"]
        pub const _00: Self = Self::new(0);
        #[doc = "Transmission aborted from the TX message buffer"]
        pub const _01: Self = Self::new(1);
        #[doc = "Transmission successful from the TX message buffer and transmission abort was not requested"]
        pub const _10: Self = Self::new(2);
        #[doc = "Transmission successful from the TX message buffer and transmission abort was requested"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmtrm_SPEC;
    pub type Tmtrm = crate::EnumBitfieldStruct<u8, Tmtrm_SPEC>;
    impl Tmtrm {
        #[doc = "TX message buffer transmission not requested"]
        pub const _0: Self = Self::new(0);
        #[doc = "TX message buffer transmission requested"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmtarm_SPEC;
    pub type Tmtarm = crate::EnumBitfieldStruct<u8, Tmtarm_SPEC>;
    impl Tmtarm {
        #[doc = "TX message buffer transmission request abort not requested"]
        pub const _0: Self = Self::new(0);
        #[doc = "TX message buffer transmission request abort requested"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmtrsts_SPEC;
impl crate::sealed::RegSpec for Cfdtmtrsts_SPEC {
    type DataType = u32;
}
#[doc = "TX Message Buffer Transmission Request Status Register"]
pub type Cfdtmtrsts = crate::RegValueT<Cfdtmtrsts_SPEC>;

impl Cfdtmtrsts {
    #[doc = "TX Message Buffer Transmission Request Status"]
    #[inline(always)]
    pub fn cfdtmtrsts(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        cfdtmtrsts::Cfdtmtrsts,
        Cfdtmtrsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            cfdtmtrsts::Cfdtmtrsts,
            Cfdtmtrsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdtmtrsts {
    #[inline(always)]
    fn default() -> Cfdtmtrsts {
        <crate::RegValueT<Cfdtmtrsts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdtmtrsts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdtmtrsts_SPEC;
    pub type Cfdtmtrsts = crate::EnumBitfieldStruct<u8, Cfdtmtrsts_SPEC>;
    impl Cfdtmtrsts {
        #[doc = "Transmission not requested for corresponding TX message buffer"]
        pub const _0: Self = Self::new(0);
        #[doc = "Transmission requested for corresponding TX message buffer"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmtarsts_SPEC;
impl crate::sealed::RegSpec for Cfdtmtarsts_SPEC {
    type DataType = u32;
}
#[doc = "TX Message Buffer Transmission Abort Request Status Register"]
pub type Cfdtmtarsts = crate::RegValueT<Cfdtmtarsts_SPEC>;

impl Cfdtmtarsts {
    #[doc = "TX Message Buffer Transmission Abort Request Status"]
    #[inline(always)]
    pub fn cfdtmtarsts(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        cfdtmtarsts::Cfdtmtarsts,
        Cfdtmtarsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            cfdtmtarsts::Cfdtmtarsts,
            Cfdtmtarsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdtmtarsts {
    #[inline(always)]
    fn default() -> Cfdtmtarsts {
        <crate::RegValueT<Cfdtmtarsts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdtmtarsts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdtmtarsts_SPEC;
    pub type Cfdtmtarsts = crate::EnumBitfieldStruct<u8, Cfdtmtarsts_SPEC>;
    impl Cfdtmtarsts {
        #[doc = "Transmission abort not requested for corresponding TX message buffer"]
        pub const _0: Self = Self::new(0);
        #[doc = "Transmission abort requested for corresponding TX message buffer"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmtcsts_SPEC;
impl crate::sealed::RegSpec for Cfdtmtcsts_SPEC {
    type DataType = u32;
}
#[doc = "TX Message Buffer Transmission Completion Status Register"]
pub type Cfdtmtcsts = crate::RegValueT<Cfdtmtcsts_SPEC>;

impl Cfdtmtcsts {
    #[doc = "TX Message Buffer Transmission Completion Status"]
    #[inline(always)]
    pub fn cfdtmtcsts(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        cfdtmtcsts::Cfdtmtcsts,
        Cfdtmtcsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            cfdtmtcsts::Cfdtmtcsts,
            Cfdtmtcsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdtmtcsts {
    #[inline(always)]
    fn default() -> Cfdtmtcsts {
        <crate::RegValueT<Cfdtmtcsts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdtmtcsts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdtmtcsts_SPEC;
    pub type Cfdtmtcsts = crate::EnumBitfieldStruct<u8, Cfdtmtcsts_SPEC>;
    impl Cfdtmtcsts {
        #[doc = "Transmission not complete for corresponding TX message buffer"]
        pub const _0: Self = Self::new(0);
        #[doc = "Transmission completed for corresponding TX message buffer"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmtasts_SPEC;
impl crate::sealed::RegSpec for Cfdtmtasts_SPEC {
    type DataType = u32;
}
#[doc = "TX Message Buffer Transmission Abort Status Register"]
pub type Cfdtmtasts = crate::RegValueT<Cfdtmtasts_SPEC>;

impl Cfdtmtasts {
    #[doc = "TX Message Buffer Transmission Abort Status"]
    #[inline(always)]
    pub fn cfdtmtasts(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        cfdtmtasts::Cfdtmtasts,
        Cfdtmtasts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            cfdtmtasts::Cfdtmtasts,
            Cfdtmtasts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdtmtasts {
    #[inline(always)]
    fn default() -> Cfdtmtasts {
        <crate::RegValueT<Cfdtmtasts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdtmtasts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdtmtasts_SPEC;
    pub type Cfdtmtasts = crate::EnumBitfieldStruct<u8, Cfdtmtasts_SPEC>;
    impl Cfdtmtasts {
        #[doc = "Transmission not aborted for corresponding TX message buffer"]
        pub const _0: Self = Self::new(0);
        #[doc = "Transmission aborted for corresponding TX message buffer"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmiec_SPEC;
impl crate::sealed::RegSpec for Cfdtmiec_SPEC {
    type DataType = u32;
}
#[doc = "TX Message Buffer Interrupt Enable Configuration Register"]
pub type Cfdtmiec = crate::RegValueT<Cfdtmiec_SPEC>;

impl Cfdtmiec {
    #[doc = "TX Message Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn tmieg(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, cfdtmiec::TmiEg, Cfdtmiec_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            cfdtmiec::TmiEg,
            Cfdtmiec_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdtmiec {
    #[inline(always)]
    fn default() -> Cfdtmiec {
        <crate::RegValueT<Cfdtmiec_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdtmiec {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct TmiEg_SPEC;
    pub type TmiEg = crate::EnumBitfieldStruct<u8, TmiEg_SPEC>;
    impl TmiEg {
        #[doc = "TX message buffer interrupt disabled for corresponding TX message buffer"]
        pub const _0: Self = Self::new(0);
        #[doc = "TX message buffer interrupt enabled for corresponding TX message buffer"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtxqcc_SPEC;
impl crate::sealed::RegSpec for Cfdtxqcc_SPEC {
    type DataType = u32;
}
#[doc = "TX Queue Configuration/Control Register"]
pub type Cfdtxqcc = crate::RegValueT<Cfdtxqcc_SPEC>;

impl Cfdtxqcc {
    #[doc = "TX Queue Enable"]
    #[inline(always)]
    pub fn txqe(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, cfdtxqcc::Txqe, Cfdtxqcc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,cfdtxqcc::Txqe, Cfdtxqcc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Queue TX Interrupt Enable"]
    #[inline(always)]
    pub fn txqtxie(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        cfdtxqcc::Txqtxie,
        Cfdtxqcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            cfdtxqcc::Txqtxie,
            Cfdtxqcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "TX Queue Interrupt Mode"]
    #[inline(always)]
    pub fn txqim(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, cfdtxqcc::Txqim, Cfdtxqcc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            cfdtxqcc::Txqim,
            Cfdtxqcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "TX Queue Depth Configuration"]
    #[inline(always)]
    pub fn txqdc(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, cfdtxqcc::Txqdc, Cfdtxqcc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            cfdtxqcc::Txqdc,
            Cfdtxqcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdtxqcc {
    #[inline(always)]
    fn default() -> Cfdtxqcc {
        <crate::RegValueT<Cfdtxqcc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdtxqcc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txqe_SPEC;
    pub type Txqe = crate::EnumBitfieldStruct<u8, Txqe_SPEC>;
    impl Txqe {
        #[doc = "TX Queue disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "TX Queue enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txqtxie_SPEC;
    pub type Txqtxie = crate::EnumBitfieldStruct<u8, Txqtxie_SPEC>;
    impl Txqtxie {
        #[doc = "TX Queue TX interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "TX Queue TX interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txqim_SPEC;
    pub type Txqim = crate::EnumBitfieldStruct<u8, Txqim_SPEC>;
    impl Txqim {
        #[doc = "When the last message is successfully transmitted"]
        pub const _0: Self = Self::new(0);
        #[doc = "At every successful transmission"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txqdc_SPEC;
    pub type Txqdc = crate::EnumBitfieldStruct<u8, Txqdc_SPEC>;
    impl Txqdc {
        #[doc = "0 messages"]
        pub const _0_X_00: Self = Self::new(0);
        #[doc = "Reserved"]
        pub const _0_X_01: Self = Self::new(1);
        #[doc = "3 messages"]
        pub const _0_X_10: Self = Self::new(0);
        #[doc = "4 messages"]
        pub const _0_X_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtxqsts_SPEC;
impl crate::sealed::RegSpec for Cfdtxqsts_SPEC {
    type DataType = u32;
}
#[doc = "TX Queue Status Register"]
pub type Cfdtxqsts = crate::RegValueT<Cfdtxqsts_SPEC>;

impl Cfdtxqsts {
    #[doc = "TX Queue Empty"]
    #[inline(always)]
    pub fn txqemp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdtxqsts::Txqemp,
        Cfdtxqsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdtxqsts::Txqemp,
            Cfdtxqsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "TX Queue Full"]
    #[inline(always)]
    pub fn txqfll(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdtxqsts::Txqfll,
        Cfdtxqsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdtxqsts::Txqfll,
            Cfdtxqsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "TX Queue TX Interrupt Flag"]
    #[inline(always)]
    pub fn txqtxif(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cfdtxqsts::Txqtxif,
        Cfdtxqsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdtxqsts::Txqtxif,
            Cfdtxqsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "TX Queue Message Count"]
    #[inline(always)]
    pub fn txqmc(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, Cfdtxqsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x7,1,0,u8, Cfdtxqsts_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtxqsts {
    #[inline(always)]
    fn default() -> Cfdtxqsts {
        <crate::RegValueT<Cfdtxqsts_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod cfdtxqsts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txqemp_SPEC;
    pub type Txqemp = crate::EnumBitfieldStruct<u8, Txqemp_SPEC>;
    impl Txqemp {
        #[doc = "TX Queue not empty"]
        pub const _0: Self = Self::new(0);
        #[doc = "TX Queue empty"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txqfll_SPEC;
    pub type Txqfll = crate::EnumBitfieldStruct<u8, Txqfll_SPEC>;
    impl Txqfll {
        #[doc = "TX Queue not full"]
        pub const _0: Self = Self::new(0);
        #[doc = "TX Queue full"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txqtxif_SPEC;
    pub type Txqtxif = crate::EnumBitfieldStruct<u8, Txqtxif_SPEC>;
    impl Txqtxif {
        #[doc = "TX Queue interrupt condition not satisfied after a frame TX"]
        pub const _0: Self = Self::new(0);
        #[doc = "TX Queue interrupt condition satisfied after a frame TX"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtxqpctr_SPEC;
impl crate::sealed::RegSpec for Cfdtxqpctr_SPEC {
    type DataType = u32;
}
#[doc = "TX Queue Pointer Control Register"]
pub type Cfdtxqpctr = crate::RegValueT<Cfdtxqpctr_SPEC>;

impl Cfdtxqpctr {
    #[doc = "TX Queue Pointer Control"]
    #[inline(always)]
    pub fn txqpc(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdtxqpctr_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdtxqpctr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtxqpctr {
    #[inline(always)]
    fn default() -> Cfdtxqpctr {
        <crate::RegValueT<Cfdtxqpctr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdthlcc_SPEC;
impl crate::sealed::RegSpec for Cfdthlcc_SPEC {
    type DataType = u32;
}
#[doc = "TX History List Configuration/Control Register"]
pub type Cfdthlcc = crate::RegValueT<Cfdthlcc_SPEC>;

impl Cfdthlcc {
    #[doc = "TX History List Enable"]
    #[inline(always)]
    pub fn thle(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, cfdthlcc::Thle, Cfdthlcc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,cfdthlcc::Thle, Cfdthlcc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX History List Interrupt Enable"]
    #[inline(always)]
    pub fn thlie(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, cfdthlcc::Thlie, Cfdthlcc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cfdthlcc::Thlie,
            Cfdthlcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "TX History List Interrupt Mode"]
    #[inline(always)]
    pub fn thlim(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, cfdthlcc::Thlim, Cfdthlcc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            cfdthlcc::Thlim,
            Cfdthlcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "TX History List Dedicated TX Enable"]
    #[inline(always)]
    pub fn thldte(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        cfdthlcc::Thldte,
        Cfdthlcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            cfdthlcc::Thldte,
            Cfdthlcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdthlcc {
    #[inline(always)]
    fn default() -> Cfdthlcc {
        <crate::RegValueT<Cfdthlcc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdthlcc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Thle_SPEC;
    pub type Thle = crate::EnumBitfieldStruct<u8, Thle_SPEC>;
    impl Thle {
        #[doc = "TX History List disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "TX History List enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Thlie_SPEC;
    pub type Thlie = crate::EnumBitfieldStruct<u8, Thlie_SPEC>;
    impl Thlie {
        #[doc = "TX History List Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "TX History List Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Thlim_SPEC;
    pub type Thlim = crate::EnumBitfieldStruct<u8, Thlim_SPEC>;
    impl Thlim {
        #[doc = "Interrupt generated if TX History List level reaches ¾ of the TX History List depth"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt generated for every successfully stored entry"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Thldte_SPEC;
    pub type Thldte = crate::EnumBitfieldStruct<u8, Thldte_SPEC>;
    impl Thldte {
        #[doc = "TX FIFO + TX Queue"]
        pub const _0: Self = Self::new(0);
        #[doc = "Flat TX MB + TX FIFO + TX Queue"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdthlsts_SPEC;
impl crate::sealed::RegSpec for Cfdthlsts_SPEC {
    type DataType = u32;
}
#[doc = "TX History List Status Register"]
pub type Cfdthlsts = crate::RegValueT<Cfdthlsts_SPEC>;

impl Cfdthlsts {
    #[doc = "TX History List Empty"]
    #[inline(always)]
    pub fn thlemp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdthlsts::Thlemp,
        Cfdthlsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdthlsts::Thlemp,
            Cfdthlsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "TX History List Full"]
    #[inline(always)]
    pub fn thlfll(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdthlsts::Thlfll,
        Cfdthlsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdthlsts::Thlfll,
            Cfdthlsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "TX History List Entry Lost"]
    #[inline(always)]
    pub fn thlelt(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cfdthlsts::Thlelt,
        Cfdthlsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdthlsts::Thlelt,
            Cfdthlsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "TX History List Interrupt Flag"]
    #[inline(always)]
    pub fn thlif(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        cfdthlsts::Thlif,
        Cfdthlsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            cfdthlsts::Thlif,
            Cfdthlsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "TX History List Message Count"]
    #[inline(always)]
    pub fn thlmc(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Cfdthlsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Cfdthlsts_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdthlsts {
    #[inline(always)]
    fn default() -> Cfdthlsts {
        <crate::RegValueT<Cfdthlsts_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod cfdthlsts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Thlemp_SPEC;
    pub type Thlemp = crate::EnumBitfieldStruct<u8, Thlemp_SPEC>;
    impl Thlemp {
        #[doc = "TX History List not empty"]
        pub const _0: Self = Self::new(0);
        #[doc = "TX History List empty"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Thlfll_SPEC;
    pub type Thlfll = crate::EnumBitfieldStruct<u8, Thlfll_SPEC>;
    impl Thlfll {
        #[doc = "TX History List not full"]
        pub const _0: Self = Self::new(0);
        #[doc = "TX History List full"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Thlelt_SPEC;
    pub type Thlelt = crate::EnumBitfieldStruct<u8, Thlelt_SPEC>;
    impl Thlelt {
        #[doc = "No entry lost in TX History List"]
        pub const _0: Self = Self::new(0);
        #[doc = "TX History List entry Lost"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Thlif_SPEC;
    pub type Thlif = crate::EnumBitfieldStruct<u8, Thlif_SPEC>;
    impl Thlif {
        #[doc = "TX History List interrupt condition not satisfied"]
        pub const _0: Self = Self::new(0);
        #[doc = "TX History List interrupt condition satisfied"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdthlpctr_SPEC;
impl crate::sealed::RegSpec for Cfdthlpctr_SPEC {
    type DataType = u32;
}
#[doc = "TX History List Pointer Control Register"]
pub type Cfdthlpctr = crate::RegValueT<Cfdthlpctr_SPEC>;

impl Cfdthlpctr {
    #[doc = "TX History List Pointer Control"]
    #[inline(always)]
    pub fn thlpc(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdthlpctr_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdthlpctr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdthlpctr {
    #[inline(always)]
    fn default() -> Cfdthlpctr {
        <crate::RegValueT<Cfdthlpctr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgtintsts_SPEC;
impl crate::sealed::RegSpec for Cfdgtintsts_SPEC {
    type DataType = u32;
}
#[doc = "Global TX Interrupt Status Register"]
pub type Cfdgtintsts = crate::RegValueT<Cfdgtintsts_SPEC>;

impl Cfdgtintsts {
    #[doc = "TX Successful Interrupt Flag"]
    #[inline(always)]
    pub fn tsif0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdgtintsts::Tsif0,
        Cfdgtintsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdgtintsts::Tsif0,
            Cfdgtintsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "TX Abort Interrupt Flag"]
    #[inline(always)]
    pub fn tai0(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdgtintsts::Tai0,
        Cfdgtintsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdgtintsts::Tai0,
            Cfdgtintsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "TX Queue Interrupt Flag"]
    #[inline(always)]
    pub fn tqif0(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cfdgtintsts::Tqif0,
        Cfdgtintsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdgtintsts::Tqif0,
            Cfdgtintsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "COM FIFO TX Mode Interrupt Flag"]
    #[inline(always)]
    pub fn cftif0(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        cfdgtintsts::Cftif0,
        Cfdgtintsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            cfdgtintsts::Cftif0,
            Cfdgtintsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "TX History List Interrupt"]
    #[inline(always)]
    pub fn thif0(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        cfdgtintsts::Thif0,
        Cfdgtintsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            cfdgtintsts::Thif0,
            Cfdgtintsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdgtintsts {
    #[inline(always)]
    fn default() -> Cfdgtintsts {
        <crate::RegValueT<Cfdgtintsts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdgtintsts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsif0_SPEC;
    pub type Tsif0 = crate::EnumBitfieldStruct<u8, Tsif0_SPEC>;
    impl Tsif0 {
        #[doc = "Channel n TX Successful Interrupt flag not set"]
        pub const _0: Self = Self::new(0);
        #[doc = "Channel n TX Successful Interrupt flag set"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tai0_SPEC;
    pub type Tai0 = crate::EnumBitfieldStruct<u8, Tai0_SPEC>;
    impl Tai0 {
        #[doc = "Channel n TX Abort Interrupt flag not set"]
        pub const _0: Self = Self::new(0);
        #[doc = "Channel n TX Abort Interrupt flag set"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tqif0_SPEC;
    pub type Tqif0 = crate::EnumBitfieldStruct<u8, Tqif0_SPEC>;
    impl Tqif0 {
        #[doc = "Channel n TX Queue Interrupt flag not set"]
        pub const _0: Self = Self::new(0);
        #[doc = "Channel n TX Queue Interrupt flag set"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cftif0_SPEC;
    pub type Cftif0 = crate::EnumBitfieldStruct<u8, Cftif0_SPEC>;
    impl Cftif0 {
        #[doc = "Channel n COM FIFO TX Mode Interrupt flag not set"]
        pub const _0: Self = Self::new(0);
        #[doc = "Channel n COM FIFO TX Mode Interrupt flag set"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Thif0_SPEC;
    pub type Thif0 = crate::EnumBitfieldStruct<u8, Thif0_SPEC>;
    impl Thif0 {
        #[doc = "Channel n TX History List Interrupt flag not set"]
        pub const _0: Self = Self::new(0);
        #[doc = "Channel n TX History List Interrupt flag set"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgtstcfg_SPEC;
impl crate::sealed::RegSpec for Cfdgtstcfg_SPEC {
    type DataType = u32;
}
#[doc = "Global Test Configuration Register"]
pub type Cfdgtstcfg = crate::RegValueT<Cfdgtstcfg_SPEC>;

impl Cfdgtstcfg {
    #[doc = "RAM Test Mode Page Select"]
    #[inline(always)]
    pub fn rtmps(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Cfdgtstcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Cfdgtstcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdgtstcfg {
    #[inline(always)]
    fn default() -> Cfdgtstcfg {
        <crate::RegValueT<Cfdgtstcfg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgtstctr_SPEC;
impl crate::sealed::RegSpec for Cfdgtstctr_SPEC {
    type DataType = u32;
}
#[doc = "Global Test Control Register"]
pub type Cfdgtstctr = crate::RegValueT<Cfdgtstctr_SPEC>;

impl Cfdgtstctr {
    #[doc = "RAM Test Mode Enable"]
    #[inline(always)]
    pub fn rtme(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cfdgtstctr::Rtme,
        Cfdgtstctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdgtstctr::Rtme,
            Cfdgtstctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdgtstctr {
    #[inline(always)]
    fn default() -> Cfdgtstctr {
        <crate::RegValueT<Cfdgtstctr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdgtstctr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtme_SPEC;
    pub type Rtme = crate::EnumBitfieldStruct<u8, Rtme_SPEC>;
    impl Rtme {
        #[doc = "RAM test mode disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "RAM test mode enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgfdcfg_SPEC;
impl crate::sealed::RegSpec for Cfdgfdcfg_SPEC {
    type DataType = u32;
}
#[doc = "Global FD Configuration Register"]
pub type Cfdgfdcfg = crate::RegValueT<Cfdgfdcfg_SPEC>;

impl Cfdgfdcfg {
    #[doc = "RES Bit Protocol Exception Disable"]
    #[inline(always)]
    pub fn rped(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdgfdcfg::Rped,
        Cfdgfdcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdgfdcfg::Rped,
            Cfdgfdcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Timestamp Capture Configuration"]
    #[inline(always)]
    pub fn tsccfg(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        cfdgfdcfg::Tsccfg,
        Cfdgfdcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            cfdgfdcfg::Tsccfg,
            Cfdgfdcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdgfdcfg {
    #[inline(always)]
    fn default() -> Cfdgfdcfg {
        <crate::RegValueT<Cfdgfdcfg_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdgfdcfg {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rped_SPEC;
    pub type Rped = crate::EnumBitfieldStruct<u8, Rped_SPEC>;
    impl Rped {
        #[doc = "Protocol exception event detection enabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Protocol exception event detection disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsccfg_SPEC;
    pub type Tsccfg = crate::EnumBitfieldStruct<u8, Tsccfg_SPEC>;
    impl Tsccfg {
        #[doc = "Timestamp capture at the sample point of SOF (start of frame)"]
        pub const _00: Self = Self::new(0);
        #[doc = "Timestamp capture at frame valid indication"]
        pub const _01: Self = Self::new(1);
        #[doc = "Timestamp capture at the sample point of RES bit"]
        pub const _10: Self = Self::new(2);
        #[doc = "Reserved"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdglockk_SPEC;
impl crate::sealed::RegSpec for Cfdglockk_SPEC {
    type DataType = u32;
}
#[doc = "Global Lock Key Register"]
pub type Cfdglockk = crate::RegValueT<Cfdglockk_SPEC>;

impl Cfdglockk {
    #[doc = "Lock Key"]
    #[inline(always)]
    pub fn lock(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Cfdglockk_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Cfdglockk_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdglockk {
    #[inline(always)]
    fn default() -> Cfdglockk {
        <crate::RegValueT<Cfdglockk_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgaflignent_SPEC;
impl crate::sealed::RegSpec for Cfdgaflignent_SPEC {
    type DataType = u32;
}
#[doc = "Global AFL Ignore Entry Register"]
pub type Cfdgaflignent = crate::RegValueT<Cfdgaflignent_SPEC>;

impl Cfdgaflignent {
    #[doc = "Ignore Rule Number"]
    #[inline(always)]
    pub fn irn(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, Cfdgaflignent_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8, Cfdgaflignent_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdgaflignent {
    #[inline(always)]
    fn default() -> Cfdgaflignent {
        <crate::RegValueT<Cfdgaflignent_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgaflignctr_SPEC;
impl crate::sealed::RegSpec for Cfdgaflignctr_SPEC {
    type DataType = u32;
}
#[doc = "Global AFL Ignore Control Register"]
pub type Cfdgaflignctr = crate::RegValueT<Cfdgaflignctr_SPEC>;

impl Cfdgaflignctr {
    #[doc = "Ignore Rule Enable"]
    #[inline(always)]
    pub fn iren(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdgaflignctr::Iren,
        Cfdgaflignctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdgaflignctr::Iren,
            Cfdgaflignctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdgaflignctr_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdgaflignctr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdgaflignctr {
    #[inline(always)]
    fn default() -> Cfdgaflignctr {
        <crate::RegValueT<Cfdgaflignctr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdgaflignctr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iren_SPEC;
    pub type Iren = crate::EnumBitfieldStruct<u8, Iren_SPEC>;
    impl Iren {
        #[doc = "AFL entry number is not ignored"]
        pub const _0: Self = Self::new(0);
        #[doc = "AFL entry number is ignored"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcdtct_SPEC;
impl crate::sealed::RegSpec for Cfdcdtct_SPEC {
    type DataType = u32;
}
#[doc = "DMA Transfer Control Register"]
pub type Cfdcdtct = crate::RegValueT<Cfdcdtct_SPEC>;

impl Cfdcdtct {
    #[doc = "DMA Transfer Enable for RXFIFO 0"]
    #[inline(always)]
    pub fn rfdmae0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdcdtct::Rfdmae0,
        Cfdcdtct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdcdtct::Rfdmae0,
            Cfdcdtct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "DMA Transfer Enable for RXFIFO 1"]
    #[inline(always)]
    pub fn rfdmae1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdcdtct::Rfdmae1,
        Cfdcdtct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdcdtct::Rfdmae1,
            Cfdcdtct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "DMA Transfer Enable for Common FIFO 0"]
    #[inline(always)]
    pub fn cfdmae(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cfdcdtct::Cfdmae,
        Cfdcdtct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cfdcdtct::Cfdmae,
            Cfdcdtct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcdtct {
    #[inline(always)]
    fn default() -> Cfdcdtct {
        <crate::RegValueT<Cfdcdtct_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcdtct {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdmae0_SPEC;
    pub type Rfdmae0 = crate::EnumBitfieldStruct<u8, Rfdmae0_SPEC>;
    impl Rfdmae0 {
        #[doc = "DMA transfer request disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "DMA transfer request enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdmae1_SPEC;
    pub type Rfdmae1 = crate::EnumBitfieldStruct<u8, Rfdmae1_SPEC>;
    impl Rfdmae1 {
        #[doc = "DMA transfer request disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "DMA transfer request enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdmae_SPEC;
    pub type Cfdmae = crate::EnumBitfieldStruct<u8, Cfdmae_SPEC>;
    impl Cfdmae {
        #[doc = "DMA transfer request disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "DMA transfer request enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcdtsts_SPEC;
impl crate::sealed::RegSpec for Cfdcdtsts_SPEC {
    type DataType = u32;
}
#[doc = "DMA Transfer Status Register"]
pub type Cfdcdtsts = crate::RegValueT<Cfdcdtsts_SPEC>;

impl Cfdcdtsts {
    #[doc = "DMA Transfer Status for RX FIFO 0"]
    #[inline(always)]
    pub fn rfdmasts0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdcdtsts::Rfdmasts0,
        Cfdcdtsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdcdtsts::Rfdmasts0,
            Cfdcdtsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "DMA Transfer Status for RX FIFO 1"]
    #[inline(always)]
    pub fn rfdmasts1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdcdtsts::Rfdmasts1,
        Cfdcdtsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdcdtsts::Rfdmasts1,
            Cfdcdtsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "DMA Transfer Status only for Common FIFO"]
    #[inline(always)]
    pub fn cfdmasts(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cfdcdtsts::Cfdmasts,
        Cfdcdtsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cfdcdtsts::Cfdmasts,
            Cfdcdtsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcdtsts {
    #[inline(always)]
    fn default() -> Cfdcdtsts {
        <crate::RegValueT<Cfdcdtsts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcdtsts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdmasts0_SPEC;
    pub type Rfdmasts0 = crate::EnumBitfieldStruct<u8, Rfdmasts0_SPEC>;
    impl Rfdmasts0 {
        #[doc = "DMA transfer stopped"]
        pub const _0: Self = Self::new(0);
        #[doc = "DMA transfer on going"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdmasts1_SPEC;
    pub type Rfdmasts1 = crate::EnumBitfieldStruct<u8, Rfdmasts1_SPEC>;
    impl Rfdmasts1 {
        #[doc = "DMA transfer stopped"]
        pub const _0: Self = Self::new(0);
        #[doc = "DMA transfer on going"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdmasts_SPEC;
    pub type Cfdmasts = crate::EnumBitfieldStruct<u8, Cfdmasts_SPEC>;
    impl Cfdmasts {
        #[doc = "DMA transfer stopped"]
        pub const _0: Self = Self::new(0);
        #[doc = "DMA transfer on going"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgrstc_SPEC;
impl crate::sealed::RegSpec for Cfdgrstc_SPEC {
    type DataType = u32;
}
#[doc = "Global SW reset Register"]
pub type Cfdgrstc = crate::RegValueT<Cfdgrstc_SPEC>;

impl Cfdgrstc {
    #[doc = "SW Reset"]
    #[inline(always)]
    pub fn srst(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, cfdgrstc::Srst, Cfdgrstc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,cfdgrstc::Srst, Cfdgrstc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdgrstc_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdgrstc_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdgrstc {
    #[inline(always)]
    fn default() -> Cfdgrstc {
        <crate::RegValueT<Cfdgrstc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdgrstc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Srst_SPEC;
    pub type Srst = crate::EnumBitfieldStruct<u8, Srst_SPEC>;
    impl Srst {
        #[doc = "Normal state"]
        pub const _0: Self = Self::new(0);
        #[doc = "SW reset state"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdc0Dcfg_SPEC;
impl crate::sealed::RegSpec for Cfdc0Dcfg_SPEC {
    type DataType = u32;
}
#[doc = "Channel 0 Data Bitrate Configuration Register"]
pub type Cfdc0Dcfg = crate::RegValueT<Cfdc0Dcfg_SPEC>;

impl Cfdc0Dcfg {
    #[doc = "Channel Data Baud Rate Prescaler"]
    #[inline(always)]
    pub fn dbrp(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdc0Dcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdc0Dcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timing Segment 1"]
    #[inline(always)]
    pub fn dtseg1(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, Cfdc0Dcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, Cfdc0Dcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timing Segment 2"]
    #[inline(always)]
    pub fn dtseg2(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Cfdc0Dcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Cfdc0Dcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Resynchronization Jump Width"]
    #[inline(always)]
    pub fn dsjw(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Cfdc0Dcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Cfdc0Dcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdc0Dcfg {
    #[inline(always)]
    fn default() -> Cfdc0Dcfg {
        <crate::RegValueT<Cfdc0Dcfg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdc0Fdcfg_SPEC;
impl crate::sealed::RegSpec for Cfdc0Fdcfg_SPEC {
    type DataType = u32;
}
#[doc = "Channel 0 CANFD Configuration Register"]
pub type Cfdc0Fdcfg = crate::RegValueT<Cfdc0Fdcfg_SPEC>;

impl Cfdc0Fdcfg {
    #[doc = "Error Occurrence Counter Configuration"]
    #[inline(always)]
    pub fn eoccfg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        cfdc0fdcfg::Eoccfg,
        Cfdc0Fdcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            cfdc0fdcfg::Eoccfg,
            Cfdc0Fdcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Transceiver Delay Compensation Offset Configuration"]
    #[inline(always)]
    pub fn tdcoc(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cfdc0fdcfg::Tdcoc,
        Cfdc0Fdcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cfdc0fdcfg::Tdcoc,
            Cfdc0Fdcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Transceiver Delay Compensation Enable"]
    #[inline(always)]
    pub fn tdce(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        cfdc0fdcfg::Tdce,
        Cfdc0Fdcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            cfdc0fdcfg::Tdce,
            Cfdc0Fdcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Error State Indication Configuration"]
    #[inline(always)]
    pub fn esic(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        cfdc0fdcfg::Esic,
        Cfdc0Fdcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            cfdc0fdcfg::Esic,
            Cfdc0Fdcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Transceiver Delay Compensation Offset"]
    #[inline(always)]
    pub fn tdco(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdc0Fdcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdc0Fdcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "FD-Only Enable"]
    #[inline(always)]
    pub fn fdoe(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        cfdc0fdcfg::Fdoe,
        Cfdc0Fdcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            cfdc0fdcfg::Fdoe,
            Cfdc0Fdcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "RX Edge Filter Enable"]
    #[inline(always)]
    pub fn refe(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        cfdc0fdcfg::Refe,
        Cfdc0Fdcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            cfdc0fdcfg::Refe,
            Cfdc0Fdcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Classical CAN Enable"]
    #[inline(always)]
    pub fn cloe(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        cfdc0fdcfg::Cloe,
        Cfdc0Fdcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            cfdc0fdcfg::Cloe,
            Cfdc0Fdcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdc0Fdcfg {
    #[inline(always)]
    fn default() -> Cfdc0Fdcfg {
        <crate::RegValueT<Cfdc0Fdcfg_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdc0fdcfg {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eoccfg_SPEC;
    pub type Eoccfg = crate::EnumBitfieldStruct<u8, Eoccfg_SPEC>;
    impl Eoccfg {
        #[doc = "All transmitter or receiver CAN frames"]
        pub const _000: Self = Self::new(0);
        #[doc = "All transmitter CAN frames"]
        pub const _001: Self = Self::new(1);
        #[doc = "All receiver CAN frames"]
        pub const _010: Self = Self::new(2);
        #[doc = "Reserved"]
        pub const _011: Self = Self::new(3);
        #[doc = "Only transmitter or receiver CANFD data-phase (fast bits)"]
        pub const _100: Self = Self::new(4);
        #[doc = "Only transmitter CANFD data-phase (fast bits)"]
        pub const _101: Self = Self::new(5);
        #[doc = "Only receiver CANFD data-phase (fast bits)"]
        pub const _110: Self = Self::new(6);
        #[doc = "Reserved"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdcoc_SPEC;
    pub type Tdcoc = crate::EnumBitfieldStruct<u8, Tdcoc_SPEC>;
    impl Tdcoc {
        #[doc = "Measured + offset"]
        pub const _0: Self = Self::new(0);
        #[doc = "Offset-only"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdce_SPEC;
    pub type Tdce = crate::EnumBitfieldStruct<u8, Tdce_SPEC>;
    impl Tdce {
        #[doc = "Transceiver delay compensation disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Transceiver delay compensation enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Esic_SPEC;
    pub type Esic = crate::EnumBitfieldStruct<u8, Esic_SPEC>;
    impl Esic {
        #[doc = "The ESI bit in the frame represents the error state of the node itself"]
        pub const _0: Self = Self::new(0);
        #[doc = "The ESI bit in the frame represents the error state of the message buffer if the node itself is not in error passive. If the node is in error passive, then the ESI bit is driven by the node itself."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fdoe_SPEC;
    pub type Fdoe = crate::EnumBitfieldStruct<u8, Fdoe_SPEC>;
    impl Fdoe {
        #[doc = "FD-only mode disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "FD-only mode enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Refe_SPEC;
    pub type Refe = crate::EnumBitfieldStruct<u8, Refe_SPEC>;
    impl Refe {
        #[doc = "RX edge filter disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "RX edge filter enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cloe_SPEC;
    pub type Cloe = crate::EnumBitfieldStruct<u8, Cloe_SPEC>;
    impl Cloe {
        #[doc = "Classical CAN mode disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Classical CAN mode enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdc0Fdctr_SPEC;
impl crate::sealed::RegSpec for Cfdc0Fdctr_SPEC {
    type DataType = u32;
}
#[doc = "Channel 0 CANFD Control Register"]
pub type Cfdc0Fdctr = crate::RegValueT<Cfdc0Fdctr_SPEC>;

impl Cfdc0Fdctr {
    #[doc = "Error Occurrence Counter Clear"]
    #[inline(always)]
    pub fn eocclr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdc0fdctr::Eocclr,
        Cfdc0Fdctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdc0fdctr::Eocclr,
            Cfdc0Fdctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Successful Occurrence Counter Clear"]
    #[inline(always)]
    pub fn socclr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdc0fdctr::Socclr,
        Cfdc0Fdctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdc0fdctr::Socclr,
            Cfdc0Fdctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdc0Fdctr {
    #[inline(always)]
    fn default() -> Cfdc0Fdctr {
        <crate::RegValueT<Cfdc0Fdctr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdc0fdctr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eocclr_SPEC;
    pub type Eocclr = crate::EnumBitfieldStruct<u8, Eocclr_SPEC>;
    impl Eocclr {
        #[doc = "No error occurrence counter clear"]
        pub const _0: Self = Self::new(0);
        #[doc = "Clear error occurrence counter"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Socclr_SPEC;
    pub type Socclr = crate::EnumBitfieldStruct<u8, Socclr_SPEC>;
    impl Socclr {
        #[doc = "No successful occurrence counter clear"]
        pub const _0: Self = Self::new(0);
        #[doc = "Clear successful occurrence counter"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdc0Fdsts_SPEC;
impl crate::sealed::RegSpec for Cfdc0Fdsts_SPEC {
    type DataType = u32;
}
#[doc = "Channel 0 CANFD Status Register"]
pub type Cfdc0Fdsts = crate::RegValueT<Cfdc0Fdsts_SPEC>;

impl Cfdc0Fdsts {
    #[doc = "Transceiver Delay Compensation Result"]
    #[inline(always)]
    pub fn tdcr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdc0Fdsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdc0Fdsts_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Error Occurrence Counter Overflow"]
    #[inline(always)]
    pub fn eoco(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cfdc0fdsts::Eoco,
        Cfdc0Fdsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cfdc0fdsts::Eoco,
            Cfdc0Fdsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Successful Occurrence Counter Overflow"]
    #[inline(always)]
    pub fn soco(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        cfdc0fdsts::Soco,
        Cfdc0Fdsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            cfdc0fdsts::Soco,
            Cfdc0Fdsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Transceiver Delay Compensation Violation Flag"]
    #[inline(always)]
    pub fn tdcvf(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        cfdc0fdsts::Tdcvf,
        Cfdc0Fdsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            cfdc0fdsts::Tdcvf,
            Cfdc0Fdsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Error Occurrence Counter"]
    #[inline(always)]
    pub fn eoc(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdc0Fdsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdc0Fdsts_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Successful occurrence counter"]
    #[inline(always)]
    pub fn soc(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdc0Fdsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdc0Fdsts_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdc0Fdsts {
    #[inline(always)]
    fn default() -> Cfdc0Fdsts {
        <crate::RegValueT<Cfdc0Fdsts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdc0fdsts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eoco_SPEC;
    pub type Eoco = crate::EnumBitfieldStruct<u8, Eoco_SPEC>;
    impl Eoco {
        #[doc = "Error occurrence counter has not overflowed"]
        pub const _0: Self = Self::new(0);
        #[doc = "Error occurrence counter has overflowed"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Soco_SPEC;
    pub type Soco = crate::EnumBitfieldStruct<u8, Soco_SPEC>;
    impl Soco {
        #[doc = "Successful occurrence counter has not overflowed"]
        pub const _0: Self = Self::new(0);
        #[doc = "Successful occurrence counter has overflowed"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdcvf_SPEC;
    pub type Tdcvf = crate::EnumBitfieldStruct<u8, Tdcvf_SPEC>;
    impl Tdcvf {
        #[doc = "Transceiver delay compensation violation has not occurred"]
        pub const _0: Self = Self::new(0);
        #[doc = "Transceiver delay compensation violation has occurred"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdc0Fdcrc_SPEC;
impl crate::sealed::RegSpec for Cfdc0Fdcrc_SPEC {
    type DataType = u32;
}
#[doc = "Channel 0 CANFD CRC Register"]
pub type Cfdc0Fdcrc = crate::RegValueT<Cfdc0Fdcrc_SPEC>;

impl Cfdc0Fdcrc {
    #[doc = "CRC Register value"]
    #[inline(always)]
    pub fn crcreg(
        self,
    ) -> crate::common::RegisterField<0, 0x1fffff, 1, 0, u32, Cfdc0Fdcrc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1fffff,1,0,u32, Cfdc0Fdcrc_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Stuff bit count"]
    #[inline(always)]
    pub fn scnt(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Cfdc0Fdcrc_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Cfdc0Fdcrc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdc0Fdcrc {
    #[inline(always)]
    fn default() -> Cfdc0Fdcrc {
        <crate::RegValueT<Cfdc0Fdcrc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgaflid_SPEC;
impl crate::sealed::RegSpec for Cfdgaflid_SPEC {
    type DataType = u32;
}
#[doc = "Global Acceptance Filter List ID Registers"]
pub type Cfdgaflid = crate::RegValueT<Cfdgaflid_SPEC>;

impl Cfdgaflid {
    #[doc = "Global Acceptance Filter List Entry ID Field"]
    #[inline(always)]
    pub fn gaflid(
        self,
    ) -> crate::common::RegisterField<0, 0x1fffffff, 1, 0, u32, Cfdgaflid_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fffffff,1,0,u32, Cfdgaflid_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Global Acceptance Filter List Entry Loopback Configuration"]
    #[inline(always)]
    pub fn gafllb(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        cfdgaflid::Gafllb,
        Cfdgaflid_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            cfdgaflid::Gafllb,
            Cfdgaflid_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Global Acceptance Filter List Entry RTR Field"]
    #[inline(always)]
    pub fn gaflrtr(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        cfdgaflid::Gaflrtr,
        Cfdgaflid_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            cfdgaflid::Gaflrtr,
            Cfdgaflid_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Global Acceptance Filter List Entry IDE Field"]
    #[inline(always)]
    pub fn gaflide(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        cfdgaflid::Gaflide,
        Cfdgaflid_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            cfdgaflid::Gaflide,
            Cfdgaflid_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdgaflid {
    #[inline(always)]
    fn default() -> Cfdgaflid {
        <crate::RegValueT<Cfdgaflid_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdgaflid {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gafllb_SPEC;
    pub type Gafllb = crate::EnumBitfieldStruct<u8, Gafllb_SPEC>;
    impl Gafllb {
        #[doc = "Global Acceptance Filter List entry ID for acceptance filtering with attribute RX"]
        pub const _0: Self = Self::new(0);
        #[doc = "Global Acceptance Filter List entry ID for acceptance filtering with attribute TX"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gaflrtr_SPEC;
    pub type Gaflrtr = crate::EnumBitfieldStruct<u8, Gaflrtr_SPEC>;
    impl Gaflrtr {
        #[doc = "Data frame"]
        pub const _0: Self = Self::new(0);
        #[doc = "Remote frame"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gaflide_SPEC;
    pub type Gaflide = crate::EnumBitfieldStruct<u8, Gaflide_SPEC>;
    impl Gaflide {
        #[doc = "Standard identifier of rule entry ID is valid for acceptance filtering"]
        pub const _0: Self = Self::new(0);
        #[doc = "Extended identifier of rule entry ID is valid for acceptance filtering"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgaflm_SPEC;
impl crate::sealed::RegSpec for Cfdgaflm_SPEC {
    type DataType = u32;
}
#[doc = "Global Acceptance Filter List Mask Registers"]
pub type Cfdgaflm = crate::RegValueT<Cfdgaflm_SPEC>;

impl Cfdgaflm {
    #[doc = "Global Acceptance Filter List ID Mask Field"]
    #[inline(always)]
    pub fn gaflidm(
        self,
    ) -> crate::common::RegisterField<0, 0x1fffffff, 1, 0, u32, Cfdgaflm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fffffff,1,0,u32, Cfdgaflm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Global Acceptance Filter List Information Label 1"]
    #[inline(always)]
    pub fn gaflifl1(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Cfdgaflm_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,Cfdgaflm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Global Acceptance Filter List Entry RTR Mask"]
    #[inline(always)]
    pub fn gaflrtrm(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        cfdgaflm::Gaflrtrm,
        Cfdgaflm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            cfdgaflm::Gaflrtrm,
            Cfdgaflm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Global Acceptance Filter List IDE Mask"]
    #[inline(always)]
    pub fn gaflidem(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        cfdgaflm::Gaflidem,
        Cfdgaflm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            cfdgaflm::Gaflidem,
            Cfdgaflm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdgaflm {
    #[inline(always)]
    fn default() -> Cfdgaflm {
        <crate::RegValueT<Cfdgaflm_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdgaflm {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gaflrtrm_SPEC;
    pub type Gaflrtrm = crate::EnumBitfieldStruct<u8, Gaflrtrm_SPEC>;
    impl Gaflrtrm {
        #[doc = "RTR bit is not used for ID matching"]
        pub const _0: Self = Self::new(0);
        #[doc = "RTR bit is used for ID matching"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gaflidem_SPEC;
    pub type Gaflidem = crate::EnumBitfieldStruct<u8, Gaflidem_SPEC>;
    impl Gaflidem {
        #[doc = "IDE bit is not used for ID matching"]
        pub const _0: Self = Self::new(0);
        #[doc = "IDE bit is used for ID matching"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgaflp0_SPEC;
impl crate::sealed::RegSpec for Cfdgaflp0_SPEC {
    type DataType = u32;
}
#[doc = "Global Acceptance Filter List Pointer 0 Registers"]
pub type Cfdgaflp0 = crate::RegValueT<Cfdgaflp0_SPEC>;

impl Cfdgaflp0 {
    #[doc = "Global Acceptance Filter List DLC Field"]
    #[inline(always)]
    pub fn gafldlc(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Cfdgaflp0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Cfdgaflp0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Global Acceptance Filter List Information Label 0"]
    #[inline(always)]
    pub fn gaflifl0(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Cfdgaflp0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Cfdgaflp0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Global Acceptance Filter List RX Message Buffer Direction Pointer"]
    #[inline(always)]
    pub fn gaflrmdp(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, Cfdgaflp0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, Cfdgaflp0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Global Acceptance Filter List RX Message Buffer Valid"]
    #[inline(always)]
    pub fn gaflrmv(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        cfdgaflp0::Gaflrmv,
        Cfdgaflp0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            cfdgaflp0::Gaflrmv,
            Cfdgaflp0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Global Acceptance Filter List Pointer"]
    #[inline(always)]
    pub fn gaflptr(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Cfdgaflp0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Cfdgaflp0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdgaflp0 {
    #[inline(always)]
    fn default() -> Cfdgaflp0 {
        <crate::RegValueT<Cfdgaflp0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdgaflp0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gaflrmv_SPEC;
    pub type Gaflrmv = crate::EnumBitfieldStruct<u8, Gaflrmv_SPEC>;
    impl Gaflrmv {
        #[doc = "Single message buffer direction pointer is invalid"]
        pub const _0: Self = Self::new(0);
        #[doc = "Single message buffer direction pointer is valid"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgaflp1_SPEC;
impl crate::sealed::RegSpec for Cfdgaflp1_SPEC {
    type DataType = u32;
}
#[doc = "Global Acceptance Filter List Pointer 1 Registers"]
pub type Cfdgaflp1 = crate::RegValueT<Cfdgaflp1_SPEC>;

impl Cfdgaflp1 {
    #[doc = "Global Acceptance Filter List FIFO Direction Pointer"]
    #[inline(always)]
    pub fn gaflfdp0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdgaflp1::Gaflfdp0,
        Cfdgaflp1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdgaflp1::Gaflfdp0,
            Cfdgaflp1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Global Acceptance Filter List FIFO Direction Pointer"]
    #[inline(always)]
    pub fn gaflfdp1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdgaflp1::Gaflfdp1,
        Cfdgaflp1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdgaflp1::Gaflfdp1,
            Cfdgaflp1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Global Acceptance Filter List FIFO Direction Pointer"]
    #[inline(always)]
    pub fn gaflfdp8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cfdgaflp1::Gaflfdp8,
        Cfdgaflp1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cfdgaflp1::Gaflfdp8,
            Cfdgaflp1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdgaflp1 {
    #[inline(always)]
    fn default() -> Cfdgaflp1 {
        <crate::RegValueT<Cfdgaflp1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdgaflp1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gaflfdp0_SPEC;
    pub type Gaflfdp0 = crate::EnumBitfieldStruct<u8, Gaflfdp0_SPEC>;
    impl Gaflfdp0 {
        #[doc = "Disable RX FIFO 0 as target for reception"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable RX FIFO 0 as target for reception"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gaflfdp1_SPEC;
    pub type Gaflfdp1 = crate::EnumBitfieldStruct<u8, Gaflfdp1_SPEC>;
    impl Gaflfdp1 {
        #[doc = "Disable RX FIFO 1 as target for reception"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable RX FIFO 1 as target for reception"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gaflfdp8_SPEC;
    pub type Gaflfdp8 = crate::EnumBitfieldStruct<u8, Gaflfdp8_SPEC>;
    impl Gaflfdp8 {
        #[doc = "Disable Common FIFO as target for reception"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable Common FIFO as target for reception"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrpgacc_SPEC;
impl crate::sealed::RegSpec for Cfdrpgacc_SPEC {
    type DataType = u32;
}
#[doc = "RAM Test Page Access Registers %s"]
pub type Cfdrpgacc = crate::RegValueT<Cfdrpgacc_SPEC>;

impl Cfdrpgacc {
    #[doc = "RAM Data Test Access"]
    #[inline(always)]
    pub fn rdta(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Cfdrpgacc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Cfdrpgacc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrpgacc {
    #[inline(always)]
    fn default() -> Cfdrpgacc {
        <crate::RegValueT<Cfdrpgacc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfid_SPEC;
impl crate::sealed::RegSpec for Cfdrfid_SPEC {
    type DataType = u32;
}
#[doc = "RX FIFO Access ID Register %s"]
pub type Cfdrfid = crate::RegValueT<Cfdrfid_SPEC>;

impl Cfdrfid {
    #[doc = "RX FIFO Buffer ID Field"]
    #[inline(always)]
    pub fn rfid(
        self,
    ) -> crate::common::RegisterField<0, 0x1fffffff, 1, 0, u32, Cfdrfid_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1fffffff,1,0,u32, Cfdrfid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer RTR bit"]
    #[inline(always)]
    pub fn rfrtr(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, cfdrfid::Rfrtr, Cfdrfid_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<30,0x1,1,0,cfdrfid::Rfrtr, Cfdrfid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer IDE bit"]
    #[inline(always)]
    pub fn rfide(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, cfdrfid::Rfide, Cfdrfid_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<31,0x1,1,0,cfdrfid::Rfide, Cfdrfid_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrfid {
    #[inline(always)]
    fn default() -> Cfdrfid {
        <crate::RegValueT<Cfdrfid_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrfid {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfrtr_SPEC;
    pub type Rfrtr = crate::EnumBitfieldStruct<u8, Rfrtr_SPEC>;
    impl Rfrtr {
        #[doc = "Data frame"]
        pub const _0: Self = Self::new(0);
        #[doc = "Remote frame"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfide_SPEC;
    pub type Rfide = crate::EnumBitfieldStruct<u8, Rfide_SPEC>;
    impl Rfide {
        #[doc = "STD-ID has been received"]
        pub const _0: Self = Self::new(0);
        #[doc = "EXT-ID has been received"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfptr_SPEC;
impl crate::sealed::RegSpec for Cfdrfptr_SPEC {
    type DataType = u32;
}
#[doc = "RX FIFO Access Pointer Register %s"]
pub type Cfdrfptr = crate::RegValueT<Cfdrfptr_SPEC>;

impl Cfdrfptr {
    #[doc = "RX FIFO Timestamp Value"]
    #[inline(always)]
    pub fn rfts(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Cfdrfptr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Cfdrfptr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer DLC Field"]
    #[inline(always)]
    pub fn rfdlc(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, Cfdrfptr_SPEC, crate::common::R> {
        crate::common::RegisterField::<28,0xf,1,0,u8, Cfdrfptr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrfptr {
    #[inline(always)]
    fn default() -> Cfdrfptr {
        <crate::RegValueT<Cfdrfptr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrffdsts_SPEC;
impl crate::sealed::RegSpec for Cfdrffdsts_SPEC {
    type DataType = u32;
}
#[doc = "RX FIFO Access CANFD Status Register %s"]
pub type Cfdrffdsts = crate::RegValueT<Cfdrffdsts_SPEC>;

impl Cfdrffdsts {
    #[doc = "Error State Indicator bit"]
    #[inline(always)]
    pub fn rfesi(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdrffdsts::Rfesi,
        Cfdrffdsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdrffdsts::Rfesi,
            Cfdrffdsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Bit Rate Switch bit"]
    #[inline(always)]
    pub fn rfbrs(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdrffdsts::Rfbrs,
        Cfdrffdsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdrffdsts::Rfbrs,
            Cfdrffdsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CAN FD Format bit"]
    #[inline(always)]
    pub fn rffdf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cfdrffdsts::Rffdf,
        Cfdrffdsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdrffdsts::Rffdf,
            Cfdrffdsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "RX FIFO Buffer Information Label Field"]
    #[inline(always)]
    pub fn rfifl(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Cfdrffdsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x3,1,0,u8, Cfdrffdsts_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Pointer Field"]
    #[inline(always)]
    pub fn cfdrfptr(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Cfdrffdsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Cfdrffdsts_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrffdsts {
    #[inline(always)]
    fn default() -> Cfdrffdsts {
        <crate::RegValueT<Cfdrffdsts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrffdsts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfesi_SPEC;
    pub type Rfesi = crate::EnumBitfieldStruct<u8, Rfesi_SPEC>;
    impl Rfesi {
        #[doc = "CANFD frame received from error active node"]
        pub const _0: Self = Self::new(0);
        #[doc = "CANFD frame received from error passive node"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfbrs_SPEC;
    pub type Rfbrs = crate::EnumBitfieldStruct<u8, Rfbrs_SPEC>;
    impl Rfbrs {
        #[doc = "CANFD frame received with no bit rate switch"]
        pub const _0: Self = Self::new(0);
        #[doc = "CANFD frame received with bit rate switch"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rffdf_SPEC;
    pub type Rffdf = crate::EnumBitfieldStruct<u8, Rffdf_SPEC>;
    impl Rffdf {
        #[doc = "Non CANFD frame received"]
        pub const _0: Self = Self::new(0);
        #[doc = "CANFD frame received"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfdf0_SPEC;
impl crate::sealed::RegSpec for Cfdrfdf0_SPEC {
    type DataType = u32;
}
#[doc = "RX FIFO Access Data Field 0 Register %s"]
pub type Cfdrfdf0 = crate::RegValueT<Cfdrfdf0_SPEC>;

impl Cfdrfdf0 {
    #[doc = "RX FIFO Buffer Data Byte (p × 4)"]
    #[inline(always)]
    pub fn rfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdrfdf0_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdrfdf0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn rfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdrfdf0_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdrfdf0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn rfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdrfdf0_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdrfdf0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn rfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdrfdf0_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdrfdf0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrfdf0 {
    #[inline(always)]
    fn default() -> Cfdrfdf0 {
        <crate::RegValueT<Cfdrfdf0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfdf1_SPEC;
impl crate::sealed::RegSpec for Cfdrfdf1_SPEC {
    type DataType = u32;
}
#[doc = "RX FIFO Access Data Field 1 Register %s"]
pub type Cfdrfdf1 = crate::RegValueT<Cfdrfdf1_SPEC>;

impl Cfdrfdf1 {
    #[doc = "RX FIFO Buffer Data Byte (p × 4)"]
    #[inline(always)]
    pub fn rfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdrfdf1_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdrfdf1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn rfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdrfdf1_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdrfdf1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn rfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdrfdf1_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdrfdf1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn rfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdrfdf1_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdrfdf1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrfdf1 {
    #[inline(always)]
    fn default() -> Cfdrfdf1 {
        <crate::RegValueT<Cfdrfdf1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfdf2_SPEC;
impl crate::sealed::RegSpec for Cfdrfdf2_SPEC {
    type DataType = u32;
}
#[doc = "RX FIFO Access Data Field 2 Register %s"]
pub type Cfdrfdf2 = crate::RegValueT<Cfdrfdf2_SPEC>;

impl Cfdrfdf2 {
    #[doc = "RX FIFO Buffer Data Byte (p × 4)"]
    #[inline(always)]
    pub fn rfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdrfdf2_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdrfdf2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn rfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdrfdf2_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdrfdf2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn rfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdrfdf2_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdrfdf2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn rfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdrfdf2_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdrfdf2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrfdf2 {
    #[inline(always)]
    fn default() -> Cfdrfdf2 {
        <crate::RegValueT<Cfdrfdf2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfdf3_SPEC;
impl crate::sealed::RegSpec for Cfdrfdf3_SPEC {
    type DataType = u32;
}
#[doc = "RX FIFO Access Data Field 3 Register %s"]
pub type Cfdrfdf3 = crate::RegValueT<Cfdrfdf3_SPEC>;

impl Cfdrfdf3 {
    #[doc = "RX FIFO Buffer Data Byte (p × 4)"]
    #[inline(always)]
    pub fn rfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdrfdf3_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdrfdf3_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn rfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdrfdf3_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdrfdf3_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn rfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdrfdf3_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdrfdf3_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn rfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdrfdf3_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdrfdf3_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrfdf3 {
    #[inline(always)]
    fn default() -> Cfdrfdf3 {
        <crate::RegValueT<Cfdrfdf3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfdf4_SPEC;
impl crate::sealed::RegSpec for Cfdrfdf4_SPEC {
    type DataType = u32;
}
#[doc = "RX FIFO Access Data Field 4 Register %s"]
pub type Cfdrfdf4 = crate::RegValueT<Cfdrfdf4_SPEC>;

impl Cfdrfdf4 {
    #[doc = "RX FIFO Buffer Data Byte (p × 4)"]
    #[inline(always)]
    pub fn rfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdrfdf4_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdrfdf4_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn rfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdrfdf4_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdrfdf4_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn rfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdrfdf4_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdrfdf4_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn rfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdrfdf4_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdrfdf4_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrfdf4 {
    #[inline(always)]
    fn default() -> Cfdrfdf4 {
        <crate::RegValueT<Cfdrfdf4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfdf5_SPEC;
impl crate::sealed::RegSpec for Cfdrfdf5_SPEC {
    type DataType = u32;
}
#[doc = "RX FIFO Access Data Field 5 Register %s"]
pub type Cfdrfdf5 = crate::RegValueT<Cfdrfdf5_SPEC>;

impl Cfdrfdf5 {
    #[doc = "RX FIFO Buffer Data Byte (p × 4)"]
    #[inline(always)]
    pub fn rfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdrfdf5_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdrfdf5_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn rfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdrfdf5_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdrfdf5_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn rfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdrfdf5_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdrfdf5_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn rfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdrfdf5_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdrfdf5_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrfdf5 {
    #[inline(always)]
    fn default() -> Cfdrfdf5 {
        <crate::RegValueT<Cfdrfdf5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfdf6_SPEC;
impl crate::sealed::RegSpec for Cfdrfdf6_SPEC {
    type DataType = u32;
}
#[doc = "RX FIFO Access Data Field 6 Register %s"]
pub type Cfdrfdf6 = crate::RegValueT<Cfdrfdf6_SPEC>;

impl Cfdrfdf6 {
    #[doc = "RX FIFO Buffer Data Byte (p × 4)"]
    #[inline(always)]
    pub fn rfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdrfdf6_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdrfdf6_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn rfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdrfdf6_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdrfdf6_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn rfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdrfdf6_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdrfdf6_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn rfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdrfdf6_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdrfdf6_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrfdf6 {
    #[inline(always)]
    fn default() -> Cfdrfdf6 {
        <crate::RegValueT<Cfdrfdf6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfdf7_SPEC;
impl crate::sealed::RegSpec for Cfdrfdf7_SPEC {
    type DataType = u32;
}
#[doc = "RX FIFO Access Data Field 7 Register %s"]
pub type Cfdrfdf7 = crate::RegValueT<Cfdrfdf7_SPEC>;

impl Cfdrfdf7 {
    #[doc = "RX FIFO Buffer Data Byte (p × 4)"]
    #[inline(always)]
    pub fn rfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdrfdf7_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdrfdf7_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn rfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdrfdf7_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdrfdf7_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn rfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdrfdf7_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdrfdf7_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn rfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdrfdf7_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdrfdf7_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrfdf7 {
    #[inline(always)]
    fn default() -> Cfdrfdf7 {
        <crate::RegValueT<Cfdrfdf7_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfdf8_SPEC;
impl crate::sealed::RegSpec for Cfdrfdf8_SPEC {
    type DataType = u32;
}
#[doc = "RX FIFO Access Data Field 8 Register %s"]
pub type Cfdrfdf8 = crate::RegValueT<Cfdrfdf8_SPEC>;

impl Cfdrfdf8 {
    #[doc = "RX FIFO Buffer Data Byte (p × 4)"]
    #[inline(always)]
    pub fn rfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdrfdf8_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdrfdf8_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn rfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdrfdf8_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdrfdf8_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn rfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdrfdf8_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdrfdf8_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn rfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdrfdf8_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdrfdf8_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrfdf8 {
    #[inline(always)]
    fn default() -> Cfdrfdf8 {
        <crate::RegValueT<Cfdrfdf8_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfdf9_SPEC;
impl crate::sealed::RegSpec for Cfdrfdf9_SPEC {
    type DataType = u32;
}
#[doc = "RX FIFO Access Data Field 9 Register %s"]
pub type Cfdrfdf9 = crate::RegValueT<Cfdrfdf9_SPEC>;

impl Cfdrfdf9 {
    #[doc = "RX FIFO Buffer Data Byte (p × 4)"]
    #[inline(always)]
    pub fn rfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdrfdf9_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdrfdf9_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn rfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdrfdf9_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdrfdf9_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn rfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdrfdf9_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdrfdf9_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn rfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdrfdf9_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdrfdf9_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrfdf9 {
    #[inline(always)]
    fn default() -> Cfdrfdf9 {
        <crate::RegValueT<Cfdrfdf9_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfdf10_SPEC;
impl crate::sealed::RegSpec for Cfdrfdf10_SPEC {
    type DataType = u32;
}
#[doc = "RX FIFO Access Data Field 10 Register %s"]
pub type Cfdrfdf10 = crate::RegValueT<Cfdrfdf10_SPEC>;

impl Cfdrfdf10 {
    #[doc = "RX FIFO Buffer Data Byte (p × 4)"]
    #[inline(always)]
    pub fn rfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdrfdf10_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdrfdf10_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn rfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdrfdf10_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdrfdf10_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn rfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdrfdf10_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdrfdf10_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn rfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdrfdf10_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdrfdf10_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrfdf10 {
    #[inline(always)]
    fn default() -> Cfdrfdf10 {
        <crate::RegValueT<Cfdrfdf10_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfdf11_SPEC;
impl crate::sealed::RegSpec for Cfdrfdf11_SPEC {
    type DataType = u32;
}
#[doc = "RX FIFO Access Data Field 11 Register %s"]
pub type Cfdrfdf11 = crate::RegValueT<Cfdrfdf11_SPEC>;

impl Cfdrfdf11 {
    #[doc = "RX FIFO Buffer Data Byte (p × 4)"]
    #[inline(always)]
    pub fn rfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdrfdf11_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdrfdf11_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn rfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdrfdf11_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdrfdf11_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn rfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdrfdf11_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdrfdf11_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn rfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdrfdf11_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdrfdf11_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrfdf11 {
    #[inline(always)]
    fn default() -> Cfdrfdf11 {
        <crate::RegValueT<Cfdrfdf11_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfdf12_SPEC;
impl crate::sealed::RegSpec for Cfdrfdf12_SPEC {
    type DataType = u32;
}
#[doc = "RX FIFO Access Data Field 12 Register %s"]
pub type Cfdrfdf12 = crate::RegValueT<Cfdrfdf12_SPEC>;

impl Cfdrfdf12 {
    #[doc = "RX FIFO Buffer Data Byte (p × 4)"]
    #[inline(always)]
    pub fn rfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdrfdf12_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdrfdf12_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn rfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdrfdf12_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdrfdf12_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn rfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdrfdf12_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdrfdf12_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn rfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdrfdf12_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdrfdf12_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrfdf12 {
    #[inline(always)]
    fn default() -> Cfdrfdf12 {
        <crate::RegValueT<Cfdrfdf12_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfdf13_SPEC;
impl crate::sealed::RegSpec for Cfdrfdf13_SPEC {
    type DataType = u32;
}
#[doc = "RX FIFO Access Data Field 13 Register %s"]
pub type Cfdrfdf13 = crate::RegValueT<Cfdrfdf13_SPEC>;

impl Cfdrfdf13 {
    #[doc = "RX FIFO Buffer Data Byte (p × 4)"]
    #[inline(always)]
    pub fn rfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdrfdf13_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdrfdf13_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn rfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdrfdf13_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdrfdf13_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn rfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdrfdf13_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdrfdf13_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn rfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdrfdf13_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdrfdf13_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrfdf13 {
    #[inline(always)]
    fn default() -> Cfdrfdf13 {
        <crate::RegValueT<Cfdrfdf13_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfdf14_SPEC;
impl crate::sealed::RegSpec for Cfdrfdf14_SPEC {
    type DataType = u32;
}
#[doc = "RX FIFO Access Data Field 14 Register %s"]
pub type Cfdrfdf14 = crate::RegValueT<Cfdrfdf14_SPEC>;

impl Cfdrfdf14 {
    #[doc = "RX FIFO Buffer Data Byte (p × 4)"]
    #[inline(always)]
    pub fn rfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdrfdf14_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdrfdf14_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn rfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdrfdf14_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdrfdf14_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn rfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdrfdf14_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdrfdf14_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn rfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdrfdf14_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdrfdf14_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrfdf14 {
    #[inline(always)]
    fn default() -> Cfdrfdf14 {
        <crate::RegValueT<Cfdrfdf14_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfdf15_SPEC;
impl crate::sealed::RegSpec for Cfdrfdf15_SPEC {
    type DataType = u32;
}
#[doc = "RX FIFO Access Data Field 15 Register %s"]
pub type Cfdrfdf15 = crate::RegValueT<Cfdrfdf15_SPEC>;

impl Cfdrfdf15 {
    #[doc = "RX FIFO Buffer Data Byte (p × 4)"]
    #[inline(always)]
    pub fn rfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdrfdf15_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdrfdf15_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn rfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdrfdf15_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdrfdf15_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn rfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdrfdf15_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdrfdf15_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX FIFO Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn rfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdrfdf15_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdrfdf15_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrfdf15 {
    #[inline(always)]
    fn default() -> Cfdrfdf15 {
        <crate::RegValueT<Cfdrfdf15_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfid_SPEC;
impl crate::sealed::RegSpec for Cfdcfid_SPEC {
    type DataType = u32;
}
#[doc = "Common FIFO Access ID Register"]
pub type Cfdcfid = crate::RegValueT<Cfdcfid_SPEC>;

impl Cfdcfid {
    #[doc = "Common FIFO Buffer ID Field"]
    #[inline(always)]
    pub fn cfid(
        self,
    ) -> crate::common::RegisterField<0, 0x1fffffff, 1, 0, u32, Cfdcfid_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fffffff,1,0,u32, Cfdcfid_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "THL Entry enable"]
    #[inline(always)]
    pub fn thlen(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, cfdcfid::Thlen, Cfdcfid_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,cfdcfid::Thlen, Cfdcfid_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Common FIFO Buffer RTR Bit"]
    #[inline(always)]
    pub fn cfrtr(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, cfdcfid::Cfrtr, Cfdcfid_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,cfdcfid::Cfrtr, Cfdcfid_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Common FIFO Buffer IDE Bit"]
    #[inline(always)]
    pub fn cfide(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, cfdcfid::Cfide, Cfdcfid_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,cfdcfid::Cfide, Cfdcfid_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfid {
    #[inline(always)]
    fn default() -> Cfdcfid {
        <crate::RegValueT<Cfdcfid_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcfid {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Thlen_SPEC;
    pub type Thlen = crate::EnumBitfieldStruct<u8, Thlen_SPEC>;
    impl Thlen {
        #[doc = "Entry will not be stored in THL after successful TX."]
        pub const _0: Self = Self::new(0);
        #[doc = "Entry will be stored in THL after successful TX."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfrtr_SPEC;
    pub type Cfrtr = crate::EnumBitfieldStruct<u8, Cfrtr_SPEC>;
    impl Cfrtr {
        #[doc = "Data Frame"]
        pub const _0: Self = Self::new(0);
        #[doc = "Remote Frame"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfide_SPEC;
    pub type Cfide = crate::EnumBitfieldStruct<u8, Cfide_SPEC>;
    impl Cfide {
        #[doc = "STD-ID will be transmitted or has been received"]
        pub const _0: Self = Self::new(0);
        #[doc = "EXT-ID will be transmitted or has been received"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfptr_SPEC;
impl crate::sealed::RegSpec for Cfdcfptr_SPEC {
    type DataType = u32;
}
#[doc = "Common FIFO Access Pointer Register"]
pub type Cfdcfptr = crate::RegValueT<Cfdcfptr_SPEC>;

impl Cfdcfptr {
    #[doc = "Common FIFO Timestamp Value"]
    #[inline(always)]
    pub fn cfts(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Cfdcfptr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Cfdcfptr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Common FIFO Buffer DLC Field"]
    #[inline(always)]
    pub fn cfdlc(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, Cfdcfptr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0xf,1,0,u8, Cfdcfptr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfptr {
    #[inline(always)]
    fn default() -> Cfdcfptr {
        <crate::RegValueT<Cfdcfptr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcffdcsts_SPEC;
impl crate::sealed::RegSpec for Cfdcffdcsts_SPEC {
    type DataType = u32;
}
#[doc = "Common FIFO Access CANFD Control/Status Register"]
pub type Cfdcffdcsts = crate::RegValueT<Cfdcffdcsts_SPEC>;

impl Cfdcffdcsts {
    #[doc = "Error State Indicator bit"]
    #[inline(always)]
    pub fn cfesi(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdcffdcsts::Cfesi,
        Cfdcffdcsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdcffdcsts::Cfesi,
            Cfdcffdcsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Bit Rate Switch bit"]
    #[inline(always)]
    pub fn cfbrs(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdcffdcsts::Cfbrs,
        Cfdcffdcsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdcffdcsts::Cfbrs,
            Cfdcffdcsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CAN FD Format bit"]
    #[inline(always)]
    pub fn cffdf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cfdcffdcsts::Cffdf,
        Cfdcffdcsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdcffdcsts::Cffdf,
            Cfdcffdcsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "COMMON FIFO Buffer Information Label Field"]
    #[inline(always)]
    pub fn cfifl(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Cfdcffdcsts_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, Cfdcffdcsts_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Common FIFO Buffer Pointer Field"]
    #[inline(always)]
    pub fn cfptr(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Cfdcffdcsts_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Cfdcffdcsts_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcffdcsts {
    #[inline(always)]
    fn default() -> Cfdcffdcsts {
        <crate::RegValueT<Cfdcffdcsts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcffdcsts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfesi_SPEC;
    pub type Cfesi = crate::EnumBitfieldStruct<u8, Cfesi_SPEC>;
    impl Cfesi {
        #[doc = "CANFD frame received or to transmit by error active node"]
        pub const _0: Self = Self::new(0);
        #[doc = "CANFD frame received or to transmit by error passive node"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfbrs_SPEC;
    pub type Cfbrs = crate::EnumBitfieldStruct<u8, Cfbrs_SPEC>;
    impl Cfbrs {
        #[doc = "CANFD frame received or to transmit with no bit rate switch"]
        pub const _0: Self = Self::new(0);
        #[doc = "CANFD frame received or to transmit with bit rate switch"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cffdf_SPEC;
    pub type Cffdf = crate::EnumBitfieldStruct<u8, Cffdf_SPEC>;
    impl Cffdf {
        #[doc = "Non CANFD frame received or to transmit"]
        pub const _0: Self = Self::new(0);
        #[doc = "CANFD frame received or to transmit"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf_SPEC {
    type DataType = u32;
}
#[doc = "Common FIFO Access Data Field %s Registers"]
pub type Cfdcfdf = crate::RegValueT<Cfdcfdf_SPEC>;

impl Cfdcfdf {
    #[doc = "Common FIFO Buffer Data Bytes (p × 4)"]
    #[inline(always)]
    pub fn cfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdcfdf_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdcfdf_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Common FIFO Buffer Data Bytes ((p × 4) + 1)"]
    #[inline(always)]
    pub fn cfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdcfdf_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdcfdf_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Common FIFO Buffer Data Bytes ((p × 4) + 2)"]
    #[inline(always)]
    pub fn cfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdcfdf_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdcfdf_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Common FIFO Buffer Data Bytes ((p × 4) + 3)"]
    #[inline(always)]
    pub fn cfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdcfdf_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdcfdf_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdf {
    #[inline(always)]
    fn default() -> Cfdcfdf {
        <crate::RegValueT<Cfdcfdf_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmid_SPEC;
impl crate::sealed::RegSpec for Cfdtmid_SPEC {
    type DataType = u32;
}
#[doc = "TX Message Buffer ID Registers"]
pub type Cfdtmid = crate::RegValueT<Cfdtmid_SPEC>;

impl Cfdtmid {
    #[doc = "TX Message Buffer ID Field"]
    #[inline(always)]
    pub fn tmid(
        self,
    ) -> crate::common::RegisterField<0, 0x1fffffff, 1, 0, u32, Cfdtmid_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fffffff,1,0,u32, Cfdtmid_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Tx History List Entry"]
    #[inline(always)]
    pub fn thlen(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, cfdtmid::Thlen, Cfdtmid_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,cfdtmid::Thlen, Cfdtmid_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer RTR bit"]
    #[inline(always)]
    pub fn tmrtr(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, cfdtmid::Tmrtr, Cfdtmid_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,cfdtmid::Tmrtr, Cfdtmid_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer IDE bit"]
    #[inline(always)]
    pub fn tmide(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, cfdtmid::Tmide, Cfdtmid_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,cfdtmid::Tmide, Cfdtmid_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmid {
    #[inline(always)]
    fn default() -> Cfdtmid {
        <crate::RegValueT<Cfdtmid_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdtmid {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Thlen_SPEC;
    pub type Thlen = crate::EnumBitfieldStruct<u8, Thlen_SPEC>;
    impl Thlen {
        #[doc = "Entry not stored in THL after successful TX"]
        pub const _0: Self = Self::new(0);
        #[doc = "Entry stored in THL after successful TX"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmrtr_SPEC;
    pub type Tmrtr = crate::EnumBitfieldStruct<u8, Tmrtr_SPEC>;
    impl Tmrtr {
        #[doc = "Data frame"]
        pub const _0: Self = Self::new(0);
        #[doc = "Remote frame"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmide_SPEC;
    pub type Tmide = crate::EnumBitfieldStruct<u8, Tmide_SPEC>;
    impl Tmide {
        #[doc = "STD-ID is transmitted"]
        pub const _0: Self = Self::new(0);
        #[doc = "EXT-ID is transmitted"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmptr_SPEC;
impl crate::sealed::RegSpec for Cfdtmptr_SPEC {
    type DataType = u32;
}
#[doc = "TX Message Buffer Pointer Register"]
pub type Cfdtmptr = crate::RegValueT<Cfdtmptr_SPEC>;

impl Cfdtmptr {
    #[doc = "TX Message Buffer DLC Field"]
    #[inline(always)]
    pub fn tmdlc(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, Cfdtmptr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0xf,1,0,u8, Cfdtmptr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmptr {
    #[inline(always)]
    fn default() -> Cfdtmptr {
        <crate::RegValueT<Cfdtmptr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmfdctr_SPEC;
impl crate::sealed::RegSpec for Cfdtmfdctr_SPEC {
    type DataType = u32;
}
#[doc = "TX Message Buffer CANFD Control Register"]
pub type Cfdtmfdctr = crate::RegValueT<Cfdtmfdctr_SPEC>;

impl Cfdtmfdctr {
    #[doc = "Error State Indicator bit"]
    #[inline(always)]
    pub fn tmesi(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdtmfdctr::Tmesi,
        Cfdtmfdctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdtmfdctr::Tmesi,
            Cfdtmfdctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Bit Rate Switch bit"]
    #[inline(always)]
    pub fn tmbrs(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdtmfdctr::Tmbrs,
        Cfdtmfdctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdtmfdctr::Tmbrs,
            Cfdtmfdctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CAN FD Format bit"]
    #[inline(always)]
    pub fn tmfdf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cfdtmfdctr::Tmfdf,
        Cfdtmfdctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdtmfdctr::Tmfdf,
            Cfdtmfdctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "TX Message Buffer Information Label Field"]
    #[inline(always)]
    pub fn tmifl(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Cfdtmfdctr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, Cfdtmfdctr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Pointer Field"]
    #[inline(always)]
    pub fn tmptr(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Cfdtmfdctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Cfdtmfdctr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmfdctr {
    #[inline(always)]
    fn default() -> Cfdtmfdctr {
        <crate::RegValueT<Cfdtmfdctr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdtmfdctr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmesi_SPEC;
    pub type Tmesi = crate::EnumBitfieldStruct<u8, Tmesi_SPEC>;
    impl Tmesi {
        #[doc = "CANFD frame to transmit by error active node"]
        pub const _0: Self = Self::new(0);
        #[doc = "CANFD frame to transmit by error passive node"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmbrs_SPEC;
    pub type Tmbrs = crate::EnumBitfieldStruct<u8, Tmbrs_SPEC>;
    impl Tmbrs {
        #[doc = "CANFD frame to transmit with no bit rate switch"]
        pub const _0: Self = Self::new(0);
        #[doc = "CANFD frame to transmit with bit rate switch"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmfdf_SPEC;
    pub type Tmfdf = crate::EnumBitfieldStruct<u8, Tmfdf_SPEC>;
    impl Tmfdf {
        #[doc = "Non CANFD frame to transmit"]
        pub const _0: Self = Self::new(0);
        #[doc = "CANFD frame to transmit"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf0_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf0_SPEC {
    type DataType = u32;
}
#[doc = "TX Message Buffer Data Field Register"]
pub type Cfdtmdf0 = crate::RegValueT<Cfdtmdf0_SPEC>;

impl Cfdtmdf0 {
    #[doc = "TX Message Buffer Data Byte ((p × 4)"]
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdtmdf0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdtmdf0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdtmdf0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdtmdf0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdtmdf0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdtmdf0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdtmdf0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdtmdf0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf0 {
    #[inline(always)]
    fn default() -> Cfdtmdf0 {
        <crate::RegValueT<Cfdtmdf0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf1_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf1_SPEC {
    type DataType = u32;
}
#[doc = "TX Message Buffer Data Field Register"]
pub type Cfdtmdf1 = crate::RegValueT<Cfdtmdf1_SPEC>;

impl Cfdtmdf1 {
    #[doc = "TX Message Buffer Data Byte ((p × 4)"]
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdtmdf1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdtmdf1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdtmdf1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdtmdf1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdtmdf1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdtmdf1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdtmdf1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdtmdf1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf1 {
    #[inline(always)]
    fn default() -> Cfdtmdf1 {
        <crate::RegValueT<Cfdtmdf1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf2_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf2_SPEC {
    type DataType = u32;
}
#[doc = "TX Message Buffer Data Field Register"]
pub type Cfdtmdf2 = crate::RegValueT<Cfdtmdf2_SPEC>;

impl Cfdtmdf2 {
    #[doc = "TX Message Buffer Data Byte ((p × 4)"]
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdtmdf2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdtmdf2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdtmdf2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdtmdf2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdtmdf2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdtmdf2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdtmdf2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdtmdf2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf2 {
    #[inline(always)]
    fn default() -> Cfdtmdf2 {
        <crate::RegValueT<Cfdtmdf2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf3_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf3_SPEC {
    type DataType = u32;
}
#[doc = "TX Message Buffer Data Field Register"]
pub type Cfdtmdf3 = crate::RegValueT<Cfdtmdf3_SPEC>;

impl Cfdtmdf3 {
    #[doc = "TX Message Buffer Data Byte ((p × 4)"]
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdtmdf3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdtmdf3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdtmdf3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdtmdf3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdtmdf3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdtmdf3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdtmdf3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdtmdf3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf3 {
    #[inline(always)]
    fn default() -> Cfdtmdf3 {
        <crate::RegValueT<Cfdtmdf3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf4_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf4_SPEC {
    type DataType = u32;
}
#[doc = "TX Message Buffer Data Field Register"]
pub type Cfdtmdf4 = crate::RegValueT<Cfdtmdf4_SPEC>;

impl Cfdtmdf4 {
    #[doc = "TX Message Buffer Data Byte ((p × 4)"]
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdtmdf4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdtmdf4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdtmdf4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdtmdf4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdtmdf4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdtmdf4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdtmdf4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdtmdf4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf4 {
    #[inline(always)]
    fn default() -> Cfdtmdf4 {
        <crate::RegValueT<Cfdtmdf4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf5_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf5_SPEC {
    type DataType = u32;
}
#[doc = "TX Message Buffer Data Field Register"]
pub type Cfdtmdf5 = crate::RegValueT<Cfdtmdf5_SPEC>;

impl Cfdtmdf5 {
    #[doc = "TX Message Buffer Data Byte ((p × 4)"]
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdtmdf5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdtmdf5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdtmdf5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdtmdf5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdtmdf5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdtmdf5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdtmdf5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdtmdf5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf5 {
    #[inline(always)]
    fn default() -> Cfdtmdf5 {
        <crate::RegValueT<Cfdtmdf5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf6_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf6_SPEC {
    type DataType = u32;
}
#[doc = "TX Message Buffer Data Field Register"]
pub type Cfdtmdf6 = crate::RegValueT<Cfdtmdf6_SPEC>;

impl Cfdtmdf6 {
    #[doc = "TX Message Buffer Data Byte ((p × 4)"]
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdtmdf6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdtmdf6_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdtmdf6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdtmdf6_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdtmdf6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdtmdf6_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdtmdf6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdtmdf6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf6 {
    #[inline(always)]
    fn default() -> Cfdtmdf6 {
        <crate::RegValueT<Cfdtmdf6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf7_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf7_SPEC {
    type DataType = u32;
}
#[doc = "TX Message Buffer Data Field Register"]
pub type Cfdtmdf7 = crate::RegValueT<Cfdtmdf7_SPEC>;

impl Cfdtmdf7 {
    #[doc = "TX Message Buffer Data Byte ((p × 4)"]
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdtmdf7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdtmdf7_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdtmdf7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdtmdf7_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdtmdf7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdtmdf7_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdtmdf7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdtmdf7_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf7 {
    #[inline(always)]
    fn default() -> Cfdtmdf7 {
        <crate::RegValueT<Cfdtmdf7_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf8_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf8_SPEC {
    type DataType = u32;
}
#[doc = "TX Message Buffer Data Field Register"]
pub type Cfdtmdf8 = crate::RegValueT<Cfdtmdf8_SPEC>;

impl Cfdtmdf8 {
    #[doc = "TX Message Buffer Data Byte ((p × 4)"]
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdtmdf8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdtmdf8_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdtmdf8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdtmdf8_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdtmdf8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdtmdf8_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdtmdf8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdtmdf8_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf8 {
    #[inline(always)]
    fn default() -> Cfdtmdf8 {
        <crate::RegValueT<Cfdtmdf8_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf9_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf9_SPEC {
    type DataType = u32;
}
#[doc = "TX Message Buffer Data Field Register"]
pub type Cfdtmdf9 = crate::RegValueT<Cfdtmdf9_SPEC>;

impl Cfdtmdf9 {
    #[doc = "TX Message Buffer Data Byte ((p × 4)"]
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdtmdf9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdtmdf9_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdtmdf9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdtmdf9_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdtmdf9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdtmdf9_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdtmdf9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdtmdf9_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf9 {
    #[inline(always)]
    fn default() -> Cfdtmdf9 {
        <crate::RegValueT<Cfdtmdf9_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf10_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf10_SPEC {
    type DataType = u32;
}
#[doc = "TX Message Buffer Data Field Register"]
pub type Cfdtmdf10 = crate::RegValueT<Cfdtmdf10_SPEC>;

impl Cfdtmdf10 {
    #[doc = "TX Message Buffer Data Byte ((p × 4)"]
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdtmdf10_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdtmdf10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdtmdf10_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdtmdf10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdtmdf10_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdtmdf10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdtmdf10_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdtmdf10_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf10 {
    #[inline(always)]
    fn default() -> Cfdtmdf10 {
        <crate::RegValueT<Cfdtmdf10_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf11_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf11_SPEC {
    type DataType = u32;
}
#[doc = "TX Message Buffer Data Field Register"]
pub type Cfdtmdf11 = crate::RegValueT<Cfdtmdf11_SPEC>;

impl Cfdtmdf11 {
    #[doc = "TX Message Buffer Data Byte ((p × 4)"]
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdtmdf11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdtmdf11_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdtmdf11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdtmdf11_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdtmdf11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdtmdf11_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdtmdf11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdtmdf11_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf11 {
    #[inline(always)]
    fn default() -> Cfdtmdf11 {
        <crate::RegValueT<Cfdtmdf11_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf12_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf12_SPEC {
    type DataType = u32;
}
#[doc = "TX Message Buffer Data Field Register"]
pub type Cfdtmdf12 = crate::RegValueT<Cfdtmdf12_SPEC>;

impl Cfdtmdf12 {
    #[doc = "TX Message Buffer Data Byte ((p × 4)"]
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdtmdf12_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdtmdf12_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdtmdf12_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdtmdf12_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdtmdf12_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdtmdf12_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdtmdf12_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdtmdf12_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf12 {
    #[inline(always)]
    fn default() -> Cfdtmdf12 {
        <crate::RegValueT<Cfdtmdf12_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf13_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf13_SPEC {
    type DataType = u32;
}
#[doc = "TX Message Buffer Data Field Register"]
pub type Cfdtmdf13 = crate::RegValueT<Cfdtmdf13_SPEC>;

impl Cfdtmdf13 {
    #[doc = "TX Message Buffer Data Byte ((p × 4)"]
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdtmdf13_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdtmdf13_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdtmdf13_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdtmdf13_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdtmdf13_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdtmdf13_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdtmdf13_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdtmdf13_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf13 {
    #[inline(always)]
    fn default() -> Cfdtmdf13 {
        <crate::RegValueT<Cfdtmdf13_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf14_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf14_SPEC {
    type DataType = u32;
}
#[doc = "TX Message Buffer Data Field Register"]
pub type Cfdtmdf14 = crate::RegValueT<Cfdtmdf14_SPEC>;

impl Cfdtmdf14 {
    #[doc = "TX Message Buffer Data Byte ((p × 4)"]
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdtmdf14_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdtmdf14_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdtmdf14_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdtmdf14_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdtmdf14_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdtmdf14_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdtmdf14_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdtmdf14_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf14 {
    #[inline(always)]
    fn default() -> Cfdtmdf14 {
        <crate::RegValueT<Cfdtmdf14_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf15_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf15_SPEC {
    type DataType = u32;
}
#[doc = "TX Message Buffer Data Field Register"]
pub type Cfdtmdf15 = crate::RegValueT<Cfdtmdf15_SPEC>;

impl Cfdtmdf15 {
    #[doc = "TX Message Buffer Data Byte ((p × 4)"]
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdtmdf15_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdtmdf15_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdtmdf15_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdtmdf15_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdtmdf15_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdtmdf15_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Message Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdtmdf15_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdtmdf15_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf15 {
    #[inline(always)]
    fn default() -> Cfdtmdf15 {
        <crate::RegValueT<Cfdtmdf15_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdthlacc0_SPEC;
impl crate::sealed::RegSpec for Cfdthlacc0_SPEC {
    type DataType = u32;
}
#[doc = "TX History List Access Register 0"]
pub type Cfdthlacc0 = crate::RegValueT<Cfdthlacc0_SPEC>;

impl Cfdthlacc0 {
    #[doc = "Buffer Type"]
    #[inline(always)]
    pub fn bt(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, cfdthlacc0::Bt, Cfdthlacc0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            cfdthlacc0::Bt,
            Cfdthlacc0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Buffer Number"]
    #[inline(always)]
    pub fn bn(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, Cfdthlacc0_SPEC, crate::common::R> {
        crate::common::RegisterField::<3,0x3,1,0,u8, Cfdthlacc0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmit Timestamp"]
    #[inline(always)]
    pub fn tmts(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Cfdthlacc0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Cfdthlacc0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdthlacc0 {
    #[inline(always)]
    fn default() -> Cfdthlacc0 {
        <crate::RegValueT<Cfdthlacc0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdthlacc0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bt_SPEC;
    pub type Bt = crate::EnumBitfieldStruct<u8, Bt_SPEC>;
    impl Bt {
        #[doc = "Flat TX message buffer"]
        pub const _001: Self = Self::new(1);
        #[doc = "TX FIFO message buffer number"]
        pub const _010: Self = Self::new(2);
        #[doc = "TX Queue message buffer number"]
        pub const _100: Self = Self::new(4);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdthlacc1_SPEC;
impl crate::sealed::RegSpec for Cfdthlacc1_SPEC {
    type DataType = u32;
}
#[doc = "TX History List Access Register 1"]
pub type Cfdthlacc1 = crate::RegValueT<Cfdthlacc1_SPEC>;

impl Cfdthlacc1 {
    #[doc = "Transmit ID"]
    #[inline(always)]
    pub fn tid(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Cfdthlacc1_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Cfdthlacc1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmit Information Label"]
    #[inline(always)]
    pub fn tifl(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, Cfdthlacc1_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3,1,0,u8, Cfdthlacc1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdthlacc1 {
    #[inline(always)]
    fn default() -> Cfdthlacc1 {
        <crate::RegValueT<Cfdthlacc1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmid_SPEC;
impl crate::sealed::RegSpec for Cfdrmid_SPEC {
    type DataType = u32;
}
#[doc = "RX Message Buffer ID Registers"]
pub type Cfdrmid = crate::RegValueT<Cfdrmid_SPEC>;

impl Cfdrmid {
    #[doc = "RX Message Buffer ID Field"]
    #[inline(always)]
    pub fn rmid(
        self,
    ) -> crate::common::RegisterField<0, 0x1fffffff, 1, 0, u32, Cfdrmid_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1fffffff,1,0,u32, Cfdrmid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer RTR Bit"]
    #[inline(always)]
    pub fn rmrtr(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, cfdrmid::Rmrtr, Cfdrmid_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<30,0x1,1,0,cfdrmid::Rmrtr, Cfdrmid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer IDE Bit"]
    #[inline(always)]
    pub fn rmide(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, cfdrmid::Rmide, Cfdrmid_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<31,0x1,1,0,cfdrmid::Rmide, Cfdrmid_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmid {
    #[inline(always)]
    fn default() -> Cfdrmid {
        <crate::RegValueT<Cfdrmid_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmid {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmrtr_SPEC;
    pub type Rmrtr = crate::EnumBitfieldStruct<u8, Rmrtr_SPEC>;
    impl Rmrtr {
        #[doc = "Data frame"]
        pub const _0: Self = Self::new(0);
        #[doc = "Remote frame"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmide_SPEC;
    pub type Rmide = crate::EnumBitfieldStruct<u8, Rmide_SPEC>;
    impl Rmide {
        #[doc = "STD-ID is stored"]
        pub const _0: Self = Self::new(0);
        #[doc = "EXT-ID is stored"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmptr_SPEC;
impl crate::sealed::RegSpec for Cfdrmptr_SPEC {
    type DataType = u32;
}
#[doc = "RX Message Buffer Pointer Registers"]
pub type Cfdrmptr = crate::RegValueT<Cfdrmptr_SPEC>;

impl Cfdrmptr {
    #[doc = "RX Message Buffer Timestamp Field"]
    #[inline(always)]
    pub fn rmts(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Cfdrmptr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Cfdrmptr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer DLC Field"]
    #[inline(always)]
    pub fn rmdlc(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, Cfdrmptr_SPEC, crate::common::R> {
        crate::common::RegisterField::<28,0xf,1,0,u8, Cfdrmptr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmptr {
    #[inline(always)]
    fn default() -> Cfdrmptr {
        <crate::RegValueT<Cfdrmptr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmfdsts_SPEC;
impl crate::sealed::RegSpec for Cfdrmfdsts_SPEC {
    type DataType = u32;
}
#[doc = "RX Message Buffer CANFD Status Registers"]
pub type Cfdrmfdsts = crate::RegValueT<Cfdrmfdsts_SPEC>;

impl Cfdrmfdsts {
    #[doc = "Error State Indicator bit"]
    #[inline(always)]
    pub fn rmesi(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdrmfdsts::Rmesi,
        Cfdrmfdsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdrmfdsts::Rmesi,
            Cfdrmfdsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Bit Rate Switch bit"]
    #[inline(always)]
    pub fn rmbrs(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdrmfdsts::Rmbrs,
        Cfdrmfdsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdrmfdsts::Rmbrs,
            Cfdrmfdsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CAN FD Format bit"]
    #[inline(always)]
    pub fn rmfdf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cfdrmfdsts::Rmfdf,
        Cfdrmfdsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdrmfdsts::Rmfdf,
            Cfdrmfdsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "RX Message Buffer Information Label Field"]
    #[inline(always)]
    pub fn rmifl(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Cfdrmfdsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x3,1,0,u8, Cfdrmfdsts_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Pointer Field"]
    #[inline(always)]
    pub fn rmptr(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Cfdrmfdsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Cfdrmfdsts_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmfdsts {
    #[inline(always)]
    fn default() -> Cfdrmfdsts {
        <crate::RegValueT<Cfdrmfdsts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmfdsts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmesi_SPEC;
    pub type Rmesi = crate::EnumBitfieldStruct<u8, Rmesi_SPEC>;
    impl Rmesi {
        #[doc = "CANFD frame received from error active node"]
        pub const _0: Self = Self::new(0);
        #[doc = "CANFD frame received from error passive node"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmbrs_SPEC;
    pub type Rmbrs = crate::EnumBitfieldStruct<u8, Rmbrs_SPEC>;
    impl Rmbrs {
        #[doc = "CANFD frame received with no bit rate switch"]
        pub const _0: Self = Self::new(0);
        #[doc = "CANFD frame received with bit rate switch"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmfdf_SPEC;
    pub type Rmfdf = crate::EnumBitfieldStruct<u8, Rmfdf_SPEC>;
    impl Rmfdf {
        #[doc = "Non CANFD frame received"]
        pub const _0: Self = Self::new(0);
        #[doc = "CANFD frame received"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf0_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf0_SPEC {
    type DataType = u32;
}
#[doc = "RX Message Buffer Data Field 0 Registers"]
pub type Cfdrmdf0 = crate::RegValueT<Cfdrmdf0_SPEC>;

impl Cfdrmdf0 {
    #[doc = "RX Message Buffer Data Byte (p × 4)"]
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdrmdf0_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdrmdf0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdrmdf0_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdrmdf0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdrmdf0_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdrmdf0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdrmdf0_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdrmdf0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf0 {
    #[inline(always)]
    fn default() -> Cfdrmdf0 {
        <crate::RegValueT<Cfdrmdf0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf1_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf1_SPEC {
    type DataType = u32;
}
#[doc = "RX Message Buffer Data Field 1 Registers"]
pub type Cfdrmdf1 = crate::RegValueT<Cfdrmdf1_SPEC>;

impl Cfdrmdf1 {
    #[doc = "RX Message Buffer Data Byte (p × 4)"]
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdrmdf1_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdrmdf1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdrmdf1_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdrmdf1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdrmdf1_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdrmdf1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdrmdf1_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdrmdf1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf1 {
    #[inline(always)]
    fn default() -> Cfdrmdf1 {
        <crate::RegValueT<Cfdrmdf1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf2_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf2_SPEC {
    type DataType = u32;
}
#[doc = "RX Message Buffer Data Field 2 Registers"]
pub type Cfdrmdf2 = crate::RegValueT<Cfdrmdf2_SPEC>;

impl Cfdrmdf2 {
    #[doc = "RX Message Buffer Data Byte (p × 4)"]
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdrmdf2_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdrmdf2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdrmdf2_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdrmdf2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdrmdf2_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdrmdf2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdrmdf2_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdrmdf2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf2 {
    #[inline(always)]
    fn default() -> Cfdrmdf2 {
        <crate::RegValueT<Cfdrmdf2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf3_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf3_SPEC {
    type DataType = u32;
}
#[doc = "RX Message Buffer Data Field 3 Registers"]
pub type Cfdrmdf3 = crate::RegValueT<Cfdrmdf3_SPEC>;

impl Cfdrmdf3 {
    #[doc = "RX Message Buffer Data Byte (p × 4)"]
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdrmdf3_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdrmdf3_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdrmdf3_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdrmdf3_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdrmdf3_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdrmdf3_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdrmdf3_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdrmdf3_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf3 {
    #[inline(always)]
    fn default() -> Cfdrmdf3 {
        <crate::RegValueT<Cfdrmdf3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf4_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf4_SPEC {
    type DataType = u32;
}
#[doc = "RX Message Buffer Data Field 4 Registers"]
pub type Cfdrmdf4 = crate::RegValueT<Cfdrmdf4_SPEC>;

impl Cfdrmdf4 {
    #[doc = "RX Message Buffer Data Byte (p × 4)"]
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdrmdf4_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdrmdf4_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdrmdf4_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdrmdf4_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdrmdf4_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdrmdf4_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdrmdf4_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdrmdf4_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf4 {
    #[inline(always)]
    fn default() -> Cfdrmdf4 {
        <crate::RegValueT<Cfdrmdf4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf5_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf5_SPEC {
    type DataType = u32;
}
#[doc = "RX Message Buffer Data Field 5 Registers"]
pub type Cfdrmdf5 = crate::RegValueT<Cfdrmdf5_SPEC>;

impl Cfdrmdf5 {
    #[doc = "RX Message Buffer Data Byte (p × 4)"]
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdrmdf5_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdrmdf5_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdrmdf5_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdrmdf5_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdrmdf5_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdrmdf5_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdrmdf5_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdrmdf5_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf5 {
    #[inline(always)]
    fn default() -> Cfdrmdf5 {
        <crate::RegValueT<Cfdrmdf5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf6_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf6_SPEC {
    type DataType = u32;
}
#[doc = "RX Message Buffer Data Field 6 Registers"]
pub type Cfdrmdf6 = crate::RegValueT<Cfdrmdf6_SPEC>;

impl Cfdrmdf6 {
    #[doc = "RX Message Buffer Data Byte (p × 4)"]
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdrmdf6_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdrmdf6_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdrmdf6_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdrmdf6_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdrmdf6_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdrmdf6_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdrmdf6_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdrmdf6_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf6 {
    #[inline(always)]
    fn default() -> Cfdrmdf6 {
        <crate::RegValueT<Cfdrmdf6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf7_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf7_SPEC {
    type DataType = u32;
}
#[doc = "RX Message Buffer Data Field 7 Registers"]
pub type Cfdrmdf7 = crate::RegValueT<Cfdrmdf7_SPEC>;

impl Cfdrmdf7 {
    #[doc = "RX Message Buffer Data Byte (p × 4)"]
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdrmdf7_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdrmdf7_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdrmdf7_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdrmdf7_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdrmdf7_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdrmdf7_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdrmdf7_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdrmdf7_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf7 {
    #[inline(always)]
    fn default() -> Cfdrmdf7 {
        <crate::RegValueT<Cfdrmdf7_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf8_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf8_SPEC {
    type DataType = u32;
}
#[doc = "RX Message Buffer Data Field 8 Registers"]
pub type Cfdrmdf8 = crate::RegValueT<Cfdrmdf8_SPEC>;

impl Cfdrmdf8 {
    #[doc = "RX Message Buffer Data Byte (p × 4)"]
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdrmdf8_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdrmdf8_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdrmdf8_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdrmdf8_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdrmdf8_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdrmdf8_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdrmdf8_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdrmdf8_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf8 {
    #[inline(always)]
    fn default() -> Cfdrmdf8 {
        <crate::RegValueT<Cfdrmdf8_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf9_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf9_SPEC {
    type DataType = u32;
}
#[doc = "RX Message Buffer Data Field 9 Registers"]
pub type Cfdrmdf9 = crate::RegValueT<Cfdrmdf9_SPEC>;

impl Cfdrmdf9 {
    #[doc = "RX Message Buffer Data Byte (p × 4)"]
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdrmdf9_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdrmdf9_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdrmdf9_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdrmdf9_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdrmdf9_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdrmdf9_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdrmdf9_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdrmdf9_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf9 {
    #[inline(always)]
    fn default() -> Cfdrmdf9 {
        <crate::RegValueT<Cfdrmdf9_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf10_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf10_SPEC {
    type DataType = u32;
}
#[doc = "RX Message Buffer Data Field 10 Registers"]
pub type Cfdrmdf10 = crate::RegValueT<Cfdrmdf10_SPEC>;

impl Cfdrmdf10 {
    #[doc = "RX Message Buffer Data Byte (p × 4)"]
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdrmdf10_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdrmdf10_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdrmdf10_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdrmdf10_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdrmdf10_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdrmdf10_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdrmdf10_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdrmdf10_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf10 {
    #[inline(always)]
    fn default() -> Cfdrmdf10 {
        <crate::RegValueT<Cfdrmdf10_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf11_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf11_SPEC {
    type DataType = u32;
}
#[doc = "RX Message Buffer Data Field 11 Registers"]
pub type Cfdrmdf11 = crate::RegValueT<Cfdrmdf11_SPEC>;

impl Cfdrmdf11 {
    #[doc = "RX Message Buffer Data Byte (p × 4)"]
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdrmdf11_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdrmdf11_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdrmdf11_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdrmdf11_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdrmdf11_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdrmdf11_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdrmdf11_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdrmdf11_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf11 {
    #[inline(always)]
    fn default() -> Cfdrmdf11 {
        <crate::RegValueT<Cfdrmdf11_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf12_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf12_SPEC {
    type DataType = u32;
}
#[doc = "RX Message Buffer Data Field 12 Registers"]
pub type Cfdrmdf12 = crate::RegValueT<Cfdrmdf12_SPEC>;

impl Cfdrmdf12 {
    #[doc = "RX Message Buffer Data Byte (p × 4)"]
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdrmdf12_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdrmdf12_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdrmdf12_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdrmdf12_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdrmdf12_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdrmdf12_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdrmdf12_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdrmdf12_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf12 {
    #[inline(always)]
    fn default() -> Cfdrmdf12 {
        <crate::RegValueT<Cfdrmdf12_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf13_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf13_SPEC {
    type DataType = u32;
}
#[doc = "RX Message Buffer Data Field 13 Registers"]
pub type Cfdrmdf13 = crate::RegValueT<Cfdrmdf13_SPEC>;

impl Cfdrmdf13 {
    #[doc = "RX Message Buffer Data Byte (p × 4)"]
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdrmdf13_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdrmdf13_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdrmdf13_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdrmdf13_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdrmdf13_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdrmdf13_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdrmdf13_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdrmdf13_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf13 {
    #[inline(always)]
    fn default() -> Cfdrmdf13 {
        <crate::RegValueT<Cfdrmdf13_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf14_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf14_SPEC {
    type DataType = u32;
}
#[doc = "RX Message Buffer Data Field 14 Registers"]
pub type Cfdrmdf14 = crate::RegValueT<Cfdrmdf14_SPEC>;

impl Cfdrmdf14 {
    #[doc = "RX Message Buffer Data Byte (p × 4)"]
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdrmdf14_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdrmdf14_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdrmdf14_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdrmdf14_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdrmdf14_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdrmdf14_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdrmdf14_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdrmdf14_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf14 {
    #[inline(always)]
    fn default() -> Cfdrmdf14 {
        <crate::RegValueT<Cfdrmdf14_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf15_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf15_SPEC {
    type DataType = u32;
}
#[doc = "RX Message Buffer Data Field 15 Registers"]
pub type Cfdrmdf15 = crate::RegValueT<Cfdrmdf15_SPEC>;

impl Cfdrmdf15 {
    #[doc = "RX Message Buffer Data Byte (p × 4)"]
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cfdrmdf15_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cfdrmdf15_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 1)"]
    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Cfdrmdf15_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Cfdrmdf15_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 2)"]
    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Cfdrmdf15_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Cfdrmdf15_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX Message Buffer Data Byte ((p × 4) + 3)"]
    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Cfdrmdf15_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Cfdrmdf15_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf15 {
    #[inline(always)]
    fn default() -> Cfdrmdf15 {
        <crate::RegValueT<Cfdrmdf15_SPEC> as RegisterValue<_>>::new(0)
    }
}
