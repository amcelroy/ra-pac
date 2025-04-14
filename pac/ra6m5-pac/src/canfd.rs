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
// Generated from SVD 1.30.00, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:20:54 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"CANFD"]
unsafe impl ::core::marker::Send for super::Canfd {}
unsafe impl ::core::marker::Sync for super::Canfd {}
impl super::Canfd {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn cfdcncfg(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcncfg_SPEC, crate::common::RW>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x0usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcctr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcctr_SPEC, crate::common::RW>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x4usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcsts(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcsts_SPEC, crate::common::RW>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x8usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcerfl(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcerfl_SPEC, crate::common::RW>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xcusize))
        }
    }

    #[inline(always)]
    pub const fn cfdgcfg(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdgcfg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdgcfg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdgctr(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdgctr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdgctr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(136usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdgsts(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdgsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cfdgsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdgerfl(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdgerfl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdgerfl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdgtsc(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdgtsc_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cfdgtsc_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdgaflectr(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdgaflectr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdgaflectr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdgaflcfg0(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdgaflcfg0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdgaflcfg0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(156usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdrmnb(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdrmnb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdrmnb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(172usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdrmnd0(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdrmnd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdrmnd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(176usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdrfcc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfcc_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xc0usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfsts(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfsts_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xe0usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfpctr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfpctr_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x100usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfcc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfcc_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x120usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfcce(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfcce_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x180usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfsts(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfsts_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1e0usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfpctr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfpctr_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x240usize))
        }
    }

    #[inline(always)]
    pub const fn cfdfests(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdfests_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cfdfests_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(672usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdffsts(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdffsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cfdffsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(676usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdfmsts(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdfmsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cfdfmsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(680usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdrfists(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdrfists_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdrfists_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(684usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdcfrists(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcfrists_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdcfrists_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(688usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdcftists(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcftists_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdcftists_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(692usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdcfofrists(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcfofrists_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cfdcfofrists_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(696usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdcfoftists(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcfoftists_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cfdcfoftists_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(700usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdcfmowsts(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcfmowsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cfdcfmowsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(704usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdfffsts(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdfffsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cfdfffsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(708usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdtmc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmc_SPEC, crate::common::RW>,
        8,
        0x1,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x330usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmsts(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmsts_SPEC, crate::common::RW>,
        8,
        0x1,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x830usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmtrsts(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmtrsts_SPEC, crate::common::R>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xcd0usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmtarsts(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmtarsts_SPEC, crate::common::R>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xd70usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmtcsts(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmtcsts_SPEC, crate::common::R>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xe10usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmtasts(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmtasts_SPEC, crate::common::R>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xeb0usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmiec(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmiec_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xf50usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtxqcc0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtxqcc0_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1000usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtxqsts0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtxqsts0_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1020usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtxqpctr0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtxqpctr0_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1040usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtxqcc1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtxqcc1_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1060usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtxqsts1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtxqsts1_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1080usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtxqpctr1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtxqpctr1_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x10a0usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtxqcc2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtxqcc2_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x10c0usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtxqsts2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtxqsts2_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x10e0usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtxqpctr2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtxqpctr2_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1100usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtxqcc3(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtxqcc3_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1120usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtxqsts3(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtxqsts3_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1140usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtxqpctr3(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtxqpctr3_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1160usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtxqests(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdtxqests_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cfdtxqests_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4480usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdtxqfists(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdtxqfists_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdtxqfists_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4484usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdtxqmsts(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdtxqmsts_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdtxqmsts_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4488usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdtxqists(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdtxqists_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdtxqists_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4496usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdtxqoftists(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdtxqoftists_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdtxqoftists_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4500usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdtxqofrists(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdtxqofrists_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdtxqofrists_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4504usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdtxqfsts(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdtxqfsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cfdtxqfsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4508usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdthlcc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdthlcc_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1200usize))
        }
    }

    #[inline(always)]
    pub const fn cfdthlsts(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdthlsts_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1220usize))
        }
    }

    #[inline(always)]
    pub const fn cfdthlpctr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdthlpctr_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1240usize))
        }
    }

    #[inline(always)]
    pub const fn cfdgtintsts0(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdgtintsts0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdgtintsts0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4864usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdgtstcfg(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdgtstcfg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdgtstcfg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4872usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdgtstctr(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdgtstctr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdgtstctr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4876usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdgfdcfg(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdgfdcfg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdgfdcfg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4884usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdglockk(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdglockk_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Cfdglockk_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(4892usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdcdtct(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcdtct_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdcdtct_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4912usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdcdtsts(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcdtsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cfdcdtsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4916usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdcdttct(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcdttct_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdcdttct_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4928usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdcdttsts(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdcdttsts_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdcdttsts_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4932usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdgrintsts(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdgrintsts_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1350usize))
        }
    }

    #[inline(always)]
    pub const fn cfdgrstc(
        &self,
    ) -> &'static crate::common::Reg<self::Cfdgrstc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfdgrstc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4992usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfdcdcfg(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcdcfg_SPEC, crate::common::RW>,
        2,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1400usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdcfg(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdcfg_SPEC, crate::common::RW>,
        2,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1404usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdctr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdctr_SPEC, crate::common::RW>,
        2,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1408usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdsts(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdsts_SPEC, crate::common::RW>,
        2,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x140cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdcrc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdcrc_SPEC, crate::common::RW>,
        2,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1410usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcblct(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcblct_SPEC, crate::common::RW>,
        2,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1418usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcblsts(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcblsts_SPEC, crate::common::RW>,
        2,
        0x20,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x141cusize))
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
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1800usize))
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
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1804usize))
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
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1808usize))
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
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x180cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmid_0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmid0_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2000usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmptr_0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmptr0_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2004usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmfdsts_0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmfdsts0_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2008usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf0__0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf00_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x200cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf1__0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf10_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2010usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf2__0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf20_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2014usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf3__0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf30_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2018usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf4__0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf40_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x201cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf5__0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf50_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2020usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf6__0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf60_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2024usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf7__0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf70_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2028usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf8__0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf80_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x202cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf9__0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf90_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2030usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf10__0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf100_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2034usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf11__0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf110_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2038usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf12__0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf120_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x203cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf13__0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf130_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2040usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf14__0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf140_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2044usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf15__0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf150_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2048usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmid_1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmid1_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2800usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmptr_1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmptr1_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2804usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmfdsts_1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmfdsts1_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2808usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf0__1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf01_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x280cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf1__1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf11_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2810usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf2__1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf21_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2814usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf3__1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf31_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2818usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf4__1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf41_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x281cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf5__1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf51_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2820usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf6__1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf61_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2824usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf7__1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf71_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2828usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf8__1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf81_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x282cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf9__1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf91_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2830usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf10__1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf101_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2834usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf11__1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf111_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2838usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf12__1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf121_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x283cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf13__1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf131_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2840usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf14__1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf141_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2844usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrmdf15__1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrmdf151_SPEC, crate::common::R>,
        16,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2848usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfid(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfid_SPEC, crate::common::R>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6000usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfptr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfptr_SPEC, crate::common::R>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6004usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrffdsts(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrffdsts_SPEC, crate::common::R>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6008usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfdf0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf0_SPEC, crate::common::R>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x600cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfdf1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf1_SPEC, crate::common::R>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6010usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfdf2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf2_SPEC, crate::common::R>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6014usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfdf3(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf3_SPEC, crate::common::R>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6018usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfdf4(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf4_SPEC, crate::common::R>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x601cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfdf5(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf5_SPEC, crate::common::R>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6020usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfdf6(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf6_SPEC, crate::common::R>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6024usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfdf7(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf7_SPEC, crate::common::R>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6028usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfdf8(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf8_SPEC, crate::common::R>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x602cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfdf9(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf9_SPEC, crate::common::R>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6030usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfdf10(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf10_SPEC, crate::common::R>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6034usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfdf11(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf11_SPEC, crate::common::R>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6038usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfdf12(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf12_SPEC, crate::common::R>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x603cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfdf13(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf13_SPEC, crate::common::R>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6040usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfdf14(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf14_SPEC, crate::common::R>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6044usize))
        }
    }

    #[inline(always)]
    pub const fn cfdrfdf15(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdrfdf15_SPEC, crate::common::R>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6048usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfid_0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfid0_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6400usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfptr_0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfptr0_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6404usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcffdcsts_0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcffdcsts0_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6408usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf0_0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdf00_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x640cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf1_0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdf10_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6410usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf2_0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdf20_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6414usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf3_0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdf30_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6418usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf4_0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdf40_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x641cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf5_0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdf50_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6420usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf6_0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdf60_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6424usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf7_0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdf70_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6428usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf8_0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdf80_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x642cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf9_0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdf90_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6430usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf10_0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdf100_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6434usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf11_0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdf110_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6438usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf12_0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdf120_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x643cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf13_0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdf130_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6440usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf14_0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdf140_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6444usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf15_0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdf150_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6448usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfid_1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfid1_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6580usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfptr_1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfptr1_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6584usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcffdcsts_1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcffdcsts1_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6588usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf0_1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdf01_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x658cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf1_1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdf11_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6590usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf2_1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdf21_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6594usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf3_1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdf31_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6598usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf4_1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdf41_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x659cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf5_1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdf51_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x65a0usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf6_1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdf61_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x65a4usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf7_1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdf71_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x65a8usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf8_1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdf81_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x65acusize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf9_1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdf91_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x65b0usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf10_1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdf101_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x65b4usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf11_1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdf111_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x65b8usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf12_1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdf121_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x65bcusize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf13_1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdf131_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x65c0usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf14_1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdf141_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x65c4usize))
        }
    }

    #[inline(always)]
    pub const fn cfdcfdf15_1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdcfdf151_SPEC, crate::common::RW>,
        3,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x65c8usize))
        }
    }

    #[inline(always)]
    pub const fn cfdthlacc0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdthlacc0_SPEC, crate::common::R>,
        2,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x8000usize))
        }
    }

    #[inline(always)]
    pub const fn cfdthlacc1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdthlacc1_SPEC, crate::common::RW>,
        2,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x8004usize))
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
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x8400usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmid_0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmid0_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x11000usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmptr_0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmptr0_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x11004usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmfdctr_0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmfdctr0_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x11008usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf0__0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf00_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1100cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf1__0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf10_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x11010usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf2__0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf20_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x11014usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf3__0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf30_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x11018usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf4__0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf40_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1101cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf5__0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf50_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x11020usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf6__0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf60_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x11024usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf7__0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf70_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x11028usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf8__0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf80_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1102cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf9__0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf90_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x11030usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf10__0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf100_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x11034usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf11__0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf110_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x11038usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf12__0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf120_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1103cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf13__0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf130_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x11040usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf14__0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf140_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x11044usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf15__0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf150_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x11048usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmid_1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmid1_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x13000usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmptr_1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmptr1_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x13004usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmfdctr_1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmfdctr1_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x13008usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf0__1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf01_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1300cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf1__1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf11_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x13010usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf2__1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf21_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x13014usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf3__1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf31_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x13018usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf4__1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf41_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1301cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf5__1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf51_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x13020usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf6__1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf61_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x13024usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf7__1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf71_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x13028usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf8__1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf81_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1302cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf9__1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf91_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x13030usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf10__1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf101_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x13034usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf11__1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf111_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x13038usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf12__1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf121_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1303cusize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf13__1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf131_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x13040usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf14__1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf141_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x13044usize))
        }
    }

    #[inline(always)]
    pub const fn cfdtmdf15__1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Cfdtmdf151_SPEC, crate::common::RW>,
        8,
        0x80,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x13048usize))
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcncfg_SPEC;
impl crate::sealed::RegSpec for Cfdcncfg_SPEC {
    type DataType = u32;
}

pub type Cfdcncfg = crate::RegValueT<Cfdcncfg_SPEC>;

impl Cfdcncfg {
    #[inline(always)]
    pub fn nbrp(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Cfdcncfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Cfdcncfg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn nsjw(
        self,
    ) -> crate::common::RegisterField<10, 0x7f, 1, 0, u8, u8, Cfdcncfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x7f,1,0,u8,u8,Cfdcncfg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ntseg1(
        self,
    ) -> crate::common::RegisterField<17, 0xff, 1, 0, u8, u8, Cfdcncfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0xff,1,0,u8,u8,Cfdcncfg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ntseg2(
        self,
    ) -> crate::common::RegisterField<25, 0x7f, 1, 0, u8, u8, Cfdcncfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x7f,1,0,u8,u8,Cfdcncfg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcncfg {
    #[inline(always)]
    fn default() -> Cfdcncfg {
        <crate::RegValueT<Cfdcncfg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcctr_SPEC;
impl crate::sealed::RegSpec for Cfdcctr_SPEC {
    type DataType = u32;
}

pub type Cfdcctr = crate::RegValueT<Cfdcctr_SPEC>;

impl Cfdcctr {
    #[inline(always)]
    pub fn chmdc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        cfdcctr::Chmdc,
        cfdcctr::Chmdc,
        Cfdcctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            cfdcctr::Chmdc,
            cfdcctr::Chmdc,
            Cfdcctr_SPEC,
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
        cfdcctr::Cslpr,
        cfdcctr::Cslpr,
        Cfdcctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdcctr::Cslpr,
            cfdcctr::Cslpr,
            Cfdcctr_SPEC,
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
        cfdcctr::Rtbo,
        cfdcctr::Rtbo,
        Cfdcctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            cfdcctr::Rtbo,
            cfdcctr::Rtbo,
            Cfdcctr_SPEC,
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
        cfdcctr::Beie,
        cfdcctr::Beie,
        Cfdcctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cfdcctr::Beie,
            cfdcctr::Beie,
            Cfdcctr_SPEC,
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
        cfdcctr::Ewie,
        cfdcctr::Ewie,
        Cfdcctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            cfdcctr::Ewie,
            cfdcctr::Ewie,
            Cfdcctr_SPEC,
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
        cfdcctr::Epie,
        cfdcctr::Epie,
        Cfdcctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            cfdcctr::Epie,
            cfdcctr::Epie,
            Cfdcctr_SPEC,
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
        cfdcctr::Boeie,
        cfdcctr::Boeie,
        Cfdcctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            cfdcctr::Boeie,
            cfdcctr::Boeie,
            Cfdcctr_SPEC,
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
        cfdcctr::Borie,
        cfdcctr::Borie,
        Cfdcctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            cfdcctr::Borie,
            cfdcctr::Borie,
            Cfdcctr_SPEC,
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
        cfdcctr::Olie,
        cfdcctr::Olie,
        Cfdcctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            cfdcctr::Olie,
            cfdcctr::Olie,
            Cfdcctr_SPEC,
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
        cfdcctr::Blie,
        cfdcctr::Blie,
        Cfdcctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            cfdcctr::Blie,
            cfdcctr::Blie,
            Cfdcctr_SPEC,
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
        cfdcctr::Alie,
        cfdcctr::Alie,
        Cfdcctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            cfdcctr::Alie,
            cfdcctr::Alie,
            Cfdcctr_SPEC,
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
        cfdcctr::Taie,
        cfdcctr::Taie,
        Cfdcctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            cfdcctr::Taie,
            cfdcctr::Taie,
            Cfdcctr_SPEC,
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
        cfdcctr::Eocoie,
        cfdcctr::Eocoie,
        Cfdcctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            cfdcctr::Eocoie,
            cfdcctr::Eocoie,
            Cfdcctr_SPEC,
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
        cfdcctr::Socoie,
        cfdcctr::Socoie,
        Cfdcctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            cfdcctr::Socoie,
            cfdcctr::Socoie,
            Cfdcctr_SPEC,
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
        cfdcctr::Tdcvfie,
        cfdcctr::Tdcvfie,
        Cfdcctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            cfdcctr::Tdcvfie,
            cfdcctr::Tdcvfie,
            Cfdcctr_SPEC,
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
        cfdcctr::Bom,
        cfdcctr::Bom,
        Cfdcctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x3,
            1,
            0,
            cfdcctr::Bom,
            cfdcctr::Bom,
            Cfdcctr_SPEC,
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
        cfdcctr::Errd,
        cfdcctr::Errd,
        Cfdcctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            cfdcctr::Errd,
            cfdcctr::Errd,
            Cfdcctr_SPEC,
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
        cfdcctr::Ctme,
        cfdcctr::Ctme,
        Cfdcctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            cfdcctr::Ctme,
            cfdcctr::Ctme,
            Cfdcctr_SPEC,
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
        cfdcctr::Ctms,
        cfdcctr::Ctms,
        Cfdcctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x3,
            1,
            0,
            cfdcctr::Ctms,
            cfdcctr::Ctms,
            Cfdcctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn crct(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        cfdcctr::Crct,
        cfdcctr::Crct,
        Cfdcctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            cfdcctr::Crct,
            cfdcctr::Crct,
            Cfdcctr_SPEC,
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
        cfdcctr::Rom,
        cfdcctr::Rom,
        Cfdcctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            cfdcctr::Rom,
            cfdcctr::Rom,
            Cfdcctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcctr {
    #[inline(always)]
    fn default() -> Cfdcctr {
        <crate::RegValueT<Cfdcctr_SPEC> as RegisterValue<_>>::new(5)
    }
}
pub mod cfdcctr {

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
    pub struct Crct_SPEC;
    pub type Crct = crate::EnumBitfieldStruct<u8, Crct_SPEC>;
    impl Crct {
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
pub struct Cfdcsts_SPEC;
impl crate::sealed::RegSpec for Cfdcsts_SPEC {
    type DataType = u32;
}

pub type Cfdcsts = crate::RegValueT<Cfdcsts_SPEC>;

impl Cfdcsts {
    #[inline(always)]
    pub fn crststs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdcsts::Crststs,
        cfdcsts::Crststs,
        Cfdcsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdcsts::Crststs,
            cfdcsts::Crststs,
            Cfdcsts_SPEC,
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
        cfdcsts::Chltsts,
        cfdcsts::Chltsts,
        Cfdcsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdcsts::Chltsts,
            cfdcsts::Chltsts,
            Cfdcsts_SPEC,
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
        cfdcsts::Cslpsts,
        cfdcsts::Cslpsts,
        Cfdcsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdcsts::Cslpsts,
            cfdcsts::Cslpsts,
            Cfdcsts_SPEC,
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
        cfdcsts::Epsts,
        cfdcsts::Epsts,
        Cfdcsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            cfdcsts::Epsts,
            cfdcsts::Epsts,
            Cfdcsts_SPEC,
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
        cfdcsts::Bosts,
        cfdcsts::Bosts,
        Cfdcsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            cfdcsts::Bosts,
            cfdcsts::Bosts,
            Cfdcsts_SPEC,
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
        cfdcsts::Trmsts,
        cfdcsts::Trmsts,
        Cfdcsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            cfdcsts::Trmsts,
            cfdcsts::Trmsts,
            Cfdcsts_SPEC,
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
        cfdcsts::Recsts,
        cfdcsts::Recsts,
        Cfdcsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            cfdcsts::Recsts,
            cfdcsts::Recsts,
            Cfdcsts_SPEC,
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
        cfdcsts::Comsts,
        cfdcsts::Comsts,
        Cfdcsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            cfdcsts::Comsts,
            cfdcsts::Comsts,
            Cfdcsts_SPEC,
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
        cfdcsts::Esif,
        cfdcsts::Esif,
        Cfdcsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cfdcsts::Esif,
            cfdcsts::Esif,
            Cfdcsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rec(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcsts_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tec(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcsts_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcsts {
    #[inline(always)]
    fn default() -> Cfdcsts {
        <crate::RegValueT<Cfdcsts_SPEC> as RegisterValue<_>>::new(5)
    }
}
pub mod cfdcsts {

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
pub struct Cfdcerfl_SPEC;
impl crate::sealed::RegSpec for Cfdcerfl_SPEC {
    type DataType = u32;
}

pub type Cfdcerfl = crate::RegValueT<Cfdcerfl_SPEC>;

impl Cfdcerfl {
    #[inline(always)]
    pub fn bef(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdcerfl::Bef,
        cfdcerfl::Bef,
        Cfdcerfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdcerfl::Bef,
            cfdcerfl::Bef,
            Cfdcerfl_SPEC,
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
        cfdcerfl::Ewf,
        cfdcerfl::Ewf,
        Cfdcerfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdcerfl::Ewf,
            cfdcerfl::Ewf,
            Cfdcerfl_SPEC,
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
        cfdcerfl::Epf,
        cfdcerfl::Epf,
        Cfdcerfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdcerfl::Epf,
            cfdcerfl::Epf,
            Cfdcerfl_SPEC,
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
        cfdcerfl::Boef,
        cfdcerfl::Boef,
        Cfdcerfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            cfdcerfl::Boef,
            cfdcerfl::Boef,
            Cfdcerfl_SPEC,
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
        cfdcerfl::Borf,
        cfdcerfl::Borf,
        Cfdcerfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            cfdcerfl::Borf,
            cfdcerfl::Borf,
            Cfdcerfl_SPEC,
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
        cfdcerfl::Ovlf,
        cfdcerfl::Ovlf,
        Cfdcerfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            cfdcerfl::Ovlf,
            cfdcerfl::Ovlf,
            Cfdcerfl_SPEC,
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
        cfdcerfl::Blf,
        cfdcerfl::Blf,
        Cfdcerfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            cfdcerfl::Blf,
            cfdcerfl::Blf,
            Cfdcerfl_SPEC,
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
        cfdcerfl::Alf,
        cfdcerfl::Alf,
        Cfdcerfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            cfdcerfl::Alf,
            cfdcerfl::Alf,
            Cfdcerfl_SPEC,
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
        cfdcerfl::Serr,
        cfdcerfl::Serr,
        Cfdcerfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cfdcerfl::Serr,
            cfdcerfl::Serr,
            Cfdcerfl_SPEC,
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
        cfdcerfl::Ferr,
        cfdcerfl::Ferr,
        Cfdcerfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            cfdcerfl::Ferr,
            cfdcerfl::Ferr,
            Cfdcerfl_SPEC,
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
        cfdcerfl::Aerr,
        cfdcerfl::Aerr,
        Cfdcerfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            cfdcerfl::Aerr,
            cfdcerfl::Aerr,
            Cfdcerfl_SPEC,
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
        cfdcerfl::Cerr,
        cfdcerfl::Cerr,
        Cfdcerfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            cfdcerfl::Cerr,
            cfdcerfl::Cerr,
            Cfdcerfl_SPEC,
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
        cfdcerfl::B1Err,
        cfdcerfl::B1Err,
        Cfdcerfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            cfdcerfl::B1Err,
            cfdcerfl::B1Err,
            Cfdcerfl_SPEC,
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
        cfdcerfl::B0Err,
        cfdcerfl::B0Err,
        Cfdcerfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            cfdcerfl::B0Err,
            cfdcerfl::B0Err,
            Cfdcerfl_SPEC,
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
        cfdcerfl::Aderr,
        cfdcerfl::Aderr,
        Cfdcerfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            cfdcerfl::Aderr,
            cfdcerfl::Aderr,
            Cfdcerfl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn crcreg(
        self,
    ) -> crate::common::RegisterField<16, 0x7fff, 1, 0, u16, u16, Cfdcerfl_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x7fff,1,0,u16,u16,Cfdcerfl_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcerfl {
    #[inline(always)]
    fn default() -> Cfdcerfl {
        <crate::RegValueT<Cfdcerfl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcerfl {

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
    pub fn tsbtcs(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x7,
        1,
        0,
        cfdgcfg::Tsbtcs,
        cfdgcfg::Tsbtcs,
        Cfdgcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x7,
            1,
            0,
            cfdgcfg::Tsbtcs,
            cfdgcfg::Tsbtcs,
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsbtcs_SPEC;
    pub type Tsbtcs = crate::EnumBitfieldStruct<u8, Tsbtcs_SPEC>;
    impl Tsbtcs {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const OTHERS: Self = Self::new(0);
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
    pub fn qmeie(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        cfdgctr::Qmeie,
        cfdgctr::Qmeie,
        Cfdgctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            cfdgctr::Qmeie,
            cfdgctr::Qmeie,
            Cfdgctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn moweie(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        cfdgctr::Moweie,
        cfdgctr::Moweie,
        Cfdgctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            cfdgctr::Moweie,
            cfdgctr::Moweie,
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
    pub struct Qmeie_SPEC;
    pub type Qmeie = crate::EnumBitfieldStruct<u8, Qmeie_SPEC>;
    impl Qmeie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Moweie_SPEC;
    pub type Moweie = crate::EnumBitfieldStruct<u8, Moweie_SPEC>;
    impl Moweie {
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
    pub fn qmes(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        cfdgerfl::Qmes,
        cfdgerfl::Qmes,
        Cfdgerfl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            cfdgerfl::Qmes,
            cfdgerfl::Qmes,
            Cfdgerfl_SPEC,
            crate::common::R,
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

    #[inline(always)]
    pub fn eef1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        cfdgerfl::Eef1,
        cfdgerfl::Eef1,
        Cfdgerfl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            cfdgerfl::Eef1,
            cfdgerfl::Eef1,
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
    pub struct Qmes_SPEC;
    pub type Qmes = crate::EnumBitfieldStruct<u8, Qmes_SPEC>;
    impl Qmes {
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eef1_SPEC;
    pub type Eef1 = crate::EnumBitfieldStruct<u8, Eef1_SPEC>;
    impl Eef1 {
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
    pub fn aflpn(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Cfdgaflectr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Cfdgaflectr_SPEC,crate::common::RW>::from_register(self,0)
    }

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
pub struct Cfdgaflcfg0_SPEC;
impl crate::sealed::RegSpec for Cfdgaflcfg0_SPEC {
    type DataType = u32;
}

pub type Cfdgaflcfg0 = crate::RegValueT<Cfdgaflcfg0_SPEC>;

impl Cfdgaflcfg0 {
    #[inline(always)]
    pub fn rnc1(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Cfdgaflcfg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Cfdgaflcfg0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rnc0(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Cfdgaflcfg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1ff,1,0,u16,u16,Cfdgaflcfg0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdgaflcfg0 {
    #[inline(always)]
    fn default() -> Cfdgaflcfg0 {
        <crate::RegValueT<Cfdgaflcfg0_SPEC> as RegisterValue<_>>::new(0)
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
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmnb_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmnb_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct Cfdrmnd0_SPEC;
impl crate::sealed::RegSpec for Cfdrmnd0_SPEC {
    type DataType = u32;
}

pub type Cfdrmnd0 = crate::RegValueT<Cfdrmnd0_SPEC>;

impl Cfdrmnd0 {
    #[inline(always)]
    pub fn rmns(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        cfdrmnd0::Rmns,
        cfdrmnd0::Rmns,
        Cfdrmnd0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            cfdrmnd0::Rmns,
            cfdrmnd0::Rmns,
            Cfdrmnd0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmnd0 {
    #[inline(always)]
    fn default() -> Cfdrmnd0 {
        <crate::RegValueT<Cfdrmnd0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmnd0 {

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
    pub fn rffie(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        cfdrfcc::Rffie,
        cfdrfcc::Rffie,
        Cfdrfcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            cfdrfcc::Rffie,
            cfdrfcc::Rffie,
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rffie_SPEC;
    pub type Rffie = crate::EnumBitfieldStruct<u8, Rffie_SPEC>;
    impl Rffie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
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
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrfsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrfsts_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rffif(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        cfdrfsts::Rffif,
        cfdrfsts::Rffif,
        Cfdrfsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            cfdrfsts::Rffif,
            cfdrfsts::Rffif,
            Cfdrfsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rffif_SPEC;
    pub type Rffif = crate::EnumBitfieldStruct<u8, Rffif_SPEC>;
    impl Rffif {
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
        0x3,
        1,
        0,
        cfdcfcc::Cfm,
        cfdcfcc::Cfm,
        Cfdcfcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
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
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Cfdcfcc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Cfdcfcc_SPEC,crate::common::RW>::from_register(self,0)
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
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
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
pub struct Cfdcfcce_SPEC;
impl crate::sealed::RegSpec for Cfdcfcce_SPEC {
    type DataType = u32;
}

pub type Cfdcfcce = crate::RegValueT<Cfdcfcce_SPEC>;

impl Cfdcfcce {
    #[inline(always)]
    pub fn cffie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdcfcce::Cffie,
        cfdcfcce::Cffie,
        Cfdcfcce_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdcfcce::Cffie,
            cfdcfcce::Cffie,
            Cfdcfcce_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfofrxie(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdcfcce::Cfofrxie,
        cfdcfcce::Cfofrxie,
        Cfdcfcce_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdcfcce::Cfofrxie,
            cfdcfcce::Cfofrxie,
            Cfdcfcce_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfoftxie(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cfdcfcce::Cfoftxie,
        cfdcfcce::Cfoftxie,
        Cfdcfcce_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdcfcce::Cfoftxie,
            cfdcfcce::Cfoftxie,
            Cfdcfcce_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfmowm(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cfdcfcce::Cfmowm,
        cfdcfcce::Cfmowm,
        Cfdcfcce_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cfdcfcce::Cfmowm,
            cfdcfcce::Cfmowm,
            Cfdcfcce_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfbme(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        cfdcfcce::Cfbme,
        cfdcfcce::Cfbme,
        Cfdcfcce_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            cfdcfcce::Cfbme,
            cfdcfcce::Cfbme,
            Cfdcfcce_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcfcce {
    #[inline(always)]
    fn default() -> Cfdcfcce {
        <crate::RegValueT<Cfdcfcce_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcfcce {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cffie_SPEC;
    pub type Cffie = crate::EnumBitfieldStruct<u8, Cffie_SPEC>;
    impl Cffie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfofrxie_SPEC;
    pub type Cfofrxie = crate::EnumBitfieldStruct<u8, Cfofrxie_SPEC>;
    impl Cfofrxie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfoftxie_SPEC;
    pub type Cfoftxie = crate::EnumBitfieldStruct<u8, Cfoftxie_SPEC>;
    impl Cfoftxie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfmowm_SPEC;
    pub type Cfmowm = crate::EnumBitfieldStruct<u8, Cfmowm_SPEC>;
    impl Cfmowm {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfbme_SPEC;
    pub type Cfbme = crate::EnumBitfieldStruct<u8, Cfbme_SPEC>;
    impl Cfbme {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
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
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdcfsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdcfsts_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cffif(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        cfdcfsts::Cffif,
        cfdcfsts::Cffif,
        Cfdcfsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            cfdcfsts::Cffif,
            cfdcfsts::Cffif,
            Cfdcfsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfofrxif(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Cfdcfsts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,Cfdcfsts_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfoftxif(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Cfdcfsts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,Cfdcfsts_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfmow(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        cfdcfsts::Cfmow,
        cfdcfsts::Cfmow,
        Cfdcfsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            cfdcfsts::Cfmow,
            cfdcfsts::Cfmow,
            Cfdcfsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cffif_SPEC;
    pub type Cffif = crate::EnumBitfieldStruct<u8, Cffif_SPEC>;
    impl Cffif {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfmow_SPEC;
    pub type Cfmow = crate::EnumBitfieldStruct<u8, Cfmow_SPEC>;
    impl Cfmow {
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
        0xff,
        1,
        0,
        cfdfests::Rfxemp,
        cfdfests::Rfxemp,
        Cfdfests_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            cfdfests::Rfxemp,
            cfdfests::Rfxemp,
            Cfdfests_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfxemp(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3f,
        1,
        0,
        cfdfests::Cfxemp,
        cfdfests::Cfxemp,
        Cfdfests_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x3f,
            1,
            0,
            cfdfests::Cfxemp,
            cfdfests::Cfxemp,
            Cfdfests_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdfests {
    #[inline(always)]
    fn default() -> Cfdfests {
        <crate::RegValueT<Cfdfests_SPEC> as RegisterValue<_>>::new(16383)
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
    pub struct Cfxemp_SPEC;
    pub type Cfxemp = crate::EnumBitfieldStruct<u8, Cfxemp_SPEC>;
    impl Cfxemp {
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
        0xff,
        1,
        0,
        cfdffsts::Rfxfll,
        cfdffsts::Rfxfll,
        Cfdffsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            cfdffsts::Rfxfll,
            cfdffsts::Rfxfll,
            Cfdffsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfxfll(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3f,
        1,
        0,
        cfdffsts::Cfxfll,
        cfdffsts::Cfxfll,
        Cfdffsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x3f,
            1,
            0,
            cfdffsts::Cfxfll,
            cfdffsts::Cfxfll,
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
    pub struct Cfxfll_SPEC;
    pub type Cfxfll = crate::EnumBitfieldStruct<u8, Cfxfll_SPEC>;
    impl Cfxfll {
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
        0xff,
        1,
        0,
        cfdfmsts::Rfxmlt,
        cfdfmsts::Rfxmlt,
        Cfdfmsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            cfdfmsts::Rfxmlt,
            cfdfmsts::Rfxmlt,
            Cfdfmsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfxmlt(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3f,
        1,
        0,
        cfdfmsts::Cfxmlt,
        cfdfmsts::Cfxmlt,
        Cfdfmsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x3f,
            1,
            0,
            cfdfmsts::Cfxmlt,
            cfdfmsts::Cfxmlt,
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
    pub struct Cfxmlt_SPEC;
    pub type Cfxmlt = crate::EnumBitfieldStruct<u8, Cfxmlt_SPEC>;
    impl Cfxmlt {
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
        0xff,
        1,
        0,
        cfdrfists::Rfxif,
        cfdrfists::Rfxif,
        Cfdrfists_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            cfdrfists::Rfxif,
            cfdrfists::Rfxif,
            Cfdrfists_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfxffll(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        cfdrfists::Rfxffll,
        cfdrfists::Rfxffll,
        Cfdrfists_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            cfdrfists::Rfxffll,
            cfdrfists::Rfxffll,
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfxffll_SPEC;
    pub type Rfxffll = crate::EnumBitfieldStruct<u8, Rfxffll_SPEC>;
    impl Rfxffll {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfrists_SPEC;
impl crate::sealed::RegSpec for Cfdcfrists_SPEC {
    type DataType = u32;
}

pub type Cfdcfrists = crate::RegValueT<Cfdcfrists_SPEC>;

impl Cfdcfrists {
    #[inline(always)]
    pub fn cfxrxif(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        cfdcfrists::Cfxrxif,
        cfdcfrists::Cfxrxif,
        Cfdcfrists_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            cfdcfrists::Cfxrxif,
            cfdcfrists::Cfxrxif,
            Cfdcfrists_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcfrists {
    #[inline(always)]
    fn default() -> Cfdcfrists {
        <crate::RegValueT<Cfdcfrists_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcfrists {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfxrxif_SPEC;
    pub type Cfxrxif = crate::EnumBitfieldStruct<u8, Cfxrxif_SPEC>;
    impl Cfxrxif {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcftists_SPEC;
impl crate::sealed::RegSpec for Cfdcftists_SPEC {
    type DataType = u32;
}

pub type Cfdcftists = crate::RegValueT<Cfdcftists_SPEC>;

impl Cfdcftists {
    #[inline(always)]
    pub fn cfxtxif(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        cfdcftists::Cfxtxif,
        cfdcftists::Cfxtxif,
        Cfdcftists_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            cfdcftists::Cfxtxif,
            cfdcftists::Cfxtxif,
            Cfdcftists_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcftists {
    #[inline(always)]
    fn default() -> Cfdcftists {
        <crate::RegValueT<Cfdcftists_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcftists {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfxtxif_SPEC;
    pub type Cfxtxif = crate::EnumBitfieldStruct<u8, Cfxtxif_SPEC>;
    impl Cfxtxif {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfofrists_SPEC;
impl crate::sealed::RegSpec for Cfdcfofrists_SPEC {
    type DataType = u32;
}

pub type Cfdcfofrists = crate::RegValueT<Cfdcfofrists_SPEC>;

impl Cfdcfofrists {
    #[inline(always)]
    pub fn cfxofrxif(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        cfdcfofrists::Cfxofrxif,
        cfdcfofrists::Cfxofrxif,
        Cfdcfofrists_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            cfdcfofrists::Cfxofrxif,
            cfdcfofrists::Cfxofrxif,
            Cfdcfofrists_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcfofrists {
    #[inline(always)]
    fn default() -> Cfdcfofrists {
        <crate::RegValueT<Cfdcfofrists_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcfofrists {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfxofrxif_SPEC;
    pub type Cfxofrxif = crate::EnumBitfieldStruct<u8, Cfxofrxif_SPEC>;
    impl Cfxofrxif {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfoftists_SPEC;
impl crate::sealed::RegSpec for Cfdcfoftists_SPEC {
    type DataType = u32;
}

pub type Cfdcfoftists = crate::RegValueT<Cfdcfoftists_SPEC>;

impl Cfdcfoftists {
    #[inline(always)]
    pub fn cfxoftxif(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        cfdcfoftists::Cfxoftxif,
        cfdcfoftists::Cfxoftxif,
        Cfdcfoftists_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            cfdcfoftists::Cfxoftxif,
            cfdcfoftists::Cfxoftxif,
            Cfdcfoftists_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcfoftists {
    #[inline(always)]
    fn default() -> Cfdcfoftists {
        <crate::RegValueT<Cfdcfoftists_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcfoftists {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfxoftxif_SPEC;
    pub type Cfxoftxif = crate::EnumBitfieldStruct<u8, Cfxoftxif_SPEC>;
    impl Cfxoftxif {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfmowsts_SPEC;
impl crate::sealed::RegSpec for Cfdcfmowsts_SPEC {
    type DataType = u32;
}

pub type Cfdcfmowsts = crate::RegValueT<Cfdcfmowsts_SPEC>;

impl Cfdcfmowsts {
    #[inline(always)]
    pub fn cfxmow(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        cfdcfmowsts::Cfxmow,
        cfdcfmowsts::Cfxmow,
        Cfdcfmowsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            cfdcfmowsts::Cfxmow,
            cfdcfmowsts::Cfxmow,
            Cfdcfmowsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcfmowsts {
    #[inline(always)]
    fn default() -> Cfdcfmowsts {
        <crate::RegValueT<Cfdcfmowsts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcfmowsts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfxmow_SPEC;
    pub type Cfxmow = crate::EnumBitfieldStruct<u8, Cfxmow_SPEC>;
    impl Cfxmow {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdfffsts_SPEC;
impl crate::sealed::RegSpec for Cfdfffsts_SPEC {
    type DataType = u32;
}

pub type Cfdfffsts = crate::RegValueT<Cfdfffsts_SPEC>;

impl Cfdfffsts {
    #[inline(always)]
    pub fn rfxffll(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        cfdfffsts::Rfxffll,
        cfdfffsts::Rfxffll,
        Cfdfffsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            cfdfffsts::Rfxffll,
            cfdfffsts::Rfxffll,
            Cfdfffsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfxffll(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3f,
        1,
        0,
        cfdfffsts::Cfxffll,
        cfdfffsts::Cfxffll,
        Cfdfffsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x3f,
            1,
            0,
            cfdfffsts::Cfxffll,
            cfdfffsts::Cfxffll,
            Cfdfffsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdfffsts {
    #[inline(always)]
    fn default() -> Cfdfffsts {
        <crate::RegValueT<Cfdfffsts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdfffsts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfxffll_SPEC;
    pub type Rfxffll = crate::EnumBitfieldStruct<u8, Rfxffll_SPEC>;
    impl Rfxffll {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfxffll_SPEC;
    pub type Cfxffll = crate::EnumBitfieldStruct<u8, Cfxffll_SPEC>;
    impl Cfxffll {
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
        0xff,
        1,
        0,
        cfdtmtrsts::Cfdtmtrsts,
        cfdtmtrsts::Cfdtmtrsts,
        Cfdtmtrsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
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
        0xff,
        1,
        0,
        cfdtmtarsts::Cfdtmtarsts,
        cfdtmtarsts::Cfdtmtarsts,
        Cfdtmtarsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
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
        0xff,
        1,
        0,
        cfdtmtcsts::Cfdtmtcsts,
        cfdtmtcsts::Cfdtmtcsts,
        Cfdtmtcsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
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
        0xff,
        1,
        0,
        cfdtmtasts::Cfdtmtasts,
        cfdtmtasts::Cfdtmtasts,
        Cfdtmtasts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
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
    pub fn tmie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        cfdtmiec::Tmie,
        cfdtmiec::Tmie,
        Cfdtmiec_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            cfdtmiec::Tmie,
            cfdtmiec::Tmie,
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
    pub struct Tmie_SPEC;
    pub type Tmie = crate::EnumBitfieldStruct<u8, Tmie_SPEC>;
    impl Tmie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtxqcc0_SPEC;
impl crate::sealed::RegSpec for Cfdtxqcc0_SPEC {
    type DataType = u32;
}

pub type Cfdtxqcc0 = crate::RegValueT<Cfdtxqcc0_SPEC>;

impl Cfdtxqcc0 {
    #[inline(always)]
    pub fn txqe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdtxqcc0::Txqe,
        cfdtxqcc0::Txqe,
        Cfdtxqcc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdtxqcc0::Txqe,
            cfdtxqcc0::Txqe,
            Cfdtxqcc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txqgwe(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdtxqcc0::Txqgwe,
        cfdtxqcc0::Txqgwe,
        Cfdtxqcc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdtxqcc0::Txqgwe,
            cfdtxqcc0::Txqgwe,
            Cfdtxqcc0_SPEC,
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
        cfdtxqcc0::Txqtxie,
        cfdtxqcc0::Txqtxie,
        Cfdtxqcc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            cfdtxqcc0::Txqtxie,
            cfdtxqcc0::Txqtxie,
            Cfdtxqcc0_SPEC,
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
        cfdtxqcc0::Txqim,
        cfdtxqcc0::Txqim,
        Cfdtxqcc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            cfdtxqcc0::Txqim,
            cfdtxqcc0::Txqim,
            Cfdtxqcc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txqdc(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, Cfdtxqcc0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,Cfdtxqcc0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn txqfie(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        cfdtxqcc0::Txqfie,
        cfdtxqcc0::Txqfie,
        Cfdtxqcc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            cfdtxqcc0::Txqfie,
            cfdtxqcc0::Txqfie,
            Cfdtxqcc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txqofrxie(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        cfdtxqcc0::Txqofrxie,
        cfdtxqcc0::Txqofrxie,
        Cfdtxqcc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            cfdtxqcc0::Txqofrxie,
            cfdtxqcc0::Txqofrxie,
            Cfdtxqcc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txqoftxie(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        cfdtxqcc0::Txqoftxie,
        cfdtxqcc0::Txqoftxie,
        Cfdtxqcc0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            cfdtxqcc0::Txqoftxie,
            cfdtxqcc0::Txqoftxie,
            Cfdtxqcc0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdtxqcc0 {
    #[inline(always)]
    fn default() -> Cfdtxqcc0 {
        <crate::RegValueT<Cfdtxqcc0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdtxqcc0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txqe_SPEC;
    pub type Txqe = crate::EnumBitfieldStruct<u8, Txqe_SPEC>;
    impl Txqe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txqgwe_SPEC;
    pub type Txqgwe = crate::EnumBitfieldStruct<u8, Txqgwe_SPEC>;
    impl Txqgwe {
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
    pub struct Txqfie_SPEC;
    pub type Txqfie = crate::EnumBitfieldStruct<u8, Txqfie_SPEC>;
    impl Txqfie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txqofrxie_SPEC;
    pub type Txqofrxie = crate::EnumBitfieldStruct<u8, Txqofrxie_SPEC>;
    impl Txqofrxie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txqoftxie_SPEC;
    pub type Txqoftxie = crate::EnumBitfieldStruct<u8, Txqoftxie_SPEC>;
    impl Txqoftxie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtxqsts0_SPEC;
impl crate::sealed::RegSpec for Cfdtxqsts0_SPEC {
    type DataType = u32;
}

pub type Cfdtxqsts0 = crate::RegValueT<Cfdtxqsts0_SPEC>;

impl Cfdtxqsts0 {
    #[inline(always)]
    pub fn txqemp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdtxqsts0::Txqemp,
        cfdtxqsts0::Txqemp,
        Cfdtxqsts0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdtxqsts0::Txqemp,
            cfdtxqsts0::Txqemp,
            Cfdtxqsts0_SPEC,
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
        cfdtxqsts0::Txqfll,
        cfdtxqsts0::Txqfll,
        Cfdtxqsts0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdtxqsts0::Txqfll,
            cfdtxqsts0::Txqfll,
            Cfdtxqsts0_SPEC,
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
        cfdtxqsts0::Txqtxif,
        cfdtxqsts0::Txqtxif,
        Cfdtxqsts0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdtxqsts0::Txqtxif,
            cfdtxqsts0::Txqtxif,
            Cfdtxqsts0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txqmc(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, Cfdtxqsts0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x3f,1,0,u8,u8,Cfdtxqsts0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn txqfif(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Cfdtxqsts0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,Cfdtxqsts0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn txqofrxif(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Cfdtxqsts0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,Cfdtxqsts0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn txqoftxif(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Cfdtxqsts0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,Cfdtxqsts0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn txqmlt(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        cfdtxqsts0::Txqmlt,
        cfdtxqsts0::Txqmlt,
        Cfdtxqsts0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            cfdtxqsts0::Txqmlt,
            cfdtxqsts0::Txqmlt,
            Cfdtxqsts0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdtxqsts0 {
    #[inline(always)]
    fn default() -> Cfdtxqsts0 {
        <crate::RegValueT<Cfdtxqsts0_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod cfdtxqsts0 {

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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txqmlt_SPEC;
    pub type Txqmlt = crate::EnumBitfieldStruct<u8, Txqmlt_SPEC>;
    impl Txqmlt {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtxqpctr0_SPEC;
impl crate::sealed::RegSpec for Cfdtxqpctr0_SPEC {
    type DataType = u32;
}

pub type Cfdtxqpctr0 = crate::RegValueT<Cfdtxqpctr0_SPEC>;

impl Cfdtxqpctr0 {
    #[inline(always)]
    pub fn txqpc(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtxqpctr0_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtxqpctr0_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtxqpctr0 {
    #[inline(always)]
    fn default() -> Cfdtxqpctr0 {
        <crate::RegValueT<Cfdtxqpctr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtxqcc1_SPEC;
impl crate::sealed::RegSpec for Cfdtxqcc1_SPEC {
    type DataType = u32;
}

pub type Cfdtxqcc1 = crate::RegValueT<Cfdtxqcc1_SPEC>;

impl Cfdtxqcc1 {
    #[inline(always)]
    pub fn txqe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdtxqcc1::Txqe,
        cfdtxqcc1::Txqe,
        Cfdtxqcc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdtxqcc1::Txqe,
            cfdtxqcc1::Txqe,
            Cfdtxqcc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txqgwe(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdtxqcc1::Txqgwe,
        cfdtxqcc1::Txqgwe,
        Cfdtxqcc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdtxqcc1::Txqgwe,
            cfdtxqcc1::Txqgwe,
            Cfdtxqcc1_SPEC,
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
        cfdtxqcc1::Txqtxie,
        cfdtxqcc1::Txqtxie,
        Cfdtxqcc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            cfdtxqcc1::Txqtxie,
            cfdtxqcc1::Txqtxie,
            Cfdtxqcc1_SPEC,
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
        cfdtxqcc1::Txqim,
        cfdtxqcc1::Txqim,
        Cfdtxqcc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            cfdtxqcc1::Txqim,
            cfdtxqcc1::Txqim,
            Cfdtxqcc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txqdc(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, Cfdtxqcc1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,Cfdtxqcc1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn txqfie(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        cfdtxqcc1::Txqfie,
        cfdtxqcc1::Txqfie,
        Cfdtxqcc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            cfdtxqcc1::Txqfie,
            cfdtxqcc1::Txqfie,
            Cfdtxqcc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txqofrxie(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        cfdtxqcc1::Txqofrxie,
        cfdtxqcc1::Txqofrxie,
        Cfdtxqcc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            cfdtxqcc1::Txqofrxie,
            cfdtxqcc1::Txqofrxie,
            Cfdtxqcc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txqoftxie(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        cfdtxqcc1::Txqoftxie,
        cfdtxqcc1::Txqoftxie,
        Cfdtxqcc1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            cfdtxqcc1::Txqoftxie,
            cfdtxqcc1::Txqoftxie,
            Cfdtxqcc1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdtxqcc1 {
    #[inline(always)]
    fn default() -> Cfdtxqcc1 {
        <crate::RegValueT<Cfdtxqcc1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdtxqcc1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txqe_SPEC;
    pub type Txqe = crate::EnumBitfieldStruct<u8, Txqe_SPEC>;
    impl Txqe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txqgwe_SPEC;
    pub type Txqgwe = crate::EnumBitfieldStruct<u8, Txqgwe_SPEC>;
    impl Txqgwe {
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
    pub struct Txqfie_SPEC;
    pub type Txqfie = crate::EnumBitfieldStruct<u8, Txqfie_SPEC>;
    impl Txqfie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txqofrxie_SPEC;
    pub type Txqofrxie = crate::EnumBitfieldStruct<u8, Txqofrxie_SPEC>;
    impl Txqofrxie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txqoftxie_SPEC;
    pub type Txqoftxie = crate::EnumBitfieldStruct<u8, Txqoftxie_SPEC>;
    impl Txqoftxie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtxqsts1_SPEC;
impl crate::sealed::RegSpec for Cfdtxqsts1_SPEC {
    type DataType = u32;
}

pub type Cfdtxqsts1 = crate::RegValueT<Cfdtxqsts1_SPEC>;

impl Cfdtxqsts1 {
    #[inline(always)]
    pub fn txqemp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdtxqsts1::Txqemp,
        cfdtxqsts1::Txqemp,
        Cfdtxqsts1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdtxqsts1::Txqemp,
            cfdtxqsts1::Txqemp,
            Cfdtxqsts1_SPEC,
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
        cfdtxqsts1::Txqfll,
        cfdtxqsts1::Txqfll,
        Cfdtxqsts1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdtxqsts1::Txqfll,
            cfdtxqsts1::Txqfll,
            Cfdtxqsts1_SPEC,
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
        cfdtxqsts1::Txqtxif,
        cfdtxqsts1::Txqtxif,
        Cfdtxqsts1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdtxqsts1::Txqtxif,
            cfdtxqsts1::Txqtxif,
            Cfdtxqsts1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txqmc(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, Cfdtxqsts1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x3f,1,0,u8,u8,Cfdtxqsts1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn txqfif(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Cfdtxqsts1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,Cfdtxqsts1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn txqofrxif(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Cfdtxqsts1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,Cfdtxqsts1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn txqoftxif(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Cfdtxqsts1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,Cfdtxqsts1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn txqmlt(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        cfdtxqsts1::Txqmlt,
        cfdtxqsts1::Txqmlt,
        Cfdtxqsts1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            cfdtxqsts1::Txqmlt,
            cfdtxqsts1::Txqmlt,
            Cfdtxqsts1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdtxqsts1 {
    #[inline(always)]
    fn default() -> Cfdtxqsts1 {
        <crate::RegValueT<Cfdtxqsts1_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod cfdtxqsts1 {

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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txqmlt_SPEC;
    pub type Txqmlt = crate::EnumBitfieldStruct<u8, Txqmlt_SPEC>;
    impl Txqmlt {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtxqpctr1_SPEC;
impl crate::sealed::RegSpec for Cfdtxqpctr1_SPEC {
    type DataType = u32;
}

pub type Cfdtxqpctr1 = crate::RegValueT<Cfdtxqpctr1_SPEC>;

impl Cfdtxqpctr1 {
    #[inline(always)]
    pub fn txqpc(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtxqpctr1_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtxqpctr1_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtxqpctr1 {
    #[inline(always)]
    fn default() -> Cfdtxqpctr1 {
        <crate::RegValueT<Cfdtxqpctr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtxqcc2_SPEC;
impl crate::sealed::RegSpec for Cfdtxqcc2_SPEC {
    type DataType = u32;
}

pub type Cfdtxqcc2 = crate::RegValueT<Cfdtxqcc2_SPEC>;

impl Cfdtxqcc2 {
    #[inline(always)]
    pub fn txqe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdtxqcc2::Txqe,
        cfdtxqcc2::Txqe,
        Cfdtxqcc2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdtxqcc2::Txqe,
            cfdtxqcc2::Txqe,
            Cfdtxqcc2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txqgwe(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdtxqcc2::Txqgwe,
        cfdtxqcc2::Txqgwe,
        Cfdtxqcc2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdtxqcc2::Txqgwe,
            cfdtxqcc2::Txqgwe,
            Cfdtxqcc2_SPEC,
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
        cfdtxqcc2::Txqtxie,
        cfdtxqcc2::Txqtxie,
        Cfdtxqcc2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            cfdtxqcc2::Txqtxie,
            cfdtxqcc2::Txqtxie,
            Cfdtxqcc2_SPEC,
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
        cfdtxqcc2::Txqim,
        cfdtxqcc2::Txqim,
        Cfdtxqcc2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            cfdtxqcc2::Txqim,
            cfdtxqcc2::Txqim,
            Cfdtxqcc2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txqdc(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, Cfdtxqcc2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,Cfdtxqcc2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn txqfie(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        cfdtxqcc2::Txqfie,
        cfdtxqcc2::Txqfie,
        Cfdtxqcc2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            cfdtxqcc2::Txqfie,
            cfdtxqcc2::Txqfie,
            Cfdtxqcc2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txqofrxie(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        cfdtxqcc2::Txqofrxie,
        cfdtxqcc2::Txqofrxie,
        Cfdtxqcc2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            cfdtxqcc2::Txqofrxie,
            cfdtxqcc2::Txqofrxie,
            Cfdtxqcc2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txqoftxie(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        cfdtxqcc2::Txqoftxie,
        cfdtxqcc2::Txqoftxie,
        Cfdtxqcc2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            cfdtxqcc2::Txqoftxie,
            cfdtxqcc2::Txqoftxie,
            Cfdtxqcc2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdtxqcc2 {
    #[inline(always)]
    fn default() -> Cfdtxqcc2 {
        <crate::RegValueT<Cfdtxqcc2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdtxqcc2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txqe_SPEC;
    pub type Txqe = crate::EnumBitfieldStruct<u8, Txqe_SPEC>;
    impl Txqe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txqgwe_SPEC;
    pub type Txqgwe = crate::EnumBitfieldStruct<u8, Txqgwe_SPEC>;
    impl Txqgwe {
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
    pub struct Txqfie_SPEC;
    pub type Txqfie = crate::EnumBitfieldStruct<u8, Txqfie_SPEC>;
    impl Txqfie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txqofrxie_SPEC;
    pub type Txqofrxie = crate::EnumBitfieldStruct<u8, Txqofrxie_SPEC>;
    impl Txqofrxie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txqoftxie_SPEC;
    pub type Txqoftxie = crate::EnumBitfieldStruct<u8, Txqoftxie_SPEC>;
    impl Txqoftxie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtxqsts2_SPEC;
impl crate::sealed::RegSpec for Cfdtxqsts2_SPEC {
    type DataType = u32;
}

pub type Cfdtxqsts2 = crate::RegValueT<Cfdtxqsts2_SPEC>;

impl Cfdtxqsts2 {
    #[inline(always)]
    pub fn txqemp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdtxqsts2::Txqemp,
        cfdtxqsts2::Txqemp,
        Cfdtxqsts2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdtxqsts2::Txqemp,
            cfdtxqsts2::Txqemp,
            Cfdtxqsts2_SPEC,
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
        cfdtxqsts2::Txqfll,
        cfdtxqsts2::Txqfll,
        Cfdtxqsts2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdtxqsts2::Txqfll,
            cfdtxqsts2::Txqfll,
            Cfdtxqsts2_SPEC,
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
        cfdtxqsts2::Txqtxif,
        cfdtxqsts2::Txqtxif,
        Cfdtxqsts2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdtxqsts2::Txqtxif,
            cfdtxqsts2::Txqtxif,
            Cfdtxqsts2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txqmc(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, Cfdtxqsts2_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x3f,1,0,u8,u8,Cfdtxqsts2_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn txqfif(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Cfdtxqsts2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,Cfdtxqsts2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn txqofrxif(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Cfdtxqsts2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,Cfdtxqsts2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn txqoftxif(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Cfdtxqsts2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,Cfdtxqsts2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn txqmlt(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        cfdtxqsts2::Txqmlt,
        cfdtxqsts2::Txqmlt,
        Cfdtxqsts2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            cfdtxqsts2::Txqmlt,
            cfdtxqsts2::Txqmlt,
            Cfdtxqsts2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdtxqsts2 {
    #[inline(always)]
    fn default() -> Cfdtxqsts2 {
        <crate::RegValueT<Cfdtxqsts2_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod cfdtxqsts2 {

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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txqmlt_SPEC;
    pub type Txqmlt = crate::EnumBitfieldStruct<u8, Txqmlt_SPEC>;
    impl Txqmlt {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtxqpctr2_SPEC;
impl crate::sealed::RegSpec for Cfdtxqpctr2_SPEC {
    type DataType = u32;
}

pub type Cfdtxqpctr2 = crate::RegValueT<Cfdtxqpctr2_SPEC>;

impl Cfdtxqpctr2 {
    #[inline(always)]
    pub fn txqpc(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtxqpctr2_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtxqpctr2_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtxqpctr2 {
    #[inline(always)]
    fn default() -> Cfdtxqpctr2 {
        <crate::RegValueT<Cfdtxqpctr2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtxqcc3_SPEC;
impl crate::sealed::RegSpec for Cfdtxqcc3_SPEC {
    type DataType = u32;
}

pub type Cfdtxqcc3 = crate::RegValueT<Cfdtxqcc3_SPEC>;

impl Cfdtxqcc3 {
    #[inline(always)]
    pub fn txqe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdtxqcc3::Txqe,
        cfdtxqcc3::Txqe,
        Cfdtxqcc3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdtxqcc3::Txqe,
            cfdtxqcc3::Txqe,
            Cfdtxqcc3_SPEC,
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
        cfdtxqcc3::Txqtxie,
        cfdtxqcc3::Txqtxie,
        Cfdtxqcc3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            cfdtxqcc3::Txqtxie,
            cfdtxqcc3::Txqtxie,
            Cfdtxqcc3_SPEC,
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
        cfdtxqcc3::Txqim,
        cfdtxqcc3::Txqim,
        Cfdtxqcc3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            cfdtxqcc3::Txqim,
            cfdtxqcc3::Txqim,
            Cfdtxqcc3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txqdc(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, Cfdtxqcc3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,Cfdtxqcc3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn txqoftxie(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        cfdtxqcc3::Txqoftxie,
        cfdtxqcc3::Txqoftxie,
        Cfdtxqcc3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            cfdtxqcc3::Txqoftxie,
            cfdtxqcc3::Txqoftxie,
            Cfdtxqcc3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdtxqcc3 {
    #[inline(always)]
    fn default() -> Cfdtxqcc3 {
        <crate::RegValueT<Cfdtxqcc3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdtxqcc3 {

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
    pub struct Txqoftxie_SPEC;
    pub type Txqoftxie = crate::EnumBitfieldStruct<u8, Txqoftxie_SPEC>;
    impl Txqoftxie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtxqsts3_SPEC;
impl crate::sealed::RegSpec for Cfdtxqsts3_SPEC {
    type DataType = u32;
}

pub type Cfdtxqsts3 = crate::RegValueT<Cfdtxqsts3_SPEC>;

impl Cfdtxqsts3 {
    #[inline(always)]
    pub fn txqemp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdtxqsts3::Txqemp,
        cfdtxqsts3::Txqemp,
        Cfdtxqsts3_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdtxqsts3::Txqemp,
            cfdtxqsts3::Txqemp,
            Cfdtxqsts3_SPEC,
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
        cfdtxqsts3::Txqfll,
        cfdtxqsts3::Txqfll,
        Cfdtxqsts3_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdtxqsts3::Txqfll,
            cfdtxqsts3::Txqfll,
            Cfdtxqsts3_SPEC,
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
        cfdtxqsts3::Txqtxif,
        cfdtxqsts3::Txqtxif,
        Cfdtxqsts3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdtxqsts3::Txqtxif,
            cfdtxqsts3::Txqtxif,
            Cfdtxqsts3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txqmc(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, Cfdtxqsts3_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x3f,1,0,u8,u8,Cfdtxqsts3_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn txqoftxif(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Cfdtxqsts3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,Cfdtxqsts3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtxqsts3 {
    #[inline(always)]
    fn default() -> Cfdtxqsts3 {
        <crate::RegValueT<Cfdtxqsts3_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod cfdtxqsts3 {

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
pub struct Cfdtxqpctr3_SPEC;
impl crate::sealed::RegSpec for Cfdtxqpctr3_SPEC {
    type DataType = u32;
}

pub type Cfdtxqpctr3 = crate::RegValueT<Cfdtxqpctr3_SPEC>;

impl Cfdtxqpctr3 {
    #[inline(always)]
    pub fn txqpc(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtxqpctr3_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtxqpctr3_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtxqpctr3 {
    #[inline(always)]
    fn default() -> Cfdtxqpctr3 {
        <crate::RegValueT<Cfdtxqpctr3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtxqests_SPEC;
impl crate::sealed::RegSpec for Cfdtxqests_SPEC {
    type DataType = u32;
}

pub type Cfdtxqests = crate::RegValueT<Cfdtxqests_SPEC>;

impl Cfdtxqests {
    #[inline(always)]
    pub fn txqxemp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        cfdtxqests::TxQxEmp,
        cfdtxqests::TxQxEmp,
        Cfdtxqests_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            cfdtxqests::TxQxEmp,
            cfdtxqests::TxQxEmp,
            Cfdtxqests_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdtxqests {
    #[inline(always)]
    fn default() -> Cfdtxqests {
        <crate::RegValueT<Cfdtxqests_SPEC> as RegisterValue<_>>::new(255)
    }
}
pub mod cfdtxqests {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct TxQxEmp_SPEC;
    pub type TxQxEmp = crate::EnumBitfieldStruct<u8, TxQxEmp_SPEC>;
    impl TxQxEmp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtxqfists_SPEC;
impl crate::sealed::RegSpec for Cfdtxqfists_SPEC {
    type DataType = u32;
}

pub type Cfdtxqfists = crate::RegValueT<Cfdtxqfists_SPEC>;

impl Cfdtxqfists {
    #[inline(always)]
    pub fn txq0full(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        cfdtxqfists::Txq0Full,
        cfdtxqfists::Txq0Full,
        Cfdtxqfists_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            cfdtxqfists::Txq0Full,
            cfdtxqfists::Txq0Full,
            Cfdtxqfists_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txq1full(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x7,
        1,
        0,
        cfdtxqfists::Txq1Full,
        cfdtxqfists::Txq1Full,
        Cfdtxqfists_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            cfdtxqfists::Txq1Full,
            cfdtxqfists::Txq1Full,
            Cfdtxqfists_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdtxqfists {
    #[inline(always)]
    fn default() -> Cfdtxqfists {
        <crate::RegValueT<Cfdtxqfists_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdtxqfists {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txq0Full_SPEC;
    pub type Txq0Full = crate::EnumBitfieldStruct<u8, Txq0Full_SPEC>;
    impl Txq0Full {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txq1Full_SPEC;
    pub type Txq1Full = crate::EnumBitfieldStruct<u8, Txq1Full_SPEC>;
    impl Txq1Full {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtxqmsts_SPEC;
impl crate::sealed::RegSpec for Cfdtxqmsts_SPEC {
    type DataType = u32;
}

pub type Cfdtxqmsts = crate::RegValueT<Cfdtxqmsts_SPEC>;

impl Cfdtxqmsts {
    #[inline(always)]
    pub fn txq0ml(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        cfdtxqmsts::Txq0Ml,
        cfdtxqmsts::Txq0Ml,
        Cfdtxqmsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            cfdtxqmsts::Txq0Ml,
            cfdtxqmsts::Txq0Ml,
            Cfdtxqmsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txq1ml(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x7,
        1,
        0,
        cfdtxqmsts::Txq1Ml,
        cfdtxqmsts::Txq1Ml,
        Cfdtxqmsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            cfdtxqmsts::Txq1Ml,
            cfdtxqmsts::Txq1Ml,
            Cfdtxqmsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdtxqmsts {
    #[inline(always)]
    fn default() -> Cfdtxqmsts {
        <crate::RegValueT<Cfdtxqmsts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdtxqmsts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txq0Ml_SPEC;
    pub type Txq0Ml = crate::EnumBitfieldStruct<u8, Txq0Ml_SPEC>;
    impl Txq0Ml {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txq1Ml_SPEC;
    pub type Txq1Ml = crate::EnumBitfieldStruct<u8, Txq1Ml_SPEC>;
    impl Txq1Ml {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtxqists_SPEC;
impl crate::sealed::RegSpec for Cfdtxqists_SPEC {
    type DataType = u32;
}

pub type Cfdtxqists = crate::RegValueT<Cfdtxqists_SPEC>;

impl Cfdtxqists {
    #[inline(always)]
    pub fn txq0isf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        cfdtxqists::Txq0Isf,
        cfdtxqists::Txq0Isf,
        Cfdtxqists_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            cfdtxqists::Txq0Isf,
            cfdtxqists::Txq0Isf,
            Cfdtxqists_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txq1isf(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xf,
        1,
        0,
        cfdtxqists::Txq1Isf,
        cfdtxqists::Txq1Isf,
        Cfdtxqists_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            cfdtxqists::Txq1Isf,
            cfdtxqists::Txq1Isf,
            Cfdtxqists_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdtxqists {
    #[inline(always)]
    fn default() -> Cfdtxqists {
        <crate::RegValueT<Cfdtxqists_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdtxqists {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txq0Isf_SPEC;
    pub type Txq0Isf = crate::EnumBitfieldStruct<u8, Txq0Isf_SPEC>;
    impl Txq0Isf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txq1Isf_SPEC;
    pub type Txq1Isf = crate::EnumBitfieldStruct<u8, Txq1Isf_SPEC>;
    impl Txq1Isf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtxqoftists_SPEC;
impl crate::sealed::RegSpec for Cfdtxqoftists_SPEC {
    type DataType = u32;
}

pub type Cfdtxqoftists = crate::RegValueT<Cfdtxqoftists_SPEC>;

impl Cfdtxqoftists {
    #[inline(always)]
    pub fn txq0oftisf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        cfdtxqoftists::Txq0Oftisf,
        cfdtxqoftists::Txq0Oftisf,
        Cfdtxqoftists_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            cfdtxqoftists::Txq0Oftisf,
            cfdtxqoftists::Txq0Oftisf,
            Cfdtxqoftists_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txq1oftisf(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xf,
        1,
        0,
        cfdtxqoftists::Txq1Oftisf,
        cfdtxqoftists::Txq1Oftisf,
        Cfdtxqoftists_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            cfdtxqoftists::Txq1Oftisf,
            cfdtxqoftists::Txq1Oftisf,
            Cfdtxqoftists_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdtxqoftists {
    #[inline(always)]
    fn default() -> Cfdtxqoftists {
        <crate::RegValueT<Cfdtxqoftists_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdtxqoftists {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txq0Oftisf_SPEC;
    pub type Txq0Oftisf = crate::EnumBitfieldStruct<u8, Txq0Oftisf_SPEC>;
    impl Txq0Oftisf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txq1Oftisf_SPEC;
    pub type Txq1Oftisf = crate::EnumBitfieldStruct<u8, Txq1Oftisf_SPEC>;
    impl Txq1Oftisf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtxqofrists_SPEC;
impl crate::sealed::RegSpec for Cfdtxqofrists_SPEC {
    type DataType = u32;
}

pub type Cfdtxqofrists = crate::RegValueT<Cfdtxqofrists_SPEC>;

impl Cfdtxqofrists {
    #[inline(always)]
    pub fn txq0ofrisf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        cfdtxqofrists::Txq0Ofrisf,
        cfdtxqofrists::Txq0Ofrisf,
        Cfdtxqofrists_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            cfdtxqofrists::Txq0Ofrisf,
            cfdtxqofrists::Txq0Ofrisf,
            Cfdtxqofrists_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txq1ofrisf(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x7,
        1,
        0,
        cfdtxqofrists::Txq1Ofrisf,
        cfdtxqofrists::Txq1Ofrisf,
        Cfdtxqofrists_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            cfdtxqofrists::Txq1Ofrisf,
            cfdtxqofrists::Txq1Ofrisf,
            Cfdtxqofrists_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdtxqofrists {
    #[inline(always)]
    fn default() -> Cfdtxqofrists {
        <crate::RegValueT<Cfdtxqofrists_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdtxqofrists {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txq0Ofrisf_SPEC;
    pub type Txq0Ofrisf = crate::EnumBitfieldStruct<u8, Txq0Ofrisf_SPEC>;
    impl Txq0Ofrisf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txq1Ofrisf_SPEC;
    pub type Txq1Ofrisf = crate::EnumBitfieldStruct<u8, Txq1Ofrisf_SPEC>;
    impl Txq1Ofrisf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtxqfsts_SPEC;
impl crate::sealed::RegSpec for Cfdtxqfsts_SPEC {
    type DataType = u32;
}

pub type Cfdtxqfsts = crate::RegValueT<Cfdtxqfsts_SPEC>;

impl Cfdtxqfsts {
    #[inline(always)]
    pub fn txq0fsf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        cfdtxqfsts::Txq0Fsf,
        cfdtxqfsts::Txq0Fsf,
        Cfdtxqfsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            cfdtxqfsts::Txq0Fsf,
            cfdtxqfsts::Txq0Fsf,
            Cfdtxqfsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txq1fsf(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xf,
        1,
        0,
        cfdtxqfsts::Txq1Fsf,
        cfdtxqfsts::Txq1Fsf,
        Cfdtxqfsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            cfdtxqfsts::Txq1Fsf,
            cfdtxqfsts::Txq1Fsf,
            Cfdtxqfsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdtxqfsts {
    #[inline(always)]
    fn default() -> Cfdtxqfsts {
        <crate::RegValueT<Cfdtxqfsts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdtxqfsts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txq0Fsf_SPEC;
    pub type Txq0Fsf = crate::EnumBitfieldStruct<u8, Txq0Fsf_SPEC>;
    impl Txq0Fsf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txq1Fsf_SPEC;
    pub type Txq1Fsf = crate::EnumBitfieldStruct<u8, Txq1Fsf_SPEC>;
    impl Txq1Fsf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
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
    pub fn thldge(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        cfdthlcc::Thldge,
        cfdthlcc::Thldge,
        Cfdthlcc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            cfdthlcc::Thldge,
            cfdthlcc::Thldge,
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Thldge_SPEC;
    pub type Thldge = crate::EnumBitfieldStruct<u8, Thldge_SPEC>;
    impl Thldge {
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
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, Cfdthlsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x3f,1,0,u8,u8,Cfdthlsts_SPEC,crate::common::R>::from_register(self,0)
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
pub struct Cfdgtintsts0_SPEC;
impl crate::sealed::RegSpec for Cfdgtintsts0_SPEC {
    type DataType = u32;
}

pub type Cfdgtintsts0 = crate::RegValueT<Cfdgtintsts0_SPEC>;

impl Cfdgtintsts0 {
    #[inline(always)]
    pub fn tsif0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdgtintsts0::Tsif0,
        cfdgtintsts0::Tsif0,
        Cfdgtintsts0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdgtintsts0::Tsif0,
            cfdgtintsts0::Tsif0,
            Cfdgtintsts0_SPEC,
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
        cfdgtintsts0::Tai0,
        cfdgtintsts0::Tai0,
        Cfdgtintsts0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdgtintsts0::Tai0,
            cfdgtintsts0::Tai0,
            Cfdgtintsts0_SPEC,
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
        cfdgtintsts0::Tqif0,
        cfdgtintsts0::Tqif0,
        Cfdgtintsts0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdgtintsts0::Tqif0,
            cfdgtintsts0::Tqif0,
            Cfdgtintsts0_SPEC,
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
        cfdgtintsts0::Cftif0,
        cfdgtintsts0::Cftif0,
        Cfdgtintsts0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            cfdgtintsts0::Cftif0,
            cfdgtintsts0::Cftif0,
            Cfdgtintsts0_SPEC,
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
        cfdgtintsts0::Thif0,
        cfdgtintsts0::Thif0,
        Cfdgtintsts0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            cfdgtintsts0::Thif0,
            cfdgtintsts0::Thif0,
            Cfdgtintsts0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tqofifo(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        cfdgtintsts0::Tqofifo,
        cfdgtintsts0::Tqofifo,
        Cfdgtintsts0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            cfdgtintsts0::Tqofifo,
            cfdgtintsts0::Tqofifo,
            Cfdgtintsts0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfotifo(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        cfdgtintsts0::Cfotifo,
        cfdgtintsts0::Cfotifo,
        Cfdgtintsts0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            cfdgtintsts0::Cfotifo,
            cfdgtintsts0::Cfotifo,
            Cfdgtintsts0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tsif1(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cfdgtintsts0::Tsif1,
        cfdgtintsts0::Tsif1,
        Cfdgtintsts0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cfdgtintsts0::Tsif1,
            cfdgtintsts0::Tsif1,
            Cfdgtintsts0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn taif1(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        cfdgtintsts0::Taif1,
        cfdgtintsts0::Taif1,
        Cfdgtintsts0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            cfdgtintsts0::Taif1,
            cfdgtintsts0::Taif1,
            Cfdgtintsts0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tqif1(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        cfdgtintsts0::Tqif1,
        cfdgtintsts0::Tqif1,
        Cfdgtintsts0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            cfdgtintsts0::Tqif1,
            cfdgtintsts0::Tqif1,
            Cfdgtintsts0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cftif1(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        cfdgtintsts0::Cftif1,
        cfdgtintsts0::Cftif1,
        Cfdgtintsts0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            cfdgtintsts0::Cftif1,
            cfdgtintsts0::Cftif1,
            Cfdgtintsts0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn thif1(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        cfdgtintsts0::Thif1,
        cfdgtintsts0::Thif1,
        Cfdgtintsts0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            cfdgtintsts0::Thif1,
            cfdgtintsts0::Thif1,
            Cfdgtintsts0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tqofif1(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        cfdgtintsts0::Tqofif1,
        cfdgtintsts0::Tqofif1,
        Cfdgtintsts0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            cfdgtintsts0::Tqofif1,
            cfdgtintsts0::Tqofif1,
            Cfdgtintsts0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfotif1(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        cfdgtintsts0::Cfotif1,
        cfdgtintsts0::Cfotif1,
        Cfdgtintsts0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            cfdgtintsts0::Cfotif1,
            cfdgtintsts0::Cfotif1,
            Cfdgtintsts0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdgtintsts0 {
    #[inline(always)]
    fn default() -> Cfdgtintsts0 {
        <crate::RegValueT<Cfdgtintsts0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdgtintsts0 {

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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tqofifo_SPEC;
    pub type Tqofifo = crate::EnumBitfieldStruct<u8, Tqofifo_SPEC>;
    impl Tqofifo {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfotifo_SPEC;
    pub type Cfotifo = crate::EnumBitfieldStruct<u8, Cfotifo_SPEC>;
    impl Cfotifo {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsif1_SPEC;
    pub type Tsif1 = crate::EnumBitfieldStruct<u8, Tsif1_SPEC>;
    impl Tsif1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Taif1_SPEC;
    pub type Taif1 = crate::EnumBitfieldStruct<u8, Taif1_SPEC>;
    impl Taif1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tqif1_SPEC;
    pub type Tqif1 = crate::EnumBitfieldStruct<u8, Tqif1_SPEC>;
    impl Tqif1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cftif1_SPEC;
    pub type Cftif1 = crate::EnumBitfieldStruct<u8, Cftif1_SPEC>;
    impl Cftif1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Thif1_SPEC;
    pub type Thif1 = crate::EnumBitfieldStruct<u8, Thif1_SPEC>;
    impl Thif1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tqofif1_SPEC;
    pub type Tqofif1 = crate::EnumBitfieldStruct<u8, Tqofif1_SPEC>;
    impl Tqofif1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfotif1_SPEC;
    pub type Cfotif1 = crate::EnumBitfieldStruct<u8, Cfotif1_SPEC>;
    impl Cfotif1 {
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
    pub fn icbce(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        cfdgtstcfg::Icbce,
        cfdgtstcfg::Icbce,
        Cfdgtstcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            cfdgtstcfg::Icbce,
            cfdgtstcfg::Icbce,
            Cfdgtstcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rtmps(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, u16, Cfdgtstcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3ff,1,0,u16,u16,Cfdgtstcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdgtstcfg {
    #[inline(always)]
    fn default() -> Cfdgtstcfg {
        <crate::RegValueT<Cfdgtstcfg_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdgtstcfg {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Icbce_SPEC;
    pub type Icbce = crate::EnumBitfieldStruct<u8, Icbce_SPEC>;
    impl Icbce {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
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
    pub fn icbctme(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdgtstctr::Icbctme,
        cfdgtstctr::Icbctme,
        Cfdgtstctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdgtstctr::Icbctme,
            cfdgtstctr::Icbctme,
            Cfdgtstctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

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
    pub struct Icbctme_SPEC;
    pub type Icbctme = crate::EnumBitfieldStruct<u8, Icbctme_SPEC>;
    impl Icbctme {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
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
    pub fn rfdmae2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cfdcdtct::Rfdmae2,
        cfdcdtct::Rfdmae2,
        Cfdcdtct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdcdtct::Rfdmae2,
            cfdcdtct::Rfdmae2,
            Cfdcdtct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdmae3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        cfdcdtct::Rfdmae3,
        cfdcdtct::Rfdmae3,
        Cfdcdtct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            cfdcdtct::Rfdmae3,
            cfdcdtct::Rfdmae3,
            Cfdcdtct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdmae4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        cfdcdtct::Rfdmae4,
        cfdcdtct::Rfdmae4,
        Cfdcdtct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            cfdcdtct::Rfdmae4,
            cfdcdtct::Rfdmae4,
            Cfdcdtct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdmae5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        cfdcdtct::Rfdmae5,
        cfdcdtct::Rfdmae5,
        Cfdcdtct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            cfdcdtct::Rfdmae5,
            cfdcdtct::Rfdmae5,
            Cfdcdtct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdmae6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        cfdcdtct::Rfdmae6,
        cfdcdtct::Rfdmae6,
        Cfdcdtct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            cfdcdtct::Rfdmae6,
            cfdcdtct::Rfdmae6,
            Cfdcdtct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdmae7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        cfdcdtct::Rfdmae7,
        cfdcdtct::Rfdmae7,
        Cfdcdtct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            cfdcdtct::Rfdmae7,
            cfdcdtct::Rfdmae7,
            Cfdcdtct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdmae0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cfdcdtct::Cfdmae0,
        cfdcdtct::Cfdmae0,
        Cfdcdtct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cfdcdtct::Cfdmae0,
            cfdcdtct::Cfdmae0,
            Cfdcdtct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdmae1(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        cfdcdtct::Cfdmae1,
        cfdcdtct::Cfdmae1,
        Cfdcdtct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            cfdcdtct::Cfdmae1,
            cfdcdtct::Cfdmae1,
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
    pub struct Rfdmae2_SPEC;
    pub type Rfdmae2 = crate::EnumBitfieldStruct<u8, Rfdmae2_SPEC>;
    impl Rfdmae2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdmae3_SPEC;
    pub type Rfdmae3 = crate::EnumBitfieldStruct<u8, Rfdmae3_SPEC>;
    impl Rfdmae3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdmae4_SPEC;
    pub type Rfdmae4 = crate::EnumBitfieldStruct<u8, Rfdmae4_SPEC>;
    impl Rfdmae4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdmae5_SPEC;
    pub type Rfdmae5 = crate::EnumBitfieldStruct<u8, Rfdmae5_SPEC>;
    impl Rfdmae5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdmae6_SPEC;
    pub type Rfdmae6 = crate::EnumBitfieldStruct<u8, Rfdmae6_SPEC>;
    impl Rfdmae6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdmae7_SPEC;
    pub type Rfdmae7 = crate::EnumBitfieldStruct<u8, Rfdmae7_SPEC>;
    impl Rfdmae7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdmae0_SPEC;
    pub type Cfdmae0 = crate::EnumBitfieldStruct<u8, Cfdmae0_SPEC>;
    impl Cfdmae0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdmae1_SPEC;
    pub type Cfdmae1 = crate::EnumBitfieldStruct<u8, Cfdmae1_SPEC>;
    impl Cfdmae1 {
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
    pub fn rfdmasts2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cfdcdtsts::Rfdmasts2,
        cfdcdtsts::Rfdmasts2,
        Cfdcdtsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdcdtsts::Rfdmasts2,
            cfdcdtsts::Rfdmasts2,
            Cfdcdtsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdmasts3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        cfdcdtsts::Rfdmasts3,
        cfdcdtsts::Rfdmasts3,
        Cfdcdtsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            cfdcdtsts::Rfdmasts3,
            cfdcdtsts::Rfdmasts3,
            Cfdcdtsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdmasts4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        cfdcdtsts::Rfdmasts4,
        cfdcdtsts::Rfdmasts4,
        Cfdcdtsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            cfdcdtsts::Rfdmasts4,
            cfdcdtsts::Rfdmasts4,
            Cfdcdtsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdmasts5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        cfdcdtsts::Rfdmasts5,
        cfdcdtsts::Rfdmasts5,
        Cfdcdtsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            cfdcdtsts::Rfdmasts5,
            cfdcdtsts::Rfdmasts5,
            Cfdcdtsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdmasts6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        cfdcdtsts::Rfdmasts6,
        cfdcdtsts::Rfdmasts6,
        Cfdcdtsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            cfdcdtsts::Rfdmasts6,
            cfdcdtsts::Rfdmasts6,
            Cfdcdtsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfdmasts7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        cfdcdtsts::Rfdmasts7,
        cfdcdtsts::Rfdmasts7,
        Cfdcdtsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            cfdcdtsts::Rfdmasts7,
            cfdcdtsts::Rfdmasts7,
            Cfdcdtsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdmasts0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cfdcdtsts::Cfdmasts0,
        cfdcdtsts::Cfdmasts0,
        Cfdcdtsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cfdcdtsts::Cfdmasts0,
            cfdcdtsts::Cfdmasts0,
            Cfdcdtsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdmasts1(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        cfdcdtsts::Cfdmasts1,
        cfdcdtsts::Cfdmasts1,
        Cfdcdtsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            cfdcdtsts::Cfdmasts1,
            cfdcdtsts::Cfdmasts1,
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
    pub struct Rfdmasts2_SPEC;
    pub type Rfdmasts2 = crate::EnumBitfieldStruct<u8, Rfdmasts2_SPEC>;
    impl Rfdmasts2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdmasts3_SPEC;
    pub type Rfdmasts3 = crate::EnumBitfieldStruct<u8, Rfdmasts3_SPEC>;
    impl Rfdmasts3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdmasts4_SPEC;
    pub type Rfdmasts4 = crate::EnumBitfieldStruct<u8, Rfdmasts4_SPEC>;
    impl Rfdmasts4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdmasts5_SPEC;
    pub type Rfdmasts5 = crate::EnumBitfieldStruct<u8, Rfdmasts5_SPEC>;
    impl Rfdmasts5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdmasts6_SPEC;
    pub type Rfdmasts6 = crate::EnumBitfieldStruct<u8, Rfdmasts6_SPEC>;
    impl Rfdmasts6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfdmasts7_SPEC;
    pub type Rfdmasts7 = crate::EnumBitfieldStruct<u8, Rfdmasts7_SPEC>;
    impl Rfdmasts7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdmasts0_SPEC;
    pub type Cfdmasts0 = crate::EnumBitfieldStruct<u8, Cfdmasts0_SPEC>;
    impl Cfdmasts0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdmasts1_SPEC;
    pub type Cfdmasts1 = crate::EnumBitfieldStruct<u8, Cfdmasts1_SPEC>;
    impl Cfdmasts1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcdttct_SPEC;
impl crate::sealed::RegSpec for Cfdcdttct_SPEC {
    type DataType = u32;
}

pub type Cfdcdttct = crate::RegValueT<Cfdcdttct_SPEC>;

impl Cfdcdttct {
    #[inline(always)]
    pub fn tq0dmae0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdcdttct::Tq0Dmae0,
        cfdcdttct::Tq0Dmae0,
        Cfdcdttct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdcdttct::Tq0Dmae0,
            cfdcdttct::Tq0Dmae0,
            Cfdcdttct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tq0dmae1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdcdttct::Tq0Dmae1,
        cfdcdttct::Tq0Dmae1,
        Cfdcdttct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdcdttct::Tq0Dmae1,
            cfdcdttct::Tq0Dmae1,
            Cfdcdttct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tq3dmae0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cfdcdttct::Tq3Dmae0,
        cfdcdttct::Tq3Dmae0,
        Cfdcdttct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cfdcdttct::Tq3Dmae0,
            cfdcdttct::Tq3Dmae0,
            Cfdcdttct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tq3dmae1(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        cfdcdttct::Tq3Dmae1,
        cfdcdttct::Tq3Dmae1,
        Cfdcdttct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            cfdcdttct::Tq3Dmae1,
            cfdcdttct::Tq3Dmae1,
            Cfdcdttct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdmae0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        cfdcdttct::Cfdmae0,
        cfdcdttct::Cfdmae0,
        Cfdcdttct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            cfdcdttct::Cfdmae0,
            cfdcdttct::Cfdmae0,
            Cfdcdttct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdmae1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        cfdcdttct::Cfdmae1,
        cfdcdttct::Cfdmae1,
        Cfdcdttct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            cfdcdttct::Cfdmae1,
            cfdcdttct::Cfdmae1,
            Cfdcdttct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcdttct {
    #[inline(always)]
    fn default() -> Cfdcdttct {
        <crate::RegValueT<Cfdcdttct_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcdttct {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tq0Dmae0_SPEC;
    pub type Tq0Dmae0 = crate::EnumBitfieldStruct<u8, Tq0Dmae0_SPEC>;
    impl Tq0Dmae0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tq0Dmae1_SPEC;
    pub type Tq0Dmae1 = crate::EnumBitfieldStruct<u8, Tq0Dmae1_SPEC>;
    impl Tq0Dmae1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tq3Dmae0_SPEC;
    pub type Tq3Dmae0 = crate::EnumBitfieldStruct<u8, Tq3Dmae0_SPEC>;
    impl Tq3Dmae0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tq3Dmae1_SPEC;
    pub type Tq3Dmae1 = crate::EnumBitfieldStruct<u8, Tq3Dmae1_SPEC>;
    impl Tq3Dmae1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdmae0_SPEC;
    pub type Cfdmae0 = crate::EnumBitfieldStruct<u8, Cfdmae0_SPEC>;
    impl Cfdmae0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdmae1_SPEC;
    pub type Cfdmae1 = crate::EnumBitfieldStruct<u8, Cfdmae1_SPEC>;
    impl Cfdmae1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcdttsts_SPEC;
impl crate::sealed::RegSpec for Cfdcdttsts_SPEC {
    type DataType = u32;
}

pub type Cfdcdttsts = crate::RegValueT<Cfdcdttsts_SPEC>;

impl Cfdcdttsts {
    #[inline(always)]
    pub fn tq0dmasts0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdcdttsts::Tq0Dmasts0,
        cfdcdttsts::Tq0Dmasts0,
        Cfdcdttsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdcdttsts::Tq0Dmasts0,
            cfdcdttsts::Tq0Dmasts0,
            Cfdcdttsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tq0dmasts1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cfdcdttsts::Tq0Dmasts1,
        cfdcdttsts::Tq0Dmasts1,
        Cfdcdttsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdcdttsts::Tq0Dmasts1,
            cfdcdttsts::Tq0Dmasts1,
            Cfdcdttsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tq3dmasts0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cfdcdttsts::Tq3Dmasts0,
        cfdcdttsts::Tq3Dmasts0,
        Cfdcdttsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cfdcdttsts::Tq3Dmasts0,
            cfdcdttsts::Tq3Dmasts0,
            Cfdcdttsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tq3dmasts1(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        cfdcdttsts::Tq3Dmasts1,
        cfdcdttsts::Tq3Dmasts1,
        Cfdcdttsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            cfdcdttsts::Tq3Dmasts1,
            cfdcdttsts::Tq3Dmasts1,
            Cfdcdttsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdmasts0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        cfdcdttsts::Cfdmasts0,
        cfdcdttsts::Cfdmasts0,
        Cfdcdttsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            cfdcdttsts::Cfdmasts0,
            cfdcdttsts::Cfdmasts0,
            Cfdcdttsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfdmasts1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        cfdcdttsts::Cfdmasts1,
        cfdcdttsts::Cfdmasts1,
        Cfdcdttsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            cfdcdttsts::Cfdmasts1,
            cfdcdttsts::Cfdmasts1,
            Cfdcdttsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcdttsts {
    #[inline(always)]
    fn default() -> Cfdcdttsts {
        <crate::RegValueT<Cfdcdttsts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcdttsts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tq0Dmasts0_SPEC;
    pub type Tq0Dmasts0 = crate::EnumBitfieldStruct<u8, Tq0Dmasts0_SPEC>;
    impl Tq0Dmasts0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tq0Dmasts1_SPEC;
    pub type Tq0Dmasts1 = crate::EnumBitfieldStruct<u8, Tq0Dmasts1_SPEC>;
    impl Tq0Dmasts1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tq3Dmasts0_SPEC;
    pub type Tq3Dmasts0 = crate::EnumBitfieldStruct<u8, Tq3Dmasts0_SPEC>;
    impl Tq3Dmasts0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tq3Dmasts1_SPEC;
    pub type Tq3Dmasts1 = crate::EnumBitfieldStruct<u8, Tq3Dmasts1_SPEC>;
    impl Tq3Dmasts1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdmasts0_SPEC;
    pub type Cfdmasts0 = crate::EnumBitfieldStruct<u8, Cfdmasts0_SPEC>;
    impl Cfdmasts0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfdmasts1_SPEC;
    pub type Cfdmasts1 = crate::EnumBitfieldStruct<u8, Cfdmasts1_SPEC>;
    impl Cfdmasts1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdgrintsts_SPEC;
impl crate::sealed::RegSpec for Cfdgrintsts_SPEC {
    type DataType = u32;
}

pub type Cfdgrintsts = crate::RegValueT<Cfdgrintsts_SPEC>;

impl Cfdgrintsts {
    #[inline(always)]
    pub fn qfif(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        cfdgrintsts::Qfif,
        cfdgrintsts::Qfif,
        Cfdgrintsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            cfdgrintsts::Qfif,
            cfdgrintsts::Qfif,
            Cfdgrintsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qofrif(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        cfdgrintsts::Qofrif,
        cfdgrintsts::Qofrif,
        Cfdgrintsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            cfdgrintsts::Qofrif,
            cfdgrintsts::Qofrif,
            Cfdgrintsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfrif(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        cfdgrintsts::Cfrif,
        cfdgrintsts::Cfrif,
        Cfdgrintsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            cfdgrintsts::Cfrif,
            cfdgrintsts::Cfrif,
            Cfdgrintsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfrfif(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        cfdgrintsts::Cfrfif,
        cfdgrintsts::Cfrfif,
        Cfdgrintsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            cfdgrintsts::Cfrfif,
            cfdgrintsts::Cfrfif,
            Cfdgrintsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfofrif(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x7,
        1,
        0,
        cfdgrintsts::Cfofrif,
        cfdgrintsts::Cfofrif,
        Cfdgrintsts_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            28,
            0x7,
            1,
            0,
            cfdgrintsts::Cfofrif,
            cfdgrintsts::Cfofrif,
            Cfdgrintsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdgrintsts {
    #[inline(always)]
    fn default() -> Cfdgrintsts {
        <crate::RegValueT<Cfdgrintsts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdgrintsts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Qfif_SPEC;
    pub type Qfif = crate::EnumBitfieldStruct<u8, Qfif_SPEC>;
    impl Qfif {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Qofrif_SPEC;
    pub type Qofrif = crate::EnumBitfieldStruct<u8, Qofrif_SPEC>;
    impl Qofrif {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfrif_SPEC;
    pub type Cfrif = crate::EnumBitfieldStruct<u8, Cfrif_SPEC>;
    impl Cfrif {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfrfif_SPEC;
    pub type Cfrfif = crate::EnumBitfieldStruct<u8, Cfrfif_SPEC>;
    impl Cfrfif {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfofrif_SPEC;
    pub type Cfofrif = crate::EnumBitfieldStruct<u8, Cfofrif_SPEC>;
    impl Cfofrif {
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
pub struct Cfdcdcfg_SPEC;
impl crate::sealed::RegSpec for Cfdcdcfg_SPEC {
    type DataType = u32;
}

pub type Cfdcdcfg = crate::RegValueT<Cfdcdcfg_SPEC>;

impl Cfdcdcfg {
    #[inline(always)]
    pub fn dbrp(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcdcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcdcfg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dtseg1(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, Cfdcdcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,Cfdcdcfg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dtseg2(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Cfdcdcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Cfdcdcfg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dsjw(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, u8, Cfdcdcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8,u8,Cfdcdcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcdcfg {
    #[inline(always)]
    fn default() -> Cfdcdcfg {
        <crate::RegValueT<Cfdcdcfg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdcfg_SPEC;
impl crate::sealed::RegSpec for Cfdcfdcfg_SPEC {
    type DataType = u32;
}

pub type Cfdcfdcfg = crate::RegValueT<Cfdcfdcfg_SPEC>;

impl Cfdcfdcfg {
    #[inline(always)]
    pub fn eoccfg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        cfdcfdcfg::Eoccfg,
        cfdcfdcfg::Eoccfg,
        Cfdcfdcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            cfdcfdcfg::Eoccfg,
            cfdcfdcfg::Eoccfg,
            Cfdcfdcfg_SPEC,
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
        cfdcfdcfg::Tdcoc,
        cfdcfdcfg::Tdcoc,
        Cfdcfdcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cfdcfdcfg::Tdcoc,
            cfdcfdcfg::Tdcoc,
            Cfdcfdcfg_SPEC,
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
        cfdcfdcfg::Tdce,
        cfdcfdcfg::Tdce,
        Cfdcfdcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            cfdcfdcfg::Tdce,
            cfdcfdcfg::Tdce,
            Cfdcfdcfg_SPEC,
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
        cfdcfdcfg::Esic,
        cfdcfdcfg::Esic,
        Cfdcfdcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            cfdcfdcfg::Esic,
            cfdcfdcfg::Esic,
            Cfdcfdcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tdco(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdcfg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gwen(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        cfdcfdcfg::Gwen,
        cfdcfdcfg::Gwen,
        Cfdcfdcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            cfdcfdcfg::Gwen,
            cfdcfdcfg::Gwen,
            Cfdcfdcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gwfdf(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        cfdcfdcfg::Gwfdf,
        cfdcfdcfg::Gwfdf,
        Cfdcfdcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            cfdcfdcfg::Gwfdf,
            cfdcfdcfg::Gwfdf,
            Cfdcfdcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gwbrs(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        cfdcfdcfg::Gwbrs,
        cfdcfdcfg::Gwbrs,
        Cfdcfdcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            cfdcfdcfg::Gwbrs,
            cfdcfdcfg::Gwbrs,
            Cfdcfdcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fdoe(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        cfdcfdcfg::Fdoe,
        cfdcfdcfg::Fdoe,
        Cfdcfdcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            cfdcfdcfg::Fdoe,
            cfdcfdcfg::Fdoe,
            Cfdcfdcfg_SPEC,
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
        cfdcfdcfg::Refe,
        cfdcfdcfg::Refe,
        Cfdcfdcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            cfdcfdcfg::Refe,
            cfdcfdcfg::Refe,
            Cfdcfdcfg_SPEC,
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
        cfdcfdcfg::Cloe,
        cfdcfdcfg::Cloe,
        Cfdcfdcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            cfdcfdcfg::Cloe,
            cfdcfdcfg::Cloe,
            Cfdcfdcfg_SPEC,
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
        cfdcfdcfg::Cfdte,
        cfdcfdcfg::Cfdte,
        Cfdcfdcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            cfdcfdcfg::Cfdte,
            cfdcfdcfg::Cfdte,
            Cfdcfdcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcfdcfg {
    #[inline(always)]
    fn default() -> Cfdcfdcfg {
        <crate::RegValueT<Cfdcfdcfg_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcfdcfg {

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
    pub struct Gwen_SPEC;
    pub type Gwen = crate::EnumBitfieldStruct<u8, Gwen_SPEC>;
    impl Gwen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gwfdf_SPEC;
    pub type Gwfdf = crate::EnumBitfieldStruct<u8, Gwfdf_SPEC>;
    impl Gwfdf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gwbrs_SPEC;
    pub type Gwbrs = crate::EnumBitfieldStruct<u8, Gwbrs_SPEC>;
    impl Gwbrs {
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
pub struct Cfdcfdctr_SPEC;
impl crate::sealed::RegSpec for Cfdcfdctr_SPEC {
    type DataType = u32;
}

pub type Cfdcfdctr = crate::RegValueT<Cfdcfdctr_SPEC>;

impl Cfdcfdctr {
    #[inline(always)]
    pub fn eocclr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdcfdctr::Eocclr,
        cfdcfdctr::Eocclr,
        Cfdcfdctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdcfdctr::Eocclr,
            cfdcfdctr::Eocclr,
            Cfdcfdctr_SPEC,
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
        cfdcfdctr::Socclr,
        cfdcfdctr::Socclr,
        Cfdcfdctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdcfdctr::Socclr,
            cfdcfdctr::Socclr,
            Cfdcfdctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcfdctr {
    #[inline(always)]
    fn default() -> Cfdcfdctr {
        <crate::RegValueT<Cfdcfdctr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcfdctr {

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
pub struct Cfdcfdsts_SPEC;
impl crate::sealed::RegSpec for Cfdcfdsts_SPEC {
    type DataType = u32;
}

pub type Cfdcfdsts = crate::RegValueT<Cfdcfdsts_SPEC>;

impl Cfdcfdsts {
    #[inline(always)]
    pub fn tdcr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdsts_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eoco(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cfdcfdsts::Eoco,
        cfdcfdsts::Eoco,
        Cfdcfdsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cfdcfdsts::Eoco,
            cfdcfdsts::Eoco,
            Cfdcfdsts_SPEC,
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
        cfdcfdsts::Soco,
        cfdcfdsts::Soco,
        Cfdcfdsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            cfdcfdsts::Soco,
            cfdcfdsts::Soco,
            Cfdcfdsts_SPEC,
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
        cfdcfdsts::Tdcvf,
        cfdcfdsts::Tdcvf,
        Cfdcfdsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            cfdcfdsts::Tdcvf,
            cfdcfdsts::Tdcvf,
            Cfdcfdsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn eoc(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdsts_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soc(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfdsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfdsts_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdsts {
    #[inline(always)]
    fn default() -> Cfdcfdsts {
        <crate::RegValueT<Cfdcfdsts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcfdsts {

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
pub struct Cfdcfdcrc_SPEC;
impl crate::sealed::RegSpec for Cfdcfdcrc_SPEC {
    type DataType = u32;
}

pub type Cfdcfdcrc = crate::RegValueT<Cfdcfdcrc_SPEC>;

impl Cfdcfdcrc {
    #[inline(always)]
    pub fn crcreg(
        self,
    ) -> crate::common::RegisterField<0, 0x1fffff, 1, 0, u32, u32, Cfdcfdcrc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1fffff,1,0,u32,u32,Cfdcfdcrc_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn scnt(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, u8, Cfdcfdcrc_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xf,1,0,u8,u8,Cfdcfdcrc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdcrc {
    #[inline(always)]
    fn default() -> Cfdcfdcrc {
        <crate::RegValueT<Cfdcfdcrc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcblct_SPEC;
impl crate::sealed::RegSpec for Cfdcblct_SPEC {
    type DataType = u32;
}

pub type Cfdcblct = crate::RegValueT<Cfdcblct_SPEC>;

impl Cfdcblct {
    #[inline(always)]
    pub fn blce(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdcblct::Blce,
        cfdcblct::Blce,
        Cfdcblct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdcblct::Blce,
            cfdcblct::Blce,
            Cfdcblct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn blcld(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Cfdcblct_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cfdcblct_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Cfdcblct {
    #[inline(always)]
    fn default() -> Cfdcblct {
        <crate::RegValueT<Cfdcblct_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcblct {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Blce_SPEC;
    pub type Blce = crate::EnumBitfieldStruct<u8, Blce_SPEC>;
    impl Blce {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcblsts_SPEC;
impl crate::sealed::RegSpec for Cfdcblsts_SPEC {
    type DataType = u32;
}

pub type Cfdcblsts = crate::RegValueT<Cfdcblsts_SPEC>;

impl Cfdcblsts {
    #[inline(always)]
    pub fn blc(
        self,
    ) -> crate::common::RegisterField<3, 0x1fffffff, 1, 0, u32, u32, Cfdcblsts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            3,
            0x1fffffff,
            1,
            0,
            u32,
            u32,
            Cfdcblsts_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcblsts {
    #[inline(always)]
    fn default() -> Cfdcblsts {
        <crate::RegValueT<Cfdcblsts_SPEC> as RegisterValue<_>>::new(0)
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
    pub fn gaflsrd0(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        cfdgaflp0::Gaflsrd0,
        cfdgaflp0::Gaflsrd0,
        Cfdgaflp0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            cfdgaflp0::Gaflsrd0,
            cfdgaflp0::Gaflsrd0,
            Cfdgaflp0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gaflsrd1(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        cfdgaflp0::Gaflsrd1,
        cfdgaflp0::Gaflsrd1,
        Cfdgaflp0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            cfdgaflp0::Gaflsrd1,
            cfdgaflp0::Gaflsrd1,
            Cfdgaflp0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gaflsrd2(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        cfdgaflp0::Gaflsrd2,
        cfdgaflp0::Gaflsrd2,
        Cfdgaflp0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            cfdgaflp0::Gaflsrd2,
            cfdgaflp0::Gaflsrd2,
            Cfdgaflp0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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
    pub struct Gaflsrd0_SPEC;
    pub type Gaflsrd0 = crate::EnumBitfieldStruct<u8, Gaflsrd0_SPEC>;
    impl Gaflsrd0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gaflsrd1_SPEC;
    pub type Gaflsrd1 = crate::EnumBitfieldStruct<u8, Gaflsrd1_SPEC>;
    impl Gaflsrd1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gaflsrd2_SPEC;
    pub type Gaflsrd2 = crate::EnumBitfieldStruct<u8, Gaflsrd2_SPEC>;
    impl Gaflsrd2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
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
    pub fn gaflfdp(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, u16, Cfdgaflp1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3fff,1,0,u16,u16,Cfdgaflp1_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct Cfdrmid0_SPEC;
impl crate::sealed::RegSpec for Cfdrmid0_SPEC {
    type DataType = u32;
}

pub type Cfdrmid0 = crate::RegValueT<Cfdrmid0_SPEC>;

impl Cfdrmid0 {
    #[inline(always)]
    pub fn rmid(
        self,
    ) -> crate::common::RegisterField<0, 0x1fffffff, 1, 0, u32, u32, Cfdrmid0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1fffffff,1,0,u32,u32,Cfdrmid0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmrtr(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        cfdrmid_0::Rmrtr,
        cfdrmid_0::Rmrtr,
        Cfdrmid0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            cfdrmid_0::Rmrtr,
            cfdrmid_0::Rmrtr,
            Cfdrmid0_SPEC,
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
        cfdrmid_0::Rmide,
        cfdrmid_0::Rmide,
        Cfdrmid0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            cfdrmid_0::Rmide,
            cfdrmid_0::Rmide,
            Cfdrmid0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmid0 {
    #[inline(always)]
    fn default() -> Cfdrmid0 {
        <crate::RegValueT<Cfdrmid0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmid_0 {

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
pub struct Cfdrmptr0_SPEC;
impl crate::sealed::RegSpec for Cfdrmptr0_SPEC {
    type DataType = u32;
}

pub type Cfdrmptr0 = crate::RegValueT<Cfdrmptr0_SPEC>;

impl Cfdrmptr0 {
    #[inline(always)]
    pub fn rmts(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Cfdrmptr0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Cfdrmptr0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdlc(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, u8, Cfdrmptr0_SPEC, crate::common::R> {
        crate::common::RegisterField::<28,0xf,1,0,u8,u8,Cfdrmptr0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmptr0 {
    #[inline(always)]
    fn default() -> Cfdrmptr0 {
        <crate::RegValueT<Cfdrmptr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmfdsts0_SPEC;
impl crate::sealed::RegSpec for Cfdrmfdsts0_SPEC {
    type DataType = u32;
}

pub type Cfdrmfdsts0 = crate::RegValueT<Cfdrmfdsts0_SPEC>;

impl Cfdrmfdsts0 {
    #[inline(always)]
    pub fn rmesi(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdrmfdsts_0::Rmesi,
        cfdrmfdsts_0::Rmesi,
        Cfdrmfdsts0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdrmfdsts_0::Rmesi,
            cfdrmfdsts_0::Rmesi,
            Cfdrmfdsts0_SPEC,
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
        cfdrmfdsts_0::Rmbrs,
        cfdrmfdsts_0::Rmbrs,
        Cfdrmfdsts0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdrmfdsts_0::Rmbrs,
            cfdrmfdsts_0::Rmbrs,
            Cfdrmfdsts0_SPEC,
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
        cfdrmfdsts_0::Rmfdf,
        cfdrmfdsts_0::Rmfdf,
        Cfdrmfdsts0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdrmfdsts_0::Rmfdf,
            cfdrmfdsts_0::Rmfdf,
            Cfdrmfdsts0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmifl(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, Cfdrmfdsts0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,Cfdrmfdsts0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmptr(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Cfdrmfdsts0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Cfdrmfdsts0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmfdsts0 {
    #[inline(always)]
    fn default() -> Cfdrmfdsts0 {
        <crate::RegValueT<Cfdrmfdsts0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmfdsts_0 {

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
pub struct Cfdrmdf00_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf00_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf00 = crate::RegValueT<Cfdrmdf00_SPEC>;

impl Cfdrmdf00 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf00_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf00_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf00_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf00_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf00_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf00_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf00_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf00_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf00 {
    #[inline(always)]
    fn default() -> Cfdrmdf00 {
        <crate::RegValueT<Cfdrmdf00_SPEC> as RegisterValue<_>>::new(0)
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
pub struct Cfdrmdf20_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf20_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf20 = crate::RegValueT<Cfdrmdf20_SPEC>;

impl Cfdrmdf20 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf20_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf20_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf20_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf20_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf20_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf20_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf20_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf20_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf20 {
    #[inline(always)]
    fn default() -> Cfdrmdf20 {
        <crate::RegValueT<Cfdrmdf20_SPEC> as RegisterValue<_>>::new(0)
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
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf30_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf30_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf30_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf30_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf30_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf30_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf30_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf30_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf30 {
    #[inline(always)]
    fn default() -> Cfdrmdf30 {
        <crate::RegValueT<Cfdrmdf30_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf40_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf40_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf40 = crate::RegValueT<Cfdrmdf40_SPEC>;

impl Cfdrmdf40 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf40_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf40_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf40_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf40_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf40_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf40_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf40_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf40_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf40 {
    #[inline(always)]
    fn default() -> Cfdrmdf40 {
        <crate::RegValueT<Cfdrmdf40_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf50_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf50_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf50 = crate::RegValueT<Cfdrmdf50_SPEC>;

impl Cfdrmdf50 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf50_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf50_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf50_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf50_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf50_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf50_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf50_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf50_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf50 {
    #[inline(always)]
    fn default() -> Cfdrmdf50 {
        <crate::RegValueT<Cfdrmdf50_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf60_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf60_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf60 = crate::RegValueT<Cfdrmdf60_SPEC>;

impl Cfdrmdf60 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf60_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf60_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf60_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf60_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf60_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf60_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf60_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf60_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf60 {
    #[inline(always)]
    fn default() -> Cfdrmdf60 {
        <crate::RegValueT<Cfdrmdf60_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf70_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf70_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf70 = crate::RegValueT<Cfdrmdf70_SPEC>;

impl Cfdrmdf70 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf70_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf70_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf70_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf70_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf70_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf70_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf70_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf70_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf70 {
    #[inline(always)]
    fn default() -> Cfdrmdf70 {
        <crate::RegValueT<Cfdrmdf70_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf80_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf80_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf80 = crate::RegValueT<Cfdrmdf80_SPEC>;

impl Cfdrmdf80 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf80_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf80_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf80_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf80_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf80_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf80_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf80_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf80_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf80 {
    #[inline(always)]
    fn default() -> Cfdrmdf80 {
        <crate::RegValueT<Cfdrmdf80_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf90_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf90_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf90 = crate::RegValueT<Cfdrmdf90_SPEC>;

impl Cfdrmdf90 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf90_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf90_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf90_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf90_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf90_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf90_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf90_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf90_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf90 {
    #[inline(always)]
    fn default() -> Cfdrmdf90 {
        <crate::RegValueT<Cfdrmdf90_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf100_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf100_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf100 = crate::RegValueT<Cfdrmdf100_SPEC>;

impl Cfdrmdf100 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf100_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf100_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf100_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf100_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf100_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf100_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf100_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf100_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf100 {
    #[inline(always)]
    fn default() -> Cfdrmdf100 {
        <crate::RegValueT<Cfdrmdf100_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf110_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf110_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf110 = crate::RegValueT<Cfdrmdf110_SPEC>;

impl Cfdrmdf110 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf110_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf110_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf110_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf110_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf110_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf110_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf110_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf110_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf110 {
    #[inline(always)]
    fn default() -> Cfdrmdf110 {
        <crate::RegValueT<Cfdrmdf110_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf120_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf120_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf120 = crate::RegValueT<Cfdrmdf120_SPEC>;

impl Cfdrmdf120 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf120_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf120_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf120_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf120_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf120_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf120_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf120_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf120_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf120 {
    #[inline(always)]
    fn default() -> Cfdrmdf120 {
        <crate::RegValueT<Cfdrmdf120_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf130_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf130_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf130 = crate::RegValueT<Cfdrmdf130_SPEC>;

impl Cfdrmdf130 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf130_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf130_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf130_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf130_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf130_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf130_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf130_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf130_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf130 {
    #[inline(always)]
    fn default() -> Cfdrmdf130 {
        <crate::RegValueT<Cfdrmdf130_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf140_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf140_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf140 = crate::RegValueT<Cfdrmdf140_SPEC>;

impl Cfdrmdf140 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf140_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf140_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf140_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf140_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf140_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf140_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf140_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf140_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf140 {
    #[inline(always)]
    fn default() -> Cfdrmdf140 {
        <crate::RegValueT<Cfdrmdf140_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf150_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf150_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf150 = crate::RegValueT<Cfdrmdf150_SPEC>;

impl Cfdrmdf150 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf150_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf150_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf150_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf150_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf150_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf150_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf150_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf150_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf150 {
    #[inline(always)]
    fn default() -> Cfdrmdf150 {
        <crate::RegValueT<Cfdrmdf150_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmid1_SPEC;
impl crate::sealed::RegSpec for Cfdrmid1_SPEC {
    type DataType = u32;
}

pub type Cfdrmid1 = crate::RegValueT<Cfdrmid1_SPEC>;

impl Cfdrmid1 {
    #[inline(always)]
    pub fn rmid(
        self,
    ) -> crate::common::RegisterField<0, 0x1fffffff, 1, 0, u32, u32, Cfdrmid1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1fffffff,1,0,u32,u32,Cfdrmid1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmrtr(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        cfdrmid_1::Rmrtr,
        cfdrmid_1::Rmrtr,
        Cfdrmid1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            cfdrmid_1::Rmrtr,
            cfdrmid_1::Rmrtr,
            Cfdrmid1_SPEC,
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
        cfdrmid_1::Rmide,
        cfdrmid_1::Rmide,
        Cfdrmid1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            cfdrmid_1::Rmide,
            cfdrmid_1::Rmide,
            Cfdrmid1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdrmid1 {
    #[inline(always)]
    fn default() -> Cfdrmid1 {
        <crate::RegValueT<Cfdrmid1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmid_1 {

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
pub struct Cfdrmptr1_SPEC;
impl crate::sealed::RegSpec for Cfdrmptr1_SPEC {
    type DataType = u32;
}

pub type Cfdrmptr1 = crate::RegValueT<Cfdrmptr1_SPEC>;

impl Cfdrmptr1 {
    #[inline(always)]
    pub fn rmts(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Cfdrmptr1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Cfdrmptr1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdlc(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, u8, Cfdrmptr1_SPEC, crate::common::R> {
        crate::common::RegisterField::<28,0xf,1,0,u8,u8,Cfdrmptr1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmptr1 {
    #[inline(always)]
    fn default() -> Cfdrmptr1 {
        <crate::RegValueT<Cfdrmptr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmfdsts1_SPEC;
impl crate::sealed::RegSpec for Cfdrmfdsts1_SPEC {
    type DataType = u32;
}

pub type Cfdrmfdsts1 = crate::RegValueT<Cfdrmfdsts1_SPEC>;

impl Cfdrmfdsts1 {
    #[inline(always)]
    pub fn rmesi(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdrmfdsts_1::Rmesi,
        cfdrmfdsts_1::Rmesi,
        Cfdrmfdsts1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdrmfdsts_1::Rmesi,
            cfdrmfdsts_1::Rmesi,
            Cfdrmfdsts1_SPEC,
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
        cfdrmfdsts_1::Rmbrs,
        cfdrmfdsts_1::Rmbrs,
        Cfdrmfdsts1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdrmfdsts_1::Rmbrs,
            cfdrmfdsts_1::Rmbrs,
            Cfdrmfdsts1_SPEC,
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
        cfdrmfdsts_1::Rmfdf,
        cfdrmfdsts_1::Rmfdf,
        Cfdrmfdsts1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdrmfdsts_1::Rmfdf,
            cfdrmfdsts_1::Rmfdf,
            Cfdrmfdsts1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rmifl(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, Cfdrmfdsts1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,Cfdrmfdsts1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmptr(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Cfdrmfdsts1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Cfdrmfdsts1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmfdsts1 {
    #[inline(always)]
    fn default() -> Cfdrmfdsts1 {
        <crate::RegValueT<Cfdrmfdsts1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdrmfdsts_1 {

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
pub struct Cfdrmdf01_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf01_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf01 = crate::RegValueT<Cfdrmdf01_SPEC>;

impl Cfdrmdf01 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf01_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf01_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf01_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf01_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf01_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf01_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf01_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf01_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf01 {
    #[inline(always)]
    fn default() -> Cfdrmdf01 {
        <crate::RegValueT<Cfdrmdf01_SPEC> as RegisterValue<_>>::new(0)
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
pub struct Cfdrmdf21_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf21_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf21 = crate::RegValueT<Cfdrmdf21_SPEC>;

impl Cfdrmdf21 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf21_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf21_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf21_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf21_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf21_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf21_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf21_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf21_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf21 {
    #[inline(always)]
    fn default() -> Cfdrmdf21 {
        <crate::RegValueT<Cfdrmdf21_SPEC> as RegisterValue<_>>::new(0)
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
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf31_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf31_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf31_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf31_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf31_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf31_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf31_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf31_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf31 {
    #[inline(always)]
    fn default() -> Cfdrmdf31 {
        <crate::RegValueT<Cfdrmdf31_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf41_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf41_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf41 = crate::RegValueT<Cfdrmdf41_SPEC>;

impl Cfdrmdf41 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf41_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf41_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf41_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf41_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf41_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf41_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf41_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf41_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf41 {
    #[inline(always)]
    fn default() -> Cfdrmdf41 {
        <crate::RegValueT<Cfdrmdf41_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf51_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf51_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf51 = crate::RegValueT<Cfdrmdf51_SPEC>;

impl Cfdrmdf51 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf51_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf51_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf51_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf51_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf51_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf51_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf51_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf51_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf51 {
    #[inline(always)]
    fn default() -> Cfdrmdf51 {
        <crate::RegValueT<Cfdrmdf51_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf61_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf61_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf61 = crate::RegValueT<Cfdrmdf61_SPEC>;

impl Cfdrmdf61 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf61_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf61_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf61_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf61_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf61_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf61_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf61_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf61_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf61 {
    #[inline(always)]
    fn default() -> Cfdrmdf61 {
        <crate::RegValueT<Cfdrmdf61_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf71_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf71_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf71 = crate::RegValueT<Cfdrmdf71_SPEC>;

impl Cfdrmdf71 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf71_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf71_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf71_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf71_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf71_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf71_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf71_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf71_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf71 {
    #[inline(always)]
    fn default() -> Cfdrmdf71 {
        <crate::RegValueT<Cfdrmdf71_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf81_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf81_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf81 = crate::RegValueT<Cfdrmdf81_SPEC>;

impl Cfdrmdf81 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf81_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf81_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf81_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf81_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf81_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf81_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf81_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf81_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf81 {
    #[inline(always)]
    fn default() -> Cfdrmdf81 {
        <crate::RegValueT<Cfdrmdf81_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf91_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf91_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf91 = crate::RegValueT<Cfdrmdf91_SPEC>;

impl Cfdrmdf91 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf91_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf91_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf91_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf91_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf91_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf91_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf91_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf91_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf91 {
    #[inline(always)]
    fn default() -> Cfdrmdf91 {
        <crate::RegValueT<Cfdrmdf91_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf101_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf101_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf101 = crate::RegValueT<Cfdrmdf101_SPEC>;

impl Cfdrmdf101 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf101_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf101_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf101_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf101_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf101_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf101_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf101_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf101_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf101 {
    #[inline(always)]
    fn default() -> Cfdrmdf101 {
        <crate::RegValueT<Cfdrmdf101_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf111_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf111_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf111 = crate::RegValueT<Cfdrmdf111_SPEC>;

impl Cfdrmdf111 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf111_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf111_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf111_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf111_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf111_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf111_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf111_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf111_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf111 {
    #[inline(always)]
    fn default() -> Cfdrmdf111 {
        <crate::RegValueT<Cfdrmdf111_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf121_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf121_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf121 = crate::RegValueT<Cfdrmdf121_SPEC>;

impl Cfdrmdf121 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf121_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf121_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf121_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf121_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf121_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf121_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf121_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf121_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf121 {
    #[inline(always)]
    fn default() -> Cfdrmdf121 {
        <crate::RegValueT<Cfdrmdf121_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf131_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf131_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf131 = crate::RegValueT<Cfdrmdf131_SPEC>;

impl Cfdrmdf131 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf131_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf131_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf131_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf131_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf131_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf131_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf131_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf131_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf131 {
    #[inline(always)]
    fn default() -> Cfdrmdf131 {
        <crate::RegValueT<Cfdrmdf131_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf141_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf141_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf141 = crate::RegValueT<Cfdrmdf141_SPEC>;

impl Cfdrmdf141 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf141_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf141_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf141_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf141_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf141_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf141_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf141_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf141_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf141 {
    #[inline(always)]
    fn default() -> Cfdrmdf141 {
        <crate::RegValueT<Cfdrmdf141_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdrmdf151_SPEC;
impl crate::sealed::RegSpec for Cfdrmdf151_SPEC {
    type DataType = u32;
}

pub type Cfdrmdf151 = crate::RegValueT<Cfdrmdf151_SPEC>;

impl Cfdrmdf151 {
    #[inline(always)]
    pub fn rmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdrmdf151_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdrmdf151_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdrmdf151_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdrmdf151_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdrmdf151_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdrmdf151_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdrmdf151_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdrmdf151_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdrmdf151 {
    #[inline(always)]
    fn default() -> Cfdrmdf151 {
        <crate::RegValueT<Cfdrmdf151_SPEC> as RegisterValue<_>>::new(0)
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
pub struct Cfdcfid0_SPEC;
impl crate::sealed::RegSpec for Cfdcfid0_SPEC {
    type DataType = u32;
}

pub type Cfdcfid0 = crate::RegValueT<Cfdcfid0_SPEC>;

impl Cfdcfid0 {
    #[inline(always)]
    pub fn cfid(
        self,
    ) -> crate::common::RegisterField<0, 0x1fffffff, 1, 0, u32, u32, Cfdcfid0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1fffffff,
            1,
            0,
            u32,
            u32,
            Cfdcfid0_SPEC,
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
        cfdcfid_0::Thlen,
        cfdcfid_0::Thlen,
        Cfdcfid0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            cfdcfid_0::Thlen,
            cfdcfid_0::Thlen,
            Cfdcfid0_SPEC,
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
        cfdcfid_0::Cfrtr,
        cfdcfid_0::Cfrtr,
        Cfdcfid0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            cfdcfid_0::Cfrtr,
            cfdcfid_0::Cfrtr,
            Cfdcfid0_SPEC,
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
        cfdcfid_0::Cfide,
        cfdcfid_0::Cfide,
        Cfdcfid0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            cfdcfid_0::Cfide,
            cfdcfid_0::Cfide,
            Cfdcfid0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcfid0 {
    #[inline(always)]
    fn default() -> Cfdcfid0 {
        <crate::RegValueT<Cfdcfid0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcfid_0 {

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
pub struct Cfdcfptr0_SPEC;
impl crate::sealed::RegSpec for Cfdcfptr0_SPEC {
    type DataType = u32;
}

pub type Cfdcfptr0 = crate::RegValueT<Cfdcfptr0_SPEC>;

impl Cfdcfptr0 {
    #[inline(always)]
    pub fn cfts(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Cfdcfptr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Cfdcfptr0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdlc(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, u8, Cfdcfptr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0xf,1,0,u8,u8,Cfdcfptr0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfptr0 {
    #[inline(always)]
    fn default() -> Cfdcfptr0 {
        <crate::RegValueT<Cfdcfptr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcffdcsts0_SPEC;
impl crate::sealed::RegSpec for Cfdcffdcsts0_SPEC {
    type DataType = u32;
}

pub type Cfdcffdcsts0 = crate::RegValueT<Cfdcffdcsts0_SPEC>;

impl Cfdcffdcsts0 {
    #[inline(always)]
    pub fn cfesi(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdcffdcsts_0::Cfesi,
        cfdcffdcsts_0::Cfesi,
        Cfdcffdcsts0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdcffdcsts_0::Cfesi,
            cfdcffdcsts_0::Cfesi,
            Cfdcffdcsts0_SPEC,
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
        cfdcffdcsts_0::Cfbrs,
        cfdcffdcsts_0::Cfbrs,
        Cfdcffdcsts0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdcffdcsts_0::Cfbrs,
            cfdcffdcsts_0::Cfbrs,
            Cfdcffdcsts0_SPEC,
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
        cfdcffdcsts_0::Cffdf,
        cfdcffdcsts_0::Cffdf,
        Cfdcffdcsts0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdcffdcsts_0::Cffdf,
            cfdcffdcsts_0::Cffdf,
            Cfdcffdcsts0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfifl(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, Cfdcffdcsts0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,Cfdcffdcsts0_SPEC,crate::common::RW>::from_register(self,0)
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
        Cfdcffdcsts0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            Cfdcffdcsts0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcffdcsts0 {
    #[inline(always)]
    fn default() -> Cfdcffdcsts0 {
        <crate::RegValueT<Cfdcffdcsts0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcffdcsts_0 {

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
pub struct Cfdcfdf00_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf00_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf00 = crate::RegValueT<Cfdcfdf00_SPEC>;

impl Cfdcfdf00 {
    #[inline(always)]
    pub fn cfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf00_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdcfdf00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdcfdf00_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdf00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdf00_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfdf00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfdf00_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdf00 {
    #[inline(always)]
    fn default() -> Cfdcfdf00 {
        <crate::RegValueT<Cfdcfdf00_SPEC> as RegisterValue<_>>::new(0)
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
    pub fn cfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf10_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdcfdf10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdcfdf10_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdf10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdf10_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfdf10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfdf10_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdf10 {
    #[inline(always)]
    fn default() -> Cfdcfdf10 {
        <crate::RegValueT<Cfdcfdf10_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf20_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf20_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf20 = crate::RegValueT<Cfdcfdf20_SPEC>;

impl Cfdcfdf20 {
    #[inline(always)]
    pub fn cfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf20_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf20_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdcfdf20_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdcfdf20_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdf20_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdf20_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfdf20_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfdf20_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdf20 {
    #[inline(always)]
    fn default() -> Cfdcfdf20 {
        <crate::RegValueT<Cfdcfdf20_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf30_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf30_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf30 = crate::RegValueT<Cfdcfdf30_SPEC>;

impl Cfdcfdf30 {
    #[inline(always)]
    pub fn cfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf30_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf30_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdcfdf30_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdcfdf30_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdf30_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdf30_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfdf30_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfdf30_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdf30 {
    #[inline(always)]
    fn default() -> Cfdcfdf30 {
        <crate::RegValueT<Cfdcfdf30_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf40_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf40_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf40 = crate::RegValueT<Cfdcfdf40_SPEC>;

impl Cfdcfdf40 {
    #[inline(always)]
    pub fn cfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf40_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf40_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdcfdf40_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdcfdf40_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdf40_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdf40_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfdf40_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfdf40_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdf40 {
    #[inline(always)]
    fn default() -> Cfdcfdf40 {
        <crate::RegValueT<Cfdcfdf40_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf50_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf50_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf50 = crate::RegValueT<Cfdcfdf50_SPEC>;

impl Cfdcfdf50 {
    #[inline(always)]
    pub fn cfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf50_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf50_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdcfdf50_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdcfdf50_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdf50_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdf50_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfdf50_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfdf50_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdf50 {
    #[inline(always)]
    fn default() -> Cfdcfdf50 {
        <crate::RegValueT<Cfdcfdf50_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf60_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf60_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf60 = crate::RegValueT<Cfdcfdf60_SPEC>;

impl Cfdcfdf60 {
    #[inline(always)]
    pub fn cfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf60_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf60_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdcfdf60_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdcfdf60_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdf60_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdf60_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfdf60_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfdf60_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdf60 {
    #[inline(always)]
    fn default() -> Cfdcfdf60 {
        <crate::RegValueT<Cfdcfdf60_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf70_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf70_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf70 = crate::RegValueT<Cfdcfdf70_SPEC>;

impl Cfdcfdf70 {
    #[inline(always)]
    pub fn cfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf70_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf70_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdcfdf70_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdcfdf70_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdf70_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdf70_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfdf70_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfdf70_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdf70 {
    #[inline(always)]
    fn default() -> Cfdcfdf70 {
        <crate::RegValueT<Cfdcfdf70_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf80_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf80_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf80 = crate::RegValueT<Cfdcfdf80_SPEC>;

impl Cfdcfdf80 {
    #[inline(always)]
    pub fn cfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf80_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf80_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdcfdf80_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdcfdf80_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdf80_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdf80_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfdf80_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfdf80_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdf80 {
    #[inline(always)]
    fn default() -> Cfdcfdf80 {
        <crate::RegValueT<Cfdcfdf80_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf90_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf90_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf90 = crate::RegValueT<Cfdcfdf90_SPEC>;

impl Cfdcfdf90 {
    #[inline(always)]
    pub fn cfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf90_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf90_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdcfdf90_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdcfdf90_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdf90_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdf90_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfdf90_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfdf90_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdf90 {
    #[inline(always)]
    fn default() -> Cfdcfdf90 {
        <crate::RegValueT<Cfdcfdf90_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf100_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf100_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf100 = crate::RegValueT<Cfdcfdf100_SPEC>;

impl Cfdcfdf100 {
    #[inline(always)]
    pub fn cfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf100_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf100_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdcfdf100_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdcfdf100_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdf100_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdf100_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfdf100_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfdf100_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdf100 {
    #[inline(always)]
    fn default() -> Cfdcfdf100 {
        <crate::RegValueT<Cfdcfdf100_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf110_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf110_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf110 = crate::RegValueT<Cfdcfdf110_SPEC>;

impl Cfdcfdf110 {
    #[inline(always)]
    pub fn cfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf110_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf110_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdcfdf110_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdcfdf110_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdf110_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdf110_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfdf110_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfdf110_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdf110 {
    #[inline(always)]
    fn default() -> Cfdcfdf110 {
        <crate::RegValueT<Cfdcfdf110_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf120_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf120_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf120 = crate::RegValueT<Cfdcfdf120_SPEC>;

impl Cfdcfdf120 {
    #[inline(always)]
    pub fn cfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf120_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf120_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdcfdf120_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdcfdf120_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdf120_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdf120_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfdf120_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfdf120_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdf120 {
    #[inline(always)]
    fn default() -> Cfdcfdf120 {
        <crate::RegValueT<Cfdcfdf120_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf130_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf130_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf130 = crate::RegValueT<Cfdcfdf130_SPEC>;

impl Cfdcfdf130 {
    #[inline(always)]
    pub fn cfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf130_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf130_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdcfdf130_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdcfdf130_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdf130_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdf130_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfdf130_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfdf130_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdf130 {
    #[inline(always)]
    fn default() -> Cfdcfdf130 {
        <crate::RegValueT<Cfdcfdf130_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf140_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf140_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf140 = crate::RegValueT<Cfdcfdf140_SPEC>;

impl Cfdcfdf140 {
    #[inline(always)]
    pub fn cfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf140_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf140_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdcfdf140_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdcfdf140_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdf140_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdf140_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfdf140_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfdf140_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdf140 {
    #[inline(always)]
    fn default() -> Cfdcfdf140 {
        <crate::RegValueT<Cfdcfdf140_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf150_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf150_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf150 = crate::RegValueT<Cfdcfdf150_SPEC>;

impl Cfdcfdf150 {
    #[inline(always)]
    pub fn cfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf150_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf150_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdcfdf150_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdcfdf150_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdf150_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdf150_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfdf150_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfdf150_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdf150 {
    #[inline(always)]
    fn default() -> Cfdcfdf150 {
        <crate::RegValueT<Cfdcfdf150_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfid1_SPEC;
impl crate::sealed::RegSpec for Cfdcfid1_SPEC {
    type DataType = u32;
}

pub type Cfdcfid1 = crate::RegValueT<Cfdcfid1_SPEC>;

impl Cfdcfid1 {
    #[inline(always)]
    pub fn cfid(
        self,
    ) -> crate::common::RegisterField<0, 0x1fffffff, 1, 0, u32, u32, Cfdcfid1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1fffffff,
            1,
            0,
            u32,
            u32,
            Cfdcfid1_SPEC,
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
        cfdcfid_1::Thlen,
        cfdcfid_1::Thlen,
        Cfdcfid1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            cfdcfid_1::Thlen,
            cfdcfid_1::Thlen,
            Cfdcfid1_SPEC,
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
        cfdcfid_1::Cfrtr,
        cfdcfid_1::Cfrtr,
        Cfdcfid1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            cfdcfid_1::Cfrtr,
            cfdcfid_1::Cfrtr,
            Cfdcfid1_SPEC,
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
        cfdcfid_1::Cfide,
        cfdcfid_1::Cfide,
        Cfdcfid1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            cfdcfid_1::Cfide,
            cfdcfid_1::Cfide,
            Cfdcfid1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcfid1 {
    #[inline(always)]
    fn default() -> Cfdcfid1 {
        <crate::RegValueT<Cfdcfid1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcfid_1 {

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
pub struct Cfdcfptr1_SPEC;
impl crate::sealed::RegSpec for Cfdcfptr1_SPEC {
    type DataType = u32;
}

pub type Cfdcfptr1 = crate::RegValueT<Cfdcfptr1_SPEC>;

impl Cfdcfptr1 {
    #[inline(always)]
    pub fn cfts(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Cfdcfptr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Cfdcfptr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdlc(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, u8, Cfdcfptr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0xf,1,0,u8,u8,Cfdcfptr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfptr1 {
    #[inline(always)]
    fn default() -> Cfdcfptr1 {
        <crate::RegValueT<Cfdcfptr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcffdcsts1_SPEC;
impl crate::sealed::RegSpec for Cfdcffdcsts1_SPEC {
    type DataType = u32;
}

pub type Cfdcffdcsts1 = crate::RegValueT<Cfdcffdcsts1_SPEC>;

impl Cfdcffdcsts1 {
    #[inline(always)]
    pub fn cfesi(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdcffdcsts_1::Cfesi,
        cfdcffdcsts_1::Cfesi,
        Cfdcffdcsts1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdcffdcsts_1::Cfesi,
            cfdcffdcsts_1::Cfesi,
            Cfdcffdcsts1_SPEC,
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
        cfdcffdcsts_1::Cfbrs,
        cfdcffdcsts_1::Cfbrs,
        Cfdcffdcsts1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdcffdcsts_1::Cfbrs,
            cfdcffdcsts_1::Cfbrs,
            Cfdcffdcsts1_SPEC,
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
        cfdcffdcsts_1::Cffdf,
        cfdcffdcsts_1::Cffdf,
        Cfdcffdcsts1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdcffdcsts_1::Cffdf,
            cfdcffdcsts_1::Cffdf,
            Cfdcffdcsts1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfifl(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, Cfdcffdcsts1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,Cfdcffdcsts1_SPEC,crate::common::RW>::from_register(self,0)
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
        Cfdcffdcsts1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            Cfdcffdcsts1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdcffdcsts1 {
    #[inline(always)]
    fn default() -> Cfdcffdcsts1 {
        <crate::RegValueT<Cfdcffdcsts1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdcffdcsts_1 {

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
pub struct Cfdcfdf01_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf01_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf01 = crate::RegValueT<Cfdcfdf01_SPEC>;

impl Cfdcfdf01 {
    #[inline(always)]
    pub fn cfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf01_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf01_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdcfdf01_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdcfdf01_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdf01_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdf01_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfdf01_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfdf01_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdf01 {
    #[inline(always)]
    fn default() -> Cfdcfdf01 {
        <crate::RegValueT<Cfdcfdf01_SPEC> as RegisterValue<_>>::new(0)
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
    pub fn cfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf11_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf11_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdcfdf11_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdcfdf11_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdf11_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdf11_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfdf11_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfdf11_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdf11 {
    #[inline(always)]
    fn default() -> Cfdcfdf11 {
        <crate::RegValueT<Cfdcfdf11_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf21_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf21_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf21 = crate::RegValueT<Cfdcfdf21_SPEC>;

impl Cfdcfdf21 {
    #[inline(always)]
    pub fn cfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf21_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf21_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdcfdf21_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdcfdf21_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdf21_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdf21_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfdf21_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfdf21_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdf21 {
    #[inline(always)]
    fn default() -> Cfdcfdf21 {
        <crate::RegValueT<Cfdcfdf21_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf31_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf31_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf31 = crate::RegValueT<Cfdcfdf31_SPEC>;

impl Cfdcfdf31 {
    #[inline(always)]
    pub fn cfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf31_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf31_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdcfdf31_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdcfdf31_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdf31_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdf31_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfdf31_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfdf31_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdf31 {
    #[inline(always)]
    fn default() -> Cfdcfdf31 {
        <crate::RegValueT<Cfdcfdf31_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf41_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf41_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf41 = crate::RegValueT<Cfdcfdf41_SPEC>;

impl Cfdcfdf41 {
    #[inline(always)]
    pub fn cfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf41_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf41_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdcfdf41_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdcfdf41_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdf41_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdf41_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfdf41_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfdf41_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdf41 {
    #[inline(always)]
    fn default() -> Cfdcfdf41 {
        <crate::RegValueT<Cfdcfdf41_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf51_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf51_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf51 = crate::RegValueT<Cfdcfdf51_SPEC>;

impl Cfdcfdf51 {
    #[inline(always)]
    pub fn cfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf51_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf51_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdcfdf51_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdcfdf51_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdf51_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdf51_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfdf51_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfdf51_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdf51 {
    #[inline(always)]
    fn default() -> Cfdcfdf51 {
        <crate::RegValueT<Cfdcfdf51_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf61_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf61_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf61 = crate::RegValueT<Cfdcfdf61_SPEC>;

impl Cfdcfdf61 {
    #[inline(always)]
    pub fn cfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf61_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf61_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdcfdf61_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdcfdf61_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdf61_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdf61_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfdf61_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfdf61_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdf61 {
    #[inline(always)]
    fn default() -> Cfdcfdf61 {
        <crate::RegValueT<Cfdcfdf61_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf71_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf71_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf71 = crate::RegValueT<Cfdcfdf71_SPEC>;

impl Cfdcfdf71 {
    #[inline(always)]
    pub fn cfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf71_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf71_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdcfdf71_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdcfdf71_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdf71_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdf71_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfdf71_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfdf71_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdf71 {
    #[inline(always)]
    fn default() -> Cfdcfdf71 {
        <crate::RegValueT<Cfdcfdf71_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf81_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf81_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf81 = crate::RegValueT<Cfdcfdf81_SPEC>;

impl Cfdcfdf81 {
    #[inline(always)]
    pub fn cfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf81_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf81_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdcfdf81_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdcfdf81_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdf81_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdf81_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfdf81_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfdf81_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdf81 {
    #[inline(always)]
    fn default() -> Cfdcfdf81 {
        <crate::RegValueT<Cfdcfdf81_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf91_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf91_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf91 = crate::RegValueT<Cfdcfdf91_SPEC>;

impl Cfdcfdf91 {
    #[inline(always)]
    pub fn cfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf91_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf91_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdcfdf91_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdcfdf91_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdf91_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdf91_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfdf91_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfdf91_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdf91 {
    #[inline(always)]
    fn default() -> Cfdcfdf91 {
        <crate::RegValueT<Cfdcfdf91_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf101_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf101_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf101 = crate::RegValueT<Cfdcfdf101_SPEC>;

impl Cfdcfdf101 {
    #[inline(always)]
    pub fn cfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf101_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf101_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdcfdf101_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdcfdf101_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdf101_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdf101_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfdf101_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfdf101_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdf101 {
    #[inline(always)]
    fn default() -> Cfdcfdf101 {
        <crate::RegValueT<Cfdcfdf101_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf111_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf111_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf111 = crate::RegValueT<Cfdcfdf111_SPEC>;

impl Cfdcfdf111 {
    #[inline(always)]
    pub fn cfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf111_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf111_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdcfdf111_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdcfdf111_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdf111_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdf111_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfdf111_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfdf111_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdf111 {
    #[inline(always)]
    fn default() -> Cfdcfdf111 {
        <crate::RegValueT<Cfdcfdf111_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf121_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf121_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf121 = crate::RegValueT<Cfdcfdf121_SPEC>;

impl Cfdcfdf121 {
    #[inline(always)]
    pub fn cfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf121_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf121_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdcfdf121_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdcfdf121_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdf121_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdf121_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfdf121_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfdf121_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdf121 {
    #[inline(always)]
    fn default() -> Cfdcfdf121 {
        <crate::RegValueT<Cfdcfdf121_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf131_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf131_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf131 = crate::RegValueT<Cfdcfdf131_SPEC>;

impl Cfdcfdf131 {
    #[inline(always)]
    pub fn cfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf131_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf131_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdcfdf131_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdcfdf131_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdf131_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdf131_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfdf131_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfdf131_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdf131 {
    #[inline(always)]
    fn default() -> Cfdcfdf131 {
        <crate::RegValueT<Cfdcfdf131_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf141_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf141_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf141 = crate::RegValueT<Cfdcfdf141_SPEC>;

impl Cfdcfdf141 {
    #[inline(always)]
    pub fn cfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf141_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf141_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdcfdf141_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdcfdf141_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdf141_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdf141_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfdf141_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfdf141_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdf141 {
    #[inline(always)]
    fn default() -> Cfdcfdf141 {
        <crate::RegValueT<Cfdcfdf141_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdcfdf151_SPEC;
impl crate::sealed::RegSpec for Cfdcfdf151_SPEC {
    type DataType = u32;
}

pub type Cfdcfdf151 = crate::RegValueT<Cfdcfdf151_SPEC>;

impl Cfdcfdf151 {
    #[inline(always)]
    pub fn cfdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdcfdf151_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdcfdf151_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdcfdf151_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdcfdf151_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdcfdf151_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdcfdf151_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cfdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdcfdf151_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdcfdf151_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdcfdf151 {
    #[inline(always)]
    fn default() -> Cfdcfdf151 {
        <crate::RegValueT<Cfdcfdf151_SPEC> as RegisterValue<_>>::new(0)
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
    ) -> crate::common::RegisterField<3, 0x7f, 1, 0, u8, u8, Cfdthlacc0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<3,0x7f,1,0,u8,u8,Cfdthlacc0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tgw(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        cfdthlacc0::Tgw,
        cfdthlacc0::Tgw,
        Cfdthlacc0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            cfdthlacc0::Tgw,
            cfdthlacc0::Tgw,
            Cfdthlacc0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
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
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tgw_SPEC;
    pub type Tgw = crate::EnumBitfieldStruct<u8, Tgw_SPEC>;
    impl Tgw {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
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
pub struct Cfdtmid0_SPEC;
impl crate::sealed::RegSpec for Cfdtmid0_SPEC {
    type DataType = u32;
}

pub type Cfdtmid0 = crate::RegValueT<Cfdtmid0_SPEC>;

impl Cfdtmid0 {
    #[inline(always)]
    pub fn tmid(
        self,
    ) -> crate::common::RegisterField<0, 0x1fffffff, 1, 0, u32, u32, Cfdtmid0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1fffffff,
            1,
            0,
            u32,
            u32,
            Cfdtmid0_SPEC,
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
        cfdtmid_0::Thlen,
        cfdtmid_0::Thlen,
        Cfdtmid0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            cfdtmid_0::Thlen,
            cfdtmid_0::Thlen,
            Cfdtmid0_SPEC,
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
        cfdtmid_0::Tmrtr,
        cfdtmid_0::Tmrtr,
        Cfdtmid0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            cfdtmid_0::Tmrtr,
            cfdtmid_0::Tmrtr,
            Cfdtmid0_SPEC,
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
        cfdtmid_0::Tmide,
        cfdtmid_0::Tmide,
        Cfdtmid0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            cfdtmid_0::Tmide,
            cfdtmid_0::Tmide,
            Cfdtmid0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdtmid0 {
    #[inline(always)]
    fn default() -> Cfdtmid0 {
        <crate::RegValueT<Cfdtmid0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdtmid_0 {

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
pub struct Cfdtmptr0_SPEC;
impl crate::sealed::RegSpec for Cfdtmptr0_SPEC {
    type DataType = u32;
}

pub type Cfdtmptr0 = crate::RegValueT<Cfdtmptr0_SPEC>;

impl Cfdtmptr0 {
    #[inline(always)]
    pub fn tmdlc(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, u8, Cfdtmptr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0xf,1,0,u8,u8,Cfdtmptr0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmptr0 {
    #[inline(always)]
    fn default() -> Cfdtmptr0 {
        <crate::RegValueT<Cfdtmptr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmfdctr0_SPEC;
impl crate::sealed::RegSpec for Cfdtmfdctr0_SPEC {
    type DataType = u32;
}

pub type Cfdtmfdctr0 = crate::RegValueT<Cfdtmfdctr0_SPEC>;

impl Cfdtmfdctr0 {
    #[inline(always)]
    pub fn tmesi(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdtmfdctr_0::Tmesi,
        cfdtmfdctr_0::Tmesi,
        Cfdtmfdctr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdtmfdctr_0::Tmesi,
            cfdtmfdctr_0::Tmesi,
            Cfdtmfdctr0_SPEC,
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
        cfdtmfdctr_0::Tmbrs,
        cfdtmfdctr_0::Tmbrs,
        Cfdtmfdctr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdtmfdctr_0::Tmbrs,
            cfdtmfdctr_0::Tmbrs,
            Cfdtmfdctr0_SPEC,
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
        cfdtmfdctr_0::Tmfdf,
        cfdtmfdctr_0::Tmfdf,
        Cfdtmfdctr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdtmfdctr_0::Tmfdf,
            cfdtmfdctr_0::Tmfdf,
            Cfdtmfdctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tmifl(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, Cfdtmfdctr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,Cfdtmfdctr0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmptr(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Cfdtmfdctr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            Cfdtmfdctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdtmfdctr0 {
    #[inline(always)]
    fn default() -> Cfdtmfdctr0 {
        <crate::RegValueT<Cfdtmfdctr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdtmfdctr_0 {

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
pub struct Cfdtmdf00_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf00_SPEC {
    type DataType = u32;
}

pub type Cfdtmdf00 = crate::RegValueT<Cfdtmdf00_SPEC>;

impl Cfdtmdf00 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf00_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf00_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf00_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf00_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf00 {
    #[inline(always)]
    fn default() -> Cfdtmdf00 {
        <crate::RegValueT<Cfdtmdf00_SPEC> as RegisterValue<_>>::new(0)
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
pub struct Cfdtmdf20_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf20_SPEC {
    type DataType = u32;
}

pub type Cfdtmdf20 = crate::RegValueT<Cfdtmdf20_SPEC>;

impl Cfdtmdf20 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf20_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf20_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf20_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf20_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf20_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf20_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf20_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf20_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf20 {
    #[inline(always)]
    fn default() -> Cfdtmdf20 {
        <crate::RegValueT<Cfdtmdf20_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf30_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf30_SPEC {
    type DataType = u32;
}

pub type Cfdtmdf30 = crate::RegValueT<Cfdtmdf30_SPEC>;

impl Cfdtmdf30 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf30_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf30_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf30_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf30_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf30_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf30_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf30_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf30_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf30 {
    #[inline(always)]
    fn default() -> Cfdtmdf30 {
        <crate::RegValueT<Cfdtmdf30_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf40_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf40_SPEC {
    type DataType = u32;
}

pub type Cfdtmdf40 = crate::RegValueT<Cfdtmdf40_SPEC>;

impl Cfdtmdf40 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf40_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf40_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf40_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf40_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf40_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf40_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf40_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf40_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf40 {
    #[inline(always)]
    fn default() -> Cfdtmdf40 {
        <crate::RegValueT<Cfdtmdf40_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf50_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf50_SPEC {
    type DataType = u32;
}

pub type Cfdtmdf50 = crate::RegValueT<Cfdtmdf50_SPEC>;

impl Cfdtmdf50 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf50_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf50_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf50_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf50_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf50_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf50_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf50_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf50_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf50 {
    #[inline(always)]
    fn default() -> Cfdtmdf50 {
        <crate::RegValueT<Cfdtmdf50_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf60_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf60_SPEC {
    type DataType = u32;
}

pub type Cfdtmdf60 = crate::RegValueT<Cfdtmdf60_SPEC>;

impl Cfdtmdf60 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf60_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf60_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf60_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf60_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf60_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf60_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf60_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf60_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf60 {
    #[inline(always)]
    fn default() -> Cfdtmdf60 {
        <crate::RegValueT<Cfdtmdf60_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf70_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf70_SPEC {
    type DataType = u32;
}

pub type Cfdtmdf70 = crate::RegValueT<Cfdtmdf70_SPEC>;

impl Cfdtmdf70 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf70_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf70_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf70_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf70_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf70_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf70_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf70_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf70_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf70 {
    #[inline(always)]
    fn default() -> Cfdtmdf70 {
        <crate::RegValueT<Cfdtmdf70_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf80_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf80_SPEC {
    type DataType = u32;
}

pub type Cfdtmdf80 = crate::RegValueT<Cfdtmdf80_SPEC>;

impl Cfdtmdf80 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf80_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf80_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf80_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf80_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf80_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf80_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf80_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf80_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf80 {
    #[inline(always)]
    fn default() -> Cfdtmdf80 {
        <crate::RegValueT<Cfdtmdf80_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf90_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf90_SPEC {
    type DataType = u32;
}

pub type Cfdtmdf90 = crate::RegValueT<Cfdtmdf90_SPEC>;

impl Cfdtmdf90 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf90_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf90_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf90_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf90_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf90_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf90_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf90_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf90_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf90 {
    #[inline(always)]
    fn default() -> Cfdtmdf90 {
        <crate::RegValueT<Cfdtmdf90_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf100_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf100_SPEC {
    type DataType = u32;
}

pub type Cfdtmdf100 = crate::RegValueT<Cfdtmdf100_SPEC>;

impl Cfdtmdf100 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf100_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf100_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf100_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf100_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf100_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf100_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf100_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf100_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf100 {
    #[inline(always)]
    fn default() -> Cfdtmdf100 {
        <crate::RegValueT<Cfdtmdf100_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf110_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf110_SPEC {
    type DataType = u32;
}

pub type Cfdtmdf110 = crate::RegValueT<Cfdtmdf110_SPEC>;

impl Cfdtmdf110 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf110_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf110_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf110_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf110_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf110_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf110_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf110_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf110_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf110 {
    #[inline(always)]
    fn default() -> Cfdtmdf110 {
        <crate::RegValueT<Cfdtmdf110_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf120_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf120_SPEC {
    type DataType = u32;
}

pub type Cfdtmdf120 = crate::RegValueT<Cfdtmdf120_SPEC>;

impl Cfdtmdf120 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf120_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf120_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf120_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf120_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf120_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf120_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf120_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf120_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf120 {
    #[inline(always)]
    fn default() -> Cfdtmdf120 {
        <crate::RegValueT<Cfdtmdf120_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf130_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf130_SPEC {
    type DataType = u32;
}

pub type Cfdtmdf130 = crate::RegValueT<Cfdtmdf130_SPEC>;

impl Cfdtmdf130 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf130_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf130_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf130_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf130_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf130_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf130_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf130_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf130_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf130 {
    #[inline(always)]
    fn default() -> Cfdtmdf130 {
        <crate::RegValueT<Cfdtmdf130_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf140_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf140_SPEC {
    type DataType = u32;
}

pub type Cfdtmdf140 = crate::RegValueT<Cfdtmdf140_SPEC>;

impl Cfdtmdf140 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf140_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf140_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf140_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf140_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf140_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf140_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf140_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf140_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf140 {
    #[inline(always)]
    fn default() -> Cfdtmdf140 {
        <crate::RegValueT<Cfdtmdf140_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf150_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf150_SPEC {
    type DataType = u32;
}

pub type Cfdtmdf150 = crate::RegValueT<Cfdtmdf150_SPEC>;

impl Cfdtmdf150 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf150_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf150_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf150_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf150_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf150_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf150_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf150_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf150_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf150 {
    #[inline(always)]
    fn default() -> Cfdtmdf150 {
        <crate::RegValueT<Cfdtmdf150_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmid1_SPEC;
impl crate::sealed::RegSpec for Cfdtmid1_SPEC {
    type DataType = u32;
}

pub type Cfdtmid1 = crate::RegValueT<Cfdtmid1_SPEC>;

impl Cfdtmid1 {
    #[inline(always)]
    pub fn tmid(
        self,
    ) -> crate::common::RegisterField<0, 0x1fffffff, 1, 0, u32, u32, Cfdtmid1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1fffffff,
            1,
            0,
            u32,
            u32,
            Cfdtmid1_SPEC,
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
        cfdtmid_1::Thlen,
        cfdtmid_1::Thlen,
        Cfdtmid1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            cfdtmid_1::Thlen,
            cfdtmid_1::Thlen,
            Cfdtmid1_SPEC,
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
        cfdtmid_1::Tmrtr,
        cfdtmid_1::Tmrtr,
        Cfdtmid1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            cfdtmid_1::Tmrtr,
            cfdtmid_1::Tmrtr,
            Cfdtmid1_SPEC,
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
        cfdtmid_1::Tmide,
        cfdtmid_1::Tmide,
        Cfdtmid1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            cfdtmid_1::Tmide,
            cfdtmid_1::Tmide,
            Cfdtmid1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdtmid1 {
    #[inline(always)]
    fn default() -> Cfdtmid1 {
        <crate::RegValueT<Cfdtmid1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdtmid_1 {

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
pub struct Cfdtmptr1_SPEC;
impl crate::sealed::RegSpec for Cfdtmptr1_SPEC {
    type DataType = u32;
}

pub type Cfdtmptr1 = crate::RegValueT<Cfdtmptr1_SPEC>;

impl Cfdtmptr1 {
    #[inline(always)]
    pub fn tmdlc(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, u8, Cfdtmptr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0xf,1,0,u8,u8,Cfdtmptr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmptr1 {
    #[inline(always)]
    fn default() -> Cfdtmptr1 {
        <crate::RegValueT<Cfdtmptr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmfdctr1_SPEC;
impl crate::sealed::RegSpec for Cfdtmfdctr1_SPEC {
    type DataType = u32;
}

pub type Cfdtmfdctr1 = crate::RegValueT<Cfdtmfdctr1_SPEC>;

impl Cfdtmfdctr1 {
    #[inline(always)]
    pub fn tmesi(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfdtmfdctr_1::Tmesi,
        cfdtmfdctr_1::Tmesi,
        Cfdtmfdctr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfdtmfdctr_1::Tmesi,
            cfdtmfdctr_1::Tmesi,
            Cfdtmfdctr1_SPEC,
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
        cfdtmfdctr_1::Tmbrs,
        cfdtmfdctr_1::Tmbrs,
        Cfdtmfdctr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cfdtmfdctr_1::Tmbrs,
            cfdtmfdctr_1::Tmbrs,
            Cfdtmfdctr1_SPEC,
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
        cfdtmfdctr_1::Tmfdf,
        cfdtmfdctr_1::Tmfdf,
        Cfdtmfdctr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cfdtmfdctr_1::Tmfdf,
            cfdtmfdctr_1::Tmfdf,
            Cfdtmfdctr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tmifl(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, Cfdtmfdctr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,Cfdtmfdctr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmptr(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Cfdtmfdctr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            Cfdtmfdctr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfdtmfdctr1 {
    #[inline(always)]
    fn default() -> Cfdtmfdctr1 {
        <crate::RegValueT<Cfdtmfdctr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfdtmfdctr_1 {

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
pub struct Cfdtmdf01_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf01_SPEC {
    type DataType = u32;
}

pub type Cfdtmdf01 = crate::RegValueT<Cfdtmdf01_SPEC>;

impl Cfdtmdf01 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf01_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf01_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf01_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf01_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf01_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf01_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf01_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf01_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf01 {
    #[inline(always)]
    fn default() -> Cfdtmdf01 {
        <crate::RegValueT<Cfdtmdf01_SPEC> as RegisterValue<_>>::new(0)
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
pub struct Cfdtmdf21_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf21_SPEC {
    type DataType = u32;
}

pub type Cfdtmdf21 = crate::RegValueT<Cfdtmdf21_SPEC>;

impl Cfdtmdf21 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf21_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf21_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf21_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf21_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf21_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf21_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf21_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf21_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf21 {
    #[inline(always)]
    fn default() -> Cfdtmdf21 {
        <crate::RegValueT<Cfdtmdf21_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf31_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf31_SPEC {
    type DataType = u32;
}

pub type Cfdtmdf31 = crate::RegValueT<Cfdtmdf31_SPEC>;

impl Cfdtmdf31 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf31_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf31_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf31_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf31_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf31_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf31_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf31_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf31_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf31 {
    #[inline(always)]
    fn default() -> Cfdtmdf31 {
        <crate::RegValueT<Cfdtmdf31_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf41_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf41_SPEC {
    type DataType = u32;
}

pub type Cfdtmdf41 = crate::RegValueT<Cfdtmdf41_SPEC>;

impl Cfdtmdf41 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf41_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf41_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf41_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf41_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf41_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf41_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf41_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf41_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf41 {
    #[inline(always)]
    fn default() -> Cfdtmdf41 {
        <crate::RegValueT<Cfdtmdf41_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf51_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf51_SPEC {
    type DataType = u32;
}

pub type Cfdtmdf51 = crate::RegValueT<Cfdtmdf51_SPEC>;

impl Cfdtmdf51 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf51_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf51_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf51_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf51_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf51_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf51_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf51_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf51_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf51 {
    #[inline(always)]
    fn default() -> Cfdtmdf51 {
        <crate::RegValueT<Cfdtmdf51_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf61_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf61_SPEC {
    type DataType = u32;
}

pub type Cfdtmdf61 = crate::RegValueT<Cfdtmdf61_SPEC>;

impl Cfdtmdf61 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf61_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf61_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf61_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf61_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf61_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf61_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf61_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf61_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf61 {
    #[inline(always)]
    fn default() -> Cfdtmdf61 {
        <crate::RegValueT<Cfdtmdf61_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf71_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf71_SPEC {
    type DataType = u32;
}

pub type Cfdtmdf71 = crate::RegValueT<Cfdtmdf71_SPEC>;

impl Cfdtmdf71 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf71_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf71_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf71_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf71_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf71_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf71_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf71_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf71_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf71 {
    #[inline(always)]
    fn default() -> Cfdtmdf71 {
        <crate::RegValueT<Cfdtmdf71_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf81_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf81_SPEC {
    type DataType = u32;
}

pub type Cfdtmdf81 = crate::RegValueT<Cfdtmdf81_SPEC>;

impl Cfdtmdf81 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf81_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf81_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf81_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf81_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf81_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf81_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf81_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf81_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf81 {
    #[inline(always)]
    fn default() -> Cfdtmdf81 {
        <crate::RegValueT<Cfdtmdf81_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf91_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf91_SPEC {
    type DataType = u32;
}

pub type Cfdtmdf91 = crate::RegValueT<Cfdtmdf91_SPEC>;

impl Cfdtmdf91 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf91_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf91_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf91_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf91_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf91_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf91_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf91_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf91_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf91 {
    #[inline(always)]
    fn default() -> Cfdtmdf91 {
        <crate::RegValueT<Cfdtmdf91_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf101_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf101_SPEC {
    type DataType = u32;
}

pub type Cfdtmdf101 = crate::RegValueT<Cfdtmdf101_SPEC>;

impl Cfdtmdf101 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf101_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf101_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf101_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf101_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf101_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf101_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf101_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf101_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf101 {
    #[inline(always)]
    fn default() -> Cfdtmdf101 {
        <crate::RegValueT<Cfdtmdf101_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf111_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf111_SPEC {
    type DataType = u32;
}

pub type Cfdtmdf111 = crate::RegValueT<Cfdtmdf111_SPEC>;

impl Cfdtmdf111 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf111_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf111_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf111_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf111_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf111_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf111_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf111_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf111_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf111 {
    #[inline(always)]
    fn default() -> Cfdtmdf111 {
        <crate::RegValueT<Cfdtmdf111_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf121_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf121_SPEC {
    type DataType = u32;
}

pub type Cfdtmdf121 = crate::RegValueT<Cfdtmdf121_SPEC>;

impl Cfdtmdf121 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf121_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf121_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf121_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf121_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf121_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf121_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf121_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf121_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf121 {
    #[inline(always)]
    fn default() -> Cfdtmdf121 {
        <crate::RegValueT<Cfdtmdf121_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf131_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf131_SPEC {
    type DataType = u32;
}

pub type Cfdtmdf131 = crate::RegValueT<Cfdtmdf131_SPEC>;

impl Cfdtmdf131 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf131_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf131_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf131_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf131_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf131_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf131_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf131_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf131_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf131 {
    #[inline(always)]
    fn default() -> Cfdtmdf131 {
        <crate::RegValueT<Cfdtmdf131_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf141_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf141_SPEC {
    type DataType = u32;
}

pub type Cfdtmdf141 = crate::RegValueT<Cfdtmdf141_SPEC>;

impl Cfdtmdf141 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf141_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf141_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf141_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf141_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf141_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf141_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf141_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf141_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf141 {
    #[inline(always)]
    fn default() -> Cfdtmdf141 {
        <crate::RegValueT<Cfdtmdf141_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfdtmdf151_SPEC;
impl crate::sealed::RegSpec for Cfdtmdf151_SPEC {
    type DataType = u32;
}

pub type Cfdtmdf151 = crate::RegValueT<Cfdtmdf151_SPEC>;

impl Cfdtmdf151 {
    #[inline(always)]
    pub fn tmdb_ll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfdtmdf151_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfdtmdf151_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_lh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Cfdtmdf151_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Cfdtmdf151_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Cfdtmdf151_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Cfdtmdf151_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmdb_hh(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Cfdtmdf151_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Cfdtmdf151_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfdtmdf151 {
    #[inline(always)]
    fn default() -> Cfdtmdf151 {
        <crate::RegValueT<Cfdtmdf151_SPEC> as RegisterValue<_>>::new(0)
    }
}
