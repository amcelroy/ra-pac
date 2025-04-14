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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:23:25 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"CANFD Module 0"]
unsafe impl ::core::marker::Send for super::Canfd0 {}
unsafe impl ::core::marker::Sync for super::Canfd0 {}
impl super::Canfd0 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

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

    #[inline(always)]
    pub const fn cfdgipv(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdgipv_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cfdgipv_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

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

    #[inline(always)]
    pub const fn cfdrfpctr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfpctr_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x4cusize))
        }
    }

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

    #[inline(always)]
    pub const fn cfdcfpctr(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcfpctr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdcfpctr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

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

    #[inline(always)]
    pub const fn cfdglotb(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdglotb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdglotb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(188usize),
            )
        }
    }

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

    #[inline(always)]
    pub const fn cfdcdtsts(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcdtsts_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdcdtsts_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(204usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdgpflectr(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdgpflectr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdgpflectr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(208usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdgpflcfg(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdgpflcfg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdgpflcfg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(212usize),
            )
        }
    }

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

    #[inline(always)]
    pub const fn cfdgpflid(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdgpflid_SPEC, crate::common::RW>,
        2,
        0x24,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x220usize))
        }
    }

    #[inline(always)]
    pub const fn cfdgpflm(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdgpflm_SPEC, crate::common::RW>,
        2,
        0x24,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x224usize))
        }
    }

    #[inline(always)]
    pub const fn cfdgpflp0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdgpflp0_SPEC, crate::common::RW>,
        2,
        0x24,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x228usize))
        }
    }

    #[inline(always)]
    pub const fn cfdgpflp1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdgpflp1_SPEC, crate::common::RW>,
        2,
        0x24,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x22cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdgpflpt(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdgpflpt_SPEC, crate::common::RW>,
        2,
        0x24,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x230usize))
        }
    }

    #[inline(always)]
    pub const fn cfdgpflpd0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdgpflpd0_SPEC, crate::common::RW>,
        2,
        0x24,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x234usize))
        }
    }

    #[inline(always)]
    pub const fn cfdgpflpm0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdgpflpm0_SPEC, crate::common::RW>,
        2,
        0x24,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x238usize))
        }
    }

    #[inline(always)]
    pub const fn cfdgpflpd1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdgpflpd1_SPEC, crate::common::RW>,
        2,
        0x24,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x23cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdgpflpm1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdgpflpm1_SPEC, crate::common::RW>,
        2,
        0x24,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x240usize))
        }
    }

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

    #[inline(always)]
    pub const fn cfdrfdf0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf0_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x52cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfdf1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf1_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x578usize))
        }
    }

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

    #[inline(always)]
    pub const fn cfdcfdf0(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcfdf0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdcfdf0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1476usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf1(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcfdf1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdcfdf1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1480usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf2(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcfdf2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdcfdf2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1484usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf3(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcfdf3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdcfdf3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1488usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf4(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcfdf4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdcfdf4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1492usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf5(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcfdf5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdcfdf5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1496usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf6(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcfdf6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdcfdf6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1500usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf7(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcfdf7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdcfdf7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1504usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf8(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcfdf8_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdcfdf8_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1508usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf9(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcfdf9_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdcfdf9_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1512usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf10(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcfdf10_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdcfdf10_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1516usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf11(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcfdf11_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdcfdf11_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1520usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf12(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcfdf12_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdcfdf12_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1524usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf13(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcfdf13_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdcfdf13_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1528usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf14(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcfdf14_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdcfdf14_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1532usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf15(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcfdf15_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdcfdf15_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1536usize),
            )
        }
    }

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

    #[inline(always)]
    pub const fn cfdtmdf0_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf0_SPEC, crate::common::RW>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x610usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf1_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf1_SPEC, crate::common::RW>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x65cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf2_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf2_SPEC, crate::common::RW>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6a8usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf3_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf3_SPEC, crate::common::RW>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6f4usize))
        }
    }

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

    #[inline(always)]
    pub const fn cfdrmdf0_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf0_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x92cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf1_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf1_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x978usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf2_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf2_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x9c4usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf3_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf3_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xa10usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf4_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf4_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xa5cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf5_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf5_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xaa8usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf6_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf6_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xaf4usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf7_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf7_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xb40usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf8_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf8_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xd2cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf9_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf9_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xd78usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf10_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf10_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xdc4usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf11_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf11_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xe10usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf12_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf12_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xe5cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf13_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf13_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xea8usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf14_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf14_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xef4usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf15_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf15_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xf40usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf16_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf16_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x112cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf17_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf17_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1178usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf18_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf18_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x11c4usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf19_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf19_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1210usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf20_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf20_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x125cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf21_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf21_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x12a8usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf22_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf22_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x12f4usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf23_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf23_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1340usize))
        }
    }

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

    #[inline(always)]
    pub const fn cfdrmdf24_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf24_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x152cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf25_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf25_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1578usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf26_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf26_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x15c4usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf27_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf27_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1610usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf28_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf28_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x165cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf29_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf29_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x16a8usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf30_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf30_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x16f4usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf31_(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf31_SPEC, crate::common::R>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1740usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfide(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfide_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1920usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfptre(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfptre_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1924usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrffdstse(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrffdstse_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1928usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfdf0e(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf0E_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x192cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfdf1e(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf1E_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1930usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfdf2e(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf2E_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1934usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfdf3e(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf3E_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1938usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfdf4e(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf4E_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x193cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfdf5e(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf5E_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1940usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfdf6e(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf6E_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1944usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfdf7e(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf7E_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1948usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfdf8e(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf8E_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x194cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfdf9e(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf9E_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1950usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfdf10e(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf10E_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1954usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfdf11e(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf11E_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1958usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfdf12e(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf12E_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x195cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfdf13e(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf13E_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1960usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfdf14e(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf14E_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1964usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfdf15e(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf15E_SPEC, crate::common::R>,
        2,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1968usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfide(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcfide_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdcfide_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6584usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdcfptre(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcfptre_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdcfptre_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6588usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdcffdcstse(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcffdcstse_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdcffdcstse_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6592usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdcfdfe(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdfe_SPEC, crate::common::RW>,
        16,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x19c4usize))
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdc0Ncfg_SPEC;
impl crate::sealed::RegSpec for Cfdc0Ncfg_SPEC {
    type DataType = u32;
}

pub type Cfdc0Ncfg = crate::RegValueT<Cfdc0Ncfg_SPEC>;

impl Cfdc0Ncfg {
    #[inline(always)]
    pub fn nbrp(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Cfdc0Ncfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Cfdc0Ncfg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn nsjw(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x7f,
        1,
        0,
        cfdc0ncfg::Nsjw,
        cfdc0ncfg::Nsjw,
        Cfdc0Ncfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x7f,
            1,
            0,
            cfdc0ncfg::Nsjw,
            cfdc0ncfg::Nsjw,
            Cfdc0Ncfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ntseg1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0xff,
        1,
        0,
        cfdc0ncfg::Ntseg1,
        cfdc0ncfg::Ntseg1,
        Cfdc0Ncfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0xff,
            1,
            0,
            cfdc0ncfg::Ntseg1,
            cfdc0ncfg::Ntseg1,
            Cfdc0Ncfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ntseg2(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x7f,
        1,
        0,
        cfdc0ncfg::Ntseg2,
        cfdc0ncfg::Ntseg2,
        Cfdc0Ncfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x7f,
            1,
            0,
            cfdc0ncfg::Ntseg2,
            cfdc0ncfg::Ntseg2,
            Cfdc0Ncfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdc0Ncfg {
    #[inline(always)]
    fn default() -> Cfdc0Ncfg {
        <crate::RegValueT<Cfdc0Ncfg_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdc0ncfg {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nsjw_SPEC;
    pub type Nsjw = crate::EnumBitfieldStruct<u8, Nsjw_SPEC>;
    impl Nsjw {
        pub const _0000000: Self = Self::new(0);

        pub const _0000001: Self = Self::new(1);

        pub const _0000010: Self = Self::new(2);

        pub const _0000011: Self = Self::new(3);

        pub const _1111110: Self = Self::new(126);

        pub const _1111111: Self = Self::new(127);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ntseg1_SPEC;
    pub type Ntseg1 = crate::EnumBitfieldStruct<u8, Ntseg1_SPEC>;
    impl Ntseg1 {
        pub const _00000000: Self = Self::new(0);

        pub const _00000001: Self = Self::new(1);

        pub const _000000010: Self = Self::new(2);

        pub const _000000011: Self = Self::new(3);

        pub const _11111110: Self = Self::new(254);

        pub const _11111111: Self = Self::new(255);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ntseg2_SPEC;
    pub type Ntseg2 = crate::EnumBitfieldStruct<u8, Ntseg2_SPEC>;
    impl Ntseg2 {
        pub const _0000000: Self = Self::new(0);

        pub const _0000001: Self = Self::new(1);

        pub const _0000010: Self = Self::new(2);

        pub const _0000011: Self = Self::new(3);

        pub const _1111110: Self = Self::new(126);

        pub const _1111111: Self = Self::new(127);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdc0Ctr_SPEC;
impl crate::sealed::RegSpec for Cfdc0Ctr_SPEC {
    type DataType = u32;
}

pub type Cfdc0Ctr = crate::RegValueT<Cfdc0Ctr_SPEC>;

impl Cfdc0Ctr {
    #[inline(always)]
    pub fn chmdc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        cfdc0ctr::Chmdc,
        cfdc0ctr::Chmdc,
        Cfdc0Ctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            cfdc0ctr::Chmdc,
            cfdc0ctr::Chmdc,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cslpr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cfdc0ctr::Cslpr,
        cfdc0ctr::Cslpr,
        Cfdc0Ctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdc0ctr::Cslpr,
            cfdc0ctr::Cslpr,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rtbo(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        cfdc0ctr::Rtbo,
        cfdc0ctr::Rtbo,
        Cfdc0Ctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            cfdc0ctr::Rtbo,
            cfdc0ctr::Rtbo,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn beie(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cfdc0ctr::Beie,
        cfdc0ctr::Beie,
        Cfdc0Ctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cfdc0ctr::Beie,
            cfdc0ctr::Beie,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ewie(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        cfdc0ctr::Ewie,
        cfdc0ctr::Ewie,
        Cfdc0Ctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            cfdc0ctr::Ewie,
            cfdc0ctr::Ewie,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn epie(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        cfdc0ctr::Epie,
        cfdc0ctr::Epie,
        Cfdc0Ctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            cfdc0ctr::Epie,
            cfdc0ctr::Epie,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn boeie(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        cfdc0ctr::Boeie,
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
            cfdc0ctr::Boeie,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn borie(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        cfdc0ctr::Borie,
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
            cfdc0ctr::Borie,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn olie(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        cfdc0ctr::Olie,
        cfdc0ctr::Olie,
        Cfdc0Ctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            cfdc0ctr::Olie,
            cfdc0ctr::Olie,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn blie(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        cfdc0ctr::Blie,
        cfdc0ctr::Blie,
        Cfdc0Ctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            cfdc0ctr::Blie,
            cfdc0ctr::Blie,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn alie(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        cfdc0ctr::Alie,
        cfdc0ctr::Alie,
        Cfdc0Ctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            cfdc0ctr::Alie,
            cfdc0ctr::Alie,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn taie(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        cfdc0ctr::Taie,
        cfdc0ctr::Taie,
        Cfdc0Ctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            cfdc0ctr::Taie,
            cfdc0ctr::Taie,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn eocoie(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        cfdc0ctr::Eocoie,
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
            cfdc0ctr::Eocoie,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn socoie(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        cfdc0ctr::Socoie,
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
            cfdc0ctr::Socoie,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tdcvfie(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        cfdc0ctr::Tdcvfie,
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
            cfdc0ctr::Tdcvfie,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Cfdc0Ctr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,Cfdc0Ctr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bom(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x3,
        1,
        0,
        cfdc0ctr::Bom,
        cfdc0ctr::Bom,
        Cfdc0Ctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x3,
            1,
            0,
            cfdc0ctr::Bom,
            cfdc0ctr::Bom,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn errd(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        cfdc0ctr::Errd,
        cfdc0ctr::Errd,
        Cfdc0Ctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            cfdc0ctr::Errd,
            cfdc0ctr::Errd,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ctme(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        cfdc0ctr::Ctme,
        cfdc0ctr::Ctme,
        Cfdc0Ctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            cfdc0ctr::Ctme,
            cfdc0ctr::Ctme,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ctms(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x3,
        1,
        0,
        cfdc0ctr::Ctms,
        cfdc0ctr::Ctms,
        Cfdc0Ctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x3,
            1,
            0,
            cfdc0ctr::Ctms,
            cfdc0ctr::Ctms,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trwe(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        cfdc0ctr::Trwe,
        cfdc0ctr::Trwe,
        Cfdc0Ctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            cfdc0ctr::Trwe,
            cfdc0ctr::Trwe,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trh(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        cfdc0ctr::Trh,
        cfdc0ctr::Trh,
        Cfdc0Ctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            cfdc0ctr::Trh,
            cfdc0ctr::Trh,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trr(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        cfdc0ctr::Trr,
        cfdc0ctr::Trr,
        Cfdc0Ctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            cfdc0ctr::Trr,
            cfdc0ctr::Trr,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bft(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        cfdc0ctr::Bft,
        cfdc0ctr::Bft,
        Cfdc0Ctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            cfdc0ctr::Bft,
            cfdc0ctr::Bft,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rom(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        cfdc0ctr::Rom,
        cfdc0ctr::Rom,
        Cfdc0Ctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            cfdc0ctr::Rom,
            cfdc0ctr::Rom,
            Cfdc0Ctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cslpr_SPEC;
    pub type Cslpr = crate::EnumBitfieldStruct<u8, Cslpr_SPEC>;
    impl Cslpr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtbo_SPEC;
    pub type Rtbo = crate::EnumBitfieldStruct<u8, Rtbo_SPEC>;
    impl Rtbo {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Beie_SPEC;
    pub type Beie = crate::EnumBitfieldStruct<u8, Beie_SPEC>;
    impl Beie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ewie_SPEC;
    pub type Ewie = crate::EnumBitfieldStruct<u8, Ewie_SPEC>;
    impl Ewie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Epie_SPEC;
    pub type Epie = crate::EnumBitfieldStruct<u8, Epie_SPEC>;
    impl Epie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Boeie_SPEC;
    pub type Boeie = crate::EnumBitfieldStruct<u8, Boeie_SPEC>;
    impl Boeie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Borie_SPEC;
    pub type Borie = crate::EnumBitfieldStruct<u8, Borie_SPEC>;
    impl Borie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Olie_SPEC;
    pub type Olie = crate::EnumBitfieldStruct<u8, Olie_SPEC>;
    impl Olie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Blie_SPEC;
    pub type Blie = crate::EnumBitfieldStruct<u8, Blie_SPEC>;
    impl Blie {
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
    pub struct Taie_SPEC;
    pub type Taie = crate::EnumBitfieldStruct<u8, Taie_SPEC>;
    impl Taie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eocoie_SPEC;
    pub type Eocoie = crate::EnumBitfieldStruct<u8, Eocoie_SPEC>;
    impl Eocoie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Socoie_SPEC;
    pub type Socoie = crate::EnumBitfieldStruct<u8, Socoie_SPEC>;
    impl Socoie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdcvfie_SPEC;
    pub type Tdcvfie = crate::EnumBitfieldStruct<u8, Tdcvfie_SPEC>;
    impl Tdcvfie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bom_SPEC;
    pub type Bom = crate::EnumBitfieldStruct<u8, Bom_SPEC>;
    impl Bom {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Errd_SPEC;
    pub type Errd = crate::EnumBitfieldStruct<u8, Errd_SPEC>;
    impl Errd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctme_SPEC;
    pub type Ctme = crate::EnumBitfieldStruct<u8, Ctme_SPEC>;
    impl Ctme {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctms_SPEC;
    pub type Ctms = crate::EnumBitfieldStruct<u8, Ctms_SPEC>;
    impl Ctms {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trwe_SPEC;
    pub type Trwe = crate::EnumBitfieldStruct<u8, Trwe_SPEC>;
    impl Trwe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trh_SPEC;
    pub type Trh = crate::EnumBitfieldStruct<u8, Trh_SPEC>;
    impl Trh {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trr_SPEC;
    pub type Trr = crate::EnumBitfieldStruct<u8, Trr_SPEC>;
    impl Trr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bft_SPEC;
    pub type Bft = crate::EnumBitfieldStruct<u8, Bft_SPEC>;
    impl Bft {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rom_SPEC;
    pub type Rom = crate::EnumBitfieldStruct<u8, Rom_SPEC>;
    impl Rom {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdc0Sts_SPEC;
impl crate::sealed::RegSpec for Cfdc0Sts_SPEC {
    type DataType = u32;
}

pub type Cfdc0Sts = crate::RegValueT<Cfdc0Sts_SPEC>;

impl Cfdc0Sts {
    #[inline(always)]
    pub fn crststs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdc0sts::Crststs,
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
            cfdc0sts::Crststs,
            Cfdc0Sts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chltsts(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdc0sts::Chltsts,
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
            cfdc0sts::Chltsts,
            Cfdc0Sts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cslpsts(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cfdc0sts::Cslpsts,
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
            cfdc0sts::Cslpsts,
            Cfdc0Sts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn epsts(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        cfdc0sts::Epsts,
        cfdc0sts::Epsts,
        Cfdc0Sts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            cfdc0sts::Epsts,
            cfdc0sts::Epsts,
            Cfdc0Sts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bosts(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        cfdc0sts::Bosts,
        cfdc0sts::Bosts,
        Cfdc0Sts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            cfdc0sts::Bosts,
            cfdc0sts::Bosts,
            Cfdc0Sts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trmsts(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        cfdc0sts::Trmsts,
        cfdc0sts::Trmsts,
        Cfdc0Sts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            cfdc0sts::Trmsts,
            cfdc0sts::Trmsts,
            Cfdc0Sts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn recsts(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        cfdc0sts::Recsts,
        cfdc0sts::Recsts,
        Cfdc0Sts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            cfdc0sts::Recsts,
            cfdc0sts::Recsts,
            Cfdc0Sts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn comsts(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        cfdc0sts::Comsts,
        cfdc0sts::Comsts,
        Cfdc0Sts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            cfdc0sts::Comsts,
            cfdc0sts::Comsts,
            Cfdc0Sts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn esif(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cfdc0sts::Esif,
        cfdc0sts::Esif,
        Cfdc0Sts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cfdc0sts::Esif,
            cfdc0sts::Esif,
            Cfdc0Sts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, u8, Cfdc0Sts_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x7f,1,0,u8,u8,Cfdc0Sts_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rec(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdc0Sts_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdc0Sts_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tec(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdc0Sts_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdc0Sts_SPEC,crate::common::RW>::from_register(self,0)
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chltsts_SPEC;
    pub type Chltsts = crate::EnumBitfieldStruct<u8, Chltsts_SPEC>;
    impl Chltsts {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cslpsts_SPEC;
    pub type Cslpsts = crate::EnumBitfieldStruct<u8, Cslpsts_SPEC>;
    impl Cslpsts {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Epsts_SPEC;
    pub type Epsts = crate::EnumBitfieldStruct<u8, Epsts_SPEC>;
    impl Epsts {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bosts_SPEC;
    pub type Bosts = crate::EnumBitfieldStruct<u8, Bosts_SPEC>;
    impl Bosts {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trmsts_SPEC;
    pub type Trmsts = crate::EnumBitfieldStruct<u8, Trmsts_SPEC>;
    impl Trmsts {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Recsts_SPEC;
    pub type Recsts = crate::EnumBitfieldStruct<u8, Recsts_SPEC>;
    impl Recsts {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Comsts_SPEC;
    pub type Comsts = crate::EnumBitfieldStruct<u8, Comsts_SPEC>;
    impl Comsts {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Esif_SPEC;
    pub type Esif = crate::EnumBitfieldStruct<u8, Esif_SPEC>;
    impl Esif {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdc0Erfl_SPEC;
impl crate::sealed::RegSpec for Cfdc0Erfl_SPEC {
    type DataType = u32;
}

pub type Cfdc0Erfl = crate::RegValueT<Cfdc0Erfl_SPEC>;

impl Cfdc0Erfl {
    #[inline(always)]
    pub fn bef(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdc0erfl::Bef,
        cfdc0erfl::Bef,
        Cfdc0Erfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdc0erfl::Bef,
            cfdc0erfl::Bef,
            Cfdc0Erfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ewf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdc0erfl::Ewf,
        cfdc0erfl::Ewf,
        Cfdc0Erfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdc0erfl::Ewf,
            cfdc0erfl::Ewf,
            Cfdc0Erfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn epf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cfdc0erfl::Epf,
        cfdc0erfl::Epf,
        Cfdc0Erfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdc0erfl::Epf,
            cfdc0erfl::Epf,
            Cfdc0Erfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn boef(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        cfdc0erfl::Boef,
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
            cfdc0erfl::Boef,
            Cfdc0Erfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn borf(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        cfdc0erfl::Borf,
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
            cfdc0erfl::Borf,
            Cfdc0Erfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovlf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        cfdc0erfl::Ovlf,
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
            cfdc0erfl::Ovlf,
            Cfdc0Erfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn blf(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        cfdc0erfl::Blf,
        cfdc0erfl::Blf,
        Cfdc0Erfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            cfdc0erfl::Blf,
            cfdc0erfl::Blf,
            Cfdc0Erfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn alf(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        cfdc0erfl::Alf,
        cfdc0erfl::Alf,
        Cfdc0Erfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            cfdc0erfl::Alf,
            cfdc0erfl::Alf,
            Cfdc0Erfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn serr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cfdc0erfl::Serr,
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
            cfdc0erfl::Serr,
            Cfdc0Erfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ferr(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        cfdc0erfl::Ferr,
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
            cfdc0erfl::Ferr,
            Cfdc0Erfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn aerr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        cfdc0erfl::Aerr,
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
            cfdc0erfl::Aerr,
            Cfdc0Erfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cerr(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        cfdc0erfl::Cerr,
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
            cfdc0erfl::Cerr,
            Cfdc0Erfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn b1err(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        cfdc0erfl::B1Err,
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
            cfdc0erfl::B1Err,
            Cfdc0Erfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn b0err(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        cfdc0erfl::B0Err,
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
            cfdc0erfl::B0Err,
            Cfdc0Erfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn aderr(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        cfdc0erfl::Aderr,
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
            cfdc0erfl::Aderr,
            Cfdc0Erfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn crcreg(
        self,
    ) -> crate::common::RegisterField<16, 0x7fff, 1, 0, u16, u16, Cfdc0Erfl_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x7fff,1,0,u16,u16,Cfdc0Erfl_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Cfdc0Erfl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,Cfdc0Erfl_SPEC,crate::common::RW>::from_register(self,0)
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ewf_SPEC;
    pub type Ewf = crate::EnumBitfieldStruct<u8, Ewf_SPEC>;
    impl Ewf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Epf_SPEC;
    pub type Epf = crate::EnumBitfieldStruct<u8, Epf_SPEC>;
    impl Epf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Boef_SPEC;
    pub type Boef = crate::EnumBitfieldStruct<u8, Boef_SPEC>;
    impl Boef {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Borf_SPEC;
    pub type Borf = crate::EnumBitfieldStruct<u8, Borf_SPEC>;
    impl Borf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovlf_SPEC;
    pub type Ovlf = crate::EnumBitfieldStruct<u8, Ovlf_SPEC>;
    impl Ovlf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Blf_SPEC;
    pub type Blf = crate::EnumBitfieldStruct<u8, Blf_SPEC>;
    impl Blf {
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
    pub struct Serr_SPEC;
    pub type Serr = crate::EnumBitfieldStruct<u8, Serr_SPEC>;
    impl Serr {
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ferr_SPEC;
    pub type Ferr = crate::EnumBitfieldStruct<u8, Ferr_SPEC>;
    impl Ferr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aerr_SPEC;
    pub type Aerr = crate::EnumBitfieldStruct<u8, Aerr_SPEC>;
    impl Aerr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cerr_SPEC;
    pub type Cerr = crate::EnumBitfieldStruct<u8, Cerr_SPEC>;
    impl Cerr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct B1Err_SPEC;
    pub type B1Err = crate::EnumBitfieldStruct<u8, B1Err_SPEC>;
    impl B1Err {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct B0Err_SPEC;
    pub type B0Err = crate::EnumBitfieldStruct<u8, B0Err_SPEC>;
    impl B0Err {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aderr_SPEC;
    pub type Aderr = crate::EnumBitfieldStruct<u8, Aderr_SPEC>;
    impl Aderr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgipv_SPEC;
impl crate::sealed::RegSpec for Cfdgipv_SPEC {
    type DataType = u32;
}

pub type Cfdgipv = crate::RegValueT<Cfdgipv_SPEC>;

impl Cfdgipv {
    #[inline(always)]
    pub fn ipv(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdgipv_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdgipv_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ipt(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, Cfdgipv_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,Cfdgipv_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<10, 0x1f, 1, 0, u8, u8, Cfdgipv_SPEC, crate::common::R> {
        crate::common::RegisterField::<10,0x1f,1,0,u8,u8,Cfdgipv_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cpubus(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Cfdgipv_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Cfdgipv_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn psi(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Cfdgipv_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Cfdgipv_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdgipv {
    #[inline(always)]
    fn default() -> Cfdgipv {
        <crate::RegValueT<Cfdgipv_SPEC> as RegisterValue<_>>::new(1015742787)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgcfg_SPEC;
impl crate::sealed::RegSpec for Cfdgcfg_SPEC {
    type DataType = u32;
}

pub type Cfdgcfg = crate::RegValueT<Cfdgcfg_SPEC>;

impl Cfdgcfg {
    #[inline(always)]
    pub fn tpri(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdgcfg::Tpri,
        cfdgcfg::Tpri,
        Cfdgcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdgcfg::Tpri,
            cfdgcfg::Tpri,
            Cfdgcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dce(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdgcfg::Dce,
        cfdgcfg::Dce,
        Cfdgcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdgcfg::Dce,
            cfdgcfg::Dce,
            Cfdgcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dre(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cfdgcfg::Dre,
        cfdgcfg::Dre,
        Cfdgcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdgcfg::Dre,
            cfdgcfg::Dre,
            Cfdgcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mme(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        cfdgcfg::Mme,
        cfdgcfg::Mme,
        Cfdgcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            cfdgcfg::Mme,
            cfdgcfg::Mme,
            Cfdgcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dcs(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        cfdgcfg::Dcs,
        cfdgcfg::Dcs,
        Cfdgcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            cfdgcfg::Dcs,
            cfdgcfg::Dcs,
            Cfdgcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpoc(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        cfdgcfg::Cmpoc,
        cfdgcfg::Cmpoc,
        Cfdgcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            cfdgcfg::Cmpoc,
            cfdgcfg::Cmpoc,
            Cfdgcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tsp(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        cfdgcfg::Tsp,
        cfdgcfg::Tsp,
        Cfdgcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            cfdgcfg::Tsp,
            cfdgcfg::Tsp,
            Cfdgcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tsss(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        cfdgcfg::Tsss,
        cfdgcfg::Tsss,
        Cfdgcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            cfdgcfg::Tsss,
            cfdgcfg::Tsss,
            Cfdgcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<13, 0x7, 1, 0, u8, u8, Cfdgcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x7,1,0,u8,u8,Cfdgcfg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn itrcp(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Cfdgcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Cfdgcfg_SPEC,crate::common::RW>::from_register(self,0)
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dce_SPEC;
    pub type Dce = crate::EnumBitfieldStruct<u8, Dce_SPEC>;
    impl Dce {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dre_SPEC;
    pub type Dre = crate::EnumBitfieldStruct<u8, Dre_SPEC>;
    impl Dre {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mme_SPEC;
    pub type Mme = crate::EnumBitfieldStruct<u8, Mme_SPEC>;
    impl Mme {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcs_SPEC;
    pub type Dcs = crate::EnumBitfieldStruct<u8, Dcs_SPEC>;
    impl Dcs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpoc_SPEC;
    pub type Cmpoc = crate::EnumBitfieldStruct<u8, Cmpoc_SPEC>;
    impl Cmpoc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsp_SPEC;
    pub type Tsp = crate::EnumBitfieldStruct<u8, Tsp_SPEC>;
    impl Tsp {
        pub const _0000: Self = Self::new(0);

        pub const _0001: Self = Self::new(1);

        pub const _1111: Self = Self::new(15);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsss_SPEC;
    pub type Tsss = crate::EnumBitfieldStruct<u8, Tsss_SPEC>;
    impl Tsss {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgctr_SPEC;
impl crate::sealed::RegSpec for Cfdgctr_SPEC {
    type DataType = u32;
}

pub type Cfdgctr = crate::RegValueT<Cfdgctr_SPEC>;

impl Cfdgctr {
    #[inline(always)]
    pub fn gmdc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        cfdgctr::Gmdc,
        cfdgctr::Gmdc,
        Cfdgctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            cfdgctr::Gmdc,
            cfdgctr::Gmdc,
            Cfdgctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gslpr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cfdgctr::Gslpr,
        cfdgctr::Gslpr,
        Cfdgctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdgctr::Gslpr,
            cfdgctr::Gslpr,
            Cfdgctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn deie(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cfdgctr::Deie,
        cfdgctr::Deie,
        Cfdgctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cfdgctr::Deie,
            cfdgctr::Deie,
            Cfdgctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn meie(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        cfdgctr::Meie,
        cfdgctr::Meie,
        Cfdgctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            cfdgctr::Meie,
            cfdgctr::Meie,
            Cfdgctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn thleie(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        cfdgctr::Thleie,
        cfdgctr::Thleie,
        Cfdgctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            cfdgctr::Thleie,
            cfdgctr::Thleie,
            Cfdgctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpofie(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        cfdgctr::Cmpofie,
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
            cfdgctr::Cmpofie,
            Cfdgctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tsrst(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        cfdgctr::Tsrst,
        cfdgctr::Tsrst,
        Cfdgctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            cfdgctr::Tsrst,
            cfdgctr::Tsrst,
            Cfdgctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tswr(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        cfdgctr::Tswr,
        cfdgctr::Tswr,
        Cfdgctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            cfdgctr::Tswr,
            cfdgctr::Tswr,
            Cfdgctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<18, 0x3fff, 1, 0, u16, u16, Cfdgctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x3fff,1,0,u16,u16,Cfdgctr_SPEC,crate::common::RW>::from_register(self,0)
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
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);

        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gslpr_SPEC;
    pub type Gslpr = crate::EnumBitfieldStruct<u8, Gslpr_SPEC>;
    impl Gslpr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Deie_SPEC;
    pub type Deie = crate::EnumBitfieldStruct<u8, Deie_SPEC>;
    impl Deie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Meie_SPEC;
    pub type Meie = crate::EnumBitfieldStruct<u8, Meie_SPEC>;
    impl Meie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Thleie_SPEC;
    pub type Thleie = crate::EnumBitfieldStruct<u8, Thleie_SPEC>;
    impl Thleie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpofie_SPEC;
    pub type Cmpofie = crate::EnumBitfieldStruct<u8, Cmpofie_SPEC>;
    impl Cmpofie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsrst_SPEC;
    pub type Tsrst = crate::EnumBitfieldStruct<u8, Tsrst_SPEC>;
    impl Tsrst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tswr_SPEC;
    pub type Tswr = crate::EnumBitfieldStruct<u8, Tswr_SPEC>;
    impl Tswr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgsts_SPEC;
impl crate::sealed::RegSpec for Cfdgsts_SPEC {
    type DataType = u32;
}

pub type Cfdgsts = crate::RegValueT<Cfdgsts_SPEC>;

impl Cfdgsts {
    #[inline(always)]
    pub fn grststs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdgsts::Grststs,
        cfdgsts::Grststs,
        Cfdgsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdgsts::Grststs,
            cfdgsts::Grststs,
            Cfdgsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ghltsts(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdgsts::Ghltsts,
        cfdgsts::Ghltsts,
        Cfdgsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdgsts::Ghltsts,
            cfdgsts::Ghltsts,
            Cfdgsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gslpsts(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cfdgsts::Gslpsts,
        cfdgsts::Gslpsts,
        Cfdgsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdgsts::Gslpsts,
            cfdgsts::Gslpsts,
            Cfdgsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn graminit(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        cfdgsts::Graminit,
        cfdgsts::Graminit,
        Cfdgsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            cfdgsts::Graminit,
            cfdgsts::Graminit,
            Cfdgsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xfffffff, 1, 0, u32, u32, Cfdgsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0xfffffff,1,0,u32,u32,Cfdgsts_SPEC,crate::common::R>::from_register(self,0)
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ghltsts_SPEC;
    pub type Ghltsts = crate::EnumBitfieldStruct<u8, Ghltsts_SPEC>;
    impl Ghltsts {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gslpsts_SPEC;
    pub type Gslpsts = crate::EnumBitfieldStruct<u8, Gslpsts_SPEC>;
    impl Gslpsts {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Graminit_SPEC;
    pub type Graminit = crate::EnumBitfieldStruct<u8, Graminit_SPEC>;
    impl Graminit {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgerfl_SPEC;
impl crate::sealed::RegSpec for Cfdgerfl_SPEC {
    type DataType = u32;
}

pub type Cfdgerfl = crate::RegValueT<Cfdgerfl_SPEC>;

impl Cfdgerfl {
    #[inline(always)]
    pub fn def(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdgerfl::Def,
        cfdgerfl::Def,
        Cfdgerfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdgerfl::Def,
            cfdgerfl::Def,
            Cfdgerfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mes(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdgerfl::Mes,
        cfdgerfl::Mes,
        Cfdgerfl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdgerfl::Mes,
            cfdgerfl::Mes,
            Cfdgerfl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn thles(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cfdgerfl::Thles,
        cfdgerfl::Thles,
        Cfdgerfl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdgerfl::Thles,
            cfdgerfl::Thles,
            Cfdgerfl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpof(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        cfdgerfl::Cmpof,
        cfdgerfl::Cmpof,
        Cfdgerfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            cfdgerfl::Cmpof,
            cfdgerfl::Cmpof,
            Cfdgerfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn otbmltsts(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        cfdgerfl::Otbmltsts,
        cfdgerfl::Otbmltsts,
        Cfdgerfl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            cfdgerfl::Otbmltsts,
            cfdgerfl::Otbmltsts,
            Cfdgerfl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rxsfail(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cfdgerfl::Rxsfail,
        cfdgerfl::Rxsfail,
        Cfdgerfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cfdgerfl::Rxsfail,
            cfdgerfl::Rxsfail,
            Cfdgerfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn eef(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        cfdgerfl::Eef,
        cfdgerfl::Eef,
        Cfdgerfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            cfdgerfl::Eef,
            cfdgerfl::Eef,
            Cfdgerfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<17, 0x7fff, 1, 0, u16, u16, Cfdgerfl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x7fff,1,0,u16,u16,Cfdgerfl_SPEC,crate::common::RW>::from_register(self,0)
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
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mes_SPEC;
    pub type Mes = crate::EnumBitfieldStruct<u8, Mes_SPEC>;
    impl Mes {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Thles_SPEC;
    pub type Thles = crate::EnumBitfieldStruct<u8, Thles_SPEC>;
    impl Thles {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpof_SPEC;
    pub type Cmpof = crate::EnumBitfieldStruct<u8, Cmpof_SPEC>;
    impl Cmpof {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Otbmltsts_SPEC;
    pub type Otbmltsts = crate::EnumBitfieldStruct<u8, Otbmltsts_SPEC>;
    impl Otbmltsts {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxsfail_SPEC;
    pub type Rxsfail = crate::EnumBitfieldStruct<u8, Rxsfail_SPEC>;
    impl Rxsfail {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eef_SPEC;
    pub type Eef = crate::EnumBitfieldStruct<u8, Eef_SPEC>;
    impl Eef {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgtsc_SPEC;
impl crate::sealed::RegSpec for Cfdgtsc_SPEC {
    type DataType = u32;
}

pub type Cfdgtsc = crate::RegValueT<Cfdgtsc_SPEC>;

impl Cfdgtsc {
    #[inline(always)]
    pub fn ts(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Cfdgtsc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Cfdgtsc_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Cfdgtsc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Cfdgtsc_SPEC,crate::common::R>::from_register(self,0)
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

pub type Cfdgaflectr = crate::RegValueT<Cfdgaflectr_SPEC>;

impl Cfdgaflectr {
    #[inline(always)]
    pub fn afldae(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cfdgaflectr::Afldae,
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
            cfdgaflectr::Afldae,
            Cfdgaflectr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x7fffff,
        1,
        0,
        u32,
        u32,
        Cfdgaflectr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x7fffff,
            1,
            0,
            u32,
            u32,
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgaflcfg_SPEC;
impl crate::sealed::RegSpec for Cfdgaflcfg_SPEC {
    type DataType = u32;
}

pub type Cfdgaflcfg = crate::RegValueT<Cfdgaflcfg_SPEC>;

impl Cfdgaflcfg {
    #[inline(always)]
    pub fn rnc(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, u8, Cfdgaflcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3f,1,0,u8,u8,Cfdgaflcfg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<22, 0x3ff, 1, 0, u16, u16, Cfdgaflcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x3ff,1,0,u16,u16,Cfdgaflcfg_SPEC,crate::common::RW>::from_register(self,0)
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

pub type Cfdrmnb = crate::RegValueT<Cfdrmnb_SPEC>;

impl Cfdrmnb {
    #[inline(always)]
    pub fn nrxmb(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Cfdrmnb_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Cfdrmnb_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmpls(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        cfdrmnb::Rmpls,
        cfdrmnb::Rmpls,
        Cfdrmnb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            cfdrmnb::Rmpls,
            cfdrmnb::Rmpls,
            Cfdrmnb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<11, 0x1fffff, 1, 0, u32, u32, Cfdrmnb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1fffff,1,0,u32,u32,Cfdrmnb_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct Cfdrmnd_SPEC;
impl crate::sealed::RegSpec for Cfdrmnd_SPEC {
    type DataType = u32;
}

pub type Cfdrmnd = crate::RegValueT<Cfdrmnd_SPEC>;

impl Cfdrmnd {
    #[inline(always)]
    pub fn rmnsu(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        cfdrmnd::RmnSu,
        cfdrmnd::RmnSu,
        Cfdrmnd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            cfdrmnd::RmnSu,
            cfdrmnd::RmnSu,
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
    pub struct RmnSu_SPEC;
    pub type RmnSu = crate::EnumBitfieldStruct<u8, RmnSu_SPEC>;
    impl RmnSu {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmiec_SPEC;
impl crate::sealed::RegSpec for Cfdrmiec_SPEC {
    type DataType = u32;
}

pub type Cfdrmiec = crate::RegValueT<Cfdrmiec_SPEC>;

impl Cfdrmiec {
    #[inline(always)]
    pub fn rmie(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cfdrmiec_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
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

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfcc_SPEC;
impl crate::sealed::RegSpec for Cfdrfcc_SPEC {
    type DataType = u32;
}

pub type Cfdrfcc = crate::RegValueT<Cfdrfcc_SPEC>;

impl Cfdrfcc {
    #[inline(always)]
    pub fn rfe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdrfcc::Rfe,
        cfdrfcc::Rfe,
        Cfdrfcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdrfcc::Rfe,
            cfdrfcc::Rfe,
            Cfdrfcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfie(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdrfcc::Rfie,
        cfdrfcc::Rfie,
        Cfdrfcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdrfcc::Rfie,
            cfdrfcc::Rfie,
            Cfdrfcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfpls(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x7,
        1,
        0,
        cfdrfcc::Rfpls,
        cfdrfcc::Rfpls,
        Cfdrfcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            cfdrfcc::Rfpls,
            cfdrfcc::Rfpls,
            Cfdrfcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdc(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        cfdrfcc::Rfdc,
        cfdrfcc::Rfdc,
        Cfdrfcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            cfdrfcc::Rfdc,
            cfdrfcc::Rfdc,
            Cfdrfcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfim(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        cfdrfcc::Rfim,
        cfdrfcc::Rfim,
        Cfdrfcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            cfdrfcc::Rfim,
            cfdrfcc::Rfim,
            Cfdrfcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfigcv(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x7,
        1,
        0,
        cfdrfcc::Rfigcv,
        cfdrfcc::Rfigcv,
        Cfdrfcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x7,
            1,
            0,
            cfdrfcc::Rfigcv,
            cfdrfcc::Rfigcv,
            Cfdrfcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Cfdrfcc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Cfdrfcc_SPEC,crate::common::RW>::from_register(self,0)
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfie_SPEC;
    pub type Rfie = crate::EnumBitfieldStruct<u8, Rfie_SPEC>;
    impl Rfie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfpls_SPEC;
    pub type Rfpls = crate::EnumBitfieldStruct<u8, Rfpls_SPEC>;
    impl Rfpls {
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
    pub struct Rfdc_SPEC;
    pub type Rfdc = crate::EnumBitfieldStruct<u8, Rfdc_SPEC>;
    impl Rfdc {
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
    pub struct Rfim_SPEC;
    pub type Rfim = crate::EnumBitfieldStruct<u8, Rfim_SPEC>;
    impl Rfim {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfigcv_SPEC;
    pub type Rfigcv = crate::EnumBitfieldStruct<u8, Rfigcv_SPEC>;
    impl Rfigcv {
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
pub struct Cfdrfsts_SPEC;
impl crate::sealed::RegSpec for Cfdrfsts_SPEC {
    type DataType = u32;
}

pub type Cfdrfsts = crate::RegValueT<Cfdrfsts_SPEC>;

impl Cfdrfsts {
    #[inline(always)]
    pub fn rfemp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdrfsts::Rfemp,
        cfdrfsts::Rfemp,
        Cfdrfsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdrfsts::Rfemp,
            cfdrfsts::Rfemp,
            Cfdrfsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rffll(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdrfsts::Rffll,
        cfdrfsts::Rffll,
        Cfdrfsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdrfsts::Rffll,
            cfdrfsts::Rffll,
            Cfdrfsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfmlt(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cfdrfsts::Rfmlt,
        cfdrfsts::Rfmlt,
        Cfdrfsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdrfsts::Rfmlt,
            cfdrfsts::Rfmlt,
            Cfdrfsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfif(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        cfdrfsts::Rfif,
        cfdrfsts::Rfif,
        Cfdrfsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            cfdrfsts::Rfif,
            cfdrfsts::Rfif,
            Cfdrfsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfmc(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, Cfdrfsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x3f,1,0,u8,u8,Cfdrfsts_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<14, 0x3ffff, 1, 0, u32, u32, Cfdrfsts_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3ffff,1,0,u32,u32,Cfdrfsts_SPEC,crate::common::RW>::from_register(self,0)
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rffll_SPEC;
    pub type Rffll = crate::EnumBitfieldStruct<u8, Rffll_SPEC>;
    impl Rffll {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfmlt_SPEC;
    pub type Rfmlt = crate::EnumBitfieldStruct<u8, Rfmlt_SPEC>;
    impl Rfmlt {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfif_SPEC;
    pub type Rfif = crate::EnumBitfieldStruct<u8, Rfif_SPEC>;
    impl Rfif {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfpctr_SPEC;
impl crate::sealed::RegSpec for Cfdrfpctr_SPEC {
    type DataType = u32;
}

pub type Cfdrfpctr = crate::RegValueT<Cfdrfpctr_SPEC>;

impl Cfdrfpctr {
    #[inline(always)]
    pub fn rfpc(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfpctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfpctr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<8, 0xffffff, 1, 0, u32, u32, Cfdrfpctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xffffff,1,0,u32,u32,Cfdrfpctr_SPEC,crate::common::RW>::from_register(self,0)
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

pub type Cfdcfcc = crate::RegValueT<Cfdcfcc_SPEC>;

impl Cfdcfcc {
    #[inline(always)]
    pub fn cfe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdcfcc::Cfe,
        cfdcfcc::Cfe,
        Cfdcfcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdcfcc::Cfe,
            cfdcfcc::Cfe,
            Cfdcfcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfrxie(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdcfcc::Cfrxie,
        cfdcfcc::Cfrxie,
        Cfdcfcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdcfcc::Cfrxie,
            cfdcfcc::Cfrxie,
            Cfdcfcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cftxie(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cfdcfcc::Cftxie,
        cfdcfcc::Cftxie,
        Cfdcfcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdcfcc::Cftxie,
            cfdcfcc::Cftxie,
            Cfdcfcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfpls(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x7,
        1,
        0,
        cfdcfcc::Cfpls,
        cfdcfcc::Cfpls,
        Cfdcfcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            cfdcfcc::Cfpls,
            cfdcfcc::Cfpls,
            Cfdcfcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfm(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cfdcfcc::Cfm,
        cfdcfcc::Cfm,
        Cfdcfcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cfdcfcc::Cfm,
            cfdcfcc::Cfm,
            Cfdcfcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfitss(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        cfdcfcc::Cfitss,
        cfdcfcc::Cfitss,
        Cfdcfcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            cfdcfcc::Cfitss,
            cfdcfcc::Cfitss,
            Cfdcfcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfitr(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        cfdcfcc::Cfitr,
        cfdcfcc::Cfitr,
        Cfdcfcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            cfdcfcc::Cfitr,
            cfdcfcc::Cfitr,
            Cfdcfcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfim(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Cfdcfcc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Cfdcfcc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn cfigcv(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x7,
        1,
        0,
        cfdcfcc::Cfigcv,
        cfdcfcc::Cfigcv,
        Cfdcfcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x7,
            1,
            0,
            cfdcfcc::Cfigcv,
            cfdcfcc::Cfigcv,
            Cfdcfcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cftml(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, u8, Cfdcfcc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8,u8,Cfdcfcc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<18, 0x7, 1, 0, u8, u8, Cfdcfcc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x7,1,0,u8,u8,Cfdcfcc_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdc(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x7,
        1,
        0,
        cfdcfcc::Cfdc,
        cfdcfcc::Cfdc,
        Cfdcfcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x7,
            1,
            0,
            cfdcfcc::Cfdc,
            cfdcfcc::Cfdc,
            Cfdcfcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfitt(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfcc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfcc_SPEC,crate::common::RW>::from_register(self,0)
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfrxie_SPEC;
    pub type Cfrxie = crate::EnumBitfieldStruct<u8, Cfrxie_SPEC>;
    impl Cfrxie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cftxie_SPEC;
    pub type Cftxie = crate::EnumBitfieldStruct<u8, Cftxie_SPEC>;
    impl Cftxie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfpls_SPEC;
    pub type Cfpls = crate::EnumBitfieldStruct<u8, Cfpls_SPEC>;
    impl Cfpls {
        pub const _000: Self = Self::new(0);

        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfm_SPEC;
    pub type Cfm = crate::EnumBitfieldStruct<u8, Cfm_SPEC>;
    impl Cfm {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfitss_SPEC;
    pub type Cfitss = crate::EnumBitfieldStruct<u8, Cfitss_SPEC>;
    impl Cfitss {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfitr_SPEC;
    pub type Cfitr = crate::EnumBitfieldStruct<u8, Cfitr_SPEC>;
    impl Cfitr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfigcv_SPEC;
    pub type Cfigcv = crate::EnumBitfieldStruct<u8, Cfigcv_SPEC>;
    impl Cfigcv {
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
    pub struct Cfdc_SPEC;
    pub type Cfdc = crate::EnumBitfieldStruct<u8, Cfdc_SPEC>;
    impl Cfdc {
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
pub struct Cfdcfsts_SPEC;
impl crate::sealed::RegSpec for Cfdcfsts_SPEC {
    type DataType = u32;
}

pub type Cfdcfsts = crate::RegValueT<Cfdcfsts_SPEC>;

impl Cfdcfsts {
    #[inline(always)]
    pub fn cfemp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdcfsts::Cfemp,
        cfdcfsts::Cfemp,
        Cfdcfsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdcfsts::Cfemp,
            cfdcfsts::Cfemp,
            Cfdcfsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cffll(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdcfsts::Cffll,
        cfdcfsts::Cffll,
        Cfdcfsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdcfsts::Cffll,
            cfdcfsts::Cffll,
            Cfdcfsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfmlt(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cfdcfsts::Cfmlt,
        cfdcfsts::Cfmlt,
        Cfdcfsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdcfsts::Cfmlt,
            cfdcfsts::Cfmlt,
            Cfdcfsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfrxif(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        cfdcfsts::Cfrxif,
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
            cfdcfsts::Cfrxif,
            Cfdcfsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cftxif(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        cfdcfsts::Cftxif,
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
            cfdcfsts::Cftxif,
            Cfdcfsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfmc(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, Cfdcfsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x3f,1,0,u8,u8,Cfdcfsts_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<14, 0x3ffff, 1, 0, u32, u32, Cfdcfsts_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3ffff,1,0,u32,u32,Cfdcfsts_SPEC,crate::common::RW>::from_register(self,0)
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cffll_SPEC;
    pub type Cffll = crate::EnumBitfieldStruct<u8, Cffll_SPEC>;
    impl Cffll {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfmlt_SPEC;
    pub type Cfmlt = crate::EnumBitfieldStruct<u8, Cfmlt_SPEC>;
    impl Cfmlt {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfrxif_SPEC;
    pub type Cfrxif = crate::EnumBitfieldStruct<u8, Cfrxif_SPEC>;
    impl Cfrxif {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cftxif_SPEC;
    pub type Cftxif = crate::EnumBitfieldStruct<u8, Cftxif_SPEC>;
    impl Cftxif {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfpctr_SPEC;
impl crate::sealed::RegSpec for Cfdcfpctr_SPEC {
    type DataType = u32;
}

pub type Cfdcfpctr = crate::RegValueT<Cfdcfpctr_SPEC>;

impl Cfdcfpctr {
    #[inline(always)]
    pub fn cfpc(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfpctr_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfpctr_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<8, 0xffffff, 1, 0, u32, u32, Cfdcfpctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xffffff,1,0,u32,u32,Cfdcfpctr_SPEC,crate::common::RW>::from_register(self,0)
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

pub type Cfdfests = crate::RegValueT<Cfdfests_SPEC>;

impl Cfdfests {
    #[inline(always)]
    pub fn rf0emp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdfests::Rf0Emp,
        cfdfests::Rf0Emp,
        Cfdfests_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdfests::Rf0Emp,
            cfdfests::Rf0Emp,
            Cfdfests_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rf1emp(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdfests::Rf1Emp,
        cfdfests::Rf1Emp,
        Cfdfests_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdfests::Rf1Emp,
            cfdfests::Rf1Emp,
            Cfdfests_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfemp(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Cfdfests_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cfdfests_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<9, 0x7fffff, 1, 0, u32, u32, Cfdfests_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<9,0x7fffff,1,0,u32,u32,Cfdfests_SPEC,crate::common::R>::from_register(self,0)
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
    pub struct Rf0Emp_SPEC;
    pub type Rf0Emp = crate::EnumBitfieldStruct<u8, Rf0Emp_SPEC>;
    impl Rf0Emp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rf1Emp_SPEC;
    pub type Rf1Emp = crate::EnumBitfieldStruct<u8, Rf1Emp_SPEC>;
    impl Rf1Emp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdffsts_SPEC;
impl crate::sealed::RegSpec for Cfdffsts_SPEC {
    type DataType = u32;
}

pub type Cfdffsts = crate::RegValueT<Cfdffsts_SPEC>;

impl Cfdffsts {
    #[inline(always)]
    pub fn rf0fll(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdffsts::Rf0Fll,
        cfdffsts::Rf0Fll,
        Cfdffsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdffsts::Rf0Fll,
            cfdffsts::Rf0Fll,
            Cfdffsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rf1fll(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdffsts::Rf1Fll,
        cfdffsts::Rf1Fll,
        Cfdffsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdffsts::Rf1Fll,
            cfdffsts::Rf1Fll,
            Cfdffsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cffll(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cfdffsts::Cffll,
        cfdffsts::Cffll,
        Cfdffsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cfdffsts::Cffll,
            cfdffsts::Cffll,
            Cfdffsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<9, 0x7fffff, 1, 0, u32, u32, Cfdffsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<9,0x7fffff,1,0,u32,u32,Cfdffsts_SPEC,crate::common::R>::from_register(self,0)
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
    pub struct Rf0Fll_SPEC;
    pub type Rf0Fll = crate::EnumBitfieldStruct<u8, Rf0Fll_SPEC>;
    impl Rf0Fll {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rf1Fll_SPEC;
    pub type Rf1Fll = crate::EnumBitfieldStruct<u8, Rf1Fll_SPEC>;
    impl Rf1Fll {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cffll_SPEC;
    pub type Cffll = crate::EnumBitfieldStruct<u8, Cffll_SPEC>;
    impl Cffll {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdfmsts_SPEC;
impl crate::sealed::RegSpec for Cfdfmsts_SPEC {
    type DataType = u32;
}

pub type Cfdfmsts = crate::RegValueT<Cfdfmsts_SPEC>;

impl Cfdfmsts {
    #[inline(always)]
    pub fn rf0mlt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdfmsts::Rf0Mlt,
        cfdfmsts::Rf0Mlt,
        Cfdfmsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdfmsts::Rf0Mlt,
            cfdfmsts::Rf0Mlt,
            Cfdfmsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rf1mlt(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdfmsts::Rf1Mlt,
        cfdfmsts::Rf1Mlt,
        Cfdfmsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdfmsts::Rf1Mlt,
            cfdfmsts::Rf1Mlt,
            Cfdfmsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfmlt(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cfdfmsts::Cfmlt,
        cfdfmsts::Cfmlt,
        Cfdfmsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cfdfmsts::Cfmlt,
            cfdfmsts::Cfmlt,
            Cfdfmsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<9, 0x7fffff, 1, 0, u32, u32, Cfdfmsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<9,0x7fffff,1,0,u32,u32,Cfdfmsts_SPEC,crate::common::R>::from_register(self,0)
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
    pub struct Rf0Mlt_SPEC;
    pub type Rf0Mlt = crate::EnumBitfieldStruct<u8, Rf0Mlt_SPEC>;
    impl Rf0Mlt {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rf1Mlt_SPEC;
    pub type Rf1Mlt = crate::EnumBitfieldStruct<u8, Rf1Mlt_SPEC>;
    impl Rf1Mlt {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfmlt_SPEC;
    pub type Cfmlt = crate::EnumBitfieldStruct<u8, Cfmlt_SPEC>;
    impl Cfmlt {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfists_SPEC;
impl crate::sealed::RegSpec for Cfdrfists_SPEC {
    type DataType = u32;
}

pub type Cfdrfists = crate::RegValueT<Cfdrfists_SPEC>;

impl Cfdrfists {
    #[inline(always)]
    pub fn rf0if(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdrfists::Rf0If,
        cfdrfists::Rf0If,
        Cfdrfists_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdrfists::Rf0If,
            cfdrfists::Rf0If,
            Cfdrfists_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rf1if(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdrfists::Rf1If,
        cfdrfists::Rf1If,
        Cfdrfists_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdrfists::Rf1If,
            cfdrfists::Rf1If,
            Cfdrfists_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3fffffff, 1, 0, u32, u32, Cfdrfists_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            2,
            0x3fffffff,
            1,
            0,
            u32,
            u32,
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
    pub struct Rf0If_SPEC;
    pub type Rf0If = crate::EnumBitfieldStruct<u8, Rf0If_SPEC>;
    impl Rf0If {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rf1If_SPEC;
    pub type Rf1If = crate::EnumBitfieldStruct<u8, Rf1If_SPEC>;
    impl Rf1If {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmc_SPEC;
impl crate::sealed::RegSpec for Cfdtmc_SPEC {
    type DataType = u8;
}

pub type Cfdtmc = crate::RegValueT<Cfdtmc_SPEC>;

impl Cfdtmc {
    #[inline(always)]
    pub fn tmtr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdtmc::Tmtr,
        cfdtmc::Tmtr,
        Cfdtmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdtmc::Tmtr,
            cfdtmc::Tmtr,
            Cfdtmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tmtar(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdtmc::Tmtar,
        cfdtmc::Tmtar,
        Cfdtmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdtmc::Tmtar,
            cfdtmc::Tmtar,
            Cfdtmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tmom(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cfdtmc::Tmom,
        cfdtmc::Tmom,
        Cfdtmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdtmc::Tmom,
            cfdtmc::Tmom,
            Cfdtmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Cfdtmc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Cfdtmc_SPEC,crate::common::RW>::from_register(self,0)
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmtar_SPEC;
    pub type Tmtar = crate::EnumBitfieldStruct<u8, Tmtar_SPEC>;
    impl Tmtar {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmom_SPEC;
    pub type Tmom = crate::EnumBitfieldStruct<u8, Tmom_SPEC>;
    impl Tmom {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmsts_SPEC;
impl crate::sealed::RegSpec for Cfdtmsts_SPEC {
    type DataType = u8;
}

pub type Cfdtmsts = crate::RegValueT<Cfdtmsts_SPEC>;

impl Cfdtmsts {
    #[inline(always)]
    pub fn tmtsts(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdtmsts::Tmtsts,
        cfdtmsts::Tmtsts,
        Cfdtmsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdtmsts::Tmtsts,
            cfdtmsts::Tmtsts,
            Cfdtmsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tmtrf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x3,
        1,
        0,
        cfdtmsts::Tmtrf,
        cfdtmsts::Tmtrf,
        Cfdtmsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x3,
            1,
            0,
            cfdtmsts::Tmtrf,
            cfdtmsts::Tmtrf,
            Cfdtmsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tmtrm(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        cfdtmsts::Tmtrm,
        cfdtmsts::Tmtrm,
        Cfdtmsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            cfdtmsts::Tmtrm,
            cfdtmsts::Tmtrm,
            Cfdtmsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tmtarm(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        cfdtmsts::Tmtarm,
        cfdtmsts::Tmtarm,
        Cfdtmsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            cfdtmsts::Tmtarm,
            cfdtmsts::Tmtarm,
            Cfdtmsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, Cfdtmsts_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,Cfdtmsts_SPEC,crate::common::RW>::from_register(self,0)
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmtrf_SPEC;
    pub type Tmtrf = crate::EnumBitfieldStruct<u8, Tmtrf_SPEC>;
    impl Tmtrf {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmtrm_SPEC;
    pub type Tmtrm = crate::EnumBitfieldStruct<u8, Tmtrm_SPEC>;
    impl Tmtrm {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmtarm_SPEC;
    pub type Tmtarm = crate::EnumBitfieldStruct<u8, Tmtarm_SPEC>;
    impl Tmtarm {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmtrsts_SPEC;
impl crate::sealed::RegSpec for Cfdtmtrsts_SPEC {
    type DataType = u32;
}

pub type Cfdtmtrsts = crate::RegValueT<Cfdtmtrsts_SPEC>;

impl Cfdtmtrsts {
    #[inline(always)]
    pub fn cfdtmtrsts(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        cfdtmtrsts::Cfdtmtrsts,
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
            cfdtmtrsts::Cfdtmtrsts,
            Cfdtmtrsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xfffffff, 1, 0, u32, u32, Cfdtmtrsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            4,
            0xfffffff,
            1,
            0,
            u32,
            u32,
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmtarsts_SPEC;
impl crate::sealed::RegSpec for Cfdtmtarsts_SPEC {
    type DataType = u32;
}

pub type Cfdtmtarsts = crate::RegValueT<Cfdtmtarsts_SPEC>;

impl Cfdtmtarsts {
    #[inline(always)]
    pub fn cfdtmtarsts(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        cfdtmtarsts::Cfdtmtarsts,
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
            cfdtmtarsts::Cfdtmtarsts,
            Cfdtmtarsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xfffffff,
        1,
        0,
        u32,
        u32,
        Cfdtmtarsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0xfffffff,
            1,
            0,
            u32,
            u32,
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmtcsts_SPEC;
impl crate::sealed::RegSpec for Cfdtmtcsts_SPEC {
    type DataType = u32;
}

pub type Cfdtmtcsts = crate::RegValueT<Cfdtmtcsts_SPEC>;

impl Cfdtmtcsts {
    #[inline(always)]
    pub fn cfdtmtcsts(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        cfdtmtcsts::Cfdtmtcsts,
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
            cfdtmtcsts::Cfdtmtcsts,
            Cfdtmtcsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xfffffff, 1, 0, u32, u32, Cfdtmtcsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            4,
            0xfffffff,
            1,
            0,
            u32,
            u32,
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmtasts_SPEC;
impl crate::sealed::RegSpec for Cfdtmtasts_SPEC {
    type DataType = u32;
}

pub type Cfdtmtasts = crate::RegValueT<Cfdtmtasts_SPEC>;

impl Cfdtmtasts {
    #[inline(always)]
    pub fn cfdtmtasts(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        cfdtmtasts::Cfdtmtasts,
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
            cfdtmtasts::Cfdtmtasts,
            Cfdtmtasts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xfffffff, 1, 0, u32, u32, Cfdtmtasts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            4,
            0xfffffff,
            1,
            0,
            u32,
            u32,
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmiec_SPEC;
impl crate::sealed::RegSpec for Cfdtmiec_SPEC {
    type DataType = u32;
}

pub type Cfdtmiec = crate::RegValueT<Cfdtmiec_SPEC>;

impl Cfdtmiec {
    #[inline(always)]
    pub fn tmie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        cfdtmiec::Tmie,
        cfdtmiec::Tmie,
        Cfdtmiec_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            cfdtmiec::Tmie,
            cfdtmiec::Tmie,
            Cfdtmiec_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xfffffff, 1, 0, u32, u32, Cfdtmiec_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xfffffff,1,0,u32,u32,Cfdtmiec_SPEC,crate::common::RW>::from_register(self,0)
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
    pub struct Tmie_SPEC;
    pub type Tmie = crate::EnumBitfieldStruct<u8, Tmie_SPEC>;
    impl Tmie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtxqcc_SPEC;
impl crate::sealed::RegSpec for Cfdtxqcc_SPEC {
    type DataType = u32;
}

pub type Cfdtxqcc = crate::RegValueT<Cfdtxqcc_SPEC>;

impl Cfdtxqcc {
    #[inline(always)]
    pub fn txqe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdtxqcc::Txqe,
        cfdtxqcc::Txqe,
        Cfdtxqcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdtxqcc::Txqe,
            cfdtxqcc::Txqe,
            Cfdtxqcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txqtxie(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        cfdtxqcc::Txqtxie,
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
            cfdtxqcc::Txqtxie,
            Cfdtxqcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txqim(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        cfdtxqcc::Txqim,
        cfdtxqcc::Txqim,
        Cfdtxqcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            cfdtxqcc::Txqim,
            cfdtxqcc::Txqim,
            Cfdtxqcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txqdc(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        cfdtxqcc::Txqdc,
        cfdtxqcc::Txqdc,
        Cfdtxqcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            cfdtxqcc::Txqdc,
            cfdtxqcc::Txqdc,
            Cfdtxqcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<10, 0x3fffff, 1, 0, u32, u32, Cfdtxqcc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3fffff,1,0,u32,u32,Cfdtxqcc_SPEC,crate::common::RW>::from_register(self,0)
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txqtxie_SPEC;
    pub type Txqtxie = crate::EnumBitfieldStruct<u8, Txqtxie_SPEC>;
    impl Txqtxie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txqim_SPEC;
    pub type Txqim = crate::EnumBitfieldStruct<u8, Txqim_SPEC>;
    impl Txqim {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txqdc_SPEC;
    pub type Txqdc = crate::EnumBitfieldStruct<u8, Txqdc_SPEC>;
    impl Txqdc {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtxqsts_SPEC;
impl crate::sealed::RegSpec for Cfdtxqsts_SPEC {
    type DataType = u32;
}

pub type Cfdtxqsts = crate::RegValueT<Cfdtxqsts_SPEC>;

impl Cfdtxqsts {
    #[inline(always)]
    pub fn txqemp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdtxqsts::Txqemp,
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
            cfdtxqsts::Txqemp,
            Cfdtxqsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txqfll(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdtxqsts::Txqfll,
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
            cfdtxqsts::Txqfll,
            Cfdtxqsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txqtxif(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cfdtxqsts::Txqtxif,
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
            cfdtxqsts::Txqtxif,
            Cfdtxqsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txqmc(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, Cfdtxqsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,Cfdtxqsts_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<11, 0x1fffff, 1, 0, u32, u32, Cfdtxqsts_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            11,
            0x1fffff,
            1,
            0,
            u32,
            u32,
            Cfdtxqsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txqfll_SPEC;
    pub type Txqfll = crate::EnumBitfieldStruct<u8, Txqfll_SPEC>;
    impl Txqfll {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txqtxif_SPEC;
    pub type Txqtxif = crate::EnumBitfieldStruct<u8, Txqtxif_SPEC>;
    impl Txqtxif {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtxqpctr_SPEC;
impl crate::sealed::RegSpec for Cfdtxqpctr_SPEC {
    type DataType = u32;
}

pub type Cfdtxqpctr = crate::RegValueT<Cfdtxqpctr_SPEC>;

impl Cfdtxqpctr {
    #[inline(always)]
    pub fn txqpc(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtxqpctr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtxqpctr_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<8, 0xffffff, 1, 0, u32, u32, Cfdtxqpctr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xffffff,1,0,u32,u32,Cfdtxqpctr_SPEC,crate::common::W>::from_register(self,0)
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

pub type Cfdthlcc = crate::RegValueT<Cfdthlcc_SPEC>;

impl Cfdthlcc {
    #[inline(always)]
    pub fn thle(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdthlcc::Thle,
        cfdthlcc::Thle,
        Cfdthlcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdthlcc::Thle,
            cfdthlcc::Thle,
            Cfdthlcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn thlie(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cfdthlcc::Thlie,
        cfdthlcc::Thlie,
        Cfdthlcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cfdthlcc::Thlie,
            cfdthlcc::Thlie,
            Cfdthlcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn thlim(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        cfdthlcc::Thlim,
        cfdthlcc::Thlim,
        Cfdthlcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            cfdthlcc::Thlim,
            cfdthlcc::Thlim,
            Cfdthlcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn thldte(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        cfdthlcc::Thldte,
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
            cfdthlcc::Thldte,
            Cfdthlcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<11, 0x1fffff, 1, 0, u32, u32, Cfdthlcc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1fffff,1,0,u32,u32,Cfdthlcc_SPEC,crate::common::RW>::from_register(self,0)
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Thlie_SPEC;
    pub type Thlie = crate::EnumBitfieldStruct<u8, Thlie_SPEC>;
    impl Thlie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Thlim_SPEC;
    pub type Thlim = crate::EnumBitfieldStruct<u8, Thlim_SPEC>;
    impl Thlim {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Thldte_SPEC;
    pub type Thldte = crate::EnumBitfieldStruct<u8, Thldte_SPEC>;
    impl Thldte {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdthlsts_SPEC;
impl crate::sealed::RegSpec for Cfdthlsts_SPEC {
    type DataType = u32;
}

pub type Cfdthlsts = crate::RegValueT<Cfdthlsts_SPEC>;

impl Cfdthlsts {
    #[inline(always)]
    pub fn thlemp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdthlsts::Thlemp,
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
            cfdthlsts::Thlemp,
            Cfdthlsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn thlfll(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdthlsts::Thlfll,
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
            cfdthlsts::Thlfll,
            Cfdthlsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn thlelt(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cfdthlsts::Thlelt,
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
            cfdthlsts::Thlelt,
            Cfdthlsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn thlif(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        cfdthlsts::Thlif,
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
            cfdthlsts::Thlif,
            Cfdthlsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn thlmc(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, Cfdthlsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,Cfdthlsts_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<12, 0xfffff, 1, 0, u32, u32, Cfdthlsts_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0xfffff,1,0,u32,u32,Cfdthlsts_SPEC,crate::common::RW>::from_register(self,0)
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Thlfll_SPEC;
    pub type Thlfll = crate::EnumBitfieldStruct<u8, Thlfll_SPEC>;
    impl Thlfll {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Thlelt_SPEC;
    pub type Thlelt = crate::EnumBitfieldStruct<u8, Thlelt_SPEC>;
    impl Thlelt {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Thlif_SPEC;
    pub type Thlif = crate::EnumBitfieldStruct<u8, Thlif_SPEC>;
    impl Thlif {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdthlpctr_SPEC;
impl crate::sealed::RegSpec for Cfdthlpctr_SPEC {
    type DataType = u32;
}

pub type Cfdthlpctr = crate::RegValueT<Cfdthlpctr_SPEC>;

impl Cfdthlpctr {
    #[inline(always)]
    pub fn thlpc(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdthlpctr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdthlpctr_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<8, 0xffffff, 1, 0, u32, u32, Cfdthlpctr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xffffff,1,0,u32,u32,Cfdthlpctr_SPEC,crate::common::W>::from_register(self,0)
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

pub type Cfdgtintsts = crate::RegValueT<Cfdgtintsts_SPEC>;

impl Cfdgtintsts {
    #[inline(always)]
    pub fn tsif(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdgtintsts::Tsif,
        cfdgtintsts::Tsif,
        Cfdgtintsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdgtintsts::Tsif,
            cfdgtintsts::Tsif,
            Cfdgtintsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn taif(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdgtintsts::Taif,
        cfdgtintsts::Taif,
        Cfdgtintsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdgtintsts::Taif,
            cfdgtintsts::Taif,
            Cfdgtintsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tqif(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cfdgtintsts::Tqif,
        cfdgtintsts::Tqif,
        Cfdgtintsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdgtintsts::Tqif,
            cfdgtintsts::Tqif,
            Cfdgtintsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cftif(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        cfdgtintsts::Cftif,
        cfdgtintsts::Cftif,
        Cfdgtintsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            cfdgtintsts::Cftif,
            cfdgtintsts::Cftif,
            Cfdgtintsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn thif(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        cfdgtintsts::Thif,
        cfdgtintsts::Thif,
        Cfdgtintsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            cfdgtintsts::Thif,
            cfdgtintsts::Thif,
            Cfdgtintsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x7ffffff,
        1,
        0,
        u32,
        u32,
        Cfdgtintsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x7ffffff,
            1,
            0,
            u32,
            u32,
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
    pub struct Tsif_SPEC;
    pub type Tsif = crate::EnumBitfieldStruct<u8, Tsif_SPEC>;
    impl Tsif {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Taif_SPEC;
    pub type Taif = crate::EnumBitfieldStruct<u8, Taif_SPEC>;
    impl Taif {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tqif_SPEC;
    pub type Tqif = crate::EnumBitfieldStruct<u8, Tqif_SPEC>;
    impl Tqif {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cftif_SPEC;
    pub type Cftif = crate::EnumBitfieldStruct<u8, Cftif_SPEC>;
    impl Cftif {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Thif_SPEC;
    pub type Thif = crate::EnumBitfieldStruct<u8, Thif_SPEC>;
    impl Thif {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgtstcfg_SPEC;
impl crate::sealed::RegSpec for Cfdgtstcfg_SPEC {
    type DataType = u32;
}

pub type Cfdgtstcfg = crate::RegValueT<Cfdgtstcfg_SPEC>;

impl Cfdgtstcfg {
    #[inline(always)]
    pub fn rtmps(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Cfdgtstcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Cfdgtstcfg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<20, 0xfff, 1, 0, u16, u16, Cfdgtstcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0xfff,1,0,u16,u16,Cfdgtstcfg_SPEC,crate::common::RW>::from_register(self,0)
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

pub type Cfdgtstctr = crate::RegValueT<Cfdgtstctr_SPEC>;

impl Cfdgtstctr {
    #[inline(always)]
    pub fn rtme(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cfdgtstctr::Rtme,
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
            cfdgtstctr::Rtme,
            Cfdgtstctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1fffffff,
        1,
        0,
        u32,
        u32,
        Cfdgtstctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1fffffff,
            1,
            0,
            u32,
            u32,
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgfdcfg_SPEC;
impl crate::sealed::RegSpec for Cfdgfdcfg_SPEC {
    type DataType = u32;
}

pub type Cfdgfdcfg = crate::RegValueT<Cfdgfdcfg_SPEC>;

impl Cfdgfdcfg {
    #[inline(always)]
    pub fn rped(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdgfdcfg::Rped,
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
            cfdgfdcfg::Rped,
            Cfdgfdcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tsccfg(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        cfdgfdcfg::Tsccfg,
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
            cfdgfdcfg::Tsccfg,
            Cfdgfdcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<10, 0x3fffff, 1, 0, u32, u32, Cfdgfdcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            10,
            0x3fffff,
            1,
            0,
            u32,
            u32,
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsccfg_SPEC;
    pub type Tsccfg = crate::EnumBitfieldStruct<u8, Tsccfg_SPEC>;
    impl Tsccfg {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdglockk_SPEC;
impl crate::sealed::RegSpec for Cfdglockk_SPEC {
    type DataType = u32;
}

pub type Cfdglockk = crate::RegValueT<Cfdglockk_SPEC>;

impl Cfdglockk {
    #[inline(always)]
    pub fn lock(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Cfdglockk_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Cfdglockk_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Cfdglockk_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Cfdglockk_SPEC,crate::common::W>::from_register(self,0)
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
pub struct Cfdglotb_SPEC;
impl crate::sealed::RegSpec for Cfdglotb_SPEC {
    type DataType = u32;
}

pub type Cfdglotb = crate::RegValueT<Cfdglotb_SPEC>;

impl Cfdglotb {
    #[inline(always)]
    pub fn otbfe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdglotb::Otbfe,
        cfdglotb::Otbfe,
        Cfdglotb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdglotb::Otbfe,
            cfdglotb::Otbfe,
            Cfdglotb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn otbemp(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cfdglotb::Otbemp,
        cfdglotb::Otbemp,
        Cfdglotb_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cfdglotb::Otbemp,
            cfdglotb::Otbemp,
            Cfdglotb_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn otbfll(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        cfdglotb::Otbfll,
        cfdglotb::Otbfll,
        Cfdglotb_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            cfdglotb::Otbfll,
            cfdglotb::Otbfll,
            Cfdglotb_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn otbmlt(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        cfdglotb::Otbmlt,
        cfdglotb::Otbmlt,
        Cfdglotb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            cfdglotb::Otbmlt,
            cfdglotb::Otbmlt,
            Cfdglotb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn otbmc(
        self,
    ) -> crate::common::RegisterField<11, 0x3, 1, 0, u8, u8, Cfdglotb_SPEC, crate::common::R> {
        crate::common::RegisterField::<11,0x3,1,0,u8,u8,Cfdglotb_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<13, 0x7ffff, 1, 0, u32, u32, Cfdglotb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x7ffff,1,0,u32,u32,Cfdglotb_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdglotb {
    #[inline(always)]
    fn default() -> Cfdglotb {
        <crate::RegValueT<Cfdglotb_SPEC> as RegisterValue<_>>::new(257)
    }
}
pub mod cfdglotb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Otbfe_SPEC;
    pub type Otbfe = crate::EnumBitfieldStruct<u8, Otbfe_SPEC>;
    impl Otbfe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Otbemp_SPEC;
    pub type Otbemp = crate::EnumBitfieldStruct<u8, Otbemp_SPEC>;
    impl Otbemp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Otbfll_SPEC;
    pub type Otbfll = crate::EnumBitfieldStruct<u8, Otbfll_SPEC>;
    impl Otbfll {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Otbmlt_SPEC;
    pub type Otbmlt = crate::EnumBitfieldStruct<u8, Otbmlt_SPEC>;
    impl Otbmlt {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgaflignent_SPEC;
impl crate::sealed::RegSpec for Cfdgaflignent_SPEC {
    type DataType = u32;
}

pub type Cfdgaflignent = crate::RegValueT<Cfdgaflignent_SPEC>;

impl Cfdgaflignent {
    #[inline(always)]
    pub fn irn(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Cfdgaflignent_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Cfdgaflignent_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        Cfdgaflignent_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            Cfdgaflignent_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type Cfdgaflignctr = crate::RegValueT<Cfdgaflignctr_SPEC>;

impl Cfdgaflignctr {
    #[inline(always)]
    pub fn iren(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdgaflignctr::Iren,
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
            cfdgaflignctr::Iren,
            Cfdgaflignctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdgaflignctr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdgaflignctr_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        Cfdgaflignctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            Cfdgaflignctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcdtct_SPEC;
impl crate::sealed::RegSpec for Cfdcdtct_SPEC {
    type DataType = u32;
}

pub type Cfdcdtct = crate::RegValueT<Cfdcdtct_SPEC>;

impl Cfdcdtct {
    #[inline(always)]
    pub fn rfdmae0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdcdtct::Rfdmae0,
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
            cfdcdtct::Rfdmae0,
            Cfdcdtct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdmae1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdcdtct::Rfdmae1,
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
            cfdcdtct::Rfdmae1,
            Cfdcdtct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdmae(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cfdcdtct::Cfdmae,
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
            cfdcdtct::Cfdmae,
            Cfdcdtct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<9, 0x7fffff, 1, 0, u32, u32, Cfdcdtct_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x7fffff,1,0,u32,u32,Cfdcdtct_SPEC,crate::common::RW>::from_register(self,0)
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdmae1_SPEC;
    pub type Rfdmae1 = crate::EnumBitfieldStruct<u8, Rfdmae1_SPEC>;
    impl Rfdmae1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdmae_SPEC;
    pub type Cfdmae = crate::EnumBitfieldStruct<u8, Cfdmae_SPEC>;
    impl Cfdmae {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcdtsts_SPEC;
impl crate::sealed::RegSpec for Cfdcdtsts_SPEC {
    type DataType = u32;
}

pub type Cfdcdtsts = crate::RegValueT<Cfdcdtsts_SPEC>;

impl Cfdcdtsts {
    #[inline(always)]
    pub fn rfdmasts0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdcdtsts::Rfdmasts0,
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
            cfdcdtsts::Rfdmasts0,
            Cfdcdtsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdmasts1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdcdtsts::Rfdmasts1,
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
            cfdcdtsts::Rfdmasts1,
            Cfdcdtsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdmasts(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cfdcdtsts::Cfdmasts,
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
            cfdcdtsts::Cfdmasts,
            Cfdcdtsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<9, 0x7fffff, 1, 0, u32, u32, Cfdcdtsts_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x7fffff,1,0,u32,u32,Cfdcdtsts_SPEC,crate::common::RW>::from_register(self,0)
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdmasts1_SPEC;
    pub type Rfdmasts1 = crate::EnumBitfieldStruct<u8, Rfdmasts1_SPEC>;
    impl Rfdmasts1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdmasts_SPEC;
    pub type Cfdmasts = crate::EnumBitfieldStruct<u8, Cfdmasts_SPEC>;
    impl Cfdmasts {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgpflectr_SPEC;
impl crate::sealed::RegSpec for Cfdgpflectr_SPEC {
    type DataType = u32;
}

pub type Cfdgpflectr = crate::RegValueT<Cfdgpflectr_SPEC>;

impl Cfdgpflectr {
    #[inline(always)]
    pub fn pfldae(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cfdgpflectr::Pfldae,
        cfdgpflectr::Pfldae,
        Cfdgpflectr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cfdgpflectr::Pfldae,
            cfdgpflectr::Pfldae,
            Cfdgpflectr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x7fffff,
        1,
        0,
        u32,
        u32,
        Cfdgpflectr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x7fffff,
            1,
            0,
            u32,
            u32,
            Cfdgpflectr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdgpflectr {
    #[inline(always)]
    fn default() -> Cfdgpflectr {
        <crate::RegValueT<Cfdgpflectr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdgpflectr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfldae_SPEC;
    pub type Pfldae = crate::EnumBitfieldStruct<u8, Pfldae_SPEC>;
    impl Pfldae {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgpflcfg_SPEC;
impl crate::sealed::RegSpec for Cfdgpflcfg_SPEC {
    type DataType = u32;
}

pub type Cfdgpflcfg = crate::RegValueT<Cfdgpflcfg_SPEC>;

impl Cfdgpflcfg {
    #[inline(always)]
    pub fn rnc(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, u8, Cfdgpflcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x3,1,0,u8,u8,Cfdgpflcfg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<26, 0x3f, 1, 0, u8, u8, Cfdgpflcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x3f,1,0,u8,u8,Cfdgpflcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdgpflcfg {
    #[inline(always)]
    fn default() -> Cfdgpflcfg {
        <crate::RegValueT<Cfdgpflcfg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgrstc_SPEC;
impl crate::sealed::RegSpec for Cfdgrstc_SPEC {
    type DataType = u32;
}

pub type Cfdgrstc = crate::RegValueT<Cfdgrstc_SPEC>;

impl Cfdgrstc {
    #[inline(always)]
    pub fn srst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdgrstc::Srst,
        cfdgrstc::Srst,
        Cfdgrstc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdgrstc::Srst,
            cfdgrstc::Srst,
            Cfdgrstc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdgrstc_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdgrstc_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Cfdgrstc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Cfdgrstc_SPEC,crate::common::RW>::from_register(self,0)
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdc0Dcfg_SPEC;
impl crate::sealed::RegSpec for Cfdc0Dcfg_SPEC {
    type DataType = u32;
}

pub type Cfdc0Dcfg = crate::RegValueT<Cfdc0Dcfg_SPEC>;

impl Cfdc0Dcfg {
    #[inline(always)]
    pub fn dbrp(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdc0Dcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdc0Dcfg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dtseg1(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1f,
        1,
        0,
        cfdc0dcfg::Dtseg1,
        cfdc0dcfg::Dtseg1,
        Cfdc0Dcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1f,
            1,
            0,
            cfdc0dcfg::Dtseg1,
            cfdc0dcfg::Dtseg1,
            Cfdc0Dcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dtseg2(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xf,
        1,
        0,
        cfdc0dcfg::Dtseg2,
        cfdc0dcfg::Dtseg2,
        Cfdc0Dcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xf,
            1,
            0,
            cfdc0dcfg::Dtseg2,
            cfdc0dcfg::Dtseg2,
            Cfdc0Dcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dsjw(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xf,
        1,
        0,
        cfdc0dcfg::Dsjw,
        cfdc0dcfg::Dsjw,
        Cfdc0Dcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xf,
            1,
            0,
            cfdc0dcfg::Dsjw,
            cfdc0dcfg::Dsjw,
            Cfdc0Dcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, u8, Cfdc0Dcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0xf,1,0,u8,u8,Cfdc0Dcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdc0Dcfg {
    #[inline(always)]
    fn default() -> Cfdc0Dcfg {
        <crate::RegValueT<Cfdc0Dcfg_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdc0dcfg {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtseg1_SPEC;
    pub type Dtseg1 = crate::EnumBitfieldStruct<u8, Dtseg1_SPEC>;
    impl Dtseg1 {
        pub const _00000: Self = Self::new(0);

        pub const _00001: Self = Self::new(1);

        pub const _11110: Self = Self::new(30);

        pub const _11111: Self = Self::new(31);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtseg2_SPEC;
    pub type Dtseg2 = crate::EnumBitfieldStruct<u8, Dtseg2_SPEC>;
    impl Dtseg2 {
        pub const _0000: Self = Self::new(0);

        pub const _0001: Self = Self::new(1);

        pub const _1110: Self = Self::new(14);

        pub const _1111: Self = Self::new(15);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dsjw_SPEC;
    pub type Dsjw = crate::EnumBitfieldStruct<u8, Dsjw_SPEC>;
    impl Dsjw {
        pub const _0000: Self = Self::new(0);

        pub const _0001: Self = Self::new(1);

        pub const _1110: Self = Self::new(14);

        pub const _1111: Self = Self::new(15);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdc0Fdcfg_SPEC;
impl crate::sealed::RegSpec for Cfdc0Fdcfg_SPEC {
    type DataType = u32;
}

pub type Cfdc0Fdcfg = crate::RegValueT<Cfdc0Fdcfg_SPEC>;

impl Cfdc0Fdcfg {
    #[inline(always)]
    pub fn eoccfg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        cfdc0fdcfg::Eoccfg,
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
            cfdc0fdcfg::Eoccfg,
            Cfdc0Fdcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tdcoc(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cfdc0fdcfg::Tdcoc,
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
            cfdc0fdcfg::Tdcoc,
            Cfdc0Fdcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tdce(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        cfdc0fdcfg::Tdce,
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
            cfdc0fdcfg::Tdce,
            Cfdc0Fdcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn esic(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        cfdc0fdcfg::Esic,
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
            cfdc0fdcfg::Esic,
            Cfdc0Fdcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rpnmd(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        cfdc0fdcfg::Rpnmd,
        cfdc0fdcfg::Rpnmd,
        Cfdc0Fdcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            cfdc0fdcfg::Rpnmd,
            cfdc0fdcfg::Rpnmd,
            Cfdc0Fdcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tdco(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdc0Fdcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdc0Fdcfg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, u8, Cfdc0Fdcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xf,1,0,u8,u8,Cfdc0Fdcfg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn fdoe(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        cfdc0fdcfg::Fdoe,
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
            cfdc0fdcfg::Fdoe,
            Cfdc0Fdcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn refe(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        cfdc0fdcfg::Refe,
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
            cfdc0fdcfg::Refe,
            Cfdc0Fdcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cloe(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        cfdc0fdcfg::Cloe,
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
            cfdc0fdcfg::Cloe,
            Cfdc0Fdcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdte(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        cfdc0fdcfg::Cfdte,
        cfdc0fdcfg::Cfdte,
        Cfdc0Fdcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            cfdc0fdcfg::Cfdte,
            cfdc0fdcfg::Cfdte,
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
    pub struct Tdcoc_SPEC;
    pub type Tdcoc = crate::EnumBitfieldStruct<u8, Tdcoc_SPEC>;
    impl Tdcoc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdce_SPEC;
    pub type Tdce = crate::EnumBitfieldStruct<u8, Tdce_SPEC>;
    impl Tdce {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Esic_SPEC;
    pub type Esic = crate::EnumBitfieldStruct<u8, Esic_SPEC>;
    impl Esic {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpnmd_SPEC;
    pub type Rpnmd = crate::EnumBitfieldStruct<u8, Rpnmd_SPEC>;
    impl Rpnmd {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fdoe_SPEC;
    pub type Fdoe = crate::EnumBitfieldStruct<u8, Fdoe_SPEC>;
    impl Fdoe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Refe_SPEC;
    pub type Refe = crate::EnumBitfieldStruct<u8, Refe_SPEC>;
    impl Refe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cloe_SPEC;
    pub type Cloe = crate::EnumBitfieldStruct<u8, Cloe_SPEC>;
    impl Cloe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdte_SPEC;
    pub type Cfdte = crate::EnumBitfieldStruct<u8, Cfdte_SPEC>;
    impl Cfdte {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdc0Fdctr_SPEC;
impl crate::sealed::RegSpec for Cfdc0Fdctr_SPEC {
    type DataType = u32;
}

pub type Cfdc0Fdctr = crate::RegValueT<Cfdc0Fdctr_SPEC>;

impl Cfdc0Fdctr {
    #[inline(always)]
    pub fn eocclr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdc0fdctr::Eocclr,
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
            cfdc0fdctr::Eocclr,
            Cfdc0Fdctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn socclr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdc0fdctr::Socclr,
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
            cfdc0fdctr::Socclr,
            Cfdc0Fdctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pnmdc(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, u8, Cfdc0Fdctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3,1,0,u8,u8,Cfdc0Fdctr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<18, 0x3f, 1, 0, u8, u8, Cfdc0Fdctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x3f,1,0,u8,u8,Cfdc0Fdctr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdc0Fdctr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdc0Fdctr_SPEC,crate::common::W>::from_register(self,0)
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Socclr_SPEC;
    pub type Socclr = crate::EnumBitfieldStruct<u8, Socclr_SPEC>;
    impl Socclr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdc0Fdsts_SPEC;
impl crate::sealed::RegSpec for Cfdc0Fdsts_SPEC {
    type DataType = u32;
}

pub type Cfdc0Fdsts = crate::RegValueT<Cfdc0Fdsts_SPEC>;

impl Cfdc0Fdsts {
    #[inline(always)]
    pub fn tdcr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        cfdc0fdsts::Tdcr,
        cfdc0fdsts::Tdcr,
        Cfdc0Fdsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            cfdc0fdsts::Tdcr,
            cfdc0fdsts::Tdcr,
            Cfdc0Fdsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn eoco(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cfdc0fdsts::Eoco,
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
            cfdc0fdsts::Eoco,
            Cfdc0Fdsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn soco(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        cfdc0fdsts::Soco,
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
            cfdc0fdsts::Soco,
            Cfdc0Fdsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pnsts(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        cfdc0fdsts::Pnsts,
        cfdc0fdsts::Pnsts,
        Cfdc0Fdsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            cfdc0fdsts::Pnsts,
            cfdc0fdsts::Pnsts,
            Cfdc0Fdsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Cfdc0Fdsts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,Cfdc0Fdsts_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tdcvf(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        cfdc0fdsts::Tdcvf,
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
            cfdc0fdsts::Tdcvf,
            Cfdc0Fdsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn eoc(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdc0Fdsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdc0Fdsts_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soc(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdc0Fdsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdc0Fdsts_SPEC,crate::common::R>::from_register(self,0)
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
    pub struct Tdcr_SPEC;
    pub type Tdcr = crate::EnumBitfieldStruct<u8, Tdcr_SPEC>;
    impl Tdcr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eoco_SPEC;
    pub type Eoco = crate::EnumBitfieldStruct<u8, Eoco_SPEC>;
    impl Eoco {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Soco_SPEC;
    pub type Soco = crate::EnumBitfieldStruct<u8, Soco_SPEC>;
    impl Soco {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pnsts_SPEC;
    pub type Pnsts = crate::EnumBitfieldStruct<u8, Pnsts_SPEC>;
    impl Pnsts {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdcvf_SPEC;
    pub type Tdcvf = crate::EnumBitfieldStruct<u8, Tdcvf_SPEC>;
    impl Tdcvf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdc0Fdcrc_SPEC;
impl crate::sealed::RegSpec for Cfdc0Fdcrc_SPEC {
    type DataType = u32;
}

pub type Cfdc0Fdcrc = crate::RegValueT<Cfdc0Fdcrc_SPEC>;

impl Cfdc0Fdcrc {
    #[inline(always)]
    pub fn crcreg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1fffff,
        1,
        0,
        cfdc0fdcrc::Crcreg,
        cfdc0fdcrc::Crcreg,
        Cfdc0Fdcrc_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1fffff,
            1,
            0,
            cfdc0fdcrc::Crcreg,
            cfdc0fdcrc::Crcreg,
            Cfdc0Fdcrc_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn scnt(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, u8, Cfdc0Fdcrc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xf,1,0,u8,u8,Cfdc0Fdcrc_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, u8, Cfdc0Fdcrc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0xf,1,0,u8,u8,Cfdc0Fdcrc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdc0Fdcrc {
    #[inline(always)]
    fn default() -> Cfdc0Fdcrc {
        <crate::RegValueT<Cfdc0Fdcrc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdc0fdcrc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Crcreg_SPEC;
    pub type Crcreg = crate::EnumBitfieldStruct<u8, Crcreg_SPEC>;
    impl Crcreg {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgaflid_SPEC;
impl crate::sealed::RegSpec for Cfdgaflid_SPEC {
    type DataType = u32;
}

pub type Cfdgaflid = crate::RegValueT<Cfdgaflid_SPEC>;

impl Cfdgaflid {
    #[inline(always)]
    pub fn gaflid(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1fffffff,
        1,
        0,
        u32,
        u32,
        Cfdgaflid_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1fffffff,
            1,
            0,
            u32,
            u32,
            Cfdgaflid_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gafllb(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        cfdgaflid::Gafllb,
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
            cfdgaflid::Gafllb,
            Cfdgaflid_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gaflrtr(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        cfdgaflid::Gaflrtr,
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
            cfdgaflid::Gaflrtr,
            Cfdgaflid_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gaflide(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        cfdgaflid::Gaflide,
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gaflrtr_SPEC;
    pub type Gaflrtr = crate::EnumBitfieldStruct<u8, Gaflrtr_SPEC>;
    impl Gaflrtr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gaflide_SPEC;
    pub type Gaflide = crate::EnumBitfieldStruct<u8, Gaflide_SPEC>;
    impl Gaflide {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgaflm_SPEC;
impl crate::sealed::RegSpec for Cfdgaflm_SPEC {
    type DataType = u32;
}

pub type Cfdgaflm = crate::RegValueT<Cfdgaflm_SPEC>;

impl Cfdgaflm {
    #[inline(always)]
    pub fn gaflidm(
        self,
    ) -> crate::common::RegisterField<0, 0x1fffffff, 1, 0, u32, u32, Cfdgaflm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1fffffff,
            1,
            0,
            u32,
            u32,
            Cfdgaflm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gaflifl1(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Cfdgaflm_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,Cfdgaflm_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gaflrtrm(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        cfdgaflm::Gaflrtrm,
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
            cfdgaflm::Gaflrtrm,
            Cfdgaflm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gaflidem(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        cfdgaflm::Gaflidem,
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gaflidem_SPEC;
    pub type Gaflidem = crate::EnumBitfieldStruct<u8, Gaflidem_SPEC>;
    impl Gaflidem {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgaflp0_SPEC;
impl crate::sealed::RegSpec for Cfdgaflp0_SPEC {
    type DataType = u32;
}

pub type Cfdgaflp0 = crate::RegValueT<Cfdgaflp0_SPEC>;

impl Cfdgaflp0 {
    #[inline(always)]
    pub fn gafldlc(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Cfdgaflp0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Cfdgaflp0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gaflifl0(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Cfdgaflp0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Cfdgaflp0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gaflrmdp(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, Cfdgaflp0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,Cfdgaflp0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, u8, Cfdgaflp0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x3,1,0,u8,u8,Cfdgaflp0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gaflrmv(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        cfdgaflp0::Gaflrmv,
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
            cfdgaflp0::Gaflrmv,
            Cfdgaflp0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gaflptr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        cfdgaflp0::Gaflptr,
        cfdgaflp0::Gaflptr,
        Cfdgaflp0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            cfdgaflp0::Gaflptr,
            cfdgaflp0::Gaflptr,
            Cfdgaflp0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gaflptr_SPEC;
    pub type Gaflptr = crate::EnumBitfieldStruct<u8, Gaflptr_SPEC>;
    impl Gaflptr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgaflp1_SPEC;
impl crate::sealed::RegSpec for Cfdgaflp1_SPEC {
    type DataType = u32;
}

pub type Cfdgaflp1 = crate::RegValueT<Cfdgaflp1_SPEC>;

impl Cfdgaflp1 {
    #[inline(always)]
    pub fn gaflfdp0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Cfdgaflp1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Cfdgaflp1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gaflfdp1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Cfdgaflp1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Cfdgaflp1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gaflfdp8(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Cfdgaflp1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Cfdgaflp1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<9, 0x7fffff, 1, 0, u32, u32, Cfdgaflp1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x7fffff,1,0,u32,u32,Cfdgaflp1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdgaflp1 {
    #[inline(always)]
    fn default() -> Cfdgaflp1 {
        <crate::RegValueT<Cfdgaflp1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgpflid_SPEC;
impl crate::sealed::RegSpec for Cfdgpflid_SPEC {
    type DataType = u32;
}

pub type Cfdgpflid = crate::RegValueT<Cfdgpflid_SPEC>;

impl Cfdgpflid {
    #[inline(always)]
    pub fn gpflid(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1fffffff,
        1,
        0,
        u32,
        u32,
        Cfdgpflid_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1fffffff,
            1,
            0,
            u32,
            u32,
            Cfdgpflid_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gpfllb(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        cfdgpflid::Gpfllb,
        cfdgpflid::Gpfllb,
        Cfdgpflid_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            cfdgpflid::Gpfllb,
            cfdgpflid::Gpfllb,
            Cfdgpflid_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gpflrtr(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        cfdgpflid::Gpflrtr,
        cfdgpflid::Gpflrtr,
        Cfdgpflid_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            cfdgpflid::Gpflrtr,
            cfdgpflid::Gpflrtr,
            Cfdgpflid_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gpflide(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        cfdgpflid::Gpflide,
        cfdgpflid::Gpflide,
        Cfdgpflid_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            cfdgpflid::Gpflide,
            cfdgpflid::Gpflide,
            Cfdgpflid_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdgpflid {
    #[inline(always)]
    fn default() -> Cfdgpflid {
        <crate::RegValueT<Cfdgpflid_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdgpflid {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gpfllb_SPEC;
    pub type Gpfllb = crate::EnumBitfieldStruct<u8, Gpfllb_SPEC>;
    impl Gpfllb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gpflrtr_SPEC;
    pub type Gpflrtr = crate::EnumBitfieldStruct<u8, Gpflrtr_SPEC>;
    impl Gpflrtr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gpflide_SPEC;
    pub type Gpflide = crate::EnumBitfieldStruct<u8, Gpflide_SPEC>;
    impl Gpflide {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgpflm_SPEC;
impl crate::sealed::RegSpec for Cfdgpflm_SPEC {
    type DataType = u32;
}

pub type Cfdgpflm = crate::RegValueT<Cfdgpflm_SPEC>;

impl Cfdgpflm {
    #[inline(always)]
    pub fn gpflidm(
        self,
    ) -> crate::common::RegisterField<0, 0x1fffffff, 1, 0, u32, u32, Cfdgpflm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1fffffff,
            1,
            0,
            u32,
            u32,
            Cfdgpflm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gpflifl1(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Cfdgpflm_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,Cfdgpflm_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gpflrtrm(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        cfdgpflm::Gpflrtrm,
        cfdgpflm::Gpflrtrm,
        Cfdgpflm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            cfdgpflm::Gpflrtrm,
            cfdgpflm::Gpflrtrm,
            Cfdgpflm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gpflidem(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        cfdgpflm::Gpflidem,
        cfdgpflm::Gpflidem,
        Cfdgpflm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            cfdgpflm::Gpflidem,
            cfdgpflm::Gpflidem,
            Cfdgpflm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdgpflm {
    #[inline(always)]
    fn default() -> Cfdgpflm {
        <crate::RegValueT<Cfdgpflm_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdgpflm {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gpflrtrm_SPEC;
    pub type Gpflrtrm = crate::EnumBitfieldStruct<u8, Gpflrtrm_SPEC>;
    impl Gpflrtrm {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gpflidem_SPEC;
    pub type Gpflidem = crate::EnumBitfieldStruct<u8, Gpflidem_SPEC>;
    impl Gpflidem {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgpflp0_SPEC;
impl crate::sealed::RegSpec for Cfdgpflp0_SPEC {
    type DataType = u32;
}

pub type Cfdgpflp0 = crate::RegValueT<Cfdgpflp0_SPEC>;

impl Cfdgpflp0 {
    #[inline(always)]
    pub fn gpfldlc(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Cfdgpflp0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Cfdgpflp0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gpflifl0(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Cfdgpflp0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Cfdgpflp0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gpflrmdp(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, Cfdgpflp0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,Cfdgpflp0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, u8, Cfdgpflp0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x3,1,0,u8,u8,Cfdgpflp0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gpflrmv(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        cfdgpflp0::Gpflrmv,
        cfdgpflp0::Gpflrmv,
        Cfdgpflp0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            cfdgpflp0::Gpflrmv,
            cfdgpflp0::Gpflrmv,
            Cfdgpflp0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gpflptr(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Cfdgpflp0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Cfdgpflp0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdgpflp0 {
    #[inline(always)]
    fn default() -> Cfdgpflp0 {
        <crate::RegValueT<Cfdgpflp0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdgpflp0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gpflrmv_SPEC;
    pub type Gpflrmv = crate::EnumBitfieldStruct<u8, Gpflrmv_SPEC>;
    impl Gpflrmv {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgpflp1_SPEC;
impl crate::sealed::RegSpec for Cfdgpflp1_SPEC {
    type DataType = u32;
}

pub type Cfdgpflp1 = crate::RegValueT<Cfdgpflp1_SPEC>;

impl Cfdgpflp1 {
    #[inline(always)]
    pub fn gpflfdp0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Cfdgpflp1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Cfdgpflp1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gpflfdp1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Cfdgpflp1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Cfdgpflp1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gpflfdp8(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Cfdgpflp1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Cfdgpflp1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<9, 0x7fffff, 1, 0, u32, u32, Cfdgpflp1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x7fffff,1,0,u32,u32,Cfdgpflp1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdgpflp1 {
    #[inline(always)]
    fn default() -> Cfdgpflp1 {
        <crate::RegValueT<Cfdgpflp1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgpflpt_SPEC;
impl crate::sealed::RegSpec for Cfdgpflpt_SPEC {
    type DataType = u32;
}

pub type Cfdgpflpt = crate::RegValueT<Cfdgpflpt_SPEC>;

impl Cfdgpflpt {
    #[inline(always)]
    pub fn gpfloffset1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Cfdgpflpt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Cfdgpflpt_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gpflout1(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        cfdgpflpt::Gpflout1,
        cfdgpflpt::Gpflout1,
        Cfdgpflpt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            cfdgpflpt::Gpflout1,
            cfdgpflpt::Gpflout1,
            Cfdgpflpt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gpflrang1(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        cfdgpflpt::Gpflrang1,
        cfdgpflpt::Gpflrang1,
        Cfdgpflpt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            cfdgpflpt::Gpflrang1,
            cfdgpflpt::Gpflrang1,
            Cfdgpflpt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gpfloffset0(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Cfdgpflpt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Cfdgpflpt_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<20, 0x1ff, 1, 0, u16, u16, Cfdgpflpt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1ff,1,0,u16,u16,Cfdgpflpt_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gpflout0(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        cfdgpflpt::Gpflout0,
        cfdgpflpt::Gpflout0,
        Cfdgpflpt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            cfdgpflpt::Gpflout0,
            cfdgpflpt::Gpflout0,
            Cfdgpflpt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gpflrang0(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        cfdgpflpt::Gpflrang0,
        cfdgpflpt::Gpflrang0,
        Cfdgpflpt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            cfdgpflpt::Gpflrang0,
            cfdgpflpt::Gpflrang0,
            Cfdgpflpt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gpflandor(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        cfdgpflpt::Gpflandor,
        cfdgpflpt::Gpflandor,
        Cfdgpflpt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            cfdgpflpt::Gpflandor,
            cfdgpflpt::Gpflandor,
            Cfdgpflpt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdgpflpt {
    #[inline(always)]
    fn default() -> Cfdgpflpt {
        <crate::RegValueT<Cfdgpflpt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdgpflpt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gpflout1_SPEC;
    pub type Gpflout1 = crate::EnumBitfieldStruct<u8, Gpflout1_SPEC>;
    impl Gpflout1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gpflrang1_SPEC;
    pub type Gpflrang1 = crate::EnumBitfieldStruct<u8, Gpflrang1_SPEC>;
    impl Gpflrang1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gpflout0_SPEC;
    pub type Gpflout0 = crate::EnumBitfieldStruct<u8, Gpflout0_SPEC>;
    impl Gpflout0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gpflrang0_SPEC;
    pub type Gpflrang0 = crate::EnumBitfieldStruct<u8, Gpflrang0_SPEC>;
    impl Gpflrang0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gpflandor_SPEC;
    pub type Gpflandor = crate::EnumBitfieldStruct<u8, Gpflandor_SPEC>;
    impl Gpflandor {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgpflpd0_SPEC;
impl crate::sealed::RegSpec for Cfdgpflpd0_SPEC {
    type DataType = u32;
}

pub type Cfdgpflpd0 = crate::RegValueT<Cfdgpflpd0_SPEC>;

impl Cfdgpflpd0 {
    #[inline(always)]
    pub fn fdata(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Cfdgpflpd0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Cfdgpflpd0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdgpflpd0 {
    #[inline(always)]
    fn default() -> Cfdgpflpd0 {
        <crate::RegValueT<Cfdgpflpd0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgpflpm0_SPEC;
impl crate::sealed::RegSpec for Cfdgpflpm0_SPEC {
    type DataType = u32;
}

pub type Cfdgpflpm0 = crate::RegValueT<Cfdgpflpm0_SPEC>;

impl Cfdgpflpm0 {
    #[inline(always)]
    pub fn fmask(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Cfdgpflpm0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Cfdgpflpm0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdgpflpm0 {
    #[inline(always)]
    fn default() -> Cfdgpflpm0 {
        <crate::RegValueT<Cfdgpflpm0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgpflpd1_SPEC;
impl crate::sealed::RegSpec for Cfdgpflpd1_SPEC {
    type DataType = u32;
}

pub type Cfdgpflpd1 = crate::RegValueT<Cfdgpflpd1_SPEC>;

impl Cfdgpflpd1 {
    #[inline(always)]
    pub fn fdata(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Cfdgpflpd1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Cfdgpflpd1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdgpflpd1 {
    #[inline(always)]
    fn default() -> Cfdgpflpd1 {
        <crate::RegValueT<Cfdgpflpd1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgpflpm1_SPEC;
impl crate::sealed::RegSpec for Cfdgpflpm1_SPEC {
    type DataType = u32;
}

pub type Cfdgpflpm1 = crate::RegValueT<Cfdgpflpm1_SPEC>;

impl Cfdgpflpm1 {
    #[inline(always)]
    pub fn fmask(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Cfdgpflpm1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Cfdgpflpm1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdgpflpm1 {
    #[inline(always)]
    fn default() -> Cfdgpflpm1 {
        <crate::RegValueT<Cfdgpflpm1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrpgacc_SPEC;
impl crate::sealed::RegSpec for Cfdrpgacc_SPEC {
    type DataType = u32;
}

pub type Cfdrpgacc = crate::RegValueT<Cfdrpgacc_SPEC>;

impl Cfdrpgacc {
    #[inline(always)]
    pub fn rdta(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Cfdrpgacc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Cfdrpgacc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type Cfdrfid = crate::RegValueT<Cfdrfid_SPEC>;

impl Cfdrfid {
    #[inline(always)]
    pub fn rfid(
        self,
    ) -> crate::common::RegisterField<0, 0x1fffffff, 1, 0, u32, u32, Cfdrfid_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1fffffff,1,0,u32,u32,Cfdrfid_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Cfdrfid_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cfdrfid_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn rfrtr(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        cfdrfid::Rfrtr,
        cfdrfid::Rfrtr,
        Cfdrfid_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            cfdrfid::Rfrtr,
            cfdrfid::Rfrtr,
            Cfdrfid_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfide(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        cfdrfid::Rfide,
        cfdrfid::Rfide,
        Cfdrfid_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            cfdrfid::Rfide,
            cfdrfid::Rfide,
            Cfdrfid_SPEC,
            crate::common::R,
        >::from_register(self, 0)
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfide_SPEC;
    pub type Rfide = crate::EnumBitfieldStruct<u8, Rfide_SPEC>;
    impl Rfide {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfptr_SPEC;
impl crate::sealed::RegSpec for Cfdrfptr_SPEC {
    type DataType = u32;
}

pub type Cfdrfptr = crate::RegValueT<Cfdrfptr_SPEC>;

impl Cfdrfptr {
    #[inline(always)]
    pub fn rfts(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Cfdrfptr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Cfdrfptr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, u16, Cfdrfptr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xfff,1,0,u16,u16,Cfdrfptr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdlc(
        self,
    ) -> crate::common::RegisterField<
        28,
        0xf,
        1,
        0,
        cfdrfptr::Rfdlc,
        cfdrfptr::Rfdlc,
        Cfdrfptr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            28,
            0xf,
            1,
            0,
            cfdrfptr::Rfdlc,
            cfdrfptr::Rfdlc,
            Cfdrfptr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrfptr {
    #[inline(always)]
    fn default() -> Cfdrfptr {
        <crate::RegValueT<Cfdrfptr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrfptr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdlc_SPEC;
    pub type Rfdlc = crate::EnumBitfieldStruct<u8, Rfdlc_SPEC>;
    impl Rfdlc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrffdsts_SPEC;
impl crate::sealed::RegSpec for Cfdrffdsts_SPEC {
    type DataType = u32;
}

pub type Cfdrffdsts = crate::RegValueT<Cfdrffdsts_SPEC>;

impl Cfdrffdsts {
    #[inline(always)]
    pub fn rfesi(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdrffdsts::Rfesi,
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
            cfdrffdsts::Rfesi,
            Cfdrffdsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfbrs(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdrffdsts::Rfbrs,
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
            cfdrffdsts::Rfbrs,
            Cfdrffdsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rffdf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cfdrffdsts::Rffdf,
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
            cfdrffdsts::Rffdf,
            Cfdrffdsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfifl(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, Cfdrffdsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,Cfdrffdsts_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<10, 0x3f, 1, 0, u8, u8, Cfdrffdsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<10,0x3f,1,0,u8,u8,Cfdrffdsts_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdrfptr(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Cfdrffdsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Cfdrffdsts_SPEC,crate::common::R>::from_register(self,0)
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfbrs_SPEC;
    pub type Rfbrs = crate::EnumBitfieldStruct<u8, Rfbrs_SPEC>;
    impl Rfbrs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rffdf_SPEC;
    pub type Rffdf = crate::EnumBitfieldStruct<u8, Rffdf_SPEC>;
    impl Rffdf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfdf0_SPEC;
impl crate::sealed::RegSpec for Cfdrfdf0_SPEC {
    type DataType = u32;
}

pub type Cfdrfdf0 = crate::RegValueT<Cfdrfdf0_SPEC>;

impl Cfdrfdf0 {
    #[inline(always)]
    pub fn rfdb0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfdf0_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfdf0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb1(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrfdf0::Rfdb1,
        cfdrfdf0::Rfdb1,
        Cfdrfdf0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrfdf0::Rfdb1,
            cfdrfdf0::Rfdb1,
            Cfdrfdf0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb2(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrfdf0::Rfdb2,
        cfdrfdf0::Rfdb2,
        Cfdrfdf0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrfdf0::Rfdb2,
            cfdrfdf0::Rfdb2,
            Cfdrfdf0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb3(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrfdf0::Rfdb3,
        cfdrfdf0::Rfdb3,
        Cfdrfdf0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrfdf0::Rfdb3,
            cfdrfdf0::Rfdb3,
            Cfdrfdf0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrfdf0 {
    #[inline(always)]
    fn default() -> Cfdrfdf0 {
        <crate::RegValueT<Cfdrfdf0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrfdf0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb1_SPEC;
    pub type Rfdb1 = crate::EnumBitfieldStruct<u8, Rfdb1_SPEC>;
    impl Rfdb1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb2_SPEC;
    pub type Rfdb2 = crate::EnumBitfieldStruct<u8, Rfdb2_SPEC>;
    impl Rfdb2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb3_SPEC;
    pub type Rfdb3 = crate::EnumBitfieldStruct<u8, Rfdb3_SPEC>;
    impl Rfdb3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfdf1_SPEC;
impl crate::sealed::RegSpec for Cfdrfdf1_SPEC {
    type DataType = u32;
}

pub type Cfdrfdf1 = crate::RegValueT<Cfdrfdf1_SPEC>;

impl Cfdrfdf1 {
    #[inline(always)]
    pub fn rfdb4(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfdf1_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfdf1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb5(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrfdf1::Rfdb5,
        cfdrfdf1::Rfdb5,
        Cfdrfdf1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrfdf1::Rfdb5,
            cfdrfdf1::Rfdb5,
            Cfdrfdf1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb6(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrfdf1::Rfdb6,
        cfdrfdf1::Rfdb6,
        Cfdrfdf1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrfdf1::Rfdb6,
            cfdrfdf1::Rfdb6,
            Cfdrfdf1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb7(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrfdf1::Rfdb7,
        cfdrfdf1::Rfdb7,
        Cfdrfdf1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrfdf1::Rfdb7,
            cfdrfdf1::Rfdb7,
            Cfdrfdf1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrfdf1 {
    #[inline(always)]
    fn default() -> Cfdrfdf1 {
        <crate::RegValueT<Cfdrfdf1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrfdf1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb5_SPEC;
    pub type Rfdb5 = crate::EnumBitfieldStruct<u8, Rfdb5_SPEC>;
    impl Rfdb5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb6_SPEC;
    pub type Rfdb6 = crate::EnumBitfieldStruct<u8, Rfdb6_SPEC>;
    impl Rfdb6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb7_SPEC;
    pub type Rfdb7 = crate::EnumBitfieldStruct<u8, Rfdb7_SPEC>;
    impl Rfdb7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfid_SPEC;
impl crate::sealed::RegSpec for Cfdcfid_SPEC {
    type DataType = u32;
}

pub type Cfdcfid = crate::RegValueT<Cfdcfid_SPEC>;

impl Cfdcfid {
    #[inline(always)]
    pub fn cfid(
        self,
    ) -> crate::common::RegisterField<0, 0x1fffffff, 1, 0, u32, u32, Cfdcfid_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fffffff,1,0,u32,u32,Cfdcfid_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn thlen(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        cfdcfid::Thlen,
        cfdcfid::Thlen,
        Cfdcfid_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            cfdcfid::Thlen,
            cfdcfid::Thlen,
            Cfdcfid_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfrtr(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        cfdcfid::Cfrtr,
        cfdcfid::Cfrtr,
        Cfdcfid_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            cfdcfid::Cfrtr,
            cfdcfid::Cfrtr,
            Cfdcfid_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfide(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        cfdcfid::Cfide,
        cfdcfid::Cfide,
        Cfdcfid_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            cfdcfid::Cfide,
            cfdcfid::Cfide,
            Cfdcfid_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfrtr_SPEC;
    pub type Cfrtr = crate::EnumBitfieldStruct<u8, Cfrtr_SPEC>;
    impl Cfrtr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfide_SPEC;
    pub type Cfide = crate::EnumBitfieldStruct<u8, Cfide_SPEC>;
    impl Cfide {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfptr_SPEC;
impl crate::sealed::RegSpec for Cfdcfptr_SPEC {
    type DataType = u32;
}

pub type Cfdcfptr = crate::RegValueT<Cfdcfptr_SPEC>;

impl Cfdcfptr {
    #[inline(always)]
    pub fn cfts(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Cfdcfptr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Cfdcfptr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, u16, Cfdcfptr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xfff,1,0,u16,u16,Cfdcfptr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdlc(
        self,
    ) -> crate::common::RegisterField<
        28,
        0xf,
        1,
        0,
        cfdcfptr::Cfdlc,
        cfdcfptr::Cfdlc,
        Cfdcfptr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0xf,
            1,
            0,
            cfdcfptr::Cfdlc,
            cfdcfptr::Cfdlc,
            Cfdcfptr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcfptr {
    #[inline(always)]
    fn default() -> Cfdcfptr {
        <crate::RegValueT<Cfdcfptr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcfptr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdlc_SPEC;
    pub type Cfdlc = crate::EnumBitfieldStruct<u8, Cfdlc_SPEC>;
    impl Cfdlc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcffdcsts_SPEC;
impl crate::sealed::RegSpec for Cfdcffdcsts_SPEC {
    type DataType = u32;
}

pub type Cfdcffdcsts = crate::RegValueT<Cfdcffdcsts_SPEC>;

impl Cfdcffdcsts {
    #[inline(always)]
    pub fn cfesi(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdcffdcsts::Cfesi,
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
            cfdcffdcsts::Cfesi,
            Cfdcffdcsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfbrs(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdcffdcsts::Cfbrs,
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
            cfdcffdcsts::Cfbrs,
            Cfdcffdcsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cffdf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cfdcffdcsts::Cffdf,
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
            cfdcffdcsts::Cffdf,
            Cfdcffdcsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfifl(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, Cfdcffdcsts_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,Cfdcffdcsts_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<10, 0x3f, 1, 0, u8, u8, Cfdcffdcsts_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3f,1,0,u8,u8,Cfdcffdcsts_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfptr(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Cfdcffdcsts_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            Cfdcffdcsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfbrs_SPEC;
    pub type Cfbrs = crate::EnumBitfieldStruct<u8, Cfbrs_SPEC>;
    impl Cfbrs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cffdf_SPEC;
    pub type Cffdf = crate::EnumBitfieldStruct<u8, Cffdf_SPEC>;
    impl Cffdf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf0_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf0_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf0 = crate::RegValueT<Cfdcfdf0_SPEC>;

impl Cfdcfdf0 {
    #[inline(always)]
    pub fn cfdb0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb1(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdcfdf0::Cfdb1,
        cfdcfdf0::Cfdb1,
        Cfdcfdf0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdcfdf0::Cfdb1,
            cfdcfdf0::Cfdb1,
            Cfdcfdf0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdb2(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdcfdf0::Cfdb2,
        cfdcfdf0::Cfdb2,
        Cfdcfdf0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdcfdf0::Cfdb2,
            cfdcfdf0::Cfdb2,
            Cfdcfdf0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdb3(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdcfdf0::Cfdb3,
        cfdcfdf0::Cfdb3,
        Cfdcfdf0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdcfdf0::Cfdb3,
            cfdcfdf0::Cfdb3,
            Cfdcfdf0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcfdf0 {
    #[inline(always)]
    fn default() -> Cfdcfdf0 {
        <crate::RegValueT<Cfdcfdf0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcfdf0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb1_SPEC;
    pub type Cfdb1 = crate::EnumBitfieldStruct<u8, Cfdb1_SPEC>;
    impl Cfdb1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb2_SPEC;
    pub type Cfdb2 = crate::EnumBitfieldStruct<u8, Cfdb2_SPEC>;
    impl Cfdb2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb3_SPEC;
    pub type Cfdb3 = crate::EnumBitfieldStruct<u8, Cfdb3_SPEC>;
    impl Cfdb3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf1_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf1_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf1 = crate::RegValueT<Cfdcfdf1_SPEC>;

impl Cfdcfdf1 {
    #[inline(always)]
    pub fn cfdb4(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb5(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdcfdf1::Cfdb5,
        cfdcfdf1::Cfdb5,
        Cfdcfdf1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdcfdf1::Cfdb5,
            cfdcfdf1::Cfdb5,
            Cfdcfdf1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdb6(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdcfdf1::Cfdb6,
        cfdcfdf1::Cfdb6,
        Cfdcfdf1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdcfdf1::Cfdb6,
            cfdcfdf1::Cfdb6,
            Cfdcfdf1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdb7(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdcfdf1::Cfdb7,
        cfdcfdf1::Cfdb7,
        Cfdcfdf1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdcfdf1::Cfdb7,
            cfdcfdf1::Cfdb7,
            Cfdcfdf1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcfdf1 {
    #[inline(always)]
    fn default() -> Cfdcfdf1 {
        <crate::RegValueT<Cfdcfdf1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcfdf1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb5_SPEC;
    pub type Cfdb5 = crate::EnumBitfieldStruct<u8, Cfdb5_SPEC>;
    impl Cfdb5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb6_SPEC;
    pub type Cfdb6 = crate::EnumBitfieldStruct<u8, Cfdb6_SPEC>;
    impl Cfdb6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb7_SPEC;
    pub type Cfdb7 = crate::EnumBitfieldStruct<u8, Cfdb7_SPEC>;
    impl Cfdb7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf2_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf2_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf2 = crate::RegValueT<Cfdcfdf2_SPEC>;

impl Cfdcfdf2 {
    #[inline(always)]
    pub fn cfdb8(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb9(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdcfdf2::Cfdb9,
        cfdcfdf2::Cfdb9,
        Cfdcfdf2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdcfdf2::Cfdb9,
            cfdcfdf2::Cfdb9,
            Cfdcfdf2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdb10(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdcfdf2::Cfdb10,
        cfdcfdf2::Cfdb10,
        Cfdcfdf2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdcfdf2::Cfdb10,
            cfdcfdf2::Cfdb10,
            Cfdcfdf2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdb11(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdcfdf2::Cfdb11,
        cfdcfdf2::Cfdb11,
        Cfdcfdf2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdcfdf2::Cfdb11,
            cfdcfdf2::Cfdb11,
            Cfdcfdf2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcfdf2 {
    #[inline(always)]
    fn default() -> Cfdcfdf2 {
        <crate::RegValueT<Cfdcfdf2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcfdf2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb9_SPEC;
    pub type Cfdb9 = crate::EnumBitfieldStruct<u8, Cfdb9_SPEC>;
    impl Cfdb9 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb10_SPEC;
    pub type Cfdb10 = crate::EnumBitfieldStruct<u8, Cfdb10_SPEC>;
    impl Cfdb10 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb11_SPEC;
    pub type Cfdb11 = crate::EnumBitfieldStruct<u8, Cfdb11_SPEC>;
    impl Cfdb11 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf3_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf3_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf3 = crate::RegValueT<Cfdcfdf3_SPEC>;

impl Cfdcfdf3 {
    #[inline(always)]
    pub fn cfdb12(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb13(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdcfdf3::Cfdb13,
        cfdcfdf3::Cfdb13,
        Cfdcfdf3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdcfdf3::Cfdb13,
            cfdcfdf3::Cfdb13,
            Cfdcfdf3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdb14(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdcfdf3::Cfdb14,
        cfdcfdf3::Cfdb14,
        Cfdcfdf3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdcfdf3::Cfdb14,
            cfdcfdf3::Cfdb14,
            Cfdcfdf3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdb15(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdcfdf3::Cfdb15,
        cfdcfdf3::Cfdb15,
        Cfdcfdf3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdcfdf3::Cfdb15,
            cfdcfdf3::Cfdb15,
            Cfdcfdf3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcfdf3 {
    #[inline(always)]
    fn default() -> Cfdcfdf3 {
        <crate::RegValueT<Cfdcfdf3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcfdf3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb13_SPEC;
    pub type Cfdb13 = crate::EnumBitfieldStruct<u8, Cfdb13_SPEC>;
    impl Cfdb13 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb14_SPEC;
    pub type Cfdb14 = crate::EnumBitfieldStruct<u8, Cfdb14_SPEC>;
    impl Cfdb14 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb15_SPEC;
    pub type Cfdb15 = crate::EnumBitfieldStruct<u8, Cfdb15_SPEC>;
    impl Cfdb15 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf4_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf4_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf4 = crate::RegValueT<Cfdcfdf4_SPEC>;

impl Cfdcfdf4 {
    #[inline(always)]
    pub fn cfdb16(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb17(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdcfdf4::Cfdb17,
        cfdcfdf4::Cfdb17,
        Cfdcfdf4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdcfdf4::Cfdb17,
            cfdcfdf4::Cfdb17,
            Cfdcfdf4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdb18(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdcfdf4::Cfdb18,
        cfdcfdf4::Cfdb18,
        Cfdcfdf4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdcfdf4::Cfdb18,
            cfdcfdf4::Cfdb18,
            Cfdcfdf4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdb19(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdcfdf4::Cfdb19,
        cfdcfdf4::Cfdb19,
        Cfdcfdf4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdcfdf4::Cfdb19,
            cfdcfdf4::Cfdb19,
            Cfdcfdf4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcfdf4 {
    #[inline(always)]
    fn default() -> Cfdcfdf4 {
        <crate::RegValueT<Cfdcfdf4_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcfdf4 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb17_SPEC;
    pub type Cfdb17 = crate::EnumBitfieldStruct<u8, Cfdb17_SPEC>;
    impl Cfdb17 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb18_SPEC;
    pub type Cfdb18 = crate::EnumBitfieldStruct<u8, Cfdb18_SPEC>;
    impl Cfdb18 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb19_SPEC;
    pub type Cfdb19 = crate::EnumBitfieldStruct<u8, Cfdb19_SPEC>;
    impl Cfdb19 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf5_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf5_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf5 = crate::RegValueT<Cfdcfdf5_SPEC>;

impl Cfdcfdf5 {
    #[inline(always)]
    pub fn cfdb20(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb21(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdcfdf5::Cfdb21,
        cfdcfdf5::Cfdb21,
        Cfdcfdf5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdcfdf5::Cfdb21,
            cfdcfdf5::Cfdb21,
            Cfdcfdf5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdb22(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdcfdf5::Cfdb22,
        cfdcfdf5::Cfdb22,
        Cfdcfdf5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdcfdf5::Cfdb22,
            cfdcfdf5::Cfdb22,
            Cfdcfdf5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdb23(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdcfdf5::Cfdb23,
        cfdcfdf5::Cfdb23,
        Cfdcfdf5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdcfdf5::Cfdb23,
            cfdcfdf5::Cfdb23,
            Cfdcfdf5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcfdf5 {
    #[inline(always)]
    fn default() -> Cfdcfdf5 {
        <crate::RegValueT<Cfdcfdf5_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcfdf5 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb21_SPEC;
    pub type Cfdb21 = crate::EnumBitfieldStruct<u8, Cfdb21_SPEC>;
    impl Cfdb21 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb22_SPEC;
    pub type Cfdb22 = crate::EnumBitfieldStruct<u8, Cfdb22_SPEC>;
    impl Cfdb22 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb23_SPEC;
    pub type Cfdb23 = crate::EnumBitfieldStruct<u8, Cfdb23_SPEC>;
    impl Cfdb23 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf6_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf6_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf6 = crate::RegValueT<Cfdcfdf6_SPEC>;

impl Cfdcfdf6 {
    #[inline(always)]
    pub fn cfdb24(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf6_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb25(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdcfdf6::Cfdb25,
        cfdcfdf6::Cfdb25,
        Cfdcfdf6_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdcfdf6::Cfdb25,
            cfdcfdf6::Cfdb25,
            Cfdcfdf6_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdb26(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdcfdf6::Cfdb26,
        cfdcfdf6::Cfdb26,
        Cfdcfdf6_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdcfdf6::Cfdb26,
            cfdcfdf6::Cfdb26,
            Cfdcfdf6_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdb27(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdcfdf6::Cfdb27,
        cfdcfdf6::Cfdb27,
        Cfdcfdf6_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdcfdf6::Cfdb27,
            cfdcfdf6::Cfdb27,
            Cfdcfdf6_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcfdf6 {
    #[inline(always)]
    fn default() -> Cfdcfdf6 {
        <crate::RegValueT<Cfdcfdf6_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcfdf6 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb25_SPEC;
    pub type Cfdb25 = crate::EnumBitfieldStruct<u8, Cfdb25_SPEC>;
    impl Cfdb25 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb26_SPEC;
    pub type Cfdb26 = crate::EnumBitfieldStruct<u8, Cfdb26_SPEC>;
    impl Cfdb26 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb27_SPEC;
    pub type Cfdb27 = crate::EnumBitfieldStruct<u8, Cfdb27_SPEC>;
    impl Cfdb27 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf7_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf7_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf7 = crate::RegValueT<Cfdcfdf7_SPEC>;

impl Cfdcfdf7 {
    #[inline(always)]
    pub fn cfdb28(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf7_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb29(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdcfdf7::Cfdb29,
        cfdcfdf7::Cfdb29,
        Cfdcfdf7_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdcfdf7::Cfdb29,
            cfdcfdf7::Cfdb29,
            Cfdcfdf7_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdb30(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdcfdf7::Cfdb30,
        cfdcfdf7::Cfdb30,
        Cfdcfdf7_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdcfdf7::Cfdb30,
            cfdcfdf7::Cfdb30,
            Cfdcfdf7_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdb31(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdcfdf7::Cfdb31,
        cfdcfdf7::Cfdb31,
        Cfdcfdf7_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdcfdf7::Cfdb31,
            cfdcfdf7::Cfdb31,
            Cfdcfdf7_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcfdf7 {
    #[inline(always)]
    fn default() -> Cfdcfdf7 {
        <crate::RegValueT<Cfdcfdf7_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcfdf7 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb29_SPEC;
    pub type Cfdb29 = crate::EnumBitfieldStruct<u8, Cfdb29_SPEC>;
    impl Cfdb29 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb30_SPEC;
    pub type Cfdb30 = crate::EnumBitfieldStruct<u8, Cfdb30_SPEC>;
    impl Cfdb30 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb31_SPEC;
    pub type Cfdb31 = crate::EnumBitfieldStruct<u8, Cfdb31_SPEC>;
    impl Cfdb31 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf8_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf8_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf8 = crate::RegValueT<Cfdcfdf8_SPEC>;

impl Cfdcfdf8 {
    #[inline(always)]
    pub fn cfdb32(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf8_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb33(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdcfdf8::Cfdb33,
        cfdcfdf8::Cfdb33,
        Cfdcfdf8_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdcfdf8::Cfdb33,
            cfdcfdf8::Cfdb33,
            Cfdcfdf8_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdb34(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdcfdf8::Cfdb34,
        cfdcfdf8::Cfdb34,
        Cfdcfdf8_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdcfdf8::Cfdb34,
            cfdcfdf8::Cfdb34,
            Cfdcfdf8_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdb35(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdcfdf8::Cfdb35,
        cfdcfdf8::Cfdb35,
        Cfdcfdf8_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdcfdf8::Cfdb35,
            cfdcfdf8::Cfdb35,
            Cfdcfdf8_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcfdf8 {
    #[inline(always)]
    fn default() -> Cfdcfdf8 {
        <crate::RegValueT<Cfdcfdf8_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcfdf8 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb33_SPEC;
    pub type Cfdb33 = crate::EnumBitfieldStruct<u8, Cfdb33_SPEC>;
    impl Cfdb33 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb34_SPEC;
    pub type Cfdb34 = crate::EnumBitfieldStruct<u8, Cfdb34_SPEC>;
    impl Cfdb34 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb35_SPEC;
    pub type Cfdb35 = crate::EnumBitfieldStruct<u8, Cfdb35_SPEC>;
    impl Cfdb35 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf9_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf9_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf9 = crate::RegValueT<Cfdcfdf9_SPEC>;

impl Cfdcfdf9 {
    #[inline(always)]
    pub fn cfdb36(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf9_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb37(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdcfdf9::Cfdb37,
        cfdcfdf9::Cfdb37,
        Cfdcfdf9_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdcfdf9::Cfdb37,
            cfdcfdf9::Cfdb37,
            Cfdcfdf9_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdb38(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdcfdf9::Cfdb38,
        cfdcfdf9::Cfdb38,
        Cfdcfdf9_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdcfdf9::Cfdb38,
            cfdcfdf9::Cfdb38,
            Cfdcfdf9_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdb39(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdcfdf9::Cfdb39,
        cfdcfdf9::Cfdb39,
        Cfdcfdf9_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdcfdf9::Cfdb39,
            cfdcfdf9::Cfdb39,
            Cfdcfdf9_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcfdf9 {
    #[inline(always)]
    fn default() -> Cfdcfdf9 {
        <crate::RegValueT<Cfdcfdf9_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcfdf9 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb37_SPEC;
    pub type Cfdb37 = crate::EnumBitfieldStruct<u8, Cfdb37_SPEC>;
    impl Cfdb37 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb38_SPEC;
    pub type Cfdb38 = crate::EnumBitfieldStruct<u8, Cfdb38_SPEC>;
    impl Cfdb38 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb39_SPEC;
    pub type Cfdb39 = crate::EnumBitfieldStruct<u8, Cfdb39_SPEC>;
    impl Cfdb39 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf10_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf10_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf10 = crate::RegValueT<Cfdcfdf10_SPEC>;

impl Cfdcfdf10 {
    #[inline(always)]
    pub fn cfdb40(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf10_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb41(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdcfdf10::Cfdb41,
        cfdcfdf10::Cfdb41,
        Cfdcfdf10_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdcfdf10::Cfdb41,
            cfdcfdf10::Cfdb41,
            Cfdcfdf10_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdb42(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdcfdf10::Cfdb42,
        cfdcfdf10::Cfdb42,
        Cfdcfdf10_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdcfdf10::Cfdb42,
            cfdcfdf10::Cfdb42,
            Cfdcfdf10_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdb43(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdcfdf10::Cfdb43,
        cfdcfdf10::Cfdb43,
        Cfdcfdf10_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdcfdf10::Cfdb43,
            cfdcfdf10::Cfdb43,
            Cfdcfdf10_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcfdf10 {
    #[inline(always)]
    fn default() -> Cfdcfdf10 {
        <crate::RegValueT<Cfdcfdf10_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcfdf10 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb41_SPEC;
    pub type Cfdb41 = crate::EnumBitfieldStruct<u8, Cfdb41_SPEC>;
    impl Cfdb41 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb42_SPEC;
    pub type Cfdb42 = crate::EnumBitfieldStruct<u8, Cfdb42_SPEC>;
    impl Cfdb42 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb43_SPEC;
    pub type Cfdb43 = crate::EnumBitfieldStruct<u8, Cfdb43_SPEC>;
    impl Cfdb43 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf11_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf11_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf11 = crate::RegValueT<Cfdcfdf11_SPEC>;

impl Cfdcfdf11 {
    #[inline(always)]
    pub fn cfdb44(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf11_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf11_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb45(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdcfdf11::Cfdb45,
        cfdcfdf11::Cfdb45,
        Cfdcfdf11_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdcfdf11::Cfdb45,
            cfdcfdf11::Cfdb45,
            Cfdcfdf11_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdb46(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdcfdf11::Cfdb46,
        cfdcfdf11::Cfdb46,
        Cfdcfdf11_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdcfdf11::Cfdb46,
            cfdcfdf11::Cfdb46,
            Cfdcfdf11_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdb47(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdcfdf11::Cfdb47,
        cfdcfdf11::Cfdb47,
        Cfdcfdf11_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdcfdf11::Cfdb47,
            cfdcfdf11::Cfdb47,
            Cfdcfdf11_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcfdf11 {
    #[inline(always)]
    fn default() -> Cfdcfdf11 {
        <crate::RegValueT<Cfdcfdf11_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcfdf11 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb45_SPEC;
    pub type Cfdb45 = crate::EnumBitfieldStruct<u8, Cfdb45_SPEC>;
    impl Cfdb45 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb46_SPEC;
    pub type Cfdb46 = crate::EnumBitfieldStruct<u8, Cfdb46_SPEC>;
    impl Cfdb46 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb47_SPEC;
    pub type Cfdb47 = crate::EnumBitfieldStruct<u8, Cfdb47_SPEC>;
    impl Cfdb47 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf12_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf12_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf12 = crate::RegValueT<Cfdcfdf12_SPEC>;

impl Cfdcfdf12 {
    #[inline(always)]
    pub fn cfdb48(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf12_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf12_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb49(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdcfdf12::Cfdb49,
        cfdcfdf12::Cfdb49,
        Cfdcfdf12_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdcfdf12::Cfdb49,
            cfdcfdf12::Cfdb49,
            Cfdcfdf12_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdb50(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdcfdf12::Cfdb50,
        cfdcfdf12::Cfdb50,
        Cfdcfdf12_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdcfdf12::Cfdb50,
            cfdcfdf12::Cfdb50,
            Cfdcfdf12_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdb51(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdcfdf12::Cfdb51,
        cfdcfdf12::Cfdb51,
        Cfdcfdf12_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdcfdf12::Cfdb51,
            cfdcfdf12::Cfdb51,
            Cfdcfdf12_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcfdf12 {
    #[inline(always)]
    fn default() -> Cfdcfdf12 {
        <crate::RegValueT<Cfdcfdf12_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcfdf12 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb49_SPEC;
    pub type Cfdb49 = crate::EnumBitfieldStruct<u8, Cfdb49_SPEC>;
    impl Cfdb49 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb50_SPEC;
    pub type Cfdb50 = crate::EnumBitfieldStruct<u8, Cfdb50_SPEC>;
    impl Cfdb50 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb51_SPEC;
    pub type Cfdb51 = crate::EnumBitfieldStruct<u8, Cfdb51_SPEC>;
    impl Cfdb51 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf13_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf13_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf13 = crate::RegValueT<Cfdcfdf13_SPEC>;

impl Cfdcfdf13 {
    #[inline(always)]
    pub fn cfdb52(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf13_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf13_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb53(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdcfdf13::Cfdb53,
        cfdcfdf13::Cfdb53,
        Cfdcfdf13_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdcfdf13::Cfdb53,
            cfdcfdf13::Cfdb53,
            Cfdcfdf13_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdb54(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdcfdf13::Cfdb54,
        cfdcfdf13::Cfdb54,
        Cfdcfdf13_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdcfdf13::Cfdb54,
            cfdcfdf13::Cfdb54,
            Cfdcfdf13_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdb55(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdcfdf13::Cfdb55,
        cfdcfdf13::Cfdb55,
        Cfdcfdf13_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdcfdf13::Cfdb55,
            cfdcfdf13::Cfdb55,
            Cfdcfdf13_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcfdf13 {
    #[inline(always)]
    fn default() -> Cfdcfdf13 {
        <crate::RegValueT<Cfdcfdf13_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcfdf13 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb53_SPEC;
    pub type Cfdb53 = crate::EnumBitfieldStruct<u8, Cfdb53_SPEC>;
    impl Cfdb53 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb54_SPEC;
    pub type Cfdb54 = crate::EnumBitfieldStruct<u8, Cfdb54_SPEC>;
    impl Cfdb54 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb55_SPEC;
    pub type Cfdb55 = crate::EnumBitfieldStruct<u8, Cfdb55_SPEC>;
    impl Cfdb55 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf14_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf14_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf14 = crate::RegValueT<Cfdcfdf14_SPEC>;

impl Cfdcfdf14 {
    #[inline(always)]
    pub fn cfdb56(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf14_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf14_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb57(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdcfdf14::Cfdb57,
        cfdcfdf14::Cfdb57,
        Cfdcfdf14_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdcfdf14::Cfdb57,
            cfdcfdf14::Cfdb57,
            Cfdcfdf14_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdb58(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdcfdf14::Cfdb58,
        cfdcfdf14::Cfdb58,
        Cfdcfdf14_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdcfdf14::Cfdb58,
            cfdcfdf14::Cfdb58,
            Cfdcfdf14_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdb59(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdcfdf14::Cfdb59,
        cfdcfdf14::Cfdb59,
        Cfdcfdf14_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdcfdf14::Cfdb59,
            cfdcfdf14::Cfdb59,
            Cfdcfdf14_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcfdf14 {
    #[inline(always)]
    fn default() -> Cfdcfdf14 {
        <crate::RegValueT<Cfdcfdf14_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcfdf14 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb57_SPEC;
    pub type Cfdb57 = crate::EnumBitfieldStruct<u8, Cfdb57_SPEC>;
    impl Cfdb57 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb58_SPEC;
    pub type Cfdb58 = crate::EnumBitfieldStruct<u8, Cfdb58_SPEC>;
    impl Cfdb58 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb59_SPEC;
    pub type Cfdb59 = crate::EnumBitfieldStruct<u8, Cfdb59_SPEC>;
    impl Cfdb59 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf15_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf15_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf15 = crate::RegValueT<Cfdcfdf15_SPEC>;

impl Cfdcfdf15 {
    #[inline(always)]
    pub fn cfdb60(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf15_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf15_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb61(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdcfdf15::Cfdb61,
        cfdcfdf15::Cfdb61,
        Cfdcfdf15_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdcfdf15::Cfdb61,
            cfdcfdf15::Cfdb61,
            Cfdcfdf15_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdb62(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdcfdf15::Cfdb62,
        cfdcfdf15::Cfdb62,
        Cfdcfdf15_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdcfdf15::Cfdb62,
            cfdcfdf15::Cfdb62,
            Cfdcfdf15_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdb63(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdcfdf15::Cfdb63,
        cfdcfdf15::Cfdb63,
        Cfdcfdf15_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdcfdf15::Cfdb63,
            cfdcfdf15::Cfdb63,
            Cfdcfdf15_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcfdf15 {
    #[inline(always)]
    fn default() -> Cfdcfdf15 {
        <crate::RegValueT<Cfdcfdf15_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcfdf15 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb61_SPEC;
    pub type Cfdb61 = crate::EnumBitfieldStruct<u8, Cfdb61_SPEC>;
    impl Cfdb61 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb62_SPEC;
    pub type Cfdb62 = crate::EnumBitfieldStruct<u8, Cfdb62_SPEC>;
    impl Cfdb62 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb63_SPEC;
    pub type Cfdb63 = crate::EnumBitfieldStruct<u8, Cfdb63_SPEC>;
    impl Cfdb63 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmid_SPEC;
impl crate::sealed::RegSpec for Cfdtmid_SPEC {
    type DataType = u32;
}

pub type Cfdtmid = crate::RegValueT<Cfdtmid_SPEC>;

impl Cfdtmid {
    #[inline(always)]
    pub fn tmid(
        self,
    ) -> crate::common::RegisterField<0, 0x1fffffff, 1, 0, u32, u32, Cfdtmid_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fffffff,1,0,u32,u32,Cfdtmid_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn thlen(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        cfdtmid::Thlen,
        cfdtmid::Thlen,
        Cfdtmid_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            cfdtmid::Thlen,
            cfdtmid::Thlen,
            Cfdtmid_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tmrtr(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        cfdtmid::Tmrtr,
        cfdtmid::Tmrtr,
        Cfdtmid_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            cfdtmid::Tmrtr,
            cfdtmid::Tmrtr,
            Cfdtmid_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tmide(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        cfdtmid::Tmide,
        cfdtmid::Tmide,
        Cfdtmid_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            cfdtmid::Tmide,
            cfdtmid::Tmide,
            Cfdtmid_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmrtr_SPEC;
    pub type Tmrtr = crate::EnumBitfieldStruct<u8, Tmrtr_SPEC>;
    impl Tmrtr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmide_SPEC;
    pub type Tmide = crate::EnumBitfieldStruct<u8, Tmide_SPEC>;
    impl Tmide {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmptr_SPEC;
impl crate::sealed::RegSpec for Cfdtmptr_SPEC {
    type DataType = u32;
}

pub type Cfdtmptr = crate::RegValueT<Cfdtmptr_SPEC>;

impl Cfdtmptr {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0xfffffff, 1, 0, u32, u32, Cfdtmptr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xfffffff,1,0,u32,u32,Cfdtmptr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdlc(
        self,
    ) -> crate::common::RegisterField<
        28,
        0xf,
        1,
        0,
        cfdtmptr::Tmdlc,
        cfdtmptr::Tmdlc,
        Cfdtmptr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0xf,
            1,
            0,
            cfdtmptr::Tmdlc,
            cfdtmptr::Tmdlc,
            Cfdtmptr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdtmptr {
    #[inline(always)]
    fn default() -> Cfdtmptr {
        <crate::RegValueT<Cfdtmptr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdtmptr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmdlc_SPEC;
    pub type Tmdlc = crate::EnumBitfieldStruct<u8, Tmdlc_SPEC>;
    impl Tmdlc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmfdctr_SPEC;
impl crate::sealed::RegSpec for Cfdtmfdctr_SPEC {
    type DataType = u32;
}

pub type Cfdtmfdctr = crate::RegValueT<Cfdtmfdctr_SPEC>;

impl Cfdtmfdctr {
    #[inline(always)]
    pub fn tmesi(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdtmfdctr::Tmesi,
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
            cfdtmfdctr::Tmesi,
            Cfdtmfdctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tmbrs(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdtmfdctr::Tmbrs,
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
            cfdtmfdctr::Tmbrs,
            Cfdtmfdctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tmfdf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cfdtmfdctr::Tmfdf,
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
            cfdtmfdctr::Tmfdf,
            Cfdtmfdctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tmifl(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, Cfdtmfdctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,Cfdtmfdctr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<10, 0x3f, 1, 0, u8, u8, Cfdtmfdctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3f,1,0,u8,u8,Cfdtmfdctr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmptr(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Cfdtmfdctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Cfdtmfdctr_SPEC,crate::common::RW>::from_register(self,0)
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmbrs_SPEC;
    pub type Tmbrs = crate::EnumBitfieldStruct<u8, Tmbrs_SPEC>;
    impl Tmbrs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmfdf_SPEC;
    pub type Tmfdf = crate::EnumBitfieldStruct<u8, Tmfdf_SPEC>;
    impl Tmfdf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf0_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf0_SPEC {
    type DataType = u32;
}

pub type Cfdtmdf0 = crate::RegValueT<Cfdtmdf0_SPEC>;

impl Cfdtmdf0 {
    #[inline(always)]
    pub fn tmdb0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb1(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb2(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb3(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf0_SPEC,crate::common::RW>::from_register(self,0)
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

pub type Cfdtmdf1 = crate::RegValueT<Cfdtmdf1_SPEC>;

impl Cfdtmdf1 {
    #[inline(always)]
    pub fn tmdb4(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb5(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb6(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb7(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf1_SPEC,crate::common::RW>::from_register(self,0)
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

pub type Cfdtmdf2 = crate::RegValueT<Cfdtmdf2_SPEC>;

impl Cfdtmdf2 {
    #[inline(always)]
    pub fn tmdb8(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb9(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb10(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb11(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf2_SPEC,crate::common::RW>::from_register(self,0)
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

pub type Cfdtmdf3 = crate::RegValueT<Cfdtmdf3_SPEC>;

impl Cfdtmdf3 {
    #[inline(always)]
    pub fn tmdb12(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb13(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb14(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb15(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf3_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct Cfdthlacc0_SPEC;
impl crate::sealed::RegSpec for Cfdthlacc0_SPEC {
    type DataType = u32;
}

pub type Cfdthlacc0 = crate::RegValueT<Cfdthlacc0_SPEC>;

impl Cfdthlacc0 {
    #[inline(always)]
    pub fn bt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        cfdthlacc0::Bt,
        cfdthlacc0::Bt,
        Cfdthlacc0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            cfdthlacc0::Bt,
            cfdthlacc0::Bt,
            Cfdthlacc0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bn(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, Cfdthlacc0_SPEC, crate::common::R> {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,Cfdthlacc0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7ff, 1, 0, u16, u16, Cfdthlacc0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<5,0x7ff,1,0,u16,u16,Cfdthlacc0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmts(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Cfdthlacc0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Cfdthlacc0_SPEC,crate::common::R>::from_register(self,0)
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
        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _100: Self = Self::new(4);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdthlacc1_SPEC;
impl crate::sealed::RegSpec for Cfdthlacc1_SPEC {
    type DataType = u32;
}

pub type Cfdthlacc1 = crate::RegValueT<Cfdthlacc1_SPEC>;

impl Cfdthlacc1 {
    #[inline(always)]
    pub fn tid(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Cfdthlacc1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Cfdthlacc1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tifl(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, u8, Cfdthlacc1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x3,1,0,u8,u8,Cfdthlacc1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<18, 0x3fff, 1, 0, u16, u16, Cfdthlacc1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<18,0x3fff,1,0,u16,u16,Cfdthlacc1_SPEC,crate::common::R>::from_register(self,0)
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
pub struct Cfdrmdf0_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf0_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf0 = crate::RegValueT<Cfdrmdf0_SPEC>;

impl Cfdrmdf0 {
    #[inline(always)]
    pub fn rmdb0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf0_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb1(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrmdf0_::Rmdb1,
        cfdrmdf0_::Rmdb1,
        Cfdrmdf0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrmdf0_::Rmdb1,
            cfdrmdf0_::Rmdb1,
            Cfdrmdf0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb2(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrmdf0_::Rmdb2,
        cfdrmdf0_::Rmdb2,
        Cfdrmdf0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrmdf0_::Rmdb2,
            cfdrmdf0_::Rmdb2,
            Cfdrmdf0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb3(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrmdf0_::Rmdb3,
        cfdrmdf0_::Rmdb3,
        Cfdrmdf0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrmdf0_::Rmdb3,
            cfdrmdf0_::Rmdb3,
            Cfdrmdf0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmdf0 {
    #[inline(always)]
    fn default() -> Cfdrmdf0 {
        <crate::RegValueT<Cfdrmdf0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmdf0_ {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb1_SPEC;
    pub type Rmdb1 = crate::EnumBitfieldStruct<u8, Rmdb1_SPEC>;
    impl Rmdb1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb2_SPEC;
    pub type Rmdb2 = crate::EnumBitfieldStruct<u8, Rmdb2_SPEC>;
    impl Rmdb2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb3_SPEC;
    pub type Rmdb3 = crate::EnumBitfieldStruct<u8, Rmdb3_SPEC>;
    impl Rmdb3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf1_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf1_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf1 = crate::RegValueT<Cfdrmdf1_SPEC>;

impl Cfdrmdf1 {
    #[inline(always)]
    pub fn rmdb4(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf1_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb5(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrmdf1_::Rmdb5,
        cfdrmdf1_::Rmdb5,
        Cfdrmdf1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrmdf1_::Rmdb5,
            cfdrmdf1_::Rmdb5,
            Cfdrmdf1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb6(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrmdf1_::Rmdb6,
        cfdrmdf1_::Rmdb6,
        Cfdrmdf1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrmdf1_::Rmdb6,
            cfdrmdf1_::Rmdb6,
            Cfdrmdf1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb7(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrmdf1_::Rmdb7,
        cfdrmdf1_::Rmdb7,
        Cfdrmdf1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrmdf1_::Rmdb7,
            cfdrmdf1_::Rmdb7,
            Cfdrmdf1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmdf1 {
    #[inline(always)]
    fn default() -> Cfdrmdf1 {
        <crate::RegValueT<Cfdrmdf1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmdf1_ {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb5_SPEC;
    pub type Rmdb5 = crate::EnumBitfieldStruct<u8, Rmdb5_SPEC>;
    impl Rmdb5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb6_SPEC;
    pub type Rmdb6 = crate::EnumBitfieldStruct<u8, Rmdb6_SPEC>;
    impl Rmdb6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb7_SPEC;
    pub type Rmdb7 = crate::EnumBitfieldStruct<u8, Rmdb7_SPEC>;
    impl Rmdb7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf2_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf2_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf2 = crate::RegValueT<Cfdrmdf2_SPEC>;

impl Cfdrmdf2 {
    #[inline(always)]
    pub fn rmdb8(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf2_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf2_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb9(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrmdf2_::Rmdb9,
        cfdrmdf2_::Rmdb9,
        Cfdrmdf2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrmdf2_::Rmdb9,
            cfdrmdf2_::Rmdb9,
            Cfdrmdf2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb10(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrmdf2_::Rmdb10,
        cfdrmdf2_::Rmdb10,
        Cfdrmdf2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrmdf2_::Rmdb10,
            cfdrmdf2_::Rmdb10,
            Cfdrmdf2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb11(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrmdf2_::Rmdb11,
        cfdrmdf2_::Rmdb11,
        Cfdrmdf2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrmdf2_::Rmdb11,
            cfdrmdf2_::Rmdb11,
            Cfdrmdf2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmdf2 {
    #[inline(always)]
    fn default() -> Cfdrmdf2 {
        <crate::RegValueT<Cfdrmdf2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmdf2_ {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb9_SPEC;
    pub type Rmdb9 = crate::EnumBitfieldStruct<u8, Rmdb9_SPEC>;
    impl Rmdb9 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb10_SPEC;
    pub type Rmdb10 = crate::EnumBitfieldStruct<u8, Rmdb10_SPEC>;
    impl Rmdb10 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb11_SPEC;
    pub type Rmdb11 = crate::EnumBitfieldStruct<u8, Rmdb11_SPEC>;
    impl Rmdb11 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf3_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf3_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf3 = crate::RegValueT<Cfdrmdf3_SPEC>;

impl Cfdrmdf3 {
    #[inline(always)]
    pub fn rmdb12(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf3_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf3_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb13(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrmdf3_::Rmdb13,
        cfdrmdf3_::Rmdb13,
        Cfdrmdf3_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrmdf3_::Rmdb13,
            cfdrmdf3_::Rmdb13,
            Cfdrmdf3_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb14(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrmdf3_::Rmdb14,
        cfdrmdf3_::Rmdb14,
        Cfdrmdf3_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrmdf3_::Rmdb14,
            cfdrmdf3_::Rmdb14,
            Cfdrmdf3_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb15(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrmdf3_::Rmdb15,
        cfdrmdf3_::Rmdb15,
        Cfdrmdf3_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrmdf3_::Rmdb15,
            cfdrmdf3_::Rmdb15,
            Cfdrmdf3_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmdf3 {
    #[inline(always)]
    fn default() -> Cfdrmdf3 {
        <crate::RegValueT<Cfdrmdf3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmdf3_ {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb13_SPEC;
    pub type Rmdb13 = crate::EnumBitfieldStruct<u8, Rmdb13_SPEC>;
    impl Rmdb13 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb14_SPEC;
    pub type Rmdb14 = crate::EnumBitfieldStruct<u8, Rmdb14_SPEC>;
    impl Rmdb14 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb15_SPEC;
    pub type Rmdb15 = crate::EnumBitfieldStruct<u8, Rmdb15_SPEC>;
    impl Rmdb15 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf4_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf4_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf4 = crate::RegValueT<Cfdrmdf4_SPEC>;

impl Cfdrmdf4 {
    #[inline(always)]
    pub fn rmdb16(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf4_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf4_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb17(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrmdf4_::Rmdb17,
        cfdrmdf4_::Rmdb17,
        Cfdrmdf4_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrmdf4_::Rmdb17,
            cfdrmdf4_::Rmdb17,
            Cfdrmdf4_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb18(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrmdf4_::Rmdb18,
        cfdrmdf4_::Rmdb18,
        Cfdrmdf4_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrmdf4_::Rmdb18,
            cfdrmdf4_::Rmdb18,
            Cfdrmdf4_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb19(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrmdf4_::Rmdb19,
        cfdrmdf4_::Rmdb19,
        Cfdrmdf4_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrmdf4_::Rmdb19,
            cfdrmdf4_::Rmdb19,
            Cfdrmdf4_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmdf4 {
    #[inline(always)]
    fn default() -> Cfdrmdf4 {
        <crate::RegValueT<Cfdrmdf4_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmdf4_ {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb17_SPEC;
    pub type Rmdb17 = crate::EnumBitfieldStruct<u8, Rmdb17_SPEC>;
    impl Rmdb17 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb18_SPEC;
    pub type Rmdb18 = crate::EnumBitfieldStruct<u8, Rmdb18_SPEC>;
    impl Rmdb18 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb19_SPEC;
    pub type Rmdb19 = crate::EnumBitfieldStruct<u8, Rmdb19_SPEC>;
    impl Rmdb19 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf5_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf5_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf5 = crate::RegValueT<Cfdrmdf5_SPEC>;

impl Cfdrmdf5 {
    #[inline(always)]
    pub fn rmdb20(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf5_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf5_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb21(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrmdf5_::Rmdb21,
        cfdrmdf5_::Rmdb21,
        Cfdrmdf5_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrmdf5_::Rmdb21,
            cfdrmdf5_::Rmdb21,
            Cfdrmdf5_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb22(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrmdf5_::Rmdb22,
        cfdrmdf5_::Rmdb22,
        Cfdrmdf5_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrmdf5_::Rmdb22,
            cfdrmdf5_::Rmdb22,
            Cfdrmdf5_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb23(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrmdf5_::Rmdb23,
        cfdrmdf5_::Rmdb23,
        Cfdrmdf5_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrmdf5_::Rmdb23,
            cfdrmdf5_::Rmdb23,
            Cfdrmdf5_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmdf5 {
    #[inline(always)]
    fn default() -> Cfdrmdf5 {
        <crate::RegValueT<Cfdrmdf5_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmdf5_ {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb21_SPEC;
    pub type Rmdb21 = crate::EnumBitfieldStruct<u8, Rmdb21_SPEC>;
    impl Rmdb21 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb22_SPEC;
    pub type Rmdb22 = crate::EnumBitfieldStruct<u8, Rmdb22_SPEC>;
    impl Rmdb22 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb23_SPEC;
    pub type Rmdb23 = crate::EnumBitfieldStruct<u8, Rmdb23_SPEC>;
    impl Rmdb23 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf6_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf6_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf6 = crate::RegValueT<Cfdrmdf6_SPEC>;

impl Cfdrmdf6 {
    #[inline(always)]
    pub fn rmdb24(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf6_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf6_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb25(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrmdf6_::Rmdb25,
        cfdrmdf6_::Rmdb25,
        Cfdrmdf6_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrmdf6_::Rmdb25,
            cfdrmdf6_::Rmdb25,
            Cfdrmdf6_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb26(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrmdf6_::Rmdb26,
        cfdrmdf6_::Rmdb26,
        Cfdrmdf6_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrmdf6_::Rmdb26,
            cfdrmdf6_::Rmdb26,
            Cfdrmdf6_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb27(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrmdf6_::Rmdb27,
        cfdrmdf6_::Rmdb27,
        Cfdrmdf6_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrmdf6_::Rmdb27,
            cfdrmdf6_::Rmdb27,
            Cfdrmdf6_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmdf6 {
    #[inline(always)]
    fn default() -> Cfdrmdf6 {
        <crate::RegValueT<Cfdrmdf6_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmdf6_ {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb25_SPEC;
    pub type Rmdb25 = crate::EnumBitfieldStruct<u8, Rmdb25_SPEC>;
    impl Rmdb25 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb26_SPEC;
    pub type Rmdb26 = crate::EnumBitfieldStruct<u8, Rmdb26_SPEC>;
    impl Rmdb26 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb27_SPEC;
    pub type Rmdb27 = crate::EnumBitfieldStruct<u8, Rmdb27_SPEC>;
    impl Rmdb27 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf7_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf7_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf7 = crate::RegValueT<Cfdrmdf7_SPEC>;

impl Cfdrmdf7 {
    #[inline(always)]
    pub fn rmdb28(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf7_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf7_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb29(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrmdf7_::Rmdb29,
        cfdrmdf7_::Rmdb29,
        Cfdrmdf7_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrmdf7_::Rmdb29,
            cfdrmdf7_::Rmdb29,
            Cfdrmdf7_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb30(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrmdf7_::Rmdb30,
        cfdrmdf7_::Rmdb30,
        Cfdrmdf7_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrmdf7_::Rmdb30,
            cfdrmdf7_::Rmdb30,
            Cfdrmdf7_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb31(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrmdf7_::Rmdb31,
        cfdrmdf7_::Rmdb31,
        Cfdrmdf7_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrmdf7_::Rmdb31,
            cfdrmdf7_::Rmdb31,
            Cfdrmdf7_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmdf7 {
    #[inline(always)]
    fn default() -> Cfdrmdf7 {
        <crate::RegValueT<Cfdrmdf7_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmdf7_ {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb29_SPEC;
    pub type Rmdb29 = crate::EnumBitfieldStruct<u8, Rmdb29_SPEC>;
    impl Rmdb29 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb30_SPEC;
    pub type Rmdb30 = crate::EnumBitfieldStruct<u8, Rmdb30_SPEC>;
    impl Rmdb30 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb31_SPEC;
    pub type Rmdb31 = crate::EnumBitfieldStruct<u8, Rmdb31_SPEC>;
    impl Rmdb31 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf8_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf8_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf8 = crate::RegValueT<Cfdrmdf8_SPEC>;

impl Cfdrmdf8 {
    #[inline(always)]
    pub fn rmdb32(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf8_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf8_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb33(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrmdf8_::Rmdb33,
        cfdrmdf8_::Rmdb33,
        Cfdrmdf8_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrmdf8_::Rmdb33,
            cfdrmdf8_::Rmdb33,
            Cfdrmdf8_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb34(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrmdf8_::Rmdb34,
        cfdrmdf8_::Rmdb34,
        Cfdrmdf8_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrmdf8_::Rmdb34,
            cfdrmdf8_::Rmdb34,
            Cfdrmdf8_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb35(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrmdf8_::Rmdb35,
        cfdrmdf8_::Rmdb35,
        Cfdrmdf8_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrmdf8_::Rmdb35,
            cfdrmdf8_::Rmdb35,
            Cfdrmdf8_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmdf8 {
    #[inline(always)]
    fn default() -> Cfdrmdf8 {
        <crate::RegValueT<Cfdrmdf8_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmdf8_ {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb33_SPEC;
    pub type Rmdb33 = crate::EnumBitfieldStruct<u8, Rmdb33_SPEC>;
    impl Rmdb33 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb34_SPEC;
    pub type Rmdb34 = crate::EnumBitfieldStruct<u8, Rmdb34_SPEC>;
    impl Rmdb34 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb35_SPEC;
    pub type Rmdb35 = crate::EnumBitfieldStruct<u8, Rmdb35_SPEC>;
    impl Rmdb35 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf9_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf9_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf9 = crate::RegValueT<Cfdrmdf9_SPEC>;

impl Cfdrmdf9 {
    #[inline(always)]
    pub fn rmdb36(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf9_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf9_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb37(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrmdf9_::Rmdb37,
        cfdrmdf9_::Rmdb37,
        Cfdrmdf9_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrmdf9_::Rmdb37,
            cfdrmdf9_::Rmdb37,
            Cfdrmdf9_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb38(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrmdf9_::Rmdb38,
        cfdrmdf9_::Rmdb38,
        Cfdrmdf9_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrmdf9_::Rmdb38,
            cfdrmdf9_::Rmdb38,
            Cfdrmdf9_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb39(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrmdf9_::Rmdb39,
        cfdrmdf9_::Rmdb39,
        Cfdrmdf9_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrmdf9_::Rmdb39,
            cfdrmdf9_::Rmdb39,
            Cfdrmdf9_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmdf9 {
    #[inline(always)]
    fn default() -> Cfdrmdf9 {
        <crate::RegValueT<Cfdrmdf9_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmdf9_ {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb37_SPEC;
    pub type Rmdb37 = crate::EnumBitfieldStruct<u8, Rmdb37_SPEC>;
    impl Rmdb37 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb38_SPEC;
    pub type Rmdb38 = crate::EnumBitfieldStruct<u8, Rmdb38_SPEC>;
    impl Rmdb38 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb39_SPEC;
    pub type Rmdb39 = crate::EnumBitfieldStruct<u8, Rmdb39_SPEC>;
    impl Rmdb39 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf10_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf10_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf10 = crate::RegValueT<Cfdrmdf10_SPEC>;

impl Cfdrmdf10 {
    #[inline(always)]
    pub fn rmdb40(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf10_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf10_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb41(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrmdf10_::Rmdb41,
        cfdrmdf10_::Rmdb41,
        Cfdrmdf10_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrmdf10_::Rmdb41,
            cfdrmdf10_::Rmdb41,
            Cfdrmdf10_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb42(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrmdf10_::Rmdb42,
        cfdrmdf10_::Rmdb42,
        Cfdrmdf10_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrmdf10_::Rmdb42,
            cfdrmdf10_::Rmdb42,
            Cfdrmdf10_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb43(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrmdf10_::Rmdb43,
        cfdrmdf10_::Rmdb43,
        Cfdrmdf10_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrmdf10_::Rmdb43,
            cfdrmdf10_::Rmdb43,
            Cfdrmdf10_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmdf10 {
    #[inline(always)]
    fn default() -> Cfdrmdf10 {
        <crate::RegValueT<Cfdrmdf10_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmdf10_ {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb41_SPEC;
    pub type Rmdb41 = crate::EnumBitfieldStruct<u8, Rmdb41_SPEC>;
    impl Rmdb41 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb42_SPEC;
    pub type Rmdb42 = crate::EnumBitfieldStruct<u8, Rmdb42_SPEC>;
    impl Rmdb42 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb43_SPEC;
    pub type Rmdb43 = crate::EnumBitfieldStruct<u8, Rmdb43_SPEC>;
    impl Rmdb43 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf11_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf11_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf11 = crate::RegValueT<Cfdrmdf11_SPEC>;

impl Cfdrmdf11 {
    #[inline(always)]
    pub fn rmdb44(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf11_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf11_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb45(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrmdf11_::Rmdb45,
        cfdrmdf11_::Rmdb45,
        Cfdrmdf11_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrmdf11_::Rmdb45,
            cfdrmdf11_::Rmdb45,
            Cfdrmdf11_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb46(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrmdf11_::Rmdb46,
        cfdrmdf11_::Rmdb46,
        Cfdrmdf11_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrmdf11_::Rmdb46,
            cfdrmdf11_::Rmdb46,
            Cfdrmdf11_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb47(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrmdf11_::Rmdb47,
        cfdrmdf11_::Rmdb47,
        Cfdrmdf11_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrmdf11_::Rmdb47,
            cfdrmdf11_::Rmdb47,
            Cfdrmdf11_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmdf11 {
    #[inline(always)]
    fn default() -> Cfdrmdf11 {
        <crate::RegValueT<Cfdrmdf11_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmdf11_ {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb45_SPEC;
    pub type Rmdb45 = crate::EnumBitfieldStruct<u8, Rmdb45_SPEC>;
    impl Rmdb45 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb46_SPEC;
    pub type Rmdb46 = crate::EnumBitfieldStruct<u8, Rmdb46_SPEC>;
    impl Rmdb46 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb47_SPEC;
    pub type Rmdb47 = crate::EnumBitfieldStruct<u8, Rmdb47_SPEC>;
    impl Rmdb47 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf12_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf12_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf12 = crate::RegValueT<Cfdrmdf12_SPEC>;

impl Cfdrmdf12 {
    #[inline(always)]
    pub fn rmdb48(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf12_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf12_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb49(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrmdf12_::Rmdb49,
        cfdrmdf12_::Rmdb49,
        Cfdrmdf12_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrmdf12_::Rmdb49,
            cfdrmdf12_::Rmdb49,
            Cfdrmdf12_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb50(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrmdf12_::Rmdb50,
        cfdrmdf12_::Rmdb50,
        Cfdrmdf12_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrmdf12_::Rmdb50,
            cfdrmdf12_::Rmdb50,
            Cfdrmdf12_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb51(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrmdf12_::Rmdb51,
        cfdrmdf12_::Rmdb51,
        Cfdrmdf12_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrmdf12_::Rmdb51,
            cfdrmdf12_::Rmdb51,
            Cfdrmdf12_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmdf12 {
    #[inline(always)]
    fn default() -> Cfdrmdf12 {
        <crate::RegValueT<Cfdrmdf12_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmdf12_ {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb49_SPEC;
    pub type Rmdb49 = crate::EnumBitfieldStruct<u8, Rmdb49_SPEC>;
    impl Rmdb49 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb50_SPEC;
    pub type Rmdb50 = crate::EnumBitfieldStruct<u8, Rmdb50_SPEC>;
    impl Rmdb50 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb51_SPEC;
    pub type Rmdb51 = crate::EnumBitfieldStruct<u8, Rmdb51_SPEC>;
    impl Rmdb51 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf13_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf13_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf13 = crate::RegValueT<Cfdrmdf13_SPEC>;

impl Cfdrmdf13 {
    #[inline(always)]
    pub fn rmdb52(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf13_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf13_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb53(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrmdf13_::Rmdb53,
        cfdrmdf13_::Rmdb53,
        Cfdrmdf13_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrmdf13_::Rmdb53,
            cfdrmdf13_::Rmdb53,
            Cfdrmdf13_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb54(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrmdf13_::Rmdb54,
        cfdrmdf13_::Rmdb54,
        Cfdrmdf13_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrmdf13_::Rmdb54,
            cfdrmdf13_::Rmdb54,
            Cfdrmdf13_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb55(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrmdf13_::Rmdb55,
        cfdrmdf13_::Rmdb55,
        Cfdrmdf13_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrmdf13_::Rmdb55,
            cfdrmdf13_::Rmdb55,
            Cfdrmdf13_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmdf13 {
    #[inline(always)]
    fn default() -> Cfdrmdf13 {
        <crate::RegValueT<Cfdrmdf13_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmdf13_ {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb53_SPEC;
    pub type Rmdb53 = crate::EnumBitfieldStruct<u8, Rmdb53_SPEC>;
    impl Rmdb53 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb54_SPEC;
    pub type Rmdb54 = crate::EnumBitfieldStruct<u8, Rmdb54_SPEC>;
    impl Rmdb54 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb55_SPEC;
    pub type Rmdb55 = crate::EnumBitfieldStruct<u8, Rmdb55_SPEC>;
    impl Rmdb55 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf14_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf14_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf14 = crate::RegValueT<Cfdrmdf14_SPEC>;

impl Cfdrmdf14 {
    #[inline(always)]
    pub fn rmdb56(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf14_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf14_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb57(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrmdf14_::Rmdb57,
        cfdrmdf14_::Rmdb57,
        Cfdrmdf14_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrmdf14_::Rmdb57,
            cfdrmdf14_::Rmdb57,
            Cfdrmdf14_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb58(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrmdf14_::Rmdb58,
        cfdrmdf14_::Rmdb58,
        Cfdrmdf14_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrmdf14_::Rmdb58,
            cfdrmdf14_::Rmdb58,
            Cfdrmdf14_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb59(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrmdf14_::Rmdb59,
        cfdrmdf14_::Rmdb59,
        Cfdrmdf14_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrmdf14_::Rmdb59,
            cfdrmdf14_::Rmdb59,
            Cfdrmdf14_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmdf14 {
    #[inline(always)]
    fn default() -> Cfdrmdf14 {
        <crate::RegValueT<Cfdrmdf14_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmdf14_ {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb57_SPEC;
    pub type Rmdb57 = crate::EnumBitfieldStruct<u8, Rmdb57_SPEC>;
    impl Rmdb57 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb58_SPEC;
    pub type Rmdb58 = crate::EnumBitfieldStruct<u8, Rmdb58_SPEC>;
    impl Rmdb58 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb59_SPEC;
    pub type Rmdb59 = crate::EnumBitfieldStruct<u8, Rmdb59_SPEC>;
    impl Rmdb59 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf15_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf15_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf15 = crate::RegValueT<Cfdrmdf15_SPEC>;

impl Cfdrmdf15 {
    #[inline(always)]
    pub fn rmdb60(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf15_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf15_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb61(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrmdf15_::Rmdb61,
        cfdrmdf15_::Rmdb61,
        Cfdrmdf15_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrmdf15_::Rmdb61,
            cfdrmdf15_::Rmdb61,
            Cfdrmdf15_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb62(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrmdf15_::Rmdb62,
        cfdrmdf15_::Rmdb62,
        Cfdrmdf15_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrmdf15_::Rmdb62,
            cfdrmdf15_::Rmdb62,
            Cfdrmdf15_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb63(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrmdf15_::Rmdb63,
        cfdrmdf15_::Rmdb63,
        Cfdrmdf15_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrmdf15_::Rmdb63,
            cfdrmdf15_::Rmdb63,
            Cfdrmdf15_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmdf15 {
    #[inline(always)]
    fn default() -> Cfdrmdf15 {
        <crate::RegValueT<Cfdrmdf15_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmdf15_ {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb61_SPEC;
    pub type Rmdb61 = crate::EnumBitfieldStruct<u8, Rmdb61_SPEC>;
    impl Rmdb61 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb62_SPEC;
    pub type Rmdb62 = crate::EnumBitfieldStruct<u8, Rmdb62_SPEC>;
    impl Rmdb62 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb63_SPEC;
    pub type Rmdb63 = crate::EnumBitfieldStruct<u8, Rmdb63_SPEC>;
    impl Rmdb63 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf16_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf16_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf16 = crate::RegValueT<Cfdrmdf16_SPEC>;

impl Cfdrmdf16 {
    #[inline(always)]
    pub fn rmdb64(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf16_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf16_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb65(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrmdf16_::Rmdb65,
        cfdrmdf16_::Rmdb65,
        Cfdrmdf16_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrmdf16_::Rmdb65,
            cfdrmdf16_::Rmdb65,
            Cfdrmdf16_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb66(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrmdf16_::Rmdb66,
        cfdrmdf16_::Rmdb66,
        Cfdrmdf16_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrmdf16_::Rmdb66,
            cfdrmdf16_::Rmdb66,
            Cfdrmdf16_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb67(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrmdf16_::Rmdb67,
        cfdrmdf16_::Rmdb67,
        Cfdrmdf16_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrmdf16_::Rmdb67,
            cfdrmdf16_::Rmdb67,
            Cfdrmdf16_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmdf16 {
    #[inline(always)]
    fn default() -> Cfdrmdf16 {
        <crate::RegValueT<Cfdrmdf16_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmdf16_ {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb65_SPEC;
    pub type Rmdb65 = crate::EnumBitfieldStruct<u8, Rmdb65_SPEC>;
    impl Rmdb65 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb66_SPEC;
    pub type Rmdb66 = crate::EnumBitfieldStruct<u8, Rmdb66_SPEC>;
    impl Rmdb66 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb67_SPEC;
    pub type Rmdb67 = crate::EnumBitfieldStruct<u8, Rmdb67_SPEC>;
    impl Rmdb67 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf17_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf17_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf17 = crate::RegValueT<Cfdrmdf17_SPEC>;

impl Cfdrmdf17 {
    #[inline(always)]
    pub fn rmdb68(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf17_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf17_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb69(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrmdf17_::Rmdb69,
        cfdrmdf17_::Rmdb69,
        Cfdrmdf17_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrmdf17_::Rmdb69,
            cfdrmdf17_::Rmdb69,
            Cfdrmdf17_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb70(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrmdf17_::Rmdb70,
        cfdrmdf17_::Rmdb70,
        Cfdrmdf17_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrmdf17_::Rmdb70,
            cfdrmdf17_::Rmdb70,
            Cfdrmdf17_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb71(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrmdf17_::Rmdb71,
        cfdrmdf17_::Rmdb71,
        Cfdrmdf17_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrmdf17_::Rmdb71,
            cfdrmdf17_::Rmdb71,
            Cfdrmdf17_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmdf17 {
    #[inline(always)]
    fn default() -> Cfdrmdf17 {
        <crate::RegValueT<Cfdrmdf17_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmdf17_ {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb69_SPEC;
    pub type Rmdb69 = crate::EnumBitfieldStruct<u8, Rmdb69_SPEC>;
    impl Rmdb69 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb70_SPEC;
    pub type Rmdb70 = crate::EnumBitfieldStruct<u8, Rmdb70_SPEC>;
    impl Rmdb70 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb71_SPEC;
    pub type Rmdb71 = crate::EnumBitfieldStruct<u8, Rmdb71_SPEC>;
    impl Rmdb71 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf18_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf18_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf18 = crate::RegValueT<Cfdrmdf18_SPEC>;

impl Cfdrmdf18 {
    #[inline(always)]
    pub fn rmdb72(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf18_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf18_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb73(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrmdf18_::Rmdb73,
        cfdrmdf18_::Rmdb73,
        Cfdrmdf18_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrmdf18_::Rmdb73,
            cfdrmdf18_::Rmdb73,
            Cfdrmdf18_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb74(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrmdf18_::Rmdb74,
        cfdrmdf18_::Rmdb74,
        Cfdrmdf18_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrmdf18_::Rmdb74,
            cfdrmdf18_::Rmdb74,
            Cfdrmdf18_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb75(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrmdf18_::Rmdb75,
        cfdrmdf18_::Rmdb75,
        Cfdrmdf18_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrmdf18_::Rmdb75,
            cfdrmdf18_::Rmdb75,
            Cfdrmdf18_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmdf18 {
    #[inline(always)]
    fn default() -> Cfdrmdf18 {
        <crate::RegValueT<Cfdrmdf18_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmdf18_ {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb73_SPEC;
    pub type Rmdb73 = crate::EnumBitfieldStruct<u8, Rmdb73_SPEC>;
    impl Rmdb73 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb74_SPEC;
    pub type Rmdb74 = crate::EnumBitfieldStruct<u8, Rmdb74_SPEC>;
    impl Rmdb74 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb75_SPEC;
    pub type Rmdb75 = crate::EnumBitfieldStruct<u8, Rmdb75_SPEC>;
    impl Rmdb75 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf19_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf19_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf19 = crate::RegValueT<Cfdrmdf19_SPEC>;

impl Cfdrmdf19 {
    #[inline(always)]
    pub fn rmdb76(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf19_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf19_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb77(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrmdf19_::Rmdb77,
        cfdrmdf19_::Rmdb77,
        Cfdrmdf19_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrmdf19_::Rmdb77,
            cfdrmdf19_::Rmdb77,
            Cfdrmdf19_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb78(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrmdf19_::Rmdb78,
        cfdrmdf19_::Rmdb78,
        Cfdrmdf19_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrmdf19_::Rmdb78,
            cfdrmdf19_::Rmdb78,
            Cfdrmdf19_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb79(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrmdf19_::Rmdb79,
        cfdrmdf19_::Rmdb79,
        Cfdrmdf19_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrmdf19_::Rmdb79,
            cfdrmdf19_::Rmdb79,
            Cfdrmdf19_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmdf19 {
    #[inline(always)]
    fn default() -> Cfdrmdf19 {
        <crate::RegValueT<Cfdrmdf19_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmdf19_ {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb77_SPEC;
    pub type Rmdb77 = crate::EnumBitfieldStruct<u8, Rmdb77_SPEC>;
    impl Rmdb77 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb78_SPEC;
    pub type Rmdb78 = crate::EnumBitfieldStruct<u8, Rmdb78_SPEC>;
    impl Rmdb78 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb79_SPEC;
    pub type Rmdb79 = crate::EnumBitfieldStruct<u8, Rmdb79_SPEC>;
    impl Rmdb79 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf20_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf20_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf20 = crate::RegValueT<Cfdrmdf20_SPEC>;

impl Cfdrmdf20 {
    #[inline(always)]
    pub fn rmdb80(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf20_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf20_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb81(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrmdf20_::Rmdb81,
        cfdrmdf20_::Rmdb81,
        Cfdrmdf20_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrmdf20_::Rmdb81,
            cfdrmdf20_::Rmdb81,
            Cfdrmdf20_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb82(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrmdf20_::Rmdb82,
        cfdrmdf20_::Rmdb82,
        Cfdrmdf20_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrmdf20_::Rmdb82,
            cfdrmdf20_::Rmdb82,
            Cfdrmdf20_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb83(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrmdf20_::Rmdb83,
        cfdrmdf20_::Rmdb83,
        Cfdrmdf20_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrmdf20_::Rmdb83,
            cfdrmdf20_::Rmdb83,
            Cfdrmdf20_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmdf20 {
    #[inline(always)]
    fn default() -> Cfdrmdf20 {
        <crate::RegValueT<Cfdrmdf20_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmdf20_ {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb81_SPEC;
    pub type Rmdb81 = crate::EnumBitfieldStruct<u8, Rmdb81_SPEC>;
    impl Rmdb81 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb82_SPEC;
    pub type Rmdb82 = crate::EnumBitfieldStruct<u8, Rmdb82_SPEC>;
    impl Rmdb82 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb83_SPEC;
    pub type Rmdb83 = crate::EnumBitfieldStruct<u8, Rmdb83_SPEC>;
    impl Rmdb83 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf21_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf21_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf21 = crate::RegValueT<Cfdrmdf21_SPEC>;

impl Cfdrmdf21 {
    #[inline(always)]
    pub fn rmdb84(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf21_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf21_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb85(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrmdf21_::Rmdb85,
        cfdrmdf21_::Rmdb85,
        Cfdrmdf21_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrmdf21_::Rmdb85,
            cfdrmdf21_::Rmdb85,
            Cfdrmdf21_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb86(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrmdf21_::Rmdb86,
        cfdrmdf21_::Rmdb86,
        Cfdrmdf21_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrmdf21_::Rmdb86,
            cfdrmdf21_::Rmdb86,
            Cfdrmdf21_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb87(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrmdf21_::Rmdb87,
        cfdrmdf21_::Rmdb87,
        Cfdrmdf21_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrmdf21_::Rmdb87,
            cfdrmdf21_::Rmdb87,
            Cfdrmdf21_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmdf21 {
    #[inline(always)]
    fn default() -> Cfdrmdf21 {
        <crate::RegValueT<Cfdrmdf21_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmdf21_ {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb85_SPEC;
    pub type Rmdb85 = crate::EnumBitfieldStruct<u8, Rmdb85_SPEC>;
    impl Rmdb85 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb86_SPEC;
    pub type Rmdb86 = crate::EnumBitfieldStruct<u8, Rmdb86_SPEC>;
    impl Rmdb86 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb87_SPEC;
    pub type Rmdb87 = crate::EnumBitfieldStruct<u8, Rmdb87_SPEC>;
    impl Rmdb87 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf22_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf22_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf22 = crate::RegValueT<Cfdrmdf22_SPEC>;

impl Cfdrmdf22 {
    #[inline(always)]
    pub fn rmdb88(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf22_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf22_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb89(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrmdf22_::Rmdb89,
        cfdrmdf22_::Rmdb89,
        Cfdrmdf22_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrmdf22_::Rmdb89,
            cfdrmdf22_::Rmdb89,
            Cfdrmdf22_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb90(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrmdf22_::Rmdb90,
        cfdrmdf22_::Rmdb90,
        Cfdrmdf22_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrmdf22_::Rmdb90,
            cfdrmdf22_::Rmdb90,
            Cfdrmdf22_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb91(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrmdf22_::Rmdb91,
        cfdrmdf22_::Rmdb91,
        Cfdrmdf22_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrmdf22_::Rmdb91,
            cfdrmdf22_::Rmdb91,
            Cfdrmdf22_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmdf22 {
    #[inline(always)]
    fn default() -> Cfdrmdf22 {
        <crate::RegValueT<Cfdrmdf22_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmdf22_ {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb89_SPEC;
    pub type Rmdb89 = crate::EnumBitfieldStruct<u8, Rmdb89_SPEC>;
    impl Rmdb89 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb90_SPEC;
    pub type Rmdb90 = crate::EnumBitfieldStruct<u8, Rmdb90_SPEC>;
    impl Rmdb90 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb91_SPEC;
    pub type Rmdb91 = crate::EnumBitfieldStruct<u8, Rmdb91_SPEC>;
    impl Rmdb91 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf23_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf23_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf23 = crate::RegValueT<Cfdrmdf23_SPEC>;

impl Cfdrmdf23 {
    #[inline(always)]
    pub fn rmdb92(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf23_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf23_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb93(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrmdf23_::Rmdb93,
        cfdrmdf23_::Rmdb93,
        Cfdrmdf23_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrmdf23_::Rmdb93,
            cfdrmdf23_::Rmdb93,
            Cfdrmdf23_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb94(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrmdf23_::Rmdb94,
        cfdrmdf23_::Rmdb94,
        Cfdrmdf23_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrmdf23_::Rmdb94,
            cfdrmdf23_::Rmdb94,
            Cfdrmdf23_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb95(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrmdf23_::Rmdb95,
        cfdrmdf23_::Rmdb95,
        Cfdrmdf23_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrmdf23_::Rmdb95,
            cfdrmdf23_::Rmdb95,
            Cfdrmdf23_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmdf23 {
    #[inline(always)]
    fn default() -> Cfdrmdf23 {
        <crate::RegValueT<Cfdrmdf23_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmdf23_ {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb93_SPEC;
    pub type Rmdb93 = crate::EnumBitfieldStruct<u8, Rmdb93_SPEC>;
    impl Rmdb93 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb94_SPEC;
    pub type Rmdb94 = crate::EnumBitfieldStruct<u8, Rmdb94_SPEC>;
    impl Rmdb94 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb95_SPEC;
    pub type Rmdb95 = crate::EnumBitfieldStruct<u8, Rmdb95_SPEC>;
    impl Rmdb95 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmid_SPEC;
impl crate::sealed::RegSpec for Cfdrmid_SPEC {
    type DataType = u32;
}

pub type Cfdrmid = crate::RegValueT<Cfdrmid_SPEC>;

impl Cfdrmid {
    #[inline(always)]
    pub fn rmid(
        self,
    ) -> crate::common::RegisterField<0, 0x1fffffff, 1, 0, u32, u32, Cfdrmid_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1fffffff,1,0,u32,u32,Cfdrmid_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Cfdrmid_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cfdrmid_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn rmrtr(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        cfdrmid::Rmrtr,
        cfdrmid::Rmrtr,
        Cfdrmid_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            cfdrmid::Rmrtr,
            cfdrmid::Rmrtr,
            Cfdrmid_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmide(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        cfdrmid::Rmide,
        cfdrmid::Rmide,
        Cfdrmid_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            cfdrmid::Rmide,
            cfdrmid::Rmide,
            Cfdrmid_SPEC,
            crate::common::R,
        >::from_register(self, 0)
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmide_SPEC;
    pub type Rmide = crate::EnumBitfieldStruct<u8, Rmide_SPEC>;
    impl Rmide {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmptr_SPEC;
impl crate::sealed::RegSpec for Cfdrmptr_SPEC {
    type DataType = u32;
}

pub type Cfdrmptr = crate::RegValueT<Cfdrmptr_SPEC>;

impl Cfdrmptr {
    #[inline(always)]
    pub fn rmts(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Cfdrmptr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Cfdrmptr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, u16, Cfdrmptr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xfff,1,0,u16,u16,Cfdrmptr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdlc(
        self,
    ) -> crate::common::RegisterField<
        28,
        0xf,
        1,
        0,
        cfdrmptr::Rmdlc,
        cfdrmptr::Rmdlc,
        Cfdrmptr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            28,
            0xf,
            1,
            0,
            cfdrmptr::Rmdlc,
            cfdrmptr::Rmdlc,
            Cfdrmptr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmptr {
    #[inline(always)]
    fn default() -> Cfdrmptr {
        <crate::RegValueT<Cfdrmptr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmptr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdlc_SPEC;
    pub type Rmdlc = crate::EnumBitfieldStruct<u8, Rmdlc_SPEC>;
    impl Rmdlc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmfdsts_SPEC;
impl crate::sealed::RegSpec for Cfdrmfdsts_SPEC {
    type DataType = u32;
}

pub type Cfdrmfdsts = crate::RegValueT<Cfdrmfdsts_SPEC>;

impl Cfdrmfdsts {
    #[inline(always)]
    pub fn rmesi(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdrmfdsts::Rmesi,
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
            cfdrmfdsts::Rmesi,
            Cfdrmfdsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmbrs(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdrmfdsts::Rmbrs,
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
            cfdrmfdsts::Rmbrs,
            Cfdrmfdsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmfdf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cfdrmfdsts::Rmfdf,
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
            cfdrmfdsts::Rmfdf,
            Cfdrmfdsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmifl(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, Cfdrmfdsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,Cfdrmfdsts_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<10, 0x3f, 1, 0, u8, u8, Cfdrmfdsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<10,0x3f,1,0,u8,u8,Cfdrmfdsts_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmptr(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Cfdrmfdsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Cfdrmfdsts_SPEC,crate::common::R>::from_register(self,0)
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmbrs_SPEC;
    pub type Rmbrs = crate::EnumBitfieldStruct<u8, Rmbrs_SPEC>;
    impl Rmbrs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmfdf_SPEC;
    pub type Rmfdf = crate::EnumBitfieldStruct<u8, Rmfdf_SPEC>;
    impl Rmfdf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf24_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf24_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf24 = crate::RegValueT<Cfdrmdf24_SPEC>;

impl Cfdrmdf24 {
    #[inline(always)]
    pub fn rmdb96(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf24_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf24_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb97(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrmdf24_::Rmdb97,
        cfdrmdf24_::Rmdb97,
        Cfdrmdf24_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrmdf24_::Rmdb97,
            cfdrmdf24_::Rmdb97,
            Cfdrmdf24_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb98(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrmdf24_::Rmdb98,
        cfdrmdf24_::Rmdb98,
        Cfdrmdf24_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrmdf24_::Rmdb98,
            cfdrmdf24_::Rmdb98,
            Cfdrmdf24_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb99(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrmdf24_::Rmdb99,
        cfdrmdf24_::Rmdb99,
        Cfdrmdf24_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrmdf24_::Rmdb99,
            cfdrmdf24_::Rmdb99,
            Cfdrmdf24_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmdf24 {
    #[inline(always)]
    fn default() -> Cfdrmdf24 {
        <crate::RegValueT<Cfdrmdf24_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmdf24_ {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb97_SPEC;
    pub type Rmdb97 = crate::EnumBitfieldStruct<u8, Rmdb97_SPEC>;
    impl Rmdb97 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb98_SPEC;
    pub type Rmdb98 = crate::EnumBitfieldStruct<u8, Rmdb98_SPEC>;
    impl Rmdb98 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb99_SPEC;
    pub type Rmdb99 = crate::EnumBitfieldStruct<u8, Rmdb99_SPEC>;
    impl Rmdb99 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf25_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf25_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf25 = crate::RegValueT<Cfdrmdf25_SPEC>;

impl Cfdrmdf25 {
    #[inline(always)]
    pub fn rmdb100(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf25_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf25_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb101(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrmdf25_::Rmdb101,
        cfdrmdf25_::Rmdb101,
        Cfdrmdf25_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrmdf25_::Rmdb101,
            cfdrmdf25_::Rmdb101,
            Cfdrmdf25_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb102(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrmdf25_::Rmdb102,
        cfdrmdf25_::Rmdb102,
        Cfdrmdf25_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrmdf25_::Rmdb102,
            cfdrmdf25_::Rmdb102,
            Cfdrmdf25_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb103(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrmdf25_::Rmdb103,
        cfdrmdf25_::Rmdb103,
        Cfdrmdf25_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrmdf25_::Rmdb103,
            cfdrmdf25_::Rmdb103,
            Cfdrmdf25_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmdf25 {
    #[inline(always)]
    fn default() -> Cfdrmdf25 {
        <crate::RegValueT<Cfdrmdf25_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmdf25_ {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb101_SPEC;
    pub type Rmdb101 = crate::EnumBitfieldStruct<u8, Rmdb101_SPEC>;
    impl Rmdb101 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb102_SPEC;
    pub type Rmdb102 = crate::EnumBitfieldStruct<u8, Rmdb102_SPEC>;
    impl Rmdb102 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb103_SPEC;
    pub type Rmdb103 = crate::EnumBitfieldStruct<u8, Rmdb103_SPEC>;
    impl Rmdb103 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf26_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf26_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf26 = crate::RegValueT<Cfdrmdf26_SPEC>;

impl Cfdrmdf26 {
    #[inline(always)]
    pub fn rmdb104(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf26_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf26_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb105(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrmdf26_::Rmdb105,
        cfdrmdf26_::Rmdb105,
        Cfdrmdf26_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrmdf26_::Rmdb105,
            cfdrmdf26_::Rmdb105,
            Cfdrmdf26_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb106(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrmdf26_::Rmdb106,
        cfdrmdf26_::Rmdb106,
        Cfdrmdf26_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrmdf26_::Rmdb106,
            cfdrmdf26_::Rmdb106,
            Cfdrmdf26_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb107(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrmdf26_::Rmdb107,
        cfdrmdf26_::Rmdb107,
        Cfdrmdf26_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrmdf26_::Rmdb107,
            cfdrmdf26_::Rmdb107,
            Cfdrmdf26_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmdf26 {
    #[inline(always)]
    fn default() -> Cfdrmdf26 {
        <crate::RegValueT<Cfdrmdf26_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmdf26_ {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb105_SPEC;
    pub type Rmdb105 = crate::EnumBitfieldStruct<u8, Rmdb105_SPEC>;
    impl Rmdb105 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb106_SPEC;
    pub type Rmdb106 = crate::EnumBitfieldStruct<u8, Rmdb106_SPEC>;
    impl Rmdb106 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb107_SPEC;
    pub type Rmdb107 = crate::EnumBitfieldStruct<u8, Rmdb107_SPEC>;
    impl Rmdb107 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf27_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf27_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf27 = crate::RegValueT<Cfdrmdf27_SPEC>;

impl Cfdrmdf27 {
    #[inline(always)]
    pub fn rmdb108(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf27_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf27_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb109(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrmdf27_::Rmdb109,
        cfdrmdf27_::Rmdb109,
        Cfdrmdf27_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrmdf27_::Rmdb109,
            cfdrmdf27_::Rmdb109,
            Cfdrmdf27_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb110(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrmdf27_::Rmdb110,
        cfdrmdf27_::Rmdb110,
        Cfdrmdf27_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrmdf27_::Rmdb110,
            cfdrmdf27_::Rmdb110,
            Cfdrmdf27_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb111(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrmdf27_::Rmdb111,
        cfdrmdf27_::Rmdb111,
        Cfdrmdf27_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrmdf27_::Rmdb111,
            cfdrmdf27_::Rmdb111,
            Cfdrmdf27_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmdf27 {
    #[inline(always)]
    fn default() -> Cfdrmdf27 {
        <crate::RegValueT<Cfdrmdf27_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmdf27_ {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb109_SPEC;
    pub type Rmdb109 = crate::EnumBitfieldStruct<u8, Rmdb109_SPEC>;
    impl Rmdb109 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb110_SPEC;
    pub type Rmdb110 = crate::EnumBitfieldStruct<u8, Rmdb110_SPEC>;
    impl Rmdb110 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb111_SPEC;
    pub type Rmdb111 = crate::EnumBitfieldStruct<u8, Rmdb111_SPEC>;
    impl Rmdb111 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf28_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf28_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf28 = crate::RegValueT<Cfdrmdf28_SPEC>;

impl Cfdrmdf28 {
    #[inline(always)]
    pub fn rmdb112(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf28_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf28_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb113(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrmdf28_::Rmdb113,
        cfdrmdf28_::Rmdb113,
        Cfdrmdf28_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrmdf28_::Rmdb113,
            cfdrmdf28_::Rmdb113,
            Cfdrmdf28_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb114(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrmdf28_::Rmdb114,
        cfdrmdf28_::Rmdb114,
        Cfdrmdf28_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrmdf28_::Rmdb114,
            cfdrmdf28_::Rmdb114,
            Cfdrmdf28_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb115(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrmdf28_::Rmdb115,
        cfdrmdf28_::Rmdb115,
        Cfdrmdf28_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrmdf28_::Rmdb115,
            cfdrmdf28_::Rmdb115,
            Cfdrmdf28_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmdf28 {
    #[inline(always)]
    fn default() -> Cfdrmdf28 {
        <crate::RegValueT<Cfdrmdf28_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmdf28_ {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb113_SPEC;
    pub type Rmdb113 = crate::EnumBitfieldStruct<u8, Rmdb113_SPEC>;
    impl Rmdb113 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb114_SPEC;
    pub type Rmdb114 = crate::EnumBitfieldStruct<u8, Rmdb114_SPEC>;
    impl Rmdb114 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb115_SPEC;
    pub type Rmdb115 = crate::EnumBitfieldStruct<u8, Rmdb115_SPEC>;
    impl Rmdb115 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf29_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf29_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf29 = crate::RegValueT<Cfdrmdf29_SPEC>;

impl Cfdrmdf29 {
    #[inline(always)]
    pub fn rmdb116(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf29_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf29_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb117(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrmdf29_::Rmdb117,
        cfdrmdf29_::Rmdb117,
        Cfdrmdf29_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrmdf29_::Rmdb117,
            cfdrmdf29_::Rmdb117,
            Cfdrmdf29_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb118(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrmdf29_::Rmdb118,
        cfdrmdf29_::Rmdb118,
        Cfdrmdf29_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrmdf29_::Rmdb118,
            cfdrmdf29_::Rmdb118,
            Cfdrmdf29_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb119(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrmdf29_::Rmdb119,
        cfdrmdf29_::Rmdb119,
        Cfdrmdf29_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrmdf29_::Rmdb119,
            cfdrmdf29_::Rmdb119,
            Cfdrmdf29_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmdf29 {
    #[inline(always)]
    fn default() -> Cfdrmdf29 {
        <crate::RegValueT<Cfdrmdf29_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmdf29_ {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb117_SPEC;
    pub type Rmdb117 = crate::EnumBitfieldStruct<u8, Rmdb117_SPEC>;
    impl Rmdb117 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb118_SPEC;
    pub type Rmdb118 = crate::EnumBitfieldStruct<u8, Rmdb118_SPEC>;
    impl Rmdb118 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb119_SPEC;
    pub type Rmdb119 = crate::EnumBitfieldStruct<u8, Rmdb119_SPEC>;
    impl Rmdb119 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf30_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf30_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf30 = crate::RegValueT<Cfdrmdf30_SPEC>;

impl Cfdrmdf30 {
    #[inline(always)]
    pub fn rmdb120(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf30_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf30_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb121(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrmdf30_::Rmdb121,
        cfdrmdf30_::Rmdb121,
        Cfdrmdf30_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrmdf30_::Rmdb121,
            cfdrmdf30_::Rmdb121,
            Cfdrmdf30_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb122(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrmdf30_::Rmdb122,
        cfdrmdf30_::Rmdb122,
        Cfdrmdf30_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrmdf30_::Rmdb122,
            cfdrmdf30_::Rmdb122,
            Cfdrmdf30_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb123(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrmdf30_::Rmdb123,
        cfdrmdf30_::Rmdb123,
        Cfdrmdf30_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrmdf30_::Rmdb123,
            cfdrmdf30_::Rmdb123,
            Cfdrmdf30_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmdf30 {
    #[inline(always)]
    fn default() -> Cfdrmdf30 {
        <crate::RegValueT<Cfdrmdf30_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmdf30_ {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb121_SPEC;
    pub type Rmdb121 = crate::EnumBitfieldStruct<u8, Rmdb121_SPEC>;
    impl Rmdb121 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb122_SPEC;
    pub type Rmdb122 = crate::EnumBitfieldStruct<u8, Rmdb122_SPEC>;
    impl Rmdb122 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb123_SPEC;
    pub type Rmdb123 = crate::EnumBitfieldStruct<u8, Rmdb123_SPEC>;
    impl Rmdb123 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf31_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf31_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf31 = crate::RegValueT<Cfdrmdf31_SPEC>;

impl Cfdrmdf31 {
    #[inline(always)]
    pub fn rmdb124(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf31_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf31_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb125(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrmdf31_::Rmdb125,
        cfdrmdf31_::Rmdb125,
        Cfdrmdf31_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrmdf31_::Rmdb125,
            cfdrmdf31_::Rmdb125,
            Cfdrmdf31_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb126(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrmdf31_::Rmdb126,
        cfdrmdf31_::Rmdb126,
        Cfdrmdf31_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrmdf31_::Rmdb126,
            cfdrmdf31_::Rmdb126,
            Cfdrmdf31_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmdb127(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrmdf31_::Rmdb127,
        cfdrmdf31_::Rmdb127,
        Cfdrmdf31_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrmdf31_::Rmdb127,
            cfdrmdf31_::Rmdb127,
            Cfdrmdf31_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmdf31 {
    #[inline(always)]
    fn default() -> Cfdrmdf31 {
        <crate::RegValueT<Cfdrmdf31_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmdf31_ {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb125_SPEC;
    pub type Rmdb125 = crate::EnumBitfieldStruct<u8, Rmdb125_SPEC>;
    impl Rmdb125 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb126_SPEC;
    pub type Rmdb126 = crate::EnumBitfieldStruct<u8, Rmdb126_SPEC>;
    impl Rmdb126 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmdb127_SPEC;
    pub type Rmdb127 = crate::EnumBitfieldStruct<u8, Rmdb127_SPEC>;
    impl Rmdb127 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfide_SPEC;
impl crate::sealed::RegSpec for Cfdrfide_SPEC {
    type DataType = u32;
}

pub type Cfdrfide = crate::RegValueT<Cfdrfide_SPEC>;

impl Cfdrfide {
    #[inline(always)]
    pub fn rfid(
        self,
    ) -> crate::common::RegisterField<0, 0x1fffffff, 1, 0, u32, u32, Cfdrfide_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1fffffff,1,0,u32,u32,Cfdrfide_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Cfdrfide_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cfdrfide_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn rfrtr(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        cfdrfide::Rfrtr,
        cfdrfide::Rfrtr,
        Cfdrfide_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            cfdrfide::Rfrtr,
            cfdrfide::Rfrtr,
            Cfdrfide_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfide(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        cfdrfide::Rfide,
        cfdrfide::Rfide,
        Cfdrfide_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            cfdrfide::Rfide,
            cfdrfide::Rfide,
            Cfdrfide_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrfide {
    #[inline(always)]
    fn default() -> Cfdrfide {
        <crate::RegValueT<Cfdrfide_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrfide {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfrtr_SPEC;
    pub type Rfrtr = crate::EnumBitfieldStruct<u8, Rfrtr_SPEC>;
    impl Rfrtr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfide_SPEC;
    pub type Rfide = crate::EnumBitfieldStruct<u8, Rfide_SPEC>;
    impl Rfide {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfptre_SPEC;
impl crate::sealed::RegSpec for Cfdrfptre_SPEC {
    type DataType = u32;
}

pub type Cfdrfptre = crate::RegValueT<Cfdrfptre_SPEC>;

impl Cfdrfptre {
    #[inline(always)]
    pub fn rfts(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Cfdrfptre_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Cfdrfptre_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, u16, Cfdrfptre_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xfff,1,0,u16,u16,Cfdrfptre_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdlc(
        self,
    ) -> crate::common::RegisterField<
        28,
        0xf,
        1,
        0,
        cfdrfptre::Rfdlc,
        cfdrfptre::Rfdlc,
        Cfdrfptre_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            28,
            0xf,
            1,
            0,
            cfdrfptre::Rfdlc,
            cfdrfptre::Rfdlc,
            Cfdrfptre_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrfptre {
    #[inline(always)]
    fn default() -> Cfdrfptre {
        <crate::RegValueT<Cfdrfptre_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrfptre {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdlc_SPEC;
    pub type Rfdlc = crate::EnumBitfieldStruct<u8, Rfdlc_SPEC>;
    impl Rfdlc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrffdstse_SPEC;
impl crate::sealed::RegSpec for Cfdrffdstse_SPEC {
    type DataType = u32;
}

pub type Cfdrffdstse = crate::RegValueT<Cfdrffdstse_SPEC>;

impl Cfdrffdstse {
    #[inline(always)]
    pub fn rfesi(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdrffdstse::Rfesi,
        cfdrffdstse::Rfesi,
        Cfdrffdstse_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdrffdstse::Rfesi,
            cfdrffdstse::Rfesi,
            Cfdrffdstse_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfbrs(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdrffdstse::Rfbrs,
        cfdrffdstse::Rfbrs,
        Cfdrffdstse_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdrffdstse::Rfbrs,
            cfdrffdstse::Rfbrs,
            Cfdrffdstse_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rffdf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cfdrffdstse::Rffdf,
        cfdrffdstse::Rffdf,
        Cfdrffdstse_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdrffdstse::Rffdf,
            cfdrffdstse::Rffdf,
            Cfdrffdstse_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfifl(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, Cfdrffdstse_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,Cfdrffdstse_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<10, 0x3f, 1, 0, u8, u8, Cfdrffdstse_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<10,0x3f,1,0,u8,u8,Cfdrffdstse_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdrfptr(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Cfdrffdstse_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Cfdrffdstse_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrffdstse {
    #[inline(always)]
    fn default() -> Cfdrffdstse {
        <crate::RegValueT<Cfdrffdstse_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrffdstse {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfesi_SPEC;
    pub type Rfesi = crate::EnumBitfieldStruct<u8, Rfesi_SPEC>;
    impl Rfesi {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfbrs_SPEC;
    pub type Rfbrs = crate::EnumBitfieldStruct<u8, Rfbrs_SPEC>;
    impl Rfbrs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rffdf_SPEC;
    pub type Rffdf = crate::EnumBitfieldStruct<u8, Rffdf_SPEC>;
    impl Rffdf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfdf0E_SPEC;
impl crate::sealed::RegSpec for Cfdrfdf0E_SPEC {
    type DataType = u32;
}

pub type Cfdrfdf0E = crate::RegValueT<Cfdrfdf0E_SPEC>;

impl Cfdrfdf0E {
    #[inline(always)]
    pub fn rfdb0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfdf0E_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfdf0E_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb1(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrfdf0e::Rfdb1,
        cfdrfdf0e::Rfdb1,
        Cfdrfdf0E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrfdf0e::Rfdb1,
            cfdrfdf0e::Rfdb1,
            Cfdrfdf0E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb2(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrfdf0e::Rfdb2,
        cfdrfdf0e::Rfdb2,
        Cfdrfdf0E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrfdf0e::Rfdb2,
            cfdrfdf0e::Rfdb2,
            Cfdrfdf0E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb3(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrfdf0e::Rfdb3,
        cfdrfdf0e::Rfdb3,
        Cfdrfdf0E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrfdf0e::Rfdb3,
            cfdrfdf0e::Rfdb3,
            Cfdrfdf0E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrfdf0E {
    #[inline(always)]
    fn default() -> Cfdrfdf0E {
        <crate::RegValueT<Cfdrfdf0E_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrfdf0e {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb1_SPEC;
    pub type Rfdb1 = crate::EnumBitfieldStruct<u8, Rfdb1_SPEC>;
    impl Rfdb1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb2_SPEC;
    pub type Rfdb2 = crate::EnumBitfieldStruct<u8, Rfdb2_SPEC>;
    impl Rfdb2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb3_SPEC;
    pub type Rfdb3 = crate::EnumBitfieldStruct<u8, Rfdb3_SPEC>;
    impl Rfdb3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfdf1E_SPEC;
impl crate::sealed::RegSpec for Cfdrfdf1E_SPEC {
    type DataType = u32;
}

pub type Cfdrfdf1E = crate::RegValueT<Cfdrfdf1E_SPEC>;

impl Cfdrfdf1E {
    #[inline(always)]
    pub fn rfdb4(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfdf1E_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfdf1E_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb5(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrfdf1e::Rfdb5,
        cfdrfdf1e::Rfdb5,
        Cfdrfdf1E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrfdf1e::Rfdb5,
            cfdrfdf1e::Rfdb5,
            Cfdrfdf1E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb6(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrfdf1e::Rfdb6,
        cfdrfdf1e::Rfdb6,
        Cfdrfdf1E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrfdf1e::Rfdb6,
            cfdrfdf1e::Rfdb6,
            Cfdrfdf1E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb7(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrfdf1e::Rfdb7,
        cfdrfdf1e::Rfdb7,
        Cfdrfdf1E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrfdf1e::Rfdb7,
            cfdrfdf1e::Rfdb7,
            Cfdrfdf1E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrfdf1E {
    #[inline(always)]
    fn default() -> Cfdrfdf1E {
        <crate::RegValueT<Cfdrfdf1E_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrfdf1e {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb5_SPEC;
    pub type Rfdb5 = crate::EnumBitfieldStruct<u8, Rfdb5_SPEC>;
    impl Rfdb5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb6_SPEC;
    pub type Rfdb6 = crate::EnumBitfieldStruct<u8, Rfdb6_SPEC>;
    impl Rfdb6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb7_SPEC;
    pub type Rfdb7 = crate::EnumBitfieldStruct<u8, Rfdb7_SPEC>;
    impl Rfdb7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfdf2E_SPEC;
impl crate::sealed::RegSpec for Cfdrfdf2E_SPEC {
    type DataType = u32;
}

pub type Cfdrfdf2E = crate::RegValueT<Cfdrfdf2E_SPEC>;

impl Cfdrfdf2E {
    #[inline(always)]
    pub fn rfdb8(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfdf2E_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfdf2E_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb9(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrfdf2e::Rfdb9,
        cfdrfdf2e::Rfdb9,
        Cfdrfdf2E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrfdf2e::Rfdb9,
            cfdrfdf2e::Rfdb9,
            Cfdrfdf2E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb10(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrfdf2e::Rfdb10,
        cfdrfdf2e::Rfdb10,
        Cfdrfdf2E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrfdf2e::Rfdb10,
            cfdrfdf2e::Rfdb10,
            Cfdrfdf2E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb11(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrfdf2e::Rfdb11,
        cfdrfdf2e::Rfdb11,
        Cfdrfdf2E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrfdf2e::Rfdb11,
            cfdrfdf2e::Rfdb11,
            Cfdrfdf2E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrfdf2E {
    #[inline(always)]
    fn default() -> Cfdrfdf2E {
        <crate::RegValueT<Cfdrfdf2E_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrfdf2e {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb9_SPEC;
    pub type Rfdb9 = crate::EnumBitfieldStruct<u8, Rfdb9_SPEC>;
    impl Rfdb9 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb10_SPEC;
    pub type Rfdb10 = crate::EnumBitfieldStruct<u8, Rfdb10_SPEC>;
    impl Rfdb10 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb11_SPEC;
    pub type Rfdb11 = crate::EnumBitfieldStruct<u8, Rfdb11_SPEC>;
    impl Rfdb11 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfdf3E_SPEC;
impl crate::sealed::RegSpec for Cfdrfdf3E_SPEC {
    type DataType = u32;
}

pub type Cfdrfdf3E = crate::RegValueT<Cfdrfdf3E_SPEC>;

impl Cfdrfdf3E {
    #[inline(always)]
    pub fn rfdb12(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfdf3E_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfdf3E_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb13(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrfdf3e::Rfdb13,
        cfdrfdf3e::Rfdb13,
        Cfdrfdf3E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrfdf3e::Rfdb13,
            cfdrfdf3e::Rfdb13,
            Cfdrfdf3E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb14(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrfdf3e::Rfdb14,
        cfdrfdf3e::Rfdb14,
        Cfdrfdf3E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrfdf3e::Rfdb14,
            cfdrfdf3e::Rfdb14,
            Cfdrfdf3E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb15(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrfdf3e::Rfdb15,
        cfdrfdf3e::Rfdb15,
        Cfdrfdf3E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrfdf3e::Rfdb15,
            cfdrfdf3e::Rfdb15,
            Cfdrfdf3E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrfdf3E {
    #[inline(always)]
    fn default() -> Cfdrfdf3E {
        <crate::RegValueT<Cfdrfdf3E_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrfdf3e {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb13_SPEC;
    pub type Rfdb13 = crate::EnumBitfieldStruct<u8, Rfdb13_SPEC>;
    impl Rfdb13 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb14_SPEC;
    pub type Rfdb14 = crate::EnumBitfieldStruct<u8, Rfdb14_SPEC>;
    impl Rfdb14 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb15_SPEC;
    pub type Rfdb15 = crate::EnumBitfieldStruct<u8, Rfdb15_SPEC>;
    impl Rfdb15 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfdf4E_SPEC;
impl crate::sealed::RegSpec for Cfdrfdf4E_SPEC {
    type DataType = u32;
}

pub type Cfdrfdf4E = crate::RegValueT<Cfdrfdf4E_SPEC>;

impl Cfdrfdf4E {
    #[inline(always)]
    pub fn rfdb16(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfdf4E_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfdf4E_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb17(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrfdf4e::Rfdb17,
        cfdrfdf4e::Rfdb17,
        Cfdrfdf4E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrfdf4e::Rfdb17,
            cfdrfdf4e::Rfdb17,
            Cfdrfdf4E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb18(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrfdf4e::Rfdb18,
        cfdrfdf4e::Rfdb18,
        Cfdrfdf4E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrfdf4e::Rfdb18,
            cfdrfdf4e::Rfdb18,
            Cfdrfdf4E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb19(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrfdf4e::Rfdb19,
        cfdrfdf4e::Rfdb19,
        Cfdrfdf4E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrfdf4e::Rfdb19,
            cfdrfdf4e::Rfdb19,
            Cfdrfdf4E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrfdf4E {
    #[inline(always)]
    fn default() -> Cfdrfdf4E {
        <crate::RegValueT<Cfdrfdf4E_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrfdf4e {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb17_SPEC;
    pub type Rfdb17 = crate::EnumBitfieldStruct<u8, Rfdb17_SPEC>;
    impl Rfdb17 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb18_SPEC;
    pub type Rfdb18 = crate::EnumBitfieldStruct<u8, Rfdb18_SPEC>;
    impl Rfdb18 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb19_SPEC;
    pub type Rfdb19 = crate::EnumBitfieldStruct<u8, Rfdb19_SPEC>;
    impl Rfdb19 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfdf5E_SPEC;
impl crate::sealed::RegSpec for Cfdrfdf5E_SPEC {
    type DataType = u32;
}

pub type Cfdrfdf5E = crate::RegValueT<Cfdrfdf5E_SPEC>;

impl Cfdrfdf5E {
    #[inline(always)]
    pub fn rfdb20(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfdf5E_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfdf5E_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb21(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrfdf5e::Rfdb21,
        cfdrfdf5e::Rfdb21,
        Cfdrfdf5E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrfdf5e::Rfdb21,
            cfdrfdf5e::Rfdb21,
            Cfdrfdf5E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb22(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrfdf5e::Rfdb22,
        cfdrfdf5e::Rfdb22,
        Cfdrfdf5E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrfdf5e::Rfdb22,
            cfdrfdf5e::Rfdb22,
            Cfdrfdf5E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb23(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrfdf5e::Rfdb23,
        cfdrfdf5e::Rfdb23,
        Cfdrfdf5E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrfdf5e::Rfdb23,
            cfdrfdf5e::Rfdb23,
            Cfdrfdf5E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrfdf5E {
    #[inline(always)]
    fn default() -> Cfdrfdf5E {
        <crate::RegValueT<Cfdrfdf5E_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrfdf5e {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb21_SPEC;
    pub type Rfdb21 = crate::EnumBitfieldStruct<u8, Rfdb21_SPEC>;
    impl Rfdb21 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb22_SPEC;
    pub type Rfdb22 = crate::EnumBitfieldStruct<u8, Rfdb22_SPEC>;
    impl Rfdb22 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb23_SPEC;
    pub type Rfdb23 = crate::EnumBitfieldStruct<u8, Rfdb23_SPEC>;
    impl Rfdb23 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfdf6E_SPEC;
impl crate::sealed::RegSpec for Cfdrfdf6E_SPEC {
    type DataType = u32;
}

pub type Cfdrfdf6E = crate::RegValueT<Cfdrfdf6E_SPEC>;

impl Cfdrfdf6E {
    #[inline(always)]
    pub fn rfdb24(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfdf6E_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfdf6E_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb25(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrfdf6e::Rfdb25,
        cfdrfdf6e::Rfdb25,
        Cfdrfdf6E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrfdf6e::Rfdb25,
            cfdrfdf6e::Rfdb25,
            Cfdrfdf6E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb26(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrfdf6e::Rfdb26,
        cfdrfdf6e::Rfdb26,
        Cfdrfdf6E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrfdf6e::Rfdb26,
            cfdrfdf6e::Rfdb26,
            Cfdrfdf6E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb27(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrfdf6e::Rfdb27,
        cfdrfdf6e::Rfdb27,
        Cfdrfdf6E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrfdf6e::Rfdb27,
            cfdrfdf6e::Rfdb27,
            Cfdrfdf6E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrfdf6E {
    #[inline(always)]
    fn default() -> Cfdrfdf6E {
        <crate::RegValueT<Cfdrfdf6E_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrfdf6e {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb25_SPEC;
    pub type Rfdb25 = crate::EnumBitfieldStruct<u8, Rfdb25_SPEC>;
    impl Rfdb25 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb26_SPEC;
    pub type Rfdb26 = crate::EnumBitfieldStruct<u8, Rfdb26_SPEC>;
    impl Rfdb26 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb27_SPEC;
    pub type Rfdb27 = crate::EnumBitfieldStruct<u8, Rfdb27_SPEC>;
    impl Rfdb27 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfdf7E_SPEC;
impl crate::sealed::RegSpec for Cfdrfdf7E_SPEC {
    type DataType = u32;
}

pub type Cfdrfdf7E = crate::RegValueT<Cfdrfdf7E_SPEC>;

impl Cfdrfdf7E {
    #[inline(always)]
    pub fn rfdb28(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfdf7E_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfdf7E_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb29(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrfdf7e::Rfdb29,
        cfdrfdf7e::Rfdb29,
        Cfdrfdf7E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrfdf7e::Rfdb29,
            cfdrfdf7e::Rfdb29,
            Cfdrfdf7E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb30(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrfdf7e::Rfdb30,
        cfdrfdf7e::Rfdb30,
        Cfdrfdf7E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrfdf7e::Rfdb30,
            cfdrfdf7e::Rfdb30,
            Cfdrfdf7E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb31(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrfdf7e::Rfdb31,
        cfdrfdf7e::Rfdb31,
        Cfdrfdf7E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrfdf7e::Rfdb31,
            cfdrfdf7e::Rfdb31,
            Cfdrfdf7E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrfdf7E {
    #[inline(always)]
    fn default() -> Cfdrfdf7E {
        <crate::RegValueT<Cfdrfdf7E_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrfdf7e {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb29_SPEC;
    pub type Rfdb29 = crate::EnumBitfieldStruct<u8, Rfdb29_SPEC>;
    impl Rfdb29 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb30_SPEC;
    pub type Rfdb30 = crate::EnumBitfieldStruct<u8, Rfdb30_SPEC>;
    impl Rfdb30 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb31_SPEC;
    pub type Rfdb31 = crate::EnumBitfieldStruct<u8, Rfdb31_SPEC>;
    impl Rfdb31 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfdf8E_SPEC;
impl crate::sealed::RegSpec for Cfdrfdf8E_SPEC {
    type DataType = u32;
}

pub type Cfdrfdf8E = crate::RegValueT<Cfdrfdf8E_SPEC>;

impl Cfdrfdf8E {
    #[inline(always)]
    pub fn rfdb32(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfdf8E_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfdf8E_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb33(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrfdf8e::Rfdb33,
        cfdrfdf8e::Rfdb33,
        Cfdrfdf8E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrfdf8e::Rfdb33,
            cfdrfdf8e::Rfdb33,
            Cfdrfdf8E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb34(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrfdf8e::Rfdb34,
        cfdrfdf8e::Rfdb34,
        Cfdrfdf8E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrfdf8e::Rfdb34,
            cfdrfdf8e::Rfdb34,
            Cfdrfdf8E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb35(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrfdf8e::Rfdb35,
        cfdrfdf8e::Rfdb35,
        Cfdrfdf8E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrfdf8e::Rfdb35,
            cfdrfdf8e::Rfdb35,
            Cfdrfdf8E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrfdf8E {
    #[inline(always)]
    fn default() -> Cfdrfdf8E {
        <crate::RegValueT<Cfdrfdf8E_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrfdf8e {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb33_SPEC;
    pub type Rfdb33 = crate::EnumBitfieldStruct<u8, Rfdb33_SPEC>;
    impl Rfdb33 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb34_SPEC;
    pub type Rfdb34 = crate::EnumBitfieldStruct<u8, Rfdb34_SPEC>;
    impl Rfdb34 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb35_SPEC;
    pub type Rfdb35 = crate::EnumBitfieldStruct<u8, Rfdb35_SPEC>;
    impl Rfdb35 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfdf9E_SPEC;
impl crate::sealed::RegSpec for Cfdrfdf9E_SPEC {
    type DataType = u32;
}

pub type Cfdrfdf9E = crate::RegValueT<Cfdrfdf9E_SPEC>;

impl Cfdrfdf9E {
    #[inline(always)]
    pub fn rfdb36(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfdf9E_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfdf9E_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb37(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrfdf9e::Rfdb37,
        cfdrfdf9e::Rfdb37,
        Cfdrfdf9E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrfdf9e::Rfdb37,
            cfdrfdf9e::Rfdb37,
            Cfdrfdf9E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb38(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrfdf9e::Rfdb38,
        cfdrfdf9e::Rfdb38,
        Cfdrfdf9E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrfdf9e::Rfdb38,
            cfdrfdf9e::Rfdb38,
            Cfdrfdf9E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb39(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrfdf9e::Rfdb39,
        cfdrfdf9e::Rfdb39,
        Cfdrfdf9E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrfdf9e::Rfdb39,
            cfdrfdf9e::Rfdb39,
            Cfdrfdf9E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrfdf9E {
    #[inline(always)]
    fn default() -> Cfdrfdf9E {
        <crate::RegValueT<Cfdrfdf9E_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrfdf9e {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb37_SPEC;
    pub type Rfdb37 = crate::EnumBitfieldStruct<u8, Rfdb37_SPEC>;
    impl Rfdb37 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb38_SPEC;
    pub type Rfdb38 = crate::EnumBitfieldStruct<u8, Rfdb38_SPEC>;
    impl Rfdb38 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb39_SPEC;
    pub type Rfdb39 = crate::EnumBitfieldStruct<u8, Rfdb39_SPEC>;
    impl Rfdb39 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfdf10E_SPEC;
impl crate::sealed::RegSpec for Cfdrfdf10E_SPEC {
    type DataType = u32;
}

pub type Cfdrfdf10E = crate::RegValueT<Cfdrfdf10E_SPEC>;

impl Cfdrfdf10E {
    #[inline(always)]
    pub fn rfdb40(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfdf10E_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfdf10E_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb41(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrfdf10e::Rfdb41,
        cfdrfdf10e::Rfdb41,
        Cfdrfdf10E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrfdf10e::Rfdb41,
            cfdrfdf10e::Rfdb41,
            Cfdrfdf10E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb42(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrfdf10e::Rfdb42,
        cfdrfdf10e::Rfdb42,
        Cfdrfdf10E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrfdf10e::Rfdb42,
            cfdrfdf10e::Rfdb42,
            Cfdrfdf10E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb43(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrfdf10e::Rfdb43,
        cfdrfdf10e::Rfdb43,
        Cfdrfdf10E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrfdf10e::Rfdb43,
            cfdrfdf10e::Rfdb43,
            Cfdrfdf10E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrfdf10E {
    #[inline(always)]
    fn default() -> Cfdrfdf10E {
        <crate::RegValueT<Cfdrfdf10E_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrfdf10e {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb41_SPEC;
    pub type Rfdb41 = crate::EnumBitfieldStruct<u8, Rfdb41_SPEC>;
    impl Rfdb41 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb42_SPEC;
    pub type Rfdb42 = crate::EnumBitfieldStruct<u8, Rfdb42_SPEC>;
    impl Rfdb42 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb43_SPEC;
    pub type Rfdb43 = crate::EnumBitfieldStruct<u8, Rfdb43_SPEC>;
    impl Rfdb43 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfdf11E_SPEC;
impl crate::sealed::RegSpec for Cfdrfdf11E_SPEC {
    type DataType = u32;
}

pub type Cfdrfdf11E = crate::RegValueT<Cfdrfdf11E_SPEC>;

impl Cfdrfdf11E {
    #[inline(always)]
    pub fn rfdb44(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfdf11E_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfdf11E_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb45(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrfdf11e::Rfdb45,
        cfdrfdf11e::Rfdb45,
        Cfdrfdf11E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrfdf11e::Rfdb45,
            cfdrfdf11e::Rfdb45,
            Cfdrfdf11E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb46(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrfdf11e::Rfdb46,
        cfdrfdf11e::Rfdb46,
        Cfdrfdf11E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrfdf11e::Rfdb46,
            cfdrfdf11e::Rfdb46,
            Cfdrfdf11E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb47(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrfdf11e::Rfdb47,
        cfdrfdf11e::Rfdb47,
        Cfdrfdf11E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrfdf11e::Rfdb47,
            cfdrfdf11e::Rfdb47,
            Cfdrfdf11E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrfdf11E {
    #[inline(always)]
    fn default() -> Cfdrfdf11E {
        <crate::RegValueT<Cfdrfdf11E_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrfdf11e {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb45_SPEC;
    pub type Rfdb45 = crate::EnumBitfieldStruct<u8, Rfdb45_SPEC>;
    impl Rfdb45 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb46_SPEC;
    pub type Rfdb46 = crate::EnumBitfieldStruct<u8, Rfdb46_SPEC>;
    impl Rfdb46 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb47_SPEC;
    pub type Rfdb47 = crate::EnumBitfieldStruct<u8, Rfdb47_SPEC>;
    impl Rfdb47 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfdf12E_SPEC;
impl crate::sealed::RegSpec for Cfdrfdf12E_SPEC {
    type DataType = u32;
}

pub type Cfdrfdf12E = crate::RegValueT<Cfdrfdf12E_SPEC>;

impl Cfdrfdf12E {
    #[inline(always)]
    pub fn rfdb48(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfdf12E_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfdf12E_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb49(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrfdf12e::Rfdb49,
        cfdrfdf12e::Rfdb49,
        Cfdrfdf12E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrfdf12e::Rfdb49,
            cfdrfdf12e::Rfdb49,
            Cfdrfdf12E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb50(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrfdf12e::Rfdb50,
        cfdrfdf12e::Rfdb50,
        Cfdrfdf12E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrfdf12e::Rfdb50,
            cfdrfdf12e::Rfdb50,
            Cfdrfdf12E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb51(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrfdf12e::Rfdb51,
        cfdrfdf12e::Rfdb51,
        Cfdrfdf12E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrfdf12e::Rfdb51,
            cfdrfdf12e::Rfdb51,
            Cfdrfdf12E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrfdf12E {
    #[inline(always)]
    fn default() -> Cfdrfdf12E {
        <crate::RegValueT<Cfdrfdf12E_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrfdf12e {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb49_SPEC;
    pub type Rfdb49 = crate::EnumBitfieldStruct<u8, Rfdb49_SPEC>;
    impl Rfdb49 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb50_SPEC;
    pub type Rfdb50 = crate::EnumBitfieldStruct<u8, Rfdb50_SPEC>;
    impl Rfdb50 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb51_SPEC;
    pub type Rfdb51 = crate::EnumBitfieldStruct<u8, Rfdb51_SPEC>;
    impl Rfdb51 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfdf13E_SPEC;
impl crate::sealed::RegSpec for Cfdrfdf13E_SPEC {
    type DataType = u32;
}

pub type Cfdrfdf13E = crate::RegValueT<Cfdrfdf13E_SPEC>;

impl Cfdrfdf13E {
    #[inline(always)]
    pub fn rfdb52(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfdf13E_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfdf13E_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb53(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrfdf13e::Rfdb53,
        cfdrfdf13e::Rfdb53,
        Cfdrfdf13E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrfdf13e::Rfdb53,
            cfdrfdf13e::Rfdb53,
            Cfdrfdf13E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb54(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrfdf13e::Rfdb54,
        cfdrfdf13e::Rfdb54,
        Cfdrfdf13E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrfdf13e::Rfdb54,
            cfdrfdf13e::Rfdb54,
            Cfdrfdf13E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb55(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrfdf13e::Rfdb55,
        cfdrfdf13e::Rfdb55,
        Cfdrfdf13E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrfdf13e::Rfdb55,
            cfdrfdf13e::Rfdb55,
            Cfdrfdf13E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrfdf13E {
    #[inline(always)]
    fn default() -> Cfdrfdf13E {
        <crate::RegValueT<Cfdrfdf13E_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrfdf13e {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb53_SPEC;
    pub type Rfdb53 = crate::EnumBitfieldStruct<u8, Rfdb53_SPEC>;
    impl Rfdb53 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb54_SPEC;
    pub type Rfdb54 = crate::EnumBitfieldStruct<u8, Rfdb54_SPEC>;
    impl Rfdb54 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb55_SPEC;
    pub type Rfdb55 = crate::EnumBitfieldStruct<u8, Rfdb55_SPEC>;
    impl Rfdb55 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfdf14E_SPEC;
impl crate::sealed::RegSpec for Cfdrfdf14E_SPEC {
    type DataType = u32;
}

pub type Cfdrfdf14E = crate::RegValueT<Cfdrfdf14E_SPEC>;

impl Cfdrfdf14E {
    #[inline(always)]
    pub fn rfdb56(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfdf14E_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfdf14E_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb57(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrfdf14e::Rfdb57,
        cfdrfdf14e::Rfdb57,
        Cfdrfdf14E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrfdf14e::Rfdb57,
            cfdrfdf14e::Rfdb57,
            Cfdrfdf14E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb58(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrfdf14e::Rfdb58,
        cfdrfdf14e::Rfdb58,
        Cfdrfdf14E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrfdf14e::Rfdb58,
            cfdrfdf14e::Rfdb58,
            Cfdrfdf14E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb59(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrfdf14e::Rfdb59,
        cfdrfdf14e::Rfdb59,
        Cfdrfdf14E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrfdf14e::Rfdb59,
            cfdrfdf14e::Rfdb59,
            Cfdrfdf14E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrfdf14E {
    #[inline(always)]
    fn default() -> Cfdrfdf14E {
        <crate::RegValueT<Cfdrfdf14E_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrfdf14e {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb57_SPEC;
    pub type Rfdb57 = crate::EnumBitfieldStruct<u8, Rfdb57_SPEC>;
    impl Rfdb57 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb58_SPEC;
    pub type Rfdb58 = crate::EnumBitfieldStruct<u8, Rfdb58_SPEC>;
    impl Rfdb58 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb59_SPEC;
    pub type Rfdb59 = crate::EnumBitfieldStruct<u8, Rfdb59_SPEC>;
    impl Rfdb59 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrfdf15E_SPEC;
impl crate::sealed::RegSpec for Cfdrfdf15E_SPEC {
    type DataType = u32;
}

pub type Cfdrfdf15E = crate::RegValueT<Cfdrfdf15E_SPEC>;

impl Cfdrfdf15E {
    #[inline(always)]
    pub fn rfdb60(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfdf15E_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfdf15E_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb61(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdrfdf15e::Rfdb61,
        cfdrfdf15e::Rfdb61,
        Cfdrfdf15E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdrfdf15e::Rfdb61,
            cfdrfdf15e::Rfdb61,
            Cfdrfdf15E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb62(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrfdf15e::Rfdb62,
        cfdrfdf15e::Rfdb62,
        Cfdrfdf15E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrfdf15e::Rfdb62,
            cfdrfdf15e::Rfdb62,
            Cfdrfdf15E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdb63(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdrfdf15e::Rfdb63,
        cfdrfdf15e::Rfdb63,
        Cfdrfdf15E_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdrfdf15e::Rfdb63,
            cfdrfdf15e::Rfdb63,
            Cfdrfdf15E_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrfdf15E {
    #[inline(always)]
    fn default() -> Cfdrfdf15E {
        <crate::RegValueT<Cfdrfdf15E_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrfdf15e {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb61_SPEC;
    pub type Rfdb61 = crate::EnumBitfieldStruct<u8, Rfdb61_SPEC>;
    impl Rfdb61 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb62_SPEC;
    pub type Rfdb62 = crate::EnumBitfieldStruct<u8, Rfdb62_SPEC>;
    impl Rfdb62 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdb63_SPEC;
    pub type Rfdb63 = crate::EnumBitfieldStruct<u8, Rfdb63_SPEC>;
    impl Rfdb63 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfide_SPEC;
impl crate::sealed::RegSpec for Cfdcfide_SPEC {
    type DataType = u32;
}

pub type Cfdcfide = crate::RegValueT<Cfdcfide_SPEC>;

impl Cfdcfide {
    #[inline(always)]
    pub fn cfid(
        self,
    ) -> crate::common::RegisterField<0, 0x1fffffff, 1, 0, u32, u32, Cfdcfide_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1fffffff,
            1,
            0,
            u32,
            u32,
            Cfdcfide_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn thlen(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        cfdcfide::Thlen,
        cfdcfide::Thlen,
        Cfdcfide_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            cfdcfide::Thlen,
            cfdcfide::Thlen,
            Cfdcfide_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfrtr(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        cfdcfide::Cfrtr,
        cfdcfide::Cfrtr,
        Cfdcfide_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            cfdcfide::Cfrtr,
            cfdcfide::Cfrtr,
            Cfdcfide_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfide(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        cfdcfide::Cfide,
        cfdcfide::Cfide,
        Cfdcfide_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            cfdcfide::Cfide,
            cfdcfide::Cfide,
            Cfdcfide_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcfide {
    #[inline(always)]
    fn default() -> Cfdcfide {
        <crate::RegValueT<Cfdcfide_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcfide {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Thlen_SPEC;
    pub type Thlen = crate::EnumBitfieldStruct<u8, Thlen_SPEC>;
    impl Thlen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfrtr_SPEC;
    pub type Cfrtr = crate::EnumBitfieldStruct<u8, Cfrtr_SPEC>;
    impl Cfrtr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfide_SPEC;
    pub type Cfide = crate::EnumBitfieldStruct<u8, Cfide_SPEC>;
    impl Cfide {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfptre_SPEC;
impl crate::sealed::RegSpec for Cfdcfptre_SPEC {
    type DataType = u32;
}

pub type Cfdcfptre = crate::RegValueT<Cfdcfptre_SPEC>;

impl Cfdcfptre {
    #[inline(always)]
    pub fn cfts(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Cfdcfptre_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Cfdcfptre_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, u16, Cfdcfptre_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xfff,1,0,u16,u16,Cfdcfptre_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdlc(
        self,
    ) -> crate::common::RegisterField<
        28,
        0xf,
        1,
        0,
        cfdcfptre::Cfdlc,
        cfdcfptre::Cfdlc,
        Cfdcfptre_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0xf,
            1,
            0,
            cfdcfptre::Cfdlc,
            cfdcfptre::Cfdlc,
            Cfdcfptre_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcfptre {
    #[inline(always)]
    fn default() -> Cfdcfptre {
        <crate::RegValueT<Cfdcfptre_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcfptre {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdlc_SPEC;
    pub type Cfdlc = crate::EnumBitfieldStruct<u8, Cfdlc_SPEC>;
    impl Cfdlc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcffdcstse_SPEC;
impl crate::sealed::RegSpec for Cfdcffdcstse_SPEC {
    type DataType = u32;
}

pub type Cfdcffdcstse = crate::RegValueT<Cfdcffdcstse_SPEC>;

impl Cfdcffdcstse {
    #[inline(always)]
    pub fn cfesi(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdcffdcstse::Cfesi,
        cfdcffdcstse::Cfesi,
        Cfdcffdcstse_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdcffdcstse::Cfesi,
            cfdcffdcstse::Cfesi,
            Cfdcffdcstse_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfbrs(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdcffdcstse::Cfbrs,
        cfdcffdcstse::Cfbrs,
        Cfdcffdcstse_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdcffdcstse::Cfbrs,
            cfdcffdcstse::Cfbrs,
            Cfdcffdcstse_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cffdf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cfdcffdcstse::Cffdf,
        cfdcffdcstse::Cffdf,
        Cfdcffdcstse_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdcffdcstse::Cffdf,
            cfdcffdcstse::Cffdf,
            Cfdcffdcstse_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfifl(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, Cfdcffdcstse_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,Cfdcffdcstse_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<10, 0x3f, 1, 0, u8, u8, Cfdcffdcstse_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3f,1,0,u8,u8,Cfdcffdcstse_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfptr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        Cfdcffdcstse_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            Cfdcffdcstse_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcffdcstse {
    #[inline(always)]
    fn default() -> Cfdcffdcstse {
        <crate::RegValueT<Cfdcffdcstse_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcffdcstse {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfesi_SPEC;
    pub type Cfesi = crate::EnumBitfieldStruct<u8, Cfesi_SPEC>;
    impl Cfesi {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfbrs_SPEC;
    pub type Cfbrs = crate::EnumBitfieldStruct<u8, Cfbrs_SPEC>;
    impl Cfbrs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cffdf_SPEC;
    pub type Cffdf = crate::EnumBitfieldStruct<u8, Cffdf_SPEC>;
    impl Cffdf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdfe_SPEC;
impl crate::sealed::RegSpec for Cfdcfdfe_SPEC {
    type DataType = u32;
}

pub type Cfdcfdfe = crate::RegValueT<Cfdcfdfe_SPEC>;

impl Cfdcfdfe {
    #[inline(always)]
    pub fn cfdb0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdfe_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdfe_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb1(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        cfdcfdfe::Cfdb1,
        cfdcfdfe::Cfdb1,
        Cfdcfdfe_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            cfdcfdfe::Cfdb1,
            cfdcfdfe::Cfdb1,
            Cfdcfdfe_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdb2(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdcfdfe::Cfdb2,
        cfdcfdfe::Cfdb2,
        Cfdcfdfe_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdcfdfe::Cfdb2,
            cfdcfdfe::Cfdb2,
            Cfdcfdfe_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdb3(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cfdcfdfe::Cfdb3,
        cfdcfdfe::Cfdb3,
        Cfdcfdfe_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cfdcfdfe::Cfdb3,
            cfdcfdfe::Cfdb3,
            Cfdcfdfe_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcfdfe {
    #[inline(always)]
    fn default() -> Cfdcfdfe {
        <crate::RegValueT<Cfdcfdfe_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcfdfe {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb1_SPEC;
    pub type Cfdb1 = crate::EnumBitfieldStruct<u8, Cfdb1_SPEC>;
    impl Cfdb1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb2_SPEC;
    pub type Cfdb2 = crate::EnumBitfieldStruct<u8, Cfdb2_SPEC>;
    impl Cfdb2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdb3_SPEC;
    pub type Cfdb3 = crate::EnumBitfieldStruct<u8, Cfdb3_SPEC>;
    impl Cfdb3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
