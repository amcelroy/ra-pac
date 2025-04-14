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
// Generated from SVD 1.00.01, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:22:53 +0000

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
        crate::common::Reg<self::Cfdrfpctr_SPEC, crate::common::W>,
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
    ) -> &'static crate::common::Reg<self::Cfdcfpctr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Cfdcfpctr_SPEC, crate::common::W>::from_ptr(
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
    ) -> &'static crate::common::Reg<self::Cfdcdtsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cfdcdtsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(204usize),
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
    ) -> &'static crate::common::Reg<self::Cfdc0Fdcrc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cfdc0Fdcrc_SPEC, crate::common::R>::from_ptr(
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
    pub const fn cfdrmid(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmid_SPEC, crate::common::R>,
        8,
        0x4c,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xd20usize))
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
    ) -> crate::common::RegisterField<10, 0x7f, 1, 0, u8, u8, Cfdc0Ncfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x7f,1,0,u8,u8,Cfdc0Ncfg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ntseg1(
        self,
    ) -> crate::common::RegisterField<17, 0xff, 1, 0, u8, u8, Cfdc0Ncfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0xff,1,0,u8,u8,Cfdc0Ncfg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ntseg2(
        self,
    ) -> crate::common::RegisterField<25, 0x7f, 1, 0, u8, u8, Cfdc0Ncfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x7f,1,0,u8,u8,Cfdc0Ncfg_SPEC,crate::common::RW>::from_register(self,0)
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
    pub fn rec(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdc0Sts_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdc0Sts_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tec(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdc0Sts_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdc0Sts_SPEC,crate::common::R>::from_register(self,0)
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

        pub const _1: Self = Self::new(1);
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
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, Cfdgcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,Cfdgcfg_SPEC,crate::common::RW>::from_register(self,0)
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
    pub fn eef0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        cfdgerfl::Eef0,
        cfdgerfl::Eef0,
        Cfdgerfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            cfdgerfl::Eef0,
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
        pub const _0: Self = Self::new(0);

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
    pub struct Eef0_SPEC;
    pub type Eef0 = crate::EnumBitfieldStruct<u8, Eef0_SPEC>;
    impl Eef0 {
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
    pub fn rnc0(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Cfdgaflcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Cfdgaflcfg_SPEC,crate::common::RW>::from_register(self,0)
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
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Cfdrmnb_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Cfdrmnb_SPEC,crate::common::RW>::from_register(self,0)
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
    type DataType = u16;
}

pub type Cfdrmnd = crate::RegValueT<Cfdrmnd_SPEC>;

impl Cfdrmnd {
    #[inline(always)]
    pub fn rmns(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        cfdrmnd::Rmns,
        cfdrmnd::Rmns,
        Cfdrmnd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            cfdrmnd::Rmns,
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmiec_SPEC;
impl crate::sealed::RegSpec for Cfdrmiec_SPEC {
    type DataType = u16;
}

pub type Cfdrmiec = crate::RegValueT<Cfdrmiec_SPEC>;

impl Cfdrmiec {
    #[inline(always)]
    pub fn rmie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        cfdrmiec::Rmie,
        cfdrmiec::Rmie,
        Cfdrmiec_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            cfdrmiec::Rmie,
            cfdrmiec::Rmie,
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
    pub struct Rmie_SPEC;
    pub type Rmie = crate::EnumBitfieldStruct<u8, Rmie_SPEC>;
    impl Rmie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
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
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfpctr_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfpctr_SPEC,crate::common::W>::from_register(self,0)
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
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        cfdcfcc::Cfim,
        cfdcfcc::Cfim,
        Cfdcfcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            cfdcfcc::Cfim,
            cfdcfcc::Cfim,
            Cfdcfcc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const _101: Self = Self::new(5);

        pub const _110: Self = Self::new(6);

        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfm_SPEC;
    pub type Cfm = crate::EnumBitfieldStruct<u8, Cfm_SPEC>;
    impl Cfm {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
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
    pub struct Cfim_SPEC;
    pub type Cfim = crate::EnumBitfieldStruct<u8, Cfim_SPEC>;
    impl Cfim {
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
    pub fn rfxemp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        cfdfests::Rfxemp,
        cfdfests::Rfxemp,
        Cfdfests_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            cfdfests::Rfxemp,
            cfdfests::Rfxemp,
            Cfdfests_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfemp(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cfdfests::Cfemp,
        cfdfests::Cfemp,
        Cfdfests_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cfdfests::Cfemp,
            cfdfests::Cfemp,
            Cfdfests_SPEC,
            crate::common::R,
        >::from_register(self, 0)
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfemp_SPEC;
    pub type Cfemp = crate::EnumBitfieldStruct<u8, Cfemp_SPEC>;
    impl Cfemp {
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
    pub fn rfxfll(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        cfdffsts::Rfxfll,
        cfdffsts::Rfxfll,
        Cfdffsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            cfdffsts::Rfxfll,
            cfdffsts::Rfxfll,
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
    pub fn rfxmlt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        cfdfmsts::Rfxmlt,
        cfdfmsts::Rfxmlt,
        Cfdfmsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            cfdfmsts::Rfxmlt,
            cfdfmsts::Rfxmlt,
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
    pub fn rfxif(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        cfdrfists::Rfxif,
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
    pub fn tmieg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        cfdtmiec::TmiEg,
        cfdtmiec::TmiEg,
        Cfdtmiec_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            cfdtmiec::TmiEg,
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
        pub const _0_X_00: Self = Self::new(0);

        pub const _0_X_01: Self = Self::new(1);

        pub const _0_X_10: Self = Self::new(0);

        pub const _0_X_11: Self = Self::new(1);
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
    pub fn tsif0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdgtintsts::Tsif0,
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
            cfdgtintsts::Tsif0,
            Cfdgtintsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tai0(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdgtintsts::Tai0,
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
            cfdgtintsts::Tai0,
            Cfdgtintsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tqif0(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cfdgtintsts::Tqif0,
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
            cfdgtintsts::Tqif0,
            Cfdgtintsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cftif0(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        cfdgtintsts::Cftif0,
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
            cfdgtintsts::Cftif0,
            Cfdgtintsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn thif0(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        cfdgtintsts::Thif0,
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tai0_SPEC;
    pub type Tai0 = crate::EnumBitfieldStruct<u8, Tai0_SPEC>;
    impl Tai0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tqif0_SPEC;
    pub type Tqif0 = crate::EnumBitfieldStruct<u8, Tqif0_SPEC>;
    impl Tqif0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cftif0_SPEC;
    pub type Cftif0 = crate::EnumBitfieldStruct<u8, Cftif0_SPEC>;
    impl Cftif0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Thif0_SPEC;
    pub type Thif0 = crate::EnumBitfieldStruct<u8, Thif0_SPEC>;
    impl Thif0 {
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

pub type Cfdgaflignent = crate::RegValueT<Cfdgaflignent_SPEC>;

impl Cfdgaflignent {
    #[inline(always)]
    pub fn irn(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Cfdgaflignent_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Cfdgaflignent_SPEC,crate::common::RW>::from_register(self,0)
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
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, Cfdc0Dcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,Cfdc0Dcfg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dtseg2(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Cfdc0Dcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Cfdc0Dcfg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dsjw(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, u8, Cfdc0Dcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xf,1,0,u8,u8,Cfdc0Dcfg_SPEC,crate::common::RW>::from_register(self,0)
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
    pub fn tdco(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdc0Fdcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdc0Fdcfg_SPEC,crate::common::RW>::from_register(self,0)
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
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdc0Fdsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdc0Fdsts_SPEC,crate::common::R>::from_register(self,0)
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
    ) -> crate::common::RegisterField<0, 0x1fffff, 1, 0, u32, u32, Cfdc0Fdcrc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1fffff,1,0,u32,u32,Cfdc0Fdcrc_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn scnt(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, u8, Cfdc0Fdcrc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xf,1,0,u8,u8,Cfdc0Fdcrc_SPEC,crate::common::R>::from_register(self,0)
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
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Cfdgaflp0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Cfdgaflp0_SPEC,crate::common::RW>::from_register(self,0)
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
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdgaflp1::Gaflfdp0,
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
            cfdgaflp1::Gaflfdp0,
            Cfdgaflp1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gaflfdp1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdgaflp1::Gaflfdp1,
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
            cfdgaflp1::Gaflfdp1,
            Cfdgaflp1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gaflfdp8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cfdgaflp1::Gaflfdp8,
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
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gaflfdp1_SPEC;
    pub type Gaflfdp1 = crate::EnumBitfieldStruct<u8, Gaflfdp1_SPEC>;
    impl Gaflfdp1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gaflfdp8_SPEC;
    pub type Gaflfdp8 = crate::EnumBitfieldStruct<u8, Gaflfdp8_SPEC>;
    impl Gaflfdp8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
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
    pub fn rfdlc(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, u8, Cfdrfptr_SPEC, crate::common::R> {
        crate::common::RegisterField::<28,0xf,1,0,u8,u8,Cfdrfptr_SPEC,crate::common::R>::from_register(self,0)
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
    pub fn rfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfdf0_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfdf0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrfdf0_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrfdf0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrfdf0_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrfdf0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrfdf0_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrfdf0_SPEC,crate::common::R>::from_register(self,0)
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

pub type Cfdrfdf1 = crate::RegValueT<Cfdrfdf1_SPEC>;

impl Cfdrfdf1 {
    #[inline(always)]
    pub fn rfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfdf1_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfdf1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrfdf1_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrfdf1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrfdf1_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrfdf1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrfdf1_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrfdf1_SPEC,crate::common::R>::from_register(self,0)
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

pub type Cfdrfdf2 = crate::RegValueT<Cfdrfdf2_SPEC>;

impl Cfdrfdf2 {
    #[inline(always)]
    pub fn rfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfdf2_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfdf2_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrfdf2_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrfdf2_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrfdf2_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrfdf2_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrfdf2_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrfdf2_SPEC,crate::common::R>::from_register(self,0)
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

pub type Cfdrfdf3 = crate::RegValueT<Cfdrfdf3_SPEC>;

impl Cfdrfdf3 {
    #[inline(always)]
    pub fn rfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfdf3_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfdf3_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrfdf3_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrfdf3_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrfdf3_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrfdf3_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrfdf3_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrfdf3_SPEC,crate::common::R>::from_register(self,0)
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

pub type Cfdrfdf4 = crate::RegValueT<Cfdrfdf4_SPEC>;

impl Cfdrfdf4 {
    #[inline(always)]
    pub fn rfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfdf4_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfdf4_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrfdf4_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrfdf4_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrfdf4_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrfdf4_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrfdf4_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrfdf4_SPEC,crate::common::R>::from_register(self,0)
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

pub type Cfdrfdf5 = crate::RegValueT<Cfdrfdf5_SPEC>;

impl Cfdrfdf5 {
    #[inline(always)]
    pub fn rfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfdf5_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfdf5_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrfdf5_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrfdf5_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrfdf5_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrfdf5_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrfdf5_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrfdf5_SPEC,crate::common::R>::from_register(self,0)
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

pub type Cfdrfdf6 = crate::RegValueT<Cfdrfdf6_SPEC>;

impl Cfdrfdf6 {
    #[inline(always)]
    pub fn rfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfdf6_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfdf6_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrfdf6_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrfdf6_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrfdf6_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrfdf6_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrfdf6_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrfdf6_SPEC,crate::common::R>::from_register(self,0)
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

pub type Cfdrfdf7 = crate::RegValueT<Cfdrfdf7_SPEC>;

impl Cfdrfdf7 {
    #[inline(always)]
    pub fn rfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfdf7_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfdf7_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrfdf7_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrfdf7_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrfdf7_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrfdf7_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrfdf7_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrfdf7_SPEC,crate::common::R>::from_register(self,0)
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

pub type Cfdrfdf8 = crate::RegValueT<Cfdrfdf8_SPEC>;

impl Cfdrfdf8 {
    #[inline(always)]
    pub fn rfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfdf8_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfdf8_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrfdf8_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrfdf8_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrfdf8_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrfdf8_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrfdf8_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrfdf8_SPEC,crate::common::R>::from_register(self,0)
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

pub type Cfdrfdf9 = crate::RegValueT<Cfdrfdf9_SPEC>;

impl Cfdrfdf9 {
    #[inline(always)]
    pub fn rfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfdf9_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfdf9_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrfdf9_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrfdf9_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrfdf9_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrfdf9_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrfdf9_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrfdf9_SPEC,crate::common::R>::from_register(self,0)
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

pub type Cfdrfdf10 = crate::RegValueT<Cfdrfdf10_SPEC>;

impl Cfdrfdf10 {
    #[inline(always)]
    pub fn rfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfdf10_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfdf10_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrfdf10_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrfdf10_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrfdf10_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrfdf10_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrfdf10_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrfdf10_SPEC,crate::common::R>::from_register(self,0)
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

pub type Cfdrfdf11 = crate::RegValueT<Cfdrfdf11_SPEC>;

impl Cfdrfdf11 {
    #[inline(always)]
    pub fn rfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfdf11_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfdf11_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrfdf11_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrfdf11_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrfdf11_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrfdf11_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrfdf11_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrfdf11_SPEC,crate::common::R>::from_register(self,0)
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

pub type Cfdrfdf12 = crate::RegValueT<Cfdrfdf12_SPEC>;

impl Cfdrfdf12 {
    #[inline(always)]
    pub fn rfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfdf12_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfdf12_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrfdf12_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrfdf12_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrfdf12_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrfdf12_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrfdf12_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrfdf12_SPEC,crate::common::R>::from_register(self,0)
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

pub type Cfdrfdf13 = crate::RegValueT<Cfdrfdf13_SPEC>;

impl Cfdrfdf13 {
    #[inline(always)]
    pub fn rfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfdf13_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfdf13_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrfdf13_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrfdf13_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrfdf13_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrfdf13_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrfdf13_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrfdf13_SPEC,crate::common::R>::from_register(self,0)
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

pub type Cfdrfdf14 = crate::RegValueT<Cfdrfdf14_SPEC>;

impl Cfdrfdf14 {
    #[inline(always)]
    pub fn rfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfdf14_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfdf14_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrfdf14_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrfdf14_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrfdf14_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrfdf14_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrfdf14_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrfdf14_SPEC,crate::common::R>::from_register(self,0)
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

pub type Cfdrfdf15 = crate::RegValueT<Cfdrfdf15_SPEC>;

impl Cfdrfdf15 {
    #[inline(always)]
    pub fn rfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrfdf15_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrfdf15_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrfdf15_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrfdf15_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrfdf15_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrfdf15_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrfdf15_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrfdf15_SPEC,crate::common::R>::from_register(self,0)
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
    pub fn cfdlc(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, u8, Cfdcfptr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0xf,1,0,u8,u8,Cfdcfptr_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct Cfdcfdf_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf = crate::RegValueT<Cfdcfdf_SPEC>;

impl Cfdcfdf {
    #[inline(always)]
    pub fn cfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdcfdf_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdcfdf_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdf_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdf_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfdf_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfdf_SPEC,crate::common::RW>::from_register(self,0)
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
    pub fn tmdlc(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, u8, Cfdtmptr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0xf,1,0,u8,u8,Cfdtmptr_SPEC,crate::common::RW>::from_register(self,0)
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
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
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
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
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
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
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
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
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
pub struct Cfdtmdf4_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf4_SPEC {
    type DataType = u32;
}

pub type Cfdtmdf4 = crate::RegValueT<Cfdtmdf4_SPEC>;

impl Cfdtmdf4 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf4_SPEC,crate::common::RW>::from_register(self,0)
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

pub type Cfdtmdf5 = crate::RegValueT<Cfdtmdf5_SPEC>;

impl Cfdtmdf5 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf5_SPEC,crate::common::RW>::from_register(self,0)
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

pub type Cfdtmdf6 = crate::RegValueT<Cfdtmdf6_SPEC>;

impl Cfdtmdf6 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf6_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf6_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf6_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf6_SPEC,crate::common::RW>::from_register(self,0)
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

pub type Cfdtmdf7 = crate::RegValueT<Cfdtmdf7_SPEC>;

impl Cfdtmdf7 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf7_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf7_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf7_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf7_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf7_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf7_SPEC,crate::common::RW>::from_register(self,0)
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

pub type Cfdtmdf8 = crate::RegValueT<Cfdtmdf8_SPEC>;

impl Cfdtmdf8 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf8_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf8_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf8_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf8_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf8_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf8_SPEC,crate::common::RW>::from_register(self,0)
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

pub type Cfdtmdf9 = crate::RegValueT<Cfdtmdf9_SPEC>;

impl Cfdtmdf9 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf9_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf9_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf9_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf9_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf9_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf9_SPEC,crate::common::RW>::from_register(self,0)
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

pub type Cfdtmdf10 = crate::RegValueT<Cfdtmdf10_SPEC>;

impl Cfdtmdf10 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf10_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf10_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf10_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf10_SPEC,crate::common::RW>::from_register(self,0)
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

pub type Cfdtmdf11 = crate::RegValueT<Cfdtmdf11_SPEC>;

impl Cfdtmdf11 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf11_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf11_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf11_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf11_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf11_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf11_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf11_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf11_SPEC,crate::common::RW>::from_register(self,0)
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

pub type Cfdtmdf12 = crate::RegValueT<Cfdtmdf12_SPEC>;

impl Cfdtmdf12 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf12_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf12_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf12_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf12_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf12_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf12_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf12_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf12_SPEC,crate::common::RW>::from_register(self,0)
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

pub type Cfdtmdf13 = crate::RegValueT<Cfdtmdf13_SPEC>;

impl Cfdtmdf13 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf13_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf13_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf13_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf13_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf13_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf13_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf13_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf13_SPEC,crate::common::RW>::from_register(self,0)
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

pub type Cfdtmdf14 = crate::RegValueT<Cfdtmdf14_SPEC>;

impl Cfdtmdf14 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf14_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf14_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf14_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf14_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf14_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf14_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf14_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf14_SPEC,crate::common::RW>::from_register(self,0)
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

pub type Cfdtmdf15 = crate::RegValueT<Cfdtmdf15_SPEC>;

impl Cfdtmdf15 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf15_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf15_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf15_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf15_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf15_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf15_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf15_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf15_SPEC,crate::common::RW>::from_register(self,0)
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
    pub fn rmdlc(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, u8, Cfdrmptr_SPEC, crate::common::R> {
        crate::common::RegisterField::<28,0xf,1,0,u8,u8,Cfdrmptr_SPEC,crate::common::R>::from_register(self,0)
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
pub struct Cfdrmdf0_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf0_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf0 = crate::RegValueT<Cfdrmdf0_SPEC>;

impl Cfdrmdf0 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf0_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf0_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf0_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf0_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf0_SPEC,crate::common::R>::from_register(self,0)
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

pub type Cfdrmdf1 = crate::RegValueT<Cfdrmdf1_SPEC>;

impl Cfdrmdf1 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf1_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf1_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf1_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf1_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf1_SPEC,crate::common::R>::from_register(self,0)
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

pub type Cfdrmdf2 = crate::RegValueT<Cfdrmdf2_SPEC>;

impl Cfdrmdf2 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf2_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf2_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf2_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf2_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf2_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf2_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf2_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf2_SPEC,crate::common::R>::from_register(self,0)
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

pub type Cfdrmdf3 = crate::RegValueT<Cfdrmdf3_SPEC>;

impl Cfdrmdf3 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf3_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf3_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf3_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf3_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf3_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf3_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf3_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf3_SPEC,crate::common::R>::from_register(self,0)
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

pub type Cfdrmdf4 = crate::RegValueT<Cfdrmdf4_SPEC>;

impl Cfdrmdf4 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf4_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf4_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf4_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf4_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf4_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf4_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf4_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf4_SPEC,crate::common::R>::from_register(self,0)
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

pub type Cfdrmdf5 = crate::RegValueT<Cfdrmdf5_SPEC>;

impl Cfdrmdf5 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf5_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf5_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf5_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf5_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf5_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf5_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf5_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf5_SPEC,crate::common::R>::from_register(self,0)
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

pub type Cfdrmdf6 = crate::RegValueT<Cfdrmdf6_SPEC>;

impl Cfdrmdf6 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf6_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf6_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf6_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf6_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf6_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf6_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf6_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf6_SPEC,crate::common::R>::from_register(self,0)
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

pub type Cfdrmdf7 = crate::RegValueT<Cfdrmdf7_SPEC>;

impl Cfdrmdf7 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf7_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf7_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf7_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf7_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf7_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf7_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf7_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf7_SPEC,crate::common::R>::from_register(self,0)
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

pub type Cfdrmdf8 = crate::RegValueT<Cfdrmdf8_SPEC>;

impl Cfdrmdf8 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf8_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf8_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf8_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf8_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf8_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf8_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf8_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf8_SPEC,crate::common::R>::from_register(self,0)
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

pub type Cfdrmdf9 = crate::RegValueT<Cfdrmdf9_SPEC>;

impl Cfdrmdf9 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf9_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf9_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf9_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf9_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf9_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf9_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf9_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf9_SPEC,crate::common::R>::from_register(self,0)
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

pub type Cfdrmdf10 = crate::RegValueT<Cfdrmdf10_SPEC>;

impl Cfdrmdf10 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf10_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf10_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf10_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf10_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf10_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf10_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf10_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf10_SPEC,crate::common::R>::from_register(self,0)
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

pub type Cfdrmdf11 = crate::RegValueT<Cfdrmdf11_SPEC>;

impl Cfdrmdf11 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf11_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf11_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf11_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf11_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf11_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf11_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf11_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf11_SPEC,crate::common::R>::from_register(self,0)
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

pub type Cfdrmdf12 = crate::RegValueT<Cfdrmdf12_SPEC>;

impl Cfdrmdf12 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf12_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf12_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf12_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf12_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf12_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf12_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf12_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf12_SPEC,crate::common::R>::from_register(self,0)
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

pub type Cfdrmdf13 = crate::RegValueT<Cfdrmdf13_SPEC>;

impl Cfdrmdf13 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf13_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf13_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf13_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf13_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf13_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf13_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf13_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf13_SPEC,crate::common::R>::from_register(self,0)
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

pub type Cfdrmdf14 = crate::RegValueT<Cfdrmdf14_SPEC>;

impl Cfdrmdf14 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf14_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf14_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf14_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf14_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf14_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf14_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf14_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf14_SPEC,crate::common::R>::from_register(self,0)
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

pub type Cfdrmdf15 = crate::RegValueT<Cfdrmdf15_SPEC>;

impl Cfdrmdf15 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf15_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf15_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf15_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf15_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf15_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf15_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf15_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf15_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf15 {
    #[inline(always)]
    fn default() -> Cfdrmdf15 {
        <crate::RegValueT<Cfdrmdf15_SPEC> as RegisterValue<_>>::new(0)
    }
}
